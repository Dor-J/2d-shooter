pub(crate) mod body;
pub(crate) mod damage;
pub(crate) mod death;
mod jet;
mod movement;
mod player;
pub(crate) mod ragdoll;
mod resolve;
pub(crate) mod respawn;
mod state;
pub(crate) mod tick;

pub use body::{apply_impulse, resolve_player_contact, Impulse, ImpulseSource, ImpulseTarget};
pub use damage::{
    absorb, assists, record_attribution, region_multiplier, Absorbed, Attribution, Bleed,
    DamageCause, DamageConfig, DamageEvent, MAX_ATTRIBUTIONS,
};
pub use death::{multi_kill_label, DeathCause, KillFeedEntry, MultiKill};
pub use movement::{advance_character, MovementConfig};
pub use player::Player;
pub use ragdoll::{Ragdoll, RagdollSegment};
pub(crate) use resolve::apply_damage;
pub use respawn::{fallback_spawn, select_spawn, RespawnConfig};
pub use state::{CharacterState, Direction, Emote, RollSource};
