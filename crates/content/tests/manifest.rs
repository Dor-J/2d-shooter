use content::manifest::{
    sign_manifest, AssetEntry, BundleError, MapBundle, MapManifest, MapMode, MapRotation,
    RotationError,
};
use ed25519_dalek::SigningKey;
use std::collections::BTreeMap;

fn manifest() -> MapManifest {
    MapManifest::new(
        "ctf_Original",
        42,
        b"pms bytes",
        vec![AssetEntry::from_bytes("textures/original.png", b"texture")],
        vec![MapMode::CaptureTheFlag],
        Some("previews/ctf_original.png".into()),
    )
    .unwrap()
}

#[test]
fn signed_manifest_verifies_and_rejects_tampering() {
    let signing_key = SigningKey::from_bytes(&[7; 32]);
    let signed = sign_manifest(manifest(), &signing_key).unwrap();
    signed.verify().unwrap();

    let mut tampered = signed.clone();
    tampered.manifest.map_hash ^= 1;
    assert_eq!(
        tampered.verify().unwrap_err(),
        BundleError::InvalidSignature
    );
}

#[test]
fn bundle_rejects_missing_assets_and_checksum_mismatch() {
    let manifest = manifest();
    let no_assets = BTreeMap::new();
    assert_eq!(
        MapBundle::verify(&manifest, b"pms bytes", &no_assets).unwrap_err(),
        BundleError::MissingAsset("textures/original.png".into())
    );

    let wrong = BTreeMap::from([("textures/original.png".into(), b"wrong".to_vec())]);
    assert_eq!(
        MapBundle::verify(&manifest, b"pms bytes", &wrong).unwrap_err(),
        BundleError::ChecksumMismatch("textures/original.png".into())
    );
}

#[test]
fn verified_bundle_accepts_exact_map_and_asset_hashes() {
    let manifest = manifest();
    let assets = BTreeMap::from([("textures/original.png".into(), b"texture".to_vec())]);

    let bundle = MapBundle::verify(&manifest, b"pms bytes", &assets).unwrap();

    assert_eq!(bundle.cache_key().map_hash, 42);
    assert_eq!(bundle.cache_key().asset_hashes.len(), 1);
}

#[test]
fn rotation_rejects_empty_duplicate_and_mode_incompatible_lists() {
    assert_eq!(
        MapRotation::new(vec![], MapMode::Deathmatch).unwrap_err(),
        RotationError::Empty
    );
    assert_eq!(
        MapRotation::new(vec![manifest(), manifest()], MapMode::CaptureTheFlag).unwrap_err(),
        RotationError::Duplicate("ctf_Original".into())
    );
    assert_eq!(
        MapRotation::new(vec![manifest()], MapMode::Deathmatch).unwrap_err(),
        RotationError::ModeMismatch("ctf_Original".into())
    );
}

#[test]
fn rotation_loops_and_supports_next_and_restart() {
    let mut second = manifest();
    second.name = "ctf_Second".into();
    second.map_hash = 43;
    let mut rotation = MapRotation::new(vec![manifest(), second], MapMode::CaptureTheFlag).unwrap();

    assert_eq!(rotation.current().name, "ctf_Original");
    assert_eq!(rotation.advance().name, "ctf_Second");
    assert_eq!(rotation.advance().name, "ctf_Original");
    assert_eq!(rotation.restart().name, "ctf_Original");
}
