use serde::{Deserialize, Serialize};

use crate::{Emote, Vec2};

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Input {
    pub seq: u32,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub jet: bool,
    #[serde(default)]
    pub crouch: bool,
    #[serde(default)]
    pub prone: bool,
    #[serde(default)]
    pub roll: bool,
    #[serde(default)]
    pub emote: Option<Emote>,
    #[serde(default)]
    pub reload: bool,
    pub fire: bool,
    pub throw_grenade: bool,
    pub aim: Vec2,
    pub weapon: u8,
    #[serde(default)]
    pub drop: bool,
    #[serde(default)]
    pub throw_weapon: bool,
    #[serde(default)]
    pub throw_knife: bool,
    #[serde(default)]
    pub pickup: bool,
}
