use crate::fixtures::SimRng;
use crate::weapons::{
    aim_dir, barrel_origin, dropped_weapon, muzzle_origin, muzzle_velocity, nearest_index,
    per_pellet_spread, projectile_kind, refusal, self_bink_on_fire, self_boost, shooter_pose,
    sync_hands, writeback_ammo, WeaponDef, WeaponKind, MAX_THROW_CHARGE,
};
use crate::{
    apply_impulse, boosts_the_shooter, charged_velocity, inaccuracy, max_deviation, recoil_radians,
    throw_frame, throw_velocity, DynamicBody, Event, FireRefusal, Impulse, ImpulseSource, Input,
    Player, Projectile, ProjectileKind, StatEvent, Vec2, WeaponSlot, DT, THROW_LAST_FRAME,
};

pub(crate) fn trigger(
    player: &mut Player,
    input: Input,
    previous: Input,
    weapon: WeaponDef,
    events: &mut Vec<Event>,
    guns: &mut Vec<crate::StationaryGun>,
    next_projectile: &mut u32,
    objects: &mut Vec<DynamicBody>,
    projectiles: &mut Vec<Projectile>,
    rng: &mut SimRng,
    stats: &mut crate::MatchStats,
    weapons: &crate::WeaponTable,
) {
    if input.fire && player.switch_timer == 0 {
        player.startup = player.startup.saturating_add(1);
    } else if !input.fire {
        player.startup = 0;
    }
    let unarmed = player.inventory.active().is_none();
    if input.fire
        && player.switch_timer == 0
        && player.cooldown == 0
        && player.reload_timer == 0
        && player.ammo == 0
        && !unarmed
        && !previous.fire
    {
        events.push(Event::Empty { player: player.id });
    }
    if input.fire
        && player.switch_timer == 0
        && player.cooldown == 0
        && player.reload_timer == 0
        && (unarmed || player.ammo > 0)
        && player.startup > weapon.start_up_time
    {
        let dx = input.aim.x - player.pos.x;
        let dy = input.aim.y - player.pos.y;
        let len = (dx * dx + dy * dy).sqrt();
        let pose = shooter_pose(player, &input);
        if len > 1.0 && len.is_finite() && !crate::firing::may_fire(&weapon, &pose) {
            // Say so once per wind-up, on the tick the shot would otherwise have gone,
            // rather than every tick the trigger stays down.
            if player.startup == weapon.start_up_time.saturating_add(1) {
                events.push(Event::FireRefused {
                    player: player.id,
                    reason: refusal(&weapon, &pose).unwrap_or(FireRefusal::NeedsBracing),
                });
            }
        } else if len > 1.0 && len.is_finite() {
            player.bink = self_bink_on_fire(&weapon, player.bink, &pose);
            player.accuracy = inaccuracy(&weapon, &pose);
            let recoil = player.recoil_aim;
            let base_angle = dy.atan2(dx) + recoil;
            for pellet in 0..weapon.pellets() {
                let fan = per_pellet_spread(&weapon, pellet, rng.next_unit());
                let jitter = (rng.next_unit() * 2.0 - 1.0) * max_deviation(player.accuracy);
                let angle = base_angle + fan + jitter;
                let dir = Vec2 {
                    x: angle.cos(),
                    y: angle.sin(),
                };
                let muzzle = muzzle_velocity(&weapon, dir, player.vel.scale(DT));
                let world_vel = muzzle.scale(1.0 / DT);
                let barrel = barrel_origin(
                    &weapon,
                    muzzle_origin(player.pos, input.aim, player.state, player.facing),
                    dir,
                    pellet,
                );
                projectiles.push(Projectile {
                    id: *next_projectile,
                    owner: player.id,
                    pos: barrel,
                    vel: world_vel,
                    ttl: weapon.timeout(),
                    damage: 0,
                    explosive: weapon.bullet_style.is_explosive(),
                    kind: projectile_kind(&weapon),
                    splash_radius: weapon.explosion_radius(),
                    weapon: Some(weapon.kind),
                    origin: barrel,
                    last_impact: barrel,
                });
                *next_projectile = next_projectile.wrapping_add(1);
            }
            if !unarmed {
                player.ammo = player.ammo.saturating_sub(1);
                writeback_ammo(player);
            }
            player.cooldown = weapon.fire_interval;
            if !unarmed && player.ammo == 0 {
                player.reload_timer = weapon.reload_time;
            }
            let barrel = muzzle_origin(player.pos, input.aim, player.state, player.facing);
            events.push(Event::MuzzleFlash {
                player: player.id,
                pos: barrel,
            });
            events.push(Event::Casing {
                player: player.id,
                pos: Vec2 {
                    x: player.pos.x - (barrel.x - player.pos.x) * 0.3,
                    y: player.pos.y + 4.0,
                },
            });
            player.recoil_aim += recoil_radians(&weapon, player.burst, &pose);
            player.burst = player.burst.saturating_add(1);
            let aim_unit = Vec2 {
                x: dx / len,
                y: dy / len,
            };
            let source = match weapon.kind {
                WeaponKind::Spas12 => ImpulseSource::SpasBoost,
                WeaponKind::Minigun => ImpulseSource::MinigunBoost,
                _ => ImpulseSource::Recoil,
            };
            let boost = if boosts_the_shooter(&weapon) {
                // A SPAS or minigun blast moves the firer, and players ride it deliberately.
                self_boost(&weapon, aim_unit, &pose).scale(1.0 / DT)
            } else {
                Vec2 {
                    x: -aim_unit.x * 1.5,
                    y: -aim_unit.y * 1.5,
                }
            };
            apply_impulse(
                player,
                Impulse {
                    velocity: boost,
                    source,
                },
            );
            events.push(Event::Shot { player: player.id });
            stats.record(StatEvent::Shot {
                player: player.id,
                weapon: weapon.kind,
            });
        }
    }
    // Holding the throw key cooks the grenade: the longer it is held the harder it goes,
    // and it only leaves the hand once the key is released or the wind-up is complete.
    if input.throw_grenade && player.grenades > 0 && player.grenade_cooldown == 0 {
        player.grenade_charge = player.grenade_charge.saturating_add(1);
    }
    let wound_up = throw_frame(player.grenade_charge) >= THROW_LAST_FRAME;
    let releasing = player.grenade_held && !input.throw_grenade;
    if (releasing || wound_up) && player.grenades > 0 && player.grenade_cooldown == 0 {
        let dx = input.aim.x - player.pos.x;
        let dy = input.aim.y - player.pos.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len > 1.0 && len.is_finite() {
            let kind = if player.cluster_grenades > 0 {
                WeaponKind::ClusterGrenade
            } else {
                WeaponKind::FragGrenade
            };
            let grenade = *weapons.get(kind);
            let dir = Vec2 {
                x: dx / len,
                y: dy / len,
            };
            let thrown = throw_velocity(
                &grenade,
                dir,
                player.grenade_charge.saturating_sub(1),
                player.vel.scale(DT),
            );
            projectiles.push(Projectile {
                id: *next_projectile,
                owner: player.id,
                pos: player.pos,
                vel: thrown.scale(1.0 / DT),
                ttl: grenade.timeout(),
                damage: 0,
                explosive: true,
                kind: ProjectileKind::FragGrenade,
                splash_radius: grenade.explosion_radius(),
                weapon: Some(kind),
                origin: player.pos,
                last_impact: player.pos,
            });
            *next_projectile = next_projectile.wrapping_add(1);
            if player.cluster_grenades > 0 {
                player.cluster_grenades -= 1;
            }
            player.grenades -= 1;
            player.grenade_cooldown = 80;
            player.grenade_charge = 0;
        }
    }
    if !input.throw_grenade {
        player.grenade_charge = 0;
    }
    player.grenade_held = input.throw_grenade;
    if player.switch_timer == 0 && input.drop && !previous.drop {
        if let Some(slot) = player.inventory.drop_active() {
            crate::weapons::reload::interrupt(&mut player.reload_timer);
            player.throw_charge = 0;
            objects.push(dropped_weapon(slot.kind, slot.ammo, player.pos, player.vel));
            sync_hands(player);
        }
    }
    if player.switch_timer == 0 && input.throw_weapon && player.inventory.active().is_some() {
        player.throw_charge = player.throw_charge.saturating_add(1).min(MAX_THROW_CHARGE);
    } else if player.throw_charge > 0 && !input.throw_weapon {
        if let Some(slot) = player.inventory.drop_active() {
            crate::weapons::reload::interrupt(&mut player.reload_timer);
            let facing = player.facing.sign();
            let dir = aim_dir(player.pos, input.aim, facing).unwrap_or(Vec2 { x: facing, y: 0.0 });
            let vel = charged_velocity(dir, player.throw_charge, player.vel);
            objects.push(dropped_weapon(slot.kind, slot.ammo, player.pos, vel));
            sync_hands(player);
        }
        player.throw_charge = 0;
    }
    if player.switch_timer == 0 && input.throw_knife && !previous.throw_knife {
        if let Some(_slot) = player
            .inventory
            .take_kind(WeaponKind::CombatKnife)
            .or_else(|| player.inventory.take_kind(WeaponKind::ThrownKnife))
        {
            crate::weapons::reload::interrupt(&mut player.reload_timer);
            if let Some(dir) = aim_dir(player.pos, input.aim, player.facing.sign()) {
                let knife = *weapons.get(WeaponKind::ThrownKnife);
                let barrel = muzzle_origin(player.pos, input.aim, player.state, player.facing);
                let muzzle = muzzle_velocity(&knife, dir, player.vel.scale(DT));
                projectiles.push(Projectile {
                    id: *next_projectile,
                    owner: player.id,
                    pos: barrel,
                    vel: muzzle.scale(1.0 / DT),
                    ttl: knife.timeout(),
                    damage: 0,
                    explosive: false,
                    kind: ProjectileKind::ThrownKnife,
                    splash_radius: 0.0,
                    weapon: Some(WeaponKind::ThrownKnife),
                    origin: barrel,
                    last_impact: barrel,
                });
                *next_projectile = next_projectile.wrapping_add(1);
            }
            sync_hands(player);
        }
    }
    if player.switch_timer == 0 && input.pickup {
        if let Some(index) = nearest_index(player.pos, &objects) {
            let object = objects.remove(index);
            if let Some(kind) = object.weapon_slot.map(WeaponKind::from_slot) {
                crate::weapons::reload::interrupt(&mut player.reload_timer);
                if let Some(displaced) = player.inventory.pickup(WeaponSlot {
                    kind,
                    ammo: object.ammo,
                }) {
                    objects.push(dropped_weapon(
                        displaced.kind,
                        displaced.ammo,
                        player.pos,
                        Vec2::default(),
                    ));
                }
                sync_hands(player);
            }
        }
        let m2_ammo = u16::from(weapons.get(WeaponKind::StationaryGun).ammo);
        for gun in &mut *guns {
            if gun.mount(player.id, player.pos) {
                player
                    .inventory
                    .force_equip(WeaponKind::StationaryGun, m2_ammo);
                sync_hands(player);
            }
        }
    }
    for gun in &mut *guns {
        if gun.mounted_by == Some(player.id) && !gun.in_reach(player.pos) {
            gun.dismount(player.id);
            player.inventory.take_kind(WeaponKind::StationaryGun);
            sync_hands(player);
        }
    }
}
