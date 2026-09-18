//! Fire: how a flame spreads from the body it touched and how long the fuel lasts.
//!
//! Follows `shared/mechanics/Bullets.pas` (the `BULLET_STYLE_FLAME` hit branch) at the pinned
//! commit. A flame that lands on someone does not simply vanish: it throws off a weaker flame
//! that keeps burning them, and that flame can do the same again until it is too weak to matter.

use super::config::{BulletStyle, WeaponDef};
use crate::Vec2;

/// A flame only passes fire on while it still carries this fraction of the flamer's full power.
pub const PROPAGATION_MIN_FRACTION: f32 = 1.0 / 3.0;

/// How much power the flame it throws off carries.
pub const PROPAGATION_SCALE: f32 = 2.0 / 3.0;

/// The lifetime a flame is reset to when it catches on a body, one tick short of a fresh flame.
pub const REKINDLE_TICKS: u16 = 31;

/// How many times one flame may pass itself on before it burns out for good.
pub const MAX_PROPAGATIONS: u8 = 1;

/// Whether a flame carrying `hit_multiply` is still hot enough to set its target alight.
pub fn propagates(flamer: &WeaponDef, hit_multiply: f32, propagations: u8) -> bool {
    matches!(flamer.bullet_style, BulletStyle::Flame)
        && propagations < MAX_PROPAGATIONS
        && hit_multiply >= flamer.damage * PROPAGATION_MIN_FRACTION
}

/// The power of the flame thrown off by one that just caught.
pub fn propagated_hit_multiply(hit_multiply: f32) -> f32 {
    hit_multiply * PROPAGATION_SCALE
}

/// The velocity of the flame thrown off a burning body: back the way that body is moving, so the
/// fire trails whoever is carrying it.
pub fn propagated_velocity(victim_velocity: Vec2) -> Vec2 {
    Vec2 {
        x: -victim_velocity.x,
        y: -victim_velocity.y,
    }
}

/// How much fuel one trigger tick consumes. The flamer's magazine is its fuel tank.
pub const FUEL_PER_SHOT: u16 = 1;

/// Whether there is fuel left to burn.
pub fn has_fuel(ammo: u16) -> bool {
    ammo >= FUEL_PER_SHOT
}
