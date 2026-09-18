//! One ledger for everything a match scores, and the rules for reading a winner out of it.
//!
//! Every point in the game arrives here as a typed event. Nothing else keeps a running total, so
//! the scoreboard, the end screen, the statistics, and the win condition can never disagree.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::rules::{ModeRules, ScoringPolicy};

/// Something that happened which is worth points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScoreEvent {
    Kill {
        killer: u32,
        victim: u32,
    },
    TeamKill {
        killer: u32,
        victim: u32,
    },
    Suicide {
        player: u32,
    },
    /// A mode objective was scored, such as a flag captured or a point held.
    Objective {
        player: u32,
        team: u8,
        points: u32,
    },
}

/// One player's running totals.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerScore {
    pub points: i32,
    pub kills: u32,
    pub deaths: u32,
    pub teamkills: u32,
    pub suicides: u32,
    pub objectives: u32,
}

/// The whole match's score.
///
/// Player totals are kept by id and survive a disconnect, so reconnecting restores a score rather
/// than handing out a fresh one.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreLedger {
    players: BTreeMap<u32, PlayerScore>,
    teams: [i32; 3],
}

impl ScoreLedger {
    pub fn player(&self, id: u32) -> PlayerScore {
        self.players.get(&id).copied().unwrap_or_default()
    }

    /// Team scores as `[neutral, alpha, bravo]`.
    pub const fn teams(&self) -> [i32; 3] {
        self.teams
    }

    pub fn team(&self, team: u8) -> i32 {
        self.teams.get(usize::from(team)).copied().unwrap_or(0)
    }

    /// Every player who has a score, lowest id first.
    pub fn entries(&self) -> impl Iterator<Item = (u32, PlayerScore)> + '_ {
        self.players.iter().map(|(id, score)| (*id, *score))
    }

    /// Makes sure a player has a row, without disturbing one they already have.
    ///
    /// This is what preserves a score across a disconnect: rejoining touches the existing row.
    pub fn ensure(&mut self, id: u32) {
        self.players.entry(id).or_default();
    }

    /// A death that is not worth a kill, which is what Rambomatch does to everyone but the bow holder.
    pub fn note_death(&mut self, id: u32) {
        self.players.entry(id).or_default().deaths += 1;
    }

    /// Forgets a player entirely. Only used when a match ends or a player is removed for good.
    pub fn forget(&mut self, id: u32) {
        self.players.remove(&id);
    }

    /// Applies one scoring event. `team_of` says which side a player is on.
    pub fn record(&mut self, event: ScoreEvent, rules: &ModeRules, team_of: impl Fn(u32) -> u8) {
        let policy = rules.scoring;
        match event {
            ScoreEvent::Kill { killer, victim } => {
                let entry = self.players.entry(killer).or_default();
                entry.kills += 1;
                entry.points += policy.kill;
                self.players.entry(victim).or_default().deaths += 1;
                self.add_team(team_of(killer), policy.kill, rules);
            }
            ScoreEvent::TeamKill { killer, victim } => {
                let entry = self.players.entry(killer).or_default();
                entry.teamkills += 1;
                entry.points += policy.teamkill;
                self.players.entry(victim).or_default().deaths += 1;
                if policy.teamkill_costs_the_team {
                    self.add_team(team_of(killer), policy.teamkill, rules);
                }
            }
            ScoreEvent::Suicide { player } => {
                let entry = self.players.entry(player).or_default();
                entry.suicides += 1;
                entry.deaths += 1;
                entry.points += policy.suicide;
                self.add_team(team_of(player), policy.suicide, rules);
            }
            ScoreEvent::Objective {
                player,
                team,
                points,
            } => {
                let entry = self.players.entry(player).or_default();
                entry.objectives += 1;
                entry.points += points as i32;
                self.add_team(team, points as i32, rules);
            }
        }
    }

    fn add_team(&mut self, team: u8, delta: i32, rules: &ModeRules) {
        if !rules.is_team_mode() {
            return;
        }
        if let Some(slot) = self.teams.get_mut(usize::from(team)) {
            *slot += delta;
        }
    }

    /// The highest score any single player has reached.
    pub fn best_player_points(&self) -> i32 {
        self.players
            .values()
            .map(|score| score.points)
            .max()
            .unwrap_or(0)
    }

    /// The most kills any single player has.
    pub fn best_player_kills(&self) -> u32 {
        self.players
            .values()
            .map(|score| score.kills)
            .max()
            .unwrap_or(0)
    }
}

/// How a match ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    /// One player won a free-for-all.
    Player(u32),
    /// One team won.
    Team(u8),
    /// Nobody was ahead.
    Draw,
}

/// Who is winning, or that nobody is.
///
/// A tie at the top is a draw, which is what overtime exists to break.
pub fn leader(ledger: &ScoreLedger, rules: &ModeRules) -> Outcome {
    if rules.is_team_mode() {
        let alpha = ledger.team(1);
        let bravo = ledger.team(2);
        return match alpha.cmp(&bravo) {
            std::cmp::Ordering::Greater => Outcome::Team(1),
            std::cmp::Ordering::Less => Outcome::Team(2),
            std::cmp::Ordering::Equal => Outcome::Draw,
        };
    }

    let mut best: Option<(u32, i32)> = None;
    let mut tied = false;
    for (id, score) in ledger.entries() {
        match best {
            None => best = Some((id, score.points)),
            Some((_, points)) if score.points > points => {
                best = Some((id, score.points));
                tied = false;
            }
            Some((_, points)) if score.points == points => tied = true,
            _ => {}
        }
    }
    match best {
        Some((id, _)) if !tied => Outcome::Player(id),
        _ => Outcome::Draw,
    }
}

/// Whether a score limit has been reached, given the mode's limits.
pub fn limit_reached(ledger: &ScoreLedger, rules: &ModeRules) -> bool {
    let limits = rules.limits;
    if limits.kills > 0 && ledger.best_player_kills() >= limits.kills {
        return true;
    }
    if limits.points > 0 && ledger.best_player_points() >= limits.points as i32 {
        return true;
    }
    if limits.captures > 0 {
        let captures = i32::try_from(limits.captures).unwrap_or(i32::MAX);
        if ledger.team(1) >= captures || ledger.team(2) >= captures {
            return true;
        }
    }
    false
}

/// A convenience for building a scoring policy that never punishes anybody, used by modes and
/// community rulesets that score only objectives.
pub const fn objectives_only() -> ScoringPolicy {
    ScoringPolicy {
        kill: 0,
        teamkill: 0,
        suicide: 0,
        teamkill_costs_the_team: false,
    }
}
