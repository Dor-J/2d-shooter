//! Realistic: harder-hitting weapons, heavier recoil, and only seeing what you can actually see.
//!
//! The modifier wraps a mode's rules rather than branching through them, so Realistic CTF is an
//! ordinary combination. Visibility is enforced when a snapshot is built for one recipient, which
//! is the only way it can be honest: a client that is never told where an enemy is cannot draw
//! them, however the client is modified.

use serde::{Deserialize, Serialize};

use crate::collision::{CollisionMask, CollisionWorld};
use crate::weapons::WeaponTable;
use crate::Vec2;

/// How far a player can see an enemy at all, in world units.
///
/// Beyond this the enemy is simply not in the snapshot, whatever the line of sight says.
pub const SIGHT_RANGE: f32 = 520.0;

/// How much harder a fall hurts in Realistic.
pub const FALL_DAMAGE_SCALE: f32 = 1.5;

/// How much slower a Realistic player recovers their aim after a shot.
pub const RECOIL_RECOVERY_SCALE: f32 = 0.6;

/// Who may see whom, and what they are told about it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    /// Sent in full: position, aim, health, everything.
    Full,
    /// Left out of the snapshot entirely.
    Hidden,
}

impl Visibility {
    pub const fn is_visible(self) -> bool {
        matches!(self, Self::Full)
    }
}

/// The weapon table Realistic uses.
pub fn weapon_table(realistic: bool) -> WeaponTable {
    WeaponTable::default_for(realistic)
}

/// Whether `observer` can see `target` through the map.
///
/// A clear line between the two bodies and close enough to make out: that is all "seeing" means
/// here, and it is deliberately simple, because anything cleverer would be guessing.
pub fn has_line_of_sight(collision: &CollisionWorld, observer: Vec2, target: Vec2) -> bool {
    let dx = target.x - observer.x;
    let dy = target.y - observer.y;
    if (dx * dx + dy * dy).sqrt() > SIGHT_RANGE {
        return false;
    }
    // A bullet could make the trip, so an eye can too.
    collision
        .raycast(observer, target, CollisionMask::BULLET)
        .is_none()
}

/// One party to a visibility question: who they are, which side they are on, where they are.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewer {
    pub id: u32,
    pub team: u8,
    pub pos: Vec2,
    /// Only meaningful for the observer; a corpse is told nothing about anybody.
    pub alive: bool,
}

impl Viewer {
    pub const fn new(id: u32, team: u8, pos: Vec2, alive: bool) -> Self {
        Self {
            id,
            team,
            pos,
            alive,
        }
    }
}

/// What `observer` is told about `target`.
///
/// Teammates are always visible to each other — a team cannot coordinate otherwise — and your own
/// player is obviously always sent. Everybody else has to be in sight.
pub fn visibility_between(
    collision: &CollisionWorld,
    observer: Viewer,
    target: Viewer,
) -> Visibility {
    if observer.id == target.id {
        return Visibility::Full;
    }
    // A dead player watching the match sees only what the living can see of each other, which is
    // what stops a corpse from calling out enemy positions.
    if !observer.alive {
        return Visibility::Hidden;
    }
    if observer.team != 0 && observer.team == target.team {
        return Visibility::Full;
    }
    if has_line_of_sight(collision, observer.pos, target.pos) {
        Visibility::Full
    } else {
        Visibility::Hidden
    }
}

/// Whether a chat message from `sender_team` should reach `recipient_team`.
///
/// Team chat is private in Realistic, which is the point of having it: an enemy who can read your
/// team's calls has beaten the mode rather than the team.
pub fn team_chat_reaches(
    realistic: bool,
    team_only: bool,
    sender_team: u8,
    recipient_team: u8,
) -> bool {
    if !team_only {
        return true;
    }
    // Team chat is private in every mode; Realistic simply makes the rule matter more.
    let _ = realistic;
    sender_team == recipient_team
}

/// How much a fall hurts under this modifier.
pub fn fall_damage(realistic: bool, base: f32) -> f32 {
    if realistic {
        base * FALL_DAMAGE_SCALE
    } else {
        base
    }
}

/// How quickly the aim settles after a shot.
///
/// A Realistic player fights their own weapon for longer, which is what makes controlled bursts
/// worth learning.
pub fn recoil_recovery(realistic: bool, base: f32) -> f32 {
    if realistic {
        base * RECOIL_RECOVERY_SCALE
    } else {
        base
    }
}

/// Whether the HUD should hide the parts of itself Realistic does without.
///
/// Realistic takes away the crosshair's certainty and the health bar's precision; the client is
/// told so rather than guessing from the mode name.
pub const fn hud_is_minimal(realistic: bool) -> bool {
    realistic
}
