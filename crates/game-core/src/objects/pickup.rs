//! Kits lying on the map: how they spawn, how they fall, and who may take them.
//!
//! A kit is a physical body like a flag or a dropped weapon, so it falls, slides, rests on terrain,
//! and is shoved about by gunfire. What differs is what taking one does, which `bonus` decides.

use serde::{Deserialize, Serialize};

use super::bonus::{BonusConfig, KitKind, KIT_RADIUS};
use crate::collision::{CollisionWorld, DynamicBody, DynamicBodyKind};
use crate::{SimRng, Vec2};

/// One kit on the map.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pickup {
    pub kind: KitKind,
    /// The body that falls and collides with the map.
    pub body: DynamicBody,
    /// Ticks before it disappears of its own accord.
    pub ticks_left: u16,
}

impl Pickup {
    pub fn new(kind: KitKind, pos: Vec2, lifetime: u16) -> Self {
        Self {
            kind,
            body: DynamicBody::new(DynamicBodyKind::Kit, pos, KIT_RADIUS),
            ticks_left: lifetime,
        }
    }

    pub fn pos(&self) -> Vec2 {
        self.body.pos
    }

    /// Advances the kit by one tick. Returns false once it has timed out.
    pub fn step(&mut self, collision: &CollisionWorld, dt: f32) -> bool {
        self.body.step(collision, dt);
        self.ticks_left = self.ticks_left.saturating_sub(1);
        self.ticks_left > 0
    }

    /// Whether a player standing at `pos` is close enough to take it.
    pub fn within_reach(&self, pos: Vec2) -> bool {
        let dx = self.body.pos.x - pos.x;
        let dy = self.body.pos.y - pos.y;
        (dx * dx + dy * dy).sqrt() <= KIT_RADIUS + 8.0
    }

    /// Shoves the kit, which is how a bullet or a blast moves one.
    pub fn push(&mut self, impulse: Vec2) {
        self.body.vel = Vec2 {
            x: self.body.vel.x + impulse.x,
            y: self.body.vel.y + impulse.y,
        };
        self.body.grounded = false;
    }
}

/// Every kit currently on the map, and the clock that puts more out.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pickups {
    pub items: Vec<Pickup>,
    /// Ticks since the last spawn roll.
    pub spawn_timer: u32,
    /// The next id to hand out, so a client can follow one kit across snapshots.
    next_id: u32,
}

impl Pickups {
    /// Advances every kit and rolls for a new one.
    ///
    /// `spawn_at` supplies somewhere to put a new kit; a map with nowhere to put one simply gets
    /// no kits rather than a pile at the origin.
    pub fn step(
        &mut self,
        config: &BonusConfig,
        collision: &CollisionWorld,
        dt: f32,
        rng: &mut SimRng,
        spawn_at: impl Fn(&mut SimRng) -> Option<Vec2>,
    ) -> Option<KitKind> {
        self.items.retain_mut(|item| item.step(collision, dt));

        if !config.any() {
            self.spawn_timer = 0;
            return None;
        }
        self.spawn_timer = self.spawn_timer.saturating_add(1);
        if self.spawn_timer < config.frequency_ticks {
            return None;
        }
        self.spawn_timer = 0;
        if self.items.len() >= config.max_on_map {
            return None;
        }

        let candidates = config.enabled();
        if candidates.is_empty() {
            return None;
        }
        let kind = candidates[(rng.next_u64() as usize) % candidates.len()];
        // Rarer kits have to win a second roll, which is how the upstream frequencies work.
        if kind.rarity() > 1 && (rng.next_u64() % u64::from(kind.rarity())) != 0 {
            return None;
        }
        let pos = spawn_at(rng)?;
        self.items
            .push(Pickup::new(kind, pos, config.lifetime_ticks));
        self.next_id = self.next_id.wrapping_add(1);
        Some(kind)
    }

    /// The kit a player standing at `pos` would pick up, if any.
    ///
    /// The nearest one wins, so walking between two kits takes the one you are actually on.
    pub fn nearest(&self, pos: Vec2) -> Option<usize> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.within_reach(pos))
            .min_by(|left, right| {
                let distance = |item: &Pickup| {
                    let dx = item.body.pos.x - pos.x;
                    let dy = item.body.pos.y - pos.y;
                    dx * dx + dy * dy
                };
                distance(left.1).total_cmp(&distance(right.1))
            })
            .map(|(index, _)| index)
    }

    /// Takes one kit off the map.
    pub fn take(&mut self, index: usize) -> Option<Pickup> {
        if index >= self.items.len() {
            return None;
        }
        Some(self.items.remove(index))
    }

    /// Shoves any kit within `radius` of `at`.
    pub fn push_near(&mut self, at: Vec2, radius: f32, impulse: Vec2) {
        for item in &mut self.items {
            let dx = item.body.pos.x - at.x;
            let dy = item.body.pos.y - at.y;
            if (dx * dx + dy * dy).sqrt() <= radius {
                item.push(impulse);
            }
        }
    }

    /// Clears the map, which is what a round reset does.
    pub fn clear(&mut self) {
        self.items.clear();
        self.spawn_timer = 0;
    }
}
