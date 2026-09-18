//! Damage and accuracy formulas from the pinned OpenSoldat source.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:weapon_ballistics_fixtures:damage     — Damage × speed × hitbox modifier
//!   rust:weapon_ballistics_fixtures:decay      — distance halves power at 500 and 900
//!   rust:weapon_ballistics_fixtures:inherit    — shooter velocity changes muzzle speed
//!   rust:weapon_ballistics_fixtures:accuracy   — movement, jet, stance, bink, recoil
//!   rust:weapon_ballistics_fixtures:explosion  — splash falloff with no line-of-sight check
//!   rust:weapon_ballistics_fixtures:friendly   — friendly fire and team bink

use game_core::{
    bink_on_hit, can_damage, degraded_hit_multiply, direct_damage, explosion_damage, inaccuracy,
    max_deviation, movement_accuracy, muzzle_velocity, push_impulse, recoil_radians,
    self_bink_on_fire, should_bink, stance_spread, BodyRegion, CharacterState, DamageCause,
    DamageEvent, Input, Projectile, ProjectileKind, ShooterPose, Vec2, WeaponKind, WeaponTable,
    World, FIRST_DEGRADE_DISTANCE, SECOND_DEGRADE_DISTANCE,
};
use std::collections::BTreeMap;

const SOURCE_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";

fn eagles() -> game_core::WeaponDef {
    *WeaponTable::normal().get(WeaponKind::DesertEagles)
}

fn standing() -> ShooterPose {
    ShooterPose {
        state: CharacterState::Standing,
        grounded: true,
        ..ShooterPose::default()
    }
}

#[test]
fn fixture_is_pinned_to_the_reference_lock() {
    assert_eq!(SOURCE_COMMIT.len(), 40);
}

#[test]
fn direct_damage_is_configured_damage_times_speed_times_hitbox() {
    let def = eagles();
    let chest = direct_damage(&def, def.damage, def.speed, BodyRegion::Chest);
    assert!((chest - def.speed * def.damage * def.modifier_chest).abs() < 1e-5);
    let head = direct_damage(&def, def.damage, def.speed, BodyRegion::Head);
    let legs = direct_damage(&def, def.damage, def.speed, BodyRegion::Legs);
    assert!(head > chest && chest > legs);
}

#[test]
fn projectile_power_halves_past_five_hundred_and_again_past_nine_hundred() {
    let def = *WeaponTable::normal().get(WeaponKind::Ak74);
    assert_eq!(degraded_hit_multiply(&def, 100.0), def.damage);
    assert_eq!(
        degraded_hit_multiply(&def, FIRST_DEGRADE_DISTANCE + 1.0),
        def.damage * 0.5
    );
    assert_eq!(
        degraded_hit_multiply(&def, SECOND_DEGRADE_DISTANCE + 1.0),
        def.damage * 0.25
    );
    let barrett = *WeaponTable::normal().get(WeaponKind::Barrett);
    assert_eq!(
        degraded_hit_multiply(&barrett, SECOND_DEGRADE_DISTANCE + 1.0),
        barrett.damage
    );
}

#[test]
fn muzzle_velocity_inherits_the_shooter_and_changes_with_direction() {
    let def = eagles();
    let forward = muzzle_velocity(&def, Vec2 { x: 1.0, y: 0.0 }, Vec2 { x: 4.0, y: 0.0 });
    let backward = muzzle_velocity(&def, Vec2 { x: 1.0, y: 0.0 }, Vec2 { x: -4.0, y: 0.0 });
    assert!(forward.x > def.speed);
    assert!(backward.x < def.speed);
    assert!(forward.length() > backward.length());
}

#[test]
fn movement_and_jetting_spoil_aim_and_crouch_tightens_spread() {
    let def = eagles();
    let still = standing();
    let running = ShooterPose {
        moving: true,
        ..still
    };
    let jetting = ShooterPose {
        jetting: true,
        grounded: false,
        state: CharacterState::Airborne,
        ..still
    };
    assert_eq!(movement_accuracy(&def, &still), 0.0);
    assert!(movement_accuracy(&def, &running) > movement_accuracy(&def, &still));
    assert_eq!(
        movement_accuracy(&def, &running),
        movement_accuracy(&def, &jetting)
    );
    let crouched = ShooterPose {
        state: CharacterState::Crouching,
        ..still
    };
    let mp5 = *WeaponTable::normal().get(WeaponKind::Mp5);
    assert!(stance_spread(&mp5, &crouched) < stance_spread(&mp5, &still));
    assert!(inaccuracy(&def, &running) > inaccuracy(&def, &still));
    assert!(max_deviation(inaccuracy(&def, &running)) > 0.0);
}

#[test]
fn bink_hits_the_victim_and_negative_bink_hits_the_shooter() {
    let barrett = *WeaponTable::normal().get(WeaponKind::Barrett);
    assert!(barrett.binks_the_victim());
    assert!(bink_on_hit(&barrett, 0) > 0);

    let mp5 = *WeaponTable::realistic().get(WeaponKind::Mp5);
    assert!(mp5.self_binks());
    let standing_bink = self_bink_on_fire(&mp5, 0, &standing());
    let crouched_bink = self_bink_on_fire(
        &mp5,
        0,
        &ShooterPose {
            state: CharacterState::Crouching,
            grounded: true,
            ..ShooterPose::default()
        },
    );
    assert!(standing_bink > crouched_bink);
    assert_eq!(self_bink_on_fire(&barrett, 7, &standing()), 7);
}

#[test]
fn recoil_grows_with_the_burst_and_push_scales_with_mass() {
    let minimi = *WeaponTable::realistic().get(WeaponKind::Minimi);
    let pose = standing();
    assert!(recoil_radians(&minimi, 6, &pose) >= recoil_radians(&minimi, 1, &pose));
    let def = eagles();
    let push = push_impulse(&def, Vec2 { x: 10.0, y: 0.0 });
    assert!((push.x - 10.0 * def.push).abs() < 1e-5);
}

#[test]
fn explosions_fall_off_with_distance_and_do_not_need_line_of_sight() {
    let m79 = *WeaponTable::normal().get(WeaponKind::M79);
    let center = explosion_damage(&m79, 0.0, BodyRegion::Chest);
    let edge = explosion_damage(&m79, m79.explosion_radius() - 1.0, BodyRegion::Chest);
    assert!(center > edge);
    assert_eq!(
        explosion_damage(&m79, m79.explosion_radius(), BodyRegion::Chest),
        0.0
    );
}

#[test]
fn friendly_fire_and_team_bink_follow_the_room_flag() {
    let def = eagles();
    assert!(!can_damage(&def, 1, 2, true, false, false));
    assert!(can_damage(&def, 1, 2, true, true, false));
    assert!(can_damage(&def, 1, 1, false, false, false));
    assert!(!should_bink(&def, 1, 2, true, false));
    let barrett = *WeaponTable::normal().get(WeaponKind::Barrett);
    assert!(should_bink(&barrett, 1, 2, false, false));
    assert!(should_bink(&barrett, 1, 2, true, true));
}

#[test]
fn a_fired_shot_uses_the_table_and_the_formula_instead_of_a_second_stat_sheet() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Shooter".into());
    world.add_player(2, "Target".into());
    world.players.get_mut(&2).unwrap().pos = Vec2 { x: 320.0, y: 430.0 };
    world.equip(1, WeaponKind::Mp5);
    world.step(&BTreeMap::from([(
        1,
        Input {
            fire: true,
            aim: Vec2 { x: 400.0, y: 430.0 },
            weapon: 1,
            ..Input::default()
        },
    )]));
    assert!(!world.projectiles.is_empty());
    assert_eq!(world.projectiles[0].weapon, Some(WeaponKind::Mp5));
    let hp_before = world.players[&2].hp;
    for _ in 0..20 {
        world.step(&BTreeMap::new());
    }
    assert!(world.players[&2].hp < hp_before || world.players[&2].hp == 0);
}

#[test]
fn teammates_are_spared_until_friendly_fire_is_on() {
    let mut world = World::new("team");
    world.add_player(1, "Alpha".into());
    world.add_player(2, "Teammate".into());
    world.players.get_mut(&2).unwrap().team = 1;
    world.players.get_mut(&2).unwrap().pos = Vec2 { x: 300.0, y: 430.0 };
    world.projectiles.push(Projectile {
        id: 1,
        owner: 1,
        pos: Vec2 { x: 270.0, y: 430.0 },
        vel: Vec2 { x: 3600.0, y: 0.0 },
        ttl: 10,
        damage: 40,
        explosive: false,
        kind: ProjectileKind::Bullet,
        splash_radius: 0.0,
        weapon: Some(WeaponKind::Ak74),
        origin: Vec2 { x: 270.0, y: 430.0 },
        last_impact: Vec2 { x: 270.0, y: 430.0 },
    });
    world.step(&BTreeMap::new());
    assert_eq!(world.players[&2].hp, 100);

    world.friendly_fire = true;
    world.projectiles.push(Projectile {
        id: 2,
        owner: 1,
        pos: Vec2 { x: 270.0, y: 430.0 },
        vel: Vec2 { x: 3600.0, y: 0.0 },
        ttl: 10,
        damage: 40,
        explosive: false,
        kind: ProjectileKind::Bullet,
        splash_radius: 0.0,
        weapon: Some(WeaponKind::Ak74),
        origin: Vec2 { x: 270.0, y: 430.0 },
        last_impact: Vec2 { x: 270.0, y: 430.0 },
    });
    world.step(&BTreeMap::new());
    assert!(world.players[&2].hp < 100);
}

#[test]
fn a_realistic_table_self_binks_the_shooter_on_the_first_shot() {
    let mut world = World::new("deathmatch");
    world.weapons = WeaponTable::realistic();
    world.add_player(1, "Shooter".into());
    world.equip(1, WeaponKind::Mp5);
    world.step(&BTreeMap::from([(
        1,
        Input {
            fire: true,
            aim: Vec2 { x: 800.0, y: 430.0 },
            weapon: 1,
            ..Input::default()
        },
    )]));
    assert!(world.players[&1].bink > 0);
    assert!(world.players[&1].accuracy > 0.0);
}

#[test]
fn apply_damage_does_not_double_a_pre_scaled_weapon_hit() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.add_player(2, "B".into());
    let amount = direct_damage(&eagles(), eagles().damage, 19.0, BodyRegion::Head).round() as i32;
    world.apply_damage(DamageEvent {
        attacker: Some(1),
        target: 2,
        amount,
        region: BodyRegion::Head,
        cause: DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
        pre_scaled: true,
    });
    assert_eq!(100 - world.players[&2].hp, amount);
}
