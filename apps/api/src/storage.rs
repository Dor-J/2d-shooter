//! File-backed server snapshot. No database crate — a versioned JSON file is enough.

use crate::config::ServerConfig;
use std::path::{Path, PathBuf};

pub const STORE_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lease {
    pub room: u32,
    pub owner: String,
    pub fence: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigBlob {
    pub name: String,
    pub password: String,
    pub max_players: u8,
    pub public: bool,
    pub locked: bool,
    pub admin_password: String,
    pub respawn_seconds: u32,
    pub bonuses: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Store {
    pub version: u32,
    pub bans: String,
    pub config: ConfigBlob,
    pub leases: Vec<Lease>,
    pub drain: bool,
    pub stats: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreError {
    Empty,
    Version,
    Fence,
}

impl From<&ServerConfig> for ConfigBlob {
    fn from(config: &ServerConfig) -> Self {
        Self {
            name: config.name.clone(),
            password: config.password.clone(),
            max_players: config.max_players,
            public: config.public,
            locked: config.locked,
            admin_password: config.admin_password.clone(),
            respawn_seconds: config.respawn_seconds,
            bonuses: config.bonuses,
        }
    }
}

impl ConfigBlob {
    pub fn apply(&self, config: &mut ServerConfig) {
        config.name = self.name.clone();
        config.password = self.password.clone();
        config.max_players = self.max_players;
        config.public = self.public;
        config.locked = self.locked;
        config.admin_password = self.admin_password.clone();
        config.respawn_seconds = self.respawn_seconds;
        config.bonuses = self.bonuses;
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "password": self.password,
            "max_players": self.max_players,
            "public": self.public,
            "locked": self.locked,
            "admin_password": self.admin_password,
            "respawn_seconds": self.respawn_seconds,
            "bonuses": self.bonuses,
        })
    }

    fn from_json(value: &serde_json::Value) -> Result<Self, StoreError> {
        Ok(Self {
            name: value["name"].as_str().unwrap_or("Arena").into(),
            password: value["password"].as_str().unwrap_or_default().into(),
            max_players: value["max_players"].as_u64().unwrap_or(16) as u8,
            public: value["public"].as_bool().unwrap_or(true),
            locked: value["locked"].as_bool().unwrap_or(false),
            admin_password: value["admin_password"].as_str().unwrap_or_default().into(),
            respawn_seconds: value["respawn_seconds"].as_u64().unwrap_or(3) as u32,
            bonuses: value["bonuses"].as_bool().unwrap_or(true),
        })
    }
}

impl Default for Store {
    fn default() -> Self {
        Self {
            version: STORE_VERSION,
            bans: String::new(),
            config: ConfigBlob::from(&ServerConfig::default()),
            leases: Vec::new(),
            drain: false,
            stats: String::new(),
        }
    }
}

impl Store {
    pub fn migrate(text: &str) -> Result<Self, StoreError> {
        if text.trim().is_empty() {
            return Err(StoreError::Empty);
        }
        let value: serde_json::Value = serde_json::from_str(text).map_err(|_| StoreError::Empty)?;
        let version = value["version"].as_u64().unwrap_or(0) as u32;
        if version == 0 || version > STORE_VERSION {
            return Err(StoreError::Version);
        }
        let leases = value["leases"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|lease| {
                Some(Lease {
                    room: lease["room"].as_u64()? as u32,
                    owner: lease["owner"].as_str()?.into(),
                    fence: lease["fence"].as_u64()?,
                })
            })
            .collect();
        Ok(Self {
            version,
            bans: value["bans"].as_str().unwrap_or_default().into(),
            config: ConfigBlob::from_json(&value["config"])?,
            leases,
            drain: value["drain"].as_bool().unwrap_or(false),
            stats: value["stats"].as_str().unwrap_or_default().into(),
        })
    }

    pub fn backup(&self) -> String {
        serde_json::json!({
            "version": self.version,
            "bans": self.bans,
            "config": self.config.to_json(),
            "leases": self.leases.iter().map(|lease| serde_json::json!({
                "room": lease.room,
                "owner": lease.owner,
                "fence": lease.fence,
            })).collect::<Vec<_>>(),
            "drain": self.drain,
            "stats": self.stats,
        })
        .to_string()
    }

    pub fn restore(text: &str) -> Result<Self, StoreError> {
        Self::migrate(text)
    }

    pub fn claim(
        &mut self,
        room: u32,
        owner: impl Into<String>,
        fence: u64,
    ) -> Result<(), StoreError> {
        if let Some(existing) = self.leases.iter().find(|lease| lease.room == room) {
            if existing.fence >= fence {
                return Err(StoreError::Fence);
            }
        }
        self.leases.retain(|lease| lease.room != room);
        self.leases.push(Lease {
            room,
            owner: owner.into(),
            fence,
        });
        Ok(())
    }

    pub fn note_match(&mut self, line: &str) {
        let mut lines: Vec<String> = self
            .stats
            .lines()
            .filter(|row| !row.is_empty())
            .map(str::to_string)
            .collect();
        lines.push(line.to_string());
        if lines.len() > 32 {
            lines.drain(0..lines.len() - 32);
        }
        self.stats = lines.join("\n");
    }

    pub fn release(&mut self, room: u32, fence: u64) -> bool {
        let before = self.leases.len();
        self.leases
            .retain(|lease| !(lease.room == room && lease.fence == fence));
        self.leases.len() != before
    }
}

pub fn data_dir() -> PathBuf {
    std::env::var_os("DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data"))
}

pub fn store_path() -> PathBuf {
    data_dir().join("store.json")
}

pub fn load_file(path: &Path) -> Store {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|text| Store::restore(&text).ok())
        .unwrap_or_default()
}

pub fn save_file(path: &Path, store: &Store) -> Result<(), StoreError> {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(path, store.backup()).map_err(|_| StoreError::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Acceptance evidence: rust:storage:restart
    fn a_snapshot_survives_backup_and_restore() {
        let mut store = Store {
            bans: "bans=1.2.3.4:cheat:perm\nadmins=".into(),
            drain: true,
            ..Store::default()
        };
        store.claim(1, "api-a", 2).unwrap();
        let text = store.backup();
        let restored = Store::restore(&text).unwrap();
        assert_eq!(restored, store);
        assert_eq!(Store::migrate(""), Err(StoreError::Empty));
    }

    #[test]
    // Acceptance evidence: rust:storage:fence
    fn a_stale_fence_cannot_steal_a_room() {
        let mut store = Store::default();
        store.claim(7, "api-a", 3).unwrap();
        assert_eq!(store.claim(7, "api-b", 3), Err(StoreError::Fence));
        store.claim(7, "api-b", 4).unwrap();
        assert_eq!(store.leases[0].owner, "api-b");
        assert!(store.release(7, 4));
        assert!(!store.release(7, 4));
    }

    #[test]
    fn a_round_trip_through_a_temp_file_keeps_the_bans() {
        let dir = std::env::temp_dir().join(format!("arena-store-{}", std::process::id()));
        let path = dir.join("store.json");
        let store = Store {
            bans: "bans=10.0.0.1:abuse:perm\nadmins=10.0.0.2".into(),
            ..Store::default()
        };
        save_file(&path, &store).unwrap();
        let loaded = load_file(&path);
        assert_eq!(loaded.bans, store.bans);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    // Acceptance evidence: rust:storage:stats
    fn finished_matches_are_kept_and_capped() {
        let mut store = Store::default();
        for i in 0..40 {
            store.note_match(&format!("match-{i}"));
        }
        assert_eq!(store.stats.lines().count(), 32);
        assert!(store.stats.contains("match-39"));
        assert!(!store.stats.contains("match-0"));
    }
}
