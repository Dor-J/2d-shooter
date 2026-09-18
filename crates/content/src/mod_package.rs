//! Versioned, content-addressed mod packages.
//!
//! A package is data: files, scales, and provenance. Paths stay relative, sizes stay bounded, and
//! a hash mismatch is an error rather than a silent load.

use crate::is_safe_content_path;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const PACKAGE_VERSION: u32 = 1;
pub const MAX_FILES: usize = 64;
pub const MAX_FILE_BYTES: u64 = 256 * 1024;
pub const MAX_TOTAL_BYTES: u64 = 1024 * 1024;
pub const MIN_SCALE: f32 = 0.25;
pub const MAX_SCALE: f32 = 4.0;

/// Historical Soldat interface names. Metadata only — no assets are shipped.
pub const HISTORICAL_INTERFACES: &[&str] = &[
    "Cabbage",
    "Classic",
    "Lacey V2",
    "Micro1",
    "Military",
    "Predator",
    "Soldat Style",
    "Storm",
    "Tech",
    "Text",
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMeta {
    pub name: String,
    pub license: String,
    pub provenance: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModPackage {
    pub format_version: u32,
    pub meta: PackageMeta,
    pub files: BTreeMap<String, Vec<u8>>,
    /// Per-asset draw scales, `mod.ini` style.
    #[serde(default)]
    pub scales: BTreeMap<String, f32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PackageError {
    Empty,
    Version,
    Path,
    TooManyFiles,
    FileTooLarge,
    TotalTooLarge,
    Scale,
    License,
    HashMismatch,
    Preview,
}

pub fn parse_package(bytes: &[u8]) -> Result<ModPackage, PackageError> {
    let package: ModPackage = serde_json::from_slice(bytes).map_err(|_| PackageError::Empty)?;
    validate(&package)?;
    Ok(package)
}

pub fn validate(package: &ModPackage) -> Result<(), PackageError> {
    if package.format_version != PACKAGE_VERSION {
        return Err(PackageError::Version);
    }
    if package.meta.name.trim().is_empty() {
        return Err(PackageError::Empty);
    }
    if package.meta.license.trim().is_empty() || package.meta.provenance.trim().is_empty() {
        return Err(PackageError::License);
    }
    if let Some(preview) = &package.meta.preview {
        if !is_safe_content_path(preview) {
            return Err(PackageError::Preview);
        }
    }
    if package.files.len() > MAX_FILES {
        return Err(PackageError::TooManyFiles);
    }
    let mut total = 0u64;
    for (path, bytes) in &package.files {
        if !is_safe_content_path(path) {
            return Err(PackageError::Path);
        }
        let size = bytes.len() as u64;
        if size > MAX_FILE_BYTES {
            return Err(PackageError::FileTooLarge);
        }
        total = total.saturating_add(size);
        if total > MAX_TOTAL_BYTES {
            return Err(PackageError::TotalTooLarge);
        }
    }
    for scale in package.scales.values() {
        if !scale.is_finite() || !(MIN_SCALE..=MAX_SCALE).contains(scale) {
            return Err(PackageError::Scale);
        }
    }
    Ok(())
}

/// Canonical hash of the package. The same files always produce the same hex.
pub fn package_hash(package: &ModPackage) -> String {
    let encoded = serde_json::to_vec(package).unwrap_or_default();
    let digest = Sha256::digest(&encoded);
    hex(&digest)
}

pub fn require_hash(package: &ModPackage, expected: &str) -> Result<(), PackageError> {
    if package_hash(package).eq_ignore_ascii_case(expected.trim()) {
        Ok(())
    } else {
        Err(PackageError::HashMismatch)
    }
}

/// Reads `mod.ini`-style scale lines: `name=1.25`, one per line, `#` comments.
pub fn parse_mod_ini(text: &str) -> Result<BTreeMap<String, f32>, PackageError> {
    let mut scales = BTreeMap::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        let (key, value) = line.split_once('=').ok_or(PackageError::Scale)?;
        let name = key.trim();
        if name.is_empty() || !is_safe_content_path(name) && name != "default" {
            return Err(PackageError::Path);
        }
        let scale: f32 = value.trim().parse().map_err(|_| PackageError::Scale)?;
        if !scale.is_finite() || !(MIN_SCALE..=MAX_SCALE).contains(&scale) {
            return Err(PackageError::Scale);
        }
        scales.insert(name.to_string(), scale);
    }
    Ok(scales)
}

pub fn historical_interface(name: &str) -> bool {
    HISTORICAL_INTERFACES
        .iter()
        .any(|entry| entry.eq_ignore_ascii_case(name.trim()))
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[(byte >> 4) as usize] as char);
        out.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    out
}
