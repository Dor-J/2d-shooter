//! The M2 stationary gun: mounting, aim limits, overheating, and its bolted-down fire rate.
//!
//! Follows `shared/mechanics/Things.pas` (`TThing.CheckStationaryGunCollision`) and the
//! `M2GUN_OVERHEAT` / `M2GUN_OVERAIM` / `STAT_RADIUS` constants at the pinned commit.

use serde::{Deserialize, Serialize};

use super::config::WeaponKind;
use super::table::WeaponTable;
use crate::Vec2;

/// How close a player has to be to take hold of the gun, from `STAT_RADIUS`.
pub const MOUNT_RADIUS: f32 = 15.0;

/// Sustained fire past this many shots overheats the barrel and stops it firing.
pub const OVERHEAT_LIMIT: u16 = 18;

/// Past this many shots the aim begins to wander, and it wanders further the longer it is held.
pub const OVERAIM_LIMIT: u16 = 4;

/// How quickly a barrel that is not being fired cools: one point every this many ticks.
pub const COOLDOWN_TICKS: u64 = 8;

/// How far the barrel swings from the mount, which is what limits where the gun can point.
pub const BARREL_LENGTH: f32 = 3.0;

/// Ticks after the gun appears before anyone may take hold of it.
pub const DEPLOY_TICKS: u16 = 60;

/// A gun bolted to the map that a player can take hold of.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StationaryGun {
    pub pos: Vec2,
    /// The player currently behind it, if any.
    pub mounted_by: Option<u32>,
    /// Shots fired without letting the barrel cool.
    pub use_time: u16,
    /// Ticks left before the gun may be used at all.
    pub deploy: u16,
    /// Where the barrel currently points, as a unit vector from the mount.
    pub barrel: Vec2,
}

impl StationaryGun {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            mounted_by: None,
            use_time: 0,
            deploy: DEPLOY_TICKS,
            barrel: Vec2 { x: 1.0, y: 0.0 },
        }
    }

    /// Whether `player_pos` is close enough to take hold of the gun.
    pub fn in_reach(&self, player_pos: Vec2) -> bool {
        self.deploy == 0
            && (self.pos.x - player_pos.x).hypot(self.pos.y - player_pos.y) < MOUNT_RADIUS
    }

    /// Takes hold of the gun, unless somebody else already has it or it is still deploying.
    pub fn mount(&mut self, player: u32, player_pos: Vec2) -> bool {
        if self.mounted_by.is_some() || !self.in_reach(player_pos) {
            return false;
        }
        self.mounted_by = Some(player);
        true
    }

    /// Lets go of the gun. Walking out of reach does this on its own.
    pub fn dismount(&mut self, player: u32) {
        if self.mounted_by == Some(player) {
            self.mounted_by = None;
        }
    }

    /// Points the barrel at `aim`, as far as the mount allows.
    ///
    /// The barrel is a short arm on the mount, so the gun can face any direction but the shot
    /// always leaves from the end of that arm rather than from the player.
    pub fn aim_at(&mut self, aim: Vec2) {
        let dx = aim.x - self.pos.x;
        let dy = aim.y - self.pos.y;
        let length = (dx * dx + dy * dy).sqrt();
        if length > f32::EPSILON {
            self.barrel = Vec2 {
                x: dx / length,
                y: dy / length,
            };
        }
    }

    /// Where a round leaves the gun.
    pub fn muzzle(&self) -> Vec2 {
        Vec2 {
            x: self.pos.x + self.barrel.x * BARREL_LENGTH,
            y: self.pos.y + self.barrel.y * BARREL_LENGTH,
        }
    }

    /// Whether the gun may fire on this tick: held, cool enough, and on its fire interval.
    pub fn can_fire(&self, tick: u64, table: &WeaponTable) -> bool {
        let interval = u64::from(table.get(WeaponKind::StationaryGun).fire_interval.max(1));
        self.mounted_by.is_some()
            && self.deploy == 0
            && self.use_time <= OVERHEAT_LIMIT
            && tick % interval == 0
    }

    /// Whether the barrel is too hot to fire at all.
    pub fn overheated(&self) -> bool {
        self.use_time > OVERHEAT_LIMIT
    }

    /// How far the aim wanders on this shot, in world units, once the barrel is running hot.
    pub fn aim_wander(&self, roll: f32) -> f32 {
        if self.use_time <= OVERAIM_LIMIT {
            return 0.0;
        }
        let spread = f32::from(self.use_time / 11);
        -spread + roll * 2.0 * spread
    }

    /// Advances the barrel's heat by one tick. Firing heats it; letting go cools it.
    pub fn step(&mut self, tick: u64, fired: bool) {
        self.deploy = self.deploy.saturating_sub(1);
        if fired {
            self.use_time = self.use_time.saturating_add(1);
            return;
        }
        // A barrel that has gone right past the limit is simply allowed to start over.
        if self.use_time > OVERHEAT_LIMIT + 1 {
            self.use_time = 0;
        } else if self.use_time > 0 && tick % COOLDOWN_TICKS == 0 {
            self.use_time -= 1;
        }
    }
}
