use std::collections::BTreeMap;

use super::movement::integrate_body;
use super::respawn::spawn_holds_parachute;
use crate::weapons::{sync_hands, WeaponTable, SELECTABLE_WEAPONS};
use crate::ModeRules;
use crate::{
    advance_character, resolve_material_velocity, select_spawn, BodyRegion, CharacterState,
    CollisionMask, CollisionWorld, DamageCause, DamageConfig, DamageEvent, Event, Input, Inventory,
    MapSpawn, MovementConfig, Player, RespawnConfig, Vec2, HEIGHT, WIDTH,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn corpse(
    player: &mut Player,
    inputs: &BTreeMap<u32, Input>,
    rules_snapshot: &ModeRules,
    respawn_config: &RespawnConfig,
    opponents: &BTreeMap<u32, Vec<Vec2>>,
    map_spawns: &[MapSpawn],
    collision: &CollisionWorld,
    mode: &str,
    weapons: &WeaponTable,
    events: &mut Vec<Event>,
    fuel_capacity: i16,
) {
    // A dead player still picks the weapon they will carry back in.
    if let Some(input) = inputs.get(&player.id) {
        player.last_seq = input.seq;
        player.weapon = input.weapon.min((SELECTABLE_WEAPONS.len() - 1) as u8);
    }
    if !crate::modes::modifiers::survival::may_respawn(rules_snapshot.modifiers.survival) {
        // In Survival there is nothing to count down to: they are out for the round.
        return;
    }
    if player.respawn > 0 {
        player.respawn -= 1;
    } else {
        let enemies = opponents.get(&player.id).cloned().unwrap_or_default();
        player.hp = 100;
        player.armor = 0;
        player.bleed = None;
        player.attackers.clear();
        player.last_damage_direction = Vec2::default();
        player.spawn_protection = respawn_config.protection_ticks;
        player.pos = select_spawn(map_spawns, mode, player.team, player.id, &enemies);
        player.parachute = spawn_holds_parachute(collision, player.pos);
        player.vel = Vec2::default();
        player.forces = Vec2::default();
        player.fuel = fuel_capacity;
        let unlocked = player.unlocked;
        player.inventory = Inventory::spawn_limited(player.weapon, weapons, |kind| {
            !rules_snapshot.modifiers.advance || unlocked.has(kind)
        });
        sync_hands(player);
        player.reload_timer = 0;
        player.startup = 0;
        player.switch_timer = 0;
        player.throw_charge = 0;
        player.pending_switch = None;
        player.grenades = 2;
        player.grenade_cooldown = 0;
        player.grenade_held = false;
        player.state = CharacterState::Airborne;
        player.previous_input = Input::default();
        player.jump_buffer_ticks = 0;
        player.airborne_ticks = 0;
        player.last_impact = 0.0;
        player.last_impulse = None;
        events.push(Event::Respawn { player: player.id });
    }
}

pub(crate) fn bleed(player: &mut Player, pending: &mut Vec<DamageEvent>) {
    if let Some(bleed) = player.bleed.as_mut() {
        if let Some(amount) = bleed.tick() {
            pending.push(DamageEvent {
                attacker: bleed.attacker,
                target: player.id,
                amount,
                region: BodyRegion::Chest,
                cause: DamageCause::Bleeding,
                direction: Vec2::default(),
                pre_scaled: false,
            });
        }
        if bleed.finished() {
            player.bleed = None;
        }
    }
}

pub(crate) fn movement(
    player: &mut Player,
    input: Input,
    collision: &CollisionWorld,
    movement: &MovementConfig,
    damage_config: &DamageConfig,
    pending: &mut Vec<DamageEvent>,
    tick: u64,
) {
    let has_standing_clearance = !player.state.is_low()
        || collision.has_standing_clearance(player.pos, CollisionMask::PLAYER);
    let start = player.pos;
    integrate_body(player);
    let impact_velocity = player.vel.y.max(0.0);
    let desired = Vec2 {
        // Acceptance evidence: rust:map:bounds
        x: player.pos.x.clamp(16.0, WIDTH - 16.0),
        y: player.pos.y.clamp(16.0, HEIGHT - 16.0),
    };
    let was_grounded = player.grounded;
    player.grounded = false;
    let contact = collision.sweep_shape(
        start,
        desired,
        &crate::character::body::shape_for(player.state),
        CollisionMask::PLAYER,
    );
    let collided = contact.is_some();
    if let Some(shape_hit) = contact {
        let hit = shape_hit.hit;
        player.pos = hit.position;
        let response = resolve_material_velocity(player.vel, hit.normal, hit.kind);
        player.vel = response.velocity;
        player.grounded = hit.normal.y < -0.5;
        if player.grounded && matches!(player.state, CharacterState::Airborne) {
            player.state = CharacterState::Standing;
        }
        if response.deadly {
            pending.push(DamageEvent {
                attacker: None,
                target: player.id,
                amount: player.hp + player.armor + 1,
                region: BodyRegion::Chest,
                cause: DamageCause::Deadly,
                direction: Vec2 { x: 0.0, y: -1.0 },
                pre_scaled: false,
            });
        }
    } else {
        player.pos = desired;
    }
    // How hard this descent is going. In free flight it simply follows the current
    // downward speed, so a jet burn genuinely softens the landing; once something is being
    // hit it keeps the fastest reading, because terrain usually bleeds off a fast fall over
    // two ticks and the body settles at a speed that no longer describes the fall.
    if was_grounded && !player.grounded {
        player.last_impact = 0.0;
    } else if !was_grounded {
        player.last_impact = if collided {
            player.last_impact.max(impact_velocity)
        } else {
            impact_velocity
        };
    }
    if !was_grounded && player.grounded && player.last_impact > damage_config.fall_damage_speed {
        let over = player.last_impact - damage_config.fall_damage_speed;
        let amount = (over * damage_config.fall_damage_per_speed)
            .round()
            .max(1.0) as i32;
        pending.push(DamageEvent {
            attacker: None,
            target: player.id,
            amount,
            region: BodyRegion::Legs,
            cause: DamageCause::Fall,
            direction: Vec2 { x: 0.0, y: 1.0 },
            pre_scaled: false,
        });
    }
    advance_character(player, input, movement, has_standing_clearance, tick);
}
