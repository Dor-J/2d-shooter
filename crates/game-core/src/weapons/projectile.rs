//! How a projectile moves and what it does when it meets a surface.
//!
//! Every behavior here is composed from the weapon's bullet style rather than written per weapon,
//! so adding a weapon is a table entry and never a new projectile loop. Follows
//! `shared/mechanics/Bullets.pas` (`TBullet.Update`, `CheckMapCollision`, `CheckMapVerticesCollision`)
//! and `shared/mechanics/Sprites.pas` (`GRENADE_SURFACECOEF`) at the pinned commit.

use super::config::{BulletStyle, WeaponDef};
use crate::Vec2;

/// How much a grenade keeps of the velocity it had after being reflected off a surface.
pub const GRENADE_SURFACE_COEF: f32 = 0.880;

/// Ticks an arrow hangs where it stuck before it drops.
pub const ARROW_RESIST_TICKS: u16 = 280;

/// A rocket only skips off a surface if it travelled this far since its last impact.
pub const RICOCHET_MIN_TRAVEL: f32 = 50.0;

/// How a ricochet mixes the incoming velocity with the reflected one.
pub const RICOCHET_KEEP: f32 = 25.0 / 35.0;
pub const RICOCHET_REFLECT: f32 = 10.0 / 35.0;

/// A flame that touches terrain burns out within this many ticks.
pub const FLAME_SURFACE_TICKS: u16 = 16;

/// A grenade that has only just left the hand passes through instead of detonating.
pub const GRENADE_ARM_TICKS: u16 = 2;

/// What a projectile does when it meets terrain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SurfaceResponse {
    /// Stops dead and is removed: plain bullets, pellets, melee swings, M2 rounds, thrown knives.
    Stop,
    /// Detonates where it landed.
    Explode,
    /// Reflects and keeps going, losing part of its speed.
    Bounce { restitution: f32 },
    /// Skips off if it came in from far enough away, otherwise detonates.
    Ricochet { min_travel: f32 },
    /// Sticks where it landed until its resist timer runs out, then falls.
    Stick { resist_ticks: u16 },
    /// Burns out shortly after contact.
    Smother { ticks: u16 },
}

/// What a projectile does when it meets a body.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImpactResponse {
    /// Wounds the body and is consumed.
    Wound,
    /// Detonates on contact.
    Explode,
    /// Wounds the body and keeps burning, passing the fire on.
    Burn,
    /// Passes through untouched: a grenade in flight is not a bullet.
    Ignore,
}

/// How terrain answers this bullet style.
pub const fn surface_response(style: BulletStyle) -> SurfaceResponse {
    match style {
        BulletStyle::FragGrenade => SurfaceResponse::Bounce {
            restitution: GRENADE_SURFACE_COEF,
        },
        BulletStyle::Flame => SurfaceResponse::Smother {
            ticks: FLAME_SURFACE_TICKS,
        },
        BulletStyle::Arrow => SurfaceResponse::Stick {
            resist_ticks: ARROW_RESIST_TICKS,
        },
        BulletStyle::M79Grenade | BulletStyle::Law | BulletStyle::FlameArrow => {
            SurfaceResponse::Ricochet {
                min_travel: RICOCHET_MIN_TRAVEL,
            }
        }
        BulletStyle::ClusterGrenade | BulletStyle::Cluster => SurfaceResponse::Explode,
        _ => SurfaceResponse::Stop,
    }
}

/// How a body answers this bullet style.
pub const fn impact_response(style: BulletStyle) -> ImpactResponse {
    match style {
        // A live grenade bounces around players; only terrain and its fuse stop it.
        BulletStyle::FragGrenade | BulletStyle::ClusterGrenade => ImpactResponse::Ignore,
        BulletStyle::M79Grenade | BulletStyle::Law | BulletStyle::FlameArrow => {
            ImpactResponse::Explode
        }
        BulletStyle::Cluster => ImpactResponse::Explode,
        BulletStyle::Flame => ImpactResponse::Burn,
        _ => ImpactResponse::Wound,
    }
}

/// How strongly gravity pulls on this style, as a multiple of the world's own gravity.
pub const fn gravity_multiplier(style: BulletStyle) -> f32 {
    match style {
        BulletStyle::FragGrenade | BulletStyle::ClusterGrenade | BulletStyle::Cluster => 1.0,
        BulletStyle::ThrownKnife => 1.0,
        BulletStyle::Arrow | BulletStyle::FlameArrow => 0.5,
        _ => 0.0,
    }
}

/// Reflects `velocity` off a surface with the given unit `normal`, keeping `restitution` of it.
///
/// Upstream subtracts the component along the surface perpendicular twice over, which is an
/// ordinary mirror reflection, and then scales what is left.
pub fn bounce_velocity(velocity: Vec2, normal: Vec2, restitution: f32) -> Vec2 {
    let along = velocity.x * normal.x + velocity.y * normal.y;
    Vec2 {
        x: (velocity.x - 2.0 * along * normal.x) * restitution,
        y: (velocity.y - 2.0 * along * normal.y) * restitution,
    }
}

/// Blends the incoming velocity with a push back out of the surface, the way a skipping rocket
/// does: it keeps most of its run along the wall and is nudged off it rather than mirrored.
///
/// `normal` points out of the surface, back towards where the round came from.
pub fn ricochet_velocity(velocity: Vec2, normal: Vec2) -> Vec2 {
    let speed = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
    Vec2 {
        x: velocity.x * RICOCHET_KEEP + normal.x * speed * RICOCHET_REFLECT,
        y: velocity.y * RICOCHET_KEEP + normal.y * speed * RICOCHET_REFLECT,
    }
}

/// Whether a grenade has been in the air long enough to answer a surface at all.
///
/// A grenade thrown while pressed against a wall would otherwise detonate in the thrower's hand.
pub fn grenade_is_armed(def: &WeaponDef, ttl: u16) -> bool {
    !matches!(def.bullet_style, BulletStyle::FragGrenade)
        || ttl + GRENADE_ARM_TICKS <= def.timeout()
}

/// Whether this style should be tested against bodies at all this tick.
///
/// An arrow that has already stuck into terrain stops wounding anyone.
pub fn collides_with_bodies(style: BulletStyle, ttl: u16) -> bool {
    !matches!(style, BulletStyle::Arrow) || ttl > ARROW_RESIST_TICKS
}

/// Whether this style shoves loose objects such as flags and kits when it passes through them.
///
/// A live frag grenade does not; it is meant to bounce past them and go off on its own timer.
pub const fn pushes_objects(style: BulletStyle) -> bool {
    matches!(
        style,
        BulletStyle::Plain | BulletStyle::Shotgun | BulletStyle::M2
    )
}

/// How much harder a bullet shoves a loose object than it shoves a body.
pub const OBJECT_PUSH_MULTIPLIER: f32 = 9.0;

/// Ticks an object ignores further bullet shoves after one lands, so a burst cannot launch it.
pub const OBJECT_PUSH_COOLDOWN_TICKS: u16 = 3;
