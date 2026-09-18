//! Contact weapons: the knife, the chainsaw, and bare hands.
//!
//! A melee swing is an ordinary projectile with a one-tick lifetime launched a short way ahead of
//! the hands, which is how upstream models it in `shared/mechanics/Sprites.pas` (`TSprite.Fire`)
//! and `shared/mechanics/Bullets.pas` (`CheckSpriteCollision`, melee branch).

use super::config::{BulletStyle, WeaponDef, WeaponKind};
use crate::Vec2;

/// How far ahead of the hands a contact weapon starts, in world units.
pub const CONTACT_OFFSET: f32 = 2.0;

/// The radius a melee swing sweeps around its own path, from `PART_RADIUS`.
pub const MELEE_RADIUS: f32 = 7.0;

/// The extra reach a swing gets from the buttstock behind the hands.
pub const BUTTSTOCK_OFFSET: f32 = 4.0;

/// Whether this weapon wounds by touching rather than by firing something across the map.
pub const fn is_contact_weapon(style: BulletStyle) -> bool {
    matches!(style, BulletStyle::Knife | BulletStyle::Punch)
}

/// Whether the weapon keeps wounding for as long as the trigger is held and the target is in
/// reach, instead of landing one discrete hit.
///
/// The chainsaw is the only one: a two-tick fire interval and a one-tick swing lifetime add up to
/// a continuous cut rather than a series of stabs.
pub fn is_continuous_contact(def: &WeaponDef) -> bool {
    def.kind == WeaponKind::Chainsaw
}

/// Where a contact weapon's swing begins: a short way along the aim from the hands.
pub fn contact_origin(hands: Vec2, aim_direction: Vec2) -> Vec2 {
    Vec2 {
        x: hands.x + aim_direction.x * CONTACT_OFFSET,
        y: hands.y + aim_direction.y * CONTACT_OFFSET,
    }
}

/// The segment a swing sweeps this tick, from just behind the hands to the end of its reach.
///
/// Upstream tests the blade against the body from the buttstock forward, so a target standing
/// right on top of the attacker is still cut.
pub fn swing_segment(hands: Vec2, aim_direction: Vec2, reach: f32) -> (Vec2, Vec2) {
    let start = Vec2 {
        x: hands.x - aim_direction.x * BUTTSTOCK_OFFSET,
        y: hands.y - aim_direction.y * BUTTSTOCK_OFFSET,
    };
    let end = Vec2 {
        x: hands.x + aim_direction.x * reach,
        y: hands.y + aim_direction.y * reach,
    };
    (start, end)
}

/// How far a swing reaches in one tick: its own speed, since it lives for exactly one tick.
pub fn reach(def: &WeaponDef) -> f32 {
    if is_contact_weapon(def.bullet_style) {
        def.speed + CONTACT_OFFSET
    } else {
        0.0
    }
}
