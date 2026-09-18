//! Where a player comes back, and whether a map can host a mode at all.

use serde::{Deserialize, Serialize};

use super::objective::{FlagLayout, ObjectiveRules};
use super::rules::{ModeKind, ModeRules};
use super::team::{ALPHA, BRAVO};
use crate::MapSpawn;

/// Why a map cannot host a mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawnProblem {
    /// The map has no spawn points at all.
    NoSpawns,
    /// A team mode needs somewhere for this side to start.
    TeamHasNoSpawn { team: u8 },
    /// A free-for-all wants somewhere for everybody to start.
    NotEnoughSpawns { found: usize, needed: usize },
    /// A flag mode needs somewhere to put this flag's base.
    NoBaseForFlag { team: u8 },
}

/// How many usable spawns a mode wants before a map is considered playable.
pub const MINIMUM_FREE_FOR_ALL_SPAWNS: usize = 2;

/// Checks a map's spawns against a mode's needs.
///
/// This runs before a map is loaded, so a server refuses an unplayable rotation entry rather than
/// dropping players into a map with nowhere to stand.
pub fn validate(rules: &ModeRules, spawns: &[MapSpawn]) -> Result<(), SpawnProblem> {
    if spawns.is_empty() {
        return Err(SpawnProblem::NoSpawns);
    }
    if !rules.is_team_mode() {
        let usable = spawns.len();
        if usable < MINIMUM_FREE_FOR_ALL_SPAWNS {
            return Err(SpawnProblem::NotEnoughSpawns {
                found: usable,
                needed: MINIMUM_FREE_FOR_ALL_SPAWNS,
            });
        }
        return Ok(());
    }
    for team in [ALPHA, BRAVO] {
        let has_side = spawns
            .iter()
            .any(|spawn| spawn.team == team || spawn.team == 0);
        if !has_side {
            return Err(SpawnProblem::TeamHasNoSpawn { team });
        }
    }

    // A mode that gives each side a flag also needs somewhere to stand it, and a neutral spawn is
    // no use as a base: both flags would start on the same square.
    if ObjectiveRules::for_mode(rules.kind).layout() == FlagLayout::PerTeam {
        for team in [ALPHA, BRAVO] {
            if !spawns.iter().any(|spawn| spawn.team == team) {
                return Err(SpawnProblem::NoBaseForFlag { team });
            }
        }
    }
    Ok(())
}

/// The spawns a player on `team` may use under these rules.
pub fn usable<'a>(
    rules: &'a ModeRules,
    spawns: &'a [MapSpawn],
    team: u8,
) -> impl Iterator<Item = &'a MapSpawn> + 'a {
    spawns
        .iter()
        .filter(move |spawn| rules.spawn_is_valid(spawn.team, team))
}

/// How long a dead player waits before coming back.
///
/// Survival keeps them out until the round ends, which is why the answer is an option rather than
/// a number: there is a real difference between a long wait and no respawn at all.
pub fn respawn_delay(rules: &ModeRules, configured: u16) -> Option<u16> {
    if rules.modifiers.respawn_is_deferred() {
        return None;
    }
    Some(configured)
}

/// Whether this mode wants side-specific spawns on its maps.
pub const fn wants_team_spawns(kind: ModeKind) -> bool {
    kind.is_team_mode()
}
