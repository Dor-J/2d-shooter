use content::anim::parse;
use game_core::{advance_character, CharacterState, Emote, Input, MovementConfig, Player, World};
use std::collections::BTreeMap;

fn authored(bytes: &str) -> content::anim::Clip {
    parse(bytes.as_bytes()).expect("authored clip")
}

fn step(world: &mut World, input: Input) {
    world.step(&BTreeMap::from([(1, input)]));
}

#[test]
fn reload_plays_clip_out_or_reload_and_returns_when_the_clip_finishes() {
    let clip_out = authored(include_str!("../../content/animations/ClipOut.json"));
    let reload = authored(include_str!("../../content/animations/Reload.json"));
    let mut world = World::new("deathmatch");
    world.add_player(1, "reloader".into());
    world.players.get_mut(&1).unwrap().ammo = 3;
    step(
        &mut world,
        Input {
            reload: true,
            ..Input::default()
        },
    );
    let played = world.players[&1].anim.body.id;
    let clip = if played == clip_out.id {
        clip_out
    } else if played == reload.id {
        reload
    } else {
        panic!("reload played {played}, expected ClipOut or Reload");
    };
    let ticks = clip.frames * clip.speed;
    for _ in 1..ticks {
        step(&mut world, Input::default());
        assert_eq!(world.players[&1].anim.body.id, played);
    }
    step(&mut world, Input::default());
    let player = &world.players[&1];
    assert_eq!(player.anim.body.id, player.anim.legs.id);
    assert_ne!(player.anim.body.id, played);
}

#[test]
fn emote_lasts_the_clip_instead_of_ninety_ticks() {
    let victory = authored(include_str!("../../content/animations/Victory.json"));
    let ticks = victory.frames * victory.speed;
    assert_ne!(ticks, 90, "victory is not the old fixed emote timer");
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "emote".into(), 0);
    player.grounded = true;
    let emote = Input {
        emote: Some(Emote::Victory),
        ..Input::default()
    };
    advance_character(&mut player, emote, &config, true, 0);
    assert!(matches!(player.state, CharacterState::Emote { .. }));
    assert_eq!(player.anim.body.id, victory.id);
    for _ in 1..ticks {
        advance_character(&mut player, Input::default(), &config, true, 0);
        assert!(
            matches!(player.state, CharacterState::Emote { .. }),
            "emote ended before {ticks} ticks"
        );
        assert_eq!(player.anim.body.id, victory.id);
    }
    advance_character(&mut player, Input::default(), &config, true, 0);
    assert!(!matches!(player.state, CharacterState::Emote { .. }));
    assert_eq!(player.anim.body.id, player.anim.legs.id);
    assert_ne!(player.anim.body.id, victory.id);
}
