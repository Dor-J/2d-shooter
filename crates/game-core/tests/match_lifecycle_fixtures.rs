//! Parity fixtures for the shared match lifecycle: limits, phases, teams, scoring, and rotation.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:match_lifecycle:limits     — kill, point, capture, and time limits
//!   rust:match_lifecycle:phases     — countdown, round start, overtime, round end, transition
//!   rust:match_lifecycle:outcome    — winner and draw resolution
//!   rust:match_lifecycle:rotation   — rotation, loop, next, restart
//!   rust:match_lifecycle:teams      — balancing, selection, spectators
//!   rust:match_lifecycle:scoring    — friendly fire, teamkills, suicides, reconnect preservation
//!
//! Source: the Soldat game-mode and server-settings documentation listed in
//! `docs/gaps/gap-list.md`, against the revision pinned in `docs/parity/reference-lock.md`.

use game_core::{
    assign_team, leader, limit_reached, validate_spawns, MapSpawn, MatchEvent, MatchLimits,
    MatchPhase, MatchState, ModeKind, ModeRules, ModifierSet, Outcome, Rotation, ScoreEvent,
    ScoreLedger, SpawnProblem, TeamChoice, TeamSizes, Vec2, ALPHA, BRAVO, NEUTRAL, SPECTATOR,
};

fn deathmatch() -> ModeRules {
    ModeRules::new(ModeKind::Deathmatch)
}

fn teammatch() -> ModeRules {
    ModeRules::new(ModeKind::Teammatch)
}

/// Runs the lifecycle until it reports something, or gives up.
fn run_until(
    state: &mut MatchState,
    rules: &ModeRules,
    ledger: &ScoreLedger,
    players: usize,
    ticks: u32,
) -> Option<MatchEvent> {
    for _ in 0..ticks {
        if let Some(event) = state.step(rules, ledger, players) {
            return Some(event);
        }
    }
    None
}

#[test]
fn a_match_waits_in_the_lobby_until_somebody_is_there_to_play() {
    let rules = deathmatch();
    let ledger = ScoreLedger::default();
    let mut state = MatchState {
        minimum_players: 2,
        ..MatchState::default()
    };

    assert_eq!(state.step(&rules, &ledger, 0), None);
    assert_eq!(state.phase, MatchPhase::Lobby);
    assert_eq!(state.step(&rules, &ledger, 1), None, "one is not enough");

    assert_eq!(
        state.step(&rules, &ledger, 2),
        Some(MatchEvent::CountdownStarted {
            ticks: rules.countdown_ticks
        })
    );
    assert_eq!(state.phase, MatchPhase::Countdown);

    // If everybody leaves mid-countdown it falls back rather than starting an empty match.
    assert_eq!(state.step(&rules, &ledger, 0), None);
    assert_eq!(state.phase, MatchPhase::Lobby);
}

#[test]
fn the_countdown_runs_down_and_then_the_round_starts() {
    let rules = deathmatch();
    let ledger = ScoreLedger::default();
    let mut state = MatchState::default();

    state.step(&rules, &ledger, 1);
    assert_eq!(state.phase, MatchPhase::Countdown);
    assert!(
        !state.phase.is_playable(),
        "nobody shoots during a countdown"
    );

    let started = run_until(&mut state, &rules, &ledger, 1, rules.countdown_ticks + 2);
    assert_eq!(started, Some(MatchEvent::RoundStarted));
    assert_eq!(state.phase, MatchPhase::Active);
    assert!(state.phase.is_playable());
    assert_eq!(state.elapsed, 0, "the clock starts when the round does");
}

#[test]
fn the_kill_limit_ends_the_match_and_names_the_winner() {
    let mut rules = deathmatch();
    rules.limits = MatchLimits {
        kills: 3,
        time_ticks: 0,
        ..MatchLimits::default()
    };
    let mut ledger = ScoreLedger::default();
    let mut state = MatchState::default();
    state.step(&rules, &ledger, 2);
    run_until(&mut state, &rules, &ledger, 2, rules.countdown_ticks + 2);
    assert_eq!(state.phase, MatchPhase::Active);

    for _ in 0..2 {
        ledger.record(
            ScoreEvent::Kill {
                killer: 1,
                victim: 2,
            },
            &rules,
            |_| NEUTRAL,
        );
    }
    assert!(
        !limit_reached(&ledger, &rules),
        "two of three is not enough"
    );
    assert_eq!(state.step(&rules, &ledger, 2), None);

    ledger.record(
        ScoreEvent::Kill {
            killer: 1,
            victim: 2,
        },
        &rules,
        |_| NEUTRAL,
    );
    assert!(limit_reached(&ledger, &rules));
    assert_eq!(
        state.step(&rules, &ledger, 2),
        Some(MatchEvent::RoundEnded {
            outcome: Outcome::Player(1)
        })
    );
    assert_eq!(state.phase, MatchPhase::RoundEnd);
}

#[test]
fn the_time_limit_ends_the_match_even_with_nobody_near_the_score_limit() {
    let mut rules = deathmatch();
    rules.limits = MatchLimits {
        kills: 50,
        time_ticks: 30,
        ..MatchLimits::default()
    };
    rules.countdown_ticks = 1;
    let mut ledger = ScoreLedger::default();
    ledger.record(
        ScoreEvent::Kill {
            killer: 1,
            victim: 2,
        },
        &rules,
        |_| NEUTRAL,
    );

    let mut state = MatchState::default();
    state.step(&rules, &ledger, 2);
    run_until(&mut state, &rules, &ledger, 2, 4);
    assert_eq!(state.phase, MatchPhase::Active);

    let ended = run_until(&mut state, &rules, &ledger, 2, 60);
    assert_eq!(
        ended,
        Some(MatchEvent::RoundEnded {
            outcome: Outcome::Player(1)
        }),
        "the clock decided it"
    );
}

#[test]
fn the_point_and_capture_limits_end_their_own_modes() {
    let mut pointmatch = ModeRules::new(ModeKind::Pointmatch);
    pointmatch.limits = MatchLimits {
        points: 5,
        kills: 0,
        captures: 0,
        time_ticks: 0,
    };
    let mut ledger = ScoreLedger::default();
    ledger.record(
        ScoreEvent::Objective {
            player: 1,
            team: NEUTRAL,
            points: 4,
        },
        &pointmatch,
        |_| NEUTRAL,
    );
    assert!(!limit_reached(&ledger, &pointmatch));
    ledger.record(
        ScoreEvent::Objective {
            player: 1,
            team: NEUTRAL,
            points: 1,
        },
        &pointmatch,
        |_| NEUTRAL,
    );
    assert!(limit_reached(&ledger, &pointmatch));

    let mut ctf = ModeRules::new(ModeKind::CaptureTheFlag);
    ctf.limits = MatchLimits {
        captures: 2,
        kills: 0,
        points: 0,
        time_ticks: 0,
    };
    let mut flags = ScoreLedger::default();
    flags.record(
        ScoreEvent::Objective {
            player: 1,
            team: ALPHA,
            points: 1,
        },
        &ctf,
        |_| ALPHA,
    );
    assert!(!limit_reached(&flags, &ctf));
    flags.record(
        ScoreEvent::Objective {
            player: 1,
            team: ALPHA,
            points: 1,
        },
        &ctf,
        |_| ALPHA,
    );
    assert!(limit_reached(&flags, &ctf));
    assert_eq!(leader(&flags, &ctf), Outcome::Team(ALPHA));
}

#[test]
fn a_level_match_goes_to_overtime_and_overtime_ends_the_moment_somebody_goes_ahead() {
    let mut rules = ModeRules::new(ModeKind::CaptureTheFlag);
    rules.limits = MatchLimits {
        captures: 0,
        kills: 0,
        points: 0,
        time_ticks: 5,
    };
    rules.countdown_ticks = 1;
    rules.overtime = true;
    rules.overtime_ticks = 600;

    let mut ledger = ScoreLedger::default();
    let mut state = MatchState::default();
    state.step(&rules, &ledger, 2);
    run_until(&mut state, &rules, &ledger, 2, 4);
    assert_eq!(state.phase, MatchPhase::Active);

    let event = run_until(&mut state, &rules, &ledger, 2, 20);
    assert_eq!(
        event,
        Some(MatchEvent::OvertimeStarted { ticks: 600 }),
        "nobody was ahead, so they play on"
    );
    assert_eq!(state.phase, MatchPhase::Overtime);
    assert!(state.phase.is_playable());

    // Play a few ticks of overtime, still level.
    assert_eq!(state.step(&rules, &ledger, 2), None);

    ledger.record(
        ScoreEvent::Objective {
            player: 1,
            team: BRAVO,
            points: 1,
        },
        &rules,
        |_| BRAVO,
    );
    assert_eq!(
        state.step(&rules, &ledger, 2),
        Some(MatchEvent::RoundEnded {
            outcome: Outcome::Team(BRAVO)
        })
    );
}

#[test]
fn overtime_that_settles_nothing_ends_as_a_draw() {
    let mut rules = teammatch();
    rules.limits = MatchLimits {
        kills: 0,
        points: 0,
        captures: 0,
        time_ticks: 2,
    };
    rules.countdown_ticks = 1;
    rules.overtime = true;
    rules.overtime_ticks = 3;

    let ledger = ScoreLedger::default();
    let mut state = MatchState::default();
    state.step(&rules, &ledger, 2);
    run_until(&mut state, &rules, &ledger, 2, 4);
    run_until(&mut state, &rules, &ledger, 2, 10);
    assert_eq!(state.phase, MatchPhase::Overtime);

    let ended = run_until(&mut state, &rules, &ledger, 2, 10);
    assert_eq!(
        ended,
        Some(MatchEvent::RoundEnded {
            outcome: Outcome::Draw
        })
    );
    assert_eq!(state.outcome, Some(Outcome::Draw));
}

#[test]
fn a_match_without_overtime_simply_stands_as_a_draw() {
    let mut rules = deathmatch();
    rules.overtime = false;
    rules.countdown_ticks = 1;
    rules.limits = MatchLimits {
        kills: 0,
        points: 0,
        captures: 0,
        time_ticks: 2,
    };
    let ledger = ScoreLedger::default();
    let mut state = MatchState::default();
    state.step(&rules, &ledger, 2);
    run_until(&mut state, &rules, &ledger, 2, 4);

    let ended = run_until(&mut state, &rules, &ledger, 2, 10);
    assert_eq!(
        ended,
        Some(MatchEvent::RoundEnded {
            outcome: Outcome::Draw
        })
    );
}

#[test]
fn a_finished_round_shows_the_scoreboard_and_then_moves_to_the_next_map() {
    let mut rules = deathmatch();
    rules.round_end_ticks = 3;
    let ledger = ScoreLedger::default();
    let mut state = MatchState {
        phase: MatchPhase::RoundEnd,
        phase_ticks: rules.round_end_ticks,
        final_screenshot: true,
        ..MatchState::default()
    };

    assert_eq!(
        state.scoreboard_event(),
        MatchEvent::ScoreboardShown { screenshot: true },
        "the scoreboard and the optional screenshot are automatic"
    );
    assert!(state.phase.is_finished());
    assert!(!state.phase.is_playable());

    let moved = run_until(&mut state, &rules, &ledger, 2, 10);
    assert_eq!(moved, Some(MatchEvent::MapTransitionStarted));
    assert_eq!(
        state.step(&rules, &ledger, 2),
        Some(MatchEvent::NextMap),
        "and the next map is asked for"
    );
}

#[test]
fn restarting_puts_the_match_back_on_the_countdown_with_a_clean_clock() {
    let rules = deathmatch();
    let mut state = MatchState {
        phase: MatchPhase::Active,
        elapsed: 5_000,
        outcome: Some(Outcome::Player(9)),
        ..MatchState::default()
    };

    assert_eq!(
        state.restart(&rules),
        MatchEvent::CountdownStarted {
            ticks: rules.countdown_ticks
        }
    );
    assert_eq!(state.phase, MatchPhase::Countdown);
    assert_eq!(state.elapsed, 0);
    assert_eq!(state.outcome, None);
}

#[test]
fn a_rotation_advances_loops_and_refuses_to_be_empty() {
    assert!(Rotation::new(Vec::new(), true).is_none());
    assert!(Rotation::new(vec!["  ".into(), String::new()], true).is_none());

    let mut looping = Rotation::new(vec!["Arena".into(), "Bunker".into()], true).unwrap();
    assert_eq!(looping.current(), "Arena");
    assert_eq!(looping.next_map(), "Bunker");
    assert_eq!(looping.next_map(), "Arena", "it starts over");
    assert!(!looping.is_exhausted());

    let mut once = Rotation::new(vec!["Arena".into(), "Bunker".into()], false).unwrap();
    assert_eq!(once.next_map(), "Bunker");
    assert!(once.is_exhausted());
    assert_eq!(once.next_map(), "Bunker", "the last map repeats");

    // A named map can be selected directly, and an unknown one leaves the rotation alone.
    assert!(once.select("Arena"));
    assert_eq!(once.current(), "Arena");
    assert!(!once.select("Nowhere"));
    assert_eq!(once.current(), "Arena");
}

#[test]
fn teams_are_assigned_to_the_smaller_side_and_a_stacking_choice_is_overridden() {
    let rules = teammatch();

    // An even match honours the choice.
    assert_eq!(
        assign_team(&rules, TeamChoice::Bravo, TeamSizes { alpha: 2, bravo: 2 }),
        BRAVO
    );
    // Choosing the side that is already ahead by one would stack it, so it is refused.
    assert_eq!(
        assign_team(&rules, TeamChoice::Alpha, TeamSizes { alpha: 3, bravo: 2 }),
        BRAVO
    );
    // Auto always fills the gap, and ties go to alpha so assignment is deterministic.
    assert_eq!(
        assign_team(&rules, TeamChoice::Auto, TeamSizes { alpha: 1, bravo: 3 }),
        ALPHA
    );
    assert_eq!(
        assign_team(&rules, TeamChoice::Auto, TeamSizes { alpha: 2, bravo: 2 }),
        ALPHA
    );

    // A free-for-all has no sides at all, whatever was asked for.
    assert_eq!(
        assign_team(&deathmatch(), TeamChoice::Alpha, TeamSizes::default()),
        NEUTRAL
    );
    // A spectator is honoured in every mode.
    assert_eq!(
        assign_team(&deathmatch(), TeamChoice::Spectator, TeamSizes::default()),
        SPECTATOR
    );
    assert_eq!(
        assign_team(
            &rules,
            TeamChoice::Spectator,
            TeamSizes { alpha: 0, bravo: 5 }
        ),
        SPECTATOR
    );
}

#[test]
fn balancing_names_the_overfull_side_only_when_it_is_really_lopsided() {
    assert_eq!(
        game_core::modes::team::overfull_side(TeamSizes { alpha: 3, bravo: 2 }),
        None,
        "one apart is a normal match"
    );
    assert_eq!(
        game_core::modes::team::overfull_side(TeamSizes { alpha: 4, bravo: 2 }),
        Some(ALPHA)
    );
    assert_eq!(
        game_core::modes::team::overfull_side(TeamSizes { alpha: 1, bravo: 4 }),
        Some(BRAVO)
    );
    assert!(game_core::modes::team::is_spectator(SPECTATOR));
    assert!(!game_core::modes::team::is_spectator(ALPHA));

    // Counting ignores spectators, so somebody watching never pulls the teams out of balance.
    let sizes = TeamSizes::count([ALPHA, ALPHA, BRAVO, SPECTATOR, NEUTRAL].into_iter());
    assert_eq!(sizes, TeamSizes { alpha: 2, bravo: 1 });
}

#[test]
fn a_teamkill_and_a_suicide_both_cost_the_player_and_a_teamkill_costs_the_team() {
    let rules = teammatch();
    let mut ledger = ScoreLedger::default();
    // Everybody in this fixture is on alpha, which is what makes the first kill a teamkill.
    let teams = |_: u32| ALPHA;

    ledger.record(
        ScoreEvent::TeamKill {
            killer: 1,
            victim: 2,
        },
        &rules,
        teams,
    );
    let killer = ledger.player(1);
    assert_eq!(killer.teamkills, 1);
    assert_eq!(killer.points, -1);
    assert_eq!(ledger.player(2).deaths, 1, "the victim still died");
    assert_eq!(ledger.team(ALPHA), -1, "and it cost the side");

    ledger.record(ScoreEvent::Suicide { player: 1 }, &rules, teams);
    let after = ledger.player(1);
    assert_eq!(after.suicides, 1);
    assert_eq!(after.deaths, 1, "a suicide is a death");
    assert_eq!(after.points, -2);

    // A kill is worth a point to the player and to their side.
    ledger.record(
        ScoreEvent::Kill {
            killer: 1,
            victim: 3,
        },
        &rules,
        |id| if id == 3 { BRAVO } else { ALPHA },
    );
    assert_eq!(ledger.player(1).kills, 1);
    assert_eq!(ledger.team(ALPHA), -1 + -1 + 1);
}

#[test]
fn a_free_for_all_never_books_a_teamkill_and_keeps_no_team_score() {
    let rules = deathmatch();
    assert!(!rules.is_team_mode());
    assert_eq!(rules.kill_value(1, 2, true), rules.scoring.kill);
    assert_eq!(rules.kill_value(1, 1, false), rules.scoring.suicide);

    let mut ledger = ScoreLedger::default();
    ledger.record(
        ScoreEvent::Kill {
            killer: 1,
            victim: 2,
        },
        &rules,
        |_| NEUTRAL,
    );
    assert_eq!(ledger.teams(), [0, 0, 0], "a free-for-all has no sides");
    assert_eq!(ledger.player(1).points, 1);
}

#[test]
fn a_team_mode_books_a_teamkill_rather_than_a_kill() {
    let rules = teammatch();
    assert_eq!(rules.kill_value(1, 2, true), rules.scoring.teamkill);
    assert_eq!(rules.kill_value(1, 2, false), rules.scoring.kill);
    assert!(rules.scoring.teamkill < 0, "a teamkill costs");
}

#[test]
fn a_reconnecting_player_keeps_the_score_they_left_with() {
    let rules = deathmatch();
    let mut ledger = ScoreLedger::default();
    for _ in 0..4 {
        ledger.record(
            ScoreEvent::Kill {
                killer: 7,
                victim: 8,
            },
            &rules,
            |_| NEUTRAL,
        );
    }
    let before = ledger.player(7);
    assert_eq!(before.kills, 4);

    // They drop out and come back: ensuring a row must not wipe one that already exists.
    ledger.ensure(7);
    assert_eq!(ledger.player(7), before, "the score came back with them");

    // A player who never scored still gets an empty row rather than being absent.
    ledger.ensure(11);
    assert_eq!(ledger.player(11).kills, 0);

    // Only an explicit removal clears it.
    ledger.forget(7);
    assert_eq!(ledger.player(7).kills, 0);
}

#[test]
fn friendly_fire_is_a_setting_and_defaults_to_off() {
    let mut rules = teammatch();
    assert!(!rules.friendly_fire, "teammates are safe by default");
    rules.friendly_fire = true;
    assert!(rules.friendly_fire);
}

#[test]
fn survival_defers_a_respawn_instead_of_shortening_it() {
    let mut rules = deathmatch();
    assert_eq!(
        game_core::modes::spawn::respawn_delay(&rules, 120),
        Some(120)
    );

    rules.modifiers = ModifierSet {
        survival: true,
        ..ModifierSet::default()
    };
    assert_eq!(
        game_core::modes::spawn::respawn_delay(&rules, 120),
        None,
        "in survival there is no respawn to wait for"
    );
}

#[test]
fn a_map_is_checked_against_the_mode_before_it_is_loaded() {
    let spawn = |x: f32, team: u8| MapSpawn {
        position: Vec2 { x, y: 400.0 },
        team,
        kind: game_core::SpawnKind::Player,
    };

    assert_eq!(
        validate_spawns(&deathmatch(), &[]),
        Err(SpawnProblem::NoSpawns)
    );
    assert_eq!(
        validate_spawns(&deathmatch(), &[spawn(100.0, 0)]),
        Err(SpawnProblem::NotEnoughSpawns {
            found: 1,
            needed: 2
        })
    );
    assert!(validate_spawns(&deathmatch(), &[spawn(100.0, 0), spawn(200.0, 0)]).is_ok());

    // A team mode needs somewhere for both sides.
    assert_eq!(
        validate_spawns(&teammatch(), &[spawn(100.0, ALPHA), spawn(200.0, ALPHA)]),
        Err(SpawnProblem::TeamHasNoSpawn { team: BRAVO })
    );
    assert!(validate_spawns(&teammatch(), &[spawn(100.0, ALPHA), spawn(900.0, BRAVO)]).is_ok());
    // Neutral spawns satisfy both sides, which is what keeps old maps playable.
    assert!(validate_spawns(&teammatch(), &[spawn(100.0, 0), spawn(900.0, 0)]).is_ok());
}

#[test]
fn a_player_only_spawns_where_their_mode_allows() {
    let spawns = [
        MapSpawn {
            position: Vec2 { x: 100.0, y: 400.0 },
            team: ALPHA,
            kind: game_core::SpawnKind::Player,
        },
        MapSpawn {
            position: Vec2 { x: 900.0, y: 400.0 },
            team: BRAVO,
            kind: game_core::SpawnKind::Player,
        },
        MapSpawn {
            position: Vec2 { x: 500.0, y: 400.0 },
            team: 0,
            kind: game_core::SpawnKind::Player,
        },
    ];

    // A free-for-all uses everything.
    assert_eq!(
        game_core::modes::spawn::usable(&deathmatch(), &spawns, NEUTRAL).count(),
        3
    );
    // A side uses its own spawns and the neutral ones, never the enemy's.
    let team_rules = teammatch();
    let alpha: Vec<_> = game_core::modes::spawn::usable(&team_rules, &spawns, ALPHA).collect();
    assert_eq!(alpha.len(), 2);
    assert!(alpha.iter().all(|spawn| spawn.team != BRAVO));
}

#[test]
fn every_mode_but_rambomatch_is_built_on_the_shared_lifecycle() {
    for playable in [
        ModeKind::Deathmatch,
        ModeKind::Teammatch,
        ModeKind::Pointmatch,
        ModeKind::CaptureTheFlag,
        ModeKind::Infiltration,
        ModeKind::HoldTheFlag,
    ] {
        assert!(playable.is_implemented(), "{playable:?}");
    }
    assert!(ModeKind::Rambomatch.is_implemented());
    for objective_mode in [
        ModeKind::Pointmatch,
        ModeKind::Rambomatch,
        ModeKind::CaptureTheFlag,
        ModeKind::Infiltration,
        ModeKind::HoldTheFlag,
    ] {
        assert!(objective_mode.has_objective(), "{objective_mode:?}");
    }

    // Room ids round-trip, and an unknown one is a playable deathmatch rather than a broken room.
    for kind in [
        ModeKind::Deathmatch,
        ModeKind::Teammatch,
        ModeKind::CaptureTheFlag,
    ] {
        assert_eq!(ModeKind::from_id(kind.id()), kind);
    }
    assert_eq!(ModeKind::from_id("nonsense"), ModeKind::Deathmatch);
    assert_eq!(ModeRules::from_mode_id("team").kind, ModeKind::Teammatch);
}

#[test]
fn the_default_limits_match_the_documented_defaults() {
    let dm = ModeRules::new(ModeKind::Deathmatch);
    assert_eq!(dm.limits.kills, 30);
    assert_eq!(dm.limits.time_ticks, 60 * 60 * 10, "ten minutes at 60 Hz");
    assert!(dm.limits.any());

    let ctf = ModeRules::new(ModeKind::CaptureTheFlag);
    assert_eq!(ctf.limits.captures, 10);
    assert_eq!(
        ctf.limits.kills, 0,
        "captures decide a flag mode, not kills"
    );
    assert!(ctf.overtime, "a level flag match is played out");
    assert!(!dm.overtime, "a deathmatch simply ends");

    // A match with every limit switched off never ends on its own, and says so.
    let endless = MatchLimits {
        kills: 0,
        points: 0,
        captures: 0,
        time_ticks: 0,
    };
    assert!(!endless.any());
}
