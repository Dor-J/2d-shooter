//! Getting a bot from where it is to where it wants to be.
//!
//! Bots walk, jump, and jet towards a destination and notice when they have stopped making
//! progress. A map with linked waypoints is followed by a breadth-first hop; a map without them
//! is walked in a straight line.

use serde::{Deserialize, Serialize};

use crate::Vec2;

/// How close counts as having arrived.
pub const ARRIVAL_RADIUS: f32 = 28.0;

/// How long a bot may make no progress before it decides it is stuck.
pub const STUCK_TICKS: u16 = 45;

/// How far it has to move in that time to count as progress.
pub const PROGRESS_DISTANCE: f32 = 24.0;

/// How long a stuck bot spends trying something else.
pub const RECOVERY_TICKS: u16 = 60;

/// One point on a map that a bot can route through.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    pub pos: Vec2,
    /// Whether reaching this point needs the jet.
    pub needs_jet: bool,
    /// Connected waypoint indices, in stored order. Unused slots stay zero.
    #[serde(default)]
    pub links: [u16; 20],
    #[serde(default)]
    pub link_count: u8,
}

/// How a bot is getting about.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Navigation {
    /// Where it is trying to get to.
    pub destination: Option<Vec2>,
    /// Where it was when progress was last checked.
    pub last_pos: Vec2,
    /// Ticks since it last made progress.
    pub idle_ticks: u16,
    /// Ticks left of trying something else after getting stuck.
    pub recovery_ticks: u16,
    /// Which way it is trying to get unstuck, so it does not simply jitter on the spot.
    pub recovery_left: bool,
}

/// What a bot wants to do with its body this tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Movement {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub jet: bool,
    pub crouch: bool,
    pub prone: bool,
    pub roll: bool,
}

impl Navigation {
    /// Points the bot somewhere new, resetting its progress check.
    pub fn go_to(&mut self, destination: Vec2, from: Vec2) {
        self.destination = Some(destination);
        self.last_pos = from;
        self.idle_ticks = 0;
    }

    /// Whether it has arrived.
    pub fn has_arrived(&self, pos: Vec2) -> bool {
        let Some(destination) = self.destination else {
            return true;
        };
        let dx = destination.x - pos.x;
        let dy = destination.y - pos.y;
        (dx * dx + dy * dy).sqrt() <= ARRIVAL_RADIUS
    }

    /// Whether the bot has given up on going straight there and is trying something else.
    pub const fn is_recovering(&self) -> bool {
        self.recovery_ticks > 0
    }

    /// Advances the progress check. Returns true on the tick it decides it is stuck.
    ///
    /// A bot walking into a wall would otherwise walk into it forever, which is the single most
    /// obvious way a bot stops looking like a player.
    pub fn check_progress(&mut self, pos: Vec2, rng_bit: bool) -> bool {
        if self.recovery_ticks > 0 {
            self.recovery_ticks -= 1;
            self.last_pos = pos;
            return false;
        }
        if self.destination.is_none() {
            self.idle_ticks = 0;
            self.last_pos = pos;
            return false;
        }

        let dx = pos.x - self.last_pos.x;
        let dy = pos.y - self.last_pos.y;
        if (dx * dx + dy * dy).sqrt() >= PROGRESS_DISTANCE {
            self.idle_ticks = 0;
            self.last_pos = pos;
            return false;
        }

        self.idle_ticks = self.idle_ticks.saturating_add(1);
        if self.idle_ticks < STUCK_TICKS {
            return false;
        }
        self.idle_ticks = 0;
        self.recovery_ticks = RECOVERY_TICKS;
        self.recovery_left = rng_bit;
        self.last_pos = pos;
        true
    }
}

/// Works out what to press to get from `pos` towards `destination`.
///
/// Uphill or across a gap it jumps; a long climb it jets. A bot that is stuck goes the other way
/// and jumps, which is what gets it off a ledge it has wedged itself against.
pub fn steer(nav: &Navigation, pos: Vec2, fuel: f32) -> Movement {
    let Some(destination) = nav.destination else {
        return Movement::default();
    };

    let dx = destination.x - pos.x;
    let dy = destination.y - pos.y;

    if nav.is_recovering() {
        // Back out and hop. A fresh crouch together with a direction starts a roll, which pins
        // the bot on the spot; holding crouch is only safe once it is already crouching.
        return Movement {
            left: nav.recovery_left,
            right: !nav.recovery_left,
            jump: true,
            jet: fuel > 0.25,
            ..Movement::default()
        };
    }

    let mut movement = Movement {
        left: dx < -8.0,
        right: dx > 8.0,
        ..Movement::default()
    };

    // Climbing: a small step is a jump, a real climb wants the jet.
    if dy < -20.0 {
        movement.jump = true;
        if dy < -90.0 && fuel > 0.2 {
            movement.jet = true;
        }
    } else if dy > 24.0 {
        movement.crouch = true;
    }
    movement
}

/// One hop along a waypoint graph.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouteStep {
    pub pos: Vec2,
    /// Jet while heading for this node.
    pub needs_jet: bool,
}

/// The next linked waypoint from `from` toward `goal`.
///
/// Empty graphs and graphs with no connections return `None`, which means walk straight at the
/// goal. The search is a `Vec` breadth-first walk in connection order, so it does not depend on
/// hash iteration.
pub fn route(waypoints: &[Waypoint], from: Vec2, goal: Vec2) -> Option<RouteStep> {
    if !waypoints.iter().any(|point| point.link_count > 0) {
        return None;
    }
    let start = nearest(waypoints, from);
    let end = nearest(waypoints, goal);
    let path = breadth_first(waypoints, start, end)?;
    // The walk target is the next hop, not the node the bot is already nearest.
    let index = path.iter().position(|&node| node == start).unwrap_or(0);
    let step = if index + 1 < path.len() {
        path[index + 1]
    } else {
        path[index]
    };
    let point = &waypoints[step];
    Some(RouteStep {
        pos: point.pos,
        needs_jet: point.needs_jet,
    })
}

fn nearest(waypoints: &[Waypoint], to: Vec2) -> usize {
    let mut best = 0;
    let mut best_distance = f32::INFINITY;
    for (index, point) in waypoints.iter().enumerate() {
        let distance = distance_squared(point.pos, to);
        if distance < best_distance {
            best = index;
            best_distance = distance;
        }
    }
    best
}

fn distance_squared(a: Vec2, b: Vec2) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

fn breadth_first(waypoints: &[Waypoint], start: usize, end: usize) -> Option<Vec<usize>> {
    let count = waypoints.len();
    if start >= count || end >= count {
        return None;
    }
    if start == end {
        return Some(vec![start]);
    }
    let mut seen = vec![false; count];
    let mut came_from = vec![usize::MAX; count];
    let mut queue = Vec::new();
    seen[start] = true;
    came_from[start] = start;
    queue.push(start);
    let mut head = 0;
    while head < queue.len() {
        let node = queue[head];
        head += 1;
        if node == end {
            break;
        }
        let point = &waypoints[node];
        let links = usize::from(point.link_count).min(point.links.len());
        for slot in 0..links {
            let next = usize::from(point.links[slot]);
            if next >= count || seen[next] {
                continue;
            }
            seen[next] = true;
            came_from[next] = node;
            queue.push(next);
        }
    }
    if !seen[end] {
        return None;
    }
    let mut path = Vec::new();
    let mut cursor = end;
    path.push(cursor);
    while cursor != start {
        cursor = came_from[cursor];
        if cursor == usize::MAX || path.len() > count {
            return None;
        }
        path.push(cursor);
    }
    path.reverse();
    Some(path)
}

/// The waypoint a bot should head for next on its way to `goal`.
///
/// Linked graphs use [`route`]. With no connections, the one nearest the goal that the bot can
/// plausibly reach wins, and with no waypoints at all it simply walks at the goal.
pub fn next_waypoint(waypoints: &[Waypoint], from: Vec2, goal: Vec2) -> Option<Vec2> {
    if waypoints.is_empty() {
        return None;
    }
    if waypoints.iter().any(|point| point.link_count > 0) {
        return route(waypoints, from, goal).map(|step| step.pos);
    }
    let distance = |a: Vec2, b: Vec2| {
        let dx = a.x - b.x;
        let dy = a.y - b.y;
        (dx * dx + dy * dy).sqrt()
    };
    let direct = distance(from, goal);
    waypoints
        .iter()
        // Only a waypoint that actually gets it closer is worth the detour.
        .filter(|point| distance(point.pos, goal) < direct)
        .min_by(|left, right| distance(from, left.pos).total_cmp(&distance(from, right.pos)))
        .map(|point| point.pos)
}
