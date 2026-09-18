//! Grenades: how hard they are thrown, how long the fuse runs, and what a cluster leaves behind.
//!
//! Follows `shared/mechanics/Sprites.pas` (`TSprite.ThrowGrenade`) and
//! `shared/mechanics/Bullets.pas` (`HIT_TYPE_CLUSTERNADE`) at the pinned commit.

use super::config::{WeaponDef, WeaponKind};
use crate::{SimRng, Vec2};

/// The throw animation frames during which a grenade may leave the hand.
pub const THROW_FIRST_FRAME: u16 = 15;
pub const THROW_LAST_FRAME: u16 = 36;

/// A throw released before this frame is a gentle lob rather than a full-strength one.
pub const THROW_WEAK_FRAME: u16 = 24;
pub const THROW_WEAK_SCALE: f32 = 0.65;

/// How many submunitions a cluster grenade scatters.
pub const CLUSTER_SUBMUNITIONS: u8 = 5;

/// A submunition inherits a reversed share of the parent's velocity before it is scattered.
pub const CLUSTER_PARENT_SHARE: f32 = -0.75;
pub const CLUSTER_SCATTER_X: f32 = 5.0;
pub const CLUSTER_SCATTER_Y: f32 = 2.5;
pub const CLUSTER_OFFSET: f32 = 2.5;

/// How long the throw button has been held, clamped to the frames a throw can be released on.
pub fn throw_frame(held_ticks: u16) -> u16 {
    (THROW_FIRST_FRAME + held_ticks).min(THROW_LAST_FRAME)
}

/// Whether a throw held this long has charged far enough to leave the hand.
pub fn throw_is_ready(held_ticks: u16) -> bool {
    throw_frame(held_ticks) > THROW_FIRST_FRAME - 1
}

/// Adds the few degrees of arc a thrown grenade takes, which fades to nothing when aiming straight
/// up or straight down.
pub fn throw_arc(aim: Vec2) -> Vec2 {
    let size = aim.x.signum() / 8.0 * (1.0 - aim.y.abs());
    let arc_x = (aim.y * std::f32::consts::FRAC_PI_2).sin() * size;
    let arc_y = (aim.x * std::f32::consts::FRAC_PI_2).sin() * size;
    let arced = Vec2 {
        x: aim.x + arc_x,
        y: aim.y - arc_y,
    };
    let length = (arced.x * arced.x + arced.y * arced.y).sqrt();
    if length > f32::EPSILON {
        Vec2 {
            x: arced.x / length,
            y: arced.y / length,
        }
    } else {
        aim
    }
}

/// The velocity a grenade leaves the hand with, in world units per tick.
///
/// Strength is the throw frame divided by the grenade's configured speed, so holding the button
/// longer throws further, and the grenade carries the thrower's own momentum with it.
pub fn throw_velocity(def: &WeaponDef, aim: Vec2, held_ticks: u16, thrower_velocity: Vec2) -> Vec2 {
    let frame = throw_frame(held_ticks);
    let direction = throw_arc(aim);
    let speed = def.speed.max(f32::EPSILON);
    let mut strength = f32::from(frame) / speed;
    if frame < THROW_WEAK_FRAME {
        strength *= THROW_WEAK_SCALE;
    }
    Vec2 {
        x: direction.x * strength + thrower_velocity.x * def.inherited_velocity,
        y: direction.y * strength + thrower_velocity.y * def.inherited_velocity,
    }
}

/// The velocities of the five submunitions a cluster grenade leaves behind.
///
/// Each one starts from a reversed share of the parent's velocity and is then scattered, so they
/// spray back out of the impact rather than following it onwards.
pub fn cluster_submunitions(parent_velocity: Vec2, rng: &mut SimRng) -> Vec<Vec2> {
    (0..CLUSTER_SUBMUNITIONS)
        .map(|_| {
            let base = Vec2 {
                x: parent_velocity.x * CLUSTER_PARENT_SHARE,
                y: parent_velocity.y * CLUSTER_PARENT_SHARE,
            };
            Vec2 {
                x: -base.x - CLUSTER_OFFSET + rng.next_unit() * CLUSTER_SCATTER_X,
                y: base.y - CLUSTER_OFFSET + rng.next_unit() * CLUSTER_SCATTER_Y,
            }
        })
        .collect()
}

/// A cluster submunition carries half the parent grenade's power.
pub fn submunition_hit_multiply(frag: &WeaponDef) -> f32 {
    frag.damage / 2.0
}

/// The weapon a thrown grenade becomes, which is the cluster variant when clusters are enabled.
pub const fn thrown_grenade_kind(cluster: bool) -> WeaponKind {
    if cluster {
        WeaponKind::ClusterGrenade
    } else {
        WeaponKind::FragGrenade
    }
}
