//! Parity fixtures for the five official flag modes, each against the shared framework.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:modes:pointmatch   — the yellow flag, the held bonus, drops, and the point limit
//!   rust:modes:rambomatch   — bow ownership, target indication, drop and reacquisition
//!   rust:modes:ctf          — bases, pickup, return, the own-flag-at-base capture rule
//!   rust:modes:infiltration — roles, defender points on a timer, attacker captures
//!   rust:modes:htf          — the neutral flag and continuous team scoring
//!
//! Source: OpenSoldat `shared/mechanics/Things.pas`, `server/ServerLoop.pas` (the HTF and
//! Infiltration scoring timers), and `shared/Constants.pas` at the commit pinned in
//! `docs/parity/reference-lock.md`.

use game_core::{
    infiltration_award, Bearer, CollisionWorld, FlagAction, FlagEvent, FlagKind, FlagState,
    ModeKind, ModeRules, ObjectivePolicy, ObjectiveRules, Objectives, TimedAward, Vec2, World,
    CTF_CAPTURE_AWARD, INFILTRATION_CAPTURE_AWARD, POINTMATCH_FLAG_MULTIPLIER,
};

const DT: f32 = 1.0 / 60.0;

fn collision() -> CollisionWorld {
    World::new("deathmatch").collision().clone()
}

fn rules(kind: ModeKind) -> ModeRules {
    ModeRules::new(kind)
}

const ALPHA_BASE: Vec2 = Vec2 { x: 200.0, y: 400.0 };
const BRAVO_BASE: Vec2 = Vec2 { x: 900.0, y: 400.0 };
const MIDDLE: Vec2 = Vec2 { x: 550.0, y: 400.0 };

fn objectives(kind: ModeKind) -> Objectives {
    Objectives::for_mode(&rules(kind), |flag| match flag {
        FlagKind::Alpha => Some(ALPHA_BASE),
        FlagKind::Bravo => Some(BRAVO_BASE),
        FlagKind::Yellow => Some(MIDDLE),
    })
}

fn bearer(id: u32, team: u8, pos: Vec2) -> Bearer {
    Bearer {
        id,
        team,
        pos,
        velocity: Vec2::default(),
        alive: true,
        throwing: false,
        aim: Vec2::default(),
    }
}

/// Steps the objectives for a while, collecting every point awarded.
fn run(
    objectives: &mut Objectives,
    kind: ModeKind,
    bearers: &[Bearer],
    sizes: (usize, usize),
    ticks: usize,
) -> Vec<game_core::Award> {
    let collision = collision();
    let rules = rules(kind);
    let mut awards = Vec::new();
    for _ in 0..ticks {
        awards.extend(objectives.step(&rules, &collision, DT, bearers, sizes));
    }
    awards
}

#[test]
fn each_mode_puts_the_right_flags_on_the_map() {
    assert!(objectives(ModeKind::Deathmatch).flags.is_empty());
    assert!(objectives(ModeKind::Teammatch).flags.is_empty());

    for neutral in [ModeKind::Pointmatch, ModeKind::HoldTheFlag] {
        let state = objectives(neutral);
        assert_eq!(state.flags.len(), 1, "{neutral:?}");
        assert_eq!(state.flags[0].kind, FlagKind::Yellow);
    }

    for per_team in [ModeKind::CaptureTheFlag, ModeKind::Infiltration] {
        let state = objectives(per_team);
        assert_eq!(state.flags.len(), 2, "{per_team:?}");
        assert!(state.flag(FlagKind::Alpha).is_some());
        assert!(state.flag(FlagKind::Bravo).is_some());
    }
}

// ----- Pointmatch -------------------------------------------------------------------------

#[test]
fn pointmatch_puts_one_yellow_flag_out_that_anybody_may_take() {
    let mut state = objectives(ModeKind::Pointmatch);
    assert_eq!(state.flags[0].kind, FlagKind::Yellow);
    assert_eq!(state.flags[0].base, MIDDLE);

    let policy = ObjectiveRules::Pointmatch;
    for team in [0u8, 1, 2] {
        assert_eq!(policy.action_for(&state.flags[0], team), FlagAction::Take);
    }

    run(
        &mut state,
        ModeKind::Pointmatch,
        &[bearer(1, 0, MIDDLE)],
        (1, 0),
        1,
    );
    assert_eq!(state.flags[0].state.carrier(), Some(1));
    assert!(state.is_carrying(1));
}

#[test]
fn pointmatch_doubles_a_kill_for_whoever_is_holding_the_flag() {
    let policy = ObjectiveRules::Pointmatch;
    assert_eq!(policy.kill_points(false), 1);
    assert_eq!(policy.kill_points(true), POINTMATCH_FLAG_MULTIPLIER);
    assert_eq!(POINTMATCH_FLAG_MULTIPLIER, 2);

    // No other mode changes what a kill is worth.
    for other in [
        ObjectiveRules::CaptureTheFlag,
        ObjectiveRules::HoldTheFlag,
        ObjectiveRules::Infiltration,
        ObjectiveRules::Rambomatch,
    ] {
        assert_eq!(other.kill_points(true), 1, "{other:?}");
    }
}

#[test]
fn pointmatch_drops_the_flag_when_its_holder_dies_and_lets_the_next_player_take_it() {
    let mut state = objectives(ModeKind::Pointmatch);
    run(
        &mut state,
        ModeKind::Pointmatch,
        &[bearer(1, 0, MIDDLE)],
        (1, 0),
        1,
    );
    assert_eq!(state.flags[0].state.carrier(), Some(1));

    let mut dead = bearer(1, 0, MIDDLE);
    dead.alive = false;
    run(&mut state, ModeKind::Pointmatch, &[dead], (1, 0), 1);
    assert!(state.flags[0].state.is_dropped(), "it fell where they did");
    assert!(state
        .events
        .iter()
        .any(|event| matches!(event, FlagEvent::Dropped { by: 1, .. })));

    // Somebody else walks over it and picks it up.
    let where_it_fell = state.flags[0].pos();
    run(
        &mut state,
        ModeKind::Pointmatch,
        &[bearer(2, 0, where_it_fell)],
        (1, 0),
        1,
    );
    assert_eq!(state.flags[0].state.carrier(), Some(2));
}

#[test]
fn pointmatch_never_scores_a_capture_because_there_is_nowhere_to_take_it() {
    let mut state = objectives(ModeKind::Pointmatch);
    let awards = run(
        &mut state,
        ModeKind::Pointmatch,
        &[bearer(1, 0, MIDDLE)],
        (1, 0),
        600,
    );
    assert!(
        awards.is_empty(),
        "pointmatch scores by killing while holding, not by carrying it home"
    );
    assert_eq!(
        state.flags[0].state.carrier(),
        Some(1),
        "and keeps holding it"
    );
}

#[test]
fn pointmatch_is_won_on_points_rather_than_on_kills() {
    let rules = rules(ModeKind::Pointmatch);
    assert_eq!(rules.limits.points, 30);
    assert_eq!(rules.limits.kills, 0);
    assert_eq!(rules.limits.captures, 0);
    assert!(
        !rules.is_team_mode(),
        "pointmatch is every player for themselves"
    );
}

// ----- Rambomatch -------------------------------------------------------------------------

#[test]
fn rambomatch_has_one_contested_weapon_rather_than_a_flag_on_a_base() {
    let state = objectives(ModeKind::Rambomatch);
    assert_eq!(
        state.flags.len(),
        1,
        "the bow stands in as the yellow contested objective"
    );
    assert_eq!(
        ObjectiveRules::Rambomatch.layout(),
        game_core::FlagLayout::Contested
    );

    let rules = rules(ModeKind::Rambomatch);
    assert_eq!(rules.limits.kills, 30, "it is still won on kills");
    assert!(!rules.is_team_mode());
}

#[test]
fn the_rambo_bow_can_be_taken_dropped_and_taken_again() {
    // The bow behaves like any other objective the framework can hand around: whoever is standing
    // on it takes it, and losing it puts it back in play.
    let mut bow = game_core::Flag::new(FlagKind::Yellow, MIDDLE);
    assert_eq!(
        ObjectiveRules::Rambomatch.action_for(&bow, 0),
        FlagAction::Take
    );

    bow.take(4);
    assert_eq!(bow.state.carrier(), Some(4), "that player is Rambo now");
    assert_eq!(
        ObjectiveRules::Rambomatch.action_for(&bow, 0),
        FlagAction::Nothing,
        "nobody takes it out of their hands"
    );

    bow.drop_at(Vec2 { x: 400.0, y: 300.0 }, Vec2::default());
    assert!(bow.state.is_dropped());
    assert_eq!(
        ObjectiveRules::Rambomatch.action_for(&bow, 0),
        FlagAction::Take,
        "and it is up for grabs again"
    );
    bow.take(5);
    assert_eq!(bow.state.carrier(), Some(5));
}

#[test]
fn rambomatch_scoring_belongs_to_whoever_holds_the_bow() {
    let policy = ObjectiveRules::Rambomatch;
    // The bow is what marks the target; carrying it does not itself pay out.
    let mut bow = game_core::Flag::new(FlagKind::Yellow, MIDDLE);
    bow.take(4);
    assert_eq!(policy.capture_award(&bow, None, 4, 0), None);
    assert_eq!(policy.kill_points(true), 1);
}

// ----- Capture the Flag -------------------------------------------------------------------

#[test]
fn ctf_gives_each_side_a_flag_on_its_own_base() {
    let state = objectives(ModeKind::CaptureTheFlag);
    assert_eq!(state.flag(FlagKind::Alpha).unwrap().base, ALPHA_BASE);
    assert_eq!(state.flag(FlagKind::Bravo).unwrap().base, BRAVO_BASE);
    assert!(rules(ModeKind::CaptureTheFlag).is_team_mode());
    assert_eq!(rules(ModeKind::CaptureTheFlag).limits.captures, 10);
}

#[test]
fn in_ctf_you_take_the_enemy_flag_and_leave_your_own_alone() {
    let state = objectives(ModeKind::CaptureTheFlag);
    let policy = ObjectiveRules::CaptureTheFlag;
    let alpha_flag = state.flag(FlagKind::Alpha).unwrap();
    let bravo_flag = state.flag(FlagKind::Bravo).unwrap();

    assert_eq!(policy.action_for(bravo_flag, 1), FlagAction::Take);
    assert_eq!(
        policy.action_for(alpha_flag, 1),
        FlagAction::Nothing,
        "your own flag at home is not picked up"
    );
    assert_eq!(policy.action_for(alpha_flag, 2), FlagAction::Take);
}

#[test]
fn touching_your_own_dropped_flag_sends_it_home_instead_of_picking_it_up() {
    let mut state = objectives(ModeKind::CaptureTheFlag);

    // Bravo runs off with Alpha's flag and is killed in the middle of the map.
    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(2, 2, ALPHA_BASE)],
        (1, 1),
        1,
    );
    assert_eq!(
        state.flag(FlagKind::Alpha).unwrap().state.carrier(),
        Some(2)
    );

    let mut dead = bearer(2, 2, MIDDLE);
    dead.alive = false;
    run(&mut state, ModeKind::CaptureTheFlag, &[dead], (1, 1), 1);
    assert!(state.flag(FlagKind::Alpha).unwrap().state.is_dropped());

    // An Alpha player touches it and it goes straight home rather than being carried.
    let where_it_fell = state.flag(FlagKind::Alpha).unwrap().pos();
    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, where_it_fell)],
        (1, 1),
        1,
    );
    let alpha_flag = state.flag(FlagKind::Alpha).unwrap();
    assert_eq!(alpha_flag.state, FlagState::AtBase);
    assert_eq!(alpha_flag.pos(), ALPHA_BASE);
    assert!(state.events.iter().any(|event| matches!(
        event,
        FlagEvent::Returned {
            flag: FlagKind::Alpha,
            by: Some(1)
        }
    )));
}

#[test]
fn a_capture_only_counts_while_your_own_flag_is_standing_at_home() {
    let mut state = objectives(ModeKind::CaptureTheFlag);

    // Both flags leave home at once: Alpha takes Bravo's, Bravo takes Alpha's.
    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, BRAVO_BASE), bearer(2, 2, ALPHA_BASE)],
        (1, 1),
        1,
    );
    assert_eq!(
        state.flag(FlagKind::Bravo).unwrap().state.carrier(),
        Some(1)
    );
    assert_eq!(
        state.flag(FlagKind::Alpha).unwrap().state.carrier(),
        Some(2)
    );

    // Alpha runs the enemy flag home, but their own is gone, so there is nothing to score against.
    let awards = run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, ALPHA_BASE), bearer(2, 2, MIDDLE)],
        (1, 1),
        1,
    );
    assert!(awards.is_empty(), "no capture while your own flag is out");
    assert_eq!(
        state.flag(FlagKind::Bravo).unwrap().state.carrier(),
        Some(1),
        "and they are still holding it"
    );
}

#[test]
fn a_capture_scores_when_both_conditions_are_met_and_sends_the_flag_home() {
    let mut state = objectives(ModeKind::CaptureTheFlag);

    // Alpha takes Bravo's flag while their own stays put.
    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, BRAVO_BASE)],
        (1, 1),
        1,
    );
    assert_eq!(
        state.flag(FlagKind::Bravo).unwrap().state.carrier(),
        Some(1)
    );

    let awards = run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, ALPHA_BASE)],
        (1, 1),
        1,
    );
    assert_eq!(awards.len(), 1, "one capture");
    assert_eq!(awards[0].team, 1);
    assert_eq!(awards[0].player, 1);
    assert_eq!(awards[0].points, CTF_CAPTURE_AWARD);

    let bravo_flag = state.flag(FlagKind::Bravo).unwrap();
    assert_eq!(bravo_flag.state, FlagState::AtBase, "the flag went home");
    assert_eq!(bravo_flag.pos(), BRAVO_BASE);
    assert!(state.events.iter().any(|event| matches!(
        event,
        FlagEvent::Captured {
            flag: FlagKind::Bravo,
            by: 1
        }
    )));
}

#[test]
fn a_carrier_can_throw_the_flag_instead_of_carrying_it_all_the_way() {
    let mut state = objectives(ModeKind::CaptureTheFlag);
    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, BRAVO_BASE)],
        (1, 1),
        1,
    );

    let mut thrower = bearer(1, 1, MIDDLE);
    thrower.throwing = true;
    thrower.aim = Vec2 { x: 100.0, y: 400.0 };
    run(&mut state, ModeKind::CaptureTheFlag, &[thrower], (1, 1), 1);

    let flag = state.flag(FlagKind::Bravo).unwrap();
    assert!(flag.state.is_dropped(), "it left their hands");
    assert!(flag.body.vel.x < 0.0, "and went the way it was thrown");
}

#[test]
fn the_hud_can_tell_which_flags_are_away_from_home_and_who_has_them() {
    let mut state = objectives(ModeKind::CaptureTheFlag);
    assert!(!state.is_missing(FlagKind::Alpha));
    assert!(!state.is_missing(FlagKind::Bravo));
    assert_eq!(state.carriers().count(), 0);

    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, BRAVO_BASE)],
        (1, 1),
        1,
    );
    assert!(state.is_missing(FlagKind::Bravo), "bravo's flag is out");
    assert!(!state.is_missing(FlagKind::Alpha));
    let carriers: Vec<_> = state.carriers().collect();
    assert_eq!(carriers, vec![(FlagKind::Bravo, 1)]);
}

// ----- Infiltration -----------------------------------------------------------------------

#[test]
fn infiltration_has_one_side_attacking_and_one_defending() {
    let policy = ObjectiveRules::Infiltration;
    assert_eq!(policy.attacking_team(), Some(1), "alpha attacks");
    assert!(policy.is_defender(2), "bravo defends");
    assert!(!policy.is_defender(1));

    // Every other mode is symmetrical.
    for symmetrical in [ObjectiveRules::CaptureTheFlag, ObjectiveRules::HoldTheFlag] {
        assert_eq!(symmetrical.attacking_team(), None);
        assert!(!symmetrical.is_defender(1));
        assert!(!symmetrical.is_defender(2));
    }
}

#[test]
fn in_infiltration_only_the_defenders_objective_is_in_play() {
    let state = objectives(ModeKind::Infiltration);
    let policy = ObjectiveRules::Infiltration;
    let alpha_flag = state.flag(FlagKind::Alpha).unwrap();
    let bravo_flag = state.flag(FlagKind::Bravo).unwrap();

    // The attackers go for the defenders' objective.
    assert_eq!(policy.action_for(bravo_flag, 1), FlagAction::Take);
    // The attackers' own flag is scenery: nobody may touch it, from either side.
    assert_eq!(policy.action_for(alpha_flag, 1), FlagAction::Nothing);
    assert_eq!(policy.action_for(alpha_flag, 2), FlagAction::Nothing);
}

#[test]
fn an_infiltration_capture_is_worth_far_more_than_a_ctf_one() {
    let mut state = objectives(ModeKind::Infiltration);
    run(
        &mut state,
        ModeKind::Infiltration,
        &[bearer(1, 1, BRAVO_BASE)],
        (1, 1),
        1,
    );
    let awards = run(
        &mut state,
        ModeKind::Infiltration,
        &[bearer(1, 1, ALPHA_BASE)],
        (1, 1),
        1,
    );
    assert_eq!(awards.len(), 1);
    assert_eq!(awards[0].points, INFILTRATION_CAPTURE_AWARD);
    assert_eq!(INFILTRATION_CAPTURE_AWARD, 30);
    // An infiltration capture is a whole round's work; a CTF one happens over and over.
    assert_eq!(CTF_CAPTURE_AWARD, 1);
}

#[test]
fn stacking_the_attack_in_infiltration_is_worth_less_per_capture() {
    assert_eq!(infiltration_award(4, 4), INFILTRATION_CAPTURE_AWARD);
    assert_eq!(infiltration_award(3, 4), INFILTRATION_CAPTURE_AWARD);
    assert_eq!(infiltration_award(5, 4), INFILTRATION_CAPTURE_AWARD - 5);
    assert_eq!(infiltration_award(8, 4), INFILTRATION_CAPTURE_AWARD - 20);
    // It never goes negative, however lopsided it gets.
    assert_eq!(infiltration_award(40, 1), 0);
}

#[test]
fn the_infiltration_defenders_score_simply_for_keeping_their_objective_at_home() {
    let mut state = objectives(ModeKind::Infiltration);
    let timer = TimedAward::INFILTRATION_DEFENCE;

    // Nothing happens before the interval is up.
    let early = run(
        &mut state,
        ModeKind::Infiltration,
        &[bearer(1, 1, MIDDLE), bearer(2, 2, BRAVO_BASE)],
        (1, 1),
        timer.base_ticks as usize - 1,
    );
    assert!(early.is_empty());

    let paid = run(
        &mut state,
        ModeKind::Infiltration,
        &[bearer(1, 1, MIDDLE), bearer(2, 2, BRAVO_BASE)],
        (1, 1),
        1,
    );
    assert_eq!(paid.len(), 1, "the defenders got their point");
    assert_eq!(paid[0].team, 2);
}

#[test]
fn the_infiltration_defenders_stop_scoring_once_their_objective_is_taken() {
    let mut state = objectives(ModeKind::Infiltration);
    let timer = TimedAward::INFILTRATION_DEFENCE;

    // An attacker grabs the objective.
    run(
        &mut state,
        ModeKind::Infiltration,
        &[bearer(1, 1, BRAVO_BASE), bearer(2, 2, MIDDLE)],
        (1, 1),
        1,
    );
    assert_eq!(
        state.flag(FlagKind::Bravo).unwrap().state.carrier(),
        Some(1)
    );

    let awards = run(
        &mut state,
        ModeKind::Infiltration,
        &[bearer(1, 1, MIDDLE), bearer(2, 2, ALPHA_BASE)],
        (1, 1),
        timer.base_ticks as usize * 2,
    );
    assert!(
        awards.is_empty(),
        "no points for a side whose objective is in enemy hands"
    );
}

// ----- Hold the Flag ----------------------------------------------------------------------

#[test]
fn htf_has_one_neutral_flag_that_either_side_may_hold() {
    let state = objectives(ModeKind::HoldTheFlag);
    assert_eq!(state.flags.len(), 1);
    assert_eq!(state.flags[0].kind, FlagKind::Yellow);
    assert!(rules(ModeKind::HoldTheFlag).is_team_mode());

    let policy = ObjectiveRules::HoldTheFlag;
    assert_eq!(policy.action_for(&state.flags[0], 1), FlagAction::Take);
    assert_eq!(policy.action_for(&state.flags[0], 2), FlagAction::Take);
}

#[test]
fn htf_scores_continuously_for_whichever_side_is_holding_the_flag() {
    let mut state = objectives(ModeKind::HoldTheFlag);
    let timer = TimedAward::HOLD_THE_FLAG;

    run(
        &mut state,
        ModeKind::HoldTheFlag,
        &[bearer(1, 1, MIDDLE)],
        (1, 1),
        1,
    );
    assert_eq!(state.flags[0].state.carrier(), Some(1));

    let awards = run(
        &mut state,
        ModeKind::HoldTheFlag,
        &[bearer(1, 1, MIDDLE), bearer(2, 2, ALPHA_BASE)],
        (1, 1),
        timer.base_ticks as usize * 3,
    );
    assert!(awards.len() >= 2, "it keeps paying: {}", awards.len());
    assert!(awards.iter().all(|award| award.team == 1));
}

#[test]
fn htf_stops_scoring_the_moment_the_flag_is_dropped() {
    let mut state = objectives(ModeKind::HoldTheFlag);
    let timer = TimedAward::HOLD_THE_FLAG;
    run(
        &mut state,
        ModeKind::HoldTheFlag,
        &[bearer(1, 1, MIDDLE)],
        (1, 1),
        1,
    );

    let mut dead = bearer(1, 1, MIDDLE);
    dead.alive = false;
    let awards = run(
        &mut state,
        ModeKind::HoldTheFlag,
        &[dead],
        (1, 1),
        timer.base_ticks as usize * 2,
    );
    assert!(awards.is_empty(), "a flag on the ground scores for nobody");
    assert!(state.flags[0].state.is_dropped());
}

#[test]
fn a_stacked_side_scores_more_slowly_rather_than_not_at_all() {
    let timer = TimedAward::HOLD_THE_FLAG;
    let even = timer.interval(0);
    let ahead_by_two = timer.interval(2);
    let behind = timer.interval(-3);

    assert_eq!(even, timer.base_ticks);
    assert!(ahead_by_two > even, "outnumbering the other side slows you");
    assert_eq!(
        behind, timer.base_ticks,
        "being outnumbered does not speed you up"
    );
    assert!(timer.interval(100) >= timer.floor_ticks);
}

#[test]
fn neither_timed_mode_scores_while_one_side_is_empty() {
    for kind in [ModeKind::HoldTheFlag, ModeKind::Infiltration] {
        let mut state = objectives(kind);
        let awards = run(&mut state, kind, &[bearer(1, 1, MIDDLE)], (1, 0), 2_000);
        assert!(
            awards.is_empty(),
            "{kind:?} should not run up a score against nobody"
        );
    }
}

// ----- Shared behaviour -------------------------------------------------------------------

#[test]
fn a_round_reset_sends_every_flag_home() {
    let mut state = objectives(ModeKind::CaptureTheFlag);
    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, BRAVO_BASE), bearer(2, 2, ALPHA_BASE)],
        (1, 1),
        1,
    );
    assert!(state.is_missing(FlagKind::Alpha));
    assert!(state.is_missing(FlagKind::Bravo));

    state.reset();
    assert!(!state.is_missing(FlagKind::Alpha));
    assert!(!state.is_missing(FlagKind::Bravo));
    assert_eq!(state.flag(FlagKind::Alpha).unwrap().pos(), ALPHA_BASE);
    assert_eq!(state.flag(FlagKind::Bravo).unwrap().pos(), BRAVO_BASE);
}

#[test]
fn a_blast_shoves_a_loose_flag_but_not_one_in_somebodys_hands() {
    let mut state = objectives(ModeKind::HoldTheFlag);
    state.push_near(MIDDLE, 90.0, Vec2 { x: 120.0, y: -80.0 });
    assert_ne!(state.flags[0].body.vel, Vec2::default());

    state.flags[0].body.vel = Vec2::default();
    state.push_near(Vec2 { x: 5.0, y: 5.0 }, 10.0, Vec2 { x: 500.0, y: 0.0 });
    assert_eq!(
        state.flags[0].body.vel,
        Vec2::default(),
        "a blast across the map does not reach it"
    );

    run(
        &mut state,
        ModeKind::HoldTheFlag,
        &[bearer(1, 1, MIDDLE)],
        (1, 1),
        1,
    );
    state.push_near(MIDDLE, 90.0, Vec2 { x: 500.0, y: 0.0 });
    assert_eq!(
        state.flags[0].body.vel,
        Vec2::default(),
        "a carried flag is not blown out of a player's hands"
    );
}

#[test]
fn a_carrier_who_leaves_the_match_does_not_take_the_flag_with_them() {
    let mut state = objectives(ModeKind::CaptureTheFlag);
    run(
        &mut state,
        ModeKind::CaptureTheFlag,
        &[bearer(1, 1, BRAVO_BASE)],
        (1, 1),
        1,
    );
    assert_eq!(
        state.flag(FlagKind::Bravo).unwrap().state.carrier(),
        Some(1)
    );

    // They disconnect: no bearer at all this tick.
    run(&mut state, ModeKind::CaptureTheFlag, &[], (1, 1), 1);
    assert!(
        state.flag(FlagKind::Bravo).unwrap().state.is_dropped(),
        "the flag stays in the match"
    );
}

#[test]
fn a_flag_mode_refuses_a_map_with_nowhere_to_stand_its_bases() {
    use game_core::{validate_spawns, MapSpawn, SpawnProblem};

    let spawn = |x: f32, team: u8| MapSpawn {
        position: Vec2 { x, y: 400.0 },
        team,
        kind: game_core::SpawnKind::Player,
    };

    // Two neutral spawns are enough for Teammatch: both sides simply start somewhere.
    assert!(validate_spawns(
        &rules(ModeKind::Teammatch),
        &[spawn(100.0, 0), spawn(900.0, 0)]
    )
    .is_ok());

    // They are not enough for CTF, because both flags would end up on the same square.
    assert_eq!(
        validate_spawns(
            &rules(ModeKind::CaptureTheFlag),
            &[spawn(100.0, 0), spawn(900.0, 0)]
        ),
        Err(SpawnProblem::NoBaseForFlag { team: 1 })
    );
    assert_eq!(
        validate_spawns(
            &rules(ModeKind::CaptureTheFlag),
            &[spawn(100.0, 1), spawn(900.0, 0)]
        ),
        Err(SpawnProblem::NoBaseForFlag { team: 2 })
    );
    assert!(validate_spawns(
        &rules(ModeKind::CaptureTheFlag),
        &[spawn(100.0, 1), spawn(900.0, 2)]
    )
    .is_ok());

    // A neutral-flag mode needs no per-side base at all.
    assert!(validate_spawns(
        &rules(ModeKind::HoldTheFlag),
        &[spawn(100.0, 0), spawn(900.0, 0)]
    )
    .is_ok());
}
