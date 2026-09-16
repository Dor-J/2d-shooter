#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
mod collision;
mod fixtures;
mod map;
mod weapons;
pub use fixtures::{replay_fixture_json, Fixture, FrameInput, SimRng, WorldDigest};
pub use map::{MapValidationError, ValidatedMap};
pub use collision::{
    resolve_material_velocity, Aabb, BodyPart, BodyRegion, BodyShape, CollisionMask,
    CollisionPolygon, CollisionWorld, Contact, ContactManifold, DynamicBody, DynamicBodyKind,
    MaterialResponse, PolygonKind, RayHit, ShapeHit, SweepHit,
};
pub use weapons::{Weapon, WeaponStyle, WEAPONS};

pub const TICK_RATE: u32 = 60;
pub const DT: f32 = 1.0 / TICK_RATE as f32;
pub const WIDTH: f32 = 1200.0;
pub const HEIGHT: f32 = 700.0;

fn default_collision_world() -> CollisionWorld {
    let rectangles = [
        (0.0, 650.0, 1200.0, 50.0),
        (120.0, 490.0, 280.0, 20.0),
        (800.0, 490.0, 280.0, 20.0),
        (480.0, 365.0, 240.0, 20.0),
        (510.0, 555.0, 180.0, 20.0),
    ];
    let polygons = rectangles
        .into_iter()
        .flat_map(|(x, y, width, height)| {
            let top_left = Vec2 { x, y };
            let top_right = Vec2 { x: x + width, y };
            let bottom_left = Vec2 { x, y: y + height };
            let bottom_right = Vec2 {
                x: x + width,
                y: y + height,
            };
            [
                CollisionPolygon::triangle(
                    [top_left, top_right, bottom_right],
                    PolygonKind::Normal,
                ),
                CollisionPolygon::triangle(
                    [top_left, bottom_right, bottom_left],
                    PolygonKind::Normal,
                ),
            ]
        })
        .collect();
    CollisionWorld::new(polygons)
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn scale(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Input {
    pub seq: u32,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub jet: bool,
    pub fire: bool,
    pub throw_grenade: bool,
    pub aim: Vec2,
    pub weapon: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub pos: Vec2,
    pub vel: Vec2,
    pub hp: i32,
    pub fuel: f32,
    pub kills: u32,
    pub deaths: u32,
    pub team: u8,
    pub grounded: bool,
    pub cooldown: u16,
    pub respawn: u16,
    pub last_seq: u32,
    pub weapon: u8,
    pub ammo: u16,
    pub reload_timer: u16,
    pub startup: u16,
    pub magazines: [u16; 14],
    pub grenades: u8,
    pub grenade_cooldown: u16,
    pub grenade_held: bool,
}
impl Player {
    pub fn new(id: u32, name: String, team: u8) -> Self {
        Self {
            id,
            name,
            pos: spawn(id, team),
            vel: Vec2::default(),
            hp: 100,
            fuel: 1.0,
            kills: 0,
            deaths: 0,
            team,
            grounded: false,
            cooldown: 0,
            respawn: 0,
            last_seq: 0,
            weapon: 0,
            ammo: WEAPONS[0].ammo,
            reload_timer: 0,
            startup: 0,
            magazines: WEAPONS.map(|weapon| weapon.ammo),
            grenades: 2,
            grenade_cooldown: 0,
            grenade_held: false,
        }
    }
}
fn spawn(id: u32, team: u8) -> Vec2 {
    let side = if team == 2 {
        880.0
    } else if team == 1 {
        260.0
    } else if id % 2 == 0 {
        880.0
    } else {
        260.0
    };
    Vec2 { x: side, y: 430.0 }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Projectile {
    pub id: u32,
    pub owner: u32,
    pub pos: Vec2,
    pub vel: Vec2,
    pub ttl: u16,
    pub damage: i32,
    pub explosive: bool,
    pub kind: ProjectileKind,
    pub splash_radius: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectileKind {
    Bullet,
    ShotgunPellet,
    M79Grenade,
    FragGrenade,
    Melee,
    LawRocket,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Shot { player: u32 },
    Hit { target: u32, hp: i32 },
    Kill { killer: u32, target: u32 },
    Respawn { player: u32 },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct World {
    pub tick: u64,
    pub mode: String,
    pub players: BTreeMap<u32, Player>,
    pub projectiles: Vec<Projectile>,
    #[serde(default)]
    pub objects: Vec<DynamicBody>,
    #[serde(default)]
    pub map_polygons: Vec<CollisionPolygon>,
    pub scores: [u32; 2],
    pub events: Vec<Event>,
    #[serde(default)]
    pub rng: SimRng,
    #[serde(default)]
    next_projectile: u32,
    #[serde(skip, default = "default_collision_world")]
    collision: CollisionWorld,
}

impl World {
    pub fn new(mode: &str) -> Self {
        Self::with_collision(mode, default_collision_world())
    }

    pub fn with_collision(mode: &str, collision: CollisionWorld) -> Self {
        let map_polygons = collision.polygons().to_vec();
        Self {
            tick: 0,
            mode: mode.into(),
            players: BTreeMap::new(),
            projectiles: Vec::new(),
            objects: Vec::new(),
            map_polygons,
            scores: [0, 0],
            events: Vec::new(),
            rng: SimRng::default(),
            next_projectile: 1,
            collision,
        }
    }

    pub fn with_map(mode: &str, map: &ValidatedMap) -> Self {
        Self::with_collision(mode, CollisionWorld::from_map(map))
    }

    pub fn digest(&self) -> WorldDigest {
        fixtures::digest(&serde_json::to_vec(self).unwrap_or_default())
    }
    pub fn add_player(&mut self, id: u32, name: String) {
        let team = if self.mode == "team" {
            if self.players.len() % 2 == 0 {
                1
            } else {
                2
            }
        } else {
            0
        };
        self.players.insert(id, Player::new(id, name, team));
    }
    pub fn step(&mut self, inputs: &BTreeMap<u32, Input>) {
        self.tick += 1;
        self.events.clear();
        for player in self.players.values_mut() {
            if player.hp <= 0 {
                if player.respawn > 0 {
                    player.respawn -= 1;
                } else {
                    player.hp = 100;
                    player.pos = spawn(player.id, player.team);
                    player.vel = Vec2::default();
                    player.fuel = 1.0;
                    player.magazines = WEAPONS.map(|weapon| weapon.ammo);
                    player.ammo = WEAPONS[player.weapon as usize].ammo;
                    player.reload_timer = 0;
                    player.startup = 0;
                    player.grenades = 2;
                    player.grenade_cooldown = 0;
                    player.grenade_held = false;
                    self.events.push(Event::Respawn { player: player.id });
                }
                continue;
            }
            let input = inputs.get(&player.id).copied().unwrap_or_default();
            player.last_seq = input.seq;
            let selected = input.weapon.min((WEAPONS.len() - 1) as u8);
            if selected != player.weapon {
                player.magazines[player.weapon as usize] = player.ammo;
                player.weapon = selected;
                player.ammo = player.magazines[selected as usize];
                player.startup = 0;
                player.reload_timer = if player.ammo == 0 {
                    WEAPONS[selected as usize].reload_ticks
                } else {
                    0
                };
            }
            let weapon = WEAPONS[player.weapon as usize];
            if player.cooldown > 0 {
                player.cooldown -= 1;
            }
            if player.grenade_cooldown > 0 {
                player.grenade_cooldown -= 1;
            }
            if player.reload_timer > 0 {
                player.reload_timer -= 1;
                if player.reload_timer == 0 {
                    player.ammo = weapon.ammo;
                    player.magazines[player.weapon as usize] = player.ammo;
                }
            }
            let horizontal = (input.right as i32 - input.left as i32) as f32;
            player.vel.x = (player.vel.x + horizontal * 1050.0 * DT).clamp(-270.0, 270.0);
            if horizontal == 0.0 {
                player.vel.x *= 0.82;
            }
            if input.jump && player.grounded {
                player.vel.y = -390.0;
                player.grounded = false;
            }
            if input.jet && player.fuel > 0.0 {
                player.vel.y = (player.vel.y - 950.0 * DT).max(-300.0);
                player.fuel = (player.fuel - 0.45 * DT).max(0.0);
            } else if player.grounded {
                player.fuel = (player.fuel + 0.5 * DT).min(1.0);
            }
            player.vel.y = (player.vel.y + 950.0 * DT).min(600.0);
            let start = player.pos;
            let desired = Vec2 {
                x: (start.x + player.vel.x * DT).clamp(16.0, WIDTH - 16.0),
                y: (start.y + player.vel.y * DT).clamp(16.0, HEIGHT - 16.0),
            };
            player.grounded = false;
            if let Some(shape_hit) = self
                .collision
                .sweep_shape(start, desired, &BodyShape::standing(), CollisionMask::PLAYER)
            {
                let hit = shape_hit.hit;
                player.pos = hit.position;
                let response = resolve_material_velocity(player.vel, hit.normal, hit.kind);
                player.vel = response.velocity;
                player.grounded = hit.normal.y < -0.5;
                if response.deadly {
                    player.hp = 0;
                    player.respawn = 120;
                }
            } else {
                player.pos = desired;
            }
            if input.fire {
                player.startup = player.startup.saturating_add(1);
            } else {
                player.startup = 0;
            }
            if input.fire
                && player.cooldown == 0
                && player.reload_timer == 0
                && player.ammo > 0
                && player.startup > weapon.startup_ticks
            {
                let dx = input.aim.x - player.pos.x;
                let dy = input.aim.y - player.pos.y;
                let len = (dx * dx + dy * dy).sqrt();
                if len > 1.0 && len.is_finite() {
                    let speed = weapon.speed * 40.0;
                    let damage = match weapon.style {
                        WeaponStyle::Explosive => 100,
                        WeaponStyle::Shotgun => {
                            ((weapon.speed * weapon.hit_multiply) / 3.0).round() as i32
                        }
                        WeaponStyle::Bullet => (weapon.speed * weapon.hit_multiply)
                            .round()
                            .clamp(1.0, 100.0)
                            as i32,
                        WeaponStyle::Melee => 100,
                    };
                    for pellet in 0..weapon.pellets {
                        let spread = if weapon.style == WeaponStyle::Shotgun {
                            (pellet as f32 - 2.5) * 0.055
                        } else {
                            0.0
                        };
                        let angle = dy.atan2(dx) + spread;
                        self.projectiles.push(Projectile {
                            id: self.next_projectile,
                            owner: player.id,
                            pos: player.pos,
                            vel: Vec2 {
                                x: angle.cos() * speed,
                                y: angle.sin() * speed,
                            },
                            ttl: if weapon.style == WeaponStyle::Melee {
                                3
                            } else {
                                90
                            },
                            damage,
                            explosive: weapon.style == WeaponStyle::Explosive,
                            kind: match (player.weapon, weapon.style) {
                                (6, _) => ProjectileKind::M79Grenade,
                                (13, _) => ProjectileKind::LawRocket,
                                (_, WeaponStyle::Shotgun) => ProjectileKind::ShotgunPellet,
                                (_, WeaponStyle::Melee) => ProjectileKind::Melee,
                                _ => ProjectileKind::Bullet,
                            },
                            splash_radius: if player.weapon == 6 {
                                64.0
                            } else if player.weapon == 13 {
                                85.0
                            } else {
                                0.0
                            },
                        });
                        self.next_projectile = self.next_projectile.wrapping_add(1);
                    }
                    player.ammo -= 1;
                    player.magazines[player.weapon as usize] = player.ammo;
                    player.cooldown = weapon.fire_interval;
                    if player.ammo == 0 {
                        player.reload_timer = weapon.reload_ticks;
                    }
                    self.events.push(Event::Shot { player: player.id });
                }
            }
            if input.throw_grenade
                && !player.grenade_held
                && player.grenades > 0
                && player.grenade_cooldown == 0
            {
                let dx = input.aim.x - player.pos.x;
                let dy = input.aim.y - player.pos.y;
                let len = (dx * dx + dy * dy).sqrt();
                if len > 1.0 && len.is_finite() {
                    self.projectiles.push(Projectile {
                        id: self.next_projectile,
                        owner: player.id,
                        pos: player.pos,
                        vel: Vec2 {
                            x: dx / len * 200.0 + player.vel.x,
                            y: dy / len * 200.0 + player.vel.y,
                        },
                        ttl: 150,
                        damage: 100,
                        explosive: true,
                        kind: ProjectileKind::FragGrenade,
                        splash_radius: 85.0,
                    });
                    self.next_projectile = self.next_projectile.wrapping_add(1);
                    player.grenades -= 1;
                    player.grenade_cooldown = 80;
                }
            }
            player.grenade_held = input.throw_grenade;
        }
        for object in &mut self.objects {
            object.step(&self.collision, DT);
        }
        let teams: BTreeMap<u32, u8> = self.players.iter().map(|(&id, p)| (id, p.team)).collect();
        let mut hits = Vec::new();
        self.projectiles.retain_mut(|bullet| {
            let previous_pos = bullet.pos;
            if matches!(
                bullet.kind,
                ProjectileKind::M79Grenade | ProjectileKind::FragGrenade
            ) {
                bullet.vel.y += 0.12 * 950.0 * DT;
            }
            bullet.pos = bullet.pos.add(bullet.vel.scale(DT));
            bullet.ttl = bullet.ttl.saturating_sub(1);
            if bullet.ttl == 0
                || bullet.pos.x < 0.0
                || bullet.pos.x > WIDTH
                || bullet.pos.y < 0.0
                || bullet.pos.y > HEIGHT
            {
                if bullet.explosive {
                    splash_hits(bullet, &self.players, &teams, &mut hits);
                }
                return false;
            }
            let Some(owner_team) = teams.get(&bullet.owner) else {
                return false;
            };
            let platform_hit = self
                .collision
                .raycast(previous_pos, bullet.pos, CollisionMask::BULLET)
                .map(|hit| hit.time);
            let mut player_hit: Option<(f32, u32)> = None;
            for target in self.players.values() {
                if target.id != bullet.owner
                    && target.hp > 0
                    && (*owner_team == 0 || *owner_team != target.team)
                {
                    if let Some(t) = segment_circle_t(previous_pos, bullet.pos, target.pos, 18.0) {
                        if player_hit.is_none_or(|(best, _)| t < best) {
                            player_hit = Some((t, target.id));
                        }
                    }
                }
            }
            let player_t = player_hit.map(|(t, _)| t);
            let impact_t = match (platform_hit, player_t) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };
            if let Some(t) = impact_t {
                bullet.pos = Vec2 {
                    x: previous_pos.x + (bullet.pos.x - previous_pos.x) * t,
                    y: previous_pos.y + (bullet.pos.y - previous_pos.y) * t,
                };
                if bullet.explosive {
                    splash_hits(bullet, &self.players, &teams, &mut hits);
                } else if player_t
                    .is_some_and(|hit_t| hit_t <= platform_hit.unwrap_or(f32::INFINITY))
                {
                    hits.push((bullet.owner, player_hit.unwrap().1, bullet.damage));
                }
                return false;
            }
            true
        });
        for (killer, target_id, damage) in hits {
            if let Some(target) = self.players.get_mut(&target_id) {
                if target.hp <= 0 {
                    continue;
                }
                target.hp = (target.hp - damage).max(0);
                self.events.push(Event::Hit {
                    target: target_id,
                    hp: target.hp,
                });
                if target.hp == 0 {
                    self.objects.push(DynamicBody::new(
                        DynamicBodyKind::Corpse,
                        target.pos,
                        10.0,
                    ));
                    target.deaths += 1;
                    target.respawn = 120;
                    if self.mode == "team" && killer != target_id {
                        if let Some(team) = teams.get(&killer) {
                            self.scores[(*team - 1) as usize] += 1;
                        }
                    }
                    self.events.push(Event::Kill {
                        killer,
                        target: target_id,
                    });
                    if killer != target_id {
                        if let Some(k) = self.players.get_mut(&killer) {
                            k.kills += 1;
                        }
                    }
                }
            }
        }
    }
}

fn segment_point_distance_squared(start: Vec2, end: Vec2, point: Vec2) -> f32 {
    let segment = Vec2 {
        x: end.x - start.x,
        y: end.y - start.y,
    };
    let length_squared = segment.x * segment.x + segment.y * segment.y;
    if length_squared == 0.0 {
        return (point.x - start.x).powi(2) + (point.y - start.y).powi(2);
    }
    let projection = (((point.x - start.x) * segment.x + (point.y - start.y) * segment.y)
        / length_squared)
        .clamp(0.0, 1.0);
    let closest = Vec2 {
        x: start.x + segment.x * projection,
        y: start.y + segment.y * projection,
    };
    (point.x - closest.x).powi(2) + (point.y - closest.y).powi(2)
}

fn segment_circle_t(start: Vec2, end: Vec2, center: Vec2, radius: f32) -> Option<f32> {
    let segment = Vec2 {
        x: end.x - start.x,
        y: end.y - start.y,
    };
    let length_squared = segment.x * segment.x + segment.y * segment.y;
    if length_squared == 0.0 {
        return (segment_point_distance_squared(start, end, center) <= radius * radius)
            .then_some(0.0);
    }
    let t = (((center.x - start.x) * segment.x + (center.y - start.y) * segment.y)
        / length_squared)
        .clamp(0.0, 1.0);
    (segment_point_distance_squared(start, end, center) <= radius * radius).then_some(t)
}

fn splash_hits(
    bullet: &Projectile,
    players: &BTreeMap<u32, Player>,
    teams: &BTreeMap<u32, u8>,
    hits: &mut Vec<(u32, u32, i32)>,
) {
    for target in players.values() {
        if target.hp <= 0 {
            continue;
        }
        if target.id != bullet.owner
            && teams.get(&bullet.owner) == Some(&target.team)
            && target.team != 0
        {
            continue;
        }
        let dx = target.pos.x - bullet.pos.x;
        let dy = target.pos.y - bullet.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        if distance < bullet.splash_radius {
            hits.push((
                bullet.owner,
                target.id,
                ((1.0 - distance / bullet.splash_radius) * bullet.damage as f32).ceil() as i32,
            ));
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn predict_player(player_json: &str, input_json: &str) -> String {
        let Ok(player) = serde_json::from_str::<Player>(player_json) else {
            return String::new();
        };
        let Ok(input) = serde_json::from_str::<Input>(input_json) else {
            return String::new();
        };
        let mut world = World::new("deathmatch");
        let id = player.id;
        world.players.insert(id, player);
        world.step(&BTreeMap::from([(id, input)]));
        serde_json::to_string(&world.players[&id]).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn replay_fixture(fixture_json: &str) -> String {
        replay_fixture_json(fixture_json).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn soldat_primary_weapon_stats_match_normal_mode() {
        assert_eq!(WEAPONS[..10].len(), 10);
        assert_eq!(WEAPONS[0].name, "Desert Eagles");
        assert_eq!(
            (
                WEAPONS[0].fire_interval,
                WEAPONS[0].ammo,
                WEAPONS[0].reload_ticks
            ),
            (24, 7, 87)
        );
        assert_eq!(
            (
                WEAPONS[1].fire_interval,
                WEAPONS[1].ammo,
                WEAPONS[1].reload_ticks
            ),
            (6, 30, 105)
        );
        assert_eq!(
            (WEAPONS[4].style, WEAPONS[4].pellets),
            (WeaponStyle::Shotgun, 6)
        );
        assert_eq!(
            (WEAPONS[6].style, WEAPONS[6].ammo),
            (WeaponStyle::Explosive, 1)
        );
        assert_eq!(
            (WEAPONS[7].startup_ticks, WEAPONS[7].fire_interval),
            (19, 225)
        );
    }
    #[test]
    fn soldat_secondary_weapon_stats_match_normal_mode() {
        assert_eq!(WEAPONS.len(), 14);
        assert_eq!(
            (WEAPONS[10].name, WEAPONS[10].ammo, WEAPONS[10].reload_ticks),
            ("USSOCOM", 14, 60)
        );
        assert_eq!(
            (WEAPONS[11].name, WEAPONS[11].style),
            ("Combat Knife", WeaponStyle::Melee)
        );
        assert_eq!(
            (WEAPONS[12].name, WEAPONS[12].fire_interval),
            ("Chainsaw", 2)
        );
        assert_eq!(
            (WEAPONS[13].name, WEAPONS[13].startup_ticks),
            ("M72 LAW", 13)
        );
    }
    #[test]
    fn frag_grenade_consumes_ammo_and_uses_source_radius() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Thrower".into());
        let grenades = w.players[&1].grenades;
        w.step(&BTreeMap::from([(
            1,
            Input {
                throw_grenade: true,
                aim: Vec2 { x: 500.0, y: 430.0 },
                ..Default::default()
            },
        )]));
        assert_eq!(w.players[&1].grenades, grenades - 1);
        let grenade = w
            .projectiles
            .iter()
            .find(|p| p.kind == ProjectileKind::FragGrenade)
            .unwrap();
        assert_eq!(grenade.splash_radius, 85.0);
    }
    #[test]
    fn fast_projectile_hits_player_crossed_between_ticks() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Shooter".into());
        w.add_player(2, "Target".into());
        w.players.get_mut(&2).unwrap().pos = Vec2 { x: 300.0, y: 430.0 };
        w.projectiles.push(Projectile {
            id: 1,
            owner: 1,
            pos: Vec2 { x: 270.0, y: 430.0 },
            vel: Vec2 { x: 3600.0, y: 0.0 },
            ttl: 10,
            damage: 20,
            explosive: false,
            kind: ProjectileKind::Bullet,
            splash_radius: 0.0,
        });
        w.step(&BTreeMap::new());
        assert_eq!(w.players[&2].hp, 80);
    }
    #[test]
    fn platform_blocks_fast_projectile_before_player() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Shooter".into());
        w.add_player(2, "Target".into());
        w.players.get_mut(&2).unwrap().pos = Vec2 { x: 350.0, y: 500.0 };
        w.projectiles.push(Projectile {
            id: 1,
            owner: 1,
            pos: Vec2 { x: 350.0, y: 460.0 },
            vel: Vec2 { x: 0.0, y: 3600.0 },
            ttl: 10,
            damage: 20,
            explosive: false,
            kind: ProjectileKind::Bullet,
            splash_radius: 0.0,
        });
        w.step(&BTreeMap::new());
        assert_eq!(w.players[&2].hp, 100);
        assert!(w.projectiles.is_empty());
    }
    #[test]
    fn holding_grenade_input_throws_only_once() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Thrower".into());
        let input = BTreeMap::from([(
            1,
            Input {
                throw_grenade: true,
                aim: Vec2 { x: 500.0, y: 430.0 },
                ..Default::default()
            },
        )]);
        for _ in 0..100 {
            w.step(&input);
        }
        assert_eq!(w.players[&1].grenades, 1);
    }
    #[test]
    fn melee_projectile_has_short_range() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Knife".into());
        w.step(&BTreeMap::from([(
            1,
            Input {
                fire: true,
                aim: Vec2 {
                    x: 1000.0,
                    y: 430.0,
                },
                weapon: 11,
                ..Default::default()
            },
        )]));
        let melee = w
            .projectiles
            .iter()
            .find(|p| p.kind == ProjectileKind::Melee)
            .unwrap();
        assert!(melee.ttl <= 3);
    }
    #[test]
    fn magazine_runs_empty_and_reloads_after_source_duration() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        let fire = Input {
            fire: true,
            aim: Vec2 {
                x: 1000.0,
                y: 430.0,
            },
            weapon: 0,
            ..Default::default()
        };
        for _ in 0..7 {
            w.players.get_mut(&1).unwrap().cooldown = 0;
            w.step(&BTreeMap::from([(1, fire)]));
        }
        assert_eq!(w.players[&1].ammo, 0);
        let before = w.projectiles.len();
        w.step(&BTreeMap::from([(1, fire)]));
        assert_eq!(w.projectiles.len(), before);
        for _ in 0..WEAPONS[0].reload_ticks {
            w.step(&BTreeMap::new());
        }
        assert_eq!(w.players[&1].ammo, WEAPONS[0].ammo);
    }
    #[test]
    fn spas_fires_multiple_pellets_for_one_shell() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        w.step(&BTreeMap::from([(
            1,
            Input {
                fire: true,
                aim: Vec2 {
                    x: 1000.0,
                    y: 430.0,
                },
                weapon: 4,
                ..Default::default()
            },
        )]));
        assert_eq!(w.projectiles.len(), WEAPONS[4].pellets as usize);
        assert_eq!(w.players[&1].ammo, WEAPONS[4].ammo - 1);
    }
    #[test]
    fn m79_explosion_damages_nearby_enemy_not_distant_enemy() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Shooter".into());
        w.add_player(2, "Near".into());
        w.add_player(3, "Far".into());
        w.players.get_mut(&2).unwrap().pos = Vec2 { x: 335.0, y: 430.0 };
        w.players.get_mut(&3).unwrap().pos = Vec2 { x: 500.0, y: 430.0 };
        w.projectiles.push(Projectile {
            id: 1,
            owner: 1,
            pos: Vec2 { x: 300.0, y: 430.0 },
            vel: Vec2::default(),
            ttl: 1,
            damage: 100,
            explosive: true,
            kind: ProjectileKind::M79Grenade,
            splash_radius: 64.0,
        });
        w.step(&BTreeMap::new());
        assert!(w.players[&2].hp < 100);
        assert_eq!(w.players[&3].hp, 100);
    }
    #[test]
    fn switching_away_and_back_does_not_refill_a_magazine() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        w.players.get_mut(&1).unwrap().ammo = 1;
        let fire = Input {
            fire: true,
            aim: Vec2 {
                x: 1000.0,
                y: 430.0,
            },
            weapon: 0,
            ..Default::default()
        };
        w.step(&BTreeMap::from([(1, fire)]));
        w.step(&BTreeMap::from([(
            1,
            Input {
                weapon: 1,
                ..Default::default()
            },
        )]));
        w.step(&BTreeMap::from([(
            1,
            Input {
                weapon: 0,
                ..Default::default()
            },
        )]));
        assert_eq!(w.players[&1].ammo, 0);
        assert!(w.players[&1].reload_timer > 0);
    }
    #[test]
    fn self_explosion_is_not_credited_as_a_kill() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        w.projectiles.push(Projectile {
            id: 1,
            owner: 1,
            pos: w.players[&1].pos,
            vel: Vec2::default(),
            ttl: 1,
            damage: 100,
            explosive: true,
            kind: ProjectileKind::M79Grenade,
            splash_radius: 64.0,
        });
        w.step(&BTreeMap::new());
        assert_eq!(w.players[&1].hp, 0);
        assert_eq!(w.players[&1].kills, 0);
        assert_eq!(w.objects.len(), 1);
        assert_eq!(w.objects[0].kind, DynamicBodyKind::Corpse);
        for _ in 0..120 {
            w.step(&BTreeMap::new());
        }
        assert!(w.objects[0].grounded);
    }
    #[test]
    fn deterministic_replay() {
        let mut a = World::new("deathmatch");
        a.add_player(1, "A".into());
        let mut b = a.clone();
        let input = BTreeMap::from([(
            1,
            Input {
                right: true,
                jet: true,
                ..Default::default()
            },
        )]);
        for _ in 0..120 {
            a.step(&input);
            b.step(&input);
        }
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
    }
    #[test]
    fn player_lands_on_platform() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        for _ in 0..120 {
            w.step(&BTreeMap::new());
        }
        assert!(w.players[&1].grounded);
        assert!((w.players[&1].pos.y - 474.0).abs() < 1.0);
    }
    #[test]
    fn fire_is_rate_limited() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        let input = BTreeMap::from([(
            1,
            Input {
                fire: true,
                aim: Vec2 {
                    x: 1000.0,
                    y: 430.0,
                },
                ..Default::default()
            },
        )]);
        w.step(&input);
        let first = w.projectiles.len();
        w.step(&input);
        assert_eq!(first, 1);
        assert_eq!(w.projectiles.len(), 1);
    }
}
