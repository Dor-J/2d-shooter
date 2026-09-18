use game_core::{
    dropped_weapon, muzzle_origin, BodyRegion, CharacterState, DamageCause, DamageEvent, Direction,
    DynamicBodyKind, Event, Input, Inventory, ProjectileKind, Vec2, WeaponKind, WeaponTable, World,
    PICKUP_RADIUS, SWITCH_DELAY_TICKS,
};
use std::collections::BTreeMap;

fn table() -> WeaponTable {
    WeaponTable::normal()
}

fn fire_at(x: f32, y: f32) -> Input {
    Input {
        fire: true,
        aim: Vec2 { x, y },
        ..Input::default()
    }
}

fn step(world: &mut World, id: u32, input: Input) {
    world.step(&BTreeMap::from([(id, input)]));
}

fn wait_switch(world: &mut World, id: u32, weapon: u8) {
    for _ in 0..SWITCH_DELAY_TICKS {
        step(
            world,
            id,
            Input {
                weapon,
                ..Input::default()
            },
        );
    }
}

#[test]
fn default_loadout_is_eagles_and_ussocom() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    let inventory = &world.players[&1].inventory;
    assert_eq!(
        inventory.slots[0].map(|slot| slot.kind),
        Some(WeaponKind::DesertEagles)
    );
    assert_eq!(
        inventory.slots[1].map(|slot| slot.kind),
        Some(WeaponKind::Ussocom)
    );
    assert_eq!(world.players[&1].weapon, 0);
}

#[test]
fn two_slots_can_hold_two_primaries() {
    let mut inventory = Inventory::spawn(0, &table());
    assert!(inventory.take_kind(WeaponKind::Ussocom).is_some());
    assert!(inventory
        .pickup(game_core::WeaponSlot {
            kind: WeaponKind::Mp5,
            ammo: 30,
        })
        .is_none());
    assert!(inventory.owns(WeaponKind::DesertEagles));
    assert!(inventory.owns(WeaponKind::Mp5));
    assert!(inventory.empty_index().is_none());
}

#[test]
fn a_third_weapon_swaps_the_active_slot() {
    let mut inventory = Inventory::spawn(0, &table());
    let displaced = inventory.pickup(game_core::WeaponSlot {
        kind: WeaponKind::Ak74,
        ammo: 20,
    });
    assert_eq!(
        displaced.map(|slot| slot.kind),
        Some(WeaponKind::DesertEagles)
    );
    assert!(inventory.owns(WeaponKind::Ak74));
    assert!(inventory.owns(WeaponKind::Ussocom));
    assert!(!inventory.owns(WeaponKind::DesertEagles));
}

#[test]
fn alive_players_cannot_select_a_weapon_they_do_not_carry() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    step(
        &mut world,
        1,
        Input {
            weapon: 7,
            ..Input::default()
        },
    );
    assert_eq!(world.players[&1].weapon, 0);
    assert!(!world.players[&1].inventory.owns(WeaponKind::Barrett));
}

#[test]
fn a_dead_player_picks_the_respawn_primary() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.add_player(2, "B".into());
    world.apply_damage(DamageEvent {
        attacker: Some(1),
        target: 2,
        amount: 500,
        region: BodyRegion::Chest,
        cause: DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
        pre_scaled: false,
    });
    let chosen = Input {
        weapon: 5,
        ..Input::default()
    };
    for _ in 0..world.respawn.delay_ticks + 2 {
        step(&mut world, 2, chosen);
    }
    assert_eq!(world.players[&2].hp, 100);
    assert_eq!(world.players[&2].weapon, 5);
    assert!(world.players[&2].inventory.owns(WeaponKind::Ruger77));
    assert!(world.players[&2].inventory.owns(WeaponKind::Ussocom));
}

#[test]
fn switching_waits_the_selection_delay() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    step(
        &mut world,
        1,
        Input {
            weapon: 10,
            ..Input::default()
        },
    );
    assert_eq!(world.players[&1].weapon, 0);
    assert!(world.players[&1].switch_timer > 0);
    wait_switch(&mut world, 1, 10);
    assert_eq!(world.players[&1].weapon, 10);
    assert_eq!(
        world.players[&1].inventory.active().map(|slot| slot.kind),
        Some(WeaponKind::Ussocom)
    );
}

#[test]
fn switching_keeps_each_slot_ammo() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    let full = world.players[&1].ammo;
    world.players.get_mut(&1).unwrap().cooldown = 0;
    step(&mut world, 1, fire_at(1000.0, 430.0));
    assert_eq!(world.players[&1].ammo, full - 1);
    wait_switch(&mut world, 1, 10);
    wait_switch(&mut world, 1, 0);
    assert_eq!(world.players[&1].ammo, full - 1);
}

#[test]
fn switching_interrupts_reload_without_a_refill() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.players.get_mut(&1).unwrap().ammo = 1;
    world.players.get_mut(&1).unwrap().cooldown = 0;
    step(&mut world, 1, fire_at(1000.0, 430.0));
    assert_eq!(world.players[&1].ammo, 0);
    assert!(world.players[&1].reload_timer > 0);
    step(
        &mut world,
        1,
        Input {
            weapon: 10,
            ..Input::default()
        },
    );
    assert_eq!(world.players[&1].reload_timer, 0);
    assert_eq!(
        world.players[&1].inventory.slots[0]
            .map(|slot| slot.ammo)
            .unwrap_or(1),
        0
    );
}

#[test]
fn manual_reload_emits_an_event_and_refills() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.players.get_mut(&1).unwrap().ammo = 3;
    step(
        &mut world,
        1,
        Input {
            reload: true,
            ..Input::default()
        },
    );
    assert!(world
        .events
        .iter()
        .any(|event| matches!(event, Event::Reload { player: 1 })));
    let reload = table().for_slot(0).reload_time;
    for _ in 0..reload {
        world.step(&BTreeMap::new());
    }
    assert_eq!(world.players[&1].ammo, u16::from(table().for_slot(0).ammo));
}

#[test]
fn empty_click_emits_empty_and_does_not_shoot() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.players.get_mut(&1).unwrap().ammo = 0;
    world.players.get_mut(&1).unwrap().reload_timer = 0;
    let before = world.projectiles.len();
    step(&mut world, 1, fire_at(1000.0, 430.0));
    assert_eq!(world.projectiles.len(), before);
    assert!(world
        .events
        .iter()
        .any(|event| matches!(event, Event::Empty { player: 1 })));
}

#[test]
fn drop_puts_the_active_weapon_on_the_ground() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    step(
        &mut world,
        1,
        Input {
            drop: true,
            ..Input::default()
        },
    );
    assert_eq!(world.objects.len(), 1);
    assert_eq!(world.objects[0].kind, DynamicBodyKind::DroppedWeapon);
    assert_eq!(world.objects[0].weapon_slot, Some(0));
    assert_eq!(world.players[&1].weapon, 10);
}

#[test]
fn pickup_takes_the_nearest_ground_weapon() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    let pos = world.players[&1].pos;
    world.objects.push(dropped_weapon(
        WeaponKind::Mp5,
        20,
        Vec2 {
            x: pos.x + 8.0,
            y: pos.y,
        },
        Vec2::default(),
    ));
    step(
        &mut world,
        1,
        Input {
            pickup: true,
            ..Input::default()
        },
    );
    assert!(world.players[&1].inventory.owns(WeaponKind::Mp5));
    assert!(!world.players[&1].inventory.owns(WeaponKind::DesertEagles));
}

#[test]
fn the_lower_player_id_wins_a_pickup_race() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "First".into());
    world.add_player(2, "Second".into());
    let pos = world.players[&1].pos;
    world.players.get_mut(&2).unwrap().pos = pos;
    world
        .objects
        .push(dropped_weapon(WeaponKind::Ak74, 15, pos, Vec2::default()));
    world.step(&BTreeMap::from([
        (
            1,
            Input {
                pickup: true,
                ..Input::default()
            },
        ),
        (
            2,
            Input {
                pickup: true,
                ..Input::default()
            },
        ),
    ]));
    assert!(world.players[&1].inventory.owns(WeaponKind::Ak74));
    assert!(!world.players[&2].inventory.owns(WeaponKind::Ak74));
}

#[test]
fn a_longer_charge_throws_harder() {
    let light = game_core::charged_velocity(Vec2 { x: 1.0, y: 0.0 }, 1, Vec2::default());
    let heavy = game_core::charged_velocity(Vec2 { x: 1.0, y: 0.0 }, 36, Vec2::default());
    assert!(heavy.x > light.x);
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    for _ in 0..8 {
        step(
            &mut world,
            1,
            Input {
                throw_weapon: true,
                aim: Vec2 {
                    x: 1000.0,
                    y: 430.0,
                },
                ..Input::default()
            },
        );
    }
    step(
        &mut world,
        1,
        Input {
            aim: Vec2 {
                x: 1000.0,
                y: 430.0,
            },
            ..Input::default()
        },
    );
    assert_eq!(world.objects.len(), 1);
    assert!(world.objects[0].vel.x > 100.0);
}

#[test]
fn throwing_the_knife_recovers_it_on_terrain() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.equip(1, WeaponKind::CombatKnife);
    step(
        &mut world,
        1,
        Input {
            throw_knife: true,
            weapon: 11,
            aim: Vec2 { x: 400.0, y: 800.0 },
            ..Input::default()
        },
    );
    assert!(world
        .projectiles
        .iter()
        .any(|projectile| projectile.kind == ProjectileKind::ThrownKnife));
    for _ in 0..120 {
        world.step(&BTreeMap::new());
        if world
            .objects
            .iter()
            .any(|object| object.weapon_slot == Some(11))
        {
            break;
        }
    }
    assert!(world
        .objects
        .iter()
        .any(|object| object.weapon_slot == Some(11)));
    assert!(!world.players[&1].inventory.owns(WeaponKind::CombatKnife));
}

#[test]
fn death_drops_both_slots() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.add_player(2, "B".into());
    world.apply_damage(DamageEvent {
        attacker: Some(1),
        target: 2,
        amount: 500,
        region: BodyRegion::Chest,
        cause: DamageCause::Bullet,
        direction: Vec2 { x: 1.0, y: 0.0 },
        pre_scaled: false,
    });
    assert_eq!(
        world
            .objects
            .iter()
            .filter(|object| object.kind == DynamicBodyKind::DroppedWeapon)
            .count(),
        2
    );
    assert!(world.players[&2].inventory.active().is_none());
}

#[test]
fn muzzle_origin_moves_with_pose() {
    let pos = Vec2 { x: 100.0, y: 200.0 };
    let aim = Vec2 { x: 200.0, y: 200.0 };
    let standing = muzzle_origin(pos, aim, CharacterState::Standing, Direction::Right);
    let prone = muzzle_origin(pos, aim, CharacterState::Prone, Direction::Right);
    assert!(standing.y < prone.y);
    assert!(standing.x > pos.x);
}

#[test]
fn firing_emits_muzzle_flash_from_the_barrel() {
    let mut world = World::new("deathmatch");
    world.add_player(1, "A".into());
    world.players.get_mut(&1).unwrap().cooldown = 0;
    step(&mut world, 1, fire_at(1000.0, 430.0));
    let pos = world.players[&1].pos;
    assert!(world.events.iter().any(|event| matches!(
        event,
        Event::MuzzleFlash { player: 1, pos: flash } if (flash.x - pos.x).abs() > 1.0
    )));
}

#[test]
fn pickup_radius_matches_the_shared_constant() {
    assert_eq!(PICKUP_RADIUS, 28.0);
}
