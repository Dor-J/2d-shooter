use game_core::{
    BodyRegion, BodyShape, CollisionMask, CollisionPolygon, CollisionWorld, DynamicBody,
    DynamicBodyKind, PolygonKind, Vec2,
};

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
