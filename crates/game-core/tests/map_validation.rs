use content::pms::{MapAsset, Polygon, Vertex};
use game_core::{
    CollisionMask, CollisionWorld, MapValidationError, PolygonKind, ValidatedMap, Vec2,
};

fn vertex(x: f32, y: f32) -> Vertex {
    Vertex {
        x,
        y,
        z: 0.0,
        rhw: 1.0,
        color: [255; 4],
        u: 0.0,
        v: 0.0,
    }
}

fn map_with_polygon(polygon: Polygon) -> MapAsset {
    MapAsset {
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
        polygons: vec![polygon],
        sectors_division: 50,
        sectors_num: 0,
        sectors: Vec::new(),
        props: Vec::new(),
        scenery: Vec::new(),
        colliders: Vec::new(),
        spawnpoints: Vec::new(),
        waypoints: Vec::new(),
    }
}

#[test]
fn rejects_degenerate_collision_polygons() {
    let polygon = Polygon {
        vertices: [vertex(0.0, 0.0), vertex(1.0, 1.0), vertex(2.0, 2.0)],
        normals: [[0.0; 3]; 3],
        kind: 0,
    };

    let error = ValidatedMap::try_from(map_with_polygon(polygon)).unwrap_err();

    assert_eq!(error, MapValidationError::DegeneratePolygon { index: 0 });
}

#[test]
fn rejects_unknown_pms_polygon_kinds() {
    let polygon = Polygon {
        vertices: [vertex(0.0, 0.0), vertex(10.0, 0.0), vertex(0.0, 10.0)],
        normals: [[0.0; 3]; 3],
        kind: 26,
    };

    assert_eq!(
        ValidatedMap::try_from(map_with_polygon(polygon)).unwrap_err(),
        MapValidationError::InvalidPolygonKind { index: 0, kind: 26 }
    );
}

#[test]
fn accepts_a_non_degenerate_collision_polygon() {
    let polygon = Polygon {
        vertices: [vertex(0.0, 0.0), vertex(10.0, 0.0), vertex(0.0, 10.0)],
        normals: [[0.0; 3]; 3],
        kind: 0,
    };

    let map = ValidatedMap::try_from(map_with_polygon(polygon)).unwrap();

    assert_eq!(map.asset().polygons.len(), 1);
}

#[test]
fn validated_pms_kinds_become_runtime_collision_materials_and_filters() {
    for (kind, expected, player_hit, bullet_hit) in [
        (0, PolygonKind::Normal, true, true),
        (1, PolygonKind::OnlyBullets, false, true),
        (2, PolygonKind::OnlyPlayers, true, false),
        (4, PolygonKind::Ice, true, true),
        (5, PolygonKind::Deadly, true, true),
        (18, PolygonKind::Bouncy, true, true),
    ] {
        let polygon = Polygon {
            vertices: [vertex(5.0, -5.0), vertex(5.0, 5.0), vertex(6.0, 0.0)],
            normals: [[0.0; 3]; 3],
            kind,
        };
        let map = ValidatedMap::try_from(map_with_polygon(polygon)).unwrap();
        let collision = CollisionWorld::from_map(&map);
        assert_eq!(collision.polygons()[0].kind, expected);
        assert_eq!(
            collision
                .raycast(
                    Vec2 { x: 0.0, y: 0.0 },
                    Vec2 { x: 10.0, y: 0.0 },
                    CollisionMask::PLAYER
                )
                .is_some(),
            player_hit
        );
        assert_eq!(
            collision
                .raycast(
                    Vec2 { x: 0.0, y: 0.0 },
                    Vec2 { x: 10.0, y: 0.0 },
                    CollisionMask::BULLET
                )
                .is_some(),
            bullet_hit
        );
    }
}

#[test]
fn non_colliding_pms_types_are_not_added_to_the_collision_world() {
    for kind in [3, 24, 25] {
        let polygon = Polygon {
            vertices: [vertex(0.0, 0.0), vertex(10.0, 0.0), vertex(0.0, 10.0)],
            normals: [[0.0; 3]; 3],
            kind,
        };
        let map = ValidatedMap::try_from(map_with_polygon(polygon)).unwrap();
        assert!(CollisionWorld::from_map(&map).polygons().is_empty());
    }
}
