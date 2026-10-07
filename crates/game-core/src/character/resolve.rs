use std::collections::BTreeMap;

use crate::weapons::{dropped_weapon, WeaponKind};
use crate::{
    absorb, assists, record_attribution, region_multiplier, Bleed, BodyRegion, CharacterState,
    DamageCause, DamageEvent, DeathCause, Event, KillFeedEntry, ObjectivePolicy, Player,
    Projectile, ProjectileKind, Ragdoll, ScoreEvent, StatEvent, Vec2, World,
};

pub(crate) fn apply_damage(world: &mut World, mut event: DamageEvent) {
    // Flame God is exactly what it says: nothing touches them while it lasts.
    if world
        .players
        .get(&event.target)
        .is_some_and(|player| player.bonus.is_invulnerable())
    {
        return;
    }
    // A Berserker hits four times as hard, which is applied once, here, rather than at every
    // place a weapon works out its damage.
    if let Some(attacker) = event.attacker.filter(|id| *id != event.target) {
        let multiplier = world
            .players
            .get(&attacker)
            .map_or(1.0, |player| player.bonus.damage_multiplier());
        if multiplier != 1.0 {
            event.amount = ((event.amount as f32) * multiplier).round() as i32;
        }
    }
    let tick = world.tick;
    let config = world.damage;
    let respawn_config = world.respawn;
    let team_mode = world.mode == "team";
    let teams: BTreeMap<u32, u8> = world
        .players
        .iter()
        .map(|(&id, player)| (id, player.team))
        .collect();

    let Some(target) = world.players.get_mut(&event.target) else {
        return;
    };
    if target.hp <= 0 {
        return;
    }
    let self_inflicted = event.attacker == Some(event.target);
    if target.spawn_protection > 0 && !self_inflicted {
        return;
    }
    let scaled = if event.pre_scaled {
        event.amount.max(0)
    } else {
        ((event.amount.max(0) as f32) * region_multiplier(&config, event.region)).round() as i32
    };
    if scaled <= 0 {
        return;
    }
    let split = absorb(&config, target.armor, scaled);
    target.armor = (target.armor - split.armor).max(0);
    target.hp = (target.hp - split.health).max(0);
    target.last_damage_direction = event.direction;
    if let Some(attacker) = event.attacker.filter(|id| *id != event.target) {
        record_attribution(&mut target.attackers, attacker, tick, split.health);
    }
    if event.cause.draws_blood() && split.health >= config.bleed_threshold && target.hp > 0 {
        target.bleed = Some(Bleed::open(&config, event.attacker));
    }
    let hp = target.hp;
    let position = target.pos;
    let velocity = target.vel;
    let target_team = target.team;
    let attackers = target.attackers.clone();
    let died = hp == 0;
    let gibbed = died && split.health >= config.gib_damage;
    let death_drops = if died {
        Some((target.inventory.take_all(), position, velocity))
    } else {
        None
    };
    // A grenade that was being cooked when its thrower was killed falls out of the dead hand
    // and goes off where it lands, which is what makes trading with a cooked grenade work.
    let dropped_grenade = if died && target.grenade_charge > 0 && target.grenades > 0 {
        target.grenades -= 1;
        target.grenade_charge = 0;
        let kind = if target.cluster_grenades > 0 {
            target.cluster_grenades -= 1;
            WeaponKind::ClusterGrenade
        } else {
            WeaponKind::FragGrenade
        };
        Some((kind, position, velocity))
    } else {
        None
    };
    if died {
        target.respawn = respawn_config.delay_ticks;
        target.bleed = None;
        target.bonus.clear();
        target.state = CharacterState::Dead;
    }

    world.events.push(Event::Damage {
        target: event.target,
        attacker: event.attacker,
        amount: split.health,
        region: event.region,
        cause: event.cause,
        direction: event.direction,
    });
    world.events.push(Event::Hit {
        target: event.target,
        hp,
    });
    if event.cause.draws_blood() {
        world.events.push(Event::Blood {
            target: event.target,
            position,
            direction: event.direction,
            amount: split.health,
        });
    }
    if let Some((slots, pos, vel)) = death_drops {
        for (index, slot) in slots.into_iter().enumerate() {
            let kick = Vec2 {
                x: if index == 0 { -60.0 } else { 60.0 },
                y: -120.0,
            };
            world.objects.push(dropped_weapon(
                slot.kind,
                slot.ammo,
                pos,
                Vec2 {
                    x: vel.x + kick.x,
                    y: vel.y + kick.y,
                },
            ));
        }
    }
    if let Some((kind, pos, vel)) = dropped_grenade {
        let grenade = *world.weapons.get(kind);
        world.projectiles.push(Projectile {
            id: world.next_projectile,
            owner: event.target,
            pos,
            // It is dropped, not thrown: it keeps the body's momentum and nothing more.
            vel,
            ttl: grenade.timeout(),
            damage: 0,
            explosive: true,
            kind: ProjectileKind::FragGrenade,
            splash_radius: grenade.explosion_radius(),
            weapon: Some(kind),
            origin: pos,
            last_impact: pos,
        });
        world.next_projectile = world.next_projectile.wrapping_add(1);
    }
    if !died {
        return;
    }

    let killer = event.attacker.filter(|id| *id != event.target);
    let teamkill = killer
        .is_some_and(|id| team_mode && target_team != 0 && teams.get(&id) == Some(&target_team));
    let suicide = killer.is_none() && event.attacker.is_some();
    let assisting = assists(
        &attackers,
        killer,
        event.target,
        tick,
        config.attribution_ticks,
    );
    let headshot = !teamkill && killer.is_some() && event.region == BodyRegion::Head;

    // Every point in the match is booked here and nowhere else.
    let rules = world.rules;
    let team_of = |id: u32| teams.get(&id).copied().unwrap_or(0);
    let score_event = if let Some(id) = killer {
        Some(if teamkill {
            ScoreEvent::TeamKill {
                killer: id,
                victim: event.target,
            }
        } else {
            ScoreEvent::Kill {
                killer: id,
                victim: event.target,
            }
        })
    } else if suicide {
        Some(ScoreEvent::Suicide {
            player: event.target,
        })
    } else {
        None
    };
    if let Some(score_event) = score_event {
        // Pointmatch pays extra while holding the flag; Rambomatch pays only the bow holder.
        if let ScoreEvent::Kill { killer, victim } = score_event {
            let policy = crate::modes::objective::ObjectiveRules::for_mode(rules.kind);
            let points = policy.kill_points(world.objectives.is_carrying(killer));
            if points == 0 {
                world.ledger.note_death(victim);
            } else {
                world.ledger.record(score_event, &rules, team_of);
                if points > 1 {
                    world.ledger.record(
                        ScoreEvent::Objective {
                            player: killer,
                            team: team_of(killer),
                            points: points - 1,
                        },
                        &rules,
                        team_of,
                    );
                }
            }
        } else {
            world.ledger.record(score_event, &rules, team_of);
        }
    } else {
        // An environmental death still counts as a death, with nobody to credit.
        world.ledger.ensure(event.target);
    }

    let mut multi = 0;
    if let Some(id) = killer.filter(|_| !teamkill) {
        if let Some(attacker) = world.players.get_mut(&id) {
            if headshot {
                attacker.headshots += 1;
            }
            multi = attacker.multi_kill.record(tick, config.multi_kill_ticks);
        }
    }
    for assist in &assisting {
        if let Some(helper) = world.players.get_mut(assist) {
            helper.assists += 1;
        }
    }

    let cause = if teamkill {
        DeathCause::TeamKill {
            by: killer.unwrap_or(event.target),
        }
    } else if let Some(id) = killer {
        DeathCause::Killed {
            by: id,
            cause: event.cause,
            region: event.region,
        }
    } else if suicide {
        DeathCause::Suicide(event.cause)
    } else {
        DeathCause::Environment(event.cause)
    };
    if let Some(target) = world.players.get_mut(&event.target) {
        target.last_death = Some(cause);
    }

    world.ragdolls.push(Ragdoll::spawn(
        event.target,
        target_team,
        position,
        velocity,
        event.direction,
        respawn_config.corpse_ticks,
        gibbed,
    ));
    if gibbed {
        world.events.push(Event::Gibs {
            target: event.target,
            position,
            velocity,
        });
    }
    world.events.push(Event::Kill {
        killer: killer.unwrap_or(event.target),
        target: event.target,
    });
    world.events.push(Event::KillFeed(KillFeedEntry {
        killer,
        target: event.target,
        cause: event.cause,
        region: event.region,
        headshot,
        teamkill,
        suicide: killer.is_none(),
        multi,
        assists: assisting,
    }));
    // Statistics are fed by the same events as the score, so the two can never disagree about
    // what happened.
    if let Some(attacker) = event.attacker.filter(|id| *id != event.target) {
        if let Some(weapon) = weapon_of(event.cause, world.players.get(&attacker)) {
            world.stats.record(StatEvent::Hit {
                player: attacker,
                weapon,
                headshot: event.region == BodyRegion::Head,
            });
            if died {
                world.stats.record(StatEvent::KilledWith {
                    killer: attacker,
                    victim: event.target,
                    weapon,
                });
            }
        }
    }
    if died {
        world.stats.record(StatEvent::DiedTo {
            victim: event.target,
            cause: event.cause,
        });
    }

    // apply_damage is an authoritative entry point in its own right, so the counters the HUD
    // reads are brought back in line here and not only at the end of a tick.
    world.project_scores();
    if rules.modifiers.advance {
        world.advance_progress(killer, event.target);
    }
}

/// The weapon a damage event should be credited to, for the statistics.
///
/// Only a wound from something a player was holding counts: a fall or a deadly polygon has no
/// weapon behind it, and crediting one would make a scoreboard lie.
fn weapon_of(cause: DamageCause, attacker: Option<&Player>) -> Option<WeaponKind> {
    match cause {
        DamageCause::Bullet | DamageCause::Pellet | DamageCause::Melee | DamageCause::Explosion => {
            attacker.map(|player| WeaponKind::from_slot(player.weapon))
        }
        _ => None,
    }
}
