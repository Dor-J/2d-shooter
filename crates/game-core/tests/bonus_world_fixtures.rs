//! Proof that kits and their effects run inside `World`, not only as a library of rules.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:bonus_world:pickup — kits spawn, are walked over, and grant what they promise
//!   rust:bonus_world:effects — invulnerability, doubled damage, and expiry in a real match
//!   rust:bonus_world:physics — kits are shoved by gunfire like any other loose object

use game_core::{
    BodyRegion, BonusConfig, BonusEffect, DamageCause, DamageEvent, Event, KitKind, Pickup,
    Projectile, ProjectileKind, TimedEffect, Vec2, WeaponKind, World, CLUSTER_GRENADES, VEST_ARMOR,
};
use std::collections::BTreeMap;

fn idle(world: &mut World, ticks: u32) {
    for _ in 0..ticks {
        world.step(&BTreeMap::new());
    }
}

fn hit(attacker: u32, target: u32, amount: i32) -> DamageEvent {
    DamageEvent {
        attacker: Some(attacker),
        target,
        amount,
        region: BodyRegion::Chest,
        cause: DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
        pre_scaled: true,
    }
}

/// A world with kits switched off, so a test's own kit is the only one on the map.
fn quiet_world() -> World {
    let mut world = World::new("deathmatch");
    world.bonuses = BonusConfig::none();
    world
}

/// Drops one kit right where a player is standing.
fn drop_kit_on(world: &mut World, player: u32, kind: KitKind) {
    let at = world.players[&player].pos;
    world.pickups.items.push(Pickup::new(kind, at, 6_000));
}

#[test]
fn a_medkit_puts_a_hurt_player_back_on_their_feet_and_is_wasted_on_a_healthy_one() {
    let mut world = quiet_world();
    world.add_player(1, "Hurt".into());
    world.players.get_mut(&1).unwrap().hp = 30;

    drop_kit_on(&mut world, 1, KitKind::Medic);
    idle(&mut world, 1);
    assert_eq!(world.players[&1].hp, 100);
    assert!(world.pickups.items.is_empty(), "the kit was used up");

    // On full health there is nothing to heal, so the kit is left for somebody who needs it.
    drop_kit_on(&mut world, 1, KitKind::Medic);
    idle(&mut world, 1);
    assert_eq!(world.pickups.items.len(), 1, "still there");
}

#[test]
fn a_vest_grants_armor_and_a_second_one_is_left_on_the_ground() {
    let mut world = quiet_world();
    world.add_player(1, "Soldier".into());
    assert_eq!(world.players[&1].armor, 0);

    drop_kit_on(&mut world, 1, KitKind::Vest);
    idle(&mut world, 1);
    assert_eq!(world.players[&1].armor, VEST_ARMOR);

    drop_kit_on(&mut world, 1, KitKind::Vest);
    idle(&mut world, 1);
    assert_eq!(world.pickups.items.len(), 1, "already wearing one");
}

#[test]
fn a_grenade_kit_refills_the_pouch_and_a_cluster_kit_swaps_it() {
    let mut world = quiet_world();
    world.add_player(1, "Thrower".into());
    world.players.get_mut(&1).unwrap().grenades = 0;

    drop_kit_on(&mut world, 1, KitKind::Grenades);
    idle(&mut world, 1);
    assert_eq!(world.players[&1].grenades, world.damage.max_grenades);
    assert_eq!(world.players[&1].cluster_grenades, 0);

    drop_kit_on(&mut world, 1, KitKind::ClusterGrenades);
    idle(&mut world, 1);
    assert_eq!(world.players[&1].cluster_grenades, CLUSTER_GRENADES);
    assert!(world.players[&1].grenades >= CLUSTER_GRENADES);
}

#[test]
fn flame_god_turns_damage_away_until_it_runs_out() {
    let mut world = quiet_world();
    world.add_player(1, "God".into());
    world.add_player(2, "Shooter".into());

    drop_kit_on(&mut world, 1, KitKind::FlameGod);
    idle(&mut world, 1);
    assert_eq!(world.players[&1].bonus.active, Some(BonusEffect::FlameGod));

    world.apply_damage(hit(2, 1, 500));
    assert_eq!(world.players[&1].hp, 100, "nothing touched them");

    // Once it runs out they are as mortal as anybody.
    idle(&mut world, BonusEffect::FlameGod.duration() as u32 + 2);
    assert_eq!(world.players[&1].bonus.active, None);
    world.apply_damage(hit(2, 1, 40));
    assert!(world.players[&1].hp < 100, "and now it hurts");
}

#[test]
fn a_berserker_hits_four_times_as_hard() {
    let mut world = quiet_world();
    world.add_player(1, "Berserker".into());
    world.add_player(2, "Victim".into());
    world.add_player(3, "Ordinary".into());

    // An ordinary attacker deals what they deal.
    world.apply_damage(hit(3, 2, 10));
    let ordinary = 100 - world.players[&2].hp;
    assert!(ordinary > 0);

    world.players.get_mut(&2).unwrap().hp = 100;
    drop_kit_on(&mut world, 1, KitKind::Berserker);
    idle(&mut world, 1);
    assert_eq!(world.players[&1].bonus.active, Some(BonusEffect::Berserker));

    world.apply_damage(hit(1, 2, 10));
    let berserk = 100 - world.players[&2].hp;
    assert_eq!(
        berserk,
        ordinary * 4,
        "four times as hard: {ordinary} then {berserk}"
    );
}

#[test]
fn a_predator_is_nearly_invisible_and_bleeding_gives_them_away() {
    let mut world = quiet_world();
    world.add_player(1, "Predator".into());
    drop_kit_on(&mut world, 1, KitKind::Predator);
    idle(&mut world, 1);

    let effect = world.players[&1].bonus;
    assert_eq!(effect.active, Some(BonusEffect::Predator));
    assert_eq!(effect.alpha(), game_core::PREDATOR_ALPHA);

    let healthy = game_core::predator_alpha(&effect, 100, 100);
    let wounded = game_core::predator_alpha(&effect, 20, 100);
    assert!(
        wounded > healthy,
        "blood shows through: {healthy} then {wounded}"
    );
}

#[test]
fn a_second_timed_kit_is_refused_while_one_is_running() {
    let mut world = quiet_world();
    world.add_player(1, "Lucky".into());

    drop_kit_on(&mut world, 1, KitKind::Predator);
    idle(&mut world, 1);
    assert_eq!(world.players[&1].bonus.active, Some(BonusEffect::Predator));

    drop_kit_on(&mut world, 1, KitKind::Berserker);
    idle(&mut world, 1);
    assert_eq!(
        world.players[&1].bonus.active,
        Some(BonusEffect::Predator),
        "you cannot be invisible and four times as strong at once"
    );
    assert_eq!(
        world.pickups.items.len(),
        1,
        "the berserker kit is still there"
    );
}

#[test]
fn dying_ends_whatever_was_running() {
    let mut world = quiet_world();
    world.add_player(1, "Berserker".into());
    world.add_player(2, "Killer".into());

    drop_kit_on(&mut world, 1, KitKind::Berserker);
    idle(&mut world, 1);
    assert!(world.players[&1].bonus.active.is_some());

    world.apply_damage(hit(2, 1, 500));
    assert!(world.players[&1].hp <= 0);
    assert_eq!(
        world.players[&1].bonus.active, None,
        "a corpse is not a berserker"
    );
}

#[test]
fn an_expiring_effect_is_announced_so_the_hud_can_say_so() {
    let mut world = quiet_world();
    world.add_player(1, "Berserker".into());
    world.players.get_mut(&1).unwrap().bonus = {
        let mut effect = TimedEffect::default();
        effect.activate(BonusEffect::Berserker);
        effect.ticks_left = 2;
        effect
    };

    let mut announced = false;
    for _ in 0..5 {
        world.step(&BTreeMap::new());
        announced |= world.events.iter().any(|event| {
            matches!(
                event,
                Event::BonusExpired {
                    player: 1,
                    effect: BonusEffect::Berserker
                }
            )
        });
    }
    assert!(announced, "the client is told it ended");
}

#[test]
fn a_kit_taken_is_announced_so_the_hud_and_the_audio_can_react() {
    let mut world = quiet_world();
    world.add_player(1, "Soldier".into());
    world.players.get_mut(&1).unwrap().hp = 50;
    drop_kit_on(&mut world, 1, KitKind::Medic);

    world.step(&BTreeMap::new());
    assert!(
        world.events.iter().any(|event| matches!(
            event,
            Event::KitTaken {
                player: 1,
                kind: KitKind::Medic
            }
        )),
        "{:?}",
        world.events
    );
}

#[test]
fn two_players_on_one_kit_means_one_of_them_gets_it() {
    let mut world = quiet_world();
    world.add_player(1, "First".into());
    world.add_player(2, "Second".into());
    world.players.get_mut(&1).unwrap().hp = 50;
    let here = Vec2 { x: 600.0, y: 300.0 };
    world.players.get_mut(&1).unwrap().pos = here;
    world.players.get_mut(&2).unwrap().pos = here;
    world.players.get_mut(&2).unwrap().hp = 50;
    world
        .pickups
        .items
        .push(Pickup::new(KitKind::Medic, here, 600));

    idle(&mut world, 1);
    let healed = [1u32, 2]
        .iter()
        .filter(|id| world.players[id].hp == 100)
        .count();
    assert_eq!(healed, 1, "one of them got it, not both");
    assert!(world.pickups.items.is_empty());
}

#[test]
fn a_bullet_shoves_a_kit_it_passes_through() {
    let mut world = quiet_world();
    world.add_player(1, "Shooter".into());
    let at = Vec2 { x: 600.0, y: 300.0 };
    world
        .pickups
        .items
        .push(Pickup::new(KitKind::Vest, at, 6_000));
    world.pickups.items[0].body.vel = Vec2::default();

    let from = Vec2 {
        x: at.x - 60.0,
        y: at.y,
    };
    world.projectiles.push(Projectile {
        id: 1,
        owner: 1,
        pos: from,
        vel: Vec2 { x: 3600.0, y: 0.0 },
        ttl: 60,
        damage: 0,
        explosive: false,
        kind: ProjectileKind::Bullet,
        splash_radius: 0.0,
        weapon: Some(WeaponKind::Ak74),
        origin: from,
        last_impact: from,
    });
    world.step(&BTreeMap::new());

    assert!(
        world.pickups.items[0].body.vel.x > 0.0,
        "the round knocked it along: {:?}",
        world.pickups.items[0].body.vel
    );
}

#[test]
fn a_server_with_kits_switched_off_never_puts_one_out() {
    let mut world = quiet_world();
    world.add_player(1, "Alone".into());
    idle(&mut world, 3_000);
    assert!(world.pickups.items.is_empty());
}

#[test]
fn a_server_with_kits_switched_on_eventually_puts_one_out() {
    let mut world = World::new("deathmatch");
    world.bonuses = BonusConfig {
        frequency_ticks: 10,
        // Only the common kits, so the rarity roll does not make this flaky.
        cluster: false,
        vest: false,
        flame_god: false,
        berserker: false,
        predator: false,
        ..BonusConfig::default()
    };
    world.add_player(1, "Waiting".into());
    // Keep them away from wherever a kit lands so they do not eat every one.
    world.players.get_mut(&1).unwrap().pos = Vec2 { x: 20.0, y: 20.0 };

    let mut seen = false;
    for _ in 0..2_000 {
        world.players.get_mut(&1).unwrap().pos = Vec2 { x: 20.0, y: 20.0 };
        world.step(&BTreeMap::new());
        seen |= !world.pickups.items.is_empty()
            || world
                .events
                .iter()
                .any(|event| matches!(event, Event::KitSpawned { .. }));
        if seen {
            break;
        }
    }
    assert!(seen, "a kit was handed out");
}

#[test]
fn restarting_sweeps_the_kits_away_and_ends_every_effect() {
    let mut world = quiet_world();
    world.add_player(1, "Berserker".into());
    drop_kit_on(&mut world, 1, KitKind::Berserker);
    idle(&mut world, 1);
    world
        .pickups
        .items
        .push(Pickup::new(KitKind::Medic, Vec2 { x: 10.0, y: 10.0 }, 600));
    assert!(world.players[&1].bonus.active.is_some());

    world.restart_match();
    assert!(world.pickups.items.is_empty());
    assert_eq!(world.players[&1].bonus.active, None);
}
