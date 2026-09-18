//! Proof that spectating and statistics run inside `World`, not only as a library of rules.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:spectator_world:follow — a spectator is given a camera and keeps one as players come and go
//!   rust:statistics_world:feed  — shots, hits, kills, and objectives reach the ledger from real play

use game_core::{
    BodyRegion, DamageCause, DamageEvent, Input, ModeKind, ModeRules, SpectateCommand, TeamChoice,
    Vec2, WeaponKind, World, SPECTATOR,
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
        pre_scaled: false,
    }
}

#[test]
fn a_player_who_joins_as_a_spectator_is_given_somebody_to_watch() {
    let mut world = World::new("team");
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);
    world.add_player_as(9, "Watcher".into(), TeamChoice::Spectator);
    assert_eq!(world.players[&9].team, SPECTATOR);

    idle(&mut world, 1);
    let watching = world.spectators.get(&9).expect("they have a camera");
    assert!(
        watching.target.is_some_and(|id| id == 1 || id == 2),
        "pointed at somebody who is playing: {:?}",
        watching.target
    );
}

#[test]
fn the_followable_list_is_the_players_and_never_the_watchers() {
    let mut world = World::new("team");
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);
    world.add_player_as(9, "Watcher".into(), TeamChoice::Spectator);

    assert_eq!(world.followable(), vec![1, 2]);
}

#[test]
fn a_spectator_can_cycle_through_the_players_and_take_a_free_camera() {
    let mut world = World::new("team");
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);
    world.add_player_as(9, "Watcher".into(), TeamChoice::Spectator);
    idle(&mut world, 1);

    world.spectate(9, SpectateCommand::Follow(2));
    assert_eq!(world.spectators[&9].target, Some(2));

    world.spectate(9, SpectateCommand::Next);
    assert_eq!(world.spectators[&9].target, Some(1), "it wrapped round");

    world.spectate(9, SpectateCommand::FreeCamera);
    assert!(world.spectators[&9].is_free());
    assert_eq!(world.spectators[&9].target, None);
}

#[test]
fn a_player_who_is_not_spectating_cannot_drive_a_spectator_camera() {
    let mut world = World::new("team");
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);
    idle(&mut world, 1);

    world.spectate(1, SpectateCommand::Follow(2));
    assert!(
        !world.spectators.contains_key(&1),
        "a playing player has no spectator camera to point"
    );
}

#[test]
fn a_spectator_whose_target_leaves_is_handed_somebody_else() {
    let mut world = World::new("team");
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);
    world.add_player_as(9, "Watcher".into(), TeamChoice::Spectator);
    idle(&mut world, 1);

    world.spectate(9, SpectateCommand::Follow(2));
    assert_eq!(world.spectators[&9].target, Some(2));

    world.players.remove(&2);
    idle(&mut world, 1);
    assert_eq!(
        world.spectators[&9].target,
        Some(1),
        "the camera moved on rather than being stranded"
    );
}

#[test]
fn a_spectator_who_starts_playing_gives_up_their_camera() {
    let mut world = World::new("team");
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(9, "Watcher".into(), TeamChoice::Spectator);
    idle(&mut world, 1);
    assert!(world.spectators.contains_key(&9));

    world.set_team(9, TeamChoice::Bravo);
    idle(&mut world, 1);
    assert!(
        !world.spectators.contains_key(&9),
        "they are playing now, not watching"
    );
}

#[test]
fn every_shot_fired_in_a_real_match_is_counted() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Shooter".into());
    let aim = Vec2 { x: 900.0, y: 430.0 };

    let firing = BTreeMap::from([(
        1,
        Input {
            fire: true,
            aim,
            ..Input::default()
        },
    )]);
    for _ in 0..120 {
        world.step(&firing);
    }

    let stats = world.stats.player(1);
    assert!(stats.shots > 0, "the trigger was pulled");
    let weapon = WeaponKind::from_slot(world.players[&1].weapon);
    assert_eq!(
        stats.weapons[&weapon].shots, stats.shots,
        "and every shot is attributed to the weapon that fired it"
    );
}

#[test]
fn a_hit_and_the_kill_it_caused_are_both_recorded_against_the_weapon() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Shooter".into());
    world.add_player(2, "Target".into());
    let weapon = WeaponKind::from_slot(world.players[&1].weapon);

    world.apply_damage(kill(1, 2));

    let stats = world.stats.player(1);
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.weapons[&weapon].hits, 1);
    assert_eq!(stats.weapons[&weapon].kills, 1);
    assert_eq!(
        world.stats.player(2).weapons[&weapon].deaths,
        1,
        "and the victim's death is recorded against it too"
    );
}

#[test]
fn a_headshot_is_counted_as_one() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Shooter".into());
    world.add_player(2, "Target".into());

    world.apply_damage(DamageEvent {
        region: BodyRegion::Head,
        ..kill(1, 2)
    });
    assert_eq!(world.stats.player(1).headshots, 1);

    // A chest shot is not.
    world.players.get_mut(&2).unwrap().hp = 100;
    world.players.get_mut(&2).unwrap().respawn = 0;
    world.apply_damage(kill(1, 2));
    assert_eq!(world.stats.player(1).headshots, 1);
    assert_eq!(world.stats.player(1).hits, 2);
}

#[test]
fn a_death_with_no_weapon_behind_it_is_recorded_by_cause_and_credited_to_nobody() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Faller".into());

    world.apply_damage(DamageEvent {
        attacker: None,
        target: 1,
        amount: 500,
        region: BodyRegion::Legs,
        cause: DamageCause::Fall,
        direction: Vec2 { x: 0.0, y: 1.0 },
        pre_scaled: false,
    });

    let stats = world.stats.player(1);
    assert_eq!(stats.deaths_by_cause[&DamageCause::Fall], 1);
    assert!(
        stats.weapons.is_empty(),
        "a fall is nobody's weapon: {:?}",
        stats.weapons
    );
}

#[test]
fn a_capture_is_recorded_as_objective_work() {
    let mut world = World::new(ModeKind::CaptureTheFlag.id());
    world.rules = ModeRules::new(ModeKind::CaptureTheFlag);
    world.place_objectives();
    world.add_player_as(1, "Alpha".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Bravo".into(), TeamChoice::Bravo);

    let alpha_base = world
        .objectives
        .flag(game_core::FlagKind::Alpha)
        .unwrap()
        .base;
    let bravo_base = world
        .objectives
        .flag(game_core::FlagKind::Bravo)
        .unwrap()
        .base;

    world.players.get_mut(&1).unwrap().pos = bravo_base;
    idle(&mut world, 1);
    world.players.get_mut(&1).unwrap().pos = alpha_base;
    idle(&mut world, 1);

    assert_eq!(world.stats.player(1).captures, 1);
    assert_eq!(world.ledger.player(1).objectives, 1);
}

#[test]
fn a_scoreboard_built_from_a_real_match_ranks_everybody_and_carries_their_statistics() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Leader".into());
    world.add_player(2, "Second".into());
    world.add_player(3, "Target".into());

    for _ in 0..3 {
        world.players.get_mut(&3).unwrap().hp = 100;
        world.players.get_mut(&3).unwrap().respawn = 0;
        world.apply_damage(kill(1, 3));
    }
    world.players.get_mut(&3).unwrap().hp = 100;
    world.players.get_mut(&3).unwrap().respawn = 0;
    world.apply_damage(kill(2, 3));

    let teams: std::collections::BTreeMap<u32, u8> = world
        .players
        .iter()
        .map(|(id, player)| (*id, player.team))
        .collect();
    let rows = game_core::scoreboard(&world.ledger, &world.stats, |id| {
        teams.get(&id).copied().unwrap_or(0)
    });

    assert_eq!(rows[0].player, 1, "most points first");
    assert_eq!(rows[0].rank, 1);
    assert_eq!(rows[0].behind_leader, 0);
    assert_eq!(rows[1].player, 2);
    assert_eq!(rows[1].behind_leader, 2);
    assert_eq!(rows[0].stats.hits, 3, "the statistics came along");
}

#[test]
fn restarting_a_match_wipes_the_statistics_as_well_as_the_score() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());
    world.apply_damage(kill(1, 2));
    assert_eq!(world.stats.player(1).hits, 1);

    world.restart_match();
    assert_eq!(world.stats.player(1).hits, 0);
    assert_eq!(world.ledger.player(1).kills, 0);
}
