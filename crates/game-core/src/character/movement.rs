use std::sync::OnceLock;

use super::{CharacterState, Direction, RollSource};
use crate::anim::Channel;
use crate::{Input, Player, Vec2};
use serde::{Deserialize, Serialize};

const RUN_SPEED: f32 = 0.118;
const FLY_SPEED: f32 = 0.03;
const JUMP_SPEED: f32 = 0.66;
const JET_SPEED: f32 = 0.10;
const MAX_VELOCITY: f32 = 11.0;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovementConfig {
    pub tick_rate: u32,
    pub crouch_speed_multiplier: f32,
    pub prone_speed_multiplier: f32,
    pub gravity: f32,
    pub crouch_jet_multiplier: f32,
    pub prone_jet_multiplier: f32,
    pub fuel_capacity: i16,
    pub input_buffer_ticks: u8,
}

impl MovementConfig {
    /// Airborne ticks before a backflip counts as late. Trigger only; the clip sets the length.
    pub const LATE_BACKFLIP_AFTER_TICKS: u16 = 12;

    pub const fn soldat_default() -> Self {
        Self {
            tick_rate: 60,
            crouch_speed_multiplier: 0.60,
            prone_speed_multiplier: 0.25,
            gravity: 0.06,
            crouch_jet_multiplier: 0.85,
            prone_jet_multiplier: 0.60,
            fuel_capacity: 119,
            input_buffer_ticks: 6,
        }
    }

    /// Realistic keeps the same locomotion; the mode changes health and weapons, not these constants.
    pub const fn realistic() -> Self {
        Self::soldat_default()
    }
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self::soldat_default()
    }
}

/// One Euler step from the forces control stored on the previous tick.
pub(crate) fn integrate_body(player: &mut Player) {
    // ponytail: one body, not the Verlet skeleton; upgrade when Sprites.pas constraint pairs are fixtures
    player.forces.y += 1.0f32 * 0.06f32;
    player.vel.x += player.forces.x;
    player.vel.y += player.forces.y;
    player.pos.x += player.vel.x;
    player.pos.y += player.vel.y;
    player.vel.x *= 0.99;
    player.vel.y *= 0.99;
    player.forces = Vec2::default();
    player.vel.x = player.vel.x.clamp(-MAX_VELOCITY, MAX_VELOCITY);
    player.vel.y = player.vel.y.clamp(-MAX_VELOCITY, MAX_VELOCITY);
}

pub fn advance_character(
    player: &mut Player,
    input: Input,
    config: &MovementConfig,
    has_standing_clearance: bool,
    tick: u64,
) {
    if player.hp <= 0 {
        player.state = CharacterState::Dead;
        player.previous_input = input;
        return;
    }
    let jump_pressed = input.jump && !player.previous_input.jump;
    let prone_pressed = input.prone && !player.previous_input.prone;
    let roll_pressed = input.roll && !player.previous_input.roll;
    if jump_pressed {
        player.jump_buffer_ticks = config.input_buffer_ticks;
    } else {
        player.jump_buffer_ticks = player.jump_buffer_ticks.saturating_sub(1);
    }
    if input.left ^ input.right {
        player.facing = if input.left {
            Direction::Left
        } else {
            Direction::Right
        };
    }

    player.state = next_state(
        player,
        input,
        prone_pressed,
        roll_pressed,
        has_standing_clearance,
    );
    let jump_started = player.jump_buffer_ticks > 0 && player.grounded && !player.state.is_low();
    if jump_started {
        player.state = CharacterState::Airborne;
        player.jump_buffer_ticks = 0;
    }
    if !matches!(player.state, CharacterState::Emote { .. }) {
        adopt_stance(player, input, jump_started);
    }
    player.anim.advance();
    write_forces(player, input, config, tick);
    player.airborne_ticks = if player.grounded {
        0
    } else {
        player.airborne_ticks.saturating_add(1)
    };
    player.previous_input = input;
}

fn write_forces(player: &mut Player, input: Input, config: &MovementConfig, tick: u64) {
    player.forces = Vec2::default();
    if input.jet && player.fuel > 0 {
        player.fuel -= 1;
        if player.grounded {
            player.forces.y = -2.5 * JET_SPEED;
        } else if player.state == CharacterState::Prone {
            player.forces.x += player.facing.sign() * JET_SPEED / 2.0;
        } else {
            player.forces.y -= JET_SPEED;
        }
    } else if !input.jet && player.fuel < config.fuel_capacity && (player.grounded || tick % 2 == 0)
    {
        player.fuel += 1;
    }

    let axis = f32::from((input.right as i8) - (input.left as i8));
    if let CharacterState::Rolling { direction, .. } = player.state {
        let speed = if player.grounded {
            RUN_SPEED / 1.2
        } else {
            2.0 * FLY_SPEED
        };
        player.forces.x = direction.sign() * speed;
    } else if let CharacterState::Backflip { direction, .. } = player.state {
        let speed = if player.grounded {
            RUN_SPEED / 1.2
        } else {
            2.0 * FLY_SPEED
        };
        player.forces.x = -direction.sign() * speed;
    } else if player.grounded && axis != 0.0 && player.state == CharacterState::Crouching {
        player.forces.x = axis * (RUN_SPEED / 0.6);
    } else if player.grounded && axis != 0.0 && player.state == CharacterState::Prone {
        player.forces.x = axis * (RUN_SPEED * 4.0);
    } else if player.grounded && axis != 0.0 && player.state == CharacterState::Standing {
        player.forces.x = axis * RUN_SPEED;
        player.forces.y = -(RUN_SPEED / 6.0);
    } else if !player.grounded
        && axis != 0.0
        && !matches!(
            player.state,
            CharacterState::Prone | CharacterState::GoingProne
        )
    {
        // ponytail: strafe stays FLYSPEED; JumpSide frame>3 && frame<11 impulse waits on a later pass
        player.forces.x = axis * FLY_SPEED;
    }

    let jump = catalog().jump;
    if player.anim.legs.id == jump.id && player.anim.legs.frame > 8 && player.anim.legs.frame < 15 {
        player.forces.y = -JUMP_SPEED;
        player.grounded = false;
        player.state = CharacterState::Airborne;
    }
}

fn next_state(
    player: &Player,
    input: Input,
    prone_pressed: bool,
    roll_pressed: bool,
    clearance: bool,
) -> CharacterState {
    use CharacterState::*;
    let clips = catalog();
    match player.state {
        Dead => Dead,
        GoingProne if !clip_done(player, clips.prone.id) => GoingProne,
        GoingProne => Prone,
        GettingUp if !clip_done(player, clips.get_up.id) => GettingUp,
        GettingUp => {
            if clearance {
                Standing
            } else {
                Crouching
            }
        }
        Rolling { direction, source } if !clip_done(player, clips.roll.id) => {
            Rolling { direction, source }
        }
        Rolling { .. } => {
            if input.crouch {
                Crouching
            } else {
                Standing
            }
        }
        Backflip { direction, late } if !clip_done(player, clips.roll_back.id) => {
            Backflip { direction, late }
        }
        Backflip { .. } => Airborne,
        Emote { emote, ticks_left } if ticks_left > 1 => Emote {
            emote,
            ticks_left: ticks_left - 1,
        },
        Emote { .. } => {
            if player.grounded {
                Standing
            } else {
                Airborne
            }
        }
        Prone if roll_pressed && (input.left || input.right) && player.anim.legs.frame >= 24 => {
            Rolling {
                direction: if input.left {
                    Direction::Left
                } else {
                    Direction::Right
                },
                source: RollSource::ProneExit,
            }
        }
        Prone if prone_pressed => GettingUp,
        Prone => Prone,
        _ if input.emote.is_some() => Emote {
            emote: input.emote.unwrap_or(super::Emote::Taunt),
            ticks_left: 90,
        },
        _ if roll_pressed && !player.grounded => Backflip {
            direction: player.facing.opposite(),
            late: player.airborne_ticks >= MovementConfig::LATE_BACKFLIP_AFTER_TICKS,
        },
        _ if roll_pressed && player.grounded => Rolling {
            direction: player.facing,
            source: RollSource::Explicit,
        },
        _ if prone_pressed && player.grounded => GoingProne,
        _ if jump_active(player) => Airborne,
        _ if input.crouch && player.grounded => Crouching,
        Crouching if !clearance => Crouching,
        _ if player.grounded => Standing,
        _ => Airborne,
    }
}

fn clip_done(player: &Player, id: u32) -> bool {
    player.anim.legs.id == id && player.anim.legs.finished()
}

fn jump_active(player: &Player) -> bool {
    let jump = catalog().jump;
    player.anim.legs.id == jump.id && !player.anim.legs.finished()
}

fn adopt_stance(player: &mut Player, input: Input, jump_started: bool) {
    let clip = stance_clip(player, input, jump_started);
    if player
        .anim
        .legs
        .is_clip(clip.id, clip.frames, clip.speed, clip.looping)
        && player
            .anim
            .body
            .is_clip(clip.id, clip.frames, clip.speed, clip.looping)
    {
        return;
    }
    let channel = Channel::start(clip.id, clip.frames, clip.speed, clip.looping);
    player.anim.legs = channel.clone();
    player.anim.body = channel;
}

fn stance_clip(player: &Player, input: Input, jump_started: bool) -> Play {
    let clips = catalog();
    let axis = f32::from((input.right as i8) - (input.left as i8));
    let jump_holding =
        jump_started || (player.anim.legs.id == clips.jump.id && !player.anim.legs.finished());
    match player.state {
        CharacterState::GoingProne => clips.prone,
        CharacterState::GettingUp => clips.get_up,
        CharacterState::Rolling { .. } => clips.roll,
        CharacterState::Backflip { .. } => clips.roll_back,
        CharacterState::Prone if axis != 0.0 => clips.prone_move,
        CharacterState::Prone => clips.prone,
        CharacterState::Crouching if axis != 0.0 => clips.crouch_run,
        CharacterState::Crouching => clips.crouch,
        CharacterState::Airborne if jump_holding => clips.jump,
        CharacterState::Airborne if axis != 0.0 => clips.jump_side,
        CharacterState::Airborne => clips.fall,
        CharacterState::Standing if running_back(player, input, axis) => clips.run_back,
        CharacterState::Standing if axis != 0.0 => clips.run,
        CharacterState::Standing => clips.stand,
        CharacterState::Dead | CharacterState::Emote { .. } => clips.stand,
    }
}

fn running_back(player: &Player, input: Input, axis: f32) -> bool {
    if axis == 0.0 || (input.aim.x == 0.0 && input.aim.y == 0.0) {
        return false;
    }
    let dx = input.aim.x - player.pos.x;
    dx.abs() > 1.0 && axis.signum() != dx.signum()
}

#[derive(Clone, Copy)]
struct Play {
    id: u32,
    frames: u32,
    speed: u32,
    looping: bool,
}

struct Catalog {
    stand: Play,
    run: Play,
    run_back: Play,
    jump: Play,
    jump_side: Play,
    fall: Play,
    crouch: Play,
    crouch_run: Play,
    roll: Play,
    roll_back: Play,
    prone: Play,
    prone_move: Play,
    get_up: Play,
}

fn catalog() -> &'static Catalog {
    static CATALOG: OnceLock<Catalog> = OnceLock::new();
    CATALOG.get_or_init(|| Catalog {
        stand: load(include_str!("../../../content/animations/Stand.json")),
        run: load(include_str!("../../../content/animations/Run.json")),
        run_back: load(include_str!("../../../content/animations/RunBack.json")),
        jump: load(include_str!("../../../content/animations/Jump.json")),
        jump_side: load(include_str!("../../../content/animations/JumpSide.json")),
        fall: load(include_str!("../../../content/animations/Fall.json")),
        crouch: load(include_str!("../../../content/animations/Crouch.json")),
        crouch_run: load(include_str!("../../../content/animations/CrouchRun.json")),
        roll: load(include_str!("../../../content/animations/Roll.json")),
        roll_back: load(include_str!("../../../content/animations/RollBack.json")),
        prone: load(include_str!("../../../content/animations/Prone.json")),
        prone_move: load(include_str!("../../../content/animations/ProneMove.json")),
        get_up: load(include_str!("../../../content/animations/GetUp.json")),
    })
}

fn load(bytes: &str) -> Play {
    let clip = content::anim::parse(bytes.as_bytes()).expect("authored stance clip");
    Play {
        id: clip.id,
        frames: clip.frames,
        speed: clip.speed,
        looping: clip.looping,
    }
}

trait Opposite {
    fn opposite(self) -> Self;
}

impl Opposite for Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::catalog;

    fn headers(bytes: &str, id: u32) -> (u32, u32, u32, bool) {
        let clip = content::anim::parse(bytes.as_bytes()).expect("clip");
        assert_eq!(clip.id, id);
        (clip.id, clip.frames, clip.speed, clip.looping)
    }

    fn same(play: super::Play, header: (u32, u32, u32, bool)) {
        assert_eq!((play.id, play.frames, play.speed, play.looping), header);
    }

    #[test]
    fn stance_catalog_reads_authored_clip_headers() {
        let clips = catalog();
        same(
            clips.stand,
            headers(include_str!("../../../content/animations/Stand.json"), 0),
        );
        same(
            clips.run,
            headers(include_str!("../../../content/animations/Run.json"), 1),
        );
        same(
            clips.run_back,
            headers(include_str!("../../../content/animations/RunBack.json"), 2),
        );
        same(
            clips.jump,
            headers(include_str!("../../../content/animations/Jump.json"), 3),
        );
        same(
            clips.jump_side,
            headers(include_str!("../../../content/animations/JumpSide.json"), 4),
        );
        same(
            clips.fall,
            headers(include_str!("../../../content/animations/Fall.json"), 5),
        );
        same(
            clips.crouch,
            headers(include_str!("../../../content/animations/Crouch.json"), 6),
        );
        same(
            clips.crouch_run,
            headers(
                include_str!("../../../content/animations/CrouchRun.json"),
                7,
            ),
        );
        same(
            clips.roll,
            headers(include_str!("../../../content/animations/Roll.json"), 22),
        );
        same(
            clips.roll_back,
            headers(
                include_str!("../../../content/animations/RollBack.json"),
                23,
            ),
        );
        same(
            clips.prone,
            headers(include_str!("../../../content/animations/Prone.json"), 34),
        );
        same(
            clips.prone_move,
            headers(
                include_str!("../../../content/animations/ProneMove.json"),
                38,
            ),
        );
        same(
            clips.get_up,
            headers(include_str!("../../../content/animations/GetUp.json"), 39),
        );
        assert_eq!(clips.roll.frames * clips.roll.speed, 34);
        assert!(!clips.roll.looping);
    }
}
