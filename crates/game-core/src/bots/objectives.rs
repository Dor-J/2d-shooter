//! What a bot is trying to achieve, which depends entirely on the mode.
//!
//! One function answers "where should this bot be going and why", and every mode's answer is a
//! case in it. A bot that fights well but ignores the flag is not playing the game.

use serde::{Deserialize, Serialize};

use crate::modes::objective::ObjectiveRules;
use crate::modes::objectives_state::Objectives;
use crate::modes::rules::{ModeKind, ModeRules};
use crate::objects::{FlagKind, FlagState};
use crate::Vec2;

/// What a bot is currently trying to do.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BotGoal {
    /// Find somebody and kill them.
    #[default]
    Fight,
    /// Go and take the enemy objective.
    TakeObjective,
    /// Carry what it is holding home.
    CarryHome,
    /// Go and touch its own dropped flag to send it back.
    ReturnOwnFlag,
    /// Stay near its own objective and keep people off it.
    Defend,
    /// Stay near whoever is carrying, and keep them alive.
    EscortCarrier,
    /// Go and pick something up.
    Collect,
}

impl BotGoal {
    /// Whether this goal means going somewhere rather than looking for a fight.
    pub const fn is_travel(self) -> bool {
        !matches!(self, Self::Fight)
    }

    pub const fn describe(self) -> &'static str {
        match self {
            Self::Fight => "fighting",
            Self::TakeObjective => "going for the objective",
            Self::CarryHome => "taking it home",
            Self::ReturnOwnFlag => "returning our flag",
            Self::Defend => "defending",
            Self::EscortCarrier => "escorting the carrier",
            Self::Collect => "picking something up",
        }
    }
}

/// What the bot should do, and where it should go to do it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Objective {
    pub goal: BotGoal,
    /// Where to go, when the goal means going somewhere.
    pub destination: Option<Vec2>,
}

impl Objective {
    pub const fn fight() -> Self {
        Self {
            goal: BotGoal::Fight,
            destination: None,
        }
    }
}

/// Decides what one bot should be doing.
///
/// `defender` says whether this bot has been assigned to hold ground rather than push, which is
/// how a team of bots ends up with some of each instead of all of them chasing the same flag.
pub fn choose(
    rules: &ModeRules,
    objectives: &Objectives,
    me: u32,
    team: u8,
    defender: bool,
) -> Objective {
    let policy = ObjectiveRules::for_mode(rules.kind);

    // Carrying something always means taking it somewhere.
    if let Some(carried) = objectives.carried_by(me) {
        return match rules.kind {
            // A flag you carry home is a capture.
            ModeKind::CaptureTheFlag | ModeKind::Infiltration => {
                let home = objectives
                    .flags
                    .iter()
                    .find(|flag| flag.kind.belongs_to(team))
                    .map(|flag| flag.base);
                Objective {
                    goal: BotGoal::CarryHome,
                    destination: home,
                }
            }
            // Holding the yellow flag is the point; keep it and keep fighting.
            _ => {
                let _ = carried;
                Objective::fight()
            }
        };
    }

    match policy {
        ObjectiveRules::None | ObjectiveRules::Rambomatch => Objective::fight(),

        // Whoever holds the yellow flag scores, so go and get it, or take it back.
        ObjectiveRules::Pointmatch | ObjectiveRules::HoldTheFlag => {
            let Some(flag) = objectives.flag(FlagKind::Yellow) else {
                return Objective::fight();
            };
            match flag.state {
                FlagState::Carried { by } => {
                    // A teammate has it: stay with them. An enemy has it: go and take it.
                    let friendly = rules.is_team_mode()
                        && objectives
                            .carriers()
                            .any(|(_, carrier)| carrier == by && carrier != me);
                    if friendly && defender {
                        Objective {
                            goal: BotGoal::EscortCarrier,
                            destination: Some(flag.pos()),
                        }
                    } else {
                        Objective {
                            goal: BotGoal::TakeObjective,
                            destination: Some(flag.pos()),
                        }
                    }
                }
                _ => Objective {
                    goal: BotGoal::TakeObjective,
                    destination: Some(flag.pos()),
                },
            }
        }

        ObjectiveRules::CaptureTheFlag | ObjectiveRules::Infiltration => {
            let own = objectives
                .flags
                .iter()
                .find(|flag| flag.kind.belongs_to(team));
            let theirs = objectives
                .flags
                .iter()
                .find(|flag| !flag.kind.belongs_to(team) && flag.kind != FlagKind::Yellow);

            // Your own flag on the ground is the most urgent thing on the map: without it home,
            // nothing your team carries can be capped.
            if let Some(own) = own.filter(|flag| flag.state.is_dropped()) {
                return Objective {
                    goal: BotGoal::ReturnOwnFlag,
                    destination: Some(own.pos()),
                };
            }

            // Infiltration's attackers always attack and its defenders always defend.
            if policy == ObjectiveRules::Infiltration {
                if policy.is_defender(team) {
                    return Objective {
                        goal: BotGoal::Defend,
                        destination: own.map(|flag| flag.base),
                    };
                }
                return Objective {
                    goal: BotGoal::TakeObjective,
                    destination: theirs.map(|flag| flag.pos()),
                };
            }

            if defender {
                return Objective {
                    goal: BotGoal::Defend,
                    destination: own.map(|flag| flag.base),
                };
            }
            Objective {
                goal: BotGoal::TakeObjective,
                destination: theirs.map(|flag| flag.pos()),
            }
        }
    }
}

/// Whether this bot should be one of the side's defenders.
///
/// Roughly a third of a team holds ground, chosen by id so the split is stable rather than
/// flickering from tick to tick.
pub fn is_defender(id: u32, team_size: usize) -> bool {
    if team_size <= 1 {
        return false;
    }
    let defenders = (team_size / 3).max(1);
    (id as usize) % team_size < defenders
}

/// Somewhere worth going when a bot has nothing better to do.
///
/// Wandering towards a spawn keeps bots moving through the map rather than milling about where
/// they died.
pub fn wander_target(spawns: &[Vec2], seed: u64) -> Option<Vec2> {
    if spawns.is_empty() {
        return None;
    }
    Some(spawns[(seed as usize) % spawns.len()])
}
