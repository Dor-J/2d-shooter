//! Survival: one life a round, and the round ends when one side is left standing.

use serde::{Deserialize, Serialize};

use crate::modes::rules::ModeRules;
use crate::modes::score::Outcome;

/// How Survival is configured.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurvivalConfig {
    /// Players who must be ready before a round begins.
    pub minimum_ready: usize,
    /// Whether players lose their weapons between rounds.
    pub clear_weapons: bool,
    /// Whether the dead are stopped from talking to the living, so a corpse cannot spot for them.
    pub anti_spy_chat: bool,
}

impl Default for SurvivalConfig {
    fn default() -> Self {
        Self {
            minimum_ready: 2,
            clear_weapons: false,
            anti_spy_chat: false,
        }
    }
}

/// How a Survival round stands right now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoundStanding {
    /// Still being fought.
    Ongoing,
    /// One player is the last one standing.
    LastPlayer(u32),
    /// One team is the last one standing.
    LastTeam(u8),
    /// Everybody died, or the last two went together.
    Wiped,
}

impl RoundStanding {
    pub const fn is_over(self) -> bool {
        !matches!(self, Self::Ongoing)
    }

    /// The match outcome this standing implies.
    pub const fn outcome(self) -> Option<Outcome> {
        match self {
            Self::Ongoing => None,
            Self::LastPlayer(id) => Some(Outcome::Player(id)),
            Self::LastTeam(team) => Some(Outcome::Team(team)),
            Self::Wiped => Some(Outcome::Draw),
        }
    }
}

/// One player, as far as a Survival round is concerned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Survivor {
    pub id: u32,
    pub team: u8,
    pub alive: bool,
    /// Whether they are watching rather than playing.
    pub spectator: bool,
}

/// Whether enough players are ready for a round to begin.
pub fn is_ready(config: &SurvivalConfig, ready: usize) -> bool {
    ready >= config.minimum_ready.max(1)
}

/// Who is left standing.
///
/// A free-for-all round ends when one player is alive; a team round when one side is. Two players
/// trading a simultaneous kill wipes the board, which is a draw rather than a win for the corpse
/// that happened to be checked first.
pub fn standing(rules: &ModeRules, survivors: &[Survivor]) -> RoundStanding {
    let playing: Vec<&Survivor> = survivors.iter().filter(|s| !s.spectator).collect();
    if playing.is_empty() {
        return RoundStanding::Ongoing;
    }
    let alive: Vec<&&Survivor> = playing.iter().filter(|s| s.alive).collect();

    if rules.is_team_mode() {
        let mut sides: Vec<u8> = alive.iter().map(|s| s.team).collect();
        sides.sort_unstable();
        sides.dedup();
        return match sides.as_slice() {
            [] => RoundStanding::Wiped,
            [team] => {
                // A side only wins once somebody else has actually been eliminated.
                let started_with: Vec<u8> = {
                    let mut teams: Vec<u8> = playing.iter().map(|s| s.team).collect();
                    teams.sort_unstable();
                    teams.dedup();
                    teams
                };
                if started_with.len() > 1 {
                    RoundStanding::LastTeam(*team)
                } else {
                    RoundStanding::Ongoing
                }
            }
            _ => RoundStanding::Ongoing,
        };
    }

    match alive.as_slice() {
        [] => RoundStanding::Wiped,
        [last] if playing.len() > 1 => RoundStanding::LastPlayer(last.id),
        _ => RoundStanding::Ongoing,
    }
}

/// Whether a dead player may respawn into the round they died in.
///
/// In Survival they may not; that is the whole mode.
pub const fn may_respawn(survival: bool) -> bool {
    !survival
}

/// Whether a dead player watches instead of waiting for a respawn timer.
pub const fn dead_players_spectate(survival: bool) -> bool {
    survival
}

/// Whether a flag may be picked up or captured right now.
///
/// Once a Survival round has been decided the flags are frozen, so a player cannot sneak a capture
/// in after the round is already over.
pub const fn flags_are_live(survival: bool, standing: RoundStanding) -> bool {
    !survival || !standing.is_over()
}

/// Whether a dead player's chat reaches the living.
pub fn chat_reaches_the_living(
    config: &SurvivalConfig,
    survival: bool,
    sender_alive: bool,
) -> bool {
    if !survival || sender_alive {
        return true;
    }
    !config.anti_spy_chat
}

/// What a player carries into the next round.
pub fn keeps_weapons(config: &SurvivalConfig) -> bool {
    !config.clear_weapons
}

impl RoundStanding {
    /// The standing a fresh round starts in, for serde defaults.
    pub const fn ongoing() -> Self {
        Self::Ongoing
    }
}
