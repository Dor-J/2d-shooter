use super::state::CharacterState;
use crate::{BodyShape, DynamicBody, Player, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpulseSource {
    Recoil,
    SpasBoost,
    MinigunBoost,
    Explosion,
    Bullet,
    Flag,
    Kit,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Impulse {
    pub velocity: Vec2,
    pub source: ImpulseSource,
}

pub trait ImpulseTarget {
    fn velocity_mut(&mut self) -> &mut Vec2;
    fn record_impulse(&mut self, source: ImpulseSource);
}

impl ImpulseTarget for Player {
    fn velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }
    fn record_impulse(&mut self, source: ImpulseSource) {
        self.last_impulse = Some(source);
    }
}

impl ImpulseTarget for DynamicBody {
    fn velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.vel
    }
    fn record_impulse(&mut self, _source: ImpulseSource) {}
}

pub fn apply_impulse(target: &mut impl ImpulseTarget, impulse: Impulse) {
    let velocity = target.velocity_mut();
    velocity.x += impulse.velocity.x;
    velocity.y += impulse.velocity.y;
    target.record_impulse(impulse.source);
}

pub fn resolve_player_contact(left: &mut Player, right: &mut Player) -> bool {
    const CONTACT_RADIUS: f32 = 10.0;
    let dx = right.pos.x - left.pos.x;
    let dy = right.pos.y - left.pos.y;
    let distance_squared = dx * dx + dy * dy;
    let minimum = CONTACT_RADIUS * 2.0;
    if distance_squared >= minimum * minimum {
        return false;
    }
    let distance = distance_squared.sqrt();
    let (nx, ny) = if distance > f32::EPSILON {
        (dx / distance, dy / distance)
    } else if left.id <= right.id {
        (1.0, 0.0)
    } else {
        (-1.0, 0.0)
    };
    let correction = (minimum - distance) * 0.5;
    left.pos.x -= nx * correction;
    left.pos.y -= ny * correction;
    right.pos.x += nx * correction;
    right.pos.y += ny * correction;
    let closing_speed = (right.vel.x - left.vel.x) * nx + (right.vel.y - left.vel.y) * ny;
    if closing_speed < 0.0 {
        let impulse = closing_speed * 0.5;
        left.vel.x += nx * impulse;
        left.vel.y += ny * impulse;
        right.vel.x -= nx * impulse;
        right.vel.y -= ny * impulse;
    }
    true
}

pub(crate) fn shape_for(state: CharacterState) -> BodyShape {
    match state {
        CharacterState::Prone | CharacterState::GoingProne { .. } => BodyShape::prone(),
        state if state.is_low() => BodyShape::crouching(),
        _ => BodyShape::standing(),
    }
}
