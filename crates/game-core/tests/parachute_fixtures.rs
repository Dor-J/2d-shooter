//! Parachute on a high spawn. Descent is one body force, not skeleton joints.

use game_core::{
    advance_character, CharacterState, CollisionPolygon, CollisionWorld, Input, MapSpawn,
    MovementConfig, Player, PolygonKind, SpawnKind, Vec2, World,
};

const SPAWN: Vec2 = Vec2 { x: 200.0, y: 80.0 };

fn world_with_ground(below: f32) -> World {
    let floor_y = SPAWN.y + below;
    let collision = CollisionWorld::new(vec![CollisionPolygon::triangle(
        [
            Vec2 { x: 0.0, y: floor_y },
            Vec2 {
                x: 400.0,
                y: floor_y,
            },
            Vec2 {
                x: 200.0,
                y: floor_y + 24.0,
            },
        ],
        PolygonKind::Normal,
    )]);
    let mut world = World::with_collision("deathmatch", collision);
    world.map_spawns.push(MapSpawn {
        position: SPAWN,
        team: 0,
        kind: SpawnKind::Player,
    });
    world.add_player(1, "spawn".into());
    let player = world.players.get_mut(&1).unwrap();
    player.hp = 0;
    player.respawn = 0;
    player.parachute = false;
    world.step(&Default::default());
    world
}

fn airborne(parachute: bool) -> Player {
    let mut player = Player::new(1, "canopy".into(), 0);
    player.parachute = parachute;
    player.grounded = false;
    player.state = CharacterState::Airborne;
    player
}

#[test]
fn a_spawn_with_clear_space_of_at_least_500_deploys_a_parachute() {
    let player = &world_with_ground(500.0).players[&1];
    assert!(player.hp > 0, "the respawn put them back");
    assert!(player.parachute, "no ground within 490");
}

#[test]
fn a_spawn_with_ground_inside_490_does_not_deploy() {
    let player = &world_with_ground(489.0).players[&1];
    assert!(player.hp > 0, "the respawn put them back");
    assert!(!player.parachute);
}

#[test]
fn a_deployed_parachute_stores_para_speed_for_the_next_tick() {
    let mut player = airborne(true);
    let steer = Input {
        right: true,
        ..Input::default()
    };
    advance_character(
        &mut player,
        steer,
        &MovementConfig::soldat_default(),
        true,
        1,
    );
    assert!(player.parachute);
    assert_eq!(player.forces.y, -0.03);
    assert_eq!(player.forces.x, 0.03, "left/right steering still applies");
}

#[test]
fn jet_clears_the_parachute_and_thrust_waits_for_the_next_tick() {
    let mut player = airborne(true);
    let jet = Input {
        jet: true,
        ..Input::default()
    };
    let config = MovementConfig::soldat_default();
    advance_character(&mut player, jet, &config, true, 1);
    assert!(!player.parachute);
    assert_ne!(player.forces.y, -0.1);
    advance_character(&mut player, jet, &config, true, 2);
    assert_eq!(player.forces.y, -0.1);
}

#[test]
fn a_snapshot_without_the_flag_still_parses() {
    let player = Player::new(1, "old".into(), 0);
    let mut value = serde_json::to_value(&player).expect("player serializes");
    value.as_object_mut().expect("object").remove("parachute");
    let restored: Player = serde_json::from_value(value).expect("old snapshot parses");
    assert!(!restored.parachute);
}
