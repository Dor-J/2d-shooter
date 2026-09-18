use game_core::{
    CollisionPolygon, CollisionWorld, DynamicBody, DynamicBodyKind, Input, PolygonKind, Projectile,
    ProjectileKind, Vec2, World,
};
use std::collections::BTreeMap;

fn vertical_wall(kind: PolygonKind) -> CollisionPolygon {
    CollisionPolygon::triangle(
        [
            Vec2 { x: 300.0, y: 350.0 },
            Vec2 { x: 300.0, y: 550.0 },
            Vec2 { x: 310.0, y: 450.0 },
        ],
        kind,
    )
}

#[test]
fn world_snapshot_exposes_the_same_polygons_used_for_collision() {
    let collision = CollisionWorld::new(vec![vertical_wall(PolygonKind::Ice)]);
    let world = World::with_collision("deathmatch", collision);

    let json = serde_json::to_value(&world).unwrap();

    assert_eq!(json["map_polygons"][0]["kind"], "Ice");
    assert_eq!(json["map_polygons"][0]["vertices"][0]["x"], 300.0);
}

#[test]
fn world_steps_dynamic_objects_through_the_shared_collision_world() {
    let floor = CollisionPolygon::triangle(
        [
            Vec2 { x: 0.0, y: 10.0 },
            Vec2 { x: 30.0, y: 10.0 },
            Vec2 { x: 15.0, y: 20.0 },
        ],
        PolygonKind::Normal,
    );
    let mut world = World::with_collision("deathmatch", CollisionWorld::new(vec![floor]));
    let mut flag = DynamicBody::new(DynamicBodyKind::Flag, Vec2 { x: 15.0, y: 0.0 }, 2.0);
    flag.vel.y = 1_000.0;
    world.objects.push(flag);

    world.step(&BTreeMap::new());

    assert!(world.objects[0].grounded);
    assert!(world.objects[0].pos.y <= 8.01);
}

#[test]
fn world_projectiles_use_loaded_polygon_geometry() {
    let collision = CollisionWorld::new(vec![vertical_wall(PolygonKind::Normal)]);
    let mut world = World::with_collision("deathmatch", collision);
    world.add_player(1, "Shooter".into());
    world.projectiles.push(Projectile {
        id: 1,
        owner: 1,
        pos: Vec2 { x: 250.0, y: 430.0 },
        vel: Vec2 { x: 6_000.0, y: 0.0 },
        ttl: 10,
        damage: 20,
        explosive: false,
        kind: ProjectileKind::Bullet,
        splash_radius: 0.0,
        weapon: None,
        origin: Vec2::default(),
        last_impact: Vec2::default(),
    });

    world.step(&BTreeMap::new());

    assert!(world.projectiles.is_empty());
}

#[test]
fn world_players_do_not_tunnel_through_loaded_vertical_geometry() {
    let collision = CollisionWorld::new(vec![vertical_wall(PolygonKind::Bouncy)]);
    let mut world = World::with_collision("deathmatch", collision);
    world.add_player(1, "Runner".into());
    let player = world.players.get_mut(&1).unwrap();
    player.pos = Vec2 { x: 288.0, y: 430.0 };
    player.vel = Vec2 { x: 2_000.0, y: 0.0 };

    world.step(&BTreeMap::from([(
        1,
        Input {
            right: true,
            ..Default::default()
        },
    )]));

    let player = &world.players[&1];
    assert!(player.pos.x <= 290.01);
    assert!(player.vel.x < 0.0);
}

#[test]
fn deadly_loaded_geometry_kills_the_player_on_contact() {
    let collision = CollisionWorld::new(vec![vertical_wall(PolygonKind::Deadly)]);
    let mut world = World::with_collision("deathmatch", collision);
    world.add_player(1, "Runner".into());
    let player = world.players.get_mut(&1).unwrap();
    player.pos = Vec2 { x: 288.0, y: 430.0 };
    player.vel.x = 2_000.0;

    world.step(&BTreeMap::from([(
        1,
        Input {
            right: true,
            ..Default::default()
        },
    )]));

    assert_eq!(world.players[&1].hp, 0);
}
