//! What one bot is like: its name, how good it is, and how it prefers to fight.

use serde::{Deserialize, Serialize};

use crate::weapons::WeaponKind;

/// How hard a bot is to play against.
///
/// Difficulty is one number that everything else is derived from, so a server operator sets one
/// thing rather than tuning six.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Difficulty {
    /// Shoots late and misses often.
    Rookie,
    #[default]
    Normal,
    Veteran,
    /// Reacts almost instantly and rarely misses.
    Elite,
}

impl Difficulty {
    pub const ALL: [Difficulty; 4] = [Self::Rookie, Self::Normal, Self::Veteran, Self::Elite];

    /// Ticks between seeing a target and acting on it.
    ///
    /// A bot that fires the instant a player rounds a corner is not hard, it is unfair; the
    /// reaction time is what makes a fight readable.
    pub const fn reaction_ticks(self) -> u16 {
        match self {
            Self::Rookie => 42,
            Self::Normal => 24,
            Self::Veteran => 12,
            Self::Elite => 5,
        }
    }

    /// How far a bot's aim wanders, in world units at the target.
    pub const fn aim_error(self) -> f32 {
        match self {
            Self::Rookie => 70.0,
            Self::Normal => 34.0,
            Self::Veteran => 14.0,
            Self::Elite => 4.0,
        }
    }

    /// How far away a bot notices somebody at all.
    pub const fn sight_range(self) -> f32 {
        match self {
            Self::Rookie => 320.0,
            Self::Normal => 420.0,
            Self::Veteran => 520.0,
            Self::Elite => 620.0,
        }
    }

    /// How readily a bot throws a grenade, as one chance in this many ticks when it has a shot.
    pub const fn grenade_reluctance(self) -> u32 {
        match self {
            Self::Rookie => 600,
            Self::Normal => 300,
            Self::Veteran => 180,
            Self::Elite => 120,
        }
    }

    /// The name a server operator types.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Rookie => "rookie",
            Self::Normal => "normal",
            Self::Veteran => "veteran",
            Self::Elite => "elite",
        }
    }

    /// The difficulty a name refers to, defaulting to normal for anything unrecognised.
    pub fn from_id(id: &str) -> Self {
        match id.trim().to_ascii_lowercase().as_str() {
            "rookie" | "easy" => Self::Rookie,
            "veteran" | "hard" => Self::Veteran,
            "elite" | "impossible" => Self::Elite,
            _ => Self::Normal,
        }
    }
}

/// One bot's character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotProfile {
    pub name: String,
    pub difficulty: Difficulty,
    /// The weapon it reaches for when it can.
    pub favourite_weapon: WeaponKind,
    /// How willing it is to chase rather than hold ground, from 0 to 100.
    pub aggression: u8,
    /// Whether it talks.
    pub chatty: bool,
}

/// The longest a bot's name may be.
pub const MAX_NAME_BYTES: usize = 24;

impl BotProfile {
    /// A profile with sensible defaults, whatever it is handed.
    pub fn new(name: &str, difficulty: Difficulty) -> Self {
        Self {
            name: clean_name(name),
            difficulty,
            favourite_weapon: WeaponKind::Ak74,
            aggression: 50,
            chatty: true,
        }
    }

    /// Checks a profile that came from a server's configuration.
    pub fn validate(&self) -> Result<(), ProfileError> {
        if self.name.trim().is_empty() {
            return Err(ProfileError::EmptyName);
        }
        if self.name.len() > MAX_NAME_BYTES {
            return Err(ProfileError::NameTooLong {
                bytes: self.name.len(),
                limit: MAX_NAME_BYTES,
            });
        }
        if self.aggression > 100 {
            return Err(ProfileError::AggressionOutOfRange {
                value: self.aggression,
            });
        }
        if self.favourite_weapon.slot().is_none() {
            return Err(ProfileError::UnselectableWeapon);
        }
        Ok(())
    }

    /// How far this bot's aim wanders, tightened a little by aggression.
    pub fn aim_error(&self) -> f32 {
        self.difficulty.aim_error()
    }
}

/// Trims a name to something a scoreboard can show.
pub fn clean_name(name: &str) -> String {
    let trimmed: String = name
        .chars()
        .filter(|c| !c.is_control())
        .take(MAX_NAME_BYTES)
        .collect();
    let trimmed = trimmed.trim().to_string();
    if trimmed.is_empty() {
        "Bot".to_string()
    } else {
        trimmed
    }
}

/// Why a bot profile cannot be used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileError {
    EmptyName,
    NameTooLong {
        bytes: usize,
        limit: usize,
    },
    AggressionOutOfRange {
        value: u8,
    },
    /// The favourite weapon is one nobody can select.
    UnselectableWeapon,
}

/// The stock bots a server hands out when no profiles are configured.
pub fn default_profiles() -> Vec<BotProfile> {
    [
        ("Boogie Man", Difficulty::Veteran, WeaponKind::Ak74),
        ("Sniper", Difficulty::Veteran, WeaponKind::Barrett),
        ("Rambo", Difficulty::Normal, WeaponKind::Minimi),
        ("Kamikaze", Difficulty::Normal, WeaponKind::Spas12),
        ("Rookie", Difficulty::Rookie, WeaponKind::Mp5),
        ("Blade", Difficulty::Elite, WeaponKind::Ruger77),
    ]
    .into_iter()
    .map(|(name, difficulty, weapon)| BotProfile {
        favourite_weapon: weapon,
        ..BotProfile::new(name, difficulty)
    })
    .collect()
}
