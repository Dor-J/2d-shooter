//! Sides: who is on which team, who is only watching, and how to keep the teams even.

use serde::{Deserialize, Serialize};

use super::rules::ModeRules;

/// The neutral team used by free-for-all modes.
pub const NEUTRAL: u8 = 0;
pub const ALPHA: u8 = 1;
pub const BRAVO: u8 = 2;
/// Spectators are not a playing side; they are marked with their own team number.
pub const SPECTATOR: u8 = 255;

/// What a player asked to be.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamChoice {
    /// Put me wherever the teams need a player.
    #[default]
    Auto,
    Alpha,
    Bravo,
    Spectator,
}

impl TeamChoice {
    pub const fn team(self) -> Option<u8> {
        match self {
            Self::Alpha => Some(ALPHA),
            Self::Bravo => Some(BRAVO),
            Self::Spectator => Some(SPECTATOR),
            Self::Auto => None,
        }
    }

    /// The choice a name refers to, defaulting to auto for anything unrecognised.
    pub fn from_id(id: &str) -> Self {
        match id {
            "alpha" | "1" | "red" => Self::Alpha,
            "bravo" | "2" | "blue" => Self::Bravo,
            "spectator" | "spec" => Self::Spectator,
            _ => Self::Auto,
        }
    }
}

/// How many players each side currently has.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TeamSizes {
    pub alpha: usize,
    pub bravo: usize,
}

impl TeamSizes {
    /// Counts the playing sides, ignoring spectators.
    pub fn count(teams: impl Iterator<Item = u8>) -> Self {
        let mut sizes = Self::default();
        for team in teams {
            match team {
                ALPHA => sizes.alpha += 1,
                BRAVO => sizes.bravo += 1,
                _ => {}
            }
        }
        sizes
    }

    /// The side that needs a player. Alpha wins a tie, so assignment is deterministic.
    pub fn smaller_side(self) -> u8 {
        if self.bravo < self.alpha {
            BRAVO
        } else {
            ALPHA
        }
    }

    /// How lopsided the sides are.
    pub fn difference(self) -> usize {
        self.alpha.abs_diff(self.bravo)
    }
}

/// The team a joining player ends up on.
///
/// A free-for-all has no sides, a spectator is honoured whatever the mode, and a choice that would
/// unbalance the teams is overridden — a player may pick a side, but not stack one.
pub fn assign(rules: &ModeRules, choice: TeamChoice, sizes: TeamSizes) -> u8 {
    if choice == TeamChoice::Spectator {
        return SPECTATOR;
    }
    if !rules.is_team_mode() {
        return NEUTRAL;
    }
    match choice.team() {
        Some(SPECTATOR) => SPECTATOR,
        Some(team) => {
            let after = match team {
                ALPHA => TeamSizes {
                    alpha: sizes.alpha + 1,
                    ..sizes
                },
                _ => TeamSizes {
                    bravo: sizes.bravo + 1,
                    ..sizes
                },
            };
            if after.difference() > 1 {
                sizes.smaller_side()
            } else {
                team
            }
        }
        None => sizes.smaller_side(),
    }
}

/// Whether the sides are uneven enough to be worth moving somebody.
pub fn needs_balancing(sizes: TeamSizes) -> bool {
    sizes.difference() > 1
}

/// Which side a balancing move should take a player from, if one is needed.
pub fn overfull_side(sizes: TeamSizes) -> Option<u8> {
    if !needs_balancing(sizes) {
        return None;
    }
    Some(if sizes.alpha > sizes.bravo {
        ALPHA
    } else {
        BRAVO
    })
}

/// Whether this player is watching rather than playing.
pub const fn is_spectator(team: u8) -> bool {
    team == SPECTATOR
}

/// Whether two players are on the same side for the purposes of friendly fire and scoring.
///
/// Nobody is a teammate in a free-for-all, which is what stops a solo match from treating every
/// kill as a teamkill.
pub fn same_team(rules: &ModeRules, left: u8, right: u8) -> bool {
    rules.is_team_mode() && left == right && left != NEUTRAL && left != SPECTATOR
}
