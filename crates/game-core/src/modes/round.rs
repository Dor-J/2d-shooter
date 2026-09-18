//! The shape of a match over time: lobby, countdown, play, overtime, the scoreboard, and the
//! transition to the next map.
//!
//! Every mode runs through the same phases. A mode decides what scores; it does not decide how a
//! round begins or ends.

use serde::{Deserialize, Serialize};

use super::rules::ModeRules;
use super::score::{leader, limit_reached, Outcome, ScoreLedger};

/// Where a match is in its life.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchPhase {
    /// Waiting for enough players to start.
    #[default]
    Lobby,
    /// Counting down to the first shot. Nobody may fire yet.
    Countdown,
    /// Being played.
    Active,
    /// Level at the final whistle and playing on to break it.
    Overtime,
    /// Over, showing the scoreboard.
    RoundEnd,
    /// Loading the next map.
    MapTransition,
}

impl MatchPhase {
    /// Whether players may move and shoot.
    pub const fn is_playable(self) -> bool {
        matches!(self, Self::Active | Self::Overtime)
    }

    /// Whether the match is finished, one way or another.
    pub const fn is_finished(self) -> bool {
        matches!(self, Self::RoundEnd | Self::MapTransition)
    }
}

/// Something the lifecycle decided, for the server and the client to act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchEvent {
    CountdownStarted {
        ticks: u32,
    },
    RoundStarted,
    /// The clock ran out or a limit was reached, and the match was level.
    OvertimeStarted {
        ticks: u32,
    },
    RoundEnded {
        outcome: Outcome,
    },
    /// The scoreboard should be shown, and a screenshot taken if that is switched on.
    ScoreboardShown {
        screenshot: bool,
    },
    MapTransitionStarted,
    /// The next map should be loaded now.
    NextMap,
}

/// The running state of one match.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchState {
    pub phase: MatchPhase,
    /// Ticks left in the current phase, where the phase is timed.
    pub phase_ticks: u32,
    /// Ticks played so far, which is what the time limit is measured against.
    pub elapsed: u32,
    /// How the match ended, once it has.
    pub outcome: Option<Outcome>,
    /// Whether a screenshot should be taken when the final scoreboard appears.
    pub final_screenshot: bool,
    /// Players needed before the countdown begins.
    pub minimum_players: usize,
}

impl Default for MatchState {
    fn default() -> Self {
        Self {
            phase: MatchPhase::Lobby,
            phase_ticks: 0,
            elapsed: 0,
            outcome: None,
            final_screenshot: false,
            minimum_players: 1,
        }
    }
}

impl MatchState {
    /// Ticks left on the clock, or `None` when the match is untimed.
    pub fn time_left(&self, rules: &ModeRules) -> Option<u32> {
        if rules.limits.time_ticks == 0 {
            return None;
        }
        Some(rules.limits.time_ticks.saturating_sub(self.elapsed))
    }

    /// Restarts the match from the countdown, keeping nothing but the rules.
    ///
    /// The caller clears the ledger; this only resets the clock, because a restart that silently
    /// kept half the old state would be worse than no restart at all.
    pub fn restart(&mut self, rules: &ModeRules) -> MatchEvent {
        self.phase = MatchPhase::Countdown;
        self.phase_ticks = rules.countdown_ticks;
        self.elapsed = 0;
        self.outcome = None;
        MatchEvent::CountdownStarted {
            ticks: rules.countdown_ticks,
        }
    }

    /// Advances the match by one tick and reports anything that changed.
    ///
    /// `players` is how many are present, which is what lifts a match out of the lobby and drops
    /// it back when everybody leaves.
    pub fn step(
        &mut self,
        rules: &ModeRules,
        ledger: &ScoreLedger,
        players: usize,
    ) -> Option<MatchEvent> {
        match self.phase {
            MatchPhase::Lobby => {
                if players >= self.minimum_players {
                    self.phase = MatchPhase::Countdown;
                    self.phase_ticks = rules.countdown_ticks;
                    return Some(MatchEvent::CountdownStarted {
                        ticks: rules.countdown_ticks,
                    });
                }
                None
            }
            MatchPhase::Countdown => {
                if players < self.minimum_players {
                    self.phase = MatchPhase::Lobby;
                    self.phase_ticks = 0;
                    return None;
                }
                self.phase_ticks = self.phase_ticks.saturating_sub(1);
                if self.phase_ticks == 0 {
                    self.phase = MatchPhase::Active;
                    self.elapsed = 0;
                    return Some(MatchEvent::RoundStarted);
                }
                None
            }
            MatchPhase::Active => {
                self.elapsed = self.elapsed.saturating_add(1);
                let out_of_time =
                    rules.limits.time_ticks > 0 && self.elapsed >= rules.limits.time_ticks;
                let scored_out = limit_reached(ledger, rules);
                if !out_of_time && !scored_out {
                    return None;
                }
                let standing = leader(ledger, rules);
                // A limit reached outright is a win. A clock that ran out on a level score is
                // what overtime is for.
                if standing == Outcome::Draw && rules.overtime && rules.overtime_ticks > 0 {
                    self.phase = MatchPhase::Overtime;
                    self.phase_ticks = rules.overtime_ticks;
                    return Some(MatchEvent::OvertimeStarted {
                        ticks: rules.overtime_ticks,
                    });
                }
                Some(self.finish(rules, standing))
            }
            MatchPhase::Overtime => {
                self.elapsed = self.elapsed.saturating_add(1);
                self.phase_ticks = self.phase_ticks.saturating_sub(1);
                let standing = leader(ledger, rules);
                // Overtime ends the moment somebody goes ahead, or when it runs out.
                if standing != Outcome::Draw || self.phase_ticks == 0 {
                    return Some(self.finish(rules, standing));
                }
                None
            }
            MatchPhase::RoundEnd => {
                self.phase_ticks = self.phase_ticks.saturating_sub(1);
                if self.phase_ticks == 0 {
                    self.phase = MatchPhase::MapTransition;
                    return Some(MatchEvent::MapTransitionStarted);
                }
                None
            }
            MatchPhase::MapTransition => Some(MatchEvent::NextMap),
        }
    }

    fn finish(&mut self, rules: &ModeRules, outcome: Outcome) -> MatchEvent {
        self.phase = MatchPhase::RoundEnd;
        self.phase_ticks = rules.round_end_ticks;
        self.outcome = Some(outcome);
        MatchEvent::RoundEnded { outcome }
    }

    /// The scoreboard event that follows a round ending, which the server raises once.
    pub fn scoreboard_event(&self) -> MatchEvent {
        MatchEvent::ScoreboardShown {
            screenshot: self.final_screenshot,
        }
    }
}

/// A map rotation: which maps a server plays, in order, and what happens at the end of the list.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rotation {
    maps: Vec<String>,
    index: usize,
    /// Whether the list starts again at the end. When it does not, the last map repeats.
    pub loops: bool,
}

impl Rotation {
    /// Builds a rotation, refusing an empty or all-blank list rather than leaving a server with
    /// nothing to load.
    pub fn new(maps: Vec<String>, loops: bool) -> Option<Self> {
        let maps: Vec<String> = maps
            .into_iter()
            .map(|name| name.trim().to_string())
            .filter(|name| !name.is_empty())
            .collect();
        if maps.is_empty() {
            return None;
        }
        Some(Self {
            maps,
            index: 0,
            loops,
        })
    }

    pub fn current(&self) -> &str {
        &self.maps[self.index]
    }

    pub fn len(&self) -> usize {
        self.maps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.maps.is_empty()
    }

    /// Moves to the next map and returns it. A non-looping rotation stays on its last map.
    pub fn next_map(&mut self) -> &str {
        if self.index + 1 < self.maps.len() {
            self.index += 1;
        } else if self.loops {
            self.index = 0;
        }
        self.current()
    }

    /// Jumps to a named map if the rotation contains it.
    pub fn select(&mut self, name: &str) -> bool {
        match self.maps.iter().position(|map| map == name) {
            Some(index) => {
                self.index = index;
                true
            }
            None => false,
        }
    }

    /// Whether the rotation has played its last map and will not start over.
    pub fn is_exhausted(&self) -> bool {
        !self.loops && self.index + 1 == self.maps.len()
    }
}
