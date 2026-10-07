use serde::{Deserialize, Serialize};

use crate::Vec2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawnKind {
    #[default]
    Player,
    Flag,
    Bonus,
    Grenade,
    StationaryGun,
}

impl SpawnKind {
    /// Soldat PMS team numbers for special spawnpoints.
    pub const fn from_pms_team(team: i32) -> Self {
        match team {
            5 | 6 | 14 | 15 => Self::Flag,
            7 => Self::Grenade,
            8..=13 => Self::Bonus,
            16 => Self::StationaryGun,
            _ => Self::Player,
        }
    }

    const fn is_player(kind: &Self) -> bool {
        matches!(kind, Self::Player)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MapSpawn {
    pub position: Vec2,
    pub team: u8,
    #[serde(default, skip_serializing_if = "SpawnKind::is_player")]
    pub kind: SpawnKind,
}

pub(crate) fn spawn(id: u32, team: u8) -> Vec2 {
    let side = if team == 2 {
        880.0
    } else if team == 1 {
        260.0
    } else if id % 2 == 0 {
        880.0
    } else {
        260.0
    };
    Vec2 { x: side, y: 430.0 }
}

pub(crate) fn map_spawn(spawns: &[MapSpawn], id: u32, team: u8) -> Vec2 {
    let matching = spawns
        .iter()
        .filter(|spawn| spawn.team == team || (team == 0 && spawn.team == 0))
        .collect::<Vec<_>>();
    let candidates = if matching.is_empty() {
        spawns.iter().collect::<Vec<_>>()
    } else {
        matching
    };
    candidates
        .get(id as usize % candidates.len().max(1))
        .map_or_else(|| spawn(id, team), |selected| selected.position)
}
