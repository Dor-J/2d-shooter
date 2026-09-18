//! Proof that the flag modes run inside `World`, not only as a library of policies.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:mode_world:flags     — flags are placed, carried, and scored inside a real world
//!   rust:mode_world:pointmatch — the held-flag kill bonus reaches the ledger
//!   rust:mode_world:physics   — bullets and blasts shove flags about

use game_core::{
    BodyRegion, DamageCause, DamageEvent, FlagKind, Input, ModeKind, Projectile, ProjectileKind,
    Vec2, WeaponKind, World,
};
use std::collections::BTreeMap;

fn idle(world: &mut World, ticks: u32) {
    for _ in 0..ticks {
        world.step(&BTreeMap::new());
    }
}

/// Knocks the flag loose, since a flag standing on its own base is deliberately immovable.
fn loosen_flag(world: &mut World) -> Vec2 {
    let at = world.objectives.flags[0].base;
    world.objectives.flags[0].take(99);
    world.objectives.flags[0].drop_at(at, Vec2::default());
    at
}

fn kill(attacker: u32, target: u32) -> DamageEvent {
    DamageEvent {
        attacker: Some(attacker),
        target,
        amount: 500,
        region: BodyRegion::Chest,
        cause: DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
        pre_scaled: false,
    }
}

/// A world in `mode` with its flags placed and two players on it.
fn world_for(mode: ModeKind) -> World {
    let mut world = World::new(mode.id());
    world.rules = game_core::ModeRules::new(mode);
    world.place_objectives();
    world
}

#[test]
fn a_deathmatch_world_has_no_flags_and_a_flag_mode_does() {
    assert!(world_for(ModeKind::Deathmatch).objectives.flags.is_empty());
    assert!(world_for(ModeKind::Teammatch).objectives.flags.is_empty());

    let ctf = world_for(ModeKind::CaptureTheFlag);
    assert_eq!(ctf.objectives.flags.len(), 2);
    assert!(ctf.objectives.flag(FlagKind::Alpha).is_some());
    assert!(ctf.objectives.flag(FlagKind::Bravo).is_some());

    let htf = world_for(ModeKind::HoldTheFlag);
    assert_eq!(htf.objectives.flags.len(), 1);
    assert_eq!(htf.objectives.flags[0].kind, FlagKind::Yellow);
}

#[test]
fn the_two_team_flags_are_placed_apart_on_their_own_sides() {
    let world = world_for(ModeKind::CaptureTheFlag);
    let alpha = world.objectives.flag(FlagKind::Alpha).unwrap().base;
    let bravo = world.objectives.flag(FlagKind::Bravo).unwrap().base;
    assert_ne!(alpha, bravo, "the bases are not the same place");
    assert!(
        (alpha.x - bravo.x).abs() > 100.0,
        "they are a map apart: {alpha:?} {bravo:?}"
    );
}

#[test]
fn a_player_standing_on_the_enemy_flag_picks_it_up_as_the_world_ticks() {
    let mut world = world_for(ModeKind::CaptureTheFlag);
    world.add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), game_core::TeamChoice::Bravo);

    let bravo_base = world.objectives.flag(FlagKind::Bravo).unwrap().base;
    world.players.get_mut(&1).unwrap().pos = bravo_base;
    idle(&mut world, 1);

    assert_eq!(
        world
            .objectives
            .flag(FlagKind::Bravo)
            .unwrap()
            .state
            .carrier(),
        Some(1)
    );
    assert!(world.objectives.is_missing(FlagKind::Bravo));
}

#[test]
fn a_capture_inside_a_real_world_scores_for_the_team() {
    let mut world = world_for(ModeKind::CaptureTheFlag);
    world.rules.countdown_ticks = 1;
    world.add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), game_core::TeamChoice::Bravo);

    let alpha_base = world.objectives.flag(FlagKind::Alpha).unwrap().base;
    let bravo_base = world.objectives.flag(FlagKind::Bravo).unwrap().base;

    world.players.get_mut(&1).unwrap().pos = bravo_base;
    idle(&mut world, 1);
    assert_eq!(
        world
            .objectives
            .flag(FlagKind::Bravo)
            .unwrap()
            .state
            .carrier(),
        Some(1)
    );

    world.players.get_mut(&1).unwrap().pos = alpha_base;
    idle(&mut world, 1);

    assert_eq!(world.scores[0], 1, "alpha captured");
    assert_eq!(world.ledger.player(1).objectives, 1);
    assert!(
        !world.objectives.is_missing(FlagKind::Bravo),
        "and the flag went home"
    );
}

#[test]
fn a_carrier_who_dies_drops_the_flag_where_they_fell() {
    let mut world = world_for(ModeKind::CaptureTheFlag);
    world.add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), game_core::TeamChoice::Bravo);

    let bravo_base = world.objectives.flag(FlagKind::Bravo).unwrap().base;
    world.players.get_mut(&1).unwrap().pos = bravo_base;
    idle(&mut world, 1);
    let fell_at = Vec2 { x: 600.0, y: 300.0 };
    world.players.get_mut(&1).unwrap().pos = fell_at;

    world.apply_damage(kill(2, 1));
    idle(&mut world, 1);

    let flag = world.objectives.flag(FlagKind::Bravo).unwrap();
    assert!(flag.state.is_dropped(), "it left their hands");
    assert!(flag.state.carrier().is_none());
    assert!(
        (flag.pos().x - fell_at.x).abs() < 50.0,
        "and it is near where they died: {:?}",
        flag.pos()
    );
}

#[test]
fn hold_the_flag_pays_the_carriers_side_over_time_inside_a_real_world() {
    let mut world = world_for(ModeKind::HoldTheFlag);
    world.rules.countdown_ticks = 1;
    world.add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), game_core::TeamChoice::Bravo);

    let middle = world.objectives.flags[0].base;
    world.players.get_mut(&1).unwrap().pos = middle;
    idle(&mut world, 1);
    assert_eq!(world.objectives.flags[0].state.carrier(), Some(1));
    assert_eq!(world.scores[0], 0);

    // Hold it long enough for the timer to pay out twice.
    for _ in 0..700 {
        world.players.get_mut(&1).unwrap().pos = middle;
        idle(&mut world, 1);
    }
    assert!(
        world.scores[0] >= 2,
        "alpha banked points: {:?}",
        world.scores
    );
    assert_eq!(world.scores[1], 0, "and bravo got none");
}

#[test]
fn pointmatch_pays_double_for_a_kill_made_while_holding_the_flag() {
    let mut world = world_for(ModeKind::Pointmatch);
    world.add_player(1, "Holder".into());
    world.add_player(2, "Victim".into());

    // A kill with no flag is worth one point.
    world.apply_damage(kill(1, 2));
    let plain = world.ledger.player(1).points;
    assert_eq!(plain, 1);

    // Pick up the flag, then kill again.
    let middle = world.objectives.flags[0].base;
    world.players.get_mut(&1).unwrap().pos = middle;
    idle(&mut world, 1);
    assert!(world.objectives.is_carrying(1), "they have the flag");

    world.players.get_mut(&2).unwrap().hp = 100;
    world.players.get_mut(&2).unwrap().respawn = 0;
    world.apply_damage(kill(1, 2));

    assert_eq!(
        world.ledger.player(1).points - plain,
        2,
        "the second kill was worth double"
    );
}

#[test]
fn a_bullet_shoves_a_loose_flag_it_passes_through() {
    let mut world = world_for(ModeKind::HoldTheFlag);
    world.add_player(1, "Shooter".into());
    let flag_pos = loosen_flag(&mut world);
    world.objectives.flags[0].body.vel = Vec2::default();

    let from = Vec2 {
        x: flag_pos.x - 60.0,
        y: flag_pos.y,
    };
    world.projectiles.push(Projectile {
        id: 1,
        owner: 1,
        pos: from,
        vel: Vec2 { x: 3600.0, y: 0.0 },
        ttl: 60,
        damage: 0,
        explosive: false,
        kind: ProjectileKind::Bullet,
        splash_radius: 0.0,
        weapon: Some(WeaponKind::Ak74),
        origin: from,
        last_impact: from,
    });
    world.step(&BTreeMap::new());

    assert!(
        world.objectives.flags[0].body.vel.x > 0.0,
        "the round knocked it along: {:?}",
        world.objectives.flags[0].body.vel
    );
}

#[test]
fn a_flag_standing_on_its_own_base_is_not_blown_off_it() {
    let mut world = world_for(ModeKind::HoldTheFlag);
    world.add_player(1, "Shooter".into());
    let flag_pos = world.objectives.flags[0].base;

    let from = Vec2 {
        x: flag_pos.x - 60.0,
        y: flag_pos.y,
    };
    world.projectiles.push(Projectile {
        id: 1,
        owner: 1,
        pos: from,
        vel: Vec2 { x: 3600.0, y: 0.0 },
        ttl: 60,
        damage: 0,
        explosive: false,
        kind: ProjectileKind::Bullet,
        splash_radius: 0.0,
        weapon: Some(WeaponKind::Ak74),
        origin: from,
        last_impact: from,
    });
    world.step(&BTreeMap::new());

    assert_eq!(
        world.objectives.flags[0].pos(),
        flag_pos,
        "a flag at home stays put however much is shot at it"
    );
}

#[test]
fn a_blast_throws_a_loose_flag_away_from_itself() {
    let mut world = world_for(ModeKind::HoldTheFlag);
    world.add_player(1, "Bomber".into());
    let flag_pos = loosen_flag(&mut world);
    world.objectives.flags[0].body.vel = Vec2::default();

    let at = Vec2 {
        x: flag_pos.x - 20.0,
        y: flag_pos.y,
    };
    world.projectiles.push(Projectile {
        id: 1,
        owner: 1,
        pos: at,
        vel: Vec2 { x: 0.0, y: 0.0 },
        ttl: 1,
        damage: 0,
        explosive: true,
        kind: ProjectileKind::M79Grenade,
        splash_radius: 85.0,
        weapon: Some(WeaponKind::M79),
        origin: at,
        last_impact: at,
    });
    world.step(&BTreeMap::new());

    assert!(
        world.objectives.flags[0].body.vel.x > 0.0,
        "it was thrown away from the blast: {:?}",
        world.objectives.flags[0].body.vel
    );
}

#[test]
fn restarting_a_flag_match_puts_the_flags_back_too() {
    let mut world = world_for(ModeKind::CaptureTheFlag);
    world.add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
    let bravo_base = world.objectives.flag(FlagKind::Bravo).unwrap().base;
    world.players.get_mut(&1).unwrap().pos = bravo_base;
    idle(&mut world, 1);
    assert!(world.objectives.is_missing(FlagKind::Bravo));

    world.restart_match();
    assert!(
        !world.objectives.is_missing(FlagKind::Bravo),
        "a restart is a clean slate for the flags as well as the score"
    );
}

#[test]
fn a_carrier_who_throws_the_flag_does_not_pick_it_straight_back_up() {
    let mut world = world_for(ModeKind::CaptureTheFlag);
    world.add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
    let bravo_base = world.objectives.flag(FlagKind::Bravo).unwrap().base;
    world.players.get_mut(&1).unwrap().pos = bravo_base;
    idle(&mut world, 1);
    assert_eq!(
        world
            .objectives
            .flag(FlagKind::Bravo)
            .unwrap()
            .state
            .carrier(),
        Some(1)
    );

    let throwing = BTreeMap::from([(
        1,
        Input {
            throw_weapon: true,
            aim: Vec2 { x: 0.0, y: 400.0 },
            ..Input::default()
        },
    )]);
    world.step(&throwing);
    assert!(
        world
            .objectives
            .flag(FlagKind::Bravo)
            .unwrap()
            .state
            .is_dropped(),
        "the throw let go of it"
    );

    // Standing still on top of it for a tick must not hand it straight back.
    world.step(&BTreeMap::new());
    assert!(
        world
            .objectives
            .flag(FlagKind::Bravo)
            .unwrap()
            .state
            .carrier()
            != Some(1),
        "the grab cooldown stopped them snatching it back"
    );
}
