//! Bonus kits and the timed effects they grant.
//!
//! Follows OpenSoldat `shared/mechanics/Things.pas` (the kit pickup branches),
//! `shared/mechanics/Sprites.pas` (`TSprite.HealthHit`, the bonus timer), and the `*BONUSTIME`,
//! `*BONUS_RANDOM`, `DEFAULTVEST`, `PREDATORALPHA`, and `KIT_RADIUS` constants at the commit
//! pinned in `docs/parity/reference-lock.md`.

use serde::{Deserialize, Serialize};

/// How long each timed bonus lasts, in ticks.
pub const FLAME_GOD_TICKS: u16 = 600;
pub const PREDATOR_TICKS: u16 = 1500;
pub const BERSERKER_TICKS: u16 = 900;

/// The armor a vest grants, which is about another full health bar.
pub const VEST_ARMOR: i32 = 100;

/// Cluster grenades a cluster kit hands over.
pub const CLUSTER_GRENADES: u8 = 3;

/// How visible a Predator is, out of 255. Not zero: a careful eye can still catch the shimmer.
pub const PREDATOR_ALPHA: u8 = 5;

/// How much harder a Berserker hits.
pub const BERSERKER_DAMAGE_MULTIPLIER: f32 = 4.0;

/// How close a player has to be to pick a kit up.
pub const KIT_RADIUS: f32 = 12.0;

/// Which kit this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum KitKind {
    /// Back to full health.
    Medic,
    /// Grenades back to the configured maximum.
    Grenades,
    /// Three cluster grenades in place of frags.
    ClusterGrenades,
    /// Another health bar's worth of armor.
    Vest,
    /// A flamethrower and nothing can hurt you.
    FlameGod,
    /// Four times the damage.
    Berserker,
    /// Near invisibility.
    Predator,
}

impl KitKind {
    /// Every kit, in a fixed order so a spawn table is deterministic.
    pub const ALL: [KitKind; 7] = [
        Self::Medic,
        Self::Grenades,
        Self::ClusterGrenades,
        Self::Vest,
        Self::FlameGod,
        Self::Berserker,
        Self::Predator,
    ];

    /// The timed effect this kit grants, if it grants one.
    ///
    /// Medic, grenades, cluster, and vest all change something and are gone; the other three put a
    /// clock on the player.
    pub const fn effect(self) -> Option<BonusEffect> {
        match self {
            Self::FlameGod => Some(BonusEffect::FlameGod),
            Self::Berserker => Some(BonusEffect::Berserker),
            Self::Predator => Some(BonusEffect::Predator),
            _ => None,
        }
    }

    /// What players call it.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Medic => "Medical Kit",
            Self::Grenades => "Grenades Kit",
            Self::ClusterGrenades => "Cluster Grenades",
            Self::Vest => "Bulletproof Vest",
            Self::FlameGod => "Flame God",
            Self::Berserker => "Berserker",
            Self::Predator => "Predator",
        }
    }

    /// How rare this kit is: one chance in this many on each spawn roll.
    ///
    /// Medic and grenade kits are common enough not to be rolled for at all; the rest match the
    /// upstream `*BONUS_RANDOM` constants.
    pub const fn rarity(self) -> u32 {
        match self {
            Self::Medic | Self::Grenades => 1,
            Self::ClusterGrenades | Self::Vest | Self::Berserker => 4,
            Self::FlameGod | Self::Predator => 5,
        }
    }
}

/// A timed effect a player is under.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum BonusEffect {
    FlameGod,
    Berserker,
    Predator,
}

impl BonusEffect {
    /// How long it runs for.
    pub const fn duration(self) -> u16 {
        match self {
            Self::FlameGod => FLAME_GOD_TICKS,
            Self::Berserker => BERSERKER_TICKS,
            Self::Predator => PREDATOR_TICKS,
        }
    }

    /// Whether nothing can hurt the player while it lasts.
    pub const fn grants_invulnerability(self) -> bool {
        matches!(self, Self::FlameGod)
    }

    /// How much the player's own damage is multiplied by.
    pub fn damage_multiplier(self) -> f32 {
        match self {
            Self::Berserker => BERSERKER_DAMAGE_MULTIPLIER,
            _ => 1.0,
        }
    }

    /// How visible the player is, out of 255.
    pub const fn alpha(self) -> u8 {
        match self {
            Self::Predator => PREDATOR_ALPHA,
            _ => 255,
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::FlameGod => "Flame God",
            Self::Berserker => "Berserker",
            Self::Predator => "Predator",
        }
    }
}

/// The effect a player is under and how long is left of it.
///
/// A player has at most one: picking up a second kit while one is running is refused rather than
/// stacked, which is what keeps a lucky run from making somebody untouchable and invisible at
/// once.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimedEffect {
    pub active: Option<BonusEffect>,
    /// Ticks left, which the HUD counts down.
    pub ticks_left: u16,
}

impl TimedEffect {
    /// Starts an effect, unless one is already running.
    ///
    /// Returns whether it took: a refused pickup leaves the kit on the ground for somebody else.
    pub fn activate(&mut self, effect: BonusEffect) -> bool {
        if self.active.is_some() {
            return false;
        }
        self.active = Some(effect);
        self.ticks_left = effect.duration();
        true
    }

    /// Advances the clock by one tick, and reports the effect that just ended.
    pub fn step(&mut self) -> Option<BonusEffect> {
        let active = self.active?;
        self.ticks_left = self.ticks_left.saturating_sub(1);
        if self.ticks_left == 0 {
            self.active = None;
            return Some(active);
        }
        None
    }

    /// Ends whatever is running, which is what dying does.
    pub fn clear(&mut self) {
        self.active = None;
        self.ticks_left = 0;
    }

    pub const fn is(&self, effect: BonusEffect) -> bool {
        matches!(self.active, Some(active) if matches!(
            (active, effect),
            (BonusEffect::FlameGod, BonusEffect::FlameGod)
                | (BonusEffect::Berserker, BonusEffect::Berserker)
                | (BonusEffect::Predator, BonusEffect::Predator)
        ))
    }

    /// Whether nothing can hurt this player right now.
    pub fn is_invulnerable(&self) -> bool {
        self.active.is_some_and(BonusEffect::grants_invulnerability)
    }

    /// How much this player's outgoing damage is multiplied by.
    pub fn damage_multiplier(&self) -> f32 {
        self.active.map_or(1.0, BonusEffect::damage_multiplier)
    }

    /// How visible this player is, out of 255.
    pub fn alpha(&self) -> u8 {
        self.active.map_or(255, BonusEffect::alpha)
    }

    /// Seconds left, for the HUD countdown.
    pub fn seconds_left(&self) -> u16 {
        self.ticks_left.div_ceil(60)
    }
}

/// How visible a Predator is once they are bleeding.
///
/// Blood gives them away: a wounded Predator is not the same problem as a fresh one, which is the
/// counterplay the mode is built around.
pub fn predator_alpha(effect: &TimedEffect, health: i32, max_health: i32) -> u8 {
    if !effect.is(BonusEffect::Predator) {
        return 255;
    }
    let hurt = max_health.max(1) / 2;
    if health <= hurt {
        // Bleeding shows through the shimmer, and the worse the wound the more it shows.
        let missing = (max_health - health).max(0) as f32 / max_health.max(1) as f32;
        return PREDATOR_ALPHA.saturating_add((missing * 90.0) as u8);
    }
    PREDATOR_ALPHA
}

/// Whether a Predator still makes a noise.
///
/// They do: invisibility is not silence, and the sound is how a good player finds them.
pub const fn predator_is_audible() -> bool {
    true
}

/// How a server is configured to hand out kits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BonusConfig {
    /// Ticks between spawn rolls. Zero switches kits off entirely.
    pub frequency_ticks: u32,
    /// Ticks a kit lies on the ground before it disappears.
    pub lifetime_ticks: u16,
    /// Which kits this server hands out at all.
    pub medic: bool,
    pub grenades: bool,
    pub cluster: bool,
    pub vest: bool,
    pub flame_god: bool,
    pub berserker: bool,
    pub predator: bool,
    /// The most kits that may be on the map at once.
    pub max_on_map: usize,
}

impl Default for BonusConfig {
    fn default() -> Self {
        Self {
            frequency_ticks: 60 * 20,
            lifetime_ticks: 60 * 25,
            medic: true,
            grenades: true,
            cluster: true,
            vest: true,
            flame_god: true,
            berserker: true,
            predator: true,
            max_on_map: 4,
        }
    }
}

impl BonusConfig {
    /// A server with every kit switched off.
    pub const fn none() -> Self {
        Self {
            frequency_ticks: 0,
            lifetime_ticks: 60 * 25,
            medic: false,
            grenades: false,
            cluster: false,
            vest: false,
            flame_god: false,
            berserker: false,
            predator: false,
            max_on_map: 0,
        }
    }

    /// Whether this kit is switched on.
    pub const fn allows(&self, kind: KitKind) -> bool {
        match kind {
            KitKind::Medic => self.medic,
            KitKind::Grenades => self.grenades,
            KitKind::ClusterGrenades => self.cluster,
            KitKind::Vest => self.vest,
            KitKind::FlameGod => self.flame_god,
            KitKind::Berserker => self.berserker,
            KitKind::Predator => self.predator,
        }
    }

    /// The kits this server may spawn, in a fixed order.
    pub fn enabled(&self) -> Vec<KitKind> {
        KitKind::ALL
            .into_iter()
            .filter(|kind| self.allows(*kind))
            .collect()
    }

    /// Whether kits are switched on at all.
    pub fn any(&self) -> bool {
        self.frequency_ticks > 0 && self.max_on_map > 0 && !self.enabled().is_empty()
    }
}
