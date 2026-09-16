use content::manifest::{BundleError, MapBundle, SignedMapManifest};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub const CHUNK_BYTES: usize = 3_072;

#[derive(Clone)]
struct HostedMap {
    signed: SignedMapManifest,
    files: Vec<(String, Vec<u8>, Option<[u8; 32]>)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransferEvent {
    Chunk {
        transfer: u64,
        path: String,
        offset: u64,
        total: u64,
        bytes: Vec<u8>,
    },
    Complete {
        transfer: u64,
    },
    Cancelled {
        transfer: u64,
    },
}

struct Transfer {
    files: Vec<(String, Vec<u8>)>,
    file: usize,
    offset: usize,
}

#[derive(Default)]
pub struct MapService {
    maps: BTreeMap<u32, HostedMap>,
    transfers: BTreeMap<u64, Transfer>,
    next_transfer: u64,
}

impl MapService {
    pub fn load_directory(root: &Path) -> Result<Self, String> {
        let mut service = Self::default();
        let entries = std::fs::read_dir(root).map_err(|error| error.to_string())?;
        for entry in entries {
            let package = entry.map_err(|error| error.to_string())?.path();
            if !package.is_dir() {
                continue;
            }
            let manifest_bytes =
                std::fs::read(package.join("manifest.json")).map_err(|error| error.to_string())?;
            let signed: SignedMapManifest =
                serde_json::from_slice(&manifest_bytes).map_err(|error| error.to_string())?;
            let pms = std::fs::read(
                package
                    .join("maps")
                    .join(format!("{}.pms", signed.manifest.name)),
            )
            .map_err(|error| error.to_string())?;
            let mut assets = BTreeMap::new();
            for asset in &signed.manifest.assets {
                assets.insert(
                    asset.path.clone(),
                    std::fs::read(package.join(&asset.path)).map_err(|error| error.to_string())?,
                );
            }
            service
                .insert(signed, pms, assets)
                .map_err(|error| format!("{error:?}"))?;
        }
        Ok(service)
    }

    pub fn insert(
        &mut self,
        signed: SignedMapManifest,
        pms: Vec<u8>,
        assets: BTreeMap<String, Vec<u8>>,
    ) -> Result<(), BundleError> {
        signed.verify()?;
        MapBundle::verify(&signed.manifest, &pms, &assets)?;
        let mut files = vec![(format!("maps/{}.pms", signed.manifest.name), pms, None)];
        for asset in &signed.manifest.assets {
            files.push((
                asset.path.clone(),
                assets[&asset.path].clone(),
                Some(asset.sha256),
            ));
        }
        self.maps
            .insert(signed.manifest.map_hash, HostedMap { signed, files });
        Ok(())
    }

    pub fn start(
        &mut self,
        map_hash: u32,
        cached_assets: &[[u8; 32]],
    ) -> Result<(u64, SignedMapManifest, u64), &'static str> {
        let hosted = self.maps.get(&map_hash).ok_or("map_missing")?;
        let cached = cached_assets.iter().copied().collect::<BTreeSet<_>>();
        let files = hosted
            .files
            .iter()
            .filter(|(_, _, hash)| hash.is_none_or(|value| !cached.contains(&value)))
            .map(|(path, bytes, _)| (path.clone(), bytes.clone()))
            .collect::<Vec<_>>();
        let total = files.iter().map(|(_, bytes)| bytes.len() as u64).sum();
        self.next_transfer = self.next_transfer.wrapping_add(1).max(1);
        let transfer = self.next_transfer;
        self.transfers.insert(
            transfer,
            Transfer {
                files,
                file: 0,
                offset: 0,
            },
        );
        Ok((transfer, hosted.signed.clone(), total))
    }

    pub fn next(&mut self, transfer: u64) -> Result<TransferEvent, &'static str> {
        let state = self
            .transfers
            .get_mut(&transfer)
            .ok_or("transfer_missing")?;
        if state.file >= state.files.len() {
            self.transfers.remove(&transfer);
            return Ok(TransferEvent::Complete { transfer });
        }
        let (path, file) = &state.files[state.file];
        let offset = state.offset;
        let end = (offset + CHUNK_BYTES).min(file.len());
        let event = TransferEvent::Chunk {
            transfer,
            path: path.clone(),
            offset: offset as u64,
            total: file.len() as u64,
            bytes: file[offset..end].to_vec(),
        };
        state.offset = end;
        if end == file.len() {
            state.file += 1;
            state.offset = 0;
        }
        Ok(event)
    }

    pub fn cancel(&mut self, transfer: u64) -> Result<TransferEvent, &'static str> {
        self.transfers.remove(&transfer).ok_or("transfer_missing")?;
        Ok(TransferEvent::Cancelled { transfer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use content::manifest::{sign_manifest, AssetEntry, MapManifest, MapMode};
    use ed25519_dalek::SigningKey;

    fn service() -> (MapService, [u8; 32]) {
        let pms = vec![1; 4_000];
        let texture = vec![2; 100];
        let asset = AssetEntry::from_bytes("textures/map.png", &texture);
        let hash = asset.sha256;
        let manifest = MapManifest::new(
            "Original",
            7,
            &pms,
            vec![asset],
            vec![MapMode::Deathmatch],
            None,
        )
        .unwrap();
        let signed = sign_manifest(manifest, &SigningKey::from_bytes(&[9; 32])).unwrap();
        let mut service = MapService::default();
        service
            .insert(
                signed,
                pms,
                BTreeMap::from([("textures/map.png".into(), texture)]),
            )
            .unwrap();
        (service, hash)
    }

    #[test]
    fn streams_bounded_chunks_with_offsets_and_completion() {
        let (mut service, _) = service();
        let (transfer, _, total) = service.start(7, &[]).unwrap();
        assert_eq!(total, 4_100);
        assert!(
            matches!(service.next(transfer).unwrap(), TransferEvent::Chunk { offset: 0, total: 4_000, ref bytes, .. } if bytes.len() == CHUNK_BYTES)
        );
        assert!(matches!(
            service.next(transfer).unwrap(),
            TransferEvent::Chunk { offset: 3_072, .. }
        ));
        assert!(
            matches!(service.next(transfer).unwrap(), TransferEvent::Chunk { path, offset: 0, total: 100, .. } if path == "textures/map.png")
        );
        assert_eq!(
            service.next(transfer).unwrap(),
            TransferEvent::Complete { transfer }
        );
    }

    #[test]
    fn skips_cached_assets_and_cancels_active_transfers() {
        let (mut service, texture_hash) = service();
        let (transfer, _, total) = service.start(7, &[texture_hash]).unwrap();
        assert_eq!(total, 4_000);
        assert_eq!(
            service.cancel(transfer).unwrap(),
            TransferEvent::Cancelled { transfer }
        );
        assert_eq!(service.next(transfer).unwrap_err(), "transfer_missing");
    }

    #[test]
    fn missing_maps_do_not_create_transfers() {
        let (mut service, _) = service();
        assert_eq!(service.start(999, &[]).unwrap_err(), "map_missing");
    }
}
