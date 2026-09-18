//! Everything a match records about how it was played.
//!
//! The score ledger answers "who is winning". This answers "what happened": shots, hits, accuracy,
//! which weapon killed whom, and how a round finished. Both are fed by the same typed events, so
//! the statistics and the scoreboard can never tell different stories.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::score::{PlayerScore, ScoreLedger};
use crate::character::DamageCause;
use crate::weapons::WeaponKind;

/// How many finished matches a server keeps in memory.
pub const MATCH_HISTORY_LIMIT: usize = 20;

/// Something worth recording that is not worth points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatEvent {
    /// A trigger was pulled and a projectile left the barrel.
    Shot { player: u32, weapon: WeaponKind },
    /// A projectile found a body.
    Hit {
        player: u32,
        weapon: WeaponKind,
        headshot: bool,
    },
    /// Somebody was killed with this weapon.
    KilledWith {
        killer: u32,
        victim: u32,
        weapon: WeaponKind,
    },
    /// Somebody died to something that was not a weapon.
    DiedTo { victim: u32, cause: DamageCause },
    /// An objective was scored: a capture, a return, or a hold.
    Objective { player: u32, kind: ObjectiveStat },
}

/// The objective events worth counting separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ObjectiveStat {
    Capture,
    Return,
    Hold,
}

/// What one player did with one weapon.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponStats {
    pub shots: u32,
    pub hits: u32,
    pub kills: u32,
    pub deaths: u32,
    pub headshots: u32,
}

impl WeaponStats {
    /// Hits as a percentage of shots, rounded down. No shots is no accuracy rather than a divide
    /// by zero or a flattering hundred per cent.
    pub fn accuracy_percent(&self) -> u32 {
        if self.shots == 0 {
            return 0;
        }
        (u64::from(self.hits) * 100 / u64::from(self.shots)) as u32
    }
}

/// Everything recorded about one player.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerStats {
    pub shots: u32,
    pub hits: u32,
    pub headshots: u32,
    pub captures: u32,
    pub returns: u32,
    pub holds: u32,
    /// Per-weapon breakdown, ordered so a scoreboard does not jump about.
    pub weapons: BTreeMap<WeaponKind, WeaponStats>,
    /// What killed this player, and how often.
    pub deaths_by_cause: BTreeMap<DamageCause, u32>,
}

impl PlayerStats {
    pub fn accuracy_percent(&self) -> u32 {
        if self.shots == 0 {
            return 0;
        }
        (u64::from(self.hits) * 100 / u64::from(self.shots)) as u32
    }

    /// The weapon this player has killed most with, if they have killed with any.
    pub fn favourite_weapon(&self) -> Option<WeaponKind> {
        self.weapons
            .iter()
            .filter(|(_, stats)| stats.kills > 0)
            .max_by_key(|(kind, stats)| (stats.kills, std::cmp::Reverse(**kind)))
            .map(|(kind, _)| *kind)
    }
}

/// The statistics for one match.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchStats {
    players: BTreeMap<u32, PlayerStats>,
}

impl MatchStats {
    pub fn player(&self, id: u32) -> PlayerStats {
        self.players.get(&id).cloned().unwrap_or_default()
    }

    pub fn entries(&self) -> impl Iterator<Item = (u32, &PlayerStats)> {
        self.players.iter().map(|(id, stats)| (*id, stats))
    }

    /// Makes sure a player has a row, without disturbing one they already have.
    pub fn ensure(&mut self, id: u32) {
        self.players.entry(id).or_default();
    }

    /// Records one event.
    pub fn record(&mut self, event: StatEvent) {
        match event {
            StatEvent::Shot { player, weapon } => {
                let stats = self.players.entry(player).or_default();
                stats.shots += 1;
                stats.weapons.entry(weapon).or_default().shots += 1;
            }
            StatEvent::Hit {
                player,
                weapon,
                headshot,
            } => {
                let stats = self.players.entry(player).or_default();
                stats.hits += 1;
                let per_weapon = stats.weapons.entry(weapon).or_default();
                per_weapon.hits += 1;
                if headshot {
                    stats.headshots += 1;
                    per_weapon.headshots += 1;
                }
            }
            StatEvent::KilledWith {
                killer,
                victim,
                weapon,
            } => {
                self.players
                    .entry(killer)
                    .or_default()
                    .weapons
                    .entry(weapon)
                    .or_default()
                    .kills += 1;
                self.players
                    .entry(victim)
                    .or_default()
                    .weapons
                    .entry(weapon)
                    .or_default()
                    .deaths += 1;
            }
            StatEvent::DiedTo { victim, cause } => {
                *self
                    .players
                    .entry(victim)
                    .or_default()
                    .deaths_by_cause
                    .entry(cause)
                    .or_default() += 1;
            }
            StatEvent::Objective { player, kind } => {
                let stats = self.players.entry(player).or_default();
                match kind {
                    ObjectiveStat::Capture => stats.captures += 1,
                    ObjectiveStat::Return => stats.returns += 1,
                    ObjectiveStat::Hold => stats.holds += 1,
                }
            }
        }
    }

    /// Forgets everything, which is what a match restart does.
    pub fn clear(&mut self) {
        self.players.clear();
    }
}

/// One row of a scoreboard: who somebody is, where they stand, and how they have played.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreboardRow {
    pub player: u32,
    pub team: u8,
    /// First is rank one.
    pub rank: u32,
    pub score: PlayerScore,
    pub stats: PlayerStats,
    /// How far behind the leader, which is zero for the leader themselves.
    pub behind_leader: i32,
}

/// Builds the scoreboard: everybody sorted by points, then by kills, then by id.
///
/// Sorting has to be total and deterministic or two clients showing the same match would disagree
/// about who is third.
pub fn scoreboard(
    ledger: &ScoreLedger,
    stats: &MatchStats,
    team_of: impl Fn(u32) -> u8,
) -> Vec<ScoreboardRow> {
    let mut rows: Vec<(u32, PlayerScore)> = ledger.entries().collect();
    rows.sort_by(|left, right| {
        right
            .1
            .points
            .cmp(&left.1.points)
            .then(right.1.kills.cmp(&left.1.kills))
            .then(left.1.deaths.cmp(&right.1.deaths))
            .then(left.0.cmp(&right.0))
    });

    let leader = rows.first().map_or(0, |(_, score)| score.points);
    rows.into_iter()
        .enumerate()
        .map(|(index, (player, score))| ScoreboardRow {
            player,
            team: team_of(player),
            rank: index as u32 + 1,
            behind_leader: leader - score.points,
            score,
            stats: stats.player(player),
        })
        .collect()
}

/// A finished match, kept so a server can show what happened before this one.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchSummary {
    pub map: String,
    pub mode: String,
    /// Ticks the match ran for.
    pub duration: u32,
    /// How it ended, as words, so a summary survives a rules change.
    pub outcome: String,
    pub rows: Vec<ScoreboardRow>,
}

impl MatchSummary {
    /// The summary as a tab-separated log, one line per player.
    ///
    /// Deliberately plain text: a server operator should be able to read it, grep it, and paste it
    /// somewhere without a tool.
    pub fn to_log(&self) -> String {
        let mut out = format!(
            "# {} on {} ({} ticks) — {}\n",
            self.mode, self.map, self.duration, self.outcome
        );
        out.push_str("rank\tplayer\tteam\tpoints\tkills\tdeaths\tshots\thits\taccuracy\n");
        for row in &self.rows {
            out.push_str(&format!(
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}%\n",
                row.rank,
                row.player,
                row.team,
                row.score.points,
                row.score.kills,
                row.score.deaths,
                row.stats.shots,
                row.stats.hits,
                row.stats.accuracy_percent(),
            ));
        }
        out
    }
}

/// The matches a server has finished, newest last.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchHistory {
    entries: Vec<MatchSummary>,
}

impl MatchHistory {
    /// Adds a finished match, dropping the oldest once the limit is reached.
    ///
    /// A server that ran for a week would otherwise hold every match it had ever played.
    pub fn push(&mut self, summary: MatchSummary) {
        self.entries.push(summary);
        if self.entries.len() > MATCH_HISTORY_LIMIT {
            let excess = self.entries.len() - MATCH_HISTORY_LIMIT;
            self.entries.drain(0..excess);
        }
    }

    pub fn entries(&self) -> &[MatchSummary] {
        &self.entries
    }

    pub fn latest(&self) -> Option<&MatchSummary> {
        self.entries.last()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Every match as one log, oldest first.
    pub fn to_log(&self) -> String {
        self.entries
            .iter()
            .map(MatchSummary::to_log)
            .collect::<Vec<_>>()
            .join("\n")
    }
}
