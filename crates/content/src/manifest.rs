use crate::is_safe_content_path;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const MANIFEST_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapMode {
    Deathmatch,
    TeamMatch,
    PointMatch,
    RamboMatch,
    CaptureTheFlag,
    HoldTheFlag,
    Infiltration,
    Community,
}

impl MapMode {
    pub fn accepts_name(self, name: &str) -> bool {
        match self {
            Self::CaptureTheFlag => name.starts_with("ctf_"),
            Self::HoldTheFlag => name.starts_with("htf_"),
            Self::Infiltration => name.starts_with("inf_"),
            Self::Community => name.contains('_'),
            _ => !["ctf_", "htf_", "inf_"]
                .iter()
                .any(|prefix| name.starts_with(prefix)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetEntry {
    pub path: String,
    pub sha256: [u8; 32],
    pub bytes: u64,
}

impl AssetEntry {
    pub fn from_bytes(path: impl Into<String>, bytes: &[u8]) -> Self {
        Self {
            path: path.into(),
            sha256: sha256(bytes),
            bytes: bytes.len() as u64,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapManifest {
    pub format_version: u32,
    pub name: String,
    pub map_hash: u32,
    pub pms_sha256: [u8; 32],
    pub assets: Vec<AssetEntry>,
    pub modes: Vec<MapMode>,
    pub preview: Option<String>,
}

impl MapManifest {
    pub fn new(
        name: impl Into<String>,
        map_hash: u32,
        pms: &[u8],
        assets: Vec<AssetEntry>,
        modes: Vec<MapMode>,
        preview: Option<String>,
    ) -> Result<Self, BundleError> {
        let manifest = Self {
            format_version: MANIFEST_VERSION,
            name: name.into(),
            map_hash,
            pms_sha256: sha256(pms),
            assets,
            modes,
            preview,
        };
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), BundleError> {
        if self.format_version != MANIFEST_VERSION {
            return Err(BundleError::UnsupportedVersion(self.format_version));
        }
        if self.name.is_empty()
            || !self.name.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '_' | '-')
            })
        {
            return Err(BundleError::InvalidName(self.name.clone()));
        }
        if self.modes.is_empty() {
            return Err(BundleError::NoModes);
        }
        if self.modes.iter().any(|mode| !mode.accepts_name(&self.name)) {
            return Err(BundleError::ModePrefixMismatch(self.name.clone()));
        }
        let mut paths = BTreeSet::new();
        for asset in &self.assets {
            if !is_safe_content_path(&asset.path) {
                return Err(BundleError::UnsafePath(asset.path.clone()));
            }
            if !paths.insert(asset.path.clone()) {
                return Err(BundleError::DuplicateAsset(asset.path.clone()));
            }
        }
        if let Some(path) = &self.preview {
            if !is_safe_content_path(path) {
                return Err(BundleError::UnsafePath(path.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedMapManifest {
    pub manifest: MapManifest,
    pub public_key: [u8; 32],
    pub signature: Vec<u8>,
}

impl SignedMapManifest {
    pub fn verify(&self) -> Result<(), BundleError> {
        self.manifest.validate()?;
        let key = VerifyingKey::from_bytes(&self.public_key)
            .map_err(|_| BundleError::InvalidSignature)?;
        let signature =
            Signature::from_slice(&self.signature).map_err(|_| BundleError::InvalidSignature)?;
        key.verify(&canonical_bytes(&self.manifest)?, &signature)
            .map_err(|_| BundleError::InvalidSignature)
    }
}

pub fn sign_manifest(
    manifest: MapManifest,
    key: &SigningKey,
) -> Result<SignedMapManifest, BundleError> {
    let signature = key.sign(&canonical_bytes(&manifest)?).to_bytes().to_vec();
    Ok(SignedMapManifest {
        manifest,
        public_key: key.verifying_key().to_bytes(),
        signature,
    })
}

#[derive(Clone, Debug)]
pub struct MapBundle {
    manifest: MapManifest,
}

impl MapBundle {
    pub fn verify(
        manifest: &MapManifest,
        pms: &[u8],
        assets: &BTreeMap<String, Vec<u8>>,
    ) -> Result<Self, BundleError> {
        if sha256(pms) != manifest.pms_sha256 {
            return Err(BundleError::ChecksumMismatch(format!(
                "{}.pms",
                manifest.name
            )));
        }
        for expected in &manifest.assets {
            let bytes = assets
                .get(&expected.path)
                .ok_or_else(|| BundleError::MissingAsset(expected.path.clone()))?;
            if bytes.len() as u64 != expected.bytes || sha256(bytes) != expected.sha256 {
                return Err(BundleError::ChecksumMismatch(expected.path.clone()));
            }
        }
        Ok(Self {
            manifest: manifest.clone(),
        })
    }

    pub fn cache_key(&self) -> CacheKey {
        CacheKey {
            map_hash: self.manifest.map_hash,
            asset_hashes: self
                .manifest
                .assets
                .iter()
                .map(|asset| asset.sha256)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CacheKey {
    pub map_hash: u32,
    pub asset_hashes: Vec<[u8; 32]>,
}

#[derive(Clone, Debug)]
pub struct MapRotation {
    maps: Vec<MapManifest>,
    current: usize,
}

impl MapRotation {
    pub fn new(maps: Vec<MapManifest>, mode: MapMode) -> Result<Self, RotationError> {
        if maps.is_empty() {
            return Err(RotationError::Empty);
        }
        let mut names = BTreeSet::new();
        for map in &maps {
            if !names.insert(map.name.clone()) {
                return Err(RotationError::Duplicate(map.name.clone()));
            }
            if !map.modes.contains(&mode) || !mode.accepts_name(&map.name) {
                return Err(RotationError::ModeMismatch(map.name.clone()));
            }
        }
        Ok(Self { maps, current: 0 })
    }

    pub fn current(&self) -> &MapManifest {
        &self.maps[self.current]
    }
    pub fn advance(&mut self) -> &MapManifest {
        self.current = (self.current + 1) % self.maps.len();
        self.current()
    }
    pub fn restart(&mut self) -> &MapManifest {
        self.current = 0;
        self.current()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BundleError {
    InvalidName(String),
    NoModes,
    UnsafePath(String),
    DuplicateAsset(String),
    MissingAsset(String),
    ChecksumMismatch(String),
    InvalidSignature,
    Serialization,
    UnsupportedVersion(u32),
    ModePrefixMismatch(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RotationError {
    Empty,
    Duplicate(String),
    ModeMismatch(String),
}

fn canonical_bytes(manifest: &MapManifest) -> Result<Vec<u8>, BundleError> {
    serde_json::to_vec(manifest).map_err(|_| BundleError::Serialization)
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
