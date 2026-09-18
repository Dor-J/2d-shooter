//! The one objective object every flag mode is built from.
//!
//! A flag is a physical body that can sit at its base, be carried, be dropped, be thrown, and find
//! its own way home. Every mode in task 13 uses this same object; what differs between them is the
//! policy that decides who may take it and what taking it is worth, which lives in `modes/`.
//!
//! Follows OpenSoldat `shared/mechanics/Things.pas` (`TThing.Update`, `CheckSpriteCollision`) and
//! the `FLAG_TIMEOUT`, `FLAG_INTEREST_TIME`, and `TOUCHDOWN_RADIUS` constants at the commit pinned
//! in `docs/parity/reference-lock.md`.

use serde::{Deserialize, Serialize};

use crate::collision::{CollisionWorld, DynamicBody, DynamicBodyKind};
use crate::Vec2;

/// Ticks a dropped flag waits before it takes itself home.
pub const FLAG_TIMEOUT: u16 = 60 * 25;

/// How close a carried flag has to be to a flag at its base to count as a capture.
pub const TOUCHDOWN_RADIUS: f32 = 28.0;

/// How close a player has to be to touch a flag on the ground.
pub const FLAG_PICKUP_RADIUS: f32 = 20.0;

/// Ticks a player who has just dropped a flag cannot pick one up again, so a death does not
/// instantly hand the flag back to whoever dropped it.
pub const GRAB_COOLDOWN_TICKS: u16 = 12;

/// The radius the flag's body uses against terrain.
pub const FLAG_RADIUS: f32 = 10.0;

/// Which flag this is. The number matches the team that owns it, so a comparison against a
/// player's team is all a mode needs to tell "mine" from "theirs".
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlagKind {
    /// Alpha's flag, which Bravo tries to take.
    Alpha,
    /// Bravo's flag, which Alpha tries to take.
    Bravo,
    /// The neutral yellow flag used by Pointmatch and Hold the Flag.
    Yellow,
}

impl FlagKind {
    /// The team that owns this flag, or zero for the neutral one.
    pub const fn owning_team(self) -> u8 {
        match self {
            Self::Alpha => 1,
            Self::Bravo => 2,
            Self::Yellow => 0,
        }
    }

    /// Whether this flag belongs to `team`.
    pub const fn belongs_to(self, team: u8) -> bool {
        self.owning_team() != 0 && self.owning_team() == team
    }

    /// What players call it.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Alpha => "Red Flag",
            Self::Bravo => "Blue Flag",
            Self::Yellow => "Yellow Flag",
        }
    }
}

/// Where a flag is right now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlagState {
    /// Sitting on its base, where it belongs.
    AtBase,
    /// Being carried by a player.
    Carried { by: u32 },
    /// On the ground somewhere it does not belong, counting down to its own return.
    Dropped { ticks_left: u16 },
}

impl FlagState {
    pub const fn is_at_base(self) -> bool {
        matches!(self, Self::AtBase)
    }
    pub const fn carrier(self) -> Option<u32> {
        match self {
            Self::Carried { by } => Some(by),
            _ => None,
        }
    }
    pub const fn is_dropped(self) -> bool {
        matches!(self, Self::Dropped { .. })
    }
}

/// Why a flag changed hands, so a mode can score it and the HUD can announce it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlagEvent {
    /// Somebody picked it up.
    Taken { flag: FlagKind, by: u32 },
    /// A carrier lost it, by dying or by throwing it.
    Dropped { flag: FlagKind, by: u32 },
    /// A player touched their own flag on the ground and sent it home.
    Returned { flag: FlagKind, by: Option<u32> },
    /// A carrier brought it to their own base while their own flag was there.
    Captured { flag: FlagKind, by: u32 },
}

/// One flag: a physical body plus where it belongs and who has it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    pub kind: FlagKind,
    /// Where it returns to.
    pub base: Vec2,
    pub state: FlagState,
    /// The body that falls, slides, and collides with the map.
    pub body: DynamicBody,
    /// Whoever last let go of this flag, and how long they have to wait before touching it again.
    ///
    /// Without this a thrown flag lands on the thrower's own feet and is picked straight back up,
    /// and a carrier killed on the spot snatches it back the instant they respawn there.
    #[serde(default)]
    pub grab_cooldown: Option<(u32, u16)>,
}

impl Flag {
    /// A flag standing on its base.
    pub fn new(kind: FlagKind, base: Vec2) -> Self {
        Self {
            kind,
            base,
            state: FlagState::AtBase,
            body: DynamicBody::new(DynamicBodyKind::Flag, base, FLAG_RADIUS),
            grab_cooldown: None,
        }
    }

    /// Where the flag is, which is the carrier's position while it is being carried.
    pub fn pos(&self) -> Vec2 {
        self.body.pos
    }

    /// Whether a capture may be scored against this flag: it has to be home and unheld.
    pub fn is_home_and_free(&self) -> bool {
        self.state.is_at_base() && self.state.carrier().is_none()
    }

    /// Whether this player is still in the cooling-off period after letting the flag go.
    pub fn is_cooling_down_for(&self, player: u32) -> bool {
        matches!(self.grab_cooldown, Some((who, ticks)) if who == player && ticks > 0)
    }

    /// Picks the flag up. Returns the event, or nothing if it was already held or the player has
    /// only just let go of it.
    pub fn take(&mut self, player: u32) -> Option<FlagEvent> {
        if self.state.carrier().is_some() || self.is_cooling_down_for(player) {
            return None;
        }
        self.grab_cooldown = None;
        self.state = FlagState::Carried { by: player };
        self.body.vel = Vec2::default();
        Some(FlagEvent::Taken {
            flag: self.kind,
            by: player,
        })
    }

    /// Drops the flag where the carrier was standing, with whatever momentum they had.
    pub fn drop_at(&mut self, pos: Vec2, velocity: Vec2) -> Option<FlagEvent> {
        let carrier = self.state.carrier()?;
        self.state = FlagState::Dropped {
            ticks_left: FLAG_TIMEOUT,
        };
        self.body.pos = pos;
        self.body.vel = velocity;
        self.body.grounded = false;
        self.grab_cooldown = Some((carrier, GRAB_COOLDOWN_TICKS));
        Some(FlagEvent::Dropped {
            flag: self.kind,
            by: carrier,
        })
    }

    /// Sends the flag home, whether because somebody touched it, it timed out, or a round reset.
    pub fn return_home(&mut self, by: Option<u32>) -> FlagEvent {
        self.state = FlagState::AtBase;
        self.body.pos = self.base;
        self.body.vel = Vec2::default();
        self.body.grounded = true;
        self.grab_cooldown = None;
        FlagEvent::Returned {
            flag: self.kind,
            by,
        }
    }

    /// Advances the flag by one tick.
    ///
    /// A carried flag rides along with whoever has it; a dropped one falls, slides, and counts
    /// down to taking itself home.
    pub fn step(
        &mut self,
        collision: &CollisionWorld,
        dt: f32,
        carrier_pos: Option<Vec2>,
    ) -> Option<FlagEvent> {
        if let Some((who, ticks)) = self.grab_cooldown {
            let left = ticks.saturating_sub(1);
            self.grab_cooldown = if left == 0 { None } else { Some((who, left)) };
        }
        match self.state {
            FlagState::AtBase => {
                self.body.pos = self.base;
                self.body.vel = Vec2::default();
                None
            }
            FlagState::Carried { .. } => {
                if let Some(pos) = carrier_pos {
                    self.body.pos = pos;
                    self.body.vel = Vec2::default();
                }
                None
            }
            FlagState::Dropped { ticks_left } => {
                self.body.step(collision, dt);
                let left = ticks_left.saturating_sub(1);
                self.state = FlagState::Dropped { ticks_left: left };
                if left == 0 {
                    // Nobody came for it, so it takes itself home rather than being lost.
                    return Some(self.return_home(None));
                }
                None
            }
        }
    }

    /// Whether `pos` is close enough to touch this flag on the ground.
    pub fn within_reach(&self, pos: Vec2) -> bool {
        if self.state.carrier().is_some() {
            return false;
        }
        let dx = self.body.pos.x - pos.x;
        let dy = self.body.pos.y - pos.y;
        (dx * dx + dy * dy).sqrt() <= FLAG_PICKUP_RADIUS
    }

    /// Whether this flag is close enough to `other` to score a touchdown against it.
    pub fn touching_down_on(&self, other: &Flag) -> bool {
        let dx = self.body.pos.x - other.body.pos.x;
        let dy = self.body.pos.y - other.body.pos.y;
        (dx * dx + dy * dy).sqrt() < TOUCHDOWN_RADIUS
    }

    /// Ticks before a dropped flag takes itself home, for the HUD.
    pub const fn return_countdown(&self) -> Option<u16> {
        match self.state {
            FlagState::Dropped { ticks_left } => Some(ticks_left),
            _ => None,
        }
    }

    /// Shoves the flag, which is how a bullet or a blast moves one.
    pub fn push(&mut self, impulse: Vec2) {
        if self.state.carrier().is_some() {
            return;
        }
        self.body.vel = Vec2 {
            x: self.body.vel.x + impulse.x,
            y: self.body.vel.y + impulse.y,
        };
        self.body.grounded = false;
    }

    /// The velocity a manual throw gives the flag.
    ///
    /// A thrown flag carries the thrower's own momentum plus a push along the aim, which is what
    /// makes passing one to a teammate possible.
    pub fn throw_velocity(aim: Vec2, thrower_velocity: Vec2) -> Vec2 {
        let length = (aim.x * aim.x + aim.y * aim.y).sqrt();
        if length <= f32::EPSILON {
            return thrower_velocity;
        }
        Vec2 {
            x: thrower_velocity.x + aim.x / length * THROW_SPEED,
            y: thrower_velocity.y + aim.y / length * THROW_SPEED,
        }
    }
}

/// How hard a manual flag throw is, in world units per second.
pub const THROW_SPEED: f32 = 320.0;
