//! Ground weapons share `DynamicBody`; slot and ammo ride along as metadata.

use crate::collision::{DynamicBody, DynamicBodyKind};
use crate::Vec2;

use super::WeaponKind;

pub const PICKUP_RADIUS: f32 = 28.0;
pub const GROUND_RADIUS: f32 = 8.0;

pub fn in_range(player: Vec2, object: Vec2) -> bool {
    let dx = player.x - object.x;
    let dy = player.y - object.y;
    dx * dx + dy * dy <= PICKUP_RADIUS * PICKUP_RADIUS
}

pub fn dropped_weapon(kind: WeaponKind, ammo: u16, pos: Vec2, vel: Vec2) -> DynamicBody {
    DynamicBody {
        kind: DynamicBodyKind::DroppedWeapon,
        pos,
        vel,
        radius: GROUND_RADIUS,
        grounded: false,
        active: true,
        weapon_slot: kind.slot(),
        ammo,
    }
}

pub fn nearest_index(player: Vec2, objects: &[DynamicBody]) -> Option<usize> {
    objects
        .iter()
        .enumerate()
        .filter(|(_, object)| {
            object.active
                && object.kind == DynamicBodyKind::DroppedWeapon
                && object.weapon_slot.is_some()
                && in_range(player, object.pos)
        })
        .min_by(|left, right| dist2(player, left.1.pos).total_cmp(&dist2(player, right.1.pos)))
        .map(|(index, _)| index)
}

fn dist2(a: Vec2, b: Vec2) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}
