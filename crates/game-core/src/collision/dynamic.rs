use super::{resolve_material_velocity, BodyShape, CollisionMask, CollisionWorld};
use crate::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DynamicBodyKind {
    Corpse,
    Flag,
    Kit,
    DroppedWeapon,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynamicBody {
    pub kind: DynamicBodyKind,
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub grounded: bool,
    pub active: bool,
    #[serde(default)]
    pub weapon_slot: Option<u8>,
    #[serde(default)]
    pub ammo: u16,
}

impl DynamicBody {
    pub fn new(kind: DynamicBodyKind, pos: Vec2, radius: f32) -> Self {
        Self {
            kind,
            pos,
            vel: Vec2::default(),
            radius,
            grounded: false,
            active: true,
            weapon_slot: None,
            ammo: 0,
        }
    }

    pub fn step(&mut self, collision: &CollisionWorld, dt: f32) {
        if !self.active || !dt.is_finite() || dt <= 0.0 {
            return;
        }
        self.vel.y += 950.0 * dt;
        let end = Vec2 {
            x: self.pos.x + self.vel.x * dt,
            y: self.pos.y + self.vel.y * dt,
        };
        self.grounded = false;
        if let Some(shape_hit) = collision.sweep_shape(
            self.pos,
            end,
            &BodyShape::circle(self.radius.max(0.0)),
            CollisionMask::PLAYER,
        ) {
            self.pos = shape_hit.hit.position;
            self.vel =
                resolve_material_velocity(self.vel, shape_hit.hit.normal, shape_hit.hit.kind)
                    .velocity;
            self.grounded = shape_hit.hit.normal.y < -0.5;
        } else {
            self.pos = end;
        }
    }
}
