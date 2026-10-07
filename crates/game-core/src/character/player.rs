use serde::{Deserialize, Serialize};

use crate::anim::AnimPlayer;
use crate::map::spawn;
use crate::{
    Attribution, Bleed, CharacterState, DeathCause, Direction, ImpulseSource, Input, Inventory,
    MultiKill, TimedEffect, Unlocked, Vec2, WeaponTable,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub pos: Vec2,
    pub vel: Vec2,
    #[serde(default)]
    pub forces: Vec2,
    pub hp: i32,
    pub fuel: i16,
    pub kills: u32,
    pub deaths: u32,
    pub team: u8,
    pub grounded: bool,
    /// High-spawn canopy. Object id 23 (`OBJECT_PARACHUTE`); one flag, no extra body.
    #[serde(default, skip_serializing_if = "is_false")]
    pub parachute: bool,
    #[serde(default)]
    pub state: CharacterState,
    /// Legs and body playback. Both channels follow the stance clip.
    #[serde(default)]
    pub anim: AnimPlayer,
    #[serde(default)]
    pub facing: Direction,
    #[serde(default)]
    pub previous_input: Input,
    #[serde(default)]
    pub jump_buffer_ticks: u8,
    #[serde(default)]
    pub airborne_ticks: u16,
    #[serde(default)]
    pub last_impact: f32,
    #[serde(default)]
    pub last_impulse: Option<ImpulseSource>,
    pub cooldown: u16,
    pub respawn: u16,
    pub last_seq: u32,
    pub weapon: u8,
    pub ammo: u16,
    pub reload_timer: u16,
    pub startup: u16,
    #[serde(default)]
    pub inventory: Inventory,
    #[serde(default)]
    pub switch_timer: u16,
    #[serde(default)]
    pub throw_charge: u16,
    #[serde(default)]
    pub pending_switch: Option<u8>,
    pub grenades: u8,
    pub grenade_cooldown: u16,
    pub grenade_held: bool,
    /// Ticks the throw key has been held. A cooked grenade is thrown harder.
    #[serde(default)]
    pub grenade_charge: u16,
    /// Cluster grenades in hand, which are thrown in place of frags while any are left.
    #[serde(default)]
    pub cluster_grenades: u8,
    /// Which primaries this player has earned, which only Advance mode ever restricts.
    #[serde(default = "Unlocked::everything")]
    pub unlocked: Unlocked,
    /// The bonus effect this player is under, if any.
    #[serde(default)]
    pub bonus: TimedEffect,
    #[serde(default)]
    pub armor: i32,
    #[serde(default)]
    pub bleed: Option<Bleed>,
    #[serde(default)]
    pub attackers: Vec<Attribution>,
    #[serde(default)]
    pub spawn_protection: u16,
    #[serde(default)]
    pub assists: u32,
    #[serde(default)]
    pub teamkills: u32,
    #[serde(default)]
    pub suicides: u32,
    #[serde(default)]
    pub headshots: u32,
    #[serde(default)]
    pub multi_kill: MultiKill,
    #[serde(default)]
    pub last_death: Option<DeathCause>,
    #[serde(default)]
    pub last_damage_direction: Vec2,
    #[serde(default)]
    pub bink: u16,
    #[serde(default)]
    pub accuracy: f32,
    #[serde(default)]
    pub recoil_aim: f32,
    #[serde(default)]
    pub burst: u16,
}
impl Player {
    pub fn new(id: u32, name: String, team: u8) -> Self {
        let table = WeaponTable::normal();
        Self {
            id,
            name,
            pos: spawn(id, team),
            vel: Vec2::default(),
            forces: Vec2::default(),
            hp: 100,
            fuel: 119,
            kills: 0,
            deaths: 0,
            team,
            grounded: false,
            parachute: false,
            state: CharacterState::Airborne,
            anim: AnimPlayer::default(),
            facing: Direction::Right,
            previous_input: Input::default(),
            jump_buffer_ticks: 0,
            airborne_ticks: 0,
            last_impact: 0.0,
            last_impulse: None,
            cooldown: 0,
            respawn: 0,
            last_seq: 0,
            weapon: 0,
            ammo: u16::from(table.for_slot(0).ammo),
            reload_timer: 0,
            startup: 0,
            inventory: Inventory::spawn(0, &table),
            switch_timer: 0,
            throw_charge: 0,
            pending_switch: None,
            grenades: 2,
            grenade_cooldown: 0,
            grenade_held: false,
            grenade_charge: 0,
            cluster_grenades: 0,
            unlocked: Unlocked::everything(),
            bonus: TimedEffect::default(),
            armor: 0,
            bleed: None,
            attackers: Vec::new(),
            spawn_protection: 0,
            assists: 0,
            teamkills: 0,
            suicides: 0,
            headshots: 0,
            multi_kill: MultiKill::default(),
            last_death: None,
            last_damage_direction: Vec2::default(),
            bink: 0,
            accuracy: 0.0,
            recoil_aim: 0.0,
            burst: 0,
        }
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}
