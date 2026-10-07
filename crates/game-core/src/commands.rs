//! Player commands typed with a leading `/`.
//!
//! The parser returns a typed request. The world decides whether it is allowed. Nothing here
//! shells out, and an unknown verb is an error rather than a no-op so a typo is visible.

use serde::{Deserialize, Serialize};

use crate::character::Emote;

/// What a player asked the server to do to themselves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerCommand {
    Kill,
    BrutalKill,
    Mercy,
    Smoke,
    Tabac,
    Takeoff,
    Victory,
    Pause,
    Unpause,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    Unknown,
    NotACommand,
}

impl PlayerCommand {
    /// The emote this command plays, when it is an emote rather than a state change.
    pub const fn emote(self) -> Option<Emote> {
        match self {
            Self::Mercy => Some(Emote::Mercy),
            Self::Smoke | Self::Tabac | Self::Takeoff => Some(Emote::Cigar),
            Self::Victory => Some(Emote::Victory),
            Self::Kill | Self::BrutalKill | Self::Pause | Self::Unpause => None,
        }
    }

    pub const fn is_pause(self) -> bool {
        matches!(self, Self::Pause | Self::Unpause)
    }
}

/// Reads one line. A leading `/` is required; anything else is `NotACommand` so chat can share
/// the same text box.
pub fn parse_player_command(line: &str) -> Result<PlayerCommand, CommandError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(CommandError::Empty);
    }
    let rest = trimmed.strip_prefix('/').ok_or(CommandError::NotACommand)?;
    let verb = rest
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_uppercase();
    match verb.as_str() {
        "KILL" => Ok(PlayerCommand::Kill),
        "BRUTALKILL" => Ok(PlayerCommand::BrutalKill),
        "MERCY" => Ok(PlayerCommand::Mercy),
        "SMOKE" => Ok(PlayerCommand::Smoke),
        "TABAC" => Ok(PlayerCommand::Tabac),
        "TAKEOFF" => Ok(PlayerCommand::Takeoff),
        "VICTORY" => Ok(PlayerCommand::Victory),
        "PAUSE" => Ok(PlayerCommand::Pause),
        "UNPAUSE" => Ok(PlayerCommand::Unpause),
        "" => Err(CommandError::Empty),
        _ => Err(CommandError::Unknown),
    }
}

pub(crate) fn apply_player_command(
    world: &mut crate::World,
    player: u32,
    command: PlayerCommand,
) -> bool {
    match command {
        PlayerCommand::Kill | PlayerCommand::BrutalKill => {
            let Some(target) = world.players.get(&player) else {
                return false;
            };
            if target.hp <= 0 {
                return false;
            }
            let amount = if command == PlayerCommand::BrutalKill {
                world.damage.gib_damage.max(target.hp)
            } else {
                target.hp
            };
            world.apply_damage(crate::DamageEvent {
                attacker: Some(player),
                target: player,
                amount,
                region: crate::BodyRegion::Chest,
                cause: crate::DamageCause::Bullet,
                direction: crate::Vec2::default(),
                pre_scaled: true,
            });
            true
        }
        PlayerCommand::Mercy
        | PlayerCommand::Smoke
        | PlayerCommand::Tabac
        | PlayerCommand::Takeoff
        | PlayerCommand::Victory => {
            let Some(emote) = command.emote() else {
                return false;
            };
            let Some(target) = world.players.get_mut(&player) else {
                return false;
            };
            if target.hp <= 0 {
                return false;
            }
            target.state = crate::CharacterState::Emote {
                emote,
                ticks_left: crate::character::emote_ticks(emote),
            };
            true
        }
        PlayerCommand::Pause | PlayerCommand::Unpause => false,
    }
}
