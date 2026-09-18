#![forbid(unsafe_code)]

use content::manifest::SignedMapManifest;
use game_core::{Input, World};
use serde::{Deserialize, Serialize};

pub const VERSION: u32 = 13;
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
        #[serde(default)]
        weapon_mod: Option<String>,
        /// Which of the three modifiers to switch on. Absent means none of them.
        #[serde(default)]
        modifiers: Option<WireModifiers>,
        /// A community ruleset to play, by name. It supplies its own base mode and modifiers.
        #[serde(default)]
        ruleset: Option<String>,
        /// Whether bonus kits are handed out at all. Absent means the server default.
        #[serde(default)]
        bonuses: Option<bool>,
        /// How many bots to fill the room with.
        #[serde(default)]
        bots: Option<u8>,
        /// How hard those bots should be.
        #[serde(default)]
        bot_difficulty: Option<String>,
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
    /// Change what a spectator is looking at.
    Spectate {
        command: WireSpectateCommand,
    },
    /// Join or leave the spectators, or switch sides.
    SetTeam {
        team: String,
    },
    /// Add or remove bots in the room the sender is in.
    Bots {
        /// How many bots the room should end up with.
        count: u8,
        #[serde(default)]
        difficulty: Option<String>,
    },
}

/// What a spectator asked to look at next.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WireSpectateCommand {
    Next,
    Previous,
    Follow { player: u32 },
    FreeCamera,
}

/// The three modifiers, as they travel on the wire.
///
/// A plain struct rather than `game_core::ModifierSet` so the protocol stays the sole definition
/// of what crosses the wire, and so an older client that sends none simply gets none.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireModifiers {
    #[serde(default)]
    pub realistic: bool,
    #[serde(default)]
    pub survival: bool,
    #[serde(default)]
    pub advance: bool,
}

impl WireModifiers {
    /// Whether any modifier at all is on, which is what the room browser shows a badge for.
    pub const fn any(&self) -> bool {
        self.realistic || self.survival || self.advance
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: u32,
    pub name: String,
    pub mode: String,
    pub players: usize,
    pub capacity: usize,
    pub map: String,
    pub weapon_mod: String,
    pub weapon_hash: u32,
    /// Which modifiers the room is running, for the browser to show.
    #[serde(default)]
    pub modifiers: WireModifiers,
    /// The community ruleset, when the room is playing one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ruleset: Option<String>,
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
        /// Boxed so one large snapshot does not inflate every other server message.
        world: Box<World>,
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
    fn create_room_accepts_an_optional_weapon_mod() {
        assert!(matches!(
            parse_client(
                r#"{"type":"create_room","name":"R","mode":"deathmatch","public":true,"weapon_mod":"realistic"}"#
            ),
            Ok(ClientMessage::CreateRoom {
                weapon_mod: Some(mod_name),
                ..
            }) if mod_name == "realistic"
        ));
    }

    #[test]
    fn spectate_and_team_messages_parse() {
        assert!(matches!(
            parse_client(r#"{"type":"spectate","command":{"type":"next"}}"#),
            Ok(ClientMessage::Spectate {
                command: WireSpectateCommand::Next
            })
        ));
        assert!(matches!(
            parse_client(r#"{"type":"spectate","command":{"type":"follow","player":7}}"#),
            Ok(ClientMessage::Spectate {
                command: WireSpectateCommand::Follow { player: 7 }
            })
        ));
        assert!(matches!(
            parse_client(r#"{"type":"set_team","team":"spectator"}"#),
            Ok(ClientMessage::SetTeam { team }) if team == "spectator"
        ));
    }

    #[test]
    fn a_room_that_names_no_modifiers_gets_none() {
        let Ok(ClientMessage::CreateRoom {
            modifiers, ruleset, ..
        }) = parse_client(
            r#"{"type":"create_room","name":"Plain","mode":"deathmatch","public":true}"#,
        )
        else {
            panic!("an older client's room still parses");
        };
        assert_eq!(modifiers, None);
        assert_eq!(ruleset, None);
    }

    #[test]
    fn a_room_can_name_its_modifiers_and_its_ruleset() {
        let Ok(ClientMessage::CreateRoom {
            modifiers, ruleset, ..
        }) = parse_client(
            r#"{"type":"create_room","name":"Hard","mode":"team","public":true,"modifiers":{"realistic":true,"survival":true,"advance":false},"ruleset":"Zombie"}"#,
        )
        else {
            panic!("a modern client's room parses");
        };
        let modifiers = modifiers.expect("modifiers were sent");
        assert!(modifiers.realistic && modifiers.survival && !modifiers.advance);
        assert!(modifiers.any());
        assert_eq!(ruleset.as_deref(), Some("Zombie"));
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
