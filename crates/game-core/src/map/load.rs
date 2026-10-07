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
    world.waypoints = bot_waypoints(&map.asset().waypoints);
    world
}

/// Active PMS points, in file order, with connections resolved against that active list.
///
/// A connection is a 0-based index when that index exists, otherwise a 1-based index when
/// `value - 1` exists. Anything else is dropped. No reverse links are invented.
fn bot_waypoints(raw: &[content::pms::Waypoint]) -> Vec<crate::Waypoint> {
    let active: Vec<&content::pms::Waypoint> = raw.iter().filter(|point| point.active).collect();
    let len = active.len();
    active
        .iter()
        .map(|point| {
            let mut links = [0u16; 20];
            let mut link_count = 0u8;
            let stored = point.connections_num.clamp(0, 20) as usize;
            for &value in point.connections.iter().take(stored) {
                let Some(index) = connection_index(value, len) else {
                    continue;
                };
                links[usize::from(link_count)] = index as u16;
                link_count += 1;
            }
            crate::Waypoint {
                pos: Vec2 {
                    x: point.x as f32,
                    y: point.y as f32,
                },
                needs_jet: point.jetpack,
                links,
                link_count,
            }
        })
        .collect()
}

fn connection_index(value: i32, len: usize) -> Option<usize> {
    let len = i32::try_from(len).unwrap_or(i32::MAX);
    if (0..len).contains(&value) {
        return Some(value as usize);
    }
    let one_based = value.checked_sub(1)?;
    if (0..len).contains(&one_based) {
        return Some(one_based as usize);
    }
    None
}

pub(crate) fn place_specials(world: &mut World) {
    world.guns = world
        .map_spawns
        .iter()
        .filter(|spawn| spawn.kind == SpawnKind::StationaryGun)
        .map(|spawn| StationaryGun::new(spawn.position))
        .collect();
}

#[cfg(test)]
mod tests {
    use content::pms::{MapAsset, Polygon, Vertex, Waypoint};

    use super::with_map;
    use crate::ValidatedMap;

    fn point(
        active: bool,
        x: i32,
        y: i32,
        jetpack: bool,
        connections_num: i32,
        connections: [i32; 20],
    ) -> Waypoint {
        Waypoint {
            active,
            id: 0,
            x,
            y,
            left: false,
            right: false,
            up: false,
            down: false,
            jetpack,
            path_num: 0,
            action: 0,
            connections_num,
            connections,
        }
    }

    #[test]
    fn with_map_copies_active_waypoints_and_resolves_connections() {
        let mut first = [0; 20];
        first[0] = 0;
        first[1] = 2;
        first[2] = 99;
        let mut second = [0; 20];
        second[0] = 1;
        let vertex = Vertex {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rhw: 1.0,
            color: [255; 4],
            u: 0.0,
            v: 0.0,
        };
        let asset = MapAsset {
            hash: 1,
            version: 1,
            name: "Fixture".into(),
            texture: "fixture.png".into(),
            background_top: [0; 4],
            background_bottom: [0; 4],
            start_jet: 100,
            grenade_packs: 0,
            medikits: 0,
            weather: 0,
            steps: 0,
            random_id: 0,
            polygons: vec![Polygon {
                vertices: [
                    vertex,
                    Vertex { x: 10.0, ..vertex },
                    Vertex { y: 10.0, ..vertex },
                ],
                normals: [[0.0; 3]; 3],
                kind: 0,
            }],
            sectors_division: 50,
            sectors_num: 0,
            sectors: Vec::new(),
            props: Vec::new(),
            scenery: Vec::new(),
            colliders: Vec::new(),
            spawnpoints: Vec::new(),
            waypoints: vec![
                point(true, 12, 34, false, 3, first),
                point(false, 1, 1, true, 1, [0; 20]),
                point(true, 56, 78, true, 1, second),
            ],
        };
        let map = ValidatedMap::try_from(asset).unwrap();
        let world = with_map("deathmatch", &map);

        assert_eq!(world.waypoints.len(), 2);
        assert_eq!(world.waypoints[0].pos.x, 12.0);
        assert_eq!(world.waypoints[0].pos.y, 34.0);
        assert!(!world.waypoints[0].needs_jet);
        assert_eq!(world.waypoints[0].link_count, 2);
        assert_eq!(world.waypoints[0].links[0], 0);
        assert_eq!(world.waypoints[0].links[1], 1);
        assert!(world.waypoints[1].needs_jet);
        assert_eq!(world.waypoints[1].pos.x, 56.0);
        assert_eq!(world.waypoints[1].link_count, 1);
        assert_eq!(world.waypoints[1].links[0], 1);
    }
}
