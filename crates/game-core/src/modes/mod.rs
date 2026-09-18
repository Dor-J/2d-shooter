//! The match framework: what a mode is, how a round runs, who is on which side, and what scores.
//!
//! `rules` describes a match, `round` runs it through its phases, `score` is the single ledger
//! every point passes through, `team` owns sides and spectators, and `spawn` decides where players
//! start and whether a map can host the mode at all. Deathmatch and Teammatch are built entirely
//! from these pieces, so the modes added in task 13 need no changes to the lifecycle.

pub mod modifiers;
pub mod objective;
pub mod objectives_state;
pub mod round;
pub mod rules;
pub mod score;
pub mod scripted;
pub mod spawn;
pub mod spectator;
pub mod statistics;
pub mod team;

pub use modifiers::{
    has_line_of_sight, survival_standing, visibility_between, AdvanceConfig, RoundStanding,
    SurvivalConfig, Survivor, Unlocked, Viewer, Visibility,
};
pub use objective::{
    infiltration_award, policy_for, Award, FlagAction, FlagLayout, ObjectivePolicy, ObjectiveRules,
    TimedAward, CTF_CAPTURE_AWARD, INFILTRATION_CAPTURE_AWARD, POINTMATCH_FLAG_MULTIPLIER,
};
pub use objectives_state::{Bearer, Objectives};
pub use round::{MatchEvent, MatchPhase, MatchState, Rotation};
pub use rules::{MatchLimits, ModeKind, ModeRules, ModifierSet, ScoringPolicy};
pub use score::{leader, limit_reached, Outcome, PlayerScore, ScoreEvent, ScoreLedger};
pub use scripted::{ScriptedError, ScriptedRules};
pub use spawn::{validate as validate_spawns, SpawnProblem};
pub use spectator::{SpectateCommand, Spectator, SpectatorView};
pub use statistics::{
    scoreboard, MatchHistory, MatchStats, MatchSummary, ObjectiveStat, PlayerStats, ScoreboardRow,
    StatEvent, WeaponStats,
};
pub use team::{assign as assign_team, TeamChoice, TeamSizes, ALPHA, BRAVO, NEUTRAL, SPECTATOR};
