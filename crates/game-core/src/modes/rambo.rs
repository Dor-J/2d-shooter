use crate::weapons::{sync_hands, WeaponKind};
use crate::{ModeKind, World};

pub(crate) fn sync_rambo_bow(world: &mut World) {
    if world.rules.kind != ModeKind::Rambomatch {
        return;
    }
    let ammo = u16::from(world.weapons.get(WeaponKind::RamboBow).ammo);
    let holders: Vec<u32> = world.objectives.carriers().map(|(_, id)| id).collect();
    let ids: Vec<u32> = world.players.keys().copied().collect();
    for id in ids {
        let rambo = holders.contains(&id);
        let Some(player) = world.players.get_mut(&id) else {
            continue;
        };
        if rambo {
            if !player.inventory.owns(WeaponKind::RamboBow)
                && !player.inventory.owns(WeaponKind::FlamedArrows)
            {
                player.inventory.force_equip(WeaponKind::RamboBow, ammo);
                sync_hands(player);
            }
        } else if player.inventory.owns(WeaponKind::RamboBow)
            || player.inventory.owns(WeaponKind::FlamedArrows)
        {
            player.inventory.take_kind(WeaponKind::RamboBow);
            player.inventory.take_kind(WeaponKind::FlamedArrows);
            sync_hands(player);
        }
    }
}
