//! Acceptance evidence: rust:mod_package:safe rust:mod_package:hash rust:mod_package:ini

use content::mod_package::{
    historical_interface, package_hash, parse_mod_ini, parse_package, require_hash, validate,
    ModPackage, PackageError, PackageMeta, HISTORICAL_INTERFACES, PACKAGE_VERSION,
};
use std::collections::BTreeMap;

fn pkg(files: BTreeMap<String, Vec<u8>>) -> ModPackage {
    ModPackage {
        format_version: PACKAGE_VERSION,
        meta: PackageMeta {
            name: "neon".into(),
            license: "MIT".into(),
            provenance: "generated-for-this-project".into(),
            preview: Some("preview.png".into()),
        },
        files,
        scales: BTreeMap::from([("soldier".into(), 1.0)]),
    }
}

#[test]
fn a_safe_package_hashes_the_same_twice() {
    let package = pkg(BTreeMap::from([("hud/health.png".into(), b"px".to_vec())]));
    validate(&package).unwrap();
    let hash = package_hash(&package);
    assert_eq!(hash.len(), 64);
    assert_eq!(hash, package_hash(&package));
    require_hash(&package, &hash).unwrap();
    assert_eq!(
        require_hash(&package, "deadbeef"),
        Err(PackageError::HashMismatch)
    );
}

#[test]
fn a_path_that_escapes_or_a_file_that_is_too_big_is_refused() {
    let escape = pkg(BTreeMap::from([("../secret".into(), b"x".to_vec())]));
    assert_eq!(validate(&escape), Err(PackageError::Path));
    let huge = pkg(BTreeMap::from([(
        "big.bin".into(),
        vec![0; 256 * 1024 + 1],
    )]));
    assert_eq!(validate(&huge), Err(PackageError::FileTooLarge));
}

#[test]
fn a_package_without_a_license_or_provenance_is_refused() {
    let mut package = pkg(BTreeMap::new());
    package.meta.license.clear();
    assert_eq!(validate(&package), Err(PackageError::License));
}

#[test]
fn mod_ini_reads_scales_and_rejects_a_cheat_size() {
    let scales = parse_mod_ini("# comment\n[scales]\nsoldier=1.5\ndefault=1\n").unwrap();
    assert_eq!(scales["soldier"], 1.5);
    assert_eq!(parse_mod_ini("soldier=99"), Err(PackageError::Scale));
}

#[test]
fn historical_interface_names_are_catalogued_and_not_shipped() {
    assert_eq!(HISTORICAL_INTERFACES.len(), 10);
    assert!(historical_interface("Classic"));
    assert!(historical_interface("soldat style"));
    assert!(!historical_interface("neon"));
}

#[test]
fn json_bytes_parse_and_a_truncated_blob_does_not() {
    let package = pkg(BTreeMap::from([("a.png".into(), b"1".to_vec())]));
    let bytes = serde_json::to_vec(&package).unwrap();
    assert_eq!(parse_package(&bytes).unwrap().meta.name, "neon");
    assert_eq!(parse_package(b"{"), Err(PackageError::Empty));
}
