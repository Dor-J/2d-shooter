use crate::collision::BodyRegion;
use crate::Vec2;
use serde::{Deserialize, Serialize};

/// What produced a unit of damage. Death messages and statistics read this rather than guessing
/// from the projectile that happened to land.
/// Ordered so a per-cause breakdown keeps a stable order on the scoreboard rather than reshuffling
/// itself between snapshots.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageCause {
    Bullet,
    Pellet,
    Explosion,
    Melee,
    Fall,
    Bleeding,
    Deadly,
}

impl DamageCause {
    /// Bleeding and falls are consequences of earlier events, so they never start new bleeding.
    pub fn draws_blood(self) -> bool {
        matches!(
            self,
            Self::Bullet | Self::Pellet | Self::Explosion | Self::Melee
        )
    }
}

/// One authoritative unit of damage. Everything that hurts a player builds one of these, so
/// attribution, armor, bleeding, and the kill feed all read the same record.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DamageEvent {
    pub attacker: Option<u32>,
    pub target: u32,
    pub amount: i32,
    pub region: BodyRegion,
    pub cause: DamageCause,
    pub direction: Vec2,
    /// When true, `amount` already includes the weapon hitbox modifier, so apply_damage must not
    /// apply the global region table a second time.
    #[serde(default)]
    pub pre_scaled: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct DamageConfig {
    /// The most grenades a player may carry, from `sv_maxgrenades`.
    #[serde(default = "default_max_grenades")]
    pub max_grenades: u8,
    pub head_multiplier: f32,
    pub chest_multiplier: f32,
    pub legs_multiplier: f32,
    /// Share of incoming damage armor takes while it lasts.
    pub armor_absorption: f32,
    /// Damage in one hit that is enough to open a wound.
    pub bleed_threshold: i32,
    pub bleed_ticks: u16,
    pub bleed_interval: u16,
    pub bleed_damage: i32,
    /// How long an attacker keeps credit for a kill and an assist after landing damage.
    pub attribution_ticks: u64,
    pub multi_kill_ticks: u64,
    /// Downward impact speed a landing must exceed before it costs health.
    pub fall_damage_speed: f32,
    pub fall_damage_per_speed: f32,
    /// Health lost in one hit that tears the body apart instead of leaving a corpse.
    pub gib_damage: i32,
}

/// Soldat's default grenade allowance.
fn default_max_grenades() -> u8 {
    2
}

impl DamageConfig {
    pub const fn soldat_default() -> Self {
        Self {
            max_grenades: 2,
            head_multiplier: 1.5,
            chest_multiplier: 1.0,
            legs_multiplier: 0.75,
            armor_absorption: 0.5,
            bleed_threshold: 25,
            bleed_ticks: 600,
            bleed_interval: 20,
            bleed_damage: 1,
            attribution_ticks: 600,
            multi_kill_ticks: 240,
            fall_damage_speed: 520.0,
            fall_damage_per_speed: 0.16,
            gib_damage: 140,
        }
    }
}

impl Default for DamageConfig {
    fn default() -> Self {
        Self::soldat_default()
    }
}

pub fn region_multiplier(config: &DamageConfig, region: BodyRegion) -> f32 {
    match region {
        BodyRegion::Head => config.head_multiplier,
        BodyRegion::Chest => config.chest_multiplier,
        BodyRegion::Legs => config.legs_multiplier,
    }
}

/// How one hit splits between the vest and the body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Absorbed {
    pub armor: i32,
    pub health: i32,
}

pub fn absorb(config: &DamageConfig, armor: i32, amount: i32) -> Absorbed {
    if amount <= 0 {
        return Absorbed {
            armor: 0,
            health: 0,
        };
    }
    let share = config.armor_absorption.clamp(0.0, 1.0);
    let wanted = ((amount as f32) * share).round() as i32;
    let taken = wanted.min(armor.max(0)).max(0);
    Absorbed {
        armor: taken,
        health: amount - taken,
    }
}

/// An open wound: a fixed amount of damage on a fixed cadence, credited to whoever opened it.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Bleed {
    pub attacker: Option<u32>,
    pub ticks_left: u16,
    pub interval: u16,
    pub countdown: u16,
    pub damage: i32,
}

impl Bleed {
    pub fn open(config: &DamageConfig, attacker: Option<u32>) -> Self {
        Self {
            attacker,
            ticks_left: config.bleed_ticks,
            interval: config.bleed_interval.max(1),
            countdown: config.bleed_interval.max(1),
            damage: config.bleed_damage.max(1),
        }
    }

    /// Advances one tick and reports the damage to apply now, if any.
    pub fn tick(&mut self) -> Option<i32> {
        if self.ticks_left == 0 {
            return None;
        }
        self.ticks_left -= 1;
        self.countdown = self.countdown.saturating_sub(1);
        if self.countdown > 0 {
            return None;
        }
        self.countdown = self.interval;
        Some(self.damage)
    }

    pub fn finished(&self) -> bool {
        self.ticks_left == 0
    }
}

/// Who hurt this player recently. Bounded and ordered by player id so replays stay deterministic.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Attribution {
    pub player: u32,
    pub tick: u64,
    pub damage: i32,
}

pub const MAX_ATTRIBUTIONS: usize = 8;

pub fn record_attribution(log: &mut Vec<Attribution>, player: u32, tick: u64, damage: i32) {
    if let Some(existing) = log.iter_mut().find(|entry| entry.player == player) {
        existing.tick = tick;
        existing.damage = existing.damage.saturating_add(damage);
        return;
    }
    log.push(Attribution {
        player,
        tick,
        damage,
    });
    log.sort_by_key(|entry| entry.player);
    if log.len() > MAX_ATTRIBUTIONS {
        // Drop the stalest contributor rather than the lowest id.
        let oldest = log
            .iter()
            .enumerate()
            .min_by_key(|(_, entry)| entry.tick)
            .map(|(index, _)| index);
        if let Some(index) = oldest {
            log.remove(index);
        }
    }
}

/// Everyone except the killer who damaged the target inside the attribution window.
pub fn assists(
    log: &[Attribution],
    killer: Option<u32>,
    target: u32,
    tick: u64,
    window: u64,
) -> Vec<u32> {
    log.iter()
        .filter(|entry| Some(entry.player) != killer && entry.player != target)
        .filter(|entry| tick.saturating_sub(entry.tick) <= window)
        .map(|entry| entry.player)
        .collect()
}
