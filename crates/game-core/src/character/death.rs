use super::damage::DamageCause;
use crate::collision::BodyRegion;
use serde::{Deserialize, Serialize};

/// Why a player died, resolved once so every message, statistic, and replay agrees.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DeathCause {
    Killed {
        by: u32,
        cause: DamageCause,
        region: BodyRegion,
    },
    TeamKill {
        by: u32,
    },
    Suicide(DamageCause),
    Environment(DamageCause),
}

/// One line of the kill feed. The client renders it; it never recomputes who killed whom.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KillFeedEntry {
    pub killer: Option<u32>,
    pub target: u32,
    pub cause: DamageCause,
    pub region: BodyRegion,
    pub headshot: bool,
    pub teamkill: bool,
    pub suicide: bool,
    /// How many kills this killer has strung together inside the multi-kill window.
    pub multi: u8,
    pub assists: Vec<u32>,
}

impl KillFeedEntry {
    pub fn describe(&self, name: impl Fn(u32) -> String) -> String {
        let target = name(self.target);
        if self.suicide {
            return format!("{target} {}", suicide_verb(self.cause));
        }
        let Some(killer) = self.killer else {
            return format!("{target} {}", suicide_verb(self.cause));
        };
        let killer = name(killer);
        let mut line = if self.teamkill {
            format!("{killer} team-killed {target}")
        } else if self.headshot {
            format!("{killer} headshot {target}")
        } else {
            format!("{killer} {} {target}", kill_verb(self.cause))
        };
        if let Some(label) = multi_kill_label(self.multi) {
            line.push_str(&format!(" ({label})"));
        }
        line
    }
}

fn kill_verb(cause: DamageCause) -> &'static str {
    match cause {
        DamageCause::Bullet | DamageCause::Pellet => "shot",
        DamageCause::Explosion => "blew up",
        DamageCause::Melee => "cut down",
        DamageCause::Fall => "dropped",
        DamageCause::Bleeding => "bled out",
        DamageCause::Deadly => "finished off",
    }
}

fn suicide_verb(cause: DamageCause) -> &'static str {
    match cause {
        DamageCause::Explosion => "blew themselves up",
        DamageCause::Fall => "fell to their death",
        DamageCause::Bleeding => "bled out",
        DamageCause::Deadly => "found the deadly ground",
        _ => "killed themselves",
    }
}

pub fn multi_kill_label(multi: u8) -> Option<&'static str> {
    match multi {
        0 | 1 => None,
        2 => Some("double kill"),
        3 => Some("triple kill"),
        4 => Some("multi kill"),
        _ => Some("rampage"),
    }
}

/// Rolling multi-kill counter for one player.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct MultiKill {
    pub count: u8,
    pub last_tick: u64,
}

impl MultiKill {
    /// Records a kill and returns the streak length including it.
    pub fn record(&mut self, tick: u64, window: u64) -> u8 {
        if self.count > 0 && tick.saturating_sub(self.last_tick) <= window {
            self.count = self.count.saturating_add(1);
        } else {
            self.count = 1;
        }
        self.last_tick = tick;
        self.count
    }
}
