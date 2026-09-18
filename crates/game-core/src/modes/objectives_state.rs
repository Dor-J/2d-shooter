//! The objective layer of a running match: the flags on the map and what happens to them.
//!
//! This is the one place that touches flags during a tick. It asks the mode's policy what is
//! allowed, moves the flags, and reports what happened as typed events and awards; it never scores
//! anything itself, because `ScoreLedger` is the only thing that keeps a total.

use serde::{Deserialize, Serialize};

use super::objective::{Award, FlagAction, ObjectivePolicy, ObjectiveRules, TimedAward};
use super::rules::ModeRules;
use crate::collision::CollisionWorld;
use crate::objects::{Flag, FlagEvent, FlagKind, FlagState};
use crate::Vec2;

/// Everything about the flags in one match.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Objectives {
    /// The flags on the map, in a fixed order so a replay is deterministic.
    pub flags: Vec<Flag>,
    /// Ticks since the timed award last paid out.
    pub award_timer: u32,
    /// What happened to the flags this tick.
    #[serde(default)]
    pub events: Vec<FlagEvent>,
}

/// One player's position, team, and whether they are alive, which is all the flags care about.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bearer {
    pub id: u32,
    pub team: u8,
    pub pos: Vec2,
    pub velocity: Vec2,
    pub alive: bool,
    /// Whether they are asking to throw what they are carrying this tick.
    pub throwing: bool,
    /// Where they are aiming, for a throw.
    pub aim: Vec2,
}

impl Objectives {
    /// Sets up the flags a mode wants, at the bases it is given.
    ///
    /// `base_for` supplies each flag's home; a mode whose map has no suitable spawn gets no flag
    /// rather than one floating at the origin.
    pub fn for_mode(rules: &ModeRules, base_for: impl Fn(FlagKind) -> Option<Vec2>) -> Self {
        let policy = ObjectiveRules::for_mode(rules.kind);
        let flags = policy
            .layout()
            .flags()
            .iter()
            .filter_map(|kind| base_for(*kind).map(|base| Flag::new(*kind, base)))
            .collect();
        Self {
            flags,
            award_timer: 0,
            events: Vec::new(),
        }
    }

    pub fn flag(&self, kind: FlagKind) -> Option<&Flag> {
        self.flags.iter().find(|flag| flag.kind == kind)
    }

    fn flag_mut(&mut self, kind: FlagKind) -> Option<&mut Flag> {
        self.flags.iter_mut().find(|flag| flag.kind == kind)
    }

    /// The flag a player is carrying, if any.
    pub fn carried_by(&self, player: u32) -> Option<&Flag> {
        self.flags
            .iter()
            .find(|flag| flag.state.carrier() == Some(player))
    }

    /// Whether this player is carrying anything, which is what Pointmatch doubles kills for.
    pub fn is_carrying(&self, player: u32) -> bool {
        self.carried_by(player).is_some()
    }

    /// Everyone currently carrying a flag, for the HUD's carrier indicator.
    pub fn carriers(&self) -> impl Iterator<Item = (FlagKind, u32)> + '_ {
        self.flags
            .iter()
            .filter_map(|flag| flag.state.carrier().map(|by| (flag.kind, by)))
    }

    /// Advances every flag and resolves what the players standing on them may do.
    ///
    /// Returns the points to be booked. Nothing here writes a score.
    pub fn step(
        &mut self,
        rules: &ModeRules,
        collision: &CollisionWorld,
        dt: f32,
        bearers: &[Bearer],
        team_sizes: (usize, usize),
    ) -> Vec<Award> {
        self.events.clear();
        let policy = ObjectiveRules::for_mode(rules.kind);
        let mut awards = Vec::new();

        // A carrier who died or left drops what they were holding, wherever they were.
        let dropped: Vec<(FlagKind, Vec2, Vec2)> = self
            .flags
            .iter()
            .filter_map(|flag| {
                let carrier = flag.state.carrier()?;
                match bearers.iter().find(|bearer| bearer.id == carrier) {
                    Some(bearer) if bearer.alive && !bearer.throwing => None,
                    Some(bearer) if bearer.throwing => Some((
                        flag.kind,
                        bearer.pos,
                        Flag::throw_velocity(
                            Vec2 {
                                x: bearer.aim.x - bearer.pos.x,
                                y: bearer.aim.y - bearer.pos.y,
                            },
                            bearer.velocity,
                        ),
                    )),
                    Some(bearer) => Some((flag.kind, bearer.pos, bearer.velocity)),
                    // The carrier is gone entirely: the flag falls where the flag last was.
                    None => Some((flag.kind, flag.body.pos, Vec2::default())),
                }
            })
            .collect();
        for (kind, pos, velocity) in dropped {
            if let Some(flag) = self.flag_mut(kind) {
                if let Some(event) = flag.drop_at(pos, velocity) {
                    self.events.push(event);
                }
            }
        }

        // Move the flags.
        let carrier_positions: Vec<(FlagKind, Option<Vec2>)> = self
            .flags
            .iter()
            .map(|flag| {
                let pos = flag.state.carrier().and_then(|carrier| {
                    bearers
                        .iter()
                        .find(|bearer| bearer.id == carrier)
                        .map(|bearer| bearer.pos)
                });
                (flag.kind, pos)
            })
            .collect();
        for (kind, carrier_pos) in carrier_positions {
            if let Some(flag) = self.flag_mut(kind) {
                if let Some(event) = flag.step(collision, dt, carrier_pos) {
                    self.events.push(event);
                }
            }
        }

        // Let living players touch the flags they are standing on.
        for bearer in bearers.iter().filter(|bearer| bearer.alive) {
            let touched: Vec<(FlagKind, FlagAction)> = self
                .flags
                .iter()
                .filter(|flag| flag.within_reach(bearer.pos))
                .map(|flag| (flag.kind, policy.action_for(flag, bearer.team)))
                .collect();
            for (kind, action) in touched {
                let Some(flag) = self.flag_mut(kind) else {
                    continue;
                };
                let event = match action {
                    FlagAction::Take => flag.take(bearer.id),
                    FlagAction::Return if !flag.state.is_at_base() => {
                        Some(flag.return_home(Some(bearer.id)))
                    }
                    _ => None,
                };
                if let Some(event) = event {
                    self.events.push(event);
                }
            }
        }

        // Anybody who can score a capture, does.
        let captures: Vec<(FlagKind, Award)> = bearers
            .iter()
            .filter(|bearer| bearer.alive)
            .filter_map(|bearer| {
                let carried = self.carried_by(bearer.id)?;
                let own = self
                    .flags
                    .iter()
                    .find(|flag| flag.kind.belongs_to(bearer.team));
                policy
                    .capture_award(carried, own, bearer.id, bearer.team)
                    .map(|award| (carried.kind, award))
            })
            .collect();
        for (kind, award) in captures {
            if let Some(flag) = self.flag_mut(kind) {
                let event = flag.return_home(None);
                self.events.push(FlagEvent::Captured {
                    flag: kind,
                    by: award.player,
                });
                self.events.push(event);
            }
            awards.push(award);
        }

        // The modes that pay out on a clock do so here.
        if let Some(timer) = policy.timed_award() {
            awards.extend(self.tick_timed_award(policy, timer, bearers, team_sizes));
        } else {
            self.award_timer = 0;
        }

        awards
    }

    /// Advances the scoring clock for Hold the Flag and Infiltration.
    fn tick_timed_award(
        &mut self,
        policy: ObjectiveRules,
        timer: TimedAward,
        bearers: &[Bearer],
        team_sizes: (usize, usize),
    ) -> Vec<Award> {
        let (alpha, bravo) = team_sizes;
        // Neither mode scores at all until both sides are actually playing.
        if alpha == 0 || bravo == 0 {
            self.award_timer = 0;
            return Vec::new();
        }

        let scoring = match policy {
            // Hold the Flag pays whoever is holding the yellow flag.
            ObjectiveRules::HoldTheFlag => self
                .flag(FlagKind::Yellow)
                .and_then(|flag| flag.state.carrier())
                .and_then(|carrier| bearers.iter().find(|bearer| bearer.id == carrier))
                .filter(|bearer| bearer.alive)
                .map(|bearer| (bearer.id, bearer.team)),
            // Infiltration pays the defenders for simply keeping their objective at home.
            ObjectiveRules::Infiltration => self
                .flag(FlagKind::Bravo)
                .filter(|flag| flag.is_home_and_free())
                .map(|flag| (0, flag.kind.owning_team())),
            _ => None,
        };

        let Some((player, team)) = scoring else {
            self.award_timer = 0;
            return Vec::new();
        };

        let advantage = match team {
            1 => i32::try_from(alpha).unwrap_or(0) - i32::try_from(bravo).unwrap_or(0),
            2 => i32::try_from(bravo).unwrap_or(0) - i32::try_from(alpha).unwrap_or(0),
            _ => 0,
        };
        let interval = timer.interval(advantage).max(1);

        self.award_timer = self.award_timer.saturating_add(1);
        if self.award_timer < interval {
            return Vec::new();
        }
        self.award_timer = 0;
        vec![Award {
            player,
            team,
            points: super::objective::HTF_HOLD_AWARD,
        }]
    }

    /// Sends every flag home, which is what a round reset does.
    pub fn reset(&mut self) {
        self.events.clear();
        for flag in &mut self.flags {
            let event = flag.return_home(None);
            self.events.push(event);
        }
        self.award_timer = 0;
    }

    /// Shoves any loose flag within `radius` of `at`, which is how a blast moves one.
    pub fn push_near(&mut self, at: Vec2, radius: f32, impulse: Vec2) {
        for flag in &mut self.flags {
            if flag.state.carrier().is_some() {
                continue;
            }
            let dx = flag.body.pos.x - at.x;
            let dy = flag.body.pos.y - at.y;
            if (dx * dx + dy * dy).sqrt() <= radius {
                flag.push(impulse);
            }
        }
    }

    /// Whether a flag is away from its base, for the HUD's missing-flag indicator.
    pub fn is_missing(&self, kind: FlagKind) -> bool {
        self.flag(kind)
            .is_some_and(|flag| !matches!(flag.state, FlagState::AtBase))
    }
}
