//! What a bot can see, and how long it takes to notice.
//!
//! Perception reads only what the server would send a human player in the same position, so a bot
//! never knows something a player could not have known. That is the whole reason it lives here
//! rather than reaching into the world directly.

use serde::{Deserialize, Serialize};

use super::profile::Difficulty;
use crate::collision::{CollisionMask, CollisionWorld};
use crate::Vec2;

/// Somebody a bot can see.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Contact {
    pub id: u32,
    pub team: u8,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub alive: bool,
}

/// What a bot currently believes about the world.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Awareness {
    /// The enemy it is watching, if it has found one.
    pub target: Option<u32>,
    /// Where it last saw them.
    pub last_seen: Vec2,
    /// Ticks since it first laid eyes on this target, which is what the reaction time is measured
    /// against.
    pub tracking_ticks: u16,
    /// Ticks since it last actually saw them, so it keeps looking for a moment after they break
    /// line of sight rather than forgetting instantly.
    pub lost_ticks: u16,
}

/// How long a bot keeps hunting somebody it can no longer see.
pub const MEMORY_TICKS: u16 = 90;

impl Awareness {
    /// Whether the bot has watched its target long enough to act.
    pub fn has_reacted(&self, difficulty: Difficulty) -> bool {
        self.target.is_some() && self.tracking_ticks >= difficulty.reaction_ticks()
    }

    /// Whether it is chasing a memory rather than something it can see.
    pub const fn is_hunting_a_memory(&self) -> bool {
        self.target.is_some() && self.lost_ticks > 0
    }

    /// Forgets everything, which is what dying does.
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// Whether `from` can see `to` through the map.
pub fn can_see(collision: &CollisionWorld, from: Vec2, to: Vec2, range: f32) -> bool {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    if (dx * dx + dy * dy).sqrt() > range {
        return false;
    }
    collision.raycast(from, to, CollisionMask::BULLET).is_none()
}

/// Updates what a bot believes, given where it is and who is about.
///
/// The nearest visible enemy wins, and a target that goes out of sight is remembered for a moment
/// before being given up on — which is what makes a bot follow somebody round a corner instead of
/// forgetting them mid-stride.
pub fn observe(
    awareness: &mut Awareness,
    collision: &CollisionWorld,
    difficulty: Difficulty,
    me: Contact,
    others: &[Contact],
) {
    if !me.alive {
        awareness.clear();
        return;
    }

    let range = difficulty.sight_range();
    let mut best: Option<(f32, Contact)> = None;
    for other in others {
        if other.id == me.id || !other.alive {
            continue;
        }
        // Teammates are not targets. A free-for-all has no teams, so everybody is.
        if me.team != 0 && other.team == me.team {
            continue;
        }
        if !can_see(collision, me.pos, other.pos, range) {
            continue;
        }
        let dx = other.pos.x - me.pos.x;
        let dy = other.pos.y - me.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        if best.is_none_or(|(closest, _)| distance < closest) {
            best = Some((distance, *other));
        }
    }

    match best {
        Some((_, contact)) => {
            if awareness.target == Some(contact.id) {
                awareness.tracking_ticks = awareness.tracking_ticks.saturating_add(1);
            } else {
                // A new face: the reaction clock starts again.
                awareness.target = Some(contact.id);
                awareness.tracking_ticks = 0;
            }
            awareness.last_seen = contact.pos;
            awareness.lost_ticks = 0;
        }
        None => {
            if awareness.target.is_none() {
                return;
            }
            awareness.lost_ticks = awareness.lost_ticks.saturating_add(1);
            if awareness.lost_ticks > MEMORY_TICKS {
                awareness.clear();
            }
        }
    }
}

/// Where a bot should actually aim, allowing for how badly it shoots and where the target is going.
///
/// The lead is deliberate: an Elite bot that fires at where somebody *was* is easy to beat by
/// strafing, which would make difficulty meaningless.
pub fn aim_point(
    difficulty: Difficulty,
    target: Contact,
    projectile_speed: f32,
    distance: f32,
    jitter: f32,
) -> Vec2 {
    let travel = if projectile_speed > f32::EPSILON {
        (distance / projectile_speed).min(1.0)
    } else {
        0.0
    };
    let error = difficulty.aim_error();
    Vec2 {
        x: target.pos.x + target.velocity.x * travel + (jitter * 2.0 - 1.0) * error,
        y: target.pos.y + target.velocity.y * travel + (jitter * 2.0 - 1.0) * error,
    }
}
