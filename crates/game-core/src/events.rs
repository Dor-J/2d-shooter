use serde::{Deserialize, Serialize};

use crate::{BodyRegion, BonusEffect, DamageCause, FireRefusal, KillFeedEntry, KitKind, Vec2};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Shot {
        player: u32,
    },
    Hit {
        target: u32,
        hp: i32,
    },
    Kill {
        killer: u32,
        target: u32,
    },
    Respawn {
        player: u32,
    },
    /// Authoritative damage, for the damage-direction indicator and hit feedback.
    Damage {
        target: u32,
        attacker: Option<u32>,
        amount: i32,
        region: BodyRegion,
        cause: DamageCause,
        direction: Vec2,
    },
    KillFeed(KillFeedEntry),
    /// Presentation only: derived from authoritative damage, never read back by the simulation.
    Blood {
        target: u32,
        position: Vec2,
        direction: Vec2,
        amount: i32,
    },
    Gibs {
        target: u32,
        position: Vec2,
        velocity: Vec2,
    },
    Reload {
        player: u32,
    },
    Empty {
        player: u32,
    },
    /// A kit appeared on the map, for the client to draw and play a sound for.
    KitSpawned {
        kind: KitKind,
    },
    /// Somebody picked a kit up.
    KitTaken {
        player: u32,
        kind: KitKind,
    },
    /// A timed effect ran out.
    BonusExpired {
        player: u32,
        effect: BonusEffect,
    },
    /// The weapon would not fire from the pose it was fired in, so the client can say why rather
    /// than leaving the player wondering whether the shot was lost to the network.
    FireRefused {
        player: u32,
        reason: FireRefusal,
    },
    MuzzleFlash {
        player: u32,
        pos: Vec2,
    },
    Casing {
        player: u32,
        pos: Vec2,
    },
    /// Something went off. Presentation only: the damage it did was already reported as `Damage`,
    /// and the client draws the fireball, the smoke, and the shake from this.
    Explosion {
        pos: Vec2,
        radius: f32,
    },
    /// A round struck terrain rather than a person, with the surface it struck. The client throws
    /// sparks along the normal; nothing reads it back.
    Impact {
        pos: Vec2,
        normal: Vec2,
    },
}
