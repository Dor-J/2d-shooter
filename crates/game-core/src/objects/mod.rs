//! Game objects that are neither players nor projectiles: the objectives every flag mode is
//! built from.

pub mod bonus;
pub mod flag;
pub mod pickup;

pub use bonus::{
    predator_alpha, BonusConfig, BonusEffect, KitKind, TimedEffect, BERSERKER_DAMAGE_MULTIPLIER,
    CLUSTER_GRENADES, KIT_RADIUS, PREDATOR_ALPHA, VEST_ARMOR,
};
pub use flag::{
    Flag, FlagEvent, FlagKind, FlagState, FLAG_PICKUP_RADIUS, FLAG_TIMEOUT, TOUCHDOWN_RADIUS,
};
pub use pickup::{Pickup, Pickups};
