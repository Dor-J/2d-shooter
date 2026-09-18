//! When a bot shoots, reloads, throws, and which weapon it reaches for.

use super::perception::{Awareness, Contact};
use super::profile::{BotProfile, Difficulty};
use crate::modes::modifiers::advance::Unlocked;
use crate::weapons::{WeaponKind, WeaponTable};
use crate::Vec2;

/// How close a bot has to be before it prefers a melee weapon to a gun.
pub const MELEE_RANGE: f32 = 60.0;

/// How close is too close to throw a grenade at somebody.
pub const GRENADE_MIN_RANGE: f32 = 120.0;

/// How far a grenade is worth throwing at all.
pub const GRENADE_MAX_RANGE: f32 = 460.0;

/// What a bot wants to do with its weapon this tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Trigger {
    pub fire: bool,
    pub reload: bool,
    pub throw_grenade: bool,
    /// The weapon slot it wants to be holding.
    pub weapon: Option<u8>,
}

/// Whether a bot should be shooting at all.
///
/// It needs a target, to have watched it long enough to react, and to actually have something
/// loaded. A bot that fires an empty magazine forever is a bot that never reloads.
pub fn should_fire(
    awareness: &Awareness,
    difficulty: Difficulty,
    ammo: u16,
    reloading: bool,
) -> bool {
    awareness.has_reacted(difficulty) && !awareness.is_hunting_a_memory() && ammo > 0 && !reloading
}

/// Whether a bot should reload now.
///
/// Empty always; low and out of contact when there is a lull, because reloading in the middle of a
/// fight is how a bot dies holding a full magazine.
pub fn should_reload(awareness: &Awareness, ammo: u16, capacity: u16, reloading: bool) -> bool {
    if reloading {
        return false;
    }
    if ammo == 0 {
        return true;
    }
    let low = capacity / 3;
    ammo <= low && awareness.target.is_none()
}

/// Whether a bot should throw a grenade at its target.
///
/// Not too close, or it blows itself up; not too far, or it wastes them. `roll` is a deterministic
/// value from the simulation's own RNG, so a replay throws the same grenades.
pub fn should_throw_grenade(
    profile: &BotProfile,
    awareness: &Awareness,
    grenades: u8,
    distance: f32,
    roll: u32,
) -> bool {
    if grenades == 0 || !awareness.has_reacted(profile.difficulty) {
        return false;
    }
    if !(GRENADE_MIN_RANGE..=GRENADE_MAX_RANGE).contains(&distance) {
        return false;
    }
    roll % profile.difficulty.grenade_reluctance() == 0
}

/// The weapon slot a bot wants to be holding.
///
/// Close enough and a blade beats a rifle; otherwise it reaches for its favourite, and falls back
/// to whatever it has actually unlocked.
pub fn preferred_weapon(
    profile: &BotProfile,
    table: &WeaponTable,
    unlocked: Unlocked,
    distance_to_target: Option<f32>,
) -> Option<u8> {
    let wanted = match distance_to_target {
        Some(distance) if distance <= MELEE_RANGE => WeaponKind::CombatKnife,
        _ => profile.favourite_weapon,
    };
    let _ = table;
    if unlocked.has(wanted) {
        return wanted.slot();
    }
    unlocked.available().next().and_then(WeaponKind::slot)
}

/// Whether a bot should be walking towards its target or keeping its distance.
///
/// A shotgun wants to be close; a Barrett does not. Aggression tips the balance.
pub fn wants_to_close(profile: &BotProfile, holding: WeaponKind, distance: f32) -> bool {
    let preferred = match holding {
        WeaponKind::Spas12 | WeaponKind::CombatKnife | WeaponKind::Chainsaw => 80.0,
        WeaponKind::Barrett | WeaponKind::Ruger77 => 420.0,
        _ => 240.0,
    };
    let eagerness = f32::from(profile.aggression) / 100.0;
    distance > preferred * (1.0 - eagerness * 0.4)
}

/// Where a bot should aim, given what it can see.
pub fn aim_at(awareness: &Awareness, target: Option<Contact>, fallback: Vec2) -> Vec2 {
    match target {
        Some(contact) => contact.pos,
        None if awareness.target.is_some() => awareness.last_seen,
        None => fallback,
    }
}
