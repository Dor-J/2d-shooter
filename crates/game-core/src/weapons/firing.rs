//! The per-weapon rules that decide whether a shot happens and what it does to the shooter.
//!
//! These are the handful of behaviors upstream special-cases by weapon number in
//! `shared/mechanics/Sprites.pas` (`TSprite.Fire`). Each one is stated once here and keyed off the
//! weapon's own identity, so the firing loop stays a single path.

use serde::{Deserialize, Serialize};

use super::accuracy::ShooterPose;
use super::config::{BulletStyle, WeaponDef, WeaponKind};
use crate::{CharacterState, Vec2};

/// How the LAW's launcher may be braced.
///
/// It is a shoulder-fired rocket: upstream refuses the shot outright unless the firer is on the
/// ground and either crouching or lying down.
pub fn law_is_braced(pose: &ShooterPose) -> bool {
    pose.grounded
        && matches!(
            pose.state,
            CharacterState::Crouching | CharacterState::Prone
        )
}

/// Whether this weapon is allowed to fire from this pose at all.
pub fn may_fire(def: &WeaponDef, pose: &ShooterPose) -> bool {
    if def.kind == WeaponKind::Law {
        return law_is_braced(pose);
    }
    true
}

/// Why a shot was refused, so the client can say so rather than silently eating the trigger.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FireRefusal {
    /// The LAW needs a braced stance.
    NeedsBracing,
}

/// The reason a weapon refused to fire from this pose, if it refused.
pub fn refusal(def: &WeaponDef, pose: &ShooterPose) -> Option<FireRefusal> {
    if def.kind == WeaponKind::Law && !law_is_braced(pose) {
        return Some(FireRefusal::NeedsBracing);
    }
    None
}

/// Whether the weapon has to be held down before its first shot, and for how long.
///
/// The Barrett takes time to settle on target and the minigun has to spin up; both are the
/// weapon's own configured start-up rather than a rule about those two weapons.
pub fn startup_ticks(def: &WeaponDef) -> u16 {
    def.start_up_time
}

/// Whether letting go of the trigger loses the spin-up a minigun has built.
pub fn startup_resets_on_release(def: &WeaponDef) -> bool {
    def.start_up_time > 0 && def.kind != WeaponKind::Barrett
}

/// The shove the weapon gives its own firer, in world units per tick.
///
/// The SPAS and the minigun are strong enough to move the shooter, which players ride deliberately;
/// everything else nudges back along the barrel.
pub fn self_boost(def: &WeaponDef, aim_direction: Vec2, pose: &ShooterPose) -> Vec2 {
    match def.kind {
        WeaponKind::Spas12 => Vec2 {
            x: -aim_direction.x * def.speed * 0.0412,
            y: -aim_direction.y * def.speed * 0.041,
        },
        WeaponKind::Minigun => {
            // Jetting bleeds most of the kick away, and the x component is always gentler.
            let (scale_x, scale_y) = if pose.jetting {
                (0.0012, 0.0009)
            } else {
                (0.0082, 0.0078)
            };
            Vec2 {
                x: -aim_direction.x * def.speed * scale_x * 0.6,
                y: -aim_direction.y * def.speed * scale_y,
            }
        }
        _ => Vec2 { x: 0.0, y: 0.0 },
    }
}

/// Whether the weapon shoves its firer hard enough to be worth reporting as a boost.
pub fn boosts_the_shooter(def: &WeaponDef) -> bool {
    matches!(def.kind, WeaponKind::Spas12 | WeaponKind::Minigun)
}

/// The sideways offset of the second Desert Eagle's muzzle, perpendicular to the aim.
///
/// Both barrels fire on the same trigger pull, from two points three units apart.
pub const EAGLE_BARREL_OFFSET: f32 = 3.0;

/// The muzzle of pellet number `pellet`, offset for a weapon that has more than one barrel.
pub fn barrel_origin(def: &WeaponDef, muzzle: Vec2, aim_direction: Vec2, pellet: u8) -> Vec2 {
    if def.kind != WeaponKind::DesertEagles || pellet == 0 {
        return muzzle;
    }
    Vec2 {
        x: muzzle.x - aim_direction.x.signum() * aim_direction.y.abs() * EAGLE_BARREL_OFFSET,
        y: muzzle.y + aim_direction.y.signum() * aim_direction.x.abs() * EAGLE_BARREL_OFFSET,
    }
}

/// How far each projectile of one trigger pull is thrown off the aim, before general inaccuracy.
///
/// A shotgun scatters its pellets and dual Eagles nudge each barrel apart; everything else fires
/// down the line it was aimed along.
pub fn per_pellet_spread(def: &WeaponDef, pellet: u8, roll: f32) -> f32 {
    let scatters = def.bullet_style == BulletStyle::Shotgun
        || (def.kind == WeaponKind::DesertEagles && pellet > 0);
    if !scatters {
        return 0.0;
    }
    (roll * 2.0 - 1.0) * def.bullet_spread
}
