use crate::weapons::{sync_hands, WeaponKind};
use crate::{
    BonusEffect, Event, KitKind, Vec2, World, CLUSTER_GRENADES, DT, HEIGHT, VEST_ARMOR, WIDTH,
};

pub(crate) fn step_bonuses(world: &mut World) {
    // Timed effects run down first, so a kit picked up this tick gets its full duration.
    let expired: Vec<(u32, BonusEffect)> = world
        .players
        .iter_mut()
        .filter_map(|(id, player)| player.bonus.step().map(|effect| (*id, effect)))
        .collect();
    for (player, effect) in expired {
        if effect == BonusEffect::FlameGod {
            if let Some(holder) = world.players.get_mut(&player) {
                holder.inventory.take_kind(WeaponKind::Flamer);
                sync_hands(holder);
            }
        }
        world.events.push(Event::BonusExpired { player, effect });
    }

    let config = world.bonuses;
    let spawns = world.map_spawns.clone();
    let mut pickups = std::mem::take(&mut world.pickups);
    let spawned = pickups.step(&config, &world.collision, DT, &mut world.rng, |rng| {
        // A kit drops somewhere a player could stand, which is what the spawns already mark.
        if spawns.is_empty() {
            return Some(Vec2 {
                x: WIDTH / 2.0,
                y: HEIGHT / 3.0,
            });
        }
        let index = (rng.next_u64() as usize) % spawns.len();
        Some(spawns[index].position)
    });
    world.pickups = pickups;
    if let Some(kind) = spawned {
        world.events.push(Event::KitSpawned { kind });
    }

    // Anybody standing on a kit takes it, if it has anything to give them.
    let standing: Vec<(u32, usize)> = world
        .players
        .values()
        .filter(|player| player.hp > 0 && !crate::modes::team::is_spectator(player.team))
        .filter_map(|player| {
            world
                .pickups
                .nearest(player.pos)
                .map(|index| (player.id, index))
        })
        .collect();
    // Two players on the same kit: the lower id gets it, and the other finds it gone.
    let mut taken: Vec<usize> = Vec::new();
    for (id, index) in standing {
        if taken.contains(&index) {
            continue;
        }
        let Some(kind) = world.pickups.items.get(index).map(|item| item.kind) else {
            continue;
        };
        if !world.apply_kit(id, kind) {
            continue;
        }
        taken.push(index);
        world.events.push(Event::KitTaken { player: id, kind });
    }
    // Remove from the back so the earlier indices stay valid.
    taken.sort_unstable();
    for index in taken.into_iter().rev() {
        world.pickups.take(index);
    }
}

pub(crate) fn apply_kit(world: &mut World, id: u32, kind: KitKind) -> bool {
    let grenade_cap = world.damage.max_grenades;
    let flamer_ammo = u16::from(world.weapons.get(WeaponKind::Flamer).ammo);
    let Some(player) = world.players.get_mut(&id) else {
        return false;
    };
    match kind {
        KitKind::Medic => {
            if player.hp >= 100 {
                return false;
            }
            player.hp = 100;
            true
        }
        KitKind::Grenades => {
            if player.grenades >= grenade_cap && player.cluster_grenades == 0 {
                return false;
            }
            player.grenades = grenade_cap;
            player.cluster_grenades = 0;
            true
        }
        KitKind::ClusterGrenades => {
            player.cluster_grenades = CLUSTER_GRENADES;
            player.grenades = player.grenades.max(CLUSTER_GRENADES);
            true
        }
        KitKind::Vest => {
            if player.armor >= VEST_ARMOR {
                return false;
            }
            player.armor = VEST_ARMOR;
            true
        }
        KitKind::FlameGod | KitKind::Berserker | KitKind::Predator => {
            let Some(effect) = kind.effect() else {
                return false;
            };
            if !player.bonus.activate(effect) {
                return false;
            }
            // Every timed kit also puts you back on your feet, which is what makes them worth
            // running for when you are hurt.
            player.hp = 100;
            if kind == KitKind::FlameGod {
                player
                    .inventory
                    .force_equip(WeaponKind::Flamer, flamer_ammo);
                sync_hands(player);
            }
            true
        }
    }
}
