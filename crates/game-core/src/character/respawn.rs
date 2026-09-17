use crate::{MapSpawn, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RespawnConfig {
    pub delay_ticks: u16,
    /// Ticks a fresh spawn cannot be hurt, so a spawn camper cannot farm the spawn point.
    pub protection_ticks: u16,
    pub corpse_ticks: u16,
}

impl RespawnConfig {
    pub const fn soldat_default() -> Self {
        Self {
            delay_ticks: 120,
            protection_ticks: 60,
            corpse_ticks: 360,
        }
    }
}

impl Default for RespawnConfig {
    fn default() -> Self {
        Self::soldat_default()
    }
}

/// The spawn point used when a map supplies none, so a match is always playable.
pub fn fallback_spawn(id: u32, team: u8) -> Vec2 {
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

/// Picks a spawn for one player: the team's own points in team modes, every point otherwise, and
/// among those the one farthest from the players they would rather not land next to.
pub fn select_spawn(spawns: &[MapSpawn], mode: &str, team: u8, id: u32, enemies: &[Vec2]) -> Vec2 {
    let team_mode = mode == "team";
    let mut candidates: Vec<&MapSpawn> = if team_mode && team > 0 {
        spawns.iter().filter(|spawn| spawn.team == team).collect()
    } else {
        spawns.iter().filter(|spawn| spawn.team == 0).collect()
    };
    if candidates.is_empty() {
        candidates = spawns.iter().collect();
    }
    if candidates.is_empty() {
        return fallback_spawn(id, team);
    }
    if enemies.is_empty() {
        let index = (id as usize) % candidates.len();
        return candidates[index].position;
    }
    candidates
        .into_iter()
        .max_by(|left, right| {
            nearest_enemy(left.position, enemies).total_cmp(&nearest_enemy(right.position, enemies))
        })
        .map_or_else(|| fallback_spawn(id, team), |spawn| spawn.position)
}

fn nearest_enemy(position: Vec2, enemies: &[Vec2]) -> f32 {
    enemies
        .iter()
        .map(|enemy| {
            let dx = enemy.x - position.x;
            let dy = enemy.y - position.y;
            dx * dx + dy * dy
        })
        .fold(f32::INFINITY, f32::min)
}
