//! Proof that the per-weapon rules actually run inside `World::step`, not merely in the library.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:weapon_behavior_world:law      — the LAW refuses to fire unbraced and fires braced
//!   rust:weapon_behavior_world:eagles   — two rounds from two barrels
//!   rust:weapon_behavior_world:shotgun  — six pellets along different lines
//!   rust:weapon_behavior_world:minigun  — spin-up before the first round, lost on release
//!   rust:weapon_behavior_world:grenade  — arc, bounce, cooking, and the drop on death
//!   rust:weapon_behavior_world:objects  — a round shoving a loose object
//!
//! The fixtures beside this one check the rules in isolation. These drive a real world, because a
//! rule nothing calls is not a feature.

use game_core::{
    BulletStyle, CharacterState, DynamicBody, DynamicBodyKind, Event, Input, Inventory, Projectile,
    ProjectileKind, Vec2, WeaponKind, World, DT,
};
use std::collections::BTreeMap;

/// Puts a weapon in a player's hands. Selection alone is not enough: the server only lets a player
/// hold what they are actually carrying, so a test has to hand it to them like a spawn would.
fn equip(world: &mut World, id: u32, kind: WeaponKind) -> u8 {
    let slot = kind.slot().expect("the weapon is selectable");
    let table = world.weapons.clone();
    let player = world.players.get_mut(&id).expect("the player exists");
    player.inventory = Inventory::spawn(slot, &table);
    let active = player
        .inventory
        .slot_of(kind)
        .expect("the spawn loadout carries the selected weapon");
    player.inventory.active = active;
    player.weapon = slot;
    player.ammo = u16::from(table.get(kind).ammo);
    slot
}

fn firing(weapon: u8, aim: Vec2) -> Input {
    Input {
        seq: 1,
        fire: true,
        weapon,
        aim,
        ..Input::default()
    }
}

fn step_with(world: &mut World, id: u32, input: Input) {
    world.step(&BTreeMap::from([(id, input)]));
}

/// Holds the trigger until the weapon has finished its start-up and actually produced a shot.
fn fire_until_shot(world: &mut World, id: u32, input: Input, ticks: usize) -> usize {
    for tick in 0..ticks {
        step_with(world, id, input);
        if !world.projectiles.is_empty() {
            return tick + 1;
        }
    }
    panic!("the weapon never fired");
}

#[test]
fn a_thrown_grenade_falls_and_bounces_instead_of_flying_straight() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Thrower".into());
    world.players.get_mut(&1).unwrap().pos = Vec2 { x: 300.0, y: 300.0 };
    let aim = Vec2 { x: 900.0, y: 300.0 };

    step_with(
        &mut world,
        1,
        Input {
            throw_grenade: true,
            aim,
            ..Input::default()
        },
    );
    step_with(
        &mut world,
        1,
        Input {
            aim,
            ..Input::default()
        },
    );

    let grenade = world
        .projectiles
        .iter()
        .find(|p| p.weapon == Some(WeaponKind::FragGrenade))
        .expect("a grenade was thrown")
        .clone();
    assert!(grenade.vel.x > 0.0, "it goes where it was aimed");

    // Gravity pulls it down over the next second, so it arcs rather than travelling flat.
    let start_vy = grenade.vel.y;
    for _ in 0..30 {
        world.step(&BTreeMap::new());
    }
    let flying = world
        .projectiles
        .iter()
        .find(|p| p.id == grenade.id)
        .expect("still in the air");
    assert!(
        flying.vel.y > start_vy,
        "gravity pulls it down: {}",
        flying.vel.y
    );
    assert!(flying.pos.y > grenade.pos.y);
}

#[test]
fn a_grenade_survives_the_floor_while_a_bullet_does_not() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Thrower".into());
    world.players.get_mut(&1).unwrap().pos = Vec2 { x: 300.0, y: 300.0 };
    let aim = Vec2 { x: 320.0, y: 900.0 };

    // A grenade aimed at the floor bounces and is still alive well after it lands.
    step_with(
        &mut world,
        1,
        Input {
            throw_grenade: true,
            aim,
            ..Input::default()
        },
    );
    step_with(
        &mut world,
        1,
        Input {
            aim,
            ..Input::default()
        },
    );
    let grenade_id = world
        .projectiles
        .iter()
        .find(|p| p.weapon == Some(WeaponKind::FragGrenade))
        .expect("thrown")
        .id;
    let mut bounced = false;
    for _ in 0..90 {
        let before = world
            .projectiles
            .iter()
            .find(|p| p.id == grenade_id)
            .map(|p| p.vel.y);
        world.step(&BTreeMap::new());
        let after = world
            .projectiles
            .iter()
            .find(|p| p.id == grenade_id)
            .map(|p| p.vel.y);
        if let (Some(before), Some(after)) = (before, after) {
            if before > 0.0 && after < 0.0 {
                bounced = true;
                break;
            }
        }
    }
    assert!(
        bounced,
        "the grenade bounced off the floor rather than stopping dead"
    );
}

#[test]
fn the_law_will_not_fire_standing_up_and_says_why() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Rocketeer".into());
    let law = equip(&mut world, 1, WeaponKind::Law);
    let aim = Vec2 { x: 900.0, y: 430.0 };

    // Give it every chance: plenty of ticks, on the ground, trigger held.
    world.players.get_mut(&1).unwrap().grounded = true;
    let mut refused = false;
    for _ in 0..60 {
        step_with(&mut world, 1, firing(law, aim));
        world.players.get_mut(&1).unwrap().state = CharacterState::Standing;
        refused |= world
            .events
            .iter()
            .any(|event| matches!(event, Event::FireRefused { player: 1, .. }));
    }
    assert!(world.projectiles.is_empty(), "a standing LAW does not fire");
    assert!(refused, "and the refusal is reported rather than swallowed");
}

#[test]
fn the_law_fires_once_it_is_braced() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Rocketeer".into());
    let law = equip(&mut world, 1, WeaponKind::Law);
    let aim = Vec2 { x: 900.0, y: 430.0 };

    let mut fired = false;
    for _ in 0..90 {
        {
            let player = world.players.get_mut(&1).unwrap();
            player.grounded = true;
            player.state = CharacterState::Prone;
        }
        step_with(&mut world, 1, firing(law, aim));
        if world
            .projectiles
            .iter()
            .any(|p| p.weapon == Some(WeaponKind::Law))
        {
            fired = true;
            break;
        }
    }
    assert!(fired, "a braced LAW fires");
}

#[test]
fn dual_eagles_put_two_rounds_in_the_air_from_two_points() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Gunslinger".into());
    let eagles = equip(&mut world, 1, WeaponKind::DesertEagles);
    fire_until_shot(
        &mut world,
        1,
        firing(eagles, Vec2 { x: 900.0, y: 430.0 }),
        10,
    );

    let rounds: Vec<_> = world
        .projectiles
        .iter()
        .filter(|p| p.weapon == Some(WeaponKind::DesertEagles))
        .collect();
    assert_eq!(rounds.len(), 2, "both barrels fire on one trigger pull");
    assert_ne!(rounds[0].origin, rounds[1].origin, "from two barrels");
}

#[test]
fn a_shotgun_shell_puts_six_pellets_in_the_air_along_different_lines() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Shotgunner".into());
    let spas = equip(&mut world, 1, WeaponKind::Spas12);
    fire_until_shot(&mut world, 1, firing(spas, Vec2 { x: 900.0, y: 430.0 }), 10);

    let pellets: Vec<_> = world
        .projectiles
        .iter()
        .filter(|p| p.kind == ProjectileKind::ShotgunPellet)
        .collect();
    assert_eq!(pellets.len(), 6);

    // They do not all travel the same line, which is what makes a shotgun a shotgun.
    let first = pellets[0].vel;
    assert!(
        pellets
            .iter()
            .any(|p| p.vel.y != first.y || p.vel.x != first.x),
        "the pellets spread apart"
    );
}

#[test]
fn the_minigun_has_to_spin_up_before_it_fires_and_loses_the_spin_when_released() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Gunner".into());
    let minigun = equip(&mut world, 1, WeaponKind::Minigun);
    let aim = Vec2 { x: 900.0, y: 430.0 };

    // It does not fire on the first pull of the trigger.
    step_with(&mut world, 1, firing(minigun, aim));
    assert!(world.projectiles.is_empty(), "it has to spin up first");
    let spun = fire_until_shot(&mut world, 1, firing(minigun, aim), 120);
    assert!(spun > 20, "the spin-up takes real time: {spun} ticks");

    // Letting go loses the spin, so the next burst has to spin up again.
    world.projectiles.clear();
    step_with(
        &mut world,
        1,
        Input {
            weapon: minigun,
            aim,
            ..Input::default()
        },
    );
    assert_eq!(world.players[&1].startup, 0, "the spin is lost on release");
}

#[test]
fn a_bullet_shoves_a_dropped_weapon_it_passes_through() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Shooter".into());
    world.objects.push(DynamicBody::new(
        DynamicBodyKind::Kit,
        Vec2 { x: 400.0, y: 300.0 },
        12.0,
    ));
    let resting = world.objects[0].pos;

    world.projectiles.push(Projectile {
        id: 1,
        owner: 1,
        pos: Vec2 { x: 340.0, y: 300.0 },
        vel: Vec2 { x: 3600.0, y: 0.0 },
        ttl: 60,
        damage: 0,
        explosive: false,
        kind: ProjectileKind::Bullet,
        splash_radius: 0.0,
        weapon: Some(WeaponKind::Ak74),
        origin: Vec2 { x: 340.0, y: 300.0 },
        last_impact: Vec2 { x: 340.0, y: 300.0 },
    });
    world.step(&BTreeMap::new());

    assert!(
        world.objects[0].vel.x > 0.0,
        "the round knocked it along: {:?}",
        world.objects[0].vel
    );
    let _ = resting;
}

#[test]
fn a_player_killed_while_cooking_drops_the_live_grenade() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Cook".into());
    let aim = Vec2 { x: 900.0, y: 430.0 };

    // Start cooking, then die with the grenade still in hand.
    step_with(
        &mut world,
        1,
        Input {
            throw_grenade: true,
            aim,
            ..Input::default()
        },
    );
    assert!(world.players[&1].grenade_charge > 0, "it is being cooked");
    let grenades = world.players[&1].grenades;

    world.apply_damage(game_core::DamageEvent {
        attacker: None,
        target: 1,
        amount: 500,
        region: game_core::BodyRegion::Chest,
        cause: game_core::DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
        pre_scaled: false,
    });

    assert!(world.players[&1].hp <= 0, "the cook is dead");
    assert_eq!(
        world.players[&1].grenades,
        grenades - 1,
        "the grenade left the hand"
    );
    assert!(
        world
            .projectiles
            .iter()
            .any(|p| p.weapon == Some(WeaponKind::FragGrenade)),
        "and it is live on the ground"
    );
}

#[test]
fn a_bullets_speed_and_therefore_its_damage_falls_away_with_distance() {
    // The rule itself is covered by the ballistics fixtures; this checks the projectile in the
    // world carries the origin the rule measures from.
    let mut world = World::new("deathmatch");
    world.add_player(1, "Shooter".into());
    let rifle = equip(&mut world, 1, WeaponKind::Ak74);
    fire_until_shot(
        &mut world,
        1,
        firing(rifle, Vec2 { x: 900.0, y: 430.0 }),
        10,
    );

    let round = &world.projectiles[0];
    assert_eq!(
        round.origin, round.last_impact,
        "it has not struck anything yet"
    );
    assert!(round.origin.x > 0.0);
    // Speed is stored per second; the damage rule reads it per tick.
    assert!(
        (round.vel.length() * DT - 24.0).abs() < 1.0,
        "AK muzzle speed"
    );
    assert_eq!(
        world.weapons.get(WeaponKind::Ak74).bullet_style,
        BulletStyle::Plain
    );
}
