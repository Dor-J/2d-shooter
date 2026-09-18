//! Watching a match rather than playing it.
//!
//! Who a spectator may follow, where a free camera may go, and how long a competitive broadcast is
//! held back are all server decisions. A client that is simply told "you are spectating" and left
//! to police itself is a client that can be modified into a wallhack, so every restriction here is
//! answered on the authoritative side.

use serde::{Deserialize, Serialize};

use super::modifiers::realistic::{visibility_between, Viewer, Visibility};
use super::rules::ModeRules;
use super::team::{is_spectator, SPECTATOR};
use crate::collision::CollisionWorld;
use crate::Vec2;

/// How far a free camera may stray outside the map, so a spectator cannot scout the void.
pub const FREE_CAMERA_MARGIN: f32 = 64.0;

/// The longest a competitive broadcast may be held back, in ticks.
pub const MAX_BROADCAST_DELAY_TICKS: u32 = 60 * 120;

/// What a spectator is looking at.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpectatorView {
    /// Following one player's camera.
    #[default]
    Following,
    /// Flying the camera about freely.
    FreeCamera,
}

/// One spectator's state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Spectator {
    pub view: SpectatorView,
    /// The player being followed, when following one.
    pub target: Option<u32>,
    /// Where a free camera is pointing.
    pub camera: Vec2,
}

impl Spectator {
    /// A spectator who has just joined and is following whoever is available.
    pub fn new(target: Option<u32>) -> Self {
        Self {
            view: SpectatorView::Following,
            target,
            camera: Vec2::default(),
        }
    }

    /// Whether this spectator is flying the camera rather than following somebody.
    pub const fn is_free(&self) -> bool {
        matches!(self.view, SpectatorView::FreeCamera)
    }
}

/// How a spectator asked to change what they are watching.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpectateCommand {
    /// Follow the next player in the list.
    Next,
    /// Follow the previous one.
    Previous,
    /// Follow one particular player.
    Follow(u32),
    /// Let go and fly the camera.
    FreeCamera,
}

/// The players a spectator may follow, lowest id first.
///
/// Spectators are not in the list: watching somebody who is themselves watching would show an
/// empty screen.
pub fn followable(candidates: impl Iterator<Item = (u32, u8)>) -> Vec<u32> {
    let mut ids: Vec<u32> = candidates
        .filter(|(_, team)| !is_spectator(*team))
        .map(|(id, _)| id)
        .collect();
    ids.sort_unstable();
    ids
}

/// Applies one spectate command, given who there is to watch.
///
/// A command that names somebody who is not there, or asks to cycle when there is nobody at all,
/// leaves the spectator where they were rather than pointing the camera at nothing.
pub fn apply(spectator: &mut Spectator, command: SpectateCommand, followable: &[u32]) {
    match command {
        SpectateCommand::FreeCamera => {
            spectator.view = SpectatorView::FreeCamera;
            spectator.target = None;
        }
        SpectateCommand::Follow(id) => {
            if followable.contains(&id) {
                spectator.view = SpectatorView::Following;
                spectator.target = Some(id);
            }
        }
        SpectateCommand::Next | SpectateCommand::Previous => {
            if followable.is_empty() {
                return;
            }
            let step: isize = if matches!(command, SpectateCommand::Next) {
                1
            } else {
                -1
            };
            let current = spectator
                .target
                .and_then(|id| followable.iter().position(|candidate| *candidate == id));
            let len = followable.len() as isize;
            let next = match current {
                Some(index) => ((index as isize + step).rem_euclid(len)) as usize,
                // Coming off a free camera, Next starts at the top and Previous at the bottom.
                None if step > 0 => 0,
                None => followable.len() - 1,
            };
            spectator.view = SpectatorView::Following;
            spectator.target = Some(followable[next]);
        }
    }
}

/// Keeps a spectator following somebody who is still in the match.
///
/// A followed player who leaves hands the camera to the next one along rather than stranding it.
pub fn retarget_if_gone(spectator: &mut Spectator, followable: &[u32]) {
    if spectator.is_free() {
        return;
    }
    let still_there = spectator.target.is_some_and(|id| followable.contains(&id));
    if !still_there {
        spectator.target = followable.first().copied();
    }
}

/// Holds a free camera inside the map, with a little margin so the edges are reachable.
pub fn clamp_camera(camera: Vec2, width: f32, height: f32) -> Vec2 {
    Vec2 {
        x: camera
            .x
            .clamp(-FREE_CAMERA_MARGIN, width + FREE_CAMERA_MARGIN),
        y: camera
            .y
            .clamp(-FREE_CAMERA_MARGIN, height + FREE_CAMERA_MARGIN),
    }
}

/// Whether a spectator's chat reaches the players.
///
/// It does not: somebody watching the whole map could otherwise call out every position. They talk
/// among themselves instead.
pub const fn chat_reaches_players() -> bool {
    false
}

/// Whether two spectators can hear each other.
pub const fn chat_reaches_spectators() -> bool {
    true
}

/// What a spectator is allowed to see of `target`.
///
/// Following somebody in Realistic shows their view and nothing more, which is what stops
/// spectating from being a way around the mode. A free camera in Realistic sees nothing but the
/// map, for the same reason.
pub fn visibility_for(
    rules: &ModeRules,
    collision: &CollisionWorld,
    spectator: &Spectator,
    followed: Option<Viewer>,
    target: Viewer,
) -> Visibility {
    if !rules.modifiers.realistic {
        return Visibility::Full;
    }
    if spectator.is_free() {
        return Visibility::Hidden;
    }
    let Some(followed) = followed else {
        return Visibility::Hidden;
    };
    if followed.id == target.id {
        return Visibility::Full;
    }
    visibility_between(collision, followed, target)
}

/// How far behind live a broadcast should run.
///
/// Zero is live. A competitive match can be delayed so a spectator cannot relay positions to a
/// player, and the delay is capped so a server cannot be configured into never showing anything.
pub fn broadcast_delay(configured: u32) -> u32 {
    configured.min(MAX_BROADCAST_DELAY_TICKS)
}

/// The tick a delayed spectator should be shown, given the live tick.
pub fn delayed_tick(live: u64, delay: u32) -> u64 {
    live.saturating_sub(u64::from(broadcast_delay(delay)))
}

/// The team number a spectator is on.
pub const fn spectator_team() -> u8 {
    SPECTATOR
}
