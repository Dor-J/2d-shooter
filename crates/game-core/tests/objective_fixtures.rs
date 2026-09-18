//! Parity fixtures for the shared objective framework: the flag object itself.
//!
//! Acceptance evidence for docs/parity/coverage.json:
//!   rust:objective:flag-states — base, carried, dropped, thrown, and the return timeout
//!   rust:objective:flag-physics — falling, terrain collision, and being shoved
//!
//! Source: OpenSoldat `shared/mechanics/Things.pas` (`TThing.Update`, `CheckSpriteCollision`) and
//! the `FLAG_TIMEOUT` / `TOUCHDOWN_RADIUS` constants at the commit pinned in
//! `docs/parity/reference-lock.md`.

use game_core::{
    CollisionWorld, Flag, FlagEvent, FlagKind, FlagState, Vec2, World, FLAG_PICKUP_RADIUS,
    FLAG_TIMEOUT, TOUCHDOWN_RADIUS,
};

const SOURCE_COMMIT: &str = "c7596cdca32416cb66339b339105eb1e07e7fbbf";
const SOURCE_THINGS: &str = "shared/mechanics/Things.pas:TThing.Update/CheckSpriteCollision";

fn empty_world() -> CollisionWorld {
    World::new("deathmatch").collision().clone()
}

#[test]
fn fixture_metadata_is_pinned() {
    assert_eq!(SOURCE_COMMIT.len(), 40);
    assert!(SOURCE_THINGS.contains("Things.pas"));
    assert_eq!(FLAG_TIMEOUT, 60 * 25, "a dropped flag waits 25 seconds");
    assert_eq!(TOUCHDOWN_RADIUS, 28.0);
}

#[test]
fn a_flag_starts_on_its_base_and_knows_whose_it_is() {
    let base = Vec2 { x: 200.0, y: 400.0 };
    let flag = Flag::new(FlagKind::Alpha, base);

    assert_eq!(flag.state, FlagState::AtBase);
    assert_eq!(flag.pos(), base);
    assert!(flag.is_home_and_free());

    assert!(FlagKind::Alpha.belongs_to(1));
    assert!(!FlagKind::Alpha.belongs_to(2));
    assert!(FlagKind::Bravo.belongs_to(2));
    // The neutral flag belongs to nobody, which is what makes it anybody's to take.
    assert!(!FlagKind::Yellow.belongs_to(1));
    assert!(!FlagKind::Yellow.belongs_to(2));
    assert_eq!(FlagKind::Yellow.owning_team(), 0);
}

#[test]
fn a_flag_can_be_taken_only_once_and_carried_by_whoever_took_it() {
    let mut flag = Flag::new(FlagKind::Bravo, Vec2 { x: 900.0, y: 400.0 });

    assert_eq!(
        flag.take(7),
        Some(FlagEvent::Taken {
            flag: FlagKind::Bravo,
            by: 7
        })
    );
    assert_eq!(flag.state.carrier(), Some(7));
    assert!(!flag.is_home_and_free(), "a carried flag is not home");

    // Somebody else cannot take it out of the carrier's hands.
    assert_eq!(flag.take(8), None);
    assert_eq!(flag.state.carrier(), Some(7));
}

#[test]
fn dropping_a_flag_leaves_it_where_the_carrier_was_with_their_momentum() {
    let mut flag = Flag::new(FlagKind::Bravo, Vec2 { x: 900.0, y: 400.0 });
    flag.take(7);

    let where_they_fell = Vec2 { x: 500.0, y: 300.0 };
    let momentum = Vec2 { x: 40.0, y: -20.0 };
    assert_eq!(
        flag.drop_at(where_they_fell, momentum),
        Some(FlagEvent::Dropped {
            flag: FlagKind::Bravo,
            by: 7
        })
    );
    assert_eq!(flag.pos(), where_they_fell);
    assert_eq!(flag.body.vel, momentum, "it keeps going the way they were");
    assert_eq!(
        flag.state,
        FlagState::Dropped {
            ticks_left: FLAG_TIMEOUT
        }
    );

    // A flag nobody is holding cannot be dropped again.
    assert_eq!(flag.drop_at(Vec2::default(), Vec2::default()), None);
}

#[test]
fn a_dropped_flag_takes_itself_home_when_nobody_comes_for_it() {
    let collision = empty_world();
    let base = Vec2 { x: 900.0, y: 400.0 };
    let mut flag = Flag::new(FlagKind::Bravo, base);
    flag.take(7);
    flag.drop_at(Vec2 { x: 300.0, y: 300.0 }, Vec2::default());

    assert_eq!(flag.return_countdown(), Some(FLAG_TIMEOUT));

    let mut returned = None;
    for _ in 0..FLAG_TIMEOUT + 5 {
        if let Some(event) = flag.step(&collision, 1.0 / 60.0, None) {
            returned = Some(event);
            break;
        }
    }
    assert_eq!(
        returned,
        Some(FlagEvent::Returned {
            flag: FlagKind::Bravo,
            by: None
        }),
        "it went home on its own"
    );
    assert_eq!(flag.state, FlagState::AtBase);
    assert_eq!(flag.pos(), base);
}

#[test]
fn a_dropped_flag_falls_and_lands_on_the_terrain_rather_than_through_it() {
    let collision = empty_world();
    let mut flag = Flag::new(FlagKind::Yellow, Vec2 { x: 600.0, y: 100.0 });
    flag.take(1);
    flag.drop_at(Vec2 { x: 600.0, y: 100.0 }, Vec2::default());

    let start = flag.pos().y;
    for _ in 0..120 {
        flag.step(&collision, 1.0 / 60.0, None);
    }
    assert!(flag.pos().y > start, "it fell");
    assert!(
        flag.pos().y < 700.0,
        "and stopped on the floor rather than falling out of the world: {:?}",
        flag.pos()
    );
    assert!(flag.body.grounded, "it is resting on something");
}

#[test]
fn a_carried_flag_rides_along_with_its_carrier() {
    let collision = empty_world();
    let mut flag = Flag::new(FlagKind::Alpha, Vec2 { x: 200.0, y: 400.0 });
    flag.take(3);

    let carrier = Vec2 { x: 640.0, y: 220.0 };
    flag.step(&collision, 1.0 / 60.0, Some(carrier));
    assert_eq!(flag.pos(), carrier);
    assert_eq!(
        flag.body.vel,
        Vec2::default(),
        "it does not fall while held"
    );
}

#[test]
fn a_flag_at_base_stays_put_however_long_the_match_runs() {
    let collision = empty_world();
    let base = Vec2 { x: 200.0, y: 100.0 };
    let mut flag = Flag::new(FlagKind::Alpha, base);

    for _ in 0..600 {
        assert_eq!(flag.step(&collision, 1.0 / 60.0, None), None);
    }
    assert_eq!(flag.pos(), base, "a flag on its base does not drift");
}

#[test]
fn only_a_loose_flag_can_be_touched_or_shoved() {
    let mut flag = Flag::new(FlagKind::Yellow, Vec2 { x: 500.0, y: 400.0 });
    let close = Vec2 {
        x: 500.0 + FLAG_PICKUP_RADIUS - 1.0,
        y: 400.0,
    };
    let far = Vec2 {
        x: 500.0 + FLAG_PICKUP_RADIUS + 5.0,
        y: 400.0,
    };

    assert!(flag.within_reach(close));
    assert!(!flag.within_reach(far), "you have to be standing on it");

    flag.push(Vec2 { x: 50.0, y: -30.0 });
    assert_ne!(flag.body.vel, Vec2::default(), "a blast moves it");

    // Once somebody has it, it is out of reach and cannot be shoved out of their hands.
    flag.take(2);
    assert!(!flag.within_reach(close));
    let held = flag.body.vel;
    flag.push(Vec2 { x: 500.0, y: 0.0 });
    assert_eq!(flag.body.vel, held, "a carried flag is not blown around");
}

#[test]
fn a_touchdown_needs_the_two_flags_to_be_close_together() {
    let home = Flag::new(FlagKind::Alpha, Vec2 { x: 200.0, y: 400.0 });
    let mut carried = Flag::new(FlagKind::Bravo, Vec2 { x: 900.0, y: 400.0 });
    carried.take(1);

    carried.body.pos = Vec2 {
        x: 200.0 + TOUCHDOWN_RADIUS + 1.0,
        y: 400.0,
    };
    assert!(!carried.touching_down_on(&home), "not close enough yet");

    carried.body.pos = Vec2 {
        x: 200.0 + TOUCHDOWN_RADIUS - 1.0,
        y: 400.0,
    };
    assert!(carried.touching_down_on(&home));
}

#[test]
fn a_thrown_flag_carries_the_throwers_momentum_plus_a_push_along_the_aim() {
    let still = Flag::throw_velocity(Vec2 { x: 1.0, y: 0.0 }, Vec2::default());
    assert!(still.x > 0.0, "it goes where it was thrown");
    assert_eq!(still.y, 0.0);

    let running = Flag::throw_velocity(Vec2 { x: 1.0, y: 0.0 }, Vec2 { x: 100.0, y: 0.0 });
    assert_eq!(
        running.x - still.x,
        100.0,
        "a running throw goes further by exactly the runner's speed"
    );

    // Throwing at your own feet is not a divide by zero.
    let nowhere = Flag::throw_velocity(Vec2::default(), Vec2 { x: 5.0, y: 5.0 });
    assert_eq!(nowhere, Vec2 { x: 5.0, y: 5.0 });
}
