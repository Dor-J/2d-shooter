use super::PolygonKind;
use crate::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct MaterialResponse {
    pub velocity: Vec2,
    pub deadly: bool,
}

pub fn resolve_material_velocity(
    velocity: Vec2,
    normal: Vec2,
    kind: PolygonKind,
) -> MaterialResponse {
    let normal_speed = dot(velocity, normal);
    if normal_speed >= 0.0 {
        return MaterialResponse {
            velocity,
            deadly: kind == PolygonKind::Deadly,
        };
    }
    let tangent = subtract(velocity, scale(normal, normal_speed));
    let friction = match kind {
        PolygonKind::Ice => 0.99,
        PolygonKind::Bouncy => 1.0,
        _ => 0.8,
    };
    let restitution = if kind == PolygonKind::Bouncy {
        0.8
    } else {
        0.0
    };
    MaterialResponse {
        velocity: Vec2 {
            x: tangent.x * friction - normal.x * normal_speed * restitution,
            y: tangent.y * friction - normal.y * normal_speed * restitution,
        },
        deadly: kind == PolygonKind::Deadly,
    }
}

fn dot(a: Vec2, b: Vec2) -> f32 {
    a.x * b.x + a.y * b.y
}
fn subtract(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}
fn scale(value: Vec2, scalar: f32) -> Vec2 {
    Vec2 {
        x: value.x * scalar,
        y: value.y * scalar,
    }
}
