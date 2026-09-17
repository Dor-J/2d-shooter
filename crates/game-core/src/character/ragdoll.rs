use crate::collision::{BodyRegion, BodyShape, CollisionMask, CollisionWorld};
use crate::{resolve_material_velocity, Vec2, DT};
use serde::{Deserialize, Serialize};

/// One limb of a corpse. Segments fall and settle independently, which is what makes a body drape
/// over terrain instead of sliding as a single disc.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RagdollSegment {
    pub region: BodyRegion,
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub grounded: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ragdoll {
    pub player: u32,
    pub team: u8,
    pub segments: Vec<RagdollSegment>,
    pub ticks_left: u16,
    /// A body torn apart by a huge hit leaves gibs instead of a corpse.
    pub gibbed: bool,
}

const GRAVITY: f32 = 950.0;
const LINK_STIFFNESS: f32 = 0.35;

impl Ragdoll {
    /// Builds a corpse from the pose the player died in, carrying their velocity plus the impulse
    /// that killed them.
    pub fn spawn(
        player: u32,
        team: u8,
        position: Vec2,
        velocity: Vec2,
        impulse: Vec2,
        ticks: u16,
        gibbed: bool,
    ) -> Self {
        let shape = BodyShape::standing();
        let segments = shape
            .parts()
            .map(|part| RagdollSegment {
                region: part.region,
                pos: Vec2 {
                    x: position.x + part.offset.x,
                    y: position.y + part.offset.y,
                },
                vel: Vec2 {
                    x: velocity.x + impulse.x,
                    y: velocity.y + impulse.y,
                },
                radius: part.radius,
                grounded: false,
            })
            .collect();
        Self {
            player,
            team,
            segments,
            ticks_left: ticks,
            gibbed,
        }
    }

    pub fn position(&self) -> Vec2 {
        let count = self.segments.len().max(1) as f32;
        let sum = self
            .segments
            .iter()
            .fold(Vec2::default(), |acc, segment| Vec2 {
                x: acc.x + segment.pos.x,
                y: acc.y + segment.pos.y,
            });
        Vec2 {
            x: sum.x / count,
            y: sum.y / count,
        }
    }

    /// Advances the corpse one tick; returns false once its lifetime has run out.
    pub fn step(&mut self, collision: &CollisionWorld) -> bool {
        if self.ticks_left == 0 {
            return false;
        }
        self.ticks_left -= 1;

        for index in 0..self.segments.len() {
            let mut segment = self.segments[index];
            segment.vel.y += GRAVITY * DT;
            let end = Vec2 {
                x: segment.pos.x + segment.vel.x * DT,
                y: segment.pos.y + segment.vel.y * DT,
            };
            segment.grounded = false;
            if let Some(hit) = collision.sweep_shape(
                segment.pos,
                end,
                &BodyShape::circle(segment.radius.max(0.5)),
                CollisionMask::PLAYER,
            ) {
                segment.pos = hit.hit.position;
                let response = resolve_material_velocity(segment.vel, hit.hit.normal, hit.hit.kind);
                segment.vel = Vec2 {
                    x: response.velocity.x * 0.6,
                    y: response.velocity.y * 0.4,
                };
                segment.grounded = hit.hit.normal.y < -0.5;
            } else {
                segment.pos = end;
            }
            self.segments[index] = segment;
        }

        self.hold_together();
        true
    }

    /// Keeps neighbouring segments roughly a body apart so a corpse never stretches across the map.
    fn hold_together(&mut self) {
        for index in 1..self.segments.len() {
            let previous = self.segments[index - 1];
            let current = self.segments[index];
            let rest = previous.radius + current.radius;
            let dx = current.pos.x - previous.pos.x;
            let dy = current.pos.y - previous.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance <= rest || distance <= f32::EPSILON {
                continue;
            }
            let pull = (distance - rest) * LINK_STIFFNESS / distance;
            self.segments[index - 1].pos.x += dx * pull * 0.5;
            self.segments[index - 1].pos.y += dy * pull * 0.5;
            self.segments[index].pos.x -= dx * pull * 0.5;
            self.segments[index].pos.y -= dy * pull * 0.5;
        }
    }
}
