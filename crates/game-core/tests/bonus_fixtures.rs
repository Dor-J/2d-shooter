//! Parity fixtures for bonus kits, the timed effects they grant, and the pickups on the map.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:bonus:effects  — every kit's effect, duration, invulnerability, damage, and visibility
//!   rust:bonus:stacking — activation, replacement, expiry, and clearing on death
//!   rust:bonus:pickups  — spawn frequency, lifetime, collision, nearest-wins, and projectile push
//!   rust:bonus:settings — per-kit server switches and the frequency setting
//!
//! Source: OpenSoldat `shared/mechanics/Things.pas` (the kit pickup branches),
//! `shared/mechanics/Sprites.pas` (`HealthHit`, the bonus timer), and `shared/Constants.pas`, at
//! the commit pinned in `docs/parity/reference-lock.md`.

use game_core::{
    predator_alpha, BonusConfig, BonusEffect, CollisionWorld, KitKind, Pickup, Pickups, SimRng,
    TimedEffect, Vec2, World, BERSERKER_DAMAGE_MULTIPLIER, CLUSTER_GRENADES, KIT_RADIUS,
    PREDATOR_ALPHA, VEST_ARMOR,
};

const DT: f32 = 1.0 / 60.0;

fn collision() -> CollisionWorld {
    World::new("deathmatch").collision().clone()
}

const SOURCE_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";

#[test]
fn fixture_metadata_is_pinned_and_the_durations_match_the_source() {
    assert_eq!(SOURCE_COMMIT.len(), 40);
    // FLAMERBONUSTIME, PREDATORBONUSTIME, BERSERKERBONUSTIME from shared/Constants.pas.
    assert_eq!(BonusEffect::FlameGod.duration(), 600);
    assert_eq!(BonusEffect::Predator.duration(), 1500);
    assert_eq!(BonusEffect::Berserker.duration(), 900);
    // DEFAULTVEST, PREDATORALPHA, KIT_RADIUS.
    assert_eq!(VEST_ARMOR, 100);
    assert_eq!(PREDATOR_ALPHA, 5);
    assert_eq!(KIT_RADIUS, 12.0);
}

#[test]
fn there_are_seven_kits_and_only_three_of_them_put_a_clock_on_you() {
    assert_eq!(KitKind::ALL.len(), 7);

    // The instant ones change something and are gone.
    for instant in [
        KitKind::Medic,
        KitKind::Grenades,
        KitKind::ClusterGrenades,
        KitKind::Vest,
    ] {
        assert_eq!(instant.effect(), None, "{instant:?}");
    }

    assert_eq!(KitKind::FlameGod.effect(), Some(BonusEffect::FlameGod));
    assert_eq!(KitKind::Berserker.effect(), Some(BonusEffect::Berserker));
    assert_eq!(KitKind::Predator.effect(), Some(BonusEffect::Predator));

    // Every kit is named, and no two share a name.
    let mut names: Vec<&str> = KitKind::ALL.iter().map(|kit| kit.display_name()).collect();
    names.sort_unstable();
    let count = names.len();
    names.dedup();
    assert_eq!(names.len(), count);
}

#[test]
fn flame_god_makes_you_untouchable_and_nothing_else_does() {
    assert!(BonusEffect::FlameGod.grants_invulnerability());
    assert!(!BonusEffect::Berserker.grants_invulnerability());
    assert!(!BonusEffect::Predator.grants_invulnerability());

    let mut effect = TimedEffect::default();
    assert!(!effect.is_invulnerable(), "nothing running");
    effect.activate(BonusEffect::FlameGod);
    assert!(effect.is_invulnerable());

    let mut berserk = TimedEffect::default();
    berserk.activate(BonusEffect::Berserker);
    assert!(!berserk.is_invulnerable());
}

#[test]
fn a_berserker_hits_four_times_as_hard_and_everybody_else_hits_normally() {
    assert_eq!(
        BonusEffect::Berserker.damage_multiplier(),
        BERSERKER_DAMAGE_MULTIPLIER
    );
    assert_eq!(BERSERKER_DAMAGE_MULTIPLIER, 4.0);
    assert_eq!(BonusEffect::FlameGod.damage_multiplier(), 1.0);
    assert_eq!(BonusEffect::Predator.damage_multiplier(), 1.0);

    let mut effect = TimedEffect::default();
    assert_eq!(effect.damage_multiplier(), 1.0);
    effect.activate(BonusEffect::Berserker);
    assert_eq!(effect.damage_multiplier(), 4.0);
}

#[test]
fn a_predator_is_nearly_invisible_and_everybody_else_is_solid() {
    assert_eq!(BonusEffect::Predator.alpha(), PREDATOR_ALPHA);
    assert_eq!(BonusEffect::Berserker.alpha(), 255);

    let mut effect = TimedEffect::default();
    assert_eq!(effect.alpha(), 255, "nothing running");
    effect.activate(BonusEffect::Predator);
    assert_eq!(effect.alpha(), PREDATOR_ALPHA);
    // Not perfectly invisible: a careful eye still catches the shimmer.
    assert_eq!(PREDATOR_ALPHA, 5);
}

#[test]
fn blood_gives_a_wounded_predator_away() {
    let mut effect = TimedEffect::default();
    effect.activate(BonusEffect::Predator);

    let healthy = predator_alpha(&effect, 100, 100);
    let scratched = predator_alpha(&effect, 60, 100);
    let hurt = predator_alpha(&effect, 40, 100);
    let dying = predator_alpha(&effect, 5, 100);

    assert_eq!(healthy, PREDATOR_ALPHA);
    assert_eq!(scratched, PREDATOR_ALPHA, "a scratch does not show");
    assert!(hurt > PREDATOR_ALPHA, "a wound does: {hurt}");
    assert!(
        dying > hurt,
        "and the worse it is the more it shows: {dying}"
    );

    // Somebody who is not a Predator is simply visible.
    let nothing = TimedEffect::default();
    assert_eq!(predator_alpha(&nothing, 10, 100), 255);
}

#[test]
fn a_predator_still_makes_a_noise() {
    assert!(
        game_core::objects::bonus::predator_is_audible(),
        "invisibility is not silence, and the sound is the counterplay"
    );
}

#[test]
fn an_effect_runs_for_its_duration_and_then_ends() {
    let mut effect = TimedEffect::default();
    assert!(effect.activate(BonusEffect::Berserker));
    assert_eq!(effect.ticks_left, BonusEffect::Berserker.duration());

    for _ in 0..BonusEffect::Berserker.duration() - 1 {
        assert_eq!(effect.step(), None);
    }
    assert_eq!(
        effect.step(),
        Some(BonusEffect::Berserker),
        "the last tick reports what ended"
    );
    assert_eq!(effect.active, None);
    assert_eq!(effect.damage_multiplier(), 1.0, "and it stopped working");

    // Stepping an idle effect reports nothing rather than repeating itself.
    assert_eq!(effect.step(), None);
}

#[test]
fn a_second_kit_is_refused_while_one_is_already_running() {
    let mut effect = TimedEffect::default();
    assert!(effect.activate(BonusEffect::Predator));
    assert!(
        !effect.activate(BonusEffect::Berserker),
        "you cannot be invisible and four times as strong at once"
    );
    assert_eq!(effect.active, Some(BonusEffect::Predator));
    assert!(
        !effect.activate(BonusEffect::Predator),
        "not even the same one"
    );

    // Once it runs out the next one takes.
    effect.clear();
    assert!(effect.activate(BonusEffect::Berserker));
    assert_eq!(effect.active, Some(BonusEffect::Berserker));
}

#[test]
fn dying_clears_whatever_was_running() {
    let mut effect = TimedEffect::default();
    effect.activate(BonusEffect::FlameGod);
    assert!(effect.is_invulnerable());

    effect.clear();
    assert_eq!(effect.active, None);
    assert_eq!(effect.ticks_left, 0);
    assert!(!effect.is_invulnerable());
}

#[test]
fn the_hud_counts_the_effect_down_in_whole_seconds() {
    let mut effect = TimedEffect::default();
    effect.activate(BonusEffect::Predator);
    assert_eq!(effect.seconds_left(), 25, "1500 ticks is 25 seconds");

    for _ in 0..60 {
        effect.step();
    }
    assert_eq!(effect.seconds_left(), 24);

    effect.clear();
    assert_eq!(effect.seconds_left(), 0);
}

#[test]
fn a_kit_falls_and_rests_on_the_terrain_rather_than_through_it() {
    let collision = collision();
    let mut kit = Pickup::new(KitKind::Medic, Vec2 { x: 600.0, y: 100.0 }, 600);
    let start = kit.pos().y;

    for _ in 0..180 {
        kit.step(&collision, DT);
    }
    assert!(kit.pos().y > start, "it fell");
    assert!(kit.pos().y < 700.0, "and landed: {:?}", kit.pos());
    assert!(kit.body.grounded);
}

#[test]
fn a_kit_disappears_once_its_lifetime_runs_out() {
    let collision = collision();
    let mut kit = Pickup::new(KitKind::Vest, Vec2 { x: 600.0, y: 400.0 }, 5);

    for _ in 0..4 {
        assert!(kit.step(&collision, DT), "still there");
    }
    assert!(!kit.step(&collision, DT), "and now it is gone");
}

#[test]
fn a_kit_is_only_taken_by_somebody_standing_on_it() {
    let kit = Pickup::new(KitKind::Medic, Vec2 { x: 500.0, y: 400.0 }, 600);
    assert!(kit.within_reach(Vec2 { x: 500.0, y: 400.0 }));
    assert!(kit.within_reach(Vec2 { x: 508.0, y: 400.0 }));
    assert!(
        !kit.within_reach(Vec2 { x: 560.0, y: 400.0 }),
        "too far away"
    );
}

#[test]
fn a_bullet_or_a_blast_shoves_a_kit_about() {
    let mut kits = Pickups::default();
    kits.items
        .push(Pickup::new(KitKind::Vest, Vec2 { x: 500.0, y: 400.0 }, 600));
    assert_eq!(kits.items[0].body.vel, Vec2::default());

    kits.push_near(
        Vec2 { x: 480.0, y: 400.0 },
        60.0,
        Vec2 { x: 120.0, y: -40.0 },
    );
    assert!(kits.items[0].body.vel.x > 0.0, "it was knocked along");

    // A blast across the map does not reach it.
    kits.items[0].body.vel = Vec2::default();
    kits.push_near(Vec2 { x: 20.0, y: 20.0 }, 30.0, Vec2 { x: 500.0, y: 0.0 });
    assert_eq!(kits.items[0].body.vel, Vec2::default());
}

#[test]
fn walking_between_two_kits_takes_the_one_you_are_actually_on() {
    let mut kits = Pickups::default();
    kits.items.push(Pickup::new(
        KitKind::Medic,
        Vec2 { x: 400.0, y: 400.0 },
        600,
    ));
    kits.items
        .push(Pickup::new(KitKind::Vest, Vec2 { x: 500.0, y: 400.0 }, 600));

    assert_eq!(kits.nearest(Vec2 { x: 498.0, y: 400.0 }), Some(1));
    assert_eq!(kits.nearest(Vec2 { x: 402.0, y: 400.0 }), Some(0));
    assert_eq!(
        kits.nearest(Vec2 { x: 450.0, y: 400.0 }),
        None,
        "standing between them takes neither"
    );

    let taken = kits.take(1).expect("the vest was taken");
    assert_eq!(taken.kind, KitKind::Vest);
    assert_eq!(kits.items.len(), 1);
    assert_eq!(
        kits.take(9),
        None,
        "taking a kit that is not there is not a panic"
    );
}

#[test]
fn kits_stop_spawning_when_the_server_switches_them_off() {
    let collision = collision();
    let mut kits = Pickups::default();
    let mut rng = SimRng::seeded(1);
    let config = BonusConfig::none();
    assert!(!config.any());

    for _ in 0..5_000 {
        kits.step(&config, &collision, DT, &mut rng, |_| {
            Some(Vec2 { x: 600.0, y: 300.0 })
        });
    }
    assert!(kits.items.is_empty(), "nothing was handed out");
}

#[test]
fn a_server_can_switch_individual_kits_off() {
    let mut config = BonusConfig::default();
    assert_eq!(config.enabled().len(), 7, "everything on by default");
    for kit in KitKind::ALL {
        assert!(config.allows(kit), "{kit:?}");
    }

    config.predator = false;
    config.berserker = false;
    let enabled = config.enabled();
    assert_eq!(enabled.len(), 5);
    assert!(!enabled.contains(&KitKind::Predator));
    assert!(!enabled.contains(&KitKind::Berserker));
    assert!(enabled.contains(&KitKind::Medic));
    assert!(config.any(), "the rest still spawn");
}

#[test]
fn kits_spawn_on_the_configured_frequency_and_never_more_than_the_cap() {
    let collision = collision();
    let mut kits = Pickups::default();
    let mut rng = SimRng::seeded(7);
    let config = BonusConfig {
        frequency_ticks: 30,
        max_on_map: 2,
        lifetime_ticks: 60_000,
        // Only the common kits, so the rarity roll does not make this flaky.
        cluster: false,
        vest: false,
        flame_god: false,
        berserker: false,
        predator: false,
        ..BonusConfig::default()
    };

    // Nothing before the first interval is up.
    for _ in 0..29 {
        kits.step(&config, &collision, DT, &mut rng, |_| {
            Some(Vec2 { x: 600.0, y: 300.0 })
        });
    }
    assert!(kits.items.is_empty(), "not yet");

    for _ in 0..600 {
        kits.step(&config, &collision, DT, &mut rng, |_| {
            Some(Vec2 { x: 600.0, y: 300.0 })
        });
    }
    assert!(!kits.items.is_empty(), "kits were handed out");
    assert!(
        kits.items.len() <= config.max_on_map,
        "and never more than the cap: {}",
        kits.items.len()
    );
}

#[test]
fn a_map_with_nowhere_to_put_a_kit_simply_gets_none() {
    let collision = collision();
    let mut kits = Pickups::default();
    let mut rng = SimRng::seeded(3);
    let config = BonusConfig {
        frequency_ticks: 1,
        ..BonusConfig::default()
    };

    for _ in 0..500 {
        kits.step(&config, &collision, DT, &mut rng, |_| None);
    }
    assert!(kits.items.is_empty(), "no pile at the origin, just no kits");
}

#[test]
fn kit_spawning_is_deterministic_so_a_replay_agrees_with_the_match() {
    let run = || {
        let collision = collision();
        let mut kits = Pickups::default();
        let mut rng = SimRng::seeded(99);
        let config = BonusConfig {
            frequency_ticks: 10,
            ..BonusConfig::default()
        };
        for _ in 0..1_000 {
            kits.step(&config, &collision, DT, &mut rng, |rng| {
                Some(Vec2 {
                    x: (rng.next_u64() % 1000) as f32,
                    y: 300.0,
                })
            });
        }
        kits.items
            .iter()
            .map(|item| (item.kind, item.pos().x))
            .collect::<Vec<_>>()
    };
    assert_eq!(run(), run());
}

#[test]
fn a_round_reset_sweeps_the_kits_off_the_map() {
    let mut kits = Pickups::default();
    kits.items
        .push(Pickup::new(KitKind::Medic, Vec2 { x: 1.0, y: 2.0 }, 600));
    kits.spawn_timer = 100;

    kits.clear();
    assert!(kits.items.is_empty());
    assert_eq!(kits.spawn_timer, 0);
}

#[test]
fn the_rarer_kits_are_rarer_than_the_common_ones() {
    assert_eq!(KitKind::Medic.rarity(), 1, "always handed out when rolled");
    assert_eq!(KitKind::Grenades.rarity(), 1);
    // VESTBONUS_RANDOM, BERSERKERBONUS_RANDOM, CLUSTERBONUS_RANDOM are 4.
    assert_eq!(KitKind::Vest.rarity(), 4);
    assert_eq!(KitKind::Berserker.rarity(), 4);
    assert_eq!(KitKind::ClusterGrenades.rarity(), 4);
    // FLAMERBONUS_RANDOM and PREDATORBONUS_RANDOM are 5.
    assert_eq!(KitKind::FlameGod.rarity(), 5);
    assert_eq!(KitKind::Predator.rarity(), 5);
}

#[test]
fn a_cluster_kit_hands_over_three_grenades() {
    assert_eq!(CLUSTER_GRENADES, 3);
}
