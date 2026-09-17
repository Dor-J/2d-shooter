use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Left,
    #[default]
    Right,
}

impl Direction {
    pub(crate) fn sign(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollSource {
    Explicit,
    CrouchWhileRunning,
    ProneExit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Emote {
    Cigar,
    Victory,
    Mercy,
    Taunt,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "pose", rename_all = "snake_case")]
pub enum CharacterState {
    #[default]
    Standing,
    Crouching,
    GoingProne {
        ticks_left: u8,
    },
    Prone,
    GettingUp {
        ticks_left: u8,
    },
    Rolling {
        direction: Direction,
        source: RollSource,
        ticks_left: u8,
    },
    Backflip {
        direction: Direction,
        late: bool,
        ticks_left: u8,
    },
    Airborne,
    Dead,
    Emote {
        emote: Emote,
        ticks_left: u16,
    },
}

impl CharacterState {
    pub fn is_low(self) -> bool {
        matches!(
            self,
            Self::Crouching
                | Self::GoingProne { .. }
                | Self::Prone
                | Self::GettingUp { .. }
                | Self::Rolling { .. }
                | Self::Backflip { .. }
        )
    }
}
