pub(crate) mod body;
pub(crate) mod damage;
pub(crate) mod death;
mod jet;
mod movement;
pub(crate) mod ragdoll;
pub(crate) mod respawn;
mod state;

pub use body::{apply_impulse, resolve_player_contact, Impulse, ImpulseSource, ImpulseTarget};
pub use damage::{
    absorb, assists, record_attribution, region_multiplier, Absorbed, Attribution, Bleed,
    DamageCause, DamageConfig, DamageEvent, MAX_ATTRIBUTIONS,
};
pub use death::{multi_kill_label, DeathCause, KillFeedEntry, MultiKill};
pub use movement::{advance_character, MovementConfig};
pub use ragdoll::{Ragdoll, RagdollSegment};
pub use respawn::{fallback_spawn, select_spawn, RespawnConfig};
pub use state::{CharacterState, Direction, Emote, RollSource};
