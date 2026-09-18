//! Charged weapon throws. Knife flight uses the thrown-knife definition in `World`.

use crate::Vec2;

pub const MAX_THROW_CHARGE: u16 = 36;

pub fn aim_dir(pos: Vec2, aim: Vec2, facing_x: f32) -> Option<Vec2> {
    let dx = aim.x - pos.x;
    let dy = aim.y - pos.y;
    let len = (dx * dx + dy * dy).sqrt();
    if len > 1.0 && len.is_finite() {
        Some(Vec2 {
            x: dx / len,
            y: dy / len,
        })
    } else if facing_x.abs() > 0.0 {
        Some(Vec2 {
            x: facing_x.signum(),
            y: 0.0,
        })
    } else {
        None
    }
}

pub fn charged_velocity(dir: Vec2, charge: u16, inherit: Vec2) -> Vec2 {
    let power = 120.0 + f32::from(charge.min(MAX_THROW_CHARGE)) * 12.0;
    Vec2 {
        x: dir.x * power + inherit.x,
        y: dir.y * power - 40.0 + inherit.y,
    }
}
