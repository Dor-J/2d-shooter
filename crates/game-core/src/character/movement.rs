use super::{CharacterState, Direction, RollSource};
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
    pub prone_transition_ticks: u8,
    pub get_up_ticks: u8,
    pub roll_ticks: u8,
    pub backflip_ticks: u8,
    pub late_backflip_after_ticks: u16,
}

impl MovementConfig {
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
            prone_transition_ticks: 26,
            get_up_ticks: 16,
            roll_ticks: 15,
            backflip_ticks: 15,
            late_backflip_after_ticks: 12,
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
        config,
    );
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
            CharacterState::Prone | CharacterState::GoingProne { .. }
        )
    {
        // ponytail: no JumpSide frame window until MOV-02
        player.forces.x = axis * FLY_SPEED;
    }

    if player.jump_buffer_ticks > 0 && player.grounded && !player.state.is_low() {
        // ponytail: one-tick jump stands in for Jump frames 9..14; MOV-02 replaces it
        player.forces.y = -JUMP_SPEED;
        player.grounded = false;
        player.state = CharacterState::Airborne;
        player.jump_buffer_ticks = 0;
    }
}

fn next_state(
    player: &Player,
    input: Input,
    prone_pressed: bool,
    roll_pressed: bool,
    clearance: bool,
    config: &MovementConfig,
) -> CharacterState {
    use CharacterState::*;
    match player.state {
        Dead => Dead,
        GoingProne { ticks_left } if ticks_left > 1 => GoingProne {
            ticks_left: ticks_left - 1,
        },
        GoingProne { .. } => Prone,
        GettingUp { ticks_left } if ticks_left > 1 => GettingUp {
            ticks_left: ticks_left - 1,
        },
        GettingUp { .. } => {
            if clearance {
                Standing
            } else {
                Crouching
            }
        }
        Rolling {
            direction,
            source,
            ticks_left,
        } if ticks_left > 1 => Rolling {
            direction,
            source,
            ticks_left: ticks_left - 1,
        },
        Rolling { .. } => {
            if input.crouch {
                Crouching
            } else {
                Standing
            }
        }
        Backflip {
            direction,
            late,
            ticks_left,
        } if ticks_left > 1 => Backflip {
            direction,
            late,
            ticks_left: ticks_left - 1,
        },
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
        Prone if roll_pressed && (input.left || input.right) => Rolling {
            direction: if input.left {
                Direction::Left
            } else {
                Direction::Right
            },
            source: RollSource::ProneExit,
            ticks_left: config.roll_ticks,
        },
        Prone if prone_pressed => GettingUp {
            ticks_left: config.get_up_ticks,
        },
        Prone => Prone,
        _ if input.emote.is_some() => Emote {
            emote: input.emote.unwrap_or(super::Emote::Taunt),
            ticks_left: 90,
        },
        _ if roll_pressed && !player.grounded => Backflip {
            direction: player.facing.opposite(),
            late: player.airborne_ticks >= config.late_backflip_after_ticks,
            ticks_left: config.backflip_ticks,
        },
        _ if roll_pressed && player.grounded => Rolling {
            direction: player.facing,
            source: RollSource::Explicit,
            ticks_left: config.roll_ticks,
        },
        _ if prone_pressed && player.grounded => GoingProne {
            ticks_left: config.prone_transition_ticks.saturating_sub(1),
        },
        _ if input.crouch && player.grounded => Crouching,
        Crouching if !clearance => Crouching,
        _ if player.grounded => Standing,
        _ => Airborne,
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
