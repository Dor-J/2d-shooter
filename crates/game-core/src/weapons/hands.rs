use super::{ShooterPose, WeaponKind};
use crate::{BonusEffect, Input, Player};

pub(crate) fn shooter_pose(player: &Player, input: &Input) -> ShooterPose {
    ShooterPose {
        state: player.state,
        grounded: player.grounded,
        moving: input.left || input.right || player.vel.x.abs() > 20.0,
        jetting: input.jet && player.fuel > 0,
        bink: player.bink,
    }
}

pub(crate) fn held_kind(player: &Player) -> WeaponKind {
    let kind = player
        .inventory
        .active()
        .map(|slot| slot.kind)
        .unwrap_or(WeaponKind::Punch);
    if kind == WeaponKind::RamboBow && player.bonus.is(BonusEffect::FlameGod) {
        return WeaponKind::FlamedArrows;
    }
    kind
}

pub(crate) fn sync_hands(player: &mut Player) {
    if let Some(slot) = player.inventory.active() {
        if let Some(index) = slot.kind.slot() {
            player.weapon = index;
        }
        player.ammo = slot.ammo;
    } else {
        player.ammo = 0;
    }
}

pub(crate) fn writeback_ammo(player: &mut Player) {
    if let Some(slot) = player.inventory.active_mut() {
        slot.ammo = player.ammo;
    }
}
