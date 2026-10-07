use std::collections::BTreeMap;

use super::{can_damage, explosion_damage, explosion_impulse, WeaponKind, WeaponTable};
use crate::{BodyRegion, DamageCause, Impulse, ImpulseSource, Player, Projectile, Vec2};

#[derive(Clone, Copy)]
pub(crate) struct PendingHit {
    pub(crate) killer: u32,
    pub(crate) target: u32,
    pub(crate) damage: i32,
    pub(crate) impulse: Impulse,
    pub(crate) region: BodyRegion,
    pub(crate) cause: DamageCause,
    pub(crate) pre_scaled: bool,
    pub(crate) bink_weapon: Option<WeaponKind>,
}

/// How hard a blast throws a loose flag, in world units per second at the centre of the blast.
pub(crate) const EXPLOSION_FLAG_PUSH: f32 = 260.0;

pub(crate) fn splash_hits(
    bullet: &Projectile,
    players: &BTreeMap<u32, Player>,
    teams: &BTreeMap<u32, u8>,
    weapons: &WeaponTable,
    friendly_fire: bool,
    hits: &mut Vec<PendingHit>,
) {
    for target in players.values() {
        if target.hp <= 0 {
            continue;
        }
        let owner_team = teams.get(&bullet.owner).copied().unwrap_or(0);
        let same_team = owner_team != 0 && owner_team == target.team;
        let allowed = if let Some(kind) = bullet.weapon {
            can_damage(
                weapons.get(kind),
                bullet.owner,
                target.id,
                same_team,
                friendly_fire,
                true,
            )
        } else {
            target.id == bullet.owner || owner_team == 0 || owner_team != target.team
        };
        if !allowed {
            continue;
        }
        let dx = target.pos.x - bullet.pos.x;
        let dy = target.pos.y - bullet.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        if distance >= bullet.splash_radius {
            continue;
        }
        let scale = 1.0 - distance / bullet.splash_radius;
        let normal = if distance > f32::EPSILON {
            Vec2 {
                x: dx / distance,
                y: dy / distance,
            }
        } else {
            Vec2 { x: 0.0, y: -1.0 }
        };
        let region = BodyRegion::Chest;
        let (damage, impulse, pre_scaled) = if let Some(kind) = bullet.weapon {
            let def = weapons.get(kind);
            (
                explosion_damage(def, distance, region).round() as i32,
                Impulse {
                    velocity: explosion_impulse(def, normal, distance),
                    source: ImpulseSource::Explosion,
                },
                true,
            )
        } else {
            (
                (scale * bullet.damage as f32).ceil() as i32,
                Impulse {
                    velocity: Vec2 {
                        x: normal.x * 90.0 * scale,
                        y: normal.y * 90.0 * scale,
                    },
                    source: ImpulseSource::Explosion,
                },
                false,
            )
        };
        hits.push(PendingHit {
            killer: bullet.owner,
            target: target.id,
            damage,
            impulse,
            region,
            cause: DamageCause::Explosion,
            pre_scaled,
            bink_weapon: bullet.weapon,
        });
    }
}
