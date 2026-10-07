use std::collections::BTreeMap;

use crate::collision::{nudge, segment_circle_t};
use crate::weapons::{
    armed_def, bounce_velocity, can_damage, cluster_submunitions, collides_with_bodies,
    degraded_hit_multiply, direct_damage, dropped_weapon, gravity_multiplier, grenade_is_armed,
    push_impulse, pushes_objects, ricochet_velocity, should_bink, splash_hits, surface_response,
    thrown_knife, BulletStyle, PendingHit, Projectile, ProjectileKind, SurfaceResponse, WeaponKind,
    EXPLOSION_FLAG_PUSH, OBJECT_PUSH_MULTIPLIER, PROJECTILE_GRAVITY,
};
use crate::{
    apply_impulse, bink_on_hit, BodyRegion, CollisionMask, DamageCause, DamageEvent, Event,
    Impulse, ImpulseSource, Vec2, World, DT, HEIGHT, WIDTH,
};

pub(crate) fn fly(world: &mut World, newborns: &mut Vec<Projectile>) {
    let teams: BTreeMap<u32, u8> = world
        .players
        .iter()
        .map(|(&id, player)| (id, player.team))
        .collect();
    let weapons = world.weapons.clone();
    let friendly_fire = world.friendly_fire;
    let mut hits = Vec::new();
    let mut knife_recoveries = Vec::new();
    // Where a cluster grenade went off, so its pieces can be scattered once the loop is done.
    let mut cluster_pieces: Vec<(u32, Vec2, Vec2)> = Vec::new();
    // Where a blast went off, so loose flags can be shoved once the loop releases them.
    let mut blast_pushes: Vec<(Vec2, f32)> = Vec::new();
    let mut terrain_impacts: Vec<(Vec2, Vec2)> = Vec::new();
    {
        let collision = &world.collision;
        let players = &world.players;
        let objects = &mut world.objects;
        let pickups = &mut world.pickups;
        let objectives = &mut world.objectives;
        world.projectiles.retain_mut(|bullet| {
            let previous_pos = bullet.pos;
            let style = bullet
                .weapon
                .map_or(BulletStyle::Plain, |kind| weapons.get(kind).bullet_style);
            let pull = gravity_multiplier(style);
            if pull > 0.0 {
                bullet.vel.y += PROJECTILE_GRAVITY * pull * DT;
            }
            bullet.pos = bullet.pos.add(bullet.vel.scale(DT));
            bullet.ttl = bullet.ttl.saturating_sub(1);
            if bullet.ttl == 0
                || bullet.pos.x < 0.0
                || bullet.pos.x > WIDTH
                || bullet.pos.y < 0.0
                || bullet.pos.y > HEIGHT
            {
                if bullet.explosive {
                    splash_hits(bullet, players, &teams, &weapons, friendly_fire, &mut hits);
                    blast_pushes.push((bullet.pos, bullet.splash_radius));
                    if matches!(style, BulletStyle::ClusterGrenade) {
                        cluster_pieces.push((bullet.owner, bullet.pos, bullet.vel));
                    }
                }
                if thrown_knife(bullet) {
                    knife_recoveries.push(bullet.pos);
                }
                return false;
            }
            let Some(owner_team) = teams.get(&bullet.owner) else {
                return false;
            };
            let terrain = collision
                .raycast(previous_pos, bullet.pos, CollisionMask::BULLET)
                .filter(|_| grenade_is_armed(&armed_def(&weapons, bullet), bullet.ttl));
            let platform_hit = terrain.map(|hit| hit.time);
            let mut player_hit: Option<(f32, u32)> = None;
            for target in players.values() {
                if target.hp <= 0 || target.id == bullet.owner {
                    continue;
                }
                if !collides_with_bodies(style, bullet.ttl) {
                    continue;
                }
                let same_team = *owner_team != 0 && *owner_team == target.team;
                let allowed = if let Some(kind) = bullet.weapon {
                    can_damage(
                        weapons.get(kind),
                        bullet.owner,
                        target.id,
                        same_team,
                        friendly_fire,
                        false,
                    )
                } else {
                    target.id != bullet.owner && (*owner_team == 0 || *owner_team != target.team)
                };
                if !allowed {
                    continue;
                }
                if let Some(t) = segment_circle_t(previous_pos, bullet.pos, target.pos, 18.0) {
                    if player_hit.is_none_or(|(best, _)| t < best) {
                        player_hit = Some((t, target.id));
                    }
                }
            }
            // Flags, kits, and dropped guns are knocked about by solid rounds passing through
            // them. The shove does not stop the round; it only moves the object.
            if pushes_objects(style) {
                if let Some(def) = bullet.weapon.map(|kind| *weapons.get(kind)) {
                    let shove =
                        push_impulse(&def, bullet.vel.scale(DT)).scale(OBJECT_PUSH_MULTIPLIER / DT);
                    for object in objects.iter_mut().filter(|object| object.active) {
                        if segment_circle_t(previous_pos, bullet.pos, object.pos, object.radius)
                            .is_some()
                        {
                            object.vel = object.vel.add(shove);
                            object.grounded = false;
                        }
                    }
                    // Kits are shoved by the same round.
                    for kit in &mut pickups.items {
                        if segment_circle_t(previous_pos, bullet.pos, kit.body.pos, kit.body.radius)
                            .is_some()
                        {
                            kit.push(shove);
                        }
                    }
                    // Flags are shoved by the same round, which is what lets a team blast a
                    // dropped flag away from the enemy standing over it.
                    for flag in &mut objectives.flags {
                        if flag.state.carrier().is_none()
                            && segment_circle_t(
                                previous_pos,
                                bullet.pos,
                                flag.body.pos,
                                flag.body.radius,
                            )
                            .is_some()
                        {
                            flag.push(shove);
                        }
                    }
                }
            }
            let player_t = player_hit.map(|(t, _)| t);
            let impact_t = match (platform_hit, player_t) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };
            if let Some(t) = impact_t {
                bullet.pos = Vec2 {
                    x: previous_pos.x + (bullet.pos.x - previous_pos.x) * t,
                    y: previous_pos.y + (bullet.pos.y - previous_pos.y) * t,
                };
                // Terrain does not end every shot. A grenade bounces off it, a rocket reached
                // from far enough away skips along it, an arrow buries itself in it, and a flame
                // gutters out against it. Only once terrain is done with the projectile does it
                // get to wound anybody or go off.
                let struck_terrain =
                    platform_hit.is_some_and(|wall_t| wall_t <= player_t.unwrap_or(f32::INFINITY));
                if struck_terrain {
                    if let Some(hit) = terrain {
                        // Every meeting of round and wall is worth a spark, whatever the round then
                        // does about it — bounce, stick, ricochet, or stop.
                        terrain_impacts.push((bullet.pos, hit.normal));
                        match surface_response(style) {
                            SurfaceResponse::Bounce { restitution } => {
                                bullet.vel = bounce_velocity(bullet.vel, hit.normal, restitution);
                                bullet.pos = nudge(bullet.pos, hit.normal);
                                bullet.last_impact = bullet.pos;
                                return true;
                            }
                            SurfaceResponse::Stick { resist_ticks } => {
                                bullet.vel = Vec2::default();
                                bullet.ttl = bullet.ttl.min(resist_ticks);
                                bullet.pos = nudge(bullet.pos, hit.normal);
                                bullet.last_impact = bullet.pos;
                                return bullet.ttl > 0;
                            }
                            SurfaceResponse::Smother { ticks } => {
                                bullet.ttl = bullet.ttl.min(ticks);
                                return bullet.ttl > 0;
                            }
                            SurfaceResponse::Ricochet { min_travel }
                                if bullet.pos.add(bullet.last_impact.scale(-1.0)).length()
                                    > min_travel =>
                            {
                                bullet.vel = ricochet_velocity(bullet.vel, hit.normal);
                                bullet.pos = nudge(bullet.pos, hit.normal);
                                bullet.last_impact = bullet.pos;
                                return true;
                            }
                            SurfaceResponse::Stop
                            | SurfaceResponse::Explode
                            | SurfaceResponse::Ricochet { .. } => {}
                        }
                    }
                }
                if bullet.explosive {
                    splash_hits(bullet, players, &teams, &weapons, friendly_fire, &mut hits);
                    blast_pushes.push((bullet.pos, bullet.splash_radius));
                    if matches!(style, BulletStyle::ClusterGrenade) {
                        cluster_pieces.push((bullet.owner, bullet.pos, bullet.vel));
                    }
                } else if player_t
                    .is_some_and(|hit_t| hit_t <= platform_hit.unwrap_or(f32::INFINITY))
                {
                    let target_id = player_hit.unwrap().1;
                    let region = players.get(&target_id).map_or(BodyRegion::Chest, |target| {
                        crate::character::body::shape_for(target.state)
                            .region_at(target.pos, bullet.pos)
                    });
                    let (damage, impulse, pre_scaled) = if let Some(kind) = bullet.weapon {
                        let def = weapons.get(kind);
                        let soldat_speed = bullet.vel.length() * DT;
                        let distance = (bullet.pos.add(bullet.origin.scale(-1.0))).length();
                        let amount = direct_damage(
                            def,
                            degraded_hit_multiply(def, distance),
                            soldat_speed,
                            region,
                        )
                        .round() as i32;
                        (
                            amount,
                            Impulse {
                                velocity: push_impulse(def, bullet.vel.scale(DT)),
                                source: ImpulseSource::Bullet,
                            },
                            true,
                        )
                    } else {
                        let direction_length = bullet.vel.length().max(1.0);
                        (
                            bullet.damage,
                            Impulse {
                                velocity: Vec2 {
                                    x: bullet.vel.x / direction_length * 8.0,
                                    y: bullet.vel.y / direction_length * 8.0,
                                },
                                source: ImpulseSource::Bullet,
                            },
                            false,
                        )
                    };
                    hits.push(PendingHit {
                        killer: bullet.owner,
                        target: target_id,
                        damage,
                        impulse,
                        region,
                        cause: match bullet.kind {
                            ProjectileKind::Melee => DamageCause::Melee,
                            ProjectileKind::ShotgunPellet => DamageCause::Pellet,
                            _ => DamageCause::Bullet,
                        },
                        pre_scaled,
                        bink_weapon: bullet.weapon,
                    });
                } else if thrown_knife(bullet) {
                    knife_recoveries.push(bullet.pos);
                }
                return false;
            }
            true
        });
    }
    world.projectiles.append(newborns);
    for (pos, normal) in terrain_impacts {
        world.events.push(Event::Impact { pos, normal });
    }
    for &(at, radius) in &blast_pushes {
        world.events.push(Event::Explosion {
            pos: at,
            radius: radius.max(1.0),
        });
    }
    for (at, radius) in blast_pushes {
        world.pickups.push_near(
            at,
            radius,
            Vec2 {
                x: 0.0,
                y: -EXPLOSION_FLAG_PUSH * 0.5,
            },
        );
        // A blast throws a loose flag away from itself, hardest at the centre.
        for flag in &mut world.objectives.flags {
            if flag.state.carrier().is_some() {
                continue;
            }
            let away = Vec2 {
                x: flag.body.pos.x - at.x,
                y: flag.body.pos.y - at.y,
            };
            let distance = away.length();
            if distance >= radius || radius <= 0.0 {
                continue;
            }
            let scale = (1.0 - distance / radius) * EXPLOSION_FLAG_PUSH;
            let direction = if distance > f32::EPSILON {
                Vec2 {
                    x: away.x / distance,
                    y: away.y / distance,
                }
            } else {
                Vec2 { x: 0.0, y: -1.0 }
            };
            flag.push(direction.scale(scale));
        }
    }
    for (owner, pos, velocity) in cluster_pieces {
        let piece = *world.weapons.get(WeaponKind::Cluster);
        for scatter in cluster_submunitions(velocity.scale(DT), &mut world.rng) {
            world.projectiles.push(Projectile {
                id: world.next_projectile,
                owner,
                pos,
                vel: scatter.scale(1.0 / DT),
                ttl: piece.timeout(),
                damage: 0,
                explosive: true,
                kind: ProjectileKind::FragGrenade,
                splash_radius: piece.explosion_radius(),
                weapon: Some(WeaponKind::Cluster),
                origin: pos,
                last_impact: pos,
            });
            world.next_projectile = world.next_projectile.wrapping_add(1);
        }
    }
    for pos in knife_recoveries {
        world.objects.push(dropped_weapon(
            WeaponKind::CombatKnife,
            1,
            pos,
            Vec2::default(),
        ));
    }
    for hit in hits {
        if let Some(target) = world.players.get_mut(&hit.target) {
            if target.hp <= 0 || target.spawn_protection > 0 {
                continue;
            }
            apply_impulse(target, hit.impulse);
            if let Some(kind) = hit.bink_weapon {
                let victim_id = target.id;
                let same_team = {
                    let attacker_team = teams.get(&hit.killer).copied().unwrap_or(0);
                    attacker_team != 0 && attacker_team == target.team
                };
                if should_bink(
                    world.weapons.get(kind),
                    hit.killer,
                    victim_id,
                    same_team,
                    world.friendly_fire,
                ) {
                    target.bink = bink_on_hit(world.weapons.get(kind), target.bink);
                }
            }
        }
        let direction = hit.impulse.velocity;
        world.apply_damage(DamageEvent {
            attacker: Some(hit.killer),
            target: hit.target,
            amount: hit.damage,
            region: hit.region,
            cause: hit.cause,
            direction,
            pre_scaled: hit.pre_scaled,
        });
    }
}
