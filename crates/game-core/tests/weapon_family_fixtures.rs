//! Parity fixtures for the explosive, flame, and stationary weapon families.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:weapon_family_fixtures:explosive   — throw charge, arc, cluster scatter, blast falloff
//!   rust:weapon_family_fixtures:flame       — flame propagation and fuel
//!   rust:weapon_family_fixtures:stationary  — M2 mount, aim, overheat
//!
//! Source: OpenSoldat `shared/mechanics/Sprites.pas` (`TSprite.ThrowGrenade`),
//! `shared/mechanics/Bullets.pas` (`HIT_TYPE_CLUSTERNADE`, the flame hit branch), and
//! `shared/mechanics/Things.pas` (`TThing.CheckStationaryGunCollision`) at the commit pinned in
//! `docs/parity/reference-lock.md`.

use game_core::{
    cluster_submunitions, explosion_damage, explosive, flame, propagated_hit_multiply,
    propagated_velocity, propagates, stationary, throw_arc, throw_frame, throw_velocity,
    BodyRegion, SimRng, StationaryGun, Vec2, WeaponKind, WeaponTable, CLUSTER_SUBMUNITIONS,
    THROW_LAST_FRAME,
};

const SOURCE_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";
const SOURCE_THROW: &str = "shared/mechanics/Sprites.pas:TSprite.ThrowGrenade";
const SOURCE_CLUSTER: &str = "shared/mechanics/Bullets.pas:TBullet.Hit(HIT_TYPE_CLUSTERNADE)";
const SOURCE_STATGUN: &str = "shared/mechanics/Things.pas:TThing.CheckStationaryGunCollision";

#[test]
fn fixture_metadata_is_pinned() {
    assert_eq!(SOURCE_COMMIT.len(), 40);
    assert!(SOURCE_THROW.contains("ThrowGrenade"));
    assert!(SOURCE_CLUSTER.contains("CLUSTERNADE"));
    assert!(SOURCE_STATGUN.contains("StationaryGun"));
}

#[test]
fn holding_the_throw_button_charges_the_grenade_up_to_its_full_strength() {
    let table = WeaponTable::normal();
    let grenade = table.get(WeaponKind::FragGrenade);
    let aim = Vec2 { x: 1.0, y: 0.0 };
    let still = Vec2 { x: 0.0, y: 0.0 };

    // The throw starts part-charged and reaches full strength after being held.
    assert_eq!(throw_frame(0), 15);
    assert_eq!(throw_frame(9), 24);
    assert_eq!(throw_frame(100), THROW_LAST_FRAME);

    let flick = throw_velocity(grenade, aim, 0, still).length();
    let held = throw_velocity(grenade, aim, 9, still).length();
    let full = throw_velocity(grenade, aim, 100, still).length();

    assert!(flick < held, "a flick is weaker than a held throw");
    assert!(held < full, "holding longer throws further");
    // A throw released early is deliberately soft, not merely a little weaker.
    assert!(flick < held * 0.8);
    assert_eq!(full, f32::from(THROW_LAST_FRAME) / grenade.speed);
}

#[test]
fn a_thrown_grenade_carries_the_throwers_momentum() {
    let table = WeaponTable::normal();
    let grenade = table.get(WeaponKind::FragGrenade);
    let aim = Vec2 { x: 1.0, y: 0.0 };

    assert_eq!(
        grenade.inherited_velocity, 1.0,
        "a grenade inherits all of it"
    );

    let running = throw_velocity(grenade, aim, 100, Vec2 { x: 3.0, y: 0.0 });
    let backing = throw_velocity(grenade, aim, 100, Vec2 { x: -3.0, y: 0.0 });
    let still = throw_velocity(grenade, aim, 100, Vec2 { x: 0.0, y: 0.0 });

    assert!(running.x > still.x, "running forwards throws further");
    assert!(backing.x < still.x, "backing away throws shorter");
    assert_eq!(running.x - still.x, 3.0);
}

#[test]
fn a_throw_arcs_sideways_and_stops_arcing_when_aimed_straight_up() {
    let sideways = throw_arc(Vec2 { x: 1.0, y: 0.0 });
    assert!(sideways.y < 0.0, "a flat throw is lobbed slightly upwards");

    let straight_up = throw_arc(Vec2 { x: 0.0, y: -1.0 });
    assert!(
        (straight_up.x).abs() < 1e-5,
        "there is nothing to arc around when aiming straight up: {straight_up:?}"
    );

    // The arc never changes how far the throw goes, only where it points.
    for aim in [
        Vec2 { x: 1.0, y: 0.0 },
        Vec2 { x: -1.0, y: 0.0 },
        Vec2 { x: 0.0, y: -1.0 },
    ] {
        assert!((throw_arc(aim).length() - 1.0).abs() < 1e-4, "{aim:?}");
    }
}

#[test]
fn a_cluster_grenade_scatters_five_submunitions_that_spray_back_out_of_the_impact() {
    let mut rng = SimRng::seeded(7);
    let incoming = Vec2 { x: 4.0, y: 2.0 };
    let pieces = cluster_submunitions(incoming, &mut rng);

    assert_eq!(pieces.len(), usize::from(CLUSTER_SUBMUNITIONS));
    assert_eq!(CLUSTER_SUBMUNITIONS, 5);

    // They do not all fly the same way, and none of them simply follows the parent onwards.
    let all_same = pieces.windows(2).all(|pair| pair[0] == pair[1]);
    assert!(!all_same, "the pieces scatter: {pieces:?}");

    // The same seed always scatters them the same way, so a replay stays honest.
    let mut again = SimRng::seeded(7);
    assert_eq!(cluster_submunitions(incoming, &mut again), pieces);
}

#[test]
fn a_cluster_piece_hits_for_half_of_what_the_parent_grenade_would() {
    let table = WeaponTable::normal();
    let frag = table.get(WeaponKind::FragGrenade);
    let piece = table.get(WeaponKind::Cluster);

    assert_eq!(
        explosive::submunition_hit_multiply(frag),
        frag.damage / 2.0,
        "a piece carries half the charge"
    );

    // The blast a piece makes counts half, and reaches nowhere near as far.
    assert_eq!(piece.explosion_radius(), 35.0);
    assert_eq!(frag.explosion_radius(), 85.0);
    let at_contact_piece = explosion_damage(piece, 0.0, BodyRegion::Chest);
    let at_contact_frag = explosion_damage(frag, 0.0, BodyRegion::Chest);
    assert_eq!(at_contact_piece, at_contact_frag / 2.0);

    // Standing back is what really saves you: a frag still bites at forty units, a piece cannot
    // reach that far at all.
    assert!(explosion_damage(frag, 40.0, BodyRegion::Chest) > 0.0);
    assert_eq!(explosion_damage(piece, 40.0, BodyRegion::Chest), 0.0);
}

#[test]
fn an_explosion_falls_off_with_distance_and_stops_at_its_radius() {
    let table = WeaponTable::normal();
    let m79 = table.get(WeaponKind::M79);

    let point_blank = explosion_damage(m79, 0.0, BodyRegion::Chest);
    let close = explosion_damage(m79, 10.0, BodyRegion::Chest);
    let edge = explosion_damage(m79, 63.0, BodyRegion::Chest);

    assert!(point_blank > close && close > edge);
    assert!(point_blank >= 100.0, "a direct hit kills: {point_blank}");
    assert_eq!(explosion_damage(m79, 64.0, BodyRegion::Chest), 0.0);
    assert_eq!(explosion_damage(m79, 500.0, BodyRegion::Chest), 0.0);
}

#[test]
fn a_flame_that_catches_passes_the_fire_on_once_and_then_burns_out() {
    let table = WeaponTable::normal();
    let flamer = table.get(WeaponKind::Flamer);

    // A fresh flame is hot enough to set someone alight.
    assert!(propagates(flamer, flamer.damage, 0));
    // The flame it throws off is weaker.
    let passed_on = propagated_hit_multiply(flamer.damage);
    assert!(passed_on < flamer.damage);
    assert_eq!(passed_on, flamer.damage * 2.0 / 3.0);

    // It only spreads once, so a single squirt cannot burn forever.
    assert!(!propagates(flamer, passed_on, 1));
    // And a flame that has cooled too far spreads nothing even on its first contact.
    assert!(!propagates(flamer, flamer.damage / 4.0, 0));

    // The fire trails whoever is carrying it rather than flying off on its own.
    let victim = Vec2 { x: 5.0, y: -2.0 };
    assert_eq!(propagated_velocity(victim), Vec2 { x: -5.0, y: 2.0 });
}

#[test]
fn the_flamer_burns_its_magazine_as_fuel() {
    let table = WeaponTable::normal();
    let flamer = table.get(WeaponKind::Flamer);

    assert_eq!(flamer.ammo, 200, "the tank is the magazine");
    assert!(flame::has_fuel(1));
    assert!(!flame::has_fuel(0));
    assert_eq!(flame::FUEL_PER_SHOT, 1);
}

#[test]
fn the_m2_can_only_be_taken_hold_of_from_close_by_and_only_by_one_player() {
    let mut gun = StationaryGun::new(Vec2 { x: 100.0, y: 100.0 });

    // It takes a moment to deploy before anyone can use it.
    assert!(!gun.in_reach(Vec2 { x: 100.0, y: 100.0 }));
    for _ in 0..stationary::DEPLOY_TICKS {
        gun.step(0, false);
    }
    assert!(gun.in_reach(Vec2 { x: 100.0, y: 100.0 }));
    assert!(!gun.in_reach(Vec2 { x: 100.0, y: 130.0 }), "out of reach");

    assert!(gun.mount(1, Vec2 { x: 105.0, y: 100.0 }));
    assert_eq!(gun.mounted_by, Some(1));
    assert!(!gun.mount(2, Vec2 { x: 105.0, y: 100.0 }), "already taken");

    gun.dismount(2);
    assert_eq!(gun.mounted_by, Some(1), "only its user may let go");
    gun.dismount(1);
    assert_eq!(gun.mounted_by, None);
    assert!(gun.mount(2, Vec2 { x: 105.0, y: 100.0 }));
}

#[test]
fn the_m2_barrel_swings_to_the_aim_and_shoots_from_its_own_muzzle() {
    let mut gun = StationaryGun::new(Vec2 { x: 100.0, y: 100.0 });
    gun.deploy = 0;

    gun.aim_at(Vec2 { x: 200.0, y: 100.0 });
    assert_eq!(gun.barrel, Vec2 { x: 1.0, y: 0.0 });
    assert_eq!(gun.muzzle().x, 100.0 + stationary::BARREL_LENGTH);
    assert_eq!(gun.muzzle().y, 100.0);

    gun.aim_at(Vec2 { x: 100.0, y: 0.0 });
    assert_eq!(gun.barrel, Vec2 { x: 0.0, y: -1.0 });
    assert_eq!(gun.muzzle().y, 100.0 - stationary::BARREL_LENGTH);

    // Aiming at the mount itself leaves the barrel where it was rather than dividing by zero.
    let before = gun.barrel;
    gun.aim_at(Vec2 { x: 100.0, y: 100.0 });
    assert_eq!(gun.barrel, before);
}

#[test]
fn the_m2_overheats_when_held_down_and_cools_when_released() {
    let table = WeaponTable::normal();
    let interval = u64::from(table.get(WeaponKind::StationaryGun).fire_interval);
    let mut gun = StationaryGun::new(Vec2 { x: 0.0, y: 0.0 });
    gun.deploy = 0;
    gun.mount(1, Vec2 { x: 0.0, y: 0.0 });

    assert!(gun.can_fire(0, &table), "a cool gun on its interval fires");
    assert!(!gun.can_fire(1, &table), "and only on its interval");

    // Hold it down and the barrel gives out.
    for _ in 0..=stationary::OVERHEAT_LIMIT {
        gun.step(0, true);
    }
    assert!(gun.overheated());
    assert!(!gun.can_fire(interval * 4, &table));

    // Let go and it comes back.
    for tick in 0..400u64 {
        gun.step(tick * stationary::COOLDOWN_TICKS, false);
    }
    assert!(!gun.overheated());
    assert!(gun.can_fire(0, &table));
}

#[test]
fn the_m2_aim_wanders_only_once_it_has_been_fired_for_a_while() {
    let mut gun = StationaryGun::new(Vec2 { x: 0.0, y: 0.0 });
    gun.deploy = 0;

    gun.use_time = stationary::OVERAIM_LIMIT;
    assert_eq!(gun.aim_wander(0.5), 0.0, "a steady barrel shoots straight");

    gun.use_time = stationary::OVERHEAT_LIMIT;
    let low = gun.aim_wander(0.0);
    let high = gun.aim_wander(1.0);
    assert!(
        low < 0.0 && high > 0.0,
        "it wanders either way: {low} {high}"
    );
    assert_eq!(gun.aim_wander(0.5), 0.0, "and is centred on the aim");
}

#[test]
fn an_unmounted_m2_never_fires() {
    let table = WeaponTable::normal();
    let mut gun = StationaryGun::new(Vec2 { x: 0.0, y: 0.0 });
    gun.deploy = 0;
    assert!(!gun.can_fire(0, &table));
    gun.mount(1, Vec2 { x: 0.0, y: 0.0 });
    assert!(gun.can_fire(0, &table));
    gun.dismount(1);
    assert!(!gun.can_fire(0, &table));
}
