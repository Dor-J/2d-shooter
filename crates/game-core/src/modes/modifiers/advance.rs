//! Advance: you start with almost nothing and earn your way into the armoury.
//!
//! Follows OpenSoldat `shared/mechanics/Sprites.pas` at the commit pinned in
//! `docs/parity/reference-lock.md`: every `sv_advancemode_amount` kills unlocks a primary the
//! player does not have, and every that many deaths takes one back off them again.

use serde::{Deserialize, Serialize};

use crate::weapons::{WeaponKind, SELECTABLE_WEAPONS};
use crate::SimRng;

/// The number of primaries a player can work through.
pub const PRIMARY_COUNT: usize = 10;

/// How Advance is configured.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvanceConfig {
    /// Kills needed for the next unlock, and deaths that cost one.
    pub kills_per_unlock: u32,
}

impl Default for AdvanceConfig {
    fn default() -> Self {
        Self {
            kills_per_unlock: 2,
        }
    }
}

/// One player's unlocked primaries, as a bit per weapon.
///
/// A set rather than a list, because what matters is only whether a weapon is available; the order
/// a player earned them in is not a rule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unlocked(pub u16);

impl Unlocked {
    /// What a player starts an Advance match with: nothing but their secondary.
    pub const fn starting() -> Self {
        Self(0)
    }

    /// Everything unlocked, which is what a non-Advance match effectively is.
    pub const fn everything() -> Self {
        Self((1 << PRIMARY_COUNT) - 1)
    }

    fn bit(slot: u8) -> u16 {
        1u16 << slot.min(15)
    }

    pub fn has(self, kind: WeaponKind) -> bool {
        match kind.slot() {
            Some(slot) if usize::from(slot) < PRIMARY_COUNT => self.0 & Self::bit(slot) != 0,
            // Secondaries are never locked: a player is never left with nothing to shoot with.
            _ => true,
        }
    }

    pub fn grant(&mut self, kind: WeaponKind) {
        if let Some(slot) = kind
            .slot()
            .filter(|slot| usize::from(*slot) < PRIMARY_COUNT)
        {
            self.0 |= Self::bit(slot);
        }
    }

    pub fn revoke(&mut self, kind: WeaponKind) {
        if let Some(slot) = kind
            .slot()
            .filter(|slot| usize::from(*slot) < PRIMARY_COUNT)
        {
            self.0 &= !Self::bit(slot);
        }
    }

    /// How many primaries this player has earned.
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// The primaries they may pick from right now, secondaries included.
    pub fn available(self) -> impl Iterator<Item = WeaponKind> {
        SELECTABLE_WEAPONS
            .into_iter()
            .filter(move |kind| self.has(*kind))
    }

    fn locked(self) -> Vec<WeaponKind> {
        SELECTABLE_WEAPONS
            .into_iter()
            .take(PRIMARY_COUNT)
            .filter(|kind| !self.has(*kind))
            .collect()
    }

    fn earned(self) -> Vec<WeaponKind> {
        SELECTABLE_WEAPONS
            .into_iter()
            .take(PRIMARY_COUNT)
            .filter(|kind| self.has(*kind))
            .collect()
    }
}

/// Grants a random primary the player does not yet have.
///
/// Deterministic for a given RNG state, which is what lets a replay agree with the match it
/// records. Returns the weapon that was unlocked, or nothing when they already have them all.
pub fn unlock_one(unlocked: &mut Unlocked, rng: &mut SimRng) -> Option<WeaponKind> {
    let candidates = unlocked.locked();
    if candidates.is_empty() {
        return None;
    }
    let index = (rng.next_u64() as usize) % candidates.len();
    let kind = candidates[index];
    unlocked.grant(kind);
    Some(kind)
}

/// Takes a random primary back off a player who is dying too often.
pub fn revoke_one(unlocked: &mut Unlocked, rng: &mut SimRng) -> Option<WeaponKind> {
    let candidates = unlocked.earned();
    if candidates.is_empty() {
        return None;
    }
    let index = (rng.next_u64() as usize) % candidates.len();
    let kind = candidates[index];
    unlocked.revoke(kind);
    Some(kind)
}

/// Whether this kill count has earned the player an unlock.
pub fn kill_earns_unlock(config: &AdvanceConfig, kills: u32) -> bool {
    let step = config.kills_per_unlock.max(1);
    kills > 0 && kills % step == 0
}

/// Whether this death count costs the player a weapon.
pub fn death_costs_unlock(config: &AdvanceConfig, deaths: u32) -> bool {
    let step = config.kills_per_unlock.max(1);
    deaths > 0 && deaths % step == 0
}

/// The weapon a player may actually spawn with, given what they have unlocked.
///
/// A locked choice falls back to the first weapon they do have rather than being refused, so a
/// player whose weapon was just taken away still spawns holding something.
pub fn resolve_choice(advance: bool, unlocked: Unlocked, wanted: WeaponKind) -> WeaponKind {
    if !advance || unlocked.has(wanted) {
        return wanted;
    }
    unlocked.available().next().unwrap_or(WeaponKind::Ussocom)
}

/// Clears every player's progress, which happens between matches and between maps.
pub fn reset() -> Unlocked {
    Unlocked::starting()
}
