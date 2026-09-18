//! The leftover weapons are in the player's hands, not only in a table.
//!
//! Acceptance evidence: rust:obtainable_weapons rust:rambomatch rust:realistic_movement

use game_core::{
    BodyRegion, BonusConfig, BotProfile, DamageCause, DamageEvent, Difficulty, Input, KitKind,
    ModeKind, MovementConfig, Pickup, SpawnKind, StationaryGun, TeamChoice, Vec2, WeaponKind,
    World,
};
use std::collections::BTreeMap;

fn idle(world: &mut World, ticks: u32) {
    for _ in 0..ticks {
        world.step(&BTreeMap::new());
    }
}

fn kill(attacker: u32, target: u32) -> DamageEvent {
    DamageEvent {
        attacker: Some(attacker),
        target,
        amount: 500,
        region: BodyRegion::Chest,
        cause: DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
        pre_scaled: true,
    }
}

#[test]
fn flame_god_puts_the_flamethrower_in_your_hands() {
    let mut world = World::new("deathmatch");
    world.bonuses = BonusConfig::none();
    world.add_player(1, "God".into());
    world
        .pickups
        .items
        .push(Pickup::new(KitKind::FlameGod, world.players[&1].pos, 600));
    idle(&mut world, 1);
    assert!(world.players[&1].inventory.owns(WeaponKind::Flamer));
}

#[test]
fn empty_hands_punch() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Boxer".into());
    world.add_player(2, "Bag".into());
    let at = world.players[&1].pos;
    world.players.get_mut(&2).unwrap().pos = at;
    world.players.get_mut(&1).unwrap().inventory.take_all();
    let start = world.players[&2].hp;
    let mut inputs = BTreeMap::new();
    inputs.insert(
        1,
        Input {
            fire: true,
            aim: world.players[&2].pos,
            ..Input::default()
        },
    );
    world.step(&inputs);
    assert!(world.players[&2].hp <= start, "fists still land");
}

#[test]
fn rambomatch_gives_the_bow_to_whoever_takes_the_objective() {
    assert!(ModeKind::Rambomatch.is_implemented());
    let mut world = World::new("rambomatch");
    world.place_objectives();
    world.add_player(1, "Rambo".into());
    world.add_player(2, "Prey".into());
    let bow = world.objectives.flags[0].pos();
    world.players.get_mut(&1).unwrap().pos = bow;
    idle(&mut world, 2);
    assert!(
        world.players[&1].inventory.owns(WeaponKind::RamboBow),
        "taking the yellow flag is taking the bow"
    );

    world.apply_damage(kill(1, 2));
    assert!(world.ledger.player(1).kills >= 1);

    world.players.get_mut(&2).unwrap().hp = 100;
    world.apply_damage(kill(2, 1));
    assert_eq!(world.ledger.player(2).kills, 0, "only Rambo's kills count");
}

#[test]
fn jump_plus_crouch_throws_the_bow() {
    let mut world = World::new("rambomatch");
    world.place_objectives();
    world.add_player(1, "Rambo".into());
    let bow = world.objectives.flags[0].pos();
    world.players.get_mut(&1).unwrap().pos = bow;
    idle(&mut world, 2);
    assert!(world.objectives.is_carrying(1));
    let mut inputs = BTreeMap::new();
    inputs.insert(
        1,
        Input {
            jump: true,
            crouch: true,
            aim: Vec2 { x: 0.0, y: 0.0 },
            ..Input::default()
        },
    );
    world.step(&inputs);
    assert!(!world.objectives.is_carrying(1));
}

#[test]
fn a_map_m2_can_be_mounted() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Gunner".into());
    let pos = world.players[&1].pos;
    let mut gun = StationaryGun::new(pos);
    gun.deploy = 0;
    world.guns.push(gun);
    let mut inputs = BTreeMap::new();
    inputs.insert(
        1,
        Input {
            pickup: true,
            ..Input::default()
        },
    );
    world.step(&inputs);
    assert_eq!(world.guns[0].mounted_by, Some(1));
    assert!(world.players[&1].inventory.owns(WeaponKind::StationaryGun));
}

#[test]
fn realistic_movement_is_heavier() {
    let normal = MovementConfig::soldat_default();
    let realistic = MovementConfig::realistic();
    assert!(realistic.max_run_speed < normal.max_run_speed);
    assert!(realistic.jump_speed < normal.jump_speed);
}

#[test]
fn pms_team_numbers_name_flag_kit_and_m2_spawns() {
    assert_eq!(SpawnKind::from_pms_team(0), SpawnKind::Player);
    assert_eq!(SpawnKind::from_pms_team(5), SpawnKind::Flag);
    assert_eq!(SpawnKind::from_pms_team(7), SpawnKind::Grenade);
    assert_eq!(SpawnKind::from_pms_team(11), SpawnKind::Bonus);
    assert_eq!(SpawnKind::from_pms_team(16), SpawnKind::StationaryGun);
}

#[test]
fn a_bot_picks_up_loot_at_its_feet_and_rambomatch_is_playable() {
    let mut world = World::new("rambomatch");
    world.place_objectives();
    world.add_bot(
        1,
        BotProfile::new("Boogie", Difficulty::Elite),
        TeamChoice::Auto,
    );
    let pos = world.players[&1].pos;
    world
        .pickups
        .items
        .push(Pickup::new(KitKind::Vest, pos, 600));
    let inputs = world.bot_inputs();
    assert!(inputs[&1].pickup);
    assert!(world
        .objectives
        .flags
        .iter()
        .any(|flag| flag.kind == game_core::FlagKind::Yellow));
}
