//! The extension point community modes are built on.
//!
//! A community mode is a set of declared restrictions and awards laid over an official mode, not a
//! program. That is deliberate: a server that runs arbitrary code from a map pack is a server that
//! can be made to do anything, and none of the eleven modes the wiki lists needs more than this.
//!
//! Every field is bounded, every list has a cap, and anything a ruleset does not say is simply the
//! underlying mode's own behavior.

use serde::{Deserialize, Serialize};

use super::rules::{ModeKind, ModeRules, ModifierSet};
use crate::weapons::WeaponKind;

/// The most weapons a ruleset may name, so a hostile file cannot make the server allocate freely.
pub const MAX_ALLOWED_WEAPONS: usize = 16;

/// The longest a ruleset's name may be.
pub const MAX_NAME_BYTES: usize = 48;

/// What a community ruleset may change about a match.
///
/// Each field is a restriction or a bonus on top of an official mode. There is no way to express
/// "run this code", which is the whole point.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptedRules {
    /// What the mode is called in the room browser.
    pub name: String,
    /// The official mode this is played on top of.
    pub base: ModeKind,
    /// The modifiers it turns on.
    pub modifiers: ModifierSet,
    /// The only weapons players may carry. Empty means no restriction.
    pub allowed_weapons: Vec<WeaponKind>,
    /// Health every player spawns with, as a percentage of normal.
    pub spawn_health_percent: u8,
    /// Whether players may hurt each other at all.
    pub damage_enabled: bool,
    /// Whether the mode is won by reaching a place rather than by fighting.
    pub race_to_finish: bool,
    /// Whether a killed player joins the killer's side instead of respawning on their own.
    pub converts_on_death: bool,
    /// Whether one side starts hidden and the other has to find them.
    pub hunt: bool,
    /// Extra points for a kill, over and above the base mode's.
    pub bonus_points_per_kill: u8,
}

impl ScriptedRules {
    /// A ruleset that changes nothing, which is what every community mode starts from.
    pub fn plain(name: &str, base: ModeKind) -> Self {
        Self {
            name: name.chars().take(MAX_NAME_BYTES).collect(),
            base,
            modifiers: ModifierSet::default(),
            allowed_weapons: Vec::new(),
            spawn_health_percent: 100,
            damage_enabled: true,
            race_to_finish: false,
            converts_on_death: false,
            hunt: false,
            bonus_points_per_kill: 0,
        }
    }

    /// Whether `kind` may be carried under this ruleset.
    pub fn allows_weapon(&self, kind: WeaponKind) -> bool {
        self.allowed_weapons.is_empty() || self.allowed_weapons.contains(&kind)
    }

    /// The health a player spawns with.
    pub fn spawn_health(&self) -> i32 {
        i32::from(self.spawn_health_percent.clamp(1, 100))
    }

    /// The mode rules this ruleset produces.
    pub fn to_mode_rules(&self) -> ModeRules {
        let mut rules = ModeRules::new(self.base);
        rules.modifiers = self.modifiers;
        rules
    }

    /// Checks a ruleset before it is used.
    ///
    /// A ruleset that came off disk or over the wire is untrusted, so it is checked once here
    /// rather than defended against everywhere it is read.
    pub fn validate(&self) -> Result<(), ScriptedError> {
        if self.name.trim().is_empty() {
            return Err(ScriptedError::EmptyName);
        }
        if self.name.len() > MAX_NAME_BYTES {
            return Err(ScriptedError::NameTooLong {
                bytes: self.name.len(),
                limit: MAX_NAME_BYTES,
            });
        }
        if self.allowed_weapons.len() > MAX_ALLOWED_WEAPONS {
            return Err(ScriptedError::TooManyWeapons {
                count: self.allowed_weapons.len(),
                limit: MAX_ALLOWED_WEAPONS,
            });
        }
        if !self.allowed_weapons.is_empty()
            && !self
                .allowed_weapons
                .iter()
                .any(|kind| kind.slot().is_some())
        {
            return Err(ScriptedError::NoUsableWeapon);
        }
        if self.spawn_health_percent == 0 || self.spawn_health_percent > 100 {
            return Err(ScriptedError::HealthOutOfRange {
                percent: self.spawn_health_percent,
            });
        }
        Ok(())
    }
}

/// Why a community ruleset cannot be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptedError {
    EmptyName,
    NameTooLong {
        bytes: usize,
        limit: usize,
    },
    TooManyWeapons {
        count: usize,
        limit: usize,
    },
    /// Every weapon named was one nobody can actually select.
    NoUsableWeapon,
    HealthOutOfRange {
        percent: u8,
    },
}

/// The eleven community modes the wiki lists, each expressed as a ruleset.
///
/// They are data, not code: the whole point of the extension point is that adding the twelfth is
/// another entry here rather than another branch in the simulation.
pub mod presets {
    use super::*;

    /// Race to the top of the map. Nobody can hurt anybody.
    pub fn climb() -> ScriptedRules {
        ScriptedRules {
            damage_enabled: false,
            race_to_finish: true,
            ..ScriptedRules::plain("Climb", ModeKind::Teammatch)
        }
    }

    /// One weapon, thrown back and forth.
    pub fn dodgeball() -> ScriptedRules {
        ScriptedRules {
            allowed_weapons: vec![WeaponKind::CombatKnife],
            ..ScriptedRules::plain("Dodgeball", ModeKind::Teammatch)
        }
    }

    /// Hold ground rather than carry anything: scored on the hold timer.
    pub fn domination() -> ScriptedRules {
        ScriptedRules::plain("Domination", ModeKind::HoldTheFlag)
    }

    /// One side hides, the other seeks.
    pub fn hide_and_seek() -> ScriptedRules {
        ScriptedRules {
            hunt: true,
            ..ScriptedRules::plain("Hide and Seek", ModeKind::Teammatch)
        }
    }

    /// Knives only.
    pub fn knife_only() -> ScriptedRules {
        ScriptedRules {
            allowed_weapons: vec![WeaponKind::CombatKnife],
            ..ScriptedRules::plain("Knife Only", ModeKind::Deathmatch)
        }
    }

    /// Everybody dies to one hit.
    pub fn one_shots() -> ScriptedRules {
        ScriptedRules {
            spawn_health_percent: 1,
            ..ScriptedRules::plain("OneShots", ModeKind::Deathmatch)
        }
    }

    /// Two sides, two loadouts.
    pub fn pirates_vs_ninjas() -> ScriptedRules {
        ScriptedRules {
            allowed_weapons: vec![
                WeaponKind::Spas12,
                WeaponKind::CombatKnife,
                WeaponKind::Ruger77,
            ],
            ..ScriptedRules::plain("Pirates vs Ninjas", ModeKind::Teammatch)
        }
    }

    /// Realistic, round-based, one life.
    pub fn realistic_counter_strike() -> ScriptedRules {
        ScriptedRules {
            modifiers: ModifierSet {
                realistic: true,
                survival: true,
                advance: false,
            },
            ..ScriptedRules::plain("RS/CS", ModeKind::Teammatch)
        }
    }

    /// Dig in and push the line.
    pub fn trench_wars() -> ScriptedRules {
        ScriptedRules::plain("Trench Wars", ModeKind::Infiltration)
    }

    /// Trench Wars with Realistic on.
    pub fn tactical_trench_wars() -> ScriptedRules {
        ScriptedRules {
            modifiers: ModifierSet {
                realistic: true,
                survival: false,
                advance: false,
            },
            ..ScriptedRules::plain("Tactical Trench Wars", ModeKind::Infiltration)
        }
    }

    /// Whoever dies joins the horde.
    pub fn zombie() -> ScriptedRules {
        ScriptedRules {
            converts_on_death: true,
            allowed_weapons: vec![WeaponKind::CombatKnife, WeaponKind::Chainsaw],
            ..ScriptedRules::plain("Zombie", ModeKind::Teammatch)
        }
    }

    /// Every community mode, for the room browser and for the tests that check them all.
    pub fn all() -> Vec<ScriptedRules> {
        vec![
            climb(),
            dodgeball(),
            domination(),
            hide_and_seek(),
            knife_only(),
            one_shots(),
            pirates_vs_ninjas(),
            realistic_counter_strike(),
            trench_wars(),
            tactical_trench_wars(),
            zombie(),
        ]
    }

    /// The ruleset a room name refers to, if it names one.
    pub fn by_id(id: &str) -> Option<ScriptedRules> {
        all()
            .into_iter()
            .find(|ruleset| ruleset.name.eq_ignore_ascii_case(id))
    }
}
