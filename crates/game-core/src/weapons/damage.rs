//! What a hit is worth.
//!
//! Follows `shared/mechanics/Bullets.pas` at the pinned commit: a direct hit is
//! `|velocity| * Damage * HitboxModifier`, an explosion is `Damage * HitboxModifier / (distance + 1)`,
//! and a bullet halves its power past 500 and again past 900 world units from where it was fired.
//! Upstream runs no line-of-sight check before applying splash damage, so neither do we; the
//! difference is deliberate and recorded on the gap line.

use super::config::{BulletStyle, WeaponDef};
use crate::{BodyRegion, Vec2};

/// The two distances at which a bullet halves its power, and the interval at which the check runs.
pub const FIRST_DEGRADE_DISTANCE: f32 = 500.0;
pub const SECOND_DEGRADE_DISTANCE: f32 = 900.0;
pub const DEGRADE_CHECK_TICKS: u16 = 6;

/// How hard an explosion shoves a body, from `shared/Constants.pas`.
pub const EXPLOSION_IMPACT_MULTIPLY: f32 = 3.75;

/// Muzzle velocity for one shot: the weapon's own speed plus the share of the shooter's velocity
/// the weapon inherits. Moving into the shot speeds the projectile up and so raises its damage;
/// backing away slows it down.
pub fn muzzle_velocity(def: &WeaponDef, direction: Vec2, shooter_velocity: Vec2) -> Vec2 {
    let length = (direction.x * direction.x + direction.y * direction.y).sqrt();
    let unit = if length > f32::EPSILON {
        Vec2 {
            x: direction.x / length,
            y: direction.y / length,
        }
    } else {
        Vec2 { x: 1.0, y: 0.0 }
    };
    Vec2 {
        x: unit.x * def.speed + shooter_velocity.x * def.inherited_velocity,
        y: unit.y * def.speed + shooter_velocity.y * def.inherited_velocity,
    }
}

/// Damage for a direct hit, given the projectile's speed in world units per tick where it landed.
///
/// Most styles scale with the speed the projectile still carries, which is what makes a distant or
/// a slowed shot weaker. A flame does not: it burns for the same amount however fast it arrived.
/// An arrow scales with speed but counts for a hundredth, because its configured damage is stated
/// on a different scale.
pub fn direct_damage(def: &WeaponDef, hit_multiply: f32, speed: f32, region: BodyRegion) -> f32 {
    let modifier = def.region_modifier(region);
    match def.bullet_style {
        BulletStyle::Flame => hit_multiply * modifier,
        BulletStyle::Arrow => speed * hit_multiply * 0.01 * modifier,
        _ => speed * hit_multiply * modifier,
    }
}

/// The power a projectile still carries after travelling `distance` from its origin.
///
/// Barrett, M79, knife, and LAW never lose power; everything else halves once past 500 units and
/// again past 900.
pub fn degraded_hit_multiply(def: &WeaponDef, distance: f32) -> f32 {
    if !def.degrades_over_distance() {
        return def.damage;
    }
    let mut multiply = def.damage;
    if distance > FIRST_DEGRADE_DISTANCE {
        multiply *= 0.5;
    }
    if distance > SECOND_DEGRADE_DISTANCE {
        multiply *= 0.5;
    }
    multiply
}

/// Damage for a body caught in a blast. Cluster and flak fragments count half.
pub fn explosion_damage(def: &WeaponDef, distance: f32, region: BodyRegion) -> f32 {
    let radius = def.explosion_radius();
    if radius <= 0.0 || distance >= radius {
        return 0.0;
    }
    let mut modifier = def.region_modifier(region);
    if matches!(def.bullet_style, BulletStyle::Cluster) {
        modifier *= 0.5;
    }
    (1.0 / (distance + 1.0)) * def.damage * modifier
}

/// The shove a blast gives a body. Frag and M79 blasts lift twice as hard as they push sideways.
pub fn explosion_impulse(def: &WeaponDef, from_blast: Vec2, distance: f32) -> Vec2 {
    let scale = (1.0 / (distance + 1.0)) * EXPLOSION_IMPACT_MULTIPLY;
    let vertical = if matches!(
        def.bullet_style,
        BulletStyle::FragGrenade | BulletStyle::M79Grenade
    ) {
        2.0
    } else {
        1.0
    };
    Vec2 {
        x: from_blast.x * scale,
        y: from_blast.y * scale * vertical,
    }
}

/// The shove a direct hit gives the body it lands on, scaled by the bullet's mass.
///
/// Grenades, flames, and arrows pass their momentum on through their own payload instead.
pub fn push_impulse(def: &WeaponDef, bullet_velocity: Vec2) -> Vec2 {
    if matches!(
        def.bullet_style,
        BulletStyle::FragGrenade | BulletStyle::Flame | BulletStyle::Arrow
    ) {
        return Vec2 { x: 0.0, y: 0.0 };
    }
    Vec2 {
        x: bullet_velocity.x * def.push,
        y: bullet_velocity.y * def.push,
    }
}

/// Whether a projectile from `attacker` may damage `victim` at all.
///
/// Self-damage always counts, a solo match has no teams to protect, and teammates are only spared
/// when friendly fire is off. A weapon's `NoCollision` bits override all of it.
pub fn can_damage(
    def: &WeaponDef,
    attacker: u32,
    victim: u32,
    same_team: bool,
    friendly_fire: bool,
    explosion: bool,
) -> bool {
    let no_collision = def.no_collision;
    let allowed_by_weapon = if attacker == victim {
        if explosion {
            no_collision.explodes_on_self()
        } else {
            no_collision.collides_with_self()
        }
    } else if same_team {
        if explosion {
            no_collision.explodes_on_team()
        } else {
            no_collision.collides_with_team()
        }
    } else if explosion {
        no_collision.explodes_on_enemy()
    } else {
        no_collision.collides_with_enemy()
    };

    if !allowed_by_weapon {
        return false;
    }
    if attacker == victim {
        return true;
    }
    !same_team || friendly_fire
}

/// Whether a hit should bink its victim. A teammate's shot only binks when friendly fire is on,
/// which keeps a team from blinding each other for free.
pub fn should_bink(
    def: &WeaponDef,
    attacker: u32,
    victim: u32,
    same_team: bool,
    friendly_fire: bool,
) -> bool {
    def.binks_the_victim() && (attacker == victim || !same_team || friendly_fire)
}
