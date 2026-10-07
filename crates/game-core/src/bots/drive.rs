use std::collections::BTreeMap;

use crate::weapons::WeaponKind;
use crate::{
    fallback_spawn, BotContact, BotView, DynamicBodyKind, Input, Vec2, World, HEIGHT, WIDTH,
};

pub(crate) fn bot_inputs(world: &mut World) -> BTreeMap<u32, Input> {
    if world.bots.is_empty() {
        return BTreeMap::new();
    }

    let contacts: Vec<BotContact> = world
        .players
        .values()
        .filter(|player| !crate::modes::team::is_spectator(player.team))
        .map(|player| BotContact {
            id: player.id,
            team: player.team,
            pos: player.pos,
            velocity: player.vel,
            alive: player.hp > 0,
        })
        .collect();
    // Somewhere to send a bot that has nothing else to do. A map with no spawn points would
    // otherwise leave every bot standing exactly where it appeared.
    let spawns: Vec<Vec2> = if world.map_spawns.is_empty() {
        vec![
            fallback_spawn(0, 1),
            fallback_spawn(1, 2),
            Vec2 {
                x: WIDTH / 2.0,
                y: HEIGHT / 2.0,
            },
        ]
    } else {
        world
            .map_spawns
            .iter()
            .map(|spawn| spawn.position)
            .collect()
    };
    let loot: Vec<Vec2> = world
        .pickups
        .items
        .iter()
        .map(|kit| kit.pos())
        .chain(
            world
                .objects
                .iter()
                .filter(|object| object.kind == DynamicBodyKind::DroppedWeapon && object.active)
                .map(|object| object.pos),
        )
        .collect();
    let rules = world.rules;
    let table = world.weapons.clone();
    let objectives = world.objectives.clone();
    let tick = world.tick;

    let ids: Vec<u32> = world.bots.keys().copied().collect();
    let mut inputs = BTreeMap::new();
    for id in ids {
        let Some(player) = world.players.get(&id) else {
            continue;
        };
        let Some(me) = contacts.iter().find(|contact| contact.id == id).copied() else {
            continue;
        };
        let holding = WeaponKind::from_slot(player.weapon);
        let team_size = contacts
            .iter()
            .filter(|contact| contact.team == player.team)
            .count();
        let view = BotView {
            me,
            fuel: f32::from(player.fuel),
            ammo: player.ammo,
            magazine: u16::from(table.get(holding).ammo),
            reloading: player.reload_timer > 0,
            grenades: player.grenades,
            holding,
            unlocked: player.unlocked,
            team_size,
        };
        let Some(mut bot) = world.bots.remove(&id) else {
            continue;
        };
        let input = crate::bots::think(
            &mut bot,
            view,
            crate::bots::BotWorld {
                others: &contacts,
                rules: &rules,
                objectives: &objectives,
                table: &table,
                collision: &world.collision,
                spawns: &spawns,
                loot: &loot,
            },
            &mut world.rng,
            (tick % u64::from(u32::MAX)) as u32,
        );
        world.bots.insert(id, bot);
        inputs.insert(id, input);
    }
    inputs
}
