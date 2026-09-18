//! A bounded, allocation-capped reader for `weapons.ini`-style weapon mods.
//!
//! The loader never trusts the file: it caps the byte count, the line count, and the line length
//! before it looks at any value, rejects unknown and duplicate sections and keys instead of
//! silently ignoring them, and range-checks every number so a mod cannot smuggle a NaN, an
//! infinity, or a value that would overflow the simulation.

use std::fmt;

use super::config::{
    BulletStyle, NoCollision, WeaponDef, WeaponKind, WeaponLimits, CONFIGURED_WEAPONS, WEAPON_KEYS,
};
use super::table::WeaponTable;

/// Everything that can go wrong while reading a weapon mod, named precisely enough to fix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponConfigError {
    TooLarge {
        bytes: usize,
        limit: usize,
    },
    TooManyLines {
        lines: usize,
        limit: usize,
    },
    LineTooLong {
        line: usize,
        bytes: usize,
        limit: usize,
    },
    MalformedLine {
        line: usize,
    },
    UnknownSection {
        section: String,
        line: usize,
    },
    DuplicateSection {
        section: String,
        line: usize,
    },
    MissingSection {
        section: String,
    },
    UnknownKey {
        section: String,
        key: String,
        line: usize,
    },
    DuplicateKey {
        section: String,
        key: String,
        line: usize,
    },
    MissingKey {
        section: String,
        key: String,
    },
    InvalidValue {
        section: String,
        key: String,
        line: usize,
    },
    OutOfRange {
        section: String,
        key: String,
        line: usize,
    },
    KeyOutsideSection {
        key: String,
        line: usize,
    },
}

impl fmt::Display for WeaponConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLarge { bytes, limit } => {
                write!(f, "weapon mod is {bytes} bytes, limit is {limit}")
            }
            Self::TooManyLines { lines, limit } => {
                write!(f, "weapon mod has {lines} lines, limit is {limit}")
            }
            Self::LineTooLong { line, bytes, limit } => {
                write!(f, "line {line} is {bytes} bytes, limit is {limit}")
            }
            Self::MalformedLine { line } => write!(f, "line {line} is neither a section nor a key"),
            Self::UnknownSection { section, line } => {
                write!(f, "unknown section [{section}] on line {line}")
            }
            Self::DuplicateSection { section, line } => {
                write!(f, "section [{section}] repeats on line {line}")
            }
            Self::MissingSection { section } => write!(f, "section [{section}] is missing"),
            Self::UnknownKey { section, key, line } => {
                write!(f, "unknown key {key} in [{section}] on line {line}")
            }
            Self::DuplicateKey { section, key, line } => {
                write!(f, "key {key} repeats in [{section}] on line {line}")
            }
            Self::MissingKey { section, key } => write!(f, "key {key} is missing in [{section}]"),
            Self::InvalidValue { section, key, line } => {
                write!(
                    f,
                    "value for {key} in [{section}] on line {line} is not a number"
                )
            }
            Self::OutOfRange { section, key, line } => {
                write!(
                    f,
                    "value for {key} in [{section}] on line {line} is out of range"
                )
            }
            Self::KeyOutsideSection { key, line } => {
                write!(f, "key {key} on line {line} is outside any section")
            }
        }
    }
}

impl std::error::Error for WeaponConfigError {}

/// The `[Info]` keys a mod may set.
const INFO_KEYS: [&str; 2] = ["Name", "Version"];
const MAX_INFO_BYTES: usize = 64;

#[derive(Default)]
struct Section {
    values: Vec<(&'static str, f64, usize)>,
    no_collision: Option<(u8, usize)>,
}

impl Section {
    fn value(&self, key: &str) -> Option<f64> {
        self.values
            .iter()
            .find(|(name, _, _)| *name == key)
            .map(|(_, value, _)| *value)
    }
    fn line(&self, key: &str) -> usize {
        self.values
            .iter()
            .find(|(name, _, _)| *name == key)
            .map_or(0, |(_, _, line)| *line)
    }
}

/// Reads a weapon mod, or explains exactly why it cannot be trusted.
pub fn parse_ini(source: &str, limits: &WeaponLimits) -> Result<WeaponTable, WeaponConfigError> {
    if source.len() > limits.max_bytes {
        return Err(WeaponConfigError::TooLarge {
            bytes: source.len(),
            limit: limits.max_bytes,
        });
    }

    let mut name = String::new();
    let mut version = String::new();
    let mut sections: Vec<(WeaponKind, Section)> = Vec::new();
    let mut info_seen = false;
    let mut info_keys: Vec<&'static str> = Vec::new();
    let mut current: Option<WeaponKind> = None;
    let mut in_info = false;
    let mut line_number = 0usize;

    for raw in source.lines() {
        line_number += 1;
        if line_number > limits.max_lines {
            return Err(WeaponConfigError::TooManyLines {
                lines: line_number,
                limit: limits.max_lines,
            });
        }
        if raw.len() > limits.max_line_bytes {
            return Err(WeaponConfigError::LineTooLong {
                line: line_number,
                bytes: raw.len(),
                limit: limits.max_line_bytes,
            });
        }
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if let Some(header) = line
            .strip_prefix('[')
            .and_then(|rest| rest.strip_suffix(']'))
        {
            let header = header.trim();
            if header == "Info" {
                if info_seen {
                    return Err(WeaponConfigError::DuplicateSection {
                        section: header.to_string(),
                        line: line_number,
                    });
                }
                info_seen = true;
                in_info = true;
                current = None;
                continue;
            }
            in_info = false;
            let Some(kind) = CONFIGURED_WEAPONS
                .into_iter()
                .find(|kind| kind.ini_name() == header)
            else {
                return Err(WeaponConfigError::UnknownSection {
                    section: header.to_string(),
                    line: line_number,
                });
            };
            if sections.iter().any(|(seen, _)| *seen == kind) {
                return Err(WeaponConfigError::DuplicateSection {
                    section: header.to_string(),
                    line: line_number,
                });
            }
            sections.push((kind, Section::default()));
            current = Some(kind);
            continue;
        }

        let Some(split) = line.find('=') else {
            return Err(WeaponConfigError::MalformedLine { line: line_number });
        };
        let key = line[..split].trim();
        let value = line[split + 1..].trim();

        if in_info {
            let Some(known) = INFO_KEYS.into_iter().find(|candidate| *candidate == key) else {
                return Err(WeaponConfigError::UnknownKey {
                    section: "Info".into(),
                    key: key.to_string(),
                    line: line_number,
                });
            };
            if info_keys.contains(&known) {
                return Err(WeaponConfigError::DuplicateKey {
                    section: "Info".into(),
                    key: key.to_string(),
                    line: line_number,
                });
            }
            if value.len() > MAX_INFO_BYTES {
                return Err(WeaponConfigError::OutOfRange {
                    section: "Info".into(),
                    key: key.to_string(),
                    line: line_number,
                });
            }
            info_keys.push(known);
            if known == "Name" {
                name = value.to_string();
            } else {
                version = value.to_string();
            }
            continue;
        }

        let Some(kind) = current else {
            return Err(WeaponConfigError::KeyOutsideSection {
                key: key.to_string(),
                line: line_number,
            });
        };
        let section_name = kind.ini_name();
        let section = sections
            .iter_mut()
            .find(|(seen, _)| *seen == kind)
            .map(|(_, section)| section)
            .expect("the current section was just pushed");

        if key == "NoCollision" {
            if section.no_collision.is_some() {
                return Err(WeaponConfigError::DuplicateKey {
                    section: section_name.into(),
                    key: key.to_string(),
                    line: line_number,
                });
            }
            let parsed = value
                .parse::<u8>()
                .map_err(|_| WeaponConfigError::InvalidValue {
                    section: section_name.into(),
                    key: key.to_string(),
                    line: line_number,
                })?;
            if parsed & !NoCollision::MASK != 0 {
                return Err(WeaponConfigError::OutOfRange {
                    section: section_name.into(),
                    key: key.to_string(),
                    line: line_number,
                });
            }
            section.no_collision = Some((parsed, line_number));
            continue;
        }

        let Some(known) = WEAPON_KEYS.into_iter().find(|candidate| *candidate == key) else {
            return Err(WeaponConfigError::UnknownKey {
                section: section_name.into(),
                key: key.to_string(),
                line: line_number,
            });
        };
        if section.values.iter().any(|(seen, _, _)| *seen == known) {
            return Err(WeaponConfigError::DuplicateKey {
                section: section_name.into(),
                key: key.to_string(),
                line: line_number,
            });
        }
        let parsed = value
            .parse::<f64>()
            .ok()
            .filter(|number| number.is_finite())
            .ok_or_else(|| WeaponConfigError::InvalidValue {
                section: section_name.into(),
                key: key.to_string(),
                line: line_number,
            })?;
        section.values.push((known, parsed, line_number));
    }

    let mut defs = Vec::with_capacity(CONFIGURED_WEAPONS.len());
    for kind in CONFIGURED_WEAPONS {
        let section_name = kind.ini_name();
        let Some((_, section)) = sections.iter().find(|(seen, _)| *seen == kind) else {
            return Err(WeaponConfigError::MissingSection {
                section: section_name.into(),
            });
        };
        for key in WEAPON_KEYS {
            if section.value(key).is_none() {
                return Err(WeaponConfigError::MissingKey {
                    section: section_name.into(),
                    key: key.into(),
                });
            }
        }
        defs.push(build(kind, section, limits)?);
    }

    Ok(WeaponTable::from_configured(name, version, defs))
}

/// Range-checks one section and turns it into a definition.
fn build(
    kind: WeaponKind,
    section: &Section,
    limits: &WeaponLimits,
) -> Result<WeaponDef, WeaponConfigError> {
    let section_name = kind.ini_name();
    let fail = |key: &str| WeaponConfigError::OutOfRange {
        section: section_name.into(),
        key: key.into(),
        line: section.line(key),
    };
    let read = |key: &str| section.value(key).expect("presence was already checked");

    let ranged = |key: &str, low: f64, high: f64| -> Result<f64, WeaponConfigError> {
        let value = read(key);
        if value < low || value > high {
            return Err(fail(key));
        }
        Ok(value)
    };

    let damage = ranged("Damage", 0.0, f64::from(limits.max_damage))?;
    let fire_interval = ranged("FireInterval", 1.0, f64::from(limits.max_ticks))?;
    let ammo = ranged("Ammo", 0.0, f64::from(limits.max_ammo))?;
    let reload_time = ranged("ReloadTime", 0.0, f64::from(limits.max_ticks))?;
    let speed = ranged("Speed", 0.0, f64::from(limits.max_speed))?;
    let style_code = ranged("BulletStyle", 1.0, 14.0)?;
    let start_up_time = ranged("StartUpTime", 0.0, f64::from(limits.max_ticks))?;
    let bink = ranged(
        "Bink",
        -f64::from(limits.max_bink),
        f64::from(limits.max_bink),
    )?;
    let movement_acc = ranged("MovementAcc", 0.0, f64::from(limits.max_accuracy))?;
    let bullet_spread = ranged("BulletSpread", 0.0, f64::from(limits.max_accuracy))?;
    let recoil = ranged("Recoil", 0.0, f64::from(limits.max_ticks))?;
    let push = ranged("Push", 0.0, f64::from(limits.max_push))?;
    let inherited_velocity = ranged(
        "InheritedVelocity",
        0.0,
        f64::from(limits.max_inherited_velocity),
    )?;
    let modifier_head = ranged("ModifierHead", 0.0, f64::from(limits.max_region_modifier))?;
    let modifier_chest = ranged("ModifierChest", 0.0, f64::from(limits.max_region_modifier))?;
    let modifier_legs = ranged("ModifierLegs", 0.0, f64::from(limits.max_region_modifier))?;

    // Integer fields have to be whole numbers, not rounded silently.
    for (key, value) in [
        ("FireInterval", fire_interval),
        ("Ammo", ammo),
        ("ReloadTime", reload_time),
        ("BulletStyle", style_code),
        ("StartUpTime", start_up_time),
        ("Bink", bink),
        ("Recoil", recoil),
    ] {
        if value.fract() != 0.0 {
            return Err(fail(key));
        }
    }

    let bullet_style =
        BulletStyle::from_code(style_code as u8).ok_or_else(|| fail("BulletStyle"))?;

    Ok(WeaponDef {
        kind,
        damage: damage as f32,
        fire_interval: fire_interval as u16,
        ammo: ammo as u8,
        reload_time: reload_time as u16,
        speed: speed as f32,
        bullet_style,
        start_up_time: start_up_time as u16,
        bink: bink as i16,
        movement_acc: movement_acc as f32,
        bullet_spread: bullet_spread as f32,
        recoil: recoil as u16,
        push: push as f32,
        inherited_velocity: inherited_velocity as f32,
        modifier_head: modifier_head as f32,
        modifier_chest: modifier_chest as f32,
        modifier_legs: modifier_legs as f32,
        no_collision: section
            .no_collision
            .map_or(NoCollision::NONE, |(bits, _)| NoCollision(bits)),
    })
}
