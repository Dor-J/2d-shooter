//! The validated weapon definition and the two default tables.
//!
//! Field meanings, units, derived lifetimes, and the canonical checksum follow OpenSoldat
//! `shared/Weapons.pas` (`TGun`, `BULLET_STYLE_*`, `BuildWeapons`, `CreateWMChecksum`) at the
//! commit pinned in `docs/parity/reference-lock.md`. The numbers themselves are derived from the
//! two configuration tables recorded in `tests/fixtures/reference/weapons.json`.

use serde::{Deserialize, Serialize};

/// Bullet lifetimes in ticks, from `shared/Constants.pas`.
const BULLET_TIMEOUT: u16 = 60 * 7;
const GRENADE_TIMEOUT: u16 = 60 * 3;
const M2BULLET_TIMEOUT: u16 = 60;
const FLAMER_TIMEOUT: u16 = 32;
const MELEE_TIMEOUT: u16 = 1;

/// Explosion radii in world units, from `shared/mechanics/Sprites.pas`.
pub const M79_EXPLOSION_RADIUS: f32 = 64.0;
pub const FRAG_EXPLOSION_RADIUS: f32 = 85.0;
pub const CLUSTER_EXPLOSION_RADIUS: f32 = 35.0;

/// Every weapon the simulation knows about, in `shared/Weapons.pas` declaration order.
///
/// The first twenty have their own `weapons.ini` section. The last three are built from a parent
/// section by `BuildWeapons` and cannot be configured directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WeaponKind {
    DesertEagles,
    Mp5,
    Ak74,
    SteyrAug,
    Spas12,
    Ruger77,
    M79,
    Barrett,
    Minimi,
    Minigun,
    Ussocom,
    CombatKnife,
    Chainsaw,
    Law,
    FlamedArrows,
    RamboBow,
    Flamer,
    StationaryGun,
    Punch,
    FragGrenade,
    ClusterGrenade,
    Cluster,
    ThrownKnife,
}

/// Weapons that have a `weapons.ini` section, in canonical-hash order.
pub const CONFIGURED_WEAPONS: [WeaponKind; 20] = [
    WeaponKind::DesertEagles,
    WeaponKind::Mp5,
    WeaponKind::Ak74,
    WeaponKind::SteyrAug,
    WeaponKind::Spas12,
    WeaponKind::Ruger77,
    WeaponKind::M79,
    WeaponKind::Barrett,
    WeaponKind::Minimi,
    WeaponKind::Minigun,
    WeaponKind::Ussocom,
    WeaponKind::CombatKnife,
    WeaponKind::Chainsaw,
    WeaponKind::Law,
    WeaponKind::FlamedArrows,
    WeaponKind::RamboBow,
    WeaponKind::Flamer,
    WeaponKind::StationaryGun,
    WeaponKind::Punch,
    WeaponKind::FragGrenade,
];

/// Every weapon, including the three derived ones.
pub const ALL_WEAPONS: [WeaponKind; 23] = {
    let mut all = [WeaponKind::DesertEagles; 23];
    let mut index = 0;
    while index < CONFIGURED_WEAPONS.len() {
        all[index] = CONFIGURED_WEAPONS[index];
        index += 1;
    }
    all[20] = WeaponKind::ClusterGrenade;
    all[21] = WeaponKind::Cluster;
    all[22] = WeaponKind::ThrownKnife;
    all
};

/// The ten primary and four secondary weapons a player can select, in selection order.
pub const SELECTABLE_WEAPONS: [WeaponKind; 14] = [
    WeaponKind::DesertEagles,
    WeaponKind::Mp5,
    WeaponKind::Ak74,
    WeaponKind::SteyrAug,
    WeaponKind::Spas12,
    WeaponKind::Ruger77,
    WeaponKind::M79,
    WeaponKind::Barrett,
    WeaponKind::Minimi,
    WeaponKind::Minigun,
    WeaponKind::Ussocom,
    WeaponKind::CombatKnife,
    WeaponKind::Chainsaw,
    WeaponKind::Law,
];

impl WeaponKind {
    /// The `weapons.ini` section name, or an empty string for a derived weapon.
    pub const fn ini_name(self) -> &'static str {
        match self {
            Self::DesertEagles => "Desert Eagles",
            Self::Mp5 => "HK MP5",
            Self::Ak74 => "Ak-74",
            Self::SteyrAug => "Steyr AUG",
            Self::Spas12 => "Spas-12",
            Self::Ruger77 => "Ruger 77",
            Self::M79 => "M79",
            Self::Barrett => "Barret M82A1",
            Self::Minimi => "FN Minimi",
            Self::Minigun => "XM214 Minigun",
            Self::Ussocom => "USSOCOM",
            Self::CombatKnife => "Combat Knife",
            Self::Chainsaw => "Chainsaw",
            Self::Law => "M72 LAW",
            Self::FlamedArrows => "Flamed Arrows",
            Self::RamboBow => "Rambo Bow",
            Self::Flamer => "Flamer",
            Self::StationaryGun => "Stationary Gun",
            Self::Punch => "Punch",
            Self::FragGrenade => "Grenade",
            Self::ClusterGrenade | Self::Cluster | Self::ThrownKnife => "",
        }
    }

    /// The name players see, which differs from the ini section for six weapons.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Barrett => "Barrett M82A1",
            Self::FlamedArrows => "Flame Bow",
            Self::RamboBow => "Bow",
            Self::StationaryGun => "M2 MG",
            Self::Punch => "Hands",
            Self::Law => "LAW",
            Self::FragGrenade | Self::ClusterGrenade | Self::Cluster => "Frag Grenade",
            Self::ThrownKnife => "Combat Knife",
            other => other.ini_name(),
        }
    }

    /// The wire number upstream uses for this weapon.
    pub const fn num(self) -> u8 {
        match self {
            Self::DesertEagles => 1,
            Self::Mp5 => 2,
            Self::Ak74 => 3,
            Self::SteyrAug => 4,
            Self::Spas12 => 5,
            Self::Ruger77 => 6,
            Self::M79 => 7,
            Self::Barrett => 8,
            Self::Minimi => 9,
            Self::Minigun => 10,
            Self::Ussocom => 0,
            Self::CombatKnife => 11,
            Self::Chainsaw => 12,
            Self::Law => 13,
            Self::Flamer => 14,
            Self::RamboBow => 15,
            Self::FlamedArrows => 16,
            Self::StationaryGun => 30,
            Self::Punch => 255,
            Self::FragGrenade => 50,
            Self::ClusterGrenade => 51,
            Self::Cluster => 52,
            Self::ThrownKnife => 53,
        }
    }

    /// Position in the full 23-weapon table.
    pub const fn index(self) -> usize {
        match self {
            Self::DesertEagles => 0,
            Self::Mp5 => 1,
            Self::Ak74 => 2,
            Self::SteyrAug => 3,
            Self::Spas12 => 4,
            Self::Ruger77 => 5,
            Self::M79 => 6,
            Self::Barrett => 7,
            Self::Minimi => 8,
            Self::Minigun => 9,
            Self::Ussocom => 10,
            Self::CombatKnife => 11,
            Self::Chainsaw => 12,
            Self::Law => 13,
            Self::FlamedArrows => 14,
            Self::RamboBow => 15,
            Self::Flamer => 16,
            Self::StationaryGun => 17,
            Self::Punch => 18,
            Self::FragGrenade => 19,
            Self::ClusterGrenade => 20,
            Self::Cluster => 21,
            Self::ThrownKnife => 22,
        }
    }

    /// The weapons a `weapons.ini` file configures.
    pub fn configured() -> impl Iterator<Item = WeaponKind> {
        CONFIGURED_WEAPONS.into_iter()
    }

    /// The ten primary and four secondary weapons a player can pick.
    pub fn selectable() -> impl Iterator<Item = WeaponKind> {
        SELECTABLE_WEAPONS.into_iter()
    }

    /// The selectable weapon at `slot`, saturating at the last one.
    pub fn from_slot(slot: u8) -> WeaponKind {
        let slot = usize::from(slot).min(SELECTABLE_WEAPONS.len() - 1);
        SELECTABLE_WEAPONS[slot]
    }

    /// Where this weapon sits in the selection list, if it is selectable at all.
    pub fn slot(self) -> Option<u8> {
        SELECTABLE_WEAPONS
            .iter()
            .position(|kind| *kind == self)
            .and_then(|index| u8::try_from(index).ok())
    }

    /// True when the weapon is one of the ten primaries.
    pub fn is_primary(self) -> bool {
        self.slot().is_some_and(|slot| slot < 10)
    }

    /// True when the weapon is one of the four secondaries.
    pub fn is_secondary(self) -> bool {
        self.slot().is_some_and(|slot| (10..14).contains(&slot))
    }

    /// Whether `BuildWeapons` derives this weapon from another section.
    pub const fn parent(self) -> Option<WeaponKind> {
        match self {
            Self::ClusterGrenade => Some(Self::FragGrenade),
            Self::Cluster => Some(Self::ClusterGrenade),
            Self::ThrownKnife => Some(Self::CombatKnife),
            _ => None,
        }
    }

    const fn clip_reload(self) -> bool {
        matches!(
            self,
            Self::DesertEagles
                | Self::Mp5
                | Self::Ak74
                | Self::SteyrAug
                | Self::M79
                | Self::Barrett
                | Self::Minimi
                | Self::Ussocom
                | Self::Law
        )
    }

    /// Fire mode 2 is semi-automatic; 0 is fully automatic.
    const fn semi_automatic(self) -> bool {
        matches!(
            self,
            Self::DesertEagles | Self::Spas12 | Self::Ruger77 | Self::Barrett | Self::Ussocom
        )
    }
}

/// The bullet style that decides how a projectile behaves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulletStyle {
    Plain,
    FragGrenade,
    Shotgun,
    M79Grenade,
    Flame,
    Punch,
    Arrow,
    FlameArrow,
    ClusterGrenade,
    Cluster,
    Knife,
    Law,
    ThrownKnife,
    M2,
}

impl BulletStyle {
    /// The `BulletStyle=` value in `weapons.ini`.
    pub const fn code(self) -> u8 {
        match self {
            Self::Plain => 1,
            Self::FragGrenade => 2,
            Self::Shotgun => 3,
            Self::M79Grenade => 4,
            Self::Flame => 5,
            Self::Punch => 6,
            Self::Arrow => 7,
            Self::FlameArrow => 8,
            Self::ClusterGrenade => 9,
            Self::Cluster => 10,
            Self::Knife => 11,
            Self::Law => 12,
            Self::ThrownKnife => 13,
            Self::M2 => 14,
        }
    }

    /// The style for a `BulletStyle=` value, if it names one.
    pub const fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::Plain),
            2 => Some(Self::FragGrenade),
            3 => Some(Self::Shotgun),
            4 => Some(Self::M79Grenade),
            5 => Some(Self::Flame),
            6 => Some(Self::Punch),
            7 => Some(Self::Arrow),
            8 => Some(Self::FlameArrow),
            9 => Some(Self::ClusterGrenade),
            10 => Some(Self::Cluster),
            11 => Some(Self::Knife),
            12 => Some(Self::Law),
            13 => Some(Self::ThrownKnife),
            14 => Some(Self::M2),
            _ => None,
        }
    }

    /// Whether impact detonates instead of hitting one body.
    pub const fn is_explosive(self) -> bool {
        matches!(
            self,
            Self::FragGrenade | Self::M79Grenade | Self::Law | Self::ClusterGrenade | Self::Cluster
        )
    }

    /// Whether the projectile is a short-lived contact weapon.
    pub const fn is_melee(self) -> bool {
        matches!(self, Self::Knife | Self::Punch)
    }

    /// Whether gravity pulls the projectile down.
    pub const fn has_gravity(self) -> bool {
        matches!(
            self,
            Self::FragGrenade
                | Self::M79Grenade
                | Self::ClusterGrenade
                | Self::Cluster
                | Self::Arrow
                | Self::FlameArrow
                | Self::ThrownKnife
        )
    }

    /// Lifetime in ticks, as `BuildWeapons` assigns it.
    pub const fn timeout(self) -> u16 {
        match self {
            Self::FragGrenade | Self::ClusterGrenade => GRENADE_TIMEOUT,
            Self::Flame => FLAMER_TIMEOUT,
            Self::Punch | Self::Knife => MELEE_TIMEOUT,
            Self::M2 => M2BULLET_TIMEOUT,
            _ => BULLET_TIMEOUT,
        }
    }

    /// Blast radius in world units, or zero for a non-explosive style.
    pub const fn explosion_radius(self) -> f32 {
        match self {
            Self::FragGrenade | Self::ClusterGrenade | Self::Law => FRAG_EXPLOSION_RADIUS,
            Self::M79Grenade => M79_EXPLOSION_RADIUS,
            Self::Cluster => CLUSTER_EXPLOSION_RADIUS,
            _ => 0.0,
        }
    }

    /// How many projectiles one trigger pull creates.
    pub const fn pellets(self) -> u8 {
        match self {
            Self::Shotgun => 6,
            _ => 1,
        }
    }

    /// Whether a bullet loses half its power past 500 and again past 900 world units.
    pub const fn degrades_over_distance(self) -> bool {
        !matches!(
            self,
            Self::M79Grenade | Self::Law | Self::Knife | Self::ThrownKnife
        )
    }
}

/// Which collisions a weapon skips, matching the `WEAPON_NOCOLLISION_*` bit flags.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoCollision(pub u8);

impl NoCollision {
    pub const NONE: Self = Self(0);
    pub const ENEMY: u8 = 1 << 0;
    pub const TEAM: u8 = 1 << 1;
    pub const SELF: u8 = 1 << 2;
    pub const EXPLOSION_ENEMY: u8 = 1 << 3;
    pub const EXPLOSION_TEAM: u8 = 1 << 4;
    pub const EXPLOSION_SELF: u8 = 1 << 5;
    /// Every meaningful bit; anything else is rejected by the loader.
    pub const MASK: u8 = 0b0011_1111;

    pub const fn collides_with_enemy(self) -> bool {
        self.0 & Self::ENEMY == 0
    }
    pub const fn collides_with_team(self) -> bool {
        self.0 & Self::TEAM == 0
    }
    pub const fn collides_with_self(self) -> bool {
        self.0 & Self::SELF == 0
    }
    pub const fn explodes_on_enemy(self) -> bool {
        self.0 & Self::EXPLOSION_ENEMY == 0
    }
    pub const fn explodes_on_team(self) -> bool {
        self.0 & Self::EXPLOSION_TEAM == 0
    }
    pub const fn explodes_on_self(self) -> bool {
        self.0 & Self::EXPLOSION_SELF == 0
    }
}

/// One fully validated weapon.
///
/// The sixteen configurable fields are exactly the ones gap 4 lists. Everything else on this type
/// is derived from them, from the bullet style, or from the weapon's fixed upstream metadata, so a
/// mod can never leave a definition half-specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeaponDef {
    pub kind: WeaponKind,
    /// `HitMultiply` upstream: damage per unit of projectile speed, before hitbox modifiers.
    pub damage: f32,
    /// Ticks between shots.
    pub fire_interval: u16,
    /// Rounds in a full magazine.
    pub ammo: u8,
    /// Ticks for a full reload.
    pub reload_time: u16,
    /// Muzzle speed in world units per tick.
    pub speed: f32,
    pub bullet_style: BulletStyle,
    /// Ticks the trigger must be held before the first shot.
    pub start_up_time: u16,
    /// Positive binks whoever is hit; negative self-binks the shooter.
    pub bink: i16,
    /// How much movement spoils the aim.
    pub movement_acc: f32,
    /// Base spread while standing still.
    pub bullet_spread: f32,
    /// How far the cursor climbs per shot in a burst.
    pub recoil: u16,
    /// Bullet mass: how hard a hit shoves the body it lands on.
    pub push: f32,
    /// Fraction of the shooter's velocity the projectile inherits.
    pub inherited_velocity: f32,
    pub modifier_head: f32,
    pub modifier_chest: f32,
    pub modifier_legs: f32,
    pub no_collision: NoCollision,
}

impl WeaponDef {
    /// Fire interval expressed in seconds at the fixed 60 Hz tick rate.
    pub fn fire_interval_seconds(&self) -> f32 {
        f32::from(self.fire_interval) / 60.0
    }
    pub fn reload_seconds(&self) -> f32 {
        f32::from(self.reload_time) / 60.0
    }
    pub fn start_up_seconds(&self) -> f32 {
        f32::from(self.start_up_time) / 60.0
    }
    /// Ticks before the magazine is swapped during a clip reload.
    pub fn clip_out_ticks(&self) -> u16 {
        if self.clip_reload() {
            (f32::from(self.reload_time) * 0.8) as u16
        } else {
            0
        }
    }
    /// Ticks before the fresh magazine is seated during a clip reload.
    pub fn clip_in_ticks(&self) -> u16 {
        if self.clip_reload() {
            (f32::from(self.reload_time) * 0.3) as u16
        } else {
            0
        }
    }
    pub fn clip_reload(&self) -> bool {
        self.kind.clip_reload()
    }
    pub fn semi_automatic(&self) -> bool {
        self.kind.semi_automatic()
    }
    pub fn timeout(&self) -> u16 {
        self.bullet_style.timeout()
    }
    pub fn explosion_radius(&self) -> f32 {
        self.bullet_style.explosion_radius()
    }
    /// Projectiles per trigger pull. Dual Eagles fire two, a shotgun shell six.
    pub fn pellets(&self) -> u8 {
        if self.kind == WeaponKind::DesertEagles {
            2
        } else {
            self.bullet_style.pellets()
        }
    }
    /// Barrett keeps full power at range even though it fires a plain bullet.
    pub fn degrades_over_distance(&self) -> bool {
        self.kind != WeaponKind::Barrett && self.bullet_style.degrades_over_distance()
    }
    /// A negative bink spoils the shooter's own next shot.
    pub fn self_binks(&self) -> bool {
        self.bink < 0
    }
    /// A positive bink spoils the aim of whoever is hit.
    pub fn binks_the_victim(&self) -> bool {
        self.bink > 0
    }
    /// The hitbox modifier for one body region.
    pub fn region_modifier(&self, region: crate::BodyRegion) -> f32 {
        match region {
            crate::BodyRegion::Head => self.modifier_head,
            crate::BodyRegion::Chest => self.modifier_chest,
            crate::BodyRegion::Legs => self.modifier_legs,
        }
    }

    /// Everything a derived weapon copies from its parent: all sixteen fields but the style.
    pub(super) fn inherit(parent: &WeaponDef, kind: WeaponKind, style: BulletStyle) -> Self {
        Self {
            kind,
            bullet_style: style,
            ..*parent
        }
    }
}

/// The bounds every configured value has to satisfy.
#[derive(Clone, Copy, Debug)]
pub struct WeaponLimits {
    pub max_bytes: usize,
    pub max_line_bytes: usize,
    pub max_lines: usize,
    pub max_damage: f32,
    pub max_speed: f32,
    pub max_ticks: u32,
    pub max_ammo: u32,
    pub max_bink: i32,
    pub max_accuracy: f32,
    pub max_push: f32,
    pub max_inherited_velocity: f32,
    pub max_region_modifier: f32,
}

impl Default for WeaponLimits {
    fn default() -> Self {
        Self {
            max_bytes: 128 * 1024,
            max_line_bytes: 256,
            max_lines: 4096,
            max_damage: 100_000.0,
            max_speed: 1_000.0,
            max_ticks: 36_000,
            max_ammo: 255,
            max_bink: 10_000,
            max_accuracy: 10.0,
            max_push: 10.0,
            max_inherited_velocity: 10.0,
            max_region_modifier: 10.0,
        }
    }
}

/// The sixteen configurable keys, in the order they are written back out.
pub const WEAPON_KEYS: [&str; 16] = [
    "Damage",
    "FireInterval",
    "Ammo",
    "ReloadTime",
    "Speed",
    "BulletStyle",
    "StartUpTime",
    "Bink",
    "MovementAcc",
    "BulletSpread",
    "Recoil",
    "Push",
    "InheritedVelocity",
    "ModifierHead",
    "ModifierChest",
    "ModifierLegs",
];

/// The thirteen keys the canonical checksum covers, in `CreateWMChecksum` order.
pub const HASHED_KEYS: [&str; 13] = [
    "Damage",
    "FireInterval",
    "Ammo",
    "ReloadTime",
    "Speed",
    "BulletStyle",
    "StartUpTime",
    "Bink",
    "MovementAcc",
    "BulletSpread",
    "Recoil",
    "Push",
    "InheritedVelocity",
];
