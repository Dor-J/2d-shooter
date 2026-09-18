//! Two-slot inventory: primary+secondary or primary+primary.

use serde::{Deserialize, Serialize};

use super::{WeaponKind, WeaponTable};
use crate::{CharacterState, Direction, Vec2};

/// Ticks between asking for a carried weapon and actually holding it.
pub const SWITCH_DELAY_TICKS: u16 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponSlot {
    pub kind: WeaponKind,
    pub ammo: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: [Option<WeaponSlot>; 2],
    pub active: u8,
}

impl Inventory {
    /// The loadout a player spawns with when only some primaries are available to them.
    ///
    /// Advance mode locks most of the armoury away, and the spawn loadout has to respect that or
    /// the gate on weapon selection means nothing: a player would simply be handed the weapon they
    /// had not earned. A locked primary leaves that slot empty rather than substituting one,
    /// because working your way back up to a primary is the mode.
    pub fn spawn_limited(
        selected: u8,
        table: &WeaponTable,
        is_available: impl Fn(WeaponKind) -> bool,
    ) -> Self {
        let mut inventory = Self::spawn(selected, table);
        for slot in inventory.slots.iter_mut() {
            if let Some(held) = slot {
                if !is_available(held.kind) {
                    *slot = None;
                }
            }
        }
        // Whatever is left becomes what they are holding, so they never spawn with empty hands
        // when they do in fact own something.
        if inventory.active().is_none() {
            inventory.active = inventory
                .slots
                .iter()
                .position(Option::is_some)
                .and_then(|index| u8::try_from(index).ok())
                .unwrap_or(0);
        }
        inventory
    }

    /// Default loadout, or the selected primary/secondary plus its partner.
    pub fn spawn(selected: u8, table: &WeaponTable) -> Self {
        let kind = WeaponKind::from_slot(selected);
        let slot = |kind: WeaponKind| WeaponSlot {
            kind,
            ammo: u16::from(table.get(kind).ammo),
        };
        if kind.is_primary() {
            Self {
                slots: [Some(slot(kind)), Some(slot(WeaponKind::Ussocom))],
                active: 0,
            }
        } else if kind.is_secondary() {
            Self {
                slots: [Some(slot(WeaponKind::DesertEagles)), Some(slot(kind))],
                active: 1,
            }
        } else {
            Self {
                slots: [
                    Some(slot(WeaponKind::DesertEagles)),
                    Some(slot(WeaponKind::Ussocom)),
                ],
                active: 0,
            }
        }
    }

    pub fn active(&self) -> Option<WeaponSlot> {
        self.slots.get(usize::from(self.active)).copied().flatten()
    }

    pub fn active_mut(&mut self) -> Option<&mut WeaponSlot> {
        self.slots.get_mut(usize::from(self.active))?.as_mut()
    }

    pub fn slot_of(&self, kind: WeaponKind) -> Option<u8> {
        self.slots
            .iter()
            .position(|slot| slot.is_some_and(|held| held.kind == kind))
            .and_then(|index| u8::try_from(index).ok())
    }

    pub fn owns(&self, kind: WeaponKind) -> bool {
        self.slot_of(kind).is_some()
    }

    /// Put `kind` in the matching slot and select it. Tests use this instead of the 14-magazine tray.
    pub fn equip(&mut self, kind: WeaponKind, ammo: u16) {
        let slot = WeaponSlot { kind, ammo };
        if kind.is_primary() {
            self.slots[0] = Some(slot);
            self.active = 0;
        } else if kind.is_secondary() {
            self.slots[1] = Some(slot);
            self.active = 1;
        }
    }

    pub fn drop_active(&mut self) -> Option<WeaponSlot> {
        let index = usize::from(self.active);
        let dropped = self.slots[index].take()?;
        let other = 1 - index;
        if self.slots[other].is_some() {
            self.active = other as u8;
        }
        Some(dropped)
    }

    pub fn take_kind(&mut self, kind: WeaponKind) -> Option<WeaponSlot> {
        let index = usize::from(self.slot_of(kind)?);
        let taken = self.slots[index].take()?;
        if usize::from(self.active) == index && self.slots[1 - index].is_some() {
            self.active = (1 - index) as u8;
        }
        Some(taken)
    }

    pub fn take_all(&mut self) -> Vec<WeaponSlot> {
        let slots = self.slots.iter_mut().filter_map(Option::take).collect();
        self.active = 0;
        slots
    }

    pub fn empty_index(&self) -> Option<usize> {
        self.slots.iter().position(Option::is_none)
    }

    /// Fill an empty slot, or replace the active weapon. Returns whatever was displaced.
    pub fn pickup(&mut self, slot: WeaponSlot) -> Option<WeaponSlot> {
        if let Some(index) = self.empty_index() {
            self.slots[index] = Some(slot);
            if self.active().is_none() {
                self.active = index as u8;
            }
            None
        } else {
            self.slots[usize::from(self.active)].replace(slot)
        }
    }
}

/// Barrel position from pose and aim, not the body center.
pub fn muzzle_origin(pos: Vec2, aim: Vec2, state: CharacterState, facing: Direction) -> Vec2 {
    let height = match state {
        CharacterState::Crouching => 6.0,
        CharacterState::Prone
        | CharacterState::GoingProne { .. }
        | CharacterState::GettingUp { .. } => 2.0,
        CharacterState::Rolling { .. } | CharacterState::Backflip { .. } => 4.0,
        _ => 10.0,
    };
    let dx = aim.x - pos.x;
    let dy = aim.y - pos.y;
    let len = (dx * dx + dy * dy).sqrt();
    let (dir_x, dir_y) = if len > 1.0 && len.is_finite() {
        (dx / len, dy / len)
    } else {
        (facing.sign(), 0.0)
    };
    Vec2 {
        x: pos.x + dir_x * 14.0,
        y: pos.y - height + dir_y * 6.0,
    }
}
