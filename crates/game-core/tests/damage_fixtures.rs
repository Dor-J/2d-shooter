// Acceptance evidence for docs/parity/coverage.json:
//   rust:damage_fixtures:regions          — body-region damage scaling and armor absorption
//   rust:damage_fixtures:bleeding         — wounds that keep costing health after the hit
//   rust:damage_fixtures:attribution      — delayed kill credit and assist tracking
//   rust:damage_fixtures:death-causes     — self kills, team kills, and typed death causes
//   rust:damage_fixtures:spawn-protection — a fresh spawn cannot be shot
//   rust:damage_fixtures:respawn          — a configurable respawn delay that counts down
//   rust:damage_fixtures:weapon-choice    — picking the weapon to respawn with while dead
//   rust:damage_fixtures:ragdoll          — corpses that fall, settle, and are cleared
//   rust:damage_fixtures:spawn-selection  — mode, team, and distance-aware spawn choice
use game_core::{
    absorb, region_multiplier, select_spawn, BodyRegion, DamageCause, DamageConfig, DamageEvent,
    DeathCause, Event, Input, MapSpawn, RespawnConfig, Vec2, World,
};
use std::collections::BTreeMap;

const SOURCE_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";
const SOURCE_DAMAGE: &str = "shared/mechanics/Bullets.pas:HitTest/shared/Sprites.pas:HealthHit";

fn hit(attacker: u32, target: u32, amount: i32, region: BodyRegion) -> DamageEvent {
    DamageEvent {
        attacker: Some(attacker),
        target,
        amount,
        region,
        cause: DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
    }
}

fn duel() -> World {
    let mut world = World::new("deathmatch");
    world.add_player(1, "attacker".into());
    world.add_player(2, "target".into());
    world
}

#[test]
fn fixture_metadata_is_pinned() {
    assert_eq!(SOURCE_COMMIT.len(), 40);
    assert!(SOURCE_DAMAGE.contains("HitTest"));
}

#[test]
fn body_regions_scale_damage_from_one_table() {
    let config = DamageConfig::default();
    assert_eq!(region_multiplier(&config, BodyRegion::Chest), 1.0);
    assert!(region_multiplier(&config, BodyRegion::Head) > 1.0);
    assert!(region_multiplier(&config, BodyRegion::Legs) < 1.0);

    let mut world = duel();
    world.apply_damage(hit(1, 2, 20, BodyRegion::Legs));
    let legs = 100 - world.players[&2].hp;
    world.players.get_mut(&2).unwrap().hp = 100;
    world.apply_damage(hit(1, 2, 20, BodyRegion::Head));
    let head = 100 - world.players[&2].hp;
    assert!(head > legs, "a headshot must hurt more than a leg hit");
}

#[test]
fn armor_absorbs_before_health_and_is_consumed() {
    let config = DamageConfig::default();
    let shared = absorb(&config, 50, 40);
    assert_eq!(shared.armor + shared.health, 40);
    assert!(shared.armor > 0 && shared.health > 0);
    assert_eq!(absorb(&config, 0, 40).health, 40);

    let spent = absorb(&config, 3, 100);
    assert_eq!(spent.armor, 3, "armor cannot absorb more than it has");
    assert_eq!(spent.health, 97);

    let mut world = duel();
    world.players.get_mut(&2).unwrap().armor = 40;
    world.apply_damage(hit(1, 2, 20, BodyRegion::Chest));
    assert!(world.players[&2].armor < 40);
    assert!(world.players[&2].hp > 80);
}

#[test]
fn a_heavy_hit_starts_bleeding_that_keeps_costing_health() {
    let mut world = duel();
    world.apply_damage(hit(1, 2, 60, BodyRegion::Chest));
    assert!(world.players[&2].bleed.is_some(), "a heavy hit draws blood");
    let after_hit = world.players[&2].hp;
    for _ in 0..90 {
        world.step(&BTreeMap::new());
    }
    assert!(
        world.players[&2].hp < after_hit,
        "bleeding keeps costing health"
    );

    let mut light = duel();
    light.apply_damage(hit(1, 2, 3, BodyRegion::Legs));
    assert!(light.players[&2].bleed.is_none(), "a graze does not bleed");
}

#[test]
fn a_bleeding_death_is_still_credited_to_the_last_attacker() {
    let mut world = duel();
    world.players.get_mut(&2).unwrap().hp = 30;
    world.apply_damage(hit(1, 2, 25, BodyRegion::Chest));
    assert!(world.players[&2].hp > 0, "the hit alone must not be lethal");
    for _ in 0..1200 {
        if world.players[&2].hp <= 0 {
            break;
        }
        world.step(&BTreeMap::new());
    }
    assert_eq!(world.players[&2].hp, 0, "bleeding finished the job");
    assert_eq!(world.players[&1].kills, 1);
    assert!(matches!(
        world.players[&2].last_death,
        Some(DeathCause::Killed {
            by: 1,
            cause: DamageCause::Bleeding,
            ..
        })
    ));
}

#[test]
fn an_earlier_attacker_is_recorded_as_an_assist() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "finisher".into());
    world.add_player(2, "victim".into());
    world.add_player(3, "helper".into());
    world.apply_damage(hit(3, 2, 40, BodyRegion::Chest));
    world.apply_damage(hit(1, 2, 200, BodyRegion::Chest));

    let feed = world
        .events
        .iter()
        .filter_map(|event| match event {
            Event::KillFeed(entry) => Some(entry.clone()),
            _ => None,
        })
        .next_back()
        .expect("a kill feed entry");
    assert_eq!(feed.killer, Some(1));
    assert_eq!(feed.assists, vec![3]);
    assert_eq!(world.players[&3].assists, 1);
    assert_eq!(world.players[&1].kills, 1);
}

#[test]
fn self_kills_and_team_kills_never_credit_a_frag() {
    let mut world = World::new("team");
    world.add_player(1, "blue".into());
    world.add_player(2, "red".into());
    world.add_player(3, "blue two".into());
    let blue = world.players[&1].team;
    assert_eq!(world.players[&3].team, blue);
    assert_ne!(world.players[&2].team, blue);

    world.apply_damage(hit(1, 3, 500, BodyRegion::Chest));
    assert_eq!(world.players[&1].kills, 0, "a teamkill is not a frag");
    assert_eq!(world.players[&1].teamkills, 1);
    assert!(matches!(
        world.players[&3].last_death,
        Some(DeathCause::TeamKill { by: 1 })
    ));

    world.apply_damage(DamageEvent {
        attacker: Some(2),
        ..hit(2, 2, 500, BodyRegion::Chest)
    });
    assert_eq!(world.players[&2].kills, 0, "a suicide is not a frag");
    assert_eq!(world.players[&2].suicides, 1);
    assert!(matches!(
        world.players[&2].last_death,
        Some(DeathCause::Suicide(DamageCause::Bullet))
    ));
}

#[test]
fn spawn_protection_blocks_damage_until_it_expires() {
    let mut world = duel();
    let protection = world.respawn.protection_ticks;
    assert!(protection > 0);
    world.players.get_mut(&2).unwrap().spawn_protection = protection;

    world.apply_damage(hit(1, 2, 50, BodyRegion::Chest));
    assert_eq!(
        world.players[&2].hp, 100,
        "a protected spawn takes no damage"
    );

    for _ in 0..protection {
        world.step(&BTreeMap::new());
    }
    assert_eq!(world.players[&2].spawn_protection, 0);
    world.apply_damage(hit(1, 2, 50, BodyRegion::Chest));
    assert!(world.players[&2].hp < 100);
}

#[test]
fn the_respawn_delay_is_configurable_and_counts_down() {
    let mut world = duel();
    world.respawn = RespawnConfig {
        delay_ticks: 30,
        ..RespawnConfig::default()
    };
    world.apply_damage(hit(1, 2, 500, BodyRegion::Chest));
    assert_eq!(world.players[&2].respawn, 30);

    for _ in 0..31 {
        world.step(&BTreeMap::new());
    }
    assert_eq!(world.players[&2].hp, 100);
    assert!(
        world.players[&2].spawn_protection > 0,
        "a fresh spawn is protected"
    );
}

#[test]
fn a_dead_player_picks_the_weapon_they_will_respawn_with() {
    let mut world = duel();
    world.apply_damage(hit(1, 2, 500, BodyRegion::Chest));
    let chosen = Input {
        weapon: 5,
        ..Input::default()
    };
    for _ in 0..world.respawn.delay_ticks + 2 {
        world.step(&BTreeMap::from([(2, chosen)]));
    }
    assert_eq!(world.players[&2].hp, 100);
    assert_eq!(world.players[&2].weapon, 5);
}

#[test]
fn a_ragdoll_falls_to_the_ground_and_is_cleared_after_its_lifetime() {
    let mut world = duel();
    world.respawn = RespawnConfig {
        corpse_ticks: 40,
        ..RespawnConfig::default()
    };
    world.players.get_mut(&2).unwrap().pos = Vec2 { x: 260.0, y: 200.0 };
    world.apply_damage(hit(1, 2, 500, BodyRegion::Chest));
    assert_eq!(world.ragdolls.len(), 1);
    assert_eq!(world.ragdolls[0].segments.len(), 3);

    let start = world.ragdolls[0].segments[0].pos.y;
    for _ in 0..20 {
        world.step(&BTreeMap::new());
    }
    assert!(
        world.ragdolls[0].segments[0].pos.y > start,
        "a corpse falls under gravity"
    );
    for _ in 0..40 {
        world.step(&BTreeMap::new());
    }
    assert!(world.ragdolls.is_empty(), "corpses do not persist forever");
}

#[test]
fn a_hard_landing_costs_health_and_a_soft_one_does_not() {
    let mut world = duel();
    let config = world.damage;
    // Well clear of the other player, so this measures the ground and not a body landed on.
    let player = world.players.get_mut(&2).unwrap();
    player.pos = Vec2 { x: 360.0, y: 100.0 };
    player.vel = Vec2 { x: 0.0, y: 0.0 };
    for _ in 0..200 {
        world.step(&BTreeMap::new());
    }
    assert!(
        world.players[&2].hp < 100,
        "falling from height hurts above {} px/s",
        config.fall_damage_speed
    );

    let mut gentle = duel();
    gentle.players.get_mut(&2).unwrap().pos = Vec2 { x: 360.0, y: 470.0 };
    for _ in 0..30 {
        gentle.step(&BTreeMap::new());
    }
    assert_eq!(gentle.players[&2].hp, 100, "a short drop is free");
}

#[test]
fn kill_feed_entries_describe_headshots_suicides_teamkills_and_multi_kills() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "ace".into());
    world.add_player(2, "first".into());
    world.add_player(3, "second".into());

    world.apply_damage(hit(1, 2, 500, BodyRegion::Head));
    let headshot = world
        .events
        .iter()
        .filter_map(|event| match event {
            Event::KillFeed(entry) => Some(entry.clone()),
            _ => None,
        })
        .next_back()
        .expect("a kill feed entry");
    assert!(headshot.headshot);
    assert!(!headshot.suicide && !headshot.teamkill);
    assert_eq!(headshot.multi, 1);

    world.apply_damage(hit(1, 3, 500, BodyRegion::Chest));
    let double = world
        .events
        .iter()
        .filter_map(|event| match event {
            Event::KillFeed(entry) => Some(entry.clone()),
            _ => None,
        })
        .next_back()
        .expect("a kill feed entry");
    assert!(!double.headshot);
    assert_eq!(double.multi, 2, "two kills inside the window is a double");
}

#[test]
fn damage_emits_presentation_events_without_feeding_them_back_into_the_simulation() {
    let mut world = duel();
    world.apply_damage(hit(1, 2, 30, BodyRegion::Chest));
    let blood = world
        .events
        .iter()
        .filter(|event| matches!(event, Event::Blood { .. }))
        .count();
    assert_eq!(blood, 1);
    let before = world.digest();
    let mut twin = duel();
    twin.apply_damage(hit(1, 2, 30, BodyRegion::Chest));
    assert_eq!(
        before,
        twin.digest(),
        "presentation events stay deterministic"
    );

    world.apply_damage(hit(1, 2, 500, BodyRegion::Chest));
    assert!(world
        .events
        .iter()
        .any(|event| matches!(event, Event::Gibs { .. })));
}

#[test]
fn spawns_are_chosen_by_mode_team_and_distance_from_enemies() {
    let spawns = [
        MapSpawn {
            team: 1,
            position: Vec2 { x: 100.0, y: 400.0 },
        },
        MapSpawn {
            team: 1,
            position: Vec2 { x: 900.0, y: 400.0 },
        },
        MapSpawn {
            team: 2,
            position: Vec2 { x: 500.0, y: 400.0 },
        },
    ];
    let enemies = [Vec2 { x: 120.0, y: 400.0 }];
    let chosen = select_spawn(&spawns, "team", 1, 7, &enemies);
    assert_eq!(
        chosen,
        Vec2 { x: 900.0, y: 400.0 },
        "team spawn away from enemies"
    );

    let free_for_all = select_spawn(&spawns, "deathmatch", 0, 7, &enemies);
    assert!(spawns.iter().any(|spawn| spawn.position == free_for_all));

    let fallback = select_spawn(&[], "deathmatch", 0, 3, &[]);
    assert!(fallback.x.is_finite() && fallback.y.is_finite());
}
