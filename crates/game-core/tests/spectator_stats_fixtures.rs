//! Parity fixtures for spectating and for the match statistics ledger.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:spectator:targets    — joining, switching, previous/next, and a free camera
//!   rust:spectator:visibility — Realistic and Survival restrictions, and the broadcast delay
//!   rust:statistics:weapons   — shots, hits, accuracy, and per-weapon kills and deaths
//!   rust:statistics:scoreboard — rank, difference from the leader, and objective counts
//!   rust:statistics:history   — end-of-round summaries, match history, and exportable logs
//!
//! Source: the Soldat spectating and statistics documentation listed in `docs/gaps/gap-list.md`,
//! against the revision pinned in `docs/parity/reference-lock.md`.

use game_core::{
    modes, scoreboard, CollisionWorld, DamageCause, MatchHistory, MatchStats, MatchSummary,
    ModeKind, ModeRules, ModifierSet, ObjectiveStat, ScoreEvent, ScoreLedger, SpectateCommand,
    Spectator, SpectatorView, StatEvent, Vec2, Viewer, Visibility, WeaponKind, World, ALPHA, BRAVO,
    NEUTRAL, SPECTATOR,
};

fn collision() -> CollisionWorld {
    World::new("deathmatch").collision().clone()
}

fn realistic() -> ModeRules {
    let mut rules = ModeRules::new(ModeKind::Teammatch);
    rules.modifiers = ModifierSet {
        realistic: true,
        ..ModifierSet::default()
    };
    rules
}

// ----- Spectating -------------------------------------------------------------------------

#[test]
fn a_spectator_may_follow_anybody_who_is_playing_but_not_another_spectator() {
    let candidates = [(1u32, ALPHA), (2, BRAVO), (9, SPECTATOR), (4, NEUTRAL)];
    let followable = modes::spectator::followable(candidates.into_iter());

    assert_eq!(followable, vec![1, 2, 4], "sorted, and no spectators");
    assert!(
        !followable.contains(&9),
        "watching a watcher would show an empty screen"
    );
}

#[test]
fn joining_as_a_spectator_starts_following_somebody() {
    let watching = Spectator::new(Some(1));
    assert_eq!(watching.view, SpectatorView::Following);
    assert_eq!(watching.target, Some(1));
    assert!(!watching.is_free());

    // Joining an empty match follows nobody rather than pointing at a player who is not there.
    let alone = Spectator::new(None);
    assert_eq!(alone.target, None);
}

#[test]
fn next_and_previous_walk_the_list_and_wrap_around() {
    let followable = vec![1u32, 2, 3];
    let mut spectator = Spectator::new(Some(1));

    modes::spectator::apply(&mut spectator, SpectateCommand::Next, &followable);
    assert_eq!(spectator.target, Some(2));
    modes::spectator::apply(&mut spectator, SpectateCommand::Next, &followable);
    assert_eq!(spectator.target, Some(3));
    modes::spectator::apply(&mut spectator, SpectateCommand::Next, &followable);
    assert_eq!(spectator.target, Some(1), "it wraps");

    modes::spectator::apply(&mut spectator, SpectateCommand::Previous, &followable);
    assert_eq!(spectator.target, Some(3), "and wraps the other way");
    modes::spectator::apply(&mut spectator, SpectateCommand::Previous, &followable);
    assert_eq!(spectator.target, Some(2));
}

#[test]
fn a_spectator_can_ask_for_one_player_and_is_refused_a_player_who_is_not_there() {
    let followable = vec![1u32, 2];
    let mut spectator = Spectator::new(Some(1));

    modes::spectator::apply(&mut spectator, SpectateCommand::Follow(2), &followable);
    assert_eq!(spectator.target, Some(2));

    modes::spectator::apply(&mut spectator, SpectateCommand::Follow(99), &followable);
    assert_eq!(
        spectator.target,
        Some(2),
        "a player who left is not followed"
    );
}

#[test]
fn a_free_camera_lets_go_of_whoever_was_being_followed_and_can_be_taken_back() {
    let followable = vec![1u32, 2, 3];
    let mut spectator = Spectator::new(Some(2));

    modes::spectator::apply(&mut spectator, SpectateCommand::FreeCamera, &followable);
    assert!(spectator.is_free());
    assert_eq!(spectator.target, None);

    // Next off a free camera starts at the top of the list, Previous at the bottom.
    modes::spectator::apply(&mut spectator, SpectateCommand::Next, &followable);
    assert_eq!(spectator.target, Some(1));
    assert!(!spectator.is_free());

    modes::spectator::apply(&mut spectator, SpectateCommand::FreeCamera, &followable);
    modes::spectator::apply(&mut spectator, SpectateCommand::Previous, &followable);
    assert_eq!(spectator.target, Some(3));
}

#[test]
fn cycling_with_nobody_to_watch_leaves_the_camera_where_it_was() {
    let mut spectator = Spectator::new(None);
    modes::spectator::apply(&mut spectator, SpectateCommand::Next, &[]);
    assert_eq!(spectator.target, None);
    modes::spectator::apply(&mut spectator, SpectateCommand::Previous, &[]);
    assert_eq!(spectator.target, None);
}

#[test]
fn a_followed_player_who_leaves_hands_the_camera_to_somebody_else() {
    let mut spectator = Spectator::new(Some(2));
    modes::spectator::retarget_if_gone(&mut spectator, &[1, 2, 3]);
    assert_eq!(spectator.target, Some(2), "still there, so nothing changed");

    modes::spectator::retarget_if_gone(&mut spectator, &[1, 3]);
    assert_eq!(spectator.target, Some(1), "they left, so it moved on");

    modes::spectator::retarget_if_gone(&mut spectator, &[]);
    assert_eq!(spectator.target, None, "nobody left to watch");

    // A free camera is left alone: it was not following anybody to begin with.
    let mut free = Spectator {
        view: SpectatorView::FreeCamera,
        target: None,
        camera: Vec2::default(),
    };
    modes::spectator::retarget_if_gone(&mut free, &[1]);
    assert!(free.is_free());
    assert_eq!(free.target, None);
}

#[test]
fn a_free_camera_is_kept_inside_the_map() {
    let clamped = modes::spectator::clamp_camera(
        Vec2 {
            x: -9000.0,
            y: 9000.0,
        },
        1200.0,
        700.0,
    );
    assert!(clamped.x >= -modes::spectator::FREE_CAMERA_MARGIN);
    assert!(clamped.y <= 700.0 + modes::spectator::FREE_CAMERA_MARGIN);

    // A camera already inside the map is left exactly where it was.
    let inside = Vec2 { x: 600.0, y: 350.0 };
    assert_eq!(
        modes::spectator::clamp_camera(inside, 1200.0, 700.0),
        inside
    );
}

#[test]
fn a_spectator_talks_to_other_spectators_and_not_to_the_players() {
    assert!(!modes::spectator::chat_reaches_players());
    assert!(modes::spectator::chat_reaches_spectators());
}

#[test]
fn spectating_a_normal_match_shows_everything() {
    let rules = ModeRules::new(ModeKind::Teammatch);
    let collision = collision();
    let spectator = Spectator::new(Some(1));
    let followed = Viewer::new(1, ALPHA, Vec2 { x: 600.0, y: 600.0 }, true);
    let hidden_away = Viewer::new(2, BRAVO, Vec2 { x: 600.0, y: 690.0 }, true);

    assert_eq!(
        modes::spectator::visibility_for(
            &rules,
            &collision,
            &spectator,
            Some(followed),
            hidden_away
        ),
        Visibility::Full,
        "there is nothing to hide outside Realistic"
    );
}

#[test]
fn spectating_realistic_shows_only_what_the_followed_player_can_see() {
    let rules = realistic();
    let collision = collision();
    let spectator = Spectator::new(Some(1));
    let followed = Viewer::new(1, ALPHA, Vec2 { x: 600.0, y: 600.0 }, true);

    // An enemy the followed player cannot see is not shown to the spectator either.
    let behind_cover = Viewer::new(2, BRAVO, Vec2 { x: 600.0, y: 690.0 }, true);
    assert_eq!(
        modes::spectator::visibility_for(
            &rules,
            &collision,
            &spectator,
            Some(followed),
            behind_cover
        ),
        Visibility::Hidden
    );

    // One they can see is shown.
    let in_the_open = Viewer::new(2, BRAVO, Vec2 { x: 640.0, y: 600.0 }, true);
    assert_eq!(
        modes::spectator::visibility_for(
            &rules,
            &collision,
            &spectator,
            Some(followed),
            in_the_open
        ),
        Visibility::Full
    );

    // And the followed player themselves is always shown.
    assert_eq!(
        modes::spectator::visibility_for(&rules, &collision, &spectator, Some(followed), followed),
        Visibility::Full
    );
}

#[test]
fn a_free_camera_in_realistic_sees_nothing_at_all() {
    let rules = realistic();
    let collision = collision();
    let free = Spectator {
        view: SpectatorView::FreeCamera,
        target: None,
        camera: Vec2::default(),
    };
    let anybody = Viewer::new(2, BRAVO, Vec2 { x: 600.0, y: 600.0 }, true);

    assert_eq!(
        modes::spectator::visibility_for(&rules, &collision, &free, None, anybody),
        Visibility::Hidden,
        "a free camera would otherwise be a way round the whole mode"
    );
}

#[test]
fn a_competitive_broadcast_can_be_delayed_and_the_delay_is_capped() {
    assert_eq!(modes::spectator::broadcast_delay(0), 0, "live by default");
    assert_eq!(modes::spectator::broadcast_delay(600), 600);
    assert_eq!(
        modes::spectator::broadcast_delay(u32::MAX),
        modes::spectator::MAX_BROADCAST_DELAY_TICKS,
        "a server cannot be configured into never showing anything"
    );

    assert_eq!(modes::spectator::delayed_tick(1_000, 0), 1_000);
    assert_eq!(modes::spectator::delayed_tick(1_000, 600), 400);
    assert_eq!(
        modes::spectator::delayed_tick(100, 600),
        0,
        "early in a match there is nothing older to show"
    );
}

// ----- Statistics -------------------------------------------------------------------------

#[test]
fn shots_and_hits_are_counted_per_player_and_per_weapon() {
    let mut stats = MatchStats::default();
    for _ in 0..10 {
        stats.record(StatEvent::Shot {
            player: 1,
            weapon: WeaponKind::Ak74,
        });
    }
    for _ in 0..4 {
        stats.record(StatEvent::Hit {
            player: 1,
            weapon: WeaponKind::Ak74,
            headshot: false,
        });
    }
    stats.record(StatEvent::Shot {
        player: 1,
        weapon: WeaponKind::Barrett,
    });
    stats.record(StatEvent::Hit {
        player: 1,
        weapon: WeaponKind::Barrett,
        headshot: true,
    });

    let player = stats.player(1);
    assert_eq!(player.shots, 11);
    assert_eq!(player.hits, 5);
    assert_eq!(player.headshots, 1);

    let rifle = player.weapons[&WeaponKind::Ak74];
    assert_eq!(rifle.shots, 10);
    assert_eq!(rifle.hits, 4);
    assert_eq!(rifle.accuracy_percent(), 40);

    let sniper = player.weapons[&WeaponKind::Barrett];
    assert_eq!(sniper.accuracy_percent(), 100);
    assert_eq!(sniper.headshots, 1);
}

#[test]
fn accuracy_with_no_shots_is_nothing_rather_than_a_divide_by_zero() {
    let stats = MatchStats::default();
    assert_eq!(stats.player(1).accuracy_percent(), 0);
    assert_eq!(game_core::WeaponStats::default().accuracy_percent(), 0);
}

#[test]
fn kills_and_deaths_are_counted_against_the_weapon_that_caused_them() {
    let mut stats = MatchStats::default();
    stats.record(StatEvent::KilledWith {
        killer: 1,
        victim: 2,
        weapon: WeaponKind::Spas12,
    });
    stats.record(StatEvent::KilledWith {
        killer: 1,
        victim: 3,
        weapon: WeaponKind::Spas12,
    });
    stats.record(StatEvent::KilledWith {
        killer: 1,
        victim: 2,
        weapon: WeaponKind::CombatKnife,
    });

    let killer = stats.player(1);
    assert_eq!(killer.weapons[&WeaponKind::Spas12].kills, 2);
    assert_eq!(killer.weapons[&WeaponKind::CombatKnife].kills, 1);
    assert_eq!(killer.favourite_weapon(), Some(WeaponKind::Spas12));

    let victim = stats.player(2);
    assert_eq!(victim.weapons[&WeaponKind::Spas12].deaths, 1);
    assert_eq!(victim.weapons[&WeaponKind::CombatKnife].deaths, 1);
    assert_eq!(
        victim.favourite_weapon(),
        None,
        "dying to a weapon is not killing with it"
    );
}

#[test]
fn deaths_to_something_other_than_a_weapon_are_counted_by_cause() {
    let mut stats = MatchStats::default();
    stats.record(StatEvent::DiedTo {
        victim: 1,
        cause: DamageCause::Fall,
    });
    stats.record(StatEvent::DiedTo {
        victim: 1,
        cause: DamageCause::Fall,
    });
    stats.record(StatEvent::DiedTo {
        victim: 1,
        cause: DamageCause::Deadly,
    });

    let player = stats.player(1);
    assert_eq!(player.deaths_by_cause[&DamageCause::Fall], 2);
    assert_eq!(player.deaths_by_cause[&DamageCause::Deadly], 1);
    assert!(!player.deaths_by_cause.contains_key(&DamageCause::Bullet));
}

#[test]
fn objective_work_is_counted_separately_from_kills() {
    let mut stats = MatchStats::default();
    stats.record(StatEvent::Objective {
        player: 1,
        kind: ObjectiveStat::Capture,
    });
    stats.record(StatEvent::Objective {
        player: 1,
        kind: ObjectiveStat::Capture,
    });
    stats.record(StatEvent::Objective {
        player: 1,
        kind: ObjectiveStat::Return,
    });
    stats.record(StatEvent::Objective {
        player: 1,
        kind: ObjectiveStat::Hold,
    });

    let player = stats.player(1);
    assert_eq!(player.captures, 2);
    assert_eq!(player.returns, 1);
    assert_eq!(player.holds, 1);
}

#[test]
fn a_scoreboard_ranks_by_points_and_says_how_far_behind_the_leader_everybody_is() {
    let rules = ModeRules::new(ModeKind::Deathmatch);
    let mut ledger = ScoreLedger::default();
    for _ in 0..5 {
        ledger.record(
            ScoreEvent::Kill {
                killer: 1,
                victim: 2,
            },
            &rules,
            |_| NEUTRAL,
        );
    }
    for _ in 0..2 {
        ledger.record(
            ScoreEvent::Kill {
                killer: 3,
                victim: 2,
            },
            &rules,
            |_| NEUTRAL,
        );
    }

    let rows = scoreboard(&ledger, &MatchStats::default(), |_| NEUTRAL);
    assert_eq!(rows.len(), 3, "everybody who has a row in the ledger");

    assert_eq!(rows[0].player, 1);
    assert_eq!(rows[0].rank, 1);
    assert_eq!(rows[0].behind_leader, 0, "the leader is not behind anybody");

    assert_eq!(rows[1].player, 3);
    assert_eq!(rows[1].rank, 2);
    assert_eq!(rows[1].behind_leader, 3);

    // Player 2 only ever died, so they are last.
    assert_eq!(rows[2].player, 2);
    assert_eq!(rows[2].score.deaths, 7);
}

#[test]
fn a_scoreboard_breaks_ties_the_same_way_every_time() {
    let rules = ModeRules::new(ModeKind::Deathmatch);
    let mut ledger = ScoreLedger::default();
    // Three players on the same points; 5 has more kills, 7 and 9 are level.
    for id in [9u32, 7, 5] {
        ledger.record(
            ScoreEvent::Kill {
                killer: id,
                victim: 99,
            },
            &rules,
            |_| NEUTRAL,
        );
    }

    let first = scoreboard(&ledger, &MatchStats::default(), |_| NEUTRAL);
    let second = scoreboard(&ledger, &MatchStats::default(), |_| NEUTRAL);
    assert_eq!(first, second, "the same match always sorts the same way");

    let order: Vec<u32> = first.iter().take(3).map(|row| row.player).collect();
    assert_eq!(order, vec![5, 7, 9], "a level score falls back to the id");
}

#[test]
fn the_scoreboard_carries_each_players_statistics_alongside_their_score() {
    let rules = ModeRules::new(ModeKind::CaptureTheFlag);
    let mut ledger = ScoreLedger::default();
    ledger.record(
        ScoreEvent::Objective {
            player: 1,
            team: ALPHA,
            points: 1,
        },
        &rules,
        |_| ALPHA,
    );

    let mut stats = MatchStats::default();
    stats.record(StatEvent::Shot {
        player: 1,
        weapon: WeaponKind::Ak74,
    });
    stats.record(StatEvent::Hit {
        player: 1,
        weapon: WeaponKind::Ak74,
        headshot: true,
    });
    stats.record(StatEvent::Objective {
        player: 1,
        kind: ObjectiveStat::Capture,
    });

    let rows = scoreboard(&ledger, &stats, |_| ALPHA);
    assert_eq!(rows[0].team, ALPHA);
    assert_eq!(rows[0].stats.captures, 1);
    assert_eq!(rows[0].stats.headshots, 1);
    assert_eq!(rows[0].stats.accuracy_percent(), 100);
    assert_eq!(rows[0].score.objectives, 1);
}

#[test]
fn an_empty_match_produces_an_empty_scoreboard_rather_than_a_phantom_row() {
    let rows = scoreboard(&ScoreLedger::default(), &MatchStats::default(), |_| NEUTRAL);
    assert!(rows.is_empty());
}

#[test]
fn a_finished_match_becomes_a_summary_that_reads_as_plain_text() {
    let rules = ModeRules::new(ModeKind::Deathmatch);
    let mut ledger = ScoreLedger::default();
    ledger.record(
        ScoreEvent::Kill {
            killer: 1,
            victim: 2,
        },
        &rules,
        |_| NEUTRAL,
    );
    let mut stats = MatchStats::default();
    stats.record(StatEvent::Shot {
        player: 1,
        weapon: WeaponKind::Ak74,
    });
    stats.record(StatEvent::Hit {
        player: 1,
        weapon: WeaponKind::Ak74,
        headshot: false,
    });

    let summary = MatchSummary {
        map: "Arena".into(),
        mode: "deathmatch".into(),
        duration: 36_000,
        outcome: "Player 1 wins".into(),
        rows: scoreboard(&ledger, &stats, |_| NEUTRAL),
    };

    let log = summary.to_log();
    assert!(log.contains("Arena"), "{log}");
    assert!(log.contains("deathmatch"));
    assert!(log.contains("Player 1 wins"));
    assert!(log.contains("100%"), "accuracy is in the log: {log}");
    // One header line, one column line, and a line per player.
    assert_eq!(log.lines().count(), 2 + summary.rows.len());
}

#[test]
fn match_history_keeps_the_recent_matches_and_forgets_the_rest() {
    let mut history = MatchHistory::default();
    assert!(history.is_empty());
    assert_eq!(history.latest(), None);

    let summary = |name: &str| MatchSummary {
        map: name.into(),
        mode: "deathmatch".into(),
        duration: 100,
        outcome: "Draw".into(),
        rows: Vec::new(),
    };

    for index in 0..modes::statistics::MATCH_HISTORY_LIMIT + 5 {
        history.push(summary(&format!("Map {index}")));
    }

    assert_eq!(
        history.len(),
        modes::statistics::MATCH_HISTORY_LIMIT,
        "a long-running server does not hold every match it ever played"
    );
    assert_eq!(
        history.latest().map(|entry| entry.map.as_str()),
        Some("Map 24"),
        "the newest is kept"
    );
    assert!(
        !history.entries().iter().any(|entry| entry.map == "Map 0"),
        "and the oldest was dropped"
    );
}

#[test]
fn the_whole_history_exports_as_one_log() {
    let mut history = MatchHistory::default();
    history.push(MatchSummary {
        map: "First".into(),
        mode: "deathmatch".into(),
        duration: 10,
        outcome: "Draw".into(),
        rows: Vec::new(),
    });
    history.push(MatchSummary {
        map: "Second".into(),
        mode: "ctf".into(),
        duration: 20,
        outcome: "Alpha wins".into(),
        rows: Vec::new(),
    });

    let log = history.to_log();
    assert!(log.contains("First"));
    assert!(log.contains("Second"));
    assert!(
        log.find("First") < log.find("Second"),
        "oldest first, so a log reads forwards"
    );
}

#[test]
fn clearing_the_statistics_leaves_nothing_behind() {
    let mut stats = MatchStats::default();
    stats.record(StatEvent::Shot {
        player: 1,
        weapon: WeaponKind::Ak74,
    });
    assert_eq!(stats.player(1).shots, 1);

    stats.clear();
    assert_eq!(stats.player(1).shots, 0);
    assert_eq!(stats.entries().count(), 0);

    // Ensuring a row makes one without inventing any activity.
    stats.ensure(1);
    assert_eq!(stats.entries().count(), 1);
    assert_eq!(stats.player(1).shots, 0);
}
