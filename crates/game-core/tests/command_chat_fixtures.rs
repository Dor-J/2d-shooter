//! Player commands and chat visibility.
//!
//! Acceptance evidence:
//!   rust:commands:parse
//!   rust:commands:kill
//!   rust:commands:emote
//!   rust:chat:scope
//!   rust:chat:visibility
//!   rust:chat:flood
//!   rust:chat:censor

use game_core::{
    censor, may_see_chat, parse_chat, parse_player_command, ChatLine, ChatScope, CommandError,
    Flood, PlayerCommand, World, ALPHA, BRAVO,
};

#[test]
fn a_slash_line_is_a_command_and_anything_else_is_chat() {
    assert_eq!(parse_player_command("/KILL").unwrap(), PlayerCommand::Kill);
    assert_eq!(
        parse_player_command("  /brutalKill  ").unwrap(),
        PlayerCommand::BrutalKill
    );
    assert_eq!(
        parse_player_command("/victory").unwrap(),
        PlayerCommand::Victory
    );
    assert_eq!(
        parse_player_command("hello"),
        Err(CommandError::NotACommand)
    );
    assert_eq!(parse_player_command("/NOPE"), Err(CommandError::Unknown));
}

#[test]
fn a_caret_is_team_chat_even_when_the_box_was_set_to_all() {
    let (scope, text) = parse_chat("^ cover me", ChatScope::All);
    assert_eq!(scope, ChatScope::Team);
    assert_eq!(text, "cover me");
}

#[test]
fn kill_drops_the_player_and_an_emote_does_not() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "Ace".into());
    assert!(world.apply_player_command(1, PlayerCommand::Victory));
    assert!(world.players[&1].hp > 0);
    assert!(world.apply_player_command(1, PlayerCommand::Kill));
    assert!(world.players[&1].hp <= 0);
    assert!(
        !world.apply_player_command(1, PlayerCommand::Kill),
        "a corpse cannot die twice"
    );
}

#[test]
fn team_chat_stays_on_its_own_side() {
    let line = ChatLine {
        player: 1,
        name: "Ace".into(),
        text: "go".into(),
        scope: ChatScope::Team,
    };
    assert!(may_see_chat(
        &line, 2, ALPHA, ALPHA, true, true, false, false
    ));
    assert!(!may_see_chat(
        &line, 3, BRAVO, ALPHA, true, true, false, false
    ));
}

#[test]
fn realistic_hides_open_chat_from_the_other_team() {
    let line = ChatLine {
        player: 1,
        name: "Ace".into(),
        text: "hi".into(),
        scope: ChatScope::All,
    };
    assert!(!may_see_chat(
        &line, 2, BRAVO, ALPHA, true, true, true, false
    ));
    assert!(may_see_chat(
        &line, 2, ALPHA, ALPHA, true, true, true, false
    ));
}

#[test]
fn the_dead_in_survival_cannot_talk_to_the_living() {
    let line = ChatLine {
        player: 1,
        name: "Ghost".into(),
        text: "he's behind you".into(),
        scope: ChatScope::All,
    };
    assert!(!may_see_chat(
        &line, 2, ALPHA, ALPHA, true, false, false, true
    ));
    assert!(may_see_chat(
        &line, 3, ALPHA, ALPHA, false, false, false, true
    ));
}

#[test]
fn a_flood_is_cut_off_after_the_burst() {
    let mut flood = Flood::default();
    for _ in 0..4 {
        assert!(flood.allow());
    }
    assert!(!flood.allow());
}

#[test]
fn a_blocked_word_is_blanked_only_when_the_filter_is_on() {
    assert_eq!(censor("oh shit", false), "oh shit");
    assert_eq!(censor("oh shit", true), "oh ***");
}
