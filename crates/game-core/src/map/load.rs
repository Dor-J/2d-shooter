use crate::collision::CollisionWorld;
use crate::{
    fallback_spawn, FlagKind, MapSpawn, Objectives, SpawnKind, StationaryGun, ValidatedMap, Vec2,
    World, HEIGHT, WIDTH,
};

pub(crate) fn place_objectives(world: &mut World) {
    let spawns = world.map_spawns.clone();
    let rules = world.rules;
    world.objectives = Objectives::for_mode(&rules, |kind| match kind {
        FlagKind::Yellow => Some(Vec2 {
            x: WIDTH / 2.0,
            y: HEIGHT / 2.0,
        }),
        team_flag => {
            let team = team_flag.owning_team();
            spawns
                .iter()
                .find(|spawn| spawn.team == team)
                .map(|spawn| spawn.position)
                .or_else(|| Some(fallback_spawn(u32::from(team), team)))
        }
    });
}

pub(crate) fn with_map(mode: &str, map: &ValidatedMap) -> World {
    let mut world = World::with_collision(mode, CollisionWorld::from_map(map));
    let start = i64::from(map.asset().start_jet.max(0));
    world.movement.fuel_capacity = i16::try_from((119 * start) / 100).unwrap_or(i16::MAX);
    world.map_spawns = map
        .asset()
        .spawnpoints
        .iter()
        .filter(|spawn| spawn.active)
        .map(|spawn| MapSpawn {
            position: Vec2 {
                x: spawn.x as f32,
                y: spawn.y as f32,
            },
            team: spawn.team.clamp(0, u8::MAX as i32) as u8,
            kind: SpawnKind::from_pms_team(spawn.team),
        })
        .collect();
    world.place_objectives();
    world.place_specials();
    world
}

pub(crate) fn place_specials(world: &mut World) {
    world.guns = world
        .map_spawns
        .iter()
        .filter(|spawn| spawn.kind == SpawnKind::StationaryGun)
        .map(|spawn| StationaryGun::new(spawn.position))
        .collect();
}
