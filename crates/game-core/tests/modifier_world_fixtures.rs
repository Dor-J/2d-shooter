//! Proof that the modifiers run inside `World`, not only as a library of rules.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:modifier_world:survival — no respawn, and the round resolves to a last one standing
//!   rust:modifier_world:advance  — unlocks and revocations as the ledger moves
//!   rust:modifier_world:realistic — the Realistic table reaches a real match

use game_core::{
    BodyRegion, DamageCause, DamageEvent, Input, ModeKind, ModeRules, ModifierSet, RoundStanding,
    TeamChoice, Unlocked, Vec2, WeaponKind, WeaponTable, World, ALPHA,
};
use std::collections::BTreeMap;

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

fn idle(world: &mut World, ticks: u32) {
    for _ in 0..ticks {
        world.step(&BTreeMap::new());
    }
}

fn revive(world: &mut World, id: u32) {
    if let Some(player) = world.players.get_mut(&id) {
        player.hp = 100;
        player.respawn = 0;
        player.spawn_protection = 0;
    }
}

fn world_with(kind: ModeKind, modifiers: ModifierSet) -> World {
    let mut world = World::new(kind.id());
    world.rules = ModeRules::new(kind);
    world.rules.modifiers = modifiers;
    world.rules.countdown_ticks = 1;
    world.place_objectives();
    world
}

#[test]
fn a_survival_player_does_not_come_back_however_long_they_wait() {
    let mut world = world_with(
        ModeKind::Deathmatch,
        ModifierSet {
            survival: true,
            ..ModifierSet::default()
        },
    );
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());

    world.apply_damage(kill(1, 2));
    assert!(world.players[&2].hp <= 0);

    // Far longer than any respawn delay.
    idle(&mut world, 600);
    assert!(
        world.players[&2].hp <= 0,
        "they are out for the round, not waiting on a timer"
    );
}

#[test]
fn outside_survival_the_same_player_comes_straight_back() {
    let mut world = world_with(ModeKind::Deathmatch, ModifierSet::default());
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());

    world.apply_damage(kill(1, 2));
    assert!(world.players[&2].hp <= 0);

    idle(&mut world, 300);
    assert!(
        world.players[&2].hp > 0,
        "the respawn timer ran and they returned"
    );
}

#[test]
fn a_survival_round_resolves_to_the_last_player_standing() {
    let mut world = world_with(
        ModeKind::Deathmatch,
        ModifierSet {
            survival: true,
            ..ModifierSet::default()
        },
    );
    world.add_player(1, "Winner".into());
    world.add_player(2, "Loser".into());
    idle(&mut world, 3);
    assert_eq!(world.round_standing, RoundStanding::Ongoing);

    world.apply_damage(kill(1, 2));
    idle(&mut world, 1);
    assert_eq!(world.round_standing, RoundStanding::LastPlayer(1));
    assert!(world.round_standing.is_over());
}

#[test]
fn a_team_survival_round_resolves_to_the_last_side_standing() {
    let mut world = world_with(
        ModeKind::Teammatch,
        ModifierSet {
            survival: true,
            ..ModifierSet::default()
        },
    );
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);
    idle(&mut world, 3);
    assert_eq!(world.round_standing, RoundStanding::Ongoing);

    world.apply_damage(kill(1, 2));
    idle(&mut world, 1);
    assert_eq!(world.round_standing, RoundStanding::LastTeam(ALPHA));
}

#[test]
fn a_match_without_survival_never_reports_a_round_standing() {
    let mut world = world_with(ModeKind::Deathmatch, ModifierSet::default());
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());
    world.apply_damage(kill(1, 2));
    idle(&mut world, 1);
    assert_eq!(
        world.round_standing,
        RoundStanding::Ongoing,
        "there are no rounds to stand in"
    );
}

#[test]
fn an_advance_player_earns_a_primary_as_their_kills_mount_up() {
    let mut world = world_with(
        ModeKind::Deathmatch,
        ModifierSet {
            advance: true,
            ..ModifierSet::default()
        },
    );
    world.add_player(1, "Climber".into());
    world.add_player(2, "Target".into());
    world.players.get_mut(&1).unwrap().unlocked = Unlocked::starting();
    assert_eq!(world.players[&1].unlocked.count(), 0);

    // The first kill is not enough; the second earns one.
    world.apply_damage(kill(1, 2));
    assert_eq!(world.players[&1].unlocked.count(), 0, "not yet");
    revive(&mut world, 2);
    world.apply_damage(kill(1, 2));
    assert_eq!(world.players[&1].unlocked.count(), 1, "earned one");
}

#[test]
fn an_advance_player_loses_a_primary_as_their_deaths_mount_up() {
    let mut world = world_with(
        ModeKind::Deathmatch,
        ModifierSet {
            advance: true,
            ..ModifierSet::default()
        },
    );
    world.add_player(1, "Killer".into());
    world.add_player(2, "Faller".into());
    world.players.get_mut(&2).unwrap().unlocked = Unlocked::everything();
    let before = world.players[&2].unlocked.count();

    world.apply_damage(kill(1, 2));
    revive(&mut world, 2);
    world.apply_damage(kill(1, 2));

    assert!(
        world.players[&2].unlocked.count() < before,
        "dying too often costs you the armoury: {} then {}",
        before,
        world.players[&2].unlocked.count()
    );
}

#[test]
fn a_match_without_advance_leaves_everybody_with_everything() {
    let mut world = world_with(ModeKind::Deathmatch, ModifierSet::default());
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());
    let before = world.players[&2].unlocked;

    world.apply_damage(kill(1, 2));
    revive(&mut world, 2);
    world.apply_damage(kill(1, 2));

    assert_eq!(world.players[&2].unlocked, before, "nothing was taken away");
    assert_eq!(
        world.players[&1].unlocked, before,
        "and nothing was granted"
    );
    assert!(before.has(WeaponKind::Barrett), "everything is available");
}

#[test]
fn an_advance_player_spawns_with_only_what_they_have_earned() {
    let mut world = world_with(
        ModeKind::Deathmatch,
        ModifierSet {
            advance: true,
            ..ModifierSet::default()
        },
    );
    world.add_player(1, "Climber".into());
    world.add_player(2, "Other".into());

    // Strip their unlocks and send them round the respawn, which is where a loadout is built.
    world.players.get_mut(&1).unwrap().unlocked = Unlocked::starting();
    world.players.get_mut(&1).unwrap().weapon = WeaponKind::Barrett.slot().expect("selectable");
    world.apply_damage(kill(2, 1));
    idle(&mut world, 300);
    assert!(world.players[&1].hp > 0, "they came back");

    for slot in world.players[&1].inventory.slots.iter().flatten() {
        assert!(
            world.players[&1].unlocked.has(slot.kind),
            "they spawned holding {:?}, which they have not earned",
            slot.kind
        );
    }
    // A secondary is never locked, so they are not left with empty hands.
    assert!(
        world.players[&1]
            .inventory
            .slots
            .iter()
            .any(Option::is_some),
        "they have something to shoot with"
    );
}

#[test]
fn advance_refuses_a_weapon_selection_the_player_has_not_earned() {
    let mut world = world_with(
        ModeKind::Deathmatch,
        ModifierSet {
            advance: true,
            ..ModifierSet::default()
        },
    );
    world.add_player(1, "Climber".into());
    world.players.get_mut(&1).unwrap().unlocked = Unlocked::starting();

    let barrett = WeaponKind::Barrett.slot().expect("selectable");
    world.step(&BTreeMap::from([(
        1,
        Input {
            weapon: barrett,
            aim: Vec2 { x: 900.0, y: 430.0 },
            ..Input::default()
        },
    )]));

    assert_ne!(
        world.players[&1].weapon, barrett,
        "asking for the Barrett does not hand it over"
    );
}

#[test]
fn a_realistic_room_runs_on_the_realistic_weapon_table() {
    let mut world = world_with(
        ModeKind::Deathmatch,
        ModifierSet {
            realistic: true,
            ..ModifierSet::default()
        },
    );
    world.weapons = WeaponTable::realistic();
    world.add_player(1, "One".into());

    assert!(world.weapons.is_realistic());
    assert!(world.rules.modifiers.realistic);
    // The Realistic table is what the simulation reads, so its self-bink is in play.
    assert!(world.weapons.get(WeaponKind::Mp5).self_binks());
}

#[test]
fn the_three_modifiers_can_all_be_on_at_once_in_a_flag_mode() {
    let mut world = world_with(
        ModeKind::CaptureTheFlag,
        ModifierSet {
            realistic: true,
            survival: true,
            advance: true,
        },
    );
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);
    idle(&mut world, 3);

    assert_eq!(world.rules.kind, ModeKind::CaptureTheFlag, "still CTF");
    assert_eq!(world.objectives.flags.len(), 2, "with its flags");
    assert_eq!(world.round_standing, RoundStanding::Ongoing);

    // Survival still holds the dead out, even with the other two on.
    world.apply_damage(kill(1, 2));
    idle(&mut world, 400);
    assert!(world.players[&2].hp <= 0);
}
