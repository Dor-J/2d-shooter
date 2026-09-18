//! Parity fixtures for bots: what they can see, how they move, how they fight, and what they are
//! trying to achieve in each mode.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:bots:profiles   — difficulty, accuracy, reaction time, and custom profiles
//!   rust:bots:perception — sight, line of sight, target choice, and memory
//!   rust:bots:navigation — movement, jet, arrival, stuck detection, and recovery
//!   rust:bots:combat     — firing, reloading, grenades, and weapon choice
//!   rust:bots:objectives — every official mode's goal, and the attack/defend split
//!   rust:bots:chat       — what a bot says and how rarely

use game_core::{
    bots, Awareness, Bot, BotChatEvent, BotContact, BotGoal, BotProfile, CollisionWorld,
    Difficulty, FlagKind, ModeKind, ModeRules, Navigation, Objectives, ProfileError, Unlocked,
    Vec2, WeaponKind, WeaponTable, World, ALPHA, BRAVO, NEUTRAL,
};

fn collision() -> CollisionWorld {
    World::new("deathmatch").collision().clone()
}

fn contact(id: u32, team: u8, pos: Vec2) -> BotContact {
    BotContact {
        id,
        team,
        pos,
        velocity: Vec2::default(),
        alive: true,
    }
}

const ALPHA_BASE: Vec2 = Vec2 { x: 200.0, y: 400.0 };
const BRAVO_BASE: Vec2 = Vec2 { x: 900.0, y: 400.0 };
const MIDDLE: Vec2 = Vec2 { x: 550.0, y: 400.0 };

fn objectives_for(kind: ModeKind) -> Objectives {
    Objectives::for_mode(&ModeRules::new(kind), |flag| match flag {
        FlagKind::Alpha => Some(ALPHA_BASE),
        FlagKind::Bravo => Some(BRAVO_BASE),
        FlagKind::Yellow => Some(MIDDLE),
    })
}

// ----- Profiles and difficulty ------------------------------------------------------------

#[test]
fn a_harder_bot_reacts_faster_shoots_straighter_and_sees_further() {
    let mut previous: Option<Difficulty> = None;
    for difficulty in Difficulty::ALL {
        if let Some(easier) = previous {
            assert!(
                difficulty.reaction_ticks() < easier.reaction_ticks(),
                "{difficulty:?} reacts faster than {easier:?}"
            );
            assert!(
                difficulty.aim_error() < easier.aim_error(),
                "{difficulty:?} shoots straighter"
            );
            assert!(
                difficulty.sight_range() > easier.sight_range(),
                "{difficulty:?} sees further"
            );
            assert!(
                difficulty.grenade_reluctance() < easier.grenade_reluctance(),
                "{difficulty:?} throws more readily"
            );
        }
        previous = Some(difficulty);
    }
    assert_eq!(Difficulty::ALL.len(), 4);
}

#[test]
fn even_an_elite_bot_takes_a_moment_to_react() {
    assert!(
        Difficulty::Elite.reaction_ticks() > 0,
        "a bot that fires the instant you round a corner is unfair, not hard"
    );
    assert!(
        Difficulty::Elite.aim_error() > 0.0,
        "and one that never misses is not a fight"
    );
}

#[test]
fn a_difficulty_can_be_named_and_an_unknown_name_is_simply_normal() {
    for difficulty in Difficulty::ALL {
        assert_eq!(Difficulty::from_id(difficulty.id()), difficulty);
    }
    assert_eq!(Difficulty::from_id("easy"), Difficulty::Rookie);
    assert_eq!(Difficulty::from_id("HARD"), Difficulty::Veteran);
    assert_eq!(Difficulty::from_id("nonsense"), Difficulty::Normal);
    assert_eq!(Difficulty::from_id(""), Difficulty::Normal);
}

#[test]
fn a_bot_profile_is_checked_before_it_is_used() {
    let base = BotProfile::new("Boogie", Difficulty::Normal);
    assert!(base.validate().is_ok());

    assert_eq!(
        BotProfile {
            name: "   ".into(),
            ..base.clone()
        }
        .validate(),
        Err(ProfileError::EmptyName)
    );
    assert!(matches!(
        BotProfile {
            name: "x".repeat(200),
            ..base.clone()
        }
        .validate(),
        Err(ProfileError::NameTooLong { .. })
    ));
    assert!(matches!(
        BotProfile {
            aggression: 200,
            ..base.clone()
        }
        .validate(),
        Err(ProfileError::AggressionOutOfRange { .. })
    ));
    assert_eq!(
        BotProfile {
            favourite_weapon: WeaponKind::StationaryGun,
            ..base
        }
        .validate(),
        Err(ProfileError::UnselectableWeapon)
    );
}

#[test]
fn a_bots_name_is_always_something_a_scoreboard_can_show() {
    assert_eq!(bots::profile::clean_name("  Sniper  "), "Sniper");
    assert_eq!(bots::profile::clean_name(""), "Bot", "never nameless");
    assert_eq!(bots::profile::clean_name("   "), "Bot");
    assert!(bots::profile::clean_name(&"y".repeat(500)).len() <= bots::profile::MAX_NAME_BYTES);
    assert!(!bots::profile::clean_name("a\nb\tc").contains('\n'));
}

#[test]
fn the_stock_profiles_are_all_usable_and_all_different() {
    let profiles = bots::profile::default_profiles();
    assert!(profiles.len() >= 4);
    for profile in &profiles {
        assert!(profile.validate().is_ok(), "{}", profile.name);
    }
    let mut names: Vec<&str> = profiles.iter().map(|p| p.name.as_str()).collect();
    names.sort_unstable();
    let count = names.len();
    names.dedup();
    assert_eq!(names.len(), count, "no two stock bots share a name");
}

// ----- Perception -------------------------------------------------------------------------

#[test]
fn a_bot_sees_through_open_air_and_not_through_the_ground() {
    let collision = collision();
    let above = Vec2 { x: 600.0, y: 600.0 };
    let below = Vec2 { x: 600.0, y: 690.0 };
    let near = Vec2 { x: 630.0, y: 600.0 };

    assert!(!bots::perception::can_see(&collision, above, below, 600.0));
    assert!(bots::perception::can_see(&collision, above, near, 600.0));
}

#[test]
fn a_bot_does_not_see_past_its_own_sight_range() {
    let collision = collision();
    let here = Vec2 { x: 100.0, y: 200.0 };
    let far = Vec2 {
        x: 100.0 + Difficulty::Rookie.sight_range() + 20.0,
        y: 200.0,
    };
    assert!(!bots::perception::can_see(
        &collision,
        here,
        far,
        Difficulty::Rookie.sight_range()
    ));
    // A better bot sees the same target.
    assert!(bots::perception::can_see(
        &collision,
        here,
        far,
        Difficulty::Elite.sight_range()
    ));
}

#[test]
fn a_bot_picks_the_nearest_enemy_and_never_a_teammate() {
    let collision = collision();
    let mut awareness = Awareness::default();
    let me = contact(1, ALPHA, Vec2 { x: 600.0, y: 400.0 });
    let others = [
        contact(2, ALPHA, Vec2 { x: 610.0, y: 400.0 }),
        contact(3, BRAVO, Vec2 { x: 700.0, y: 400.0 }),
        contact(4, BRAVO, Vec2 { x: 660.0, y: 400.0 }),
    ];

    bots::perception::observe(&mut awareness, &collision, Difficulty::Elite, me, &others);
    assert_eq!(
        awareness.target,
        Some(4),
        "the nearest enemy, not the teammate"
    );
}

#[test]
fn in_a_free_for_all_everybody_is_a_target() {
    let collision = collision();
    let mut awareness = Awareness::default();
    let me = contact(1, NEUTRAL, Vec2 { x: 600.0, y: 400.0 });
    let others = [contact(2, NEUTRAL, Vec2 { x: 640.0, y: 400.0 })];

    bots::perception::observe(&mut awareness, &collision, Difficulty::Elite, me, &others);
    assert_eq!(awareness.target, Some(2));
}

#[test]
fn a_bot_waits_its_reaction_time_before_acting_on_what_it_sees() {
    let collision = collision();
    let mut awareness = Awareness::default();
    let me = contact(1, ALPHA, Vec2 { x: 600.0, y: 400.0 });
    let others = [contact(2, BRAVO, Vec2 { x: 640.0, y: 400.0 })];

    bots::perception::observe(&mut awareness, &collision, Difficulty::Normal, me, &others);
    assert_eq!(awareness.target, Some(2));
    assert!(!awareness.has_reacted(Difficulty::Normal), "not yet");

    for _ in 0..Difficulty::Normal.reaction_ticks() {
        bots::perception::observe(&mut awareness, &collision, Difficulty::Normal, me, &others);
    }
    assert!(awareness.has_reacted(Difficulty::Normal));
}

#[test]
fn a_new_target_starts_the_reaction_clock_again() {
    let collision = collision();
    let mut awareness = Awareness::default();
    let me = contact(1, ALPHA, Vec2 { x: 600.0, y: 400.0 });
    let first = [contact(2, BRAVO, Vec2 { x: 640.0, y: 400.0 })];

    for _ in 0..100 {
        bots::perception::observe(&mut awareness, &collision, Difficulty::Normal, me, &first);
    }
    assert!(awareness.has_reacted(Difficulty::Normal));

    // Somebody closer appears: the bot has to notice them too.
    let second = [
        contact(2, BRAVO, Vec2 { x: 640.0, y: 400.0 }),
        contact(3, BRAVO, Vec2 { x: 610.0, y: 400.0 }),
    ];
    bots::perception::observe(&mut awareness, &collision, Difficulty::Normal, me, &second);
    assert_eq!(awareness.target, Some(3));
    assert!(
        !awareness.has_reacted(Difficulty::Normal),
        "a new face, a new clock"
    );
}

#[test]
fn a_bot_keeps_hunting_for_a_moment_after_losing_sight_and_then_gives_up() {
    let collision = collision();
    let mut awareness = Awareness::default();
    let me = contact(1, ALPHA, Vec2 { x: 600.0, y: 400.0 });
    let seen = [contact(2, BRAVO, Vec2 { x: 640.0, y: 400.0 })];

    bots::perception::observe(&mut awareness, &collision, Difficulty::Elite, me, &seen);
    let last_seen = awareness.last_seen;
    assert_eq!(last_seen, Vec2 { x: 640.0, y: 400.0 });

    // They vanish. The bot remembers where they were.
    bots::perception::observe(&mut awareness, &collision, Difficulty::Elite, me, &[]);
    assert_eq!(awareness.target, Some(2));
    assert!(awareness.is_hunting_a_memory());
    assert_eq!(awareness.last_seen, last_seen);

    for _ in 0..bots::perception::MEMORY_TICKS + 2 {
        bots::perception::observe(&mut awareness, &collision, Difficulty::Elite, me, &[]);
    }
    assert_eq!(awareness.target, None, "eventually it gives up");
}

#[test]
fn a_dead_bot_forgets_everything() {
    let collision = collision();
    let mut awareness = Awareness::default();
    let mut me = contact(1, ALPHA, Vec2 { x: 600.0, y: 400.0 });
    let others = [contact(2, BRAVO, Vec2 { x: 640.0, y: 400.0 })];

    bots::perception::observe(&mut awareness, &collision, Difficulty::Elite, me, &others);
    assert!(awareness.target.is_some());

    me.alive = false;
    bots::perception::observe(&mut awareness, &collision, Difficulty::Elite, me, &others);
    assert_eq!(awareness.target, None);
}

#[test]
fn a_bot_leads_a_moving_target_rather_than_shooting_where_they_were() {
    let mut moving = contact(2, BRAVO, Vec2 { x: 600.0, y: 400.0 });
    moving.velocity = Vec2 { x: 200.0, y: 0.0 };

    // With no aim error at all, the lead is the whole difference.
    let led = bots::perception::aim_point(Difficulty::Elite, moving, 400.0, 400.0, 0.5);
    assert!(led.x > moving.pos.x, "it aims ahead of them: {led:?}");

    let still = contact(2, BRAVO, Vec2 { x: 600.0, y: 400.0 });
    let straight = bots::perception::aim_point(Difficulty::Elite, still, 400.0, 400.0, 0.5);
    assert_eq!(straight.x, still.pos.x, "and straight at a stationary one");
}

#[test]
fn a_worse_bot_misses_by_more() {
    let target = contact(2, BRAVO, Vec2 { x: 600.0, y: 400.0 });
    // The same jitter through two difficulties: the worse one is further out.
    let rookie = bots::perception::aim_point(Difficulty::Rookie, target, 400.0, 100.0, 1.0);
    let elite = bots::perception::aim_point(Difficulty::Elite, target, 400.0, 100.0, 1.0);
    assert!(
        (rookie.x - target.pos.x).abs() > (elite.x - target.pos.x).abs(),
        "rookie {rookie:?} elite {elite:?}"
    );
}

// ----- Navigation -------------------------------------------------------------------------

#[test]
fn a_bot_walks_towards_where_it_is_going() {
    let mut nav = Navigation::default();
    let here = Vec2 { x: 400.0, y: 400.0 };
    nav.go_to(Vec2 { x: 800.0, y: 400.0 }, here);

    let movement = bots::navigation::steer(&nav, here, 1.0);
    assert!(movement.right, "the destination is to the right");
    assert!(!movement.left);

    nav.go_to(Vec2 { x: 100.0, y: 400.0 }, here);
    assert!(bots::navigation::steer(&nav, here, 1.0).left);
}

#[test]
fn a_bot_jumps_at_a_step_and_jets_at_a_climb() {
    let here = Vec2 { x: 400.0, y: 400.0 };
    let mut nav = Navigation::default();

    nav.go_to(Vec2 { x: 440.0, y: 360.0 }, here);
    let step = bots::navigation::steer(&nav, here, 1.0);
    assert!(step.jump, "a small step is a jump");
    assert!(!step.jet, "and not worth the fuel");

    nav.go_to(Vec2 { x: 440.0, y: 200.0 }, here);
    let climb = bots::navigation::steer(&nav, here, 1.0);
    assert!(climb.jump && climb.jet, "a real climb wants the jet");

    // With an empty tank it does what it can.
    let dry = bots::navigation::steer(&nav, here, 0.0);
    assert!(dry.jump && !dry.jet);
}

#[test]
fn a_bot_with_nowhere_to_go_stands_still() {
    let nav = Navigation::default();
    let movement = bots::navigation::steer(&nav, Vec2 { x: 400.0, y: 400.0 }, 1.0);
    assert_eq!(movement, game_core::Movement::default());
}

#[test]
fn a_bot_knows_when_it_has_arrived() {
    let mut nav = Navigation::default();
    let here = Vec2 { x: 400.0, y: 400.0 };
    assert!(nav.has_arrived(here), "nowhere to go is already there");

    nav.go_to(Vec2 { x: 800.0, y: 400.0 }, here);
    assert!(!nav.has_arrived(here));
    assert!(nav.has_arrived(Vec2 { x: 790.0, y: 400.0 }));
}

#[test]
fn a_bot_that_stops_making_progress_decides_it_is_stuck_and_tries_something_else() {
    let mut nav = Navigation::default();
    let wall = Vec2 { x: 400.0, y: 400.0 };
    nav.go_to(Vec2 { x: 800.0, y: 400.0 }, wall);

    let mut stuck_at = None;
    for tick in 0..bots::navigation::STUCK_TICKS + 5 {
        if nav.check_progress(wall, true) {
            stuck_at = Some(tick);
            break;
        }
    }
    assert!(stuck_at.is_some(), "it noticed it was going nowhere");
    assert!(nav.is_recovering());

    // While recovering it backs the other way and hops, which clears most snags.
    let recovery = bots::navigation::steer(&nav, wall, 1.0);
    assert!(recovery.jump);
    assert!(
        recovery.left != recovery.right,
        "it commits to one direction"
    );
}

#[test]
fn a_bot_that_is_moving_never_thinks_it_is_stuck() {
    let mut nav = Navigation::default();
    let mut pos = Vec2 { x: 400.0, y: 400.0 };
    nav.go_to(Vec2 { x: 900.0, y: 400.0 }, pos);

    for _ in 0..200 {
        pos.x += bots::navigation::PROGRESS_DISTANCE + 1.0;
        assert!(!nav.check_progress(pos, true), "it is making progress");
    }
    assert!(!nav.is_recovering());
}

#[test]
fn a_bot_routes_through_a_waypoint_that_actually_helps() {
    let from = Vec2 { x: 100.0, y: 400.0 };
    let goal = Vec2 { x: 900.0, y: 400.0 };
    let helpful = bots::navigation::Waypoint {
        pos: Vec2 { x: 500.0, y: 400.0 },
        needs_jet: false,
    };
    let useless = bots::navigation::Waypoint {
        pos: Vec2 { x: 50.0, y: 400.0 },
        needs_jet: false,
    };

    assert_eq!(
        bots::navigation::next_waypoint(&[helpful, useless], from, goal),
        Some(helpful.pos),
        "the one that gets it closer"
    );
    assert_eq!(
        bots::navigation::next_waypoint(&[useless], from, goal),
        None,
        "a detour that helps nothing is not taken"
    );
    assert_eq!(
        bots::navigation::next_waypoint(&[], from, goal),
        None,
        "a map with no waypoints simply walks at the goal"
    );
}

// ----- Combat -----------------------------------------------------------------------------

#[test]
fn a_bot_shoots_only_once_it_has_reacted_and_only_with_something_loaded() {
    let mut awareness = Awareness::default();
    assert!(
        !bots::combat::should_fire(&awareness, Difficulty::Normal, 30, false),
        "no target"
    );

    awareness.target = Some(2);
    awareness.tracking_ticks = Difficulty::Normal.reaction_ticks();
    assert!(bots::combat::should_fire(
        &awareness,
        Difficulty::Normal,
        30,
        false
    ));

    assert!(
        !bots::combat::should_fire(&awareness, Difficulty::Normal, 0, false),
        "an empty magazine"
    );
    assert!(
        !bots::combat::should_fire(&awareness, Difficulty::Normal, 30, true),
        "mid-reload"
    );

    awareness.lost_ticks = 5;
    assert!(
        !bots::combat::should_fire(&awareness, Difficulty::Normal, 30, false),
        "it does not shoot at a memory"
    );
}

#[test]
fn a_bot_reloads_when_empty_and_in_a_lull_but_not_mid_fight() {
    let mut awareness = Awareness::default();
    assert!(
        bots::combat::should_reload(&awareness, 0, 30, false),
        "empty, always"
    );
    assert!(
        !bots::combat::should_reload(&awareness, 0, 30, true),
        "already reloading"
    );
    assert!(
        bots::combat::should_reload(&awareness, 5, 30, false),
        "low and nobody about"
    );
    assert!(
        !bots::combat::should_reload(&awareness, 20, 30, false),
        "plenty left"
    );

    awareness.target = Some(2);
    assert!(
        !bots::combat::should_reload(&awareness, 5, 30, false),
        "reloading in a fight is how a bot dies holding a full magazine"
    );
}

#[test]
fn a_bot_throws_a_grenade_at_a_sensible_range_and_never_at_its_own_feet() {
    let profile = BotProfile::new("Thrower", Difficulty::Elite);
    let awareness = Awareness {
        target: Some(2),
        tracking_ticks: 100,
        ..Awareness::default()
    };

    // A roll that always passes, so only the range is under test.
    let throws =
        |distance: f32| bots::combat::should_throw_grenade(&profile, &awareness, 2, distance, 0);
    assert!(!throws(40.0), "too close: it would blow itself up");
    assert!(throws(200.0));
    assert!(!throws(900.0), "too far to be worth one");

    assert!(
        !bots::combat::should_throw_grenade(&profile, &awareness, 0, 200.0, 0),
        "none left"
    );

    // It has to have reacted first.
    let fresh = Awareness {
        target: Some(2),
        ..Awareness::default()
    };
    assert!(!bots::combat::should_throw_grenade(
        &profile, &fresh, 2, 200.0, 0
    ));
}

#[test]
fn a_bot_reaches_for_a_blade_up_close_and_its_favourite_at_range() {
    let table = WeaponTable::normal();
    let profile = BotProfile {
        favourite_weapon: WeaponKind::Barrett,
        ..BotProfile::new("Sniper", Difficulty::Veteran)
    };
    let everything = Unlocked::everything();

    assert_eq!(
        bots::combat::preferred_weapon(&profile, &table, everything, Some(20.0)),
        WeaponKind::CombatKnife.slot()
    );
    assert_eq!(
        bots::combat::preferred_weapon(&profile, &table, everything, Some(500.0)),
        WeaponKind::Barrett.slot()
    );
    assert_eq!(
        bots::combat::preferred_weapon(&profile, &table, everything, None),
        WeaponKind::Barrett.slot()
    );
}

#[test]
fn a_bot_in_advance_mode_reaches_for_something_it_has_actually_earned() {
    let table = WeaponTable::normal();
    let profile = BotProfile {
        favourite_weapon: WeaponKind::Barrett,
        ..BotProfile::new("Climber", Difficulty::Normal)
    };
    let nothing = Unlocked::starting();

    let chosen = bots::combat::preferred_weapon(&profile, &table, nothing, Some(400.0))
        .expect("it picked something");
    assert_ne!(
        Some(chosen),
        WeaponKind::Barrett.slot(),
        "it has not earned the Barrett"
    );
    assert!(nothing.has(WeaponKind::from_slot(chosen)));
}

#[test]
fn a_shotgun_bot_closes_and_a_sniper_bot_keeps_its_distance() {
    let profile = BotProfile::new("Fighter", Difficulty::Normal);

    assert!(
        bots::combat::wants_to_close(&profile, WeaponKind::Spas12, 300.0),
        "a shotgun wants to be close"
    );
    assert!(
        !bots::combat::wants_to_close(&profile, WeaponKind::Barrett, 300.0),
        "a Barrett does not"
    );
    assert!(
        bots::combat::wants_to_close(&profile, WeaponKind::Barrett, 900.0),
        "though even a sniper closes from right across the map"
    );
}

#[test]
fn an_aggressive_bot_closes_from_further_out() {
    let calm = BotProfile {
        aggression: 0,
        ..BotProfile::new("Calm", Difficulty::Normal)
    };
    let eager = BotProfile {
        aggression: 100,
        ..BotProfile::new("Eager", Difficulty::Normal)
    };

    // A distance where the eager one advances and the calm one holds.
    let distance = 160.0;
    assert!(bots::combat::wants_to_close(
        &eager,
        WeaponKind::Ak74,
        distance
    ));
    assert!(!bots::combat::wants_to_close(
        &calm,
        WeaponKind::Ak74,
        distance
    ));
}

// ----- Objectives -------------------------------------------------------------------------

#[test]
fn a_deathmatch_bot_simply_fights() {
    let rules = ModeRules::new(ModeKind::Deathmatch);
    let objectives = objectives_for(ModeKind::Deathmatch);
    let objective = bots::objectives::choose(&rules, &objectives, 1, NEUTRAL, false);
    assert_eq!(objective.goal, BotGoal::Fight);
    assert_eq!(objective.destination, None);
    assert!(!objective.goal.is_travel());
}

#[test]
fn a_ctf_attacker_goes_for_the_enemy_flag_and_a_defender_stays_home() {
    let rules = ModeRules::new(ModeKind::CaptureTheFlag);
    let objectives = objectives_for(ModeKind::CaptureTheFlag);

    let attacker = bots::objectives::choose(&rules, &objectives, 1, ALPHA, false);
    assert_eq!(attacker.goal, BotGoal::TakeObjective);
    assert_eq!(attacker.destination, Some(BRAVO_BASE));

    let defender = bots::objectives::choose(&rules, &objectives, 2, ALPHA, true);
    assert_eq!(defender.goal, BotGoal::Defend);
    assert_eq!(defender.destination, Some(ALPHA_BASE));
}

#[test]
fn a_ctf_bot_carrying_the_flag_takes_it_home() {
    let rules = ModeRules::new(ModeKind::CaptureTheFlag);
    let mut objectives = objectives_for(ModeKind::CaptureTheFlag);
    objectives
        .flags
        .iter_mut()
        .find(|flag| flag.kind == FlagKind::Bravo)
        .unwrap()
        .take(1);

    let objective = bots::objectives::choose(&rules, &objectives, 1, ALPHA, false);
    assert_eq!(objective.goal, BotGoal::CarryHome);
    assert_eq!(objective.destination, Some(ALPHA_BASE));
}

#[test]
fn a_ctf_bot_drops_everything_to_return_its_own_flag() {
    let rules = ModeRules::new(ModeKind::CaptureTheFlag);
    let mut objectives = objectives_for(ModeKind::CaptureTheFlag);
    let fell_at = Vec2 { x: 500.0, y: 300.0 };
    {
        let own = objectives
            .flags
            .iter_mut()
            .find(|flag| flag.kind == FlagKind::Alpha)
            .unwrap();
        own.take(9);
        own.drop_at(fell_at, Vec2::default());
    }

    // Even the attacker goes for it: nothing can be capped while it is out.
    let objective = bots::objectives::choose(&rules, &objectives, 1, ALPHA, false);
    assert_eq!(objective.goal, BotGoal::ReturnOwnFlag);
    assert_eq!(objective.destination, Some(fell_at));
}

#[test]
fn an_infiltration_attacker_always_attacks_and_a_defender_always_defends() {
    let rules = ModeRules::new(ModeKind::Infiltration);
    let objectives = objectives_for(ModeKind::Infiltration);

    // Alpha attacks, whatever the defender split says.
    let attacker = bots::objectives::choose(&rules, &objectives, 1, ALPHA, true);
    assert_eq!(attacker.goal, BotGoal::TakeObjective);

    // Bravo defends, whatever it says.
    let defender = bots::objectives::choose(&rules, &objectives, 2, BRAVO, false);
    assert_eq!(defender.goal, BotGoal::Defend);
    assert_eq!(defender.destination, Some(BRAVO_BASE));
}

#[test]
fn an_htf_bot_goes_for_the_yellow_flag() {
    let rules = ModeRules::new(ModeKind::HoldTheFlag);
    let objectives = objectives_for(ModeKind::HoldTheFlag);

    let objective = bots::objectives::choose(&rules, &objectives, 1, ALPHA, false);
    assert_eq!(objective.goal, BotGoal::TakeObjective);
    assert_eq!(objective.destination, Some(MIDDLE));
}

#[test]
fn an_htf_defender_escorts_a_teammate_who_has_the_flag() {
    let rules = ModeRules::new(ModeKind::HoldTheFlag);
    let mut objectives = objectives_for(ModeKind::HoldTheFlag);
    objectives.flags[0].take(7);

    let escort = bots::objectives::choose(&rules, &objectives, 1, ALPHA, true);
    assert_eq!(escort.goal, BotGoal::EscortCarrier);

    // An attacker goes for whoever is holding it instead.
    let chaser = bots::objectives::choose(&rules, &objectives, 2, ALPHA, false);
    assert_eq!(chaser.goal, BotGoal::TakeObjective);
}

#[test]
fn a_pointmatch_bot_goes_for_the_flag_and_then_fights_while_holding_it() {
    let rules = ModeRules::new(ModeKind::Pointmatch);
    let mut objectives = objectives_for(ModeKind::Pointmatch);

    assert_eq!(
        bots::objectives::choose(&rules, &objectives, 1, NEUTRAL, false).goal,
        BotGoal::TakeObjective
    );

    objectives.flags[0].take(1);
    assert_eq!(
        bots::objectives::choose(&rules, &objectives, 1, NEUTRAL, false).goal,
        BotGoal::Fight,
        "holding it, the point is to kill people"
    );
}

#[test]
fn a_team_splits_into_attackers_and_defenders_rather_than_all_chasing_one_flag() {
    // A team of six gets at least one of each, and the split does not flicker.
    let roles: Vec<bool> = (0..6)
        .map(|id| bots::objectives::is_defender(id, 6))
        .collect();
    assert!(
        roles.iter().any(|defender| *defender),
        "somebody holds ground"
    );
    assert!(roles.iter().any(|defender| !*defender), "somebody pushes");
    assert_eq!(
        roles,
        (0..6)
            .map(|id| bots::objectives::is_defender(id, 6))
            .collect::<Vec<_>>(),
        "and it is the same answer every time"
    );

    // A lone bot never sits at home doing nothing.
    assert!(!bots::objectives::is_defender(0, 1));
}

#[test]
fn a_bot_with_nothing_to_do_wanders_towards_a_spawn() {
    let spawns = [Vec2 { x: 100.0, y: 400.0 }, Vec2 { x: 900.0, y: 400.0 }];
    let target = bots::objectives::wander_target(&spawns, 1);
    assert!(spawns.contains(&target.expect("somewhere to go")));
    assert_eq!(bots::objectives::wander_target(&[], 1), None);
}

// ----- Counts and chat --------------------------------------------------------------------

#[test]
fn bots_fill_both_sides_evenly_in_a_team_mode() {
    assert_eq!(bots::wanted_per_team(4, true), (2, 2));
    assert_eq!(
        bots::wanted_per_team(5, true),
        (3, 2),
        "an odd one goes to alpha"
    );
    assert_eq!(bots::wanted_per_team(0, true), (0, 0));
    assert_eq!(bots::wanted_per_team(6, false), (6, 0), "no sides to fill");
}

#[test]
fn a_bot_says_something_occasionally_and_then_keeps_quiet() {
    let mut bot = Bot::new(BotProfile::new("Chatty", Difficulty::Normal));
    bot.chat_cooldown = 0;

    let first = bots::chat_line(&mut bot, BotChatEvent::Killed);
    assert!(first.is_some(), "it said something");
    assert!(
        bots::chat_line(&mut bot, BotChatEvent::Died).is_none(),
        "and then shut up for a while"
    );

    bot.chat_cooldown = 0;
    assert!(bots::chat_line(&mut bot, BotChatEvent::Captured).is_some());
}

#[test]
fn a_quiet_bot_never_says_anything() {
    let mut bot = Bot::new(BotProfile {
        chatty: false,
        ..BotProfile::new("Silent", Difficulty::Normal)
    });
    bot.chat_cooldown = 0;
    assert_eq!(bots::chat_line(&mut bot, BotChatEvent::Killed), None);
}

#[test]
fn a_bot_has_something_to_say_about_each_of_the_things_it_notices() {
    for event in [
        BotChatEvent::Killed,
        BotChatEvent::Died,
        BotChatEvent::TookObjective,
        BotChatEvent::Captured,
    ] {
        let mut bot = Bot::new(BotProfile::new("Chatty", Difficulty::Normal));
        bot.chat_cooldown = 0;
        let line = bots::chat_line(&mut bot, event).expect("a line");
        assert!(!line.trim().is_empty(), "{event:?}");
    }
}
