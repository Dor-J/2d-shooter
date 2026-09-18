use super::jet::force_multiplier;
use super::{CharacterState, Direction, RollSource};
use crate::{Input, Player, DT};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MovementConfig {
    pub tick_rate: u32,
    pub source_run_speed: f32,
    pub source_jump_speed: f32,
    pub source_jet_speed: f32,
    pub ground_acceleration: f32,
    pub air_acceleration: f32,
    pub max_run_speed: f32,
    pub crouch_speed_multiplier: f32,
    pub prone_speed_multiplier: f32,
    pub ground_friction: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub terminal_velocity: f32,
    pub jet_acceleration: f32,
    pub max_jet_speed: f32,
    pub crouch_jet_multiplier: f32,
    pub prone_jet_multiplier: f32,
    pub fuel_capacity: f32,
    pub fuel_depletion_per_tick: f32,
    pub fuel_recharge_per_tick: f32,
    pub input_buffer_ticks: u8,
    pub prone_transition_ticks: u8,
    pub get_up_ticks: u8,
    pub roll_ticks: u8,
    pub backflip_ticks: u8,
    pub late_backflip_after_ticks: u16,
    pub roll_speed: f32,
    pub backflip_speed: f32,
}

impl MovementConfig {
    pub const fn soldat_default() -> Self {
        Self {
            tick_rate: 60,
            source_run_speed: 0.118,
            source_jump_speed: 0.66,
            source_jet_speed: 0.10,
            ground_acceleration: 1050.0,
            air_acceleration: 350.0,
            max_run_speed: 270.0,
            crouch_speed_multiplier: 0.60,
            prone_speed_multiplier: 0.25,
            ground_friction: 0.82,
            jump_speed: 390.0,
            gravity: 950.0,
            terminal_velocity: 600.0,
            jet_acceleration: 1400.0,
            max_jet_speed: 300.0,
            crouch_jet_multiplier: 0.85,
            prone_jet_multiplier: 0.60,
            fuel_capacity: 1.0,
            fuel_depletion_per_tick: 0.45 / 60.0,
            fuel_recharge_per_tick: 0.5 / 60.0,
            input_buffer_ticks: 6,
            prone_transition_ticks: 26,
            get_up_ticks: 16,
            roll_ticks: 15,
            backflip_ticks: 15,
            late_backflip_after_ticks: 12,
            roll_speed: 310.0,
            backflip_speed: 260.0,
        }
    }

    /// Realistic is the same locomotion, just heavier on the feet.
    pub const fn realistic() -> Self {
        let mut config = Self::soldat_default();
        config.max_run_speed = 230.0;
        config.jump_speed = 350.0;
        config.air_acceleration = 280.0;
        config
    }
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self::soldat_default()
    }
}

pub fn advance_character(
    player: &mut Player,
    input: Input,
    config: &MovementConfig,
    has_standing_clearance: bool,
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

    let axis = (input.right as i8 - input.left as i8) as f32;
    let speed_scale = match player.state {
        CharacterState::Prone | CharacterState::GoingProne { .. } => config.prone_speed_multiplier,
        CharacterState::Crouching | CharacterState::GettingUp { .. } => {
            config.crouch_speed_multiplier
        }
        _ => 1.0,
    };
    if let CharacterState::Rolling { direction, .. } = player.state {
        player.vel.x = direction.sign() * config.roll_speed;
    } else if let CharacterState::Backflip { direction, .. } = player.state {
        player.vel.x = direction.sign() * config.backflip_speed;
    } else if axis != 0.0 {
        let acceleration = if player.grounded {
            config.ground_acceleration
        } else {
            config.air_acceleration
        };
        player.vel.x = (player.vel.x + axis * acceleration * speed_scale * DT).clamp(
            -config.max_run_speed * speed_scale,
            config.max_run_speed * speed_scale,
        );
    } else if player.grounded {
        player.vel.x *= config.ground_friction;
    }

    if player.jump_buffer_ticks > 0 && player.grounded && !player.state.is_low() {
        player.vel.y = -config.jump_speed;
        player.grounded = false;
        player.state = CharacterState::Airborne;
        player.jump_buffer_ticks = 0;
    }
    if input.jet && player.fuel > 0.0 {
        player.vel.y = (player.vel.y
            - config.jet_acceleration * force_multiplier(player.state, config) * DT)
            .max(-config.max_jet_speed);
        player.fuel = (player.fuel - config.fuel_depletion_per_tick).max(0.0);
    } else if player.grounded {
        player.fuel = (player.fuel + config.fuel_recharge_per_tick).min(config.fuel_capacity);
    }
    player.vel.y = (player.vel.y + config.gravity * DT).min(config.terminal_velocity);
    player.airborne_ticks = if player.grounded {
        0
    } else {
        player.airborne_ticks.saturating_add(1)
    };
    player.previous_input = input;
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
