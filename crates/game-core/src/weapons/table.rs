//! The active weapon ruleset: twenty configured weapons plus the three derived from them.

use serde::{Deserialize, Serialize};

use super::config::{
    BulletStyle, NoCollision, WeaponDef, WeaponKind, WeaponLimits, ALL_WEAPONS, CONFIGURED_WEAPONS,
    HASHED_KEYS, SELECTABLE_WEAPONS, WEAPON_KEYS,
};
use super::loader::{parse_ini, WeaponConfigError};
use super::tables::{RawWeapon, DEFAULT_VERSION, NORMAL, NORMAL_NAME, REALISTIC, REALISTIC_NAME};

/// One validated weapon mod.
///
/// A table is immutable once built. Every consumer — simulation, server, renderer, HUD, audio —
/// reads the same definitions, so there is no second copy of the numbers anywhere.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "WireTable", into = "WireTable")]
pub struct WeaponTable {
    name: String,
    version: String,
    defs: Vec<WeaponDef>,
}

impl Default for WeaponTable {
    fn default() -> Self {
        Self::normal()
    }
}

impl WeaponTable {
    /// The default normal-mode table.
    pub fn normal() -> Self {
        Self::from_raw(NORMAL_NAME, DEFAULT_VERSION, &NORMAL)
    }

    /// The default Realistic-mode table.
    pub fn realistic() -> Self {
        Self::from_raw(REALISTIC_NAME, DEFAULT_VERSION, &REALISTIC)
    }

    /// The default table for a ruleset.
    pub fn default_for(realistic: bool) -> Self {
        if realistic {
            Self::realistic()
        } else {
            Self::normal()
        }
    }

    /// Reads a weapon mod from `weapons.ini`-style text.
    pub fn parse_ini(source: &str, limits: &WeaponLimits) -> Result<Self, WeaponConfigError> {
        parse_ini(source, limits)
    }

    /// Re-checks every bound the loader already enforced, so a table that arrived on the wire
    /// cannot skip validation.
    pub fn validate(&self, limits: &WeaponLimits) -> Result<(), WeaponConfigError> {
        Self::parse_ini(&self.to_ini(), limits).map(|_| ())
    }

    /// Magazine sizes for the fourteen selectable slots, in selection order.
    pub fn selectable_ammo(&self) -> [u16; 14] {
        let mut ammo = [0u16; 14];
        for (index, kind) in SELECTABLE_WEAPONS.iter().enumerate() {
            ammo[index] = u16::from(self.get(*kind).ammo);
        }
        ammo
    }

    /// Display names for the fourteen selectable slots, for HUD and audio that must not keep
    /// their own copy of the numbers.
    pub fn selectable_names(&self) -> Vec<String> {
        SELECTABLE_WEAPONS
            .iter()
            .map(|kind| kind.display_name().to_string())
            .collect()
    }

    /// Builds a table from the twenty configured definitions, deriving the other three.
    pub(super) fn from_configured(
        name: String,
        version: String,
        configured: Vec<WeaponDef>,
    ) -> Self {
        debug_assert_eq!(configured.len(), CONFIGURED_WEAPONS.len());
        let mut defs = configured;

        // `BuildWeapons` copies every field but the bullet style from the parent section.
        let grenade = defs[WeaponKind::FragGrenade.index()];
        let cluster_grenade = WeaponDef::inherit(
            &grenade,
            WeaponKind::ClusterGrenade,
            BulletStyle::ClusterGrenade,
        );
        let cluster =
            WeaponDef::inherit(&cluster_grenade, WeaponKind::Cluster, BulletStyle::Cluster);
        let knife = defs[WeaponKind::CombatKnife.index()];
        let thrown = WeaponDef::inherit(&knife, WeaponKind::ThrownKnife, BulletStyle::ThrownKnife);

        defs.push(cluster_grenade);
        defs.push(cluster);
        defs.push(thrown);
        debug_assert_eq!(defs.len(), ALL_WEAPONS.len());
        Self {
            name,
            version,
            defs,
        }
    }

    fn from_raw(name: &str, version: &str, raw: &[RawWeapon; 20]) -> Self {
        let configured = CONFIGURED_WEAPONS
            .into_iter()
            .zip(raw.iter())
            .map(|(kind, values)| WeaponDef {
                kind,
                damage: values.damage,
                fire_interval: values.fire_interval,
                ammo: values.ammo,
                reload_time: values.reload_time,
                speed: values.speed,
                bullet_style: BulletStyle::from_code(values.bullet_style)
                    .expect("generated tables only contain known bullet styles"),
                start_up_time: values.start_up_time,
                bink: values.bink,
                movement_acc: values.movement_acc,
                bullet_spread: values.bullet_spread,
                recoil: values.recoil,
                push: values.push,
                inherited_velocity: values.inherited_velocity,
                modifier_head: values.modifier_head,
                modifier_chest: values.modifier_chest,
                modifier_legs: values.modifier_legs,
                no_collision: NoCollision::NONE,
            })
            .collect();
        Self::from_configured(name.to_string(), version.to_string(), configured)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    /// Whether this is the Realistic table. Derived from the name so a mod can declare either.
    pub fn is_realistic(&self) -> bool {
        self.name == REALISTIC_NAME
    }

    /// The definition for one weapon.
    pub fn get(&self, kind: WeaponKind) -> &WeaponDef {
        &self.defs[kind.index()]
    }

    /// The definition a player's selection slot refers to.
    pub fn for_slot(&self, slot: u8) -> &WeaponDef {
        self.get(WeaponKind::from_slot(slot))
    }

    /// Every definition, in canonical order.
    pub fn definitions(&self) -> &[WeaponDef] {
        &self.defs
    }

    /// The djb2 checksum `CreateWMChecksum` produces: thirteen fields over the twenty configured
    /// weapons, with the hitbox modifiers deliberately left out so older clients still agree.
    pub fn canonical_hash(&self) -> u32 {
        let mut hash: u32 = 5381;
        for kind in CONFIGURED_WEAPONS {
            let def = self.get(kind);
            for key in HASHED_KEYS {
                let scaled = round_half_to_even(1000.0 * hashed_field(def, key));
                hash = hash.wrapping_add(hash << 5).wrapping_add(scaled as u32);
            }
        }
        hash
    }

    /// The weapon sections, ready to be written back out or re-parsed.
    pub fn to_ini_body(&self) -> String {
        let mut out = String::new();
        for kind in CONFIGURED_WEAPONS {
            let def = self.get(kind);
            out.push_str(&format!("\n[{}]\n", kind.ini_name()));
            for key in WEAPON_KEYS {
                out.push_str(&format!("{key}={}\n", format_value(def, key)));
            }
            if def.no_collision != NoCollision::NONE {
                out.push_str(&format!("NoCollision={}\n", def.no_collision.0));
            }
        }
        out
    }

    /// The whole table as `weapons.ini` text, including its `[Info]` header.
    pub fn to_ini(&self) -> String {
        format!(
            "[Info]\nName={}\nVersion={}\n{}",
            self.name,
            self.version,
            self.to_ini_body()
        )
    }
}

/// Free Pascal's `Round` is round-half-to-even, and the checksum depends on it: XM214 Minigun's
/// `MovementAcc` of 0.0625 scales to exactly 62.5.
fn round_half_to_even(value: f64) -> i64 {
    let floor = value.floor();
    let fraction = value - floor;
    let floor = floor as i64;
    if fraction > 0.5 || (fraction == 0.5 && floor % 2 != 0) {
        floor + 1
    } else {
        floor
    }
}

fn hashed_field(def: &WeaponDef, key: &str) -> f64 {
    match key {
        "Damage" => f64::from(def.damage),
        "FireInterval" => f64::from(def.fire_interval),
        "Ammo" => f64::from(def.ammo),
        "ReloadTime" => f64::from(def.reload_time),
        "Speed" => f64::from(def.speed),
        "BulletStyle" => f64::from(def.bullet_style.code()),
        "StartUpTime" => f64::from(def.start_up_time),
        "Bink" => f64::from(def.bink),
        "MovementAcc" => f64::from(def.movement_acc),
        "BulletSpread" => f64::from(def.bullet_spread),
        "Recoil" => f64::from(def.recoil),
        "Push" => f64::from(def.push),
        "InheritedVelocity" => f64::from(def.inherited_velocity),
        other => unreachable!("{other} is not a hashed field"),
    }
}

/// Writes a value the way the upstream tables do, so a round trip is byte-stable.
fn format_value(def: &WeaponDef, key: &str) -> String {
    match key {
        "FireInterval" => def.fire_interval.to_string(),
        "Ammo" => def.ammo.to_string(),
        "ReloadTime" => def.reload_time.to_string(),
        "BulletStyle" => def.bullet_style.code().to_string(),
        "StartUpTime" => def.start_up_time.to_string(),
        "Bink" => def.bink.to_string(),
        "Recoil" => def.recoil.to_string(),
        "ModifierHead" => def.modifier_head.to_string(),
        "ModifierChest" => def.modifier_chest.to_string(),
        "ModifierLegs" => def.modifier_legs.to_string(),
        "Damage" => def.damage.to_string(),
        "Speed" => def.speed.to_string(),
        "MovementAcc" => def.movement_acc.to_string(),
        "BulletSpread" => def.bullet_spread.to_string(),
        "Push" => def.push.to_string(),
        "InheritedVelocity" => def.inherited_velocity.to_string(),
        other => unreachable!("{other} is not a weapon key"),
    }
}

/// The wire form. A default table travels as three short fields; only a genuine mod pays for the
/// full definition list, which keeps it out of every 60 Hz snapshot.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct WireTable {
    name: String,
    version: String,
    hash: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    names: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    defs: Option<Vec<WeaponDef>>,
}

impl From<WeaponTable> for WireTable {
    fn from(table: WeaponTable) -> Self {
        let hash = table.canonical_hash();
        let is_default = [WeaponTable::normal(), WeaponTable::realistic()].contains(&table);
        Self {
            name: table.name.clone(),
            version: table.version.clone(),
            hash,
            names: table.selectable_names(),
            defs: if is_default {
                None
            } else {
                Some(table.defs.clone())
            },
        }
    }
}

impl From<WireTable> for WeaponTable {
    fn from(wire: WireTable) -> Self {
        match wire.defs {
            Some(defs) if defs.len() == ALL_WEAPONS.len() => Self {
                name: wire.name,
                version: wire.version,
                defs,
            },
            // A default table is named, never listed; an unusable list falls back to the default
            // rather than leaving the simulation with a partial ruleset.
            _ => Self::default_for(wire.name == REALISTIC_NAME),
        }
    }
}
