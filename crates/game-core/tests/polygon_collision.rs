use game_core::{
    resolve_material_velocity, Aabb, CollisionMask, CollisionPolygon, CollisionWorld, PolygonKind,
    Vec2,
};

fn vertical_wall(kind: PolygonKind) -> CollisionPolygon {
    CollisionPolygon::triangle(
        [
            Vec2 { x: 5.0, y: -5.0 },
            Vec2 { x: 5.0, y: 5.0 },
            Vec2 { x: 6.0, y: 0.0 },
        ],
        kind,
    )
}

#[test]
fn aabb_query_returns_overlapping_polygons_in_source_order() {
    let world = CollisionWorld::new(vec![
        vertical_wall(PolygonKind::Normal),
        CollisionPolygon::triangle(
            [
                Vec2 { x: 50.0, y: 50.0 },
                Vec2 { x: 55.0, y: 50.0 },
                Vec2 { x: 50.0, y: 55.0 },
            ],
            PolygonKind::Normal,
        ),
    ]);

    assert_eq!(world.query_aabb(Aabb::new(4.0, -1.0, 7.0, 1.0)), vec![0]);
}

#[test]
fn raycast_returns_the_first_polygon_contact() {
    let world = CollisionWorld::new(vec![vertical_wall(PolygonKind::Normal)]);

    let hit = world
        .raycast(
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 10.0, y: 0.0 },
            CollisionMask::PLAYER,
        )
        .unwrap();

    assert!((hit.time - 0.5).abs() < 0.0001);
    assert!((hit.position.x - 5.0).abs() < 0.0001);
    assert_eq!(hit.polygon, 0);
}

#[test]
fn polygon_masks_distinguish_players_from_bullets() {
    let world = CollisionWorld::new(vec![vertical_wall(PolygonKind::OnlyBullets)]);

    assert!(world
        .raycast(
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 10.0, y: 0.0 },
            CollisionMask::PLAYER,
        )
        .is_none());
    assert!(world
        .raycast(
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 10.0, y: 0.0 },
            CollisionMask::BULLET,
        )
        .is_some());
}

#[test]
fn one_way_polygon_blocks_downward_motion_from_above_only() {
    let floor = CollisionPolygon::triangle(
        [
            Vec2 { x: 0.0, y: 5.0 },
            Vec2 { x: 10.0, y: 5.0 },
            Vec2 { x: 5.0, y: 6.0 },
        ],
        PolygonKind::OneWay,
    );
    let world = CollisionWorld::new(vec![floor]);

    assert!(world
        .raycast(
            Vec2 { x: 5.0, y: 0.0 },
            Vec2 { x: 5.0, y: 10.0 },
            CollisionMask::PLAYER,
        )
        .is_some());
    assert!(world
        .raycast(
            Vec2 { x: 5.0, y: 10.0 },
            Vec2 { x: 5.0, y: 0.0 },
            CollisionMask::PLAYER,
        )
        .is_none());
}

#[test]
fn swept_circle_stops_at_radius_before_a_vertical_wall() {
    let world = CollisionWorld::new(vec![vertical_wall(PolygonKind::Normal)]);

    let hit = world
        .sweep_circle(
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 10.0, y: 0.0 },
            1.0,
            CollisionMask::PLAYER,
        )
        .unwrap();

    assert!((hit.time - 0.4).abs() < 0.0001);
    assert!((hit.position.x - 4.0).abs() < 0.0001);
    assert!((hit.normal.x + 1.0).abs() < 0.0001);
    assert!(hit.normal.y.abs() < 0.0001);
    let manifold = hit.manifold();
    assert_eq!(manifold.contacts.len(), 1);
    assert_eq!(manifold.contacts[0].polygon, 0);
}

#[test]
fn bouncy_material_reflects_velocity_and_deadly_material_marks_contact() {
    let incoming = Vec2 { x: 10.0, y: 4.0 };
    let normal = Vec2 { x: -1.0, y: 0.0 };

    let bounce = resolve_material_velocity(incoming, normal, PolygonKind::Bouncy);
    let deadly = resolve_material_velocity(incoming, normal, PolygonKind::Deadly);

    assert!((bounce.velocity.x + 8.0).abs() < 0.0001);
    assert!((bounce.velocity.y - 4.0).abs() < 0.0001);
    assert!(!bounce.deadly);
    assert!(deadly.deadly);
    assert!(deadly.velocity.x.abs() < 0.0001);
}

#[test]
fn ice_preserves_more_tangential_velocity_than_normal_ground() {
    let incoming = Vec2 { x: 4.0, y: 10.0 };
    let normal = Vec2 { x: 0.0, y: -1.0 };

    let normal_ground = resolve_material_velocity(incoming, normal, PolygonKind::Normal);
    let ice = resolve_material_velocity(incoming, normal, PolygonKind::Ice);

    assert!(ice.velocity.x > normal_ground.velocity.x);
    assert!(ice.velocity.y.abs() < 0.0001);
}
