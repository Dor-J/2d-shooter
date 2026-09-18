//! Proof that the match lifecycle runs inside `World`, not only as a library of rules.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:match_world:lifecycle — the phases advance as a real world ticks
//!   rust:match_world:ledger    — the scoreboard is a projection of the one ledger
//!   rust:match_world:teams     — joining, switching, balancing, and spectating
//!   rust:match_world:restart   — a restart clears the score and the clock

use game_core::{
    BodyRegion, DamageCause, DamageEvent, MatchEvent, MatchPhase, ModeKind, Outcome, TeamChoice,
    Vec2, World, ALPHA, BRAVO, NEUTRAL, SPECTATOR,
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

/// Puts a dead player back on their feet, the way a respawn would, so a test can stage a second
/// kill without waiting out the respawn timer.
fn revive(world: &mut World, id: u32) {
    if let Some(player) = world.players.get_mut(&id) {
        player.hp = 100;
        player.respawn = 0;
        player.spawn_protection = 0;
    }
}

fn idle(world: &mut World, ticks: u32) -> Vec<MatchEvent> {
    let mut seen = Vec::new();
    for _ in 0..ticks {
        world.step(&BTreeMap::new());
        seen.extend(world.match_events.iter().copied());
    }
    seen
}

#[test]
fn a_new_world_starts_in_the_lobby_and_counts_down_once_somebody_joins() {
    let mut world = World::new("deathmatch");
    assert_eq!(world.match_state.phase, MatchPhase::Lobby);
    assert_eq!(world.rules.kind, ModeKind::Deathmatch);

    world.add_player(1, "Alone".into());
    let events = idle(&mut world, 1);
    assert!(matches!(
        events.first(),
        Some(MatchEvent::CountdownStarted { .. })
    ));
    assert_eq!(world.match_state.phase, MatchPhase::Countdown);

    let countdown = world.rules.countdown_ticks;
    let events = idle(&mut world, countdown + 2);
    assert!(events.contains(&MatchEvent::RoundStarted));
    assert_eq!(world.match_state.phase, MatchPhase::Active);
}

#[test]
fn reaching_the_kill_limit_ends_the_match_and_shows_the_scoreboard() {
    let mut world = World::new("deathmatch");
    world.rules.limits.kills = 2;
    world.rules.limits.time_ticks = 0;
    world.rules.countdown_ticks = 1;
    world.add_player(1, "Winner".into());
    world.add_player(2, "Loser".into());
    idle(&mut world, 4);
    assert_eq!(world.match_state.phase, MatchPhase::Active);

    world.apply_damage(kill(1, 2));
    revive(&mut world, 2);
    world.apply_damage(kill(1, 2));
    assert_eq!(
        world.players[&1].kills, 2,
        "the scoreboard follows the ledger"
    );

    let events = idle(&mut world, 2);
    assert!(
        events.contains(&MatchEvent::RoundEnded {
            outcome: Outcome::Player(1)
        }),
        "{events:?}"
    );
    assert!(
        events
            .iter()
            .any(|event| matches!(event, MatchEvent::ScoreboardShown { .. })),
        "the scoreboard comes up on its own"
    );
    assert_eq!(world.match_state.phase, MatchPhase::RoundEnd);
    assert!(!world.match_state.phase.is_playable());
}

#[test]
fn the_per_player_counters_and_the_team_score_are_projections_of_the_one_ledger() {
    let mut world = World::new("team");
    world.add_player(1, "Alpha one".into());
    world.add_player(2, "Bravo one".into());
    world.add_player(3, "Alpha two".into());
    assert_eq!(world.players[&1].team, ALPHA);
    assert_eq!(world.players[&2].team, BRAVO);
    assert_eq!(world.players[&3].team, ALPHA, "the sides fill evenly");

    world.apply_damage(kill(1, 2));
    assert_eq!(world.ledger.player(1).kills, 1);
    assert_eq!(world.players[&1].kills, 1);
    assert_eq!(world.players[&2].deaths, 1);
    assert_eq!(world.scores[0], 1, "alpha is a point up");

    // A teamkill costs the killer and the side rather than scoring.
    world.apply_damage(kill(1, 3));
    assert_eq!(world.players[&1].kills, 1, "still only the one frag");
    assert_eq!(world.players[&1].teamkills, 1);
    assert_eq!(world.ledger.team(ALPHA), 0, "the point was given back");
    assert_eq!(world.scores[0], 0);
}

#[test]
fn a_reconnecting_player_gets_their_score_back() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Regular".into());
    world.add_player(2, "Target".into());
    world.apply_damage(kill(1, 2));
    revive(&mut world, 2);
    world.apply_damage(kill(1, 2));
    assert_eq!(world.players[&1].kills, 2);

    // They drop out: the player leaves, but the ledger keeps their row.
    world.players.remove(&1);
    world.add_player(1, "Regular".into());
    assert_eq!(
        world.players[&1].kills, 2,
        "the score came back with them, not a fresh zero"
    );
    assert_eq!(world.ledger.player(1).kills, 2);
}

#[test]
fn a_player_may_choose_a_side_but_not_stack_one() {
    let mut world = World::new("team");
    world.add_player_as(1, "One".into(), TeamChoice::Alpha);
    world.add_player_as(2, "Two".into(), TeamChoice::Alpha);
    assert_eq!(world.players[&1].team, ALPHA);
    assert_eq!(
        world.players[&2].team, BRAVO,
        "the second alpha would have stacked it"
    );

    // With one player a side, switching would empty one of them, so the request is refused the
    // same way the join was.
    assert_eq!(world.set_team(2, TeamChoice::Alpha), Some(BRAVO));

    // Moving off an overfull side is what switching is for, and it goes through. Set up three
    // against one, then let one of the three cross over.
    world.add_player_as(3, "Three".into(), TeamChoice::Auto);
    world.add_player_as(4, "Four".into(), TeamChoice::Auto);
    for id in [1u32, 3, 4] {
        world.players.get_mut(&id).unwrap().team = ALPHA;
    }
    world.players.get_mut(&2).unwrap().team = BRAVO;
    assert!(
        world.needs_balancing(),
        "three against one needs evening up"
    );

    world.players.get_mut(&4).unwrap().pos = Vec2 { x: 5.0, y: 5.0 };
    assert_eq!(world.set_team(4, TeamChoice::Bravo), Some(BRAVO));
    assert_eq!(world.players[&4].team, BRAVO);
    assert!(!world.needs_balancing(), "and the sides are even again");
    assert_ne!(
        world.players[&4].pos,
        Vec2 { x: 5.0, y: 5.0 },
        "switching respawns rather than teleporting"
    );
}

#[test]
fn a_spectator_is_honoured_and_does_not_hold_up_the_match() {
    let mut world = World::new("team");
    world.add_player_as(1, "Watcher".into(), TeamChoice::Spectator);
    assert_eq!(world.players[&1].team, SPECTATOR);

    // A lobby of nothing but spectators never starts, because nobody is playing.
    world.match_state.minimum_players = 1;
    idle(&mut world, 3);
    assert_eq!(world.match_state.phase, MatchPhase::Lobby);

    world.add_player_as(2, "Player".into(), TeamChoice::Auto);
    idle(&mut world, 1);
    assert_eq!(world.match_state.phase, MatchPhase::Countdown);
}

#[test]
fn lopsided_sides_are_reported_so_the_server_can_even_them_up() {
    let mut world = World::new("team");
    for id in 1..=4u32 {
        world.add_player_as(id, format!("Player {id}"), TeamChoice::Auto);
    }
    assert!(!world.needs_balancing(), "auto assignment keeps it even");

    // Force three onto one side and the imbalance is reported.
    world.players.get_mut(&2).unwrap().team = ALPHA;
    world.players.get_mut(&4).unwrap().team = ALPHA;
    assert!(world.needs_balancing());

    // A free-for-all has no sides to balance.
    let mut solo = World::new("deathmatch");
    solo.add_player(1, "One".into());
    assert!(!solo.needs_balancing());
    assert_eq!(solo.players[&1].team, NEUTRAL);
}

#[test]
fn restarting_wipes_the_score_and_the_clock_but_keeps_the_players() {
    let mut world = World::new("deathmatch");
    world.rules.countdown_ticks = 1;
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());
    idle(&mut world, 4);
    world.apply_damage(kill(1, 2));
    assert_eq!(world.players[&1].kills, 1);
    assert!(world.match_state.elapsed > 0);

    world.restart_match();
    assert_eq!(world.match_state.phase, MatchPhase::Countdown);
    assert_eq!(world.match_state.elapsed, 0);
    assert_eq!(world.match_state.outcome, None);
    assert_eq!(world.players[&1].kills, 0, "the score is wiped");
    assert_eq!(world.players.len(), 2, "but the players are still here");
    assert_eq!(world.scores, [0, 0]);
}

#[test]
fn the_time_limit_ends_a_world_that_nobody_is_scoring_in() {
    let mut world = World::new("deathmatch");
    world.rules.countdown_ticks = 1;
    world.rules.limits.kills = 0;
    world.rules.limits.time_ticks = 20;
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());

    let events = idle(&mut world, 60);
    assert!(
        events.iter().any(|event| matches!(
            event,
            MatchEvent::RoundEnded {
                outcome: Outcome::Draw
            }
        )),
        "nobody scored, so it is a draw: {events:?}"
    );
}

#[test]
fn a_finished_round_asks_for_the_next_map() {
    let mut world = World::new("deathmatch");
    world.rules.countdown_ticks = 1;
    world.rules.round_end_ticks = 2;
    world.rules.limits.kills = 1;
    world.rules.limits.time_ticks = 0;
    world.add_player(1, "One".into());
    world.add_player(2, "Two".into());
    idle(&mut world, 4);
    world.apply_damage(kill(1, 2));

    let events = idle(&mut world, 10);
    assert!(
        events.contains(&MatchEvent::MapTransitionStarted),
        "{events:?}"
    );
    assert!(events.contains(&MatchEvent::NextMap));
}
