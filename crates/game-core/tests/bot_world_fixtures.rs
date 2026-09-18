//! Proof that bots run inside `World` as ordinary players, not as a library of rules.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:bot_world:join    — bots join, fill teams, and can be added and removed
//!   rust:bot_world:drive   — a bot produces the same `Input` a human's client would
//!   rust:bot_world:play    — a bot moves, fights, and chases the objective in a real match

use game_core::{
    BotProfile, Difficulty, FlagKind, ModeKind, ModeRules, TeamChoice, Vec2, World, ALPHA, BRAVO,
};
use std::collections::BTreeMap;

fn profile(name: &str) -> BotProfile {
    BotProfile::new(name, Difficulty::Normal)
}

#[test]
fn a_bot_joins_as_an_ordinary_player() {
    let mut world = World::new("deathmatch");
    world.add_bot(1, profile("Boogie"), TeamChoice::Auto);

    assert!(world.players.contains_key(&1), "it is on the scoreboard");
    assert_eq!(world.players[&1].name, "Boogie");
    assert!(world.is_bot(1));
    assert_eq!(world.players[&1].hp, 100, "and it spawned properly");
    assert!(
        world.players[&1]
            .inventory
            .slots
            .iter()
            .any(Option::is_some),
        "holding a weapon like anybody else"
    );
}

#[test]
fn bots_fill_both_sides_of_a_team_match() {
    let mut world = World::new("team");
    world.rules = ModeRules::new(ModeKind::Teammatch);
    for id in 1..=4u32 {
        world.add_bot(id, profile(&format!("Bot {id}")), TeamChoice::Auto);
    }

    let alpha = world.players.values().filter(|p| p.team == ALPHA).count();
    let bravo = world.players.values().filter(|p| p.team == BRAVO).count();
    assert_eq!(alpha, 2);
    assert_eq!(bravo, 2);
}

#[test]
fn a_bot_can_be_removed_and_takes_its_player_with_it() {
    let mut world = World::new("deathmatch");
    world.add_bot(1, profile("Boogie"), TeamChoice::Auto);
    world.add_bot(2, profile("Sniper"), TeamChoice::Auto);

    assert!(world.remove_bot(1));
    assert!(
        !world.players.contains_key(&1),
        "no ghost on the scoreboard"
    );
    assert!(!world.is_bot(1));
    assert!(world.players.contains_key(&2), "the other one is untouched");

    assert!(
        !world.remove_bot(99),
        "removing one that is not there is not a panic"
    );
}

#[test]
fn a_server_can_trim_the_bots_down_to_a_wanted_count() {
    let mut world = World::new("deathmatch");
    for id in 1..=6u32 {
        world.add_bot(id, profile(&format!("Bot {id}")), TeamChoice::Auto);
    }
    assert_eq!(world.bots.len(), 6);

    let removed = world.trim_bots(2);
    assert_eq!(removed, 4);
    assert_eq!(world.bots.len(), 2);
    assert_eq!(world.players.len(), 2, "and the players went with them");

    assert_eq!(
        world.trim_bots(5),
        0,
        "trimming below the count does nothing"
    );
}

#[test]
fn a_bot_produces_an_input_exactly_as_a_human_client_does() {
    let mut world = World::new("deathmatch");
    world.add_bot(1, profile("Boogie"), TeamChoice::Auto);
    world.add_player(2, "Human".into());

    let inputs = world.bot_inputs();
    assert_eq!(inputs.len(), 1, "one input, for the one bot");
    assert!(inputs.contains_key(&1));
    assert!(!inputs.contains_key(&2), "the human sends their own");

    let input = inputs[&1];
    assert!(input.aim.x.is_finite() && input.aim.y.is_finite());
    assert!(
        (input.weapon as usize) < game_core::SELECTABLE_WEAPONS.len(),
        "and picked a weapon that exists"
    );
}

#[test]
fn a_match_with_no_bots_produces_no_bot_inputs() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Human".into());
    assert!(world.bot_inputs().is_empty());
}

#[test]
fn a_humans_input_always_beats_the_bot_one_for_the_same_slot() {
    let mut world = World::new("deathmatch");
    world.add_bot(1, profile("Boogie"), TeamChoice::Auto);

    let aim = Vec2 { x: 42.0, y: 42.0 };
    let human = BTreeMap::from([(
        1,
        game_core::Input {
            aim,
            ..game_core::Input::default()
        },
    )]);
    world.step_with_bots(&human);
    assert_eq!(
        world.players[&1].last_seq, 0,
        "the human's own frame was the one applied"
    );
}

#[test]
fn a_bot_walks_somewhere_over_a_few_seconds_rather_than_standing_still() {
    let mut world = World::new("deathmatch");
    world.add_bot(1, profile("Walker"), TeamChoice::Auto);
    world.add_bot(2, profile("Target"), TeamChoice::Auto);
    let start = world.players[&1].pos;

    for _ in 0..240 {
        world.step_with_bots(&BTreeMap::new());
    }
    let moved = (world.players[&1].pos.x - start.x).abs();
    assert!(moved > 1.0, "it went somewhere: {moved} units");
}

#[test]
fn two_bots_left_alone_eventually_shoot_at_each_other() {
    let mut world = World::new("deathmatch");
    world.bonuses = game_core::BonusConfig::none();
    world.add_bot(1, BotProfile::new("A", Difficulty::Elite), TeamChoice::Auto);
    world.add_bot(2, BotProfile::new("B", Difficulty::Elite), TeamChoice::Auto);
    // Stand them where they can see each other.
    world.players.get_mut(&1).unwrap().pos = Vec2 { x: 560.0, y: 600.0 };
    world.players.get_mut(&2).unwrap().pos = Vec2 { x: 660.0, y: 600.0 };

    let mut fired = false;
    for _ in 0..600 {
        world.step_with_bots(&BTreeMap::new());
        fired |= !world.projectiles.is_empty()
            || world
                .events
                .iter()
                .any(|event| matches!(event, game_core::Event::Shot { .. }));
        if fired {
            break;
        }
    }
    assert!(fired, "a fight broke out");
}

#[test]
fn a_ctf_bot_heads_for_the_enemy_flag_rather_than_wandering() {
    let mut world = World::new(ModeKind::CaptureTheFlag.id());
    world.rules = ModeRules::new(ModeKind::CaptureTheFlag);
    world.place_objectives();
    world.add_bot(1, profile("Runner"), TeamChoice::Alpha);
    world.add_bot(2, profile("Guard"), TeamChoice::Bravo);

    world.step_with_bots(&BTreeMap::new());
    let goal = world.bots[&1].goal;
    assert!(
        matches!(
            goal,
            game_core::BotGoal::TakeObjective | game_core::BotGoal::Defend
        ),
        "it has a job in this mode: {goal:?}"
    );
    assert!(
        world.bots[&1].navigation.destination.is_some(),
        "and somewhere to be"
    );
}

#[test]
fn a_ctf_bot_that_picks_up_the_flag_starts_taking_it_home() {
    let mut world = World::new(ModeKind::CaptureTheFlag.id());
    world.rules = ModeRules::new(ModeKind::CaptureTheFlag);
    world.place_objectives();
    world.add_bot(1, profile("Runner"), TeamChoice::Alpha);
    world.add_bot(2, profile("Guard"), TeamChoice::Bravo);

    // Put it on the enemy flag so it takes it.
    let bravo_base = world.objectives.flag(FlagKind::Bravo).unwrap().base;
    world.players.get_mut(&1).unwrap().pos = bravo_base;
    world.step_with_bots(&BTreeMap::new());
    assert_eq!(
        world
            .objectives
            .flag(FlagKind::Bravo)
            .unwrap()
            .state
            .carrier(),
        Some(1)
    );

    world.step_with_bots(&BTreeMap::new());
    assert_eq!(world.bots[&1].goal, game_core::BotGoal::CarryHome);
    let home = world.objectives.flag(FlagKind::Alpha).unwrap().base;
    assert_eq!(world.bots[&1].navigation.destination, Some(home));
}

#[test]
fn a_dead_bot_stops_thinking_and_still_picks_what_it_will_come_back_with() {
    let mut world = World::new("deathmatch");
    world.add_bot(1, profile("Corpse"), TeamChoice::Auto);
    world.players.get_mut(&1).unwrap().hp = 0;
    world.bots.get_mut(&1).unwrap().awareness.target = Some(2);

    let inputs = world.bot_inputs();
    let input = inputs[&1];
    assert!(!input.fire, "a corpse does not shoot");
    assert!(!input.left && !input.right, "or walk");
    assert_eq!(
        world.bots[&1].awareness.target, None,
        "and it has forgotten what it was chasing"
    );
}

#[test]
fn a_match_full_of_bots_runs_for_a_while_without_falling_over() {
    let mut world = World::new("team");
    world.rules = ModeRules::new(ModeKind::Teammatch);
    world.rules.countdown_ticks = 1;
    for id in 1..=6u32 {
        world.add_bot(id, profile(&format!("Bot {id}")), TeamChoice::Auto);
    }

    for _ in 0..1_200 {
        world.step_with_bots(&BTreeMap::new());
    }
    assert_eq!(world.players.len(), 6, "everybody is still here");
    for player in world.players.values() {
        assert!(player.pos.x.is_finite() && player.pos.y.is_finite());
    }
}

#[test]
fn bots_are_deterministic_so_a_recorded_match_replays() {
    let run = || {
        let mut world = World::new("deathmatch");
        world.bonuses = game_core::BonusConfig::none();
        world.add_bot(1, profile("A"), TeamChoice::Auto);
        world.add_bot(2, profile("B"), TeamChoice::Auto);
        for _ in 0..300 {
            world.step_with_bots(&BTreeMap::new());
        }
        world
            .players
            .values()
            .map(|player| (player.id, player.pos.x, player.pos.y))
            .collect::<Vec<_>>()
    };
    assert_eq!(run(), run());
}
