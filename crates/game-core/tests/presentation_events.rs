//! Proof that the simulation tells the client what to draw, without ever reading it back.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:presentation:explosion — an explosion is reported once, with where and how big it was
//!   rust:presentation:impact    — a round striking terrain is reported with the surface it struck

use game_core::{
    CollisionPolygon, CollisionWorld, Event, PolygonKind, Projectile, ProjectileKind, Vec2, World,
};
use std::collections::BTreeMap;

fn wall() -> CollisionPolygon {
    CollisionPolygon::triangle(
        [
            Vec2 { x: 300.0, y: 300.0 },
            Vec2 { x: 320.0, y: 300.0 },
            Vec2 { x: 310.0, y: 560.0 },
        ],
        PolygonKind::Normal,
    )
}

fn round(explosive: bool) -> Projectile {
    Projectile {
        id: 1,
        owner: 1,
        pos: Vec2 { x: 250.0, y: 430.0 },
        vel: Vec2 { x: 6_000.0, y: 0.0 },
        ttl: 30,
        damage: 40,
        explosive,
        kind: if explosive {
            ProjectileKind::LawRocket
        } else {
            ProjectileKind::Bullet
        },
        splash_radius: if explosive { 64.0 } else { 0.0 },
        weapon: None,
        origin: Vec2 { x: 250.0, y: 430.0 },
        last_impact: Vec2 { x: 250.0, y: 430.0 },
    }
}

fn explosions(world: &World) -> Vec<(Vec2, f32)> {
    world
        .events
        .iter()
        .filter_map(|event| match event {
            Event::Explosion { pos, radius } => Some((*pos, *radius)),
            _ => None,
        })
        .collect()
}

fn impacts(world: &World) -> Vec<(Vec2, Vec2)> {
    world
        .events
        .iter()
        .filter_map(|event| match event {
            Event::Impact { pos, normal } => Some((*pos, *normal)),
            _ => None,
        })
        .collect()
}

#[test]
fn a_round_that_goes_off_says_where_and_how_big() {
    let mut world = World::with_collision("deathmatch", CollisionWorld::new(vec![wall()]));
    world.add_player(1, "Gunner".into());
    world.projectiles.push(round(true));

    world.step(&BTreeMap::new());

    let fireballs = explosions(&world);
    assert_eq!(fireballs.len(), 1, "one round, one fireball");
    let (pos, radius) = fireballs[0];
    assert_eq!(radius, 64.0, "the client knows how big to draw it");
    assert!(
        pos.x >= 250.0 && pos.x <= 320.0,
        "and where it went off: {pos:?}"
    );
}

#[test]
fn an_explosion_is_reported_once_rather_than_every_tick_after() {
    let mut world = World::with_collision("deathmatch", CollisionWorld::new(vec![wall()]));
    world.add_player(1, "Gunner".into());
    world.projectiles.push(round(true));

    let mut total = 0;
    for _ in 0..30 {
        world.step(&BTreeMap::new());
        total += explosions(&world).len();
    }
    assert_eq!(total, 1);
}

#[test]
fn a_blast_with_no_radius_is_still_something_the_client_can_draw() {
    let mut world = World::with_collision("deathmatch", CollisionWorld::new(vec![wall()]));
    world.add_player(1, "Gunner".into());
    let mut dud = round(true);
    dud.splash_radius = 0.0;
    world.projectiles.push(dud);

    world.step(&BTreeMap::new());

    let (_, radius) = explosions(&world)[0];
    assert!(radius > 0.0, "a fireball of no size is not a fireball");
}

#[test]
fn a_round_that_strikes_terrain_says_which_way_the_surface_faces() {
    let mut world = World::with_collision("deathmatch", CollisionWorld::new(vec![wall()]));
    world.add_player(1, "Gunner".into());
    world.projectiles.push(round(false));

    world.step(&BTreeMap::new());

    let hits = impacts(&world);
    assert_eq!(hits.len(), 1, "one round, one shower of sparks");
    let (pos, normal) = hits[0];
    let length = (normal.x * normal.x + normal.y * normal.y).sqrt();
    assert!(
        (length - 1.0).abs() < 1e-3,
        "a surface normal is a direction, not a distance: {length}"
    );
    assert!(normal.x < 0.0, "and it faces back the way the round came");
    assert!(pos.x <= 320.0 && pos.x >= 250.0, "at the wall: {pos:?}");
}

#[test]
fn an_explosive_round_reports_both_the_wall_it_hit_and_the_blast_it_made() {
    let mut world = World::with_collision("deathmatch", CollisionWorld::new(vec![wall()]));
    world.add_player(1, "Gunner".into());
    world.projectiles.push(round(true));

    world.step(&BTreeMap::new());

    assert_eq!(impacts(&world).len(), 1);
    assert_eq!(explosions(&world).len(), 1);
}

#[test]
fn a_quiet_tick_reports_nothing_to_draw() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Idle".into());
    world.step(&BTreeMap::new());
    assert!(explosions(&world).is_empty());
    assert!(impacts(&world).is_empty());
}

#[test]
fn presentation_events_survive_the_trip_through_json_to_a_client() {
    let event = Event::Explosion {
        pos: Vec2 { x: 12.0, y: -3.5 },
        radius: 64.0,
    };
    let text = serde_json::to_string(&event).expect("an explosion serialises");
    assert!(text.contains("Explosion"), "{text}");
    assert!(text.contains("radius"), "{text}");

    let impact = Event::Impact {
        pos: Vec2 { x: 1.0, y: 2.0 },
        normal: Vec2 { x: 0.0, y: -1.0 },
    };
    let text = serde_json::to_string(&impact).expect("an impact serialises");
    assert!(text.contains("normal"), "{text}");
}
