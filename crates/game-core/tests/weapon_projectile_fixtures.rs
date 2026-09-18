//! Parity fixtures for projectile movement, terrain response, and the per-weapon firing rules.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:weapon_projectile_fixtures:styles        — lifetimes, gravity, terrain and body responses
//!   rust:weapon_projectile_fixtures:firing-rules  — LAW bracing, start-up, boost, barrels, spread
//!
//! Source: OpenSoldat `shared/mechanics/Bullets.pas` (`TBullet.Update`, `CheckMapCollision`,
//! `CheckMapVerticesCollision`, `CheckThingCollision`) and `shared/mechanics/Sprites.pas`
//! (`TSprite.Fire`, `GRENADE_SURFACECOEF`) at the commit pinned in
//! `docs/parity/reference-lock.md`.

use game_core::{
    barrel_origin, boosts_the_shooter, bounce_velocity, collides_with_bodies, firing,
    gravity_multiplier, grenade_is_armed, impact_response, melee_reach, per_pellet_spread,
    projectile_rules, pushes_objects, refusal, ricochet_velocity, self_boost, surface_response,
    swing_segment, BulletStyle, CharacterState, FireRefusal, ImpactResponse, ShooterPose,
    SurfaceResponse, Vec2, WeaponKind, WeaponTable, ARROW_RESIST_TICKS, GRENADE_SURFACE_COEF,
    MELEE_RADIUS, OBJECT_PUSH_MULTIPLIER, RICOCHET_MIN_TRAVEL,
};

const SOURCE_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";
const SOURCE_BULLETS: &str = "shared/mechanics/Bullets.pas:TBullet.Update/CheckMapCollision";
const SOURCE_SPRITES: &str = "shared/mechanics/Sprites.pas:TSprite.Fire";

fn pose(state: CharacterState, grounded: bool) -> ShooterPose {
    ShooterPose {
        state,
        grounded,
        moving: false,
        jetting: false,
        bink: 0,
    }
}

#[test]
fn fixture_metadata_is_pinned() {
    assert_eq!(SOURCE_COMMIT.len(), 40);
    assert!(SOURCE_BULLETS.contains("CheckMapCollision"));
    assert!(SOURCE_SPRITES.contains("TSprite.Fire"));
}

#[test]
fn every_selectable_weapon_has_its_own_lifetime_and_gravity() {
    let table = WeaponTable::normal();

    // Lifetimes come from the bullet style, which is why every plain-bullet weapon shares one.
    for kind in [
        WeaponKind::DesertEagles,
        WeaponKind::Mp5,
        WeaponKind::Ak74,
        WeaponKind::SteyrAug,
        WeaponKind::Ruger77,
        WeaponKind::Barrett,
        WeaponKind::Minimi,
        WeaponKind::Minigun,
        WeaponKind::Ussocom,
    ] {
        let def = table.get(kind);
        assert_eq!(def.bullet_style, BulletStyle::Plain, "{kind:?}");
        assert_eq!(def.timeout(), 420, "{kind:?} lifetime");
        assert_eq!(
            gravity_multiplier(def.bullet_style),
            0.0,
            "{kind:?} gravity"
        );
    }

    // The five that differ, each for its own reason.
    assert_eq!(
        table.get(WeaponKind::Spas12).bullet_style,
        BulletStyle::Shotgun
    );
    assert_eq!(table.get(WeaponKind::Spas12).timeout(), 420);
    assert_eq!(table.get(WeaponKind::M79).timeout(), 420);
    assert_eq!(table.get(WeaponKind::Law).timeout(), 420);
    assert_eq!(table.get(WeaponKind::CombatKnife).timeout(), 1);
    assert_eq!(table.get(WeaponKind::Chainsaw).timeout(), 1);
    assert_eq!(table.get(WeaponKind::FragGrenade).timeout(), 180);
    assert_eq!(table.get(WeaponKind::Flamer).timeout(), 32);
    assert_eq!(table.get(WeaponKind::StationaryGun).timeout(), 60);

    // Only the things that arc are pulled down.
    assert_eq!(gravity_multiplier(BulletStyle::FragGrenade), 1.0);
    assert_eq!(gravity_multiplier(BulletStyle::ClusterGrenade), 1.0);
    assert_eq!(gravity_multiplier(BulletStyle::ThrownKnife), 1.0);
    assert_eq!(gravity_multiplier(BulletStyle::Arrow), 0.5);
    assert_eq!(gravity_multiplier(BulletStyle::Plain), 0.0);
    assert_eq!(gravity_multiplier(BulletStyle::M79Grenade), 0.0);
}

#[test]
fn terrain_answers_each_bullet_style_differently() {
    assert_eq!(surface_response(BulletStyle::Plain), SurfaceResponse::Stop);
    assert_eq!(
        surface_response(BulletStyle::Shotgun),
        SurfaceResponse::Stop
    );
    assert_eq!(surface_response(BulletStyle::M2), SurfaceResponse::Stop);
    assert_eq!(
        surface_response(BulletStyle::ThrownKnife),
        SurfaceResponse::Stop
    );
    assert_eq!(
        surface_response(BulletStyle::FragGrenade),
        SurfaceResponse::Bounce {
            restitution: GRENADE_SURFACE_COEF
        }
    );
    assert_eq!(
        surface_response(BulletStyle::M79Grenade),
        SurfaceResponse::Ricochet {
            min_travel: RICOCHET_MIN_TRAVEL
        }
    );
    assert_eq!(
        surface_response(BulletStyle::Law),
        SurfaceResponse::Ricochet {
            min_travel: RICOCHET_MIN_TRAVEL
        }
    );
    assert_eq!(
        surface_response(BulletStyle::Arrow),
        SurfaceResponse::Stick {
            resist_ticks: ARROW_RESIST_TICKS
        }
    );
    assert_eq!(
        surface_response(BulletStyle::Flame),
        SurfaceResponse::Smother { ticks: 16 }
    );
    assert_eq!(
        surface_response(BulletStyle::ClusterGrenade),
        SurfaceResponse::Explode
    );
}

#[test]
fn bodies_answer_each_bullet_style_differently() {
    assert_eq!(impact_response(BulletStyle::Plain), ImpactResponse::Wound);
    assert_eq!(impact_response(BulletStyle::Knife), ImpactResponse::Wound);
    assert_eq!(
        impact_response(BulletStyle::M79Grenade),
        ImpactResponse::Explode
    );
    assert_eq!(impact_response(BulletStyle::Law), ImpactResponse::Explode);
    assert_eq!(impact_response(BulletStyle::Flame), ImpactResponse::Burn);
    // A live grenade rolls past people; its fuse decides when it matters.
    assert_eq!(
        impact_response(BulletStyle::FragGrenade),
        ImpactResponse::Ignore
    );
}

#[test]
fn a_grenade_bounces_off_a_floor_and_keeps_most_of_its_speed() {
    let down = Vec2 { x: 6.0, y: 8.0 };
    let floor_normal = Vec2 { x: 0.0, y: -1.0 };
    let bounced = bounce_velocity(down, floor_normal, GRENADE_SURFACE_COEF);

    // It still travels the same way along the floor and now travels back up it.
    assert!(bounced.x > 0.0 && bounced.y < 0.0);
    assert_eq!(bounced.x, 6.0 * GRENADE_SURFACE_COEF);
    assert_eq!(bounced.y, -8.0 * GRENADE_SURFACE_COEF);

    // Each bounce loses speed, so a grenade eventually settles instead of bouncing forever.
    assert!(bounced.length() < down.length());
    let again = bounce_velocity(bounced, floor_normal, GRENADE_SURFACE_COEF);
    assert!(again.length() < bounced.length());
}

#[test]
fn a_grenade_is_not_armed_for_the_first_two_ticks_of_its_flight() {
    let table = WeaponTable::normal();
    let grenade = table.get(WeaponKind::FragGrenade);
    let fresh = grenade.timeout();

    assert!(!grenade_is_armed(grenade, fresh));
    assert!(!grenade_is_armed(grenade, fresh - 1));
    assert!(grenade_is_armed(grenade, fresh - 2));

    // A bullet has nothing to arm, so it answers terrain the instant it is created.
    let rifle = table.get(WeaponKind::Ak74);
    assert!(grenade_is_armed(rifle, rifle.timeout()));
}

#[test]
fn a_rocket_skips_off_a_surface_it_reached_from_far_enough_away() {
    let SurfaceResponse::Ricochet { min_travel } = surface_response(BulletStyle::M79Grenade) else {
        panic!("an M79 round ricochets");
    };
    assert_eq!(min_travel, 50.0);

    let incoming = Vec2 { x: 30.0, y: 10.0 };
    let wall_normal = Vec2 { x: -1.0, y: 0.0 };
    let skipped = ricochet_velocity(incoming, wall_normal);

    // It keeps most of its forward run and gains a push back off the wall.
    assert!(skipped.x > 0.0, "kept some of its run: {skipped:?}");
    assert!(skipped.x < incoming.x, "but less than it arrived with");
    assert!(skipped.length() < incoming.length());
}

#[test]
fn an_arrow_sticks_where_it_lands_and_stops_wounding_anyone() {
    let SurfaceResponse::Stick { resist_ticks } = surface_response(BulletStyle::Arrow) else {
        panic!("an arrow sticks");
    };
    assert_eq!(resist_ticks, ARROW_RESIST_TICKS);

    // In flight it still hits people; once it is stuck it is scenery.
    assert!(collides_with_bodies(
        BulletStyle::Arrow,
        ARROW_RESIST_TICKS + 1
    ));
    assert!(!collides_with_bodies(
        BulletStyle::Arrow,
        ARROW_RESIST_TICKS
    ));
    assert!(!collides_with_bodies(BulletStyle::Arrow, 10));

    // Nothing else has a stuck state at all.
    assert!(collides_with_bodies(BulletStyle::Plain, 1));
}

#[test]
fn only_solid_rounds_shove_flags_and_kits_around() {
    assert!(pushes_objects(BulletStyle::Plain));
    assert!(pushes_objects(BulletStyle::Shotgun));
    assert!(pushes_objects(BulletStyle::M2));
    // A live grenade is meant to bounce past an objective, not launch it.
    assert!(!pushes_objects(BulletStyle::FragGrenade));
    assert!(!pushes_objects(BulletStyle::Flame));
    assert_eq!(OBJECT_PUSH_MULTIPLIER, 9.0);
}

#[test]
fn the_law_refuses_to_fire_unless_it_is_braced() {
    let table = WeaponTable::normal();
    let law = table.get(WeaponKind::Law);

    assert!(!firing::may_fire(
        law,
        &pose(CharacterState::Standing, true)
    ));
    assert_eq!(
        refusal(law, &pose(CharacterState::Standing, true)),
        Some(FireRefusal::NeedsBracing)
    );
    assert!(!firing::may_fire(
        law,
        &pose(CharacterState::Crouching, false)
    ));
    assert!(!firing::may_fire(
        law,
        &pose(CharacterState::Airborne, false)
    ));

    assert!(firing::may_fire(
        law,
        &pose(CharacterState::Crouching, true)
    ));
    assert!(firing::may_fire(law, &pose(CharacterState::Prone, true)));
    assert_eq!(refusal(law, &pose(CharacterState::Prone, true)), None);

    // No other weapon cares where it is fired from.
    let rifle = table.get(WeaponKind::Ak74);
    assert!(firing::may_fire(
        rifle,
        &pose(CharacterState::Airborne, false)
    ));
    assert_eq!(refusal(rifle, &pose(CharacterState::Airborne, false)), None);
}

#[test]
fn the_barrett_settles_and_the_minigun_spins_up_before_either_fires() {
    let table = WeaponTable::normal();

    assert_eq!(table.get(WeaponKind::Barrett).start_up_time, 19);
    assert_eq!(table.get(WeaponKind::Minigun).start_up_time, 25);
    assert_eq!(table.get(WeaponKind::Ak74).start_up_time, 0);

    // Letting go loses a minigun's spin, but the Barrett keeps its settled aim.
    assert!(startup_resets_on_release_for(WeaponKind::Minigun));
    assert!(!startup_resets_on_release_for(WeaponKind::Barrett));
    assert!(!startup_resets_on_release_for(WeaponKind::Ak74));
}

fn startup_resets_on_release_for(kind: WeaponKind) -> bool {
    game_core::startup_resets_on_release(WeaponTable::normal().get(kind))
}

#[test]
fn the_spas_and_the_minigun_shove_their_own_firer() {
    let table = WeaponTable::normal();
    let aim = Vec2 { x: 1.0, y: 0.0 };
    let standing = pose(CharacterState::Standing, true);

    assert!(boosts_the_shooter(table.get(WeaponKind::Spas12)));
    assert!(boosts_the_shooter(table.get(WeaponKind::Minigun)));
    assert!(!boosts_the_shooter(table.get(WeaponKind::Ak74)));

    let spas = self_boost(table.get(WeaponKind::Spas12), aim, &standing);
    assert!(spas.x < 0.0, "a shotgun blast pushes back along the barrel");
    assert_eq!(spas.x, -14.0 * 0.0412);

    // A minigun kicks less than a shotgun, and jetting bleeds nearly all of it away.
    let minigun = self_boost(table.get(WeaponKind::Minigun), aim, &standing);
    assert!(minigun.x < 0.0);
    assert!(minigun.x.abs() < spas.x.abs());
    let jetting = self_boost(
        table.get(WeaponKind::Minigun),
        aim,
        &ShooterPose {
            jetting: true,
            ..standing
        },
    );
    assert!(jetting.x.abs() < minigun.x.abs());

    // Everything else leaves the firer where they stood.
    assert_eq!(
        self_boost(table.get(WeaponKind::Ak74), aim, &standing).x,
        0.0
    );
}

#[test]
fn dual_eagles_fire_two_rounds_from_two_barrels() {
    let table = WeaponTable::normal();
    let eagles = table.get(WeaponKind::DesertEagles);
    assert_eq!(eagles.pellets(), 2);

    let muzzle = Vec2 { x: 100.0, y: 50.0 };
    let aim = Vec2 { x: 1.0, y: 0.0 };

    // The first round leaves the muzzle the aim points from; the second leaves the other barrel.
    assert_eq!(barrel_origin(eagles, muzzle, aim, 0), muzzle);
    let second = barrel_origin(eagles, muzzle, aim, 1);
    assert_ne!(second, muzzle);
    assert!(
        (second.y - muzzle.y).abs() > 0.0,
        "the barrels sit apart: {second:?}"
    );

    // Only the first round follows the aim exactly; the second is thrown off by the spread.
    assert_eq!(per_pellet_spread(eagles, 0, 0.5), 0.0);
    assert_ne!(per_pellet_spread(eagles, 1, 0.0), 0.0);

    // A rifle has one barrel, so the offset never applies.
    let rifle = table.get(WeaponKind::Ak74);
    assert_eq!(barrel_origin(rifle, muzzle, aim, 1), muzzle);
}

#[test]
fn a_shotgun_shell_scatters_six_pellets_across_its_spread() {
    let table = WeaponTable::normal();
    let spas = table.get(WeaponKind::Spas12);
    assert_eq!(spas.pellets(), 6);
    assert_eq!(spas.bullet_spread, 0.8);

    // Each pellet lands somewhere inside the spread, and the extremes reach its full width.
    assert_eq!(per_pellet_spread(spas, 0, 0.0), -spas.bullet_spread);
    assert_eq!(per_pellet_spread(spas, 5, 1.0), spas.bullet_spread);
    assert_eq!(per_pellet_spread(spas, 3, 0.5), 0.0);
    for pellet in 0..spas.pellets() {
        for roll in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let spread = per_pellet_spread(spas, pellet, roll);
            assert!(
                spread.abs() <= spas.bullet_spread,
                "pellet {pellet} at {roll}"
            );
        }
    }
}

#[test]
fn contact_weapons_reach_only_as_far_as_they_swing() {
    let table = WeaponTable::normal();
    let knife = table.get(WeaponKind::CombatKnife);
    let chainsaw = table.get(WeaponKind::Chainsaw);
    let rifle = table.get(WeaponKind::Ak74);

    // A swing lives one tick, so its reach is the distance it covers in that tick.
    assert_eq!(melee_reach(knife), knife.speed + 2.0);
    assert!(melee_reach(chainsaw) > 0.0);
    assert_eq!(melee_reach(rifle), 0.0, "a rifle does not swing");

    // The swing starts behind the hands so someone pressed right up against you is still cut.
    let hands = Vec2 { x: 100.0, y: 50.0 };
    let aim = Vec2 { x: 1.0, y: 0.0 };
    let (start, end) = swing_segment(hands, aim, melee_reach(knife));
    assert!(start.x < hands.x, "the blade sweeps from behind the hands");
    assert!(end.x > hands.x);
    assert!(end.x - start.x > melee_reach(knife));
    assert_eq!(MELEE_RADIUS, 7.0);
}

#[test]
fn the_chainsaw_cuts_continuously_and_the_knife_stabs_once() {
    let table = WeaponTable::normal();
    let chainsaw = table.get(WeaponKind::Chainsaw);
    let knife = table.get(WeaponKind::CombatKnife);

    assert!(game_core::is_continuous_contact(chainsaw));
    assert!(!game_core::is_continuous_contact(knife));

    // The chainsaw's two-tick interval and one-tick swing leave no gap worth dodging through.
    assert_eq!(chainsaw.fire_interval, 2);
    assert_eq!(chainsaw.timeout(), 1);
    assert!(knife.fire_interval > chainsaw.fire_interval);
}

#[test]
fn projectile_rules_are_reachable_as_one_module() {
    // The behavior of a weapon is composed from its style, never keyed off a weapon number in the
    // simulation loop; this is the seam that keeps it that way.
    assert_eq!(
        projectile_rules::surface_response(BulletStyle::Plain),
        SurfaceResponse::Stop
    );
    assert_eq!(projectile_rules::GRENADE_ARM_TICKS, 2);
}
