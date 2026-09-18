//! The policy that turns a flag into a game mode.
//!
//! The objects in `objects::flag` know how to be carried, dropped, and returned. They do not know
//! who is allowed to do any of it or what it is worth. That is this: one trait's worth of
//! questions, answered differently by each of the five official flag modes, with the lifecycle and
//! the flag itself left alone.

use serde::{Deserialize, Serialize};

use super::rules::{ModeKind, ModeRules};
use super::score::ScoreEvent;
use crate::objects::{Flag, FlagKind, FlagState};

/// What a player may do to a flag they are standing on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlagAction {
    /// Pick it up and carry it.
    Take,
    /// Send it home without carrying it, which is how a friendly flag is rescued.
    Return,
    /// Leave it alone.
    Nothing,
}

/// How many points something is worth and to whom.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Award {
    pub player: u32,
    pub team: u8,
    pub points: u32,
}

impl Award {
    /// The score event this award becomes.
    pub const fn into_event(self) -> ScoreEvent {
        ScoreEvent::Objective {
            player: self.player,
            team: self.team,
            points: self.points,
        }
    }
}

/// Which flags a mode puts on the map, and where they come from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlagLayout {
    /// No flags at all: Deathmatch and Teammatch.
    None,
    /// One neutral flag in the middle: Pointmatch and Hold the Flag.
    Neutral,
    /// One flag per team at its own base: CTF and Infiltration.
    PerTeam,
    /// A single objective the attackers carry away: Rambomatch's bow stands in for it.
    Contested,
}

impl FlagLayout {
    /// What a mode needs on its maps.
    pub const fn for_mode(kind: ModeKind) -> Self {
        match kind {
            ModeKind::Deathmatch | ModeKind::Teammatch => Self::None,
            ModeKind::Pointmatch | ModeKind::HoldTheFlag => Self::Neutral,
            ModeKind::CaptureTheFlag | ModeKind::Infiltration => Self::PerTeam,
            ModeKind::Rambomatch => Self::Contested,
        }
    }

    /// The flags this layout puts on a map.
    pub fn flags(self) -> &'static [FlagKind] {
        match self {
            Self::None | Self::Contested => &[],
            Self::Neutral => &[FlagKind::Yellow],
            Self::PerTeam => &[FlagKind::Alpha, FlagKind::Bravo],
        }
    }
}

/// How often the timed modes hand out a point, and how team sizes shift it.
///
/// Both Hold the Flag and Infiltration award on a timer whose interval stretches when the scoring
/// side outnumbers the other, so a stacked team earns more slowly rather than being barred.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimedAward {
    /// The shortest the interval can ever be, in ticks.
    pub floor_ticks: u32,
    /// The configured interval, in ticks.
    pub base_ticks: u32,
    /// Extra ticks added per extra player on the scoring side.
    pub per_extra_player_ticks: u32,
}

impl TimedAward {
    /// Hold the Flag's default: a point every five seconds, never faster.
    pub const HOLD_THE_FLAG: Self = Self {
        floor_ticks: 300,
        base_ticks: 300,
        per_extra_player_ticks: 120,
    };

    /// Infiltration's default for the defenders holding their objective.
    pub const INFILTRATION_DEFENCE: Self = Self {
        floor_ticks: 300,
        base_ticks: 300,
        per_extra_player_ticks: 120,
    };

    /// The interval to use given how the sides are stacked.
    ///
    /// `advantage` is how many more players the scoring side has than the other; a side that is
    /// outnumbered scores at the base rate rather than faster.
    pub fn interval(&self, advantage: i32) -> u32 {
        let stretch = u32::try_from(advantage.max(0)).unwrap_or(0) * self.per_extra_player_ticks;
        self.base_ticks
            .saturating_add(stretch)
            .max(self.floor_ticks)
    }
}

/// Points Infiltration gives an attacker for getting the objective home.
pub const INFILTRATION_CAPTURE_AWARD: u32 = 30;

/// Points Infiltration takes off the attackers for each extra player they field.
pub const INFILTRATION_STACK_PENALTY: u32 = 5;

/// Points a capture is worth in Capture the Flag.
pub const CTF_CAPTURE_AWARD: u32 = 1;

/// Points holding the yellow flag is worth per Hold the Flag interval.
pub const HTF_HOLD_AWARD: u32 = 1;

/// How much holding the point flag multiplies a Pointmatch kill by.
pub const POINTMATCH_FLAG_MULTIPLIER: u32 = 2;

/// Everything a mode decides about its objective.
pub trait ObjectivePolicy {
    /// What `player` on `team` may do to `flag` by standing on it.
    fn action_for(&self, flag: &Flag, team: u8) -> FlagAction;

    /// What bringing `carried` home is worth, if it is worth anything right now.
    ///
    /// `own_flag` is the carrier's own flag, which most modes require to be at base.
    fn capture_award(
        &self,
        carried: &Flag,
        own_flag: Option<&Flag>,
        carrier: u32,
        team: u8,
    ) -> Option<Award>;

    /// What a kill is worth to a player who may or may not be carrying the objective.
    ///
    /// Only Pointmatch changes this; everything else scores a kill the ordinary way.
    fn kill_points(&self, _carrying: bool) -> u32 {
        1
    }
}

/// The policy for one mode.
///
/// This is an enum rather than a boxed trait object so a `World` stays plain data that serialises,
/// which the deterministic replay depends on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectiveRules {
    /// No objective at all.
    None,
    /// The yellow flag doubles your kills while you hold it.
    Pointmatch,
    /// The bow makes you the target; only its holder scores.
    Rambomatch,
    /// Take theirs home while yours is still at base.
    CaptureTheFlag,
    /// One side attacks the objective, the other holds it.
    Infiltration,
    /// Hold the yellow flag and your side scores on a timer.
    HoldTheFlag,
}

impl ObjectiveRules {
    /// The policy a mode uses.
    pub const fn for_mode(kind: ModeKind) -> Self {
        match kind {
            ModeKind::Deathmatch | ModeKind::Teammatch => Self::None,
            ModeKind::Pointmatch => Self::Pointmatch,
            ModeKind::Rambomatch => Self::Rambomatch,
            ModeKind::CaptureTheFlag => Self::CaptureTheFlag,
            ModeKind::Infiltration => Self::Infiltration,
            ModeKind::HoldTheFlag => Self::HoldTheFlag,
        }
    }

    /// The flags this mode puts on a map.
    pub const fn layout(self) -> FlagLayout {
        match self {
            Self::None => FlagLayout::None,
            Self::Pointmatch | Self::HoldTheFlag => FlagLayout::Neutral,
            Self::CaptureTheFlag | Self::Infiltration => FlagLayout::PerTeam,
            Self::Rambomatch => FlagLayout::Contested,
        }
    }

    /// The timer this mode scores on, if it scores on one at all.
    pub const fn timed_award(self) -> Option<TimedAward> {
        match self {
            Self::HoldTheFlag => Some(TimedAward::HOLD_THE_FLAG),
            Self::Infiltration => Some(TimedAward::INFILTRATION_DEFENCE),
            _ => None,
        }
    }

    /// In Infiltration, Alpha attacks and Bravo defends. Every other mode is symmetrical.
    pub const fn attacking_team(self) -> Option<u8> {
        match self {
            Self::Infiltration => Some(1),
            _ => None,
        }
    }

    /// Whether `team` is the side defending an objective rather than attacking it.
    pub const fn is_defender(self, team: u8) -> bool {
        matches!(self.attacking_team(), Some(attackers) if attackers != team)
    }
}

impl ObjectivePolicy for ObjectiveRules {
    fn action_for(&self, flag: &Flag, team: u8) -> FlagAction {
        if flag.state.carrier().is_some() {
            return FlagAction::Nothing;
        }
        match self {
            Self::None => FlagAction::Nothing,
            // The yellow flag is anybody's, wherever it is.
            Self::Pointmatch | Self::HoldTheFlag => FlagAction::Take,
            Self::Rambomatch => FlagAction::Take,
            Self::CaptureTheFlag => {
                if flag.kind.belongs_to(team) {
                    // Your own flag: worth rescuing if it is out of place, otherwise leave it.
                    if flag.state.is_dropped() {
                        FlagAction::Return
                    } else {
                        FlagAction::Nothing
                    }
                } else {
                    FlagAction::Take
                }
            }
            Self::Infiltration => {
                // Only Bravo's objective is in play; Alpha's flag is scenery.
                if flag.kind == FlagKind::Alpha {
                    return FlagAction::Nothing;
                }
                if flag.kind.belongs_to(team) {
                    if flag.state.is_dropped() {
                        FlagAction::Return
                    } else {
                        FlagAction::Nothing
                    }
                } else {
                    FlagAction::Take
                }
            }
        }
    }

    fn capture_award(
        &self,
        carried: &Flag,
        own_flag: Option<&Flag>,
        carrier: u32,
        team: u8,
    ) -> Option<Award> {
        let FlagState::Carried { by } = carried.state else {
            return None;
        };
        if by != carrier {
            return None;
        }
        match self {
            // A capture needs your own flag standing at home, which is what stops two teams from
            // simply swapping flags for a point each.
            Self::CaptureTheFlag => {
                let own = own_flag?;
                if !own.is_home_and_free() || !carried.touching_down_on(own) {
                    return None;
                }
                Some(Award {
                    player: carrier,
                    team,
                    points: CTF_CAPTURE_AWARD,
                })
            }
            Self::Infiltration => {
                let own = own_flag?;
                if !own.is_home_and_free() || !carried.touching_down_on(own) {
                    return None;
                }
                Some(Award {
                    player: carrier,
                    team,
                    points: INFILTRATION_CAPTURE_AWARD,
                })
            }
            // Pointmatch, Hold the Flag, and Rambomatch are not won by carrying anything home.
            _ => None,
        }
    }

    fn kill_points(&self, carrying: bool) -> u32 {
        match self {
            Self::Pointmatch if carrying => POINTMATCH_FLAG_MULTIPLIER,
            _ => 1,
        }
    }
}

/// The Infiltration penalty for fielding more attackers than defenders.
///
/// Stacking the attack is allowed, but it is worth less: each extra attacker costs the side five
/// of the points a capture earns, and the score never goes below nothing.
pub fn infiltration_award(attackers: usize, defenders: usize) -> u32 {
    let extra = attackers.saturating_sub(defenders);
    let penalty = u32::try_from(extra).unwrap_or(0) * INFILTRATION_STACK_PENALTY;
    INFILTRATION_CAPTURE_AWARD.saturating_sub(penalty)
}

/// The objective rules a set of mode rules implies.
pub fn policy_for(rules: &ModeRules) -> ObjectiveRules {
    ObjectiveRules::for_mode(rules.kind)
}
