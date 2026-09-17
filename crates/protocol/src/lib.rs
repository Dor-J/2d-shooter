#![forbid(unsafe_code)]

use content::manifest::SignedMapManifest;
use game_core::{Input, World};
use serde::{Deserialize, Serialize};

pub const VERSION: u32 = 6;
pub const MAX_MESSAGE_BYTES: usize = 4096;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Hello {
        version: u32,
        name: String,
        resume: Option<String>,
    },
    Rooms,
    CreateRoom {
        name: String,
        mode: String,
        public: bool,
        #[serde(default)]
        map: Option<String>,
    },
    JoinRoom {
        room: u32,
    },
    JoinByCode {
        code: String,
    },
    LeaveRoom,
    Input {
        input: Input,
    },
    Chat {
        text: String,
    },
    Ping {
        nonce: u64,
    },
    MapDownloadStart {
        map_hash: u32,
        cached_assets: Vec<[u8; 32]>,
    },
    MapDownloadCancel {
        transfer: u64,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: u32,
    pub name: String,
    pub mode: String,
    pub players: usize,
    pub capacity: usize,
    pub map: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapInfo {
    pub name: String,
    pub mode: String,
    pub preview: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Welcome {
        version: u32,
        player: u32,
        token: String,
    },
    Rooms {
        rooms: Vec<RoomInfo>,
    },
    MapCatalog {
        maps: Vec<MapInfo>,
    },
    Joined {
        room: u32,
        player: u32,
        name: String,
        code: String,
        public: bool,
    },
    Snapshot {
        room: u32,
        world: World,
    },
    Chat {
        player: u32,
        name: String,
        text: String,
    },
    Error {
        code: String,
        message: String,
    },
    Pong {
        nonce: u64,
    },
    MapDownloadManifest {
        transfer: u64,
        manifest: SignedMapManifest,
        total_bytes: u64,
    },
    MapDownloadChunk {
        transfer: u64,
        path: String,
        offset: u64,
        total: u64,
        bytes: Vec<u8>,
    },
    MapDownloadComplete {
        transfer: u64,
    },
    MapDownloadCancelled {
        transfer: u64,
    },
}

pub fn parse_client(raw: &str) -> Result<ClientMessage, &'static str> {
    if raw.len() > MAX_MESSAGE_BYTES {
        return Err("message_too_large");
    }
    serde_json::from_str(raw).map_err(|_| "invalid_message")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_message() {
        assert!(parse_client("{").is_err());
    }
    #[test]
    fn rejects_large_message() {
        assert_eq!(
            parse_client(&"x".repeat(MAX_MESSAGE_BYTES + 1)).unwrap_err(),
            "message_too_large"
        );
    }
    #[test]
    fn room_visibility_and_invite_code_messages_parse() {
        assert!(matches!(
            parse_client(
                r#"{"type":"create_room","name":"Friends","mode":"deathmatch","public":false}"#
            ),
            Ok(ClientMessage::CreateRoom { public: false, .. })
        ));
        assert!(matches!(
            parse_client(r#"{"type":"join_by_code","code":"AB12CD"}"#),
            Ok(ClientMessage::JoinByCode { code }) if code == "AB12CD"
        ));
    }
    #[test]
    fn map_download_start_and_cancel_messages_parse() {
        assert!(matches!(
            parse_client(r#"{"type":"map_download_start","map_hash":42,"cached_assets":[]}"#),
            Ok(ClientMessage::MapDownloadStart { map_hash: 42, .. })
        ));
        assert!(matches!(
            parse_client(r#"{"type":"map_download_cancel","transfer":7}"#),
            Ok(ClientMessage::MapDownloadCancel { transfer: 7 })
        ));
    }
}
