use game_core::{
    advance_character, apply_impulse, BodyShape, CharacterState, Direction, Emote, Impulse,
    ImpulseSource, Input, MovementConfig, Player, RollSource, Vec2, World, DT,
};

const SOURCE_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";
const SOURCE_CONSTANTS: &str = "shared/Constants.pas:RUNSPEED/RUNSPEEDUP/JUMPSPEED/CROUCHRUNSPEED/PRONESPEED/ROLLSPEED/JETSPEED";
const SOURCE_CONTROL: &str = "shared/mechanics/Control.pas:TSpriteSystem.Control";

fn input() -> Input {
    Input::default()
}

#[test]
fn fixture_metadata_is_pinned() {
    assert_eq!(SOURCE_COMMIT.len(), 40);
    assert!(SOURCE_CONSTANTS.contains("JUMPSPEED"));
    assert!(SOURCE_CONTROL.contains("Control"));
}

#[test]
fn source_constants_are_exposed_at_sixty_hertz() {
    let config = MovementConfig::soldat_default();
    assert_eq!(config.tick_rate, 60);
    assert_eq!(DT, 1.0 / 60.0);
    assert_eq!(config.source_run_speed, 0.118);
    assert_eq!(config.source_jump_speed, 0.66);
    assert_eq!(config.source_jet_speed, 0.10);
}

#[test]
fn acceleration_friction_momentum_and_air_control_are_deterministic() {
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "runner".into(), 0);
    player.grounded = true;
    let mut right = input();
    right.right = true;
    advance_character(&mut player, right, &config, true);
    assert_eq!(player.vel.x, config.ground_acceleration * DT);

    let momentum = player.vel.x;
    advance_character(&mut player, input(), &config, true);
    assert_eq!(player.vel.x, momentum * config.ground_friction);

    player.grounded = false;
    player.vel.x = 0.0;
    advance_character(&mut player, right, &config, true);
    assert_eq!(player.vel.x, config.air_acceleration * DT);
}

#[test]
fn jump_is_edge_buffered_and_transitions_airborne() {
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "jumper".into(), 0);
    player.grounded = false;
    let mut jump = input();
    jump.jump = true;
    advance_character(&mut player, jump, &config, true);
    assert!(player.jump_buffer_ticks > 0);
    player.grounded = true;
    advance_character(&mut player, input(), &config, true);
    assert_eq!(player.state, CharacterState::Airborne);
    assert_eq!(player.vel.y, -config.jump_speed + config.gravity * DT);
}

#[test]
fn stance_transitions_use_deterministic_source_timers() {
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "stances".into(), 0);
    player.grounded = true;
    let mut prone = input();
    prone.prone = true;
    advance_character(&mut player, prone, &config, true);
    assert_eq!(
        player.state,
        CharacterState::GoingProne {
            ticks_left: config.prone_transition_ticks - 1
        }
    );
    for _ in 1..config.prone_transition_ticks {
        advance_character(&mut player, input(), &config, true);
    }
    assert_eq!(player.state, CharacterState::Prone);

    advance_character(&mut player, prone, &config, true);
    assert!(matches!(player.state, CharacterState::GettingUp { .. }));
    for _ in 0..config.get_up_ticks {
        advance_character(&mut player, input(), &config, false);
    }
    assert_eq!(player.state, CharacterState::Crouching);
    advance_character(&mut player, input(), &config, true);
    assert_eq!(player.state, CharacterState::Standing);
}

#[test]
fn roll_direction_source_and_backflip_variants_are_explicit() {
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "roller".into(), 0);
    player.grounded = true;
    let mut roll = input();
    roll.right = true;
    roll.crouch = true;
    roll.roll = true;
    advance_character(&mut player, roll, &config, true);
    assert!(matches!(
        player.state,
        CharacterState::Rolling {
            direction: Direction::Right,
            source: RollSource::Explicit,
            ..
        }
    ));

    player.state = CharacterState::Airborne;
    player.grounded = false;
    player.facing = Direction::Right;
    player.previous_input = input();
    let mut backflip = input();
    backflip.left = true;
    backflip.jump = true;
    backflip.roll = true;
    advance_character(&mut player, backflip, &config, true);
    assert!(matches!(
        player.state,
        CharacterState::Backflip { late: false, .. }
    ));
    player.state = CharacterState::Airborne;
    player.airborne_ticks = config.late_backflip_after_ticks;
    player.previous_input = input();
    advance_character(&mut player, backflip, &config, true);
    assert!(matches!(
        player.state,
        CharacterState::Backflip { late: true, .. }
    ));
}

#[test]
fn prone_can_exit_directly_into_a_directional_roll() {
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "prone roller".into(), 0);
    player.state = CharacterState::Prone;
    player.grounded = true;
    let mut roll = input();
    roll.left = true;
    roll.roll = true;
    advance_character(&mut player, roll, &config, true);
    assert!(matches!(
        player.state,
        CharacterState::Rolling {
            direction: Direction::Left,
            source: RollSource::ProneExit,
            ..
        }
    ));
}

#[test]
fn jet_force_depends_on_pose_and_fuel_recharges_on_ground() {
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "jet".into(), 0);
    player.state = CharacterState::Prone;
    player.grounded = false;
    player.fuel = config.fuel_capacity;
    let mut jet = input();
    jet.jet = true;
    advance_character(&mut player, jet, &config, true);
    let prone_velocity = player.vel.y;
    assert_eq!(
        player.fuel,
        config.fuel_capacity - config.fuel_depletion_per_tick
    );

    player.state = CharacterState::Airborne;
    player.vel.y = 0.0;
    player.fuel = config.fuel_capacity;
    player.previous_input = input();
    advance_character(&mut player, jet, &config, true);
    assert!(player.vel.y < prone_velocity);

    player.grounded = true;
    player.vel.y = 0.0;
    player.fuel = config.fuel_capacity;
    player.previous_input = input();
    advance_character(&mut player, jet, &config, true);
    assert!(player.vel.y < 0.0, "jet lifts off the ground");

    player.grounded = true;
    player.fuel = config.fuel_capacity / 2.0;
    let fuel = player.fuel;
    advance_character(&mut player, input(), &config, true);
    assert_eq!(player.fuel, fuel + config.fuel_recharge_per_tick);
}

#[test]
fn body_volumes_share_head_chest_and_legs_across_poses() {
    for shape in [
        BodyShape::standing(),
        BodyShape::crouching(),
        BodyShape::prone(),
    ] {
        let regions = shape.parts().map(|part| part.region).collect::<Vec<_>>();
        assert_eq!(regions.len(), 3);
    }
    assert!(BodyShape::standing().height() > BodyShape::crouching().height());
    assert!(BodyShape::crouching().height() > BodyShape::prone().height());
}

#[test]
fn all_external_velocity_changes_share_one_impulse_api() {
    let mut player = Player::new(1, "impulses".into(), 0);
    let sources = [
        ImpulseSource::Recoil,
        ImpulseSource::SpasBoost,
        ImpulseSource::MinigunBoost,
        ImpulseSource::Explosion,
        ImpulseSource::Bullet,
        ImpulseSource::Flag,
        ImpulseSource::Kit,
    ];
    for source in sources {
        apply_impulse(
            &mut player,
            Impulse {
                velocity: Vec2 { x: 1.0, y: -1.0 },
                source,
            },
        );
    }
    assert_eq!(player.vel.x, sources.len() as f32);
    assert_eq!(player.last_impulse, Some(ImpulseSource::Kit));

    use game_core::{DynamicBody, DynamicBodyKind};
    let mut flag = DynamicBody::new(DynamicBodyKind::Flag, Vec2::default(), 5.0);
    let mut kit = DynamicBody::new(DynamicBodyKind::Kit, Vec2::default(), 5.0);
    apply_impulse(
        &mut flag,
        Impulse {
            velocity: Vec2 { x: 3.0, y: 0.0 },
            source: ImpulseSource::Flag,
        },
    );
    apply_impulse(
        &mut kit,
        Impulse {
            velocity: Vec2 { x: 0.0, y: -3.0 },
            source: ImpulseSource::Kit,
        },
    );
    assert_eq!(flag.vel.x, 3.0);
    assert_eq!(kit.vel.y, -3.0);
}

#[test]
fn emotes_and_death_are_controller_states() {
    let config = MovementConfig::soldat_default();
    let mut player = Player::new(1, "state".into(), 0);
    let mut emote = input();
    emote.emote = Some(Emote::Victory);
    advance_character(&mut player, emote, &config, true);
    assert!(matches!(player.state, CharacterState::Emote { .. }));
    player.hp = 0;
    advance_character(&mut player, input(), &config, true);
    assert_eq!(player.state, CharacterState::Dead);
}

#[test]
fn fall_impact_is_recorded_and_slopes_preserve_tangential_momentum() {
    use game_core::{resolve_material_velocity, PolygonKind};
    let mut world = World::new("dm");
    world.add_player(1, "slider".into());
    let player = world.players.get_mut(&1).expect("player");
    player.pos = Vec2 { x: 600.0, y: 610.0 };
    player.vel = Vec2 { x: 120.0, y: 300.0 };
    for _ in 0..6 {
        world.step(&Default::default());
    }
    let player = &world.players[&1];
    assert!(player.last_impact > 0.0);
    let slope = resolve_material_velocity(
        Vec2 { x: 120.0, y: 300.0 },
        Vec2 {
            x: -0.447_213_6,
            y: -0.894_427_2,
        },
        PolygonKind::Normal,
    );
    assert!(slope.velocity.x.abs() > 0.0);
    assert!(slope.velocity.y.abs() > 0.0);
}

#[test]
fn player_contact_is_symmetric_and_deterministic() {
    use game_core::resolve_player_contact;
    let mut left = Player::new(1, "left".into(), 0);
    let mut right = Player::new(2, "right".into(), 0);
    left.pos = Vec2 { x: 100.0, y: 100.0 };
    right.pos = Vec2 { x: 110.0, y: 100.0 };
    left.vel.x = 20.0;
    right.vel.x = -20.0;
    assert!(resolve_player_contact(&mut left, &mut right));
    assert_eq!(right.pos.x - left.pos.x, 20.0);
    assert_eq!(left.vel.x, 0.0);
    assert_eq!(right.vel.x, 0.0);
}

#[test]
fn a_crouched_player_on_open_ground_stands_again_when_the_input_is_released() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "stance".into());
    for _ in 0..180 {
        world.step(&Default::default());
    }
    assert!(world.players[&1].grounded, "the player should have landed");

    let crouch = Input {
        crouch: true,
        ..Input::default()
    };
    for _ in 0..5 {
        world.step(&std::collections::BTreeMap::from([(1, crouch)]));
    }
    assert_eq!(world.players[&1].state, CharacterState::Crouching);

    for _ in 0..5 {
        world.step(&Default::default());
    }
    assert_eq!(world.players[&1].state, CharacterState::Standing);
}

#[test]
fn getting_up_from_prone_completes_on_open_ground() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "stance".into());
    for _ in 0..180 {
        world.step(&Default::default());
    }
    let prone = Input {
        prone: true,
        ..Input::default()
    };
    world.step(&std::collections::BTreeMap::from([(1, prone)]));
    for _ in 0..MovementConfig::soldat_default().prone_transition_ticks + 2 {
        world.step(&Default::default());
    }
    assert_eq!(world.players[&1].state, CharacterState::Prone);

    world.step(&std::collections::BTreeMap::from([(1, prone)]));
    for _ in 0..MovementConfig::soldat_default().get_up_ticks + 2 {
        world.step(&Default::default());
    }
    assert_eq!(world.players[&1].state, CharacterState::Standing);
}

#[test]
fn every_pose_keeps_its_feet_at_one_height_so_a_stance_change_never_lifts_the_body() {
    let feet = |shape: &BodyShape| {
        shape
            .parts()
            .map(|part| part.offset.y + part.radius)
            .fold(f32::NEG_INFINITY, f32::max)
    };
    let standing = feet(&BodyShape::standing());
    assert_eq!(feet(&BodyShape::crouching()), standing);
    assert_eq!(feet(&BodyShape::prone()), standing);
}
