#![forbid(unsafe_code)]

use ed25519_dalek::SigningKey;
use map_editor::{original_default_map, EditorProject, DEFAULT_MAPS};
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: map-editor <output-directory>")?;
    let key_hex =
        std::env::var("MAP_SIGNING_KEY_HEX").map_err(|_| "MAP_SIGNING_KEY_HEX is required")?;
    let key_bytes = decode_key(&key_hex)?;
    let key = SigningKey::from_bytes(&key_bytes);
    for entry in DEFAULT_MAPS {
        let editor = EditorProject::new(original_default_map(*entry));
        editor.offline_test(entry.mode)?;
        editor.package(entry.mode, &key)?.deploy(&output)?;
    }
    println!("deployed {} original compatible maps", DEFAULT_MAPS.len());
    Ok(())
}

fn decode_key(hex: &str) -> Result<[u8; 32], String> {
    if hex.len() != 64 {
        return Err("signing key must be 64 hex characters".into());
    }
    let mut bytes = [0; 32];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16)
            .map_err(|_| "invalid signing key hex")?;
    }
    Ok(bytes)
}
