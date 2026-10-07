use game_core::{
    resolve_material_velocity, BodyRegion, BodyShape, CharacterState, CollisionMask,
    CollisionPolygon, CollisionWorld, Direction, DynamicBody, DynamicBodyKind, Input, PolygonKind,
    Vec2, World,
};
use std::collections::BTreeMap;

fn triangle(vertices: [(f32, f32); 3], kind: PolygonKind) -> CollisionPolygon {
    CollisionPolygon::triangle(vertices.map(|(x, y)| Vec2 { x, y }), kind)
}

#[test]
fn fixtures_cover_floor_ceiling_wall_slope_and_edge_contacts() {
    let world = CollisionWorld::new(vec![
        triangle(
            [(0.0, 20.0), (40.0, 20.0), (20.0, 30.0)],
            PolygonKind::Normal,
        ),
        triangle(
            [(0.0, 0.0), (20.0, -10.0), (40.0, 0.0)],
            PolygonKind::Normal,
        ),
        triangle(
            [(50.0, 0.0), (50.0, 30.0), (60.0, 15.0)],
            PolygonKind::Normal,
        ),
        triangle(
            [(70.0, 20.0), (100.0, 5.0), (100.0, 20.0)],
            PolygonKind::Normal,
        ),
    ]);

    let floor = world
        .sweep_circle(
            Vec2 { x: 10.0, y: 5.0 },
            Vec2 { x: 10.0, y: 30.0 },
            2.0,
            CollisionMask::PLAYER,
        )
        .unwrap();
    assert!(floor.normal.y < -0.9);
    let ceiling = world
        .sweep_circle(
            Vec2 { x: 10.0, y: 15.0 },
            Vec2 { x: 10.0, y: -10.0 },
            2.0,
            CollisionMask::PLAYER,
        )
        .unwrap();
    assert!(ceiling.normal.y > 0.9);
    let wall = world
        .sweep_circle(
            Vec2 { x: 40.0, y: 10.0 },
            Vec2 { x: 70.0, y: 10.0 },
            2.0,
            CollisionMask::PLAYER,
        )
        .unwrap();
    assert!(wall.normal.x < -0.9);
    let slope = world
        .sweep_circle(
            Vec2 { x: 80.0, y: 0.0 },
            Vec2 { x: 80.0, y: 30.0 },
            2.0,
            CollisionMask::PLAYER,
        )
        .unwrap();
    assert!(slope.normal.x < 0.0 && slope.normal.y < 0.0);
    let edge = world
        .sweep_circle(
            Vec2 { x: -5.0, y: 15.0 },
            Vec2 { x: 1.0, y: 19.0 },
            2.0,
            CollisionMask::PLAYER,
        )
        .unwrap();
    assert!(edge.time <= 1.0);
}

#[test]
fn standing_body_exposes_head_chest_and_legs_and_reports_the_first_region() {
    let shape = BodyShape::standing();
    assert_eq!(
        shape.parts().map(|part| part.region).collect::<Vec<_>>(),
        vec![BodyRegion::Head, BodyRegion::Chest, BodyRegion::Legs]
    );
    let ceiling = CollisionWorld::new(vec![triangle(
        [(-20.0, -20.0), (20.0, -20.0), (0.0, -25.0)],
        PolygonKind::Normal,
    )]);

    let hit = ceiling
        .sweep_shape(
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 0.0, y: -20.0 },
            &shape,
            CollisionMask::PLAYER,
        )
        .unwrap();

    assert_eq!(hit.region, BodyRegion::Head);
}

#[test]
fn every_dynamic_object_kind_uses_the_same_polygon_sweep() {
    let floor = CollisionWorld::new(vec![triangle(
        [(-20.0, 10.0), (20.0, 10.0), (0.0, 20.0)],
        PolygonKind::Normal,
    )]);
    for kind in [
        DynamicBodyKind::Corpse,
        DynamicBodyKind::Flag,
        DynamicBodyKind::Kit,
        DynamicBodyKind::DroppedWeapon,
    ] {
        let mut body = DynamicBody::new(kind, Vec2 { x: 0.0, y: 0.0 }, 2.0);
        body.vel.y = 1_000.0;
        body.step(&floor, 1.0 / 60.0);
        assert!(body.pos.y <= 8.01, "{kind:?} tunneled through the floor");
        assert!(body.grounded);
    }
}

#[test]
fn one_way_surface_ignores_side_and_upward_entry() {
    let world = CollisionWorld::new(vec![triangle(
        [(0.0, 10.0), (20.0, 10.0), (10.0, 15.0)],
        PolygonKind::OneWay,
    )]);
    assert!(world
        .sweep_circle(
            Vec2 { x: -5.0, y: 12.0 },
            Vec2 { x: 5.0, y: 12.0 },
            1.0,
            CollisionMask::PLAYER
        )
        .is_none());
    assert!(world
        .sweep_circle(
            Vec2 { x: 10.0, y: 20.0 },
            Vec2 { x: 10.0, y: 0.0 },
            1.0,
            CollisionMask::PLAYER
        )
        .is_none());
    assert!(world
        .sweep_circle(
            Vec2 { x: 10.0, y: 0.0 },
            Vec2 { x: 10.0, y: 20.0 },
            1.0,
            CollisionMask::PLAYER
        )
        .is_some());
}

#[test]
fn standing_clearance_only_looks_at_the_headroom_above_a_low_stance() {
    // A plain floor: the legs of a crouched body already touch it, so the ground must never be
    // mistaken for a ceiling that keeps the player down.
    let open = CollisionWorld::new(vec![triangle(
        [(0.0, 20.0), (80.0, 20.0), (40.0, 60.0)],
        PolygonKind::Normal,
    )]);
    assert!(open.has_standing_clearance(Vec2 { x: 40.0, y: 10.0 }, CollisionMask::PLAYER));

    // The same floor with a ceiling just above the crouched head keeps the player crouched.
    let mut polygons = open.polygons().to_vec();
    polygons.push(triangle(
        [(0.0, -6.0), (80.0, -6.0), (40.0, 2.0)],
        PolygonKind::Normal,
    ));
    let low = CollisionWorld::new(polygons);
    assert!(!low.has_standing_clearance(Vec2 { x: 40.0, y: 10.0 }, CollisionMask::PLAYER));
}

fn flat_floor() -> CollisionWorld {
    CollisionWorld::new(vec![triangle(
        [(0.0, 200.0), (600.0, 200.0), (300.0, 280.0)],
        PolygonKind::Normal,
    )])
}

/// Horizontal speed on the tick the landing jump first carries the body up.
fn landing_jump_speed(facing: Direction) -> f32 {
    let mut world = World::with_collision("deathmatch", flat_floor());
    world.add_player(1, "jumper".into());
    {
        let player = world.players.get_mut(&1).unwrap();
        player.pos = Vec2 { x: 200.0, y: 182.0 };
        player.vel = Vec2 { x: 2.0, y: 1.0 };
        player.facing = facing;
        player.grounded = false;
        player.state = CharacterState::Airborne;
        player.parachute = false;
    }
    let mut jump_held = false;
    for _ in 0..40 {
        let grounded = world.players[&1].grounded;
        world.step(&BTreeMap::from([(
            1,
            Input {
                jump: grounded || jump_held,
                ..Input::default()
            },
        )]));
        jump_held |= grounded;
        let player = &world.players[&1];
        if player.vel.y < 0.0 {
            assert_eq!(player.facing, facing);
            return player.vel.x;
        }
    }
    panic!("landing jump never left the floor");
}

fn spike(peak: Vec2) -> CollisionWorld {
    CollisionWorld::new(vec![triangle(
        [
            (peak.x - 1.0, peak.y + 40.0),
            (peak.x, peak.y),
            (peak.x + 1.0, peak.y + 40.0),
        ],
        PolygonKind::Normal,
    )])
}

#[test]
fn landing_jump_speed_is_identical_either_facing() {
    let forward = landing_jump_speed(Direction::Right);
    let backward = landing_jump_speed(Direction::Left);
    // gap: forward 0.15075698 and backward 0.15075698 (ratio 1); the sweep ignores facing, so the tutorial's ~5% is absent.
    assert_eq!(forward, backward);
    assert_eq!(forward.to_bits(), 1_041_915_914);
    assert_eq!(backward.to_bits(), 1_041_915_914);
}

#[test]
fn clipping_a_polygon_tip_launches_the_body_upward() {
    let peak = Vec2 { x: 50.0, y: 50.0 };
    let start = Vec2 { x: 36.0, y: 40.0 };
    let incoming = Vec2 { x: 8.0, y: 0.0 };
    let shape = BodyShape::standing();
    let hit = spike(peak)
        .sweep_shape(
            start,
            Vec2 {
                x: start.x + incoming.x,
                y: start.y + incoming.y,
            },
            &shape,
            CollisionMask::PLAYER,
        )
        .expect("the standing body reaches the tip");
    let legs = shape
        .parts()
        .find(|part| part.region == BodyRegion::Legs)
        .copied()
        .unwrap();
    let center = Vec2 {
        x: hit.hit.position.x + legs.offset.x,
        y: hit.hit.position.y + legs.offset.y,
    };
    let dx = center.x - peak.x;
    let dy = center.y - peak.y;
    let distance = (dx * dx + dy * dy).sqrt();
    assert_eq!(hit.region, BodyRegion::Legs);
    assert!(
        (distance - legs.radius).abs() < 0.001,
        "contact is the tip, distance {distance}"
    );
    let out = resolve_material_velocity(incoming, hit.hit.normal, hit.hit.kind).velocity;
    // Measured launch: horizontal 8 leaves the sweep at (0.39999962, -1.5491933), speed 1.6.
    assert_eq!(out.x.to_bits(), 1_053_609_152);
    assert_eq!(out.y.to_bits(), 3_217_443_831);

    let mut world = World::with_collision("deathmatch", spike(peak));
    world.add_player(1, "corner".into());
    {
        let player = world.players.get_mut(&1).unwrap();
        player.pos = start;
        player.vel = incoming;
        player.grounded = false;
        player.state = CharacterState::Airborne;
        player.parachute = false;
    }
    world.step(&BTreeMap::new());
    let player = &world.players[&1];
    // Same approach through one player tick: velocity (0.36642534, -1.4554204), speed 1.5008385.
    assert_eq!(player.vel.x.to_bits(), 1_052_482_586);
    assert_eq!(player.vel.y.to_bits(), 3_216_657_207);
}
