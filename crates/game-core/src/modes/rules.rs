//! What a mode is: which limits end it, how a kill scores, and who may spawn where.
//!
//! One `ModeRules` value describes a match completely. Modes that need objectives supply their own
//! policy on top of this; the lifecycle itself never branches on the mode name.

use serde::{Deserialize, Serialize};

/// The official modes. Only the first two are playable today; the rest are named here so the
/// lifecycle, the server, and the wire format are already shaped for them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModeKind {
    #[default]
    Deathmatch,
    Teammatch,
    Pointmatch,
    Rambomatch,
    CaptureTheFlag,
    Infiltration,
    HoldTheFlag,
}

impl ModeKind {
    /// The name this mode travels under and that a room is created with.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Deathmatch => "deathmatch",
            Self::Teammatch => "team",
            Self::Pointmatch => "pointmatch",
            Self::Rambomatch => "rambomatch",
            Self::CaptureTheFlag => "ctf",
            Self::Infiltration => "infiltration",
            Self::HoldTheFlag => "htf",
        }
    }

    /// The mode a room name refers to, defaulting to Deathmatch for anything unknown so an
    /// unrecognised room is still playable rather than broken.
    pub fn from_id(id: &str) -> Self {
        match id {
            "team" | "teammatch" => Self::Teammatch,
            "pointmatch" => Self::Pointmatch,
            "rambomatch" => Self::Rambomatch,
            "ctf" => Self::CaptureTheFlag,
            "infiltration" | "inf" => Self::Infiltration,
            "htf" => Self::HoldTheFlag,
            _ => Self::Deathmatch,
        }
    }

    /// Whether players are divided into teams that score together.
    pub const fn is_team_mode(self) -> bool {
        matches!(
            self,
            Self::Teammatch | Self::CaptureTheFlag | Self::Infiltration | Self::HoldTheFlag
        )
    }

    /// Whether the mode is won by carrying or holding an objective rather than only by kills.
    pub const fn has_objective(self) -> bool {
        matches!(
            self,
            Self::Pointmatch
                | Self::Rambomatch
                | Self::CaptureTheFlag
                | Self::Infiltration
                | Self::HoldTheFlag
        )
    }

    /// Whether the mode is playable right now.
    pub const fn is_implemented(self) -> bool {
        true
    }
}

/// The limits that end a match. A limit of zero means that limit does not apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchLimits {
    /// Kills one player needs to win a free-for-all.
    pub kills: u32,
    /// Points a side needs, for modes that score by something other than kills.
    pub points: u32,
    /// Captures a team needs.
    pub captures: u32,
    /// Ticks the match may run for. Zero means it runs until a score limit is reached.
    pub time_ticks: u32,
}

impl MatchLimits {
    /// Soldat's defaults: ten minutes, or the first side to the mode's score limit.
    pub const fn soldat_default(kind: ModeKind) -> Self {
        let (kills, points, captures) = match kind {
            ModeKind::Deathmatch | ModeKind::Teammatch | ModeKind::Rambomatch => (30, 0, 0),
            ModeKind::Pointmatch => (0, 30, 0),
            ModeKind::CaptureTheFlag | ModeKind::Infiltration | ModeKind::HoldTheFlag => (0, 0, 10),
        };
        Self {
            kills,
            points,
            captures,
            time_ticks: 60 * 60 * 10,
        }
    }

    /// Whether any limit at all is set. A match with none never ends on its own.
    pub const fn any(&self) -> bool {
        self.kills > 0 || self.points > 0 || self.captures > 0 || self.time_ticks > 0
    }
}

impl Default for MatchLimits {
    fn default() -> Self {
        Self::soldat_default(ModeKind::Deathmatch)
    }
}

/// The three modifiers, which are orthogonal to the mode and to each other.
///
/// They wrap a mode's rules rather than branching through them, so Realistic CTF and Survival
/// Teammatch are ordinary combinations rather than special cases.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifierSet {
    pub realistic: bool,
    pub survival: bool,
    pub advance: bool,
}

impl ModifierSet {
    /// Whether a dead player waits for the next round instead of respawning into this one.
    pub const fn respawn_is_deferred(&self) -> bool {
        self.survival
    }
}

/// How a kill scores, which is the one thing every mode has an opinion about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringPolicy {
    /// What a normal kill is worth to the killer.
    pub kill: i32,
    /// What killing a teammate is worth. Negative by default, so it costs.
    pub teamkill: i32,
    /// What killing yourself is worth. Negative by default.
    pub suicide: i32,
    /// Whether a teamkill also counts against the team's score, not just the player's.
    pub teamkill_costs_the_team: bool,
}

impl Default for ScoringPolicy {
    fn default() -> Self {
        Self {
            kill: 1,
            teamkill: -1,
            suicide: -1,
            teamkill_costs_the_team: true,
        }
    }
}

/// Everything that makes one match different from another.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeRules {
    pub kind: ModeKind,
    pub limits: MatchLimits,
    pub modifiers: ModifierSet,
    pub scoring: ScoringPolicy,
    /// Whether players may hurt their own team at all.
    pub friendly_fire: bool,
    /// Ticks of countdown before a round begins.
    pub countdown_ticks: u32,
    /// Ticks the end-of-round scoreboard is shown before the next map loads.
    pub round_end_ticks: u32,
    /// Whether a draw is broken by extra time instead of standing as a draw.
    pub overtime: bool,
    /// Ticks of overtime played when a match ends level.
    pub overtime_ticks: u32,
}

impl ModeRules {
    pub fn new(kind: ModeKind) -> Self {
        Self {
            kind,
            limits: MatchLimits::soldat_default(kind),
            modifiers: ModifierSet::default(),
            scoring: ScoringPolicy::default(),
            friendly_fire: false,
            countdown_ticks: 60 * 3,
            round_end_ticks: 60 * 8,
            overtime: kind.has_objective(),
            overtime_ticks: 60 * 60 * 2,
        }
    }

    /// The rules a room name asks for.
    pub fn from_mode_id(id: &str) -> Self {
        Self::new(ModeKind::from_id(id))
    }

    pub const fn is_team_mode(&self) -> bool {
        self.kind.is_team_mode()
    }

    /// Whether a spawn belonging to `spawn_team` may be used by a player on `team`.
    ///
    /// Free-for-all modes ignore spawn teams entirely; team modes use their own side's spawns and
    /// fall back to the neutral ones when a map provides no side-specific spawn.
    pub fn spawn_is_valid(&self, spawn_team: u8, team: u8) -> bool {
        if !self.is_team_mode() {
            return true;
        }
        spawn_team == 0 || spawn_team == team
    }

    /// What one kill is worth to the killer, given who they killed.
    pub fn kill_value(&self, killer: u32, victim: u32, same_team: bool) -> i32 {
        if killer == victim {
            self.scoring.suicide
        } else if same_team && self.is_team_mode() {
            self.scoring.teamkill
        } else {
            self.scoring.kill
        }
    }
}

impl Default for ModeRules {
    fn default() -> Self {
        Self::new(ModeKind::Deathmatch)
    }
}
