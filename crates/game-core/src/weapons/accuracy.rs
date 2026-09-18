//! Where a shot actually goes: bink, movement accuracy, bullet spread, and recoil.
//!
//! Follows `shared/mechanics/Sprites.pas` (`TSprite.Fire`, `TSprite.GetMoveacc`) and
//! `shared/Weapons.pas` (`CalculateBink`) at the pinned commit. Upstream computes bink only for
//! the local player; the server owns it here so bots and remote players obey the same rule.

use super::config::{WeaponDef, WeaponKind};
use crate::CharacterState;

/// The hard ceiling on aim deviation, from `shared/Constants.pas`.
pub const MAX_INACCURACY: f32 = 0.5;

/// What the shooter is doing at the moment of firing.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShooterPose {
    pub state: CharacterState,
    pub grounded: bool,
    /// Running, rolling, or otherwise moving under power.
    pub moving: bool,
    /// Jetting spoils the aim as much as running does.
    pub jetting: bool,
    /// Accumulated bink, in the same units `CalculateBink` produces.
    pub bink: u16,
}

impl ShooterPose {
    fn crouched(&self) -> bool {
        matches!(self.state, CharacterState::Crouching)
    }
    fn prone(&self) -> bool {
        matches!(self.state, CharacterState::Prone)
    }
    fn airborne(&self) -> bool {
        !self.grounded && !self.prone() && !self.crouched()
    }
}

/// Adds bink with diminishing returns, exactly as `CalculateBink` does.
pub fn calculate_bink(accumulated: u16, bink: u16) -> u16 {
    if bink == 0 {
        return accumulated;
    }
    let accumulated_f = f32::from(accumulated);
    let bink_f = f32::from(bink);
    let decay = (accumulated_f * (accumulated_f / ((10.0 * bink_f) + accumulated_f))).round();
    let total = accumulated_f + bink_f - decay;
    total.clamp(0.0, f32::from(u16::MAX)) as u16
}

/// The bink a victim accumulates when this weapon hits them, if it binks at all.
pub fn bink_on_hit(def: &WeaponDef, accumulated: u16) -> u16 {
    if def.bink > 0 {
        calculate_bink(accumulated, def.bink.unsigned_abs())
    } else {
        accumulated
    }
}

/// The bink the shooter gives themselves by firing. Crouching or lying down halves it.
pub fn self_bink_on_fire(def: &WeaponDef, accumulated: u16, pose: &ShooterPose) -> u16 {
    if def.bink >= 0 {
        return accumulated;
    }
    let magnitude = def.bink.unsigned_abs();
    let magnitude = if pose.crouched() || pose.prone() {
        // Upstream rounds the halved value before accumulating it.
        (f32::from(magnitude) / 2.0).round() as u16
    } else {
        magnitude
    };
    calculate_bink(accumulated, magnitude)
}

/// How much movement spoils the aim: sevenfold while running or jetting, threefold in the air or
/// mid-transition, nothing while set.
pub fn movement_accuracy(def: &WeaponDef, pose: &ShooterPose) -> f32 {
    if def.movement_acc <= 0.0 {
        return 0.0;
    }
    if pose.jetting || (pose.moving && !pose.prone() && !pose.crouched()) {
        def.movement_acc * 7.0
    } else if pose.airborne()
        || matches!(
            pose.state,
            CharacterState::GettingUp { .. } | CharacterState::GoingProne { .. }
        )
    {
        def.movement_acc * 3.0
    } else {
        0.0
    }
}

/// The base spread, tightened by crouching and tightened further by lying down. Eagles and
/// shotguns apply their spread per pellet instead, so they contribute nothing here.
pub fn stance_spread(def: &WeaponDef, pose: &ShooterPose) -> f32 {
    if def.bullet_spread <= 0.0
        || def.kind == WeaponKind::DesertEagles
        || def.pellets() > 1
        || def.bullet_style.pellets() > 1
    {
        return 0.0;
    }
    if pose.prone() {
        def.bullet_spread / 1.625
    } else if pose.crouched() {
        def.bullet_spread / 1.3
    } else {
        def.bullet_spread
    }
}

/// The total inaccuracy of one shot, before it is turned into a deviation.
pub fn inaccuracy(def: &WeaponDef, pose: &ShooterPose) -> f32 {
    let total =
        f32::from(pose.bink) * 0.01 + movement_accuracy(def, pose) + stance_spread(def, pose);
    // Upstream scales the sum down by four because the aim vector is already normalised.
    (total * 0.25).min(MAX_INACCURACY)
}

/// The largest angle-free offset a shot may take, scaled so it approaches the ceiling smoothly.
pub fn max_deviation(inaccuracy: f32) -> f32 {
    let clamped = inaccuracy.clamp(0.0, MAX_INACCURACY);
    MAX_INACCURACY * ((clamped / MAX_INACCURACY) * std::f32::consts::FRAC_PI_2).sin()
}

/// How far the cursor climbs after `burst` consecutive shots, in radians. Crouching halves it and
/// lying down cuts it to a third.
pub fn recoil_radians(def: &WeaponDef, burst: u16, pose: &ShooterPose) -> f32 {
    if def.recoil == 0 {
        return 0.0;
    }
    let mut strength = f32::from(burst) / 10.0 * f32::from(def.recoil);
    if pose.grounded {
        if pose.crouched() {
            strength /= 2.0;
        } else if pose.prone() {
            strength /= 3.0;
        }
    }
    if strength <= 0.0 {
        return 0.0;
    }
    let degrees = def.speed * f32::from(def.fire_interval) / 364.0 * strength;
    degrees.to_radians().sin()
}
