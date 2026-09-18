//! Weapon configuration, ballistics, projectile behavior, and the two-slot inventory.
//!
//! Projectile behavior is composed from a weapon's bullet style rather than written per weapon:
//! `projectile` says how a shot answers terrain and bodies, `explosive`, `melee`, `flame`, and
//! `stationary` hold the rules of one weapon family each, and `firing` holds the few per-weapon
//! rules that decide whether a shot happens at all.

pub mod accuracy;
pub mod config;
pub mod damage;
pub mod explosive;
pub mod firing;
pub mod flame;
pub mod inventory;
mod loader;
pub mod melee;
pub mod pickups;
pub mod projectile;
pub mod reload;
pub mod stationary;
mod table;
mod tables;
pub mod throw;

pub use accuracy::{
    bink_on_hit, calculate_bink, inaccuracy, max_deviation, movement_accuracy, recoil_radians,
    self_bink_on_fire, stance_spread, ShooterPose, MAX_INACCURACY,
};
pub use config::{
    BulletStyle, NoCollision, WeaponDef, WeaponKind, WeaponLimits, ALL_WEAPONS, CONFIGURED_WEAPONS,
    SELECTABLE_WEAPONS,
};
pub use damage::{
    can_damage, degraded_hit_multiply, direct_damage, explosion_damage, explosion_impulse,
    muzzle_velocity, push_impulse, should_bink, FIRST_DEGRADE_DISTANCE, SECOND_DEGRADE_DISTANCE,
};
pub use explosive::{
    cluster_submunitions, throw_arc, throw_frame, throw_velocity, CLUSTER_SUBMUNITIONS,
    THROW_LAST_FRAME,
};
pub use firing::{
    barrel_origin, boosts_the_shooter, per_pellet_spread, refusal, self_boost,
    startup_resets_on_release, FireRefusal,
};
pub use flame::{propagated_hit_multiply, propagated_velocity, propagates};
pub use inventory::{muzzle_origin, Inventory, WeaponSlot, SWITCH_DELAY_TICKS};
pub use loader::WeaponConfigError;
pub use melee::{is_continuous_contact, reach as melee_reach, swing_segment, MELEE_RADIUS};
pub use pickups::{dropped_weapon, nearest_index, PICKUP_RADIUS};
pub use projectile::{
    bounce_velocity, collides_with_bodies, gravity_multiplier, grenade_is_armed, impact_response,
    pushes_objects, ricochet_velocity, surface_response, ImpactResponse, SurfaceResponse,
    ARROW_RESIST_TICKS, GRENADE_SURFACE_COEF, OBJECT_PUSH_MULTIPLIER, RICOCHET_MIN_TRAVEL,
};
pub use stationary::StationaryGun;
pub use table::WeaponTable;
pub use throw::{aim_dir, charged_velocity, MAX_THROW_CHARGE};
