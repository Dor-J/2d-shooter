use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
mod weapons;
pub use weapons::{Weapon, WeaponStyle, WEAPONS};

pub const TICK_RATE: u32 = 60;
pub const DT: f32 = 1.0 / TICK_RATE as f32;
pub const WIDTH: f32 = 1200.0;
pub const HEIGHT: f32 = 700.0;

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
pub const PLATFORMS: [Platform; 5] = [
    Platform {
        x: 0.0,
        y: 650.0,
        w: 1200.0,
        h: 50.0,
    },
    Platform {
        x: 120.0,
        y: 490.0,
        w: 280.0,
        h: 20.0,
    },
    Platform {
        x: 800.0,
        y: 490.0,
        w: 280.0,
        h: 20.0,
    },
    Platform {
        x: 480.0,
        y: 365.0,
        w: 240.0,
        h: 20.0,
    },
    Platform {
        x: 510.0,
        y: 555.0,
        w: 180.0,
        h: 20.0,
    },
];

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Input {
    pub seq: u32,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub jet: bool,
    pub fire: bool,
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
    pub magazines: [u16; 10],
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
    pub scores: [u32; 2],
    pub events: Vec<Event>,
    next_projectile: u32,
}

impl World {
    pub fn new(mode: &str) -> Self {
        Self {
            tick: 0,
            mode: mode.into(),
            players: BTreeMap::new(),
            projectiles: Vec::new(),
            scores: [0, 0],
            events: Vec::new(),
            next_projectile: 1,
        }
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
            player.pos.x = (player.pos.x + player.vel.x * DT).clamp(14.0, WIDTH - 14.0);
            let old_bottom = player.pos.y + 16.0;
            player.pos.y = (player.pos.y + player.vel.y * DT).clamp(16.0, HEIGHT - 16.0);
            player.grounded = false;
            for platform in PLATFORMS {
                if player.vel.y >= 0.0
                    && old_bottom <= platform.y + 5.0
                    && player.pos.y + 16.0 >= platform.y
                    && player.pos.x + 12.0 > platform.x
                    && player.pos.x - 12.0 < platform.x + platform.w
                {
                    player.pos.y = platform.y - 16.0;
                    player.vel.y = 0.0;
                    player.grounded = true;
                }
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
                            ttl: 90,
                            damage,
                            explosive: weapon.style == WeaponStyle::Explosive,
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
        }
        let teams: BTreeMap<u32, u8> = self.players.iter().map(|(&id, p)| (id, p.team)).collect();
        let mut hits = Vec::new();
        self.projectiles.retain_mut(|bullet| {
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
            if PLATFORMS.iter().any(|p| {
                bullet.pos.x >= p.x
                    && bullet.pos.x <= p.x + p.w
                    && bullet.pos.y >= p.y
                    && bullet.pos.y <= p.y + p.h
            }) {
                if bullet.explosive {
                    splash_hits(bullet, &self.players, &teams, &mut hits);
                }
                return false;
            }
            let Some(owner_team) = teams.get(&bullet.owner) else {
                return false;
            };
            for target in self.players.values() {
                if target.id != bullet.owner
                    && target.hp > 0
                    && (*owner_team == 0 || *owner_team != target.team)
                    && (target.pos.x - bullet.pos.x).powi(2) + (target.pos.y - bullet.pos.y).powi(2)
                        < 18.0_f32.powi(2)
                {
                    if bullet.explosive {
                        splash_hits(bullet, &self.players, &teams, &mut hits);
                    } else {
                        hits.push((bullet.owner, target.id, bullet.damage));
                    }
                    return false;
                }
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
        if distance < 64.0 {
            hits.push((
                bullet.owner,
                target.id,
                ((1.0 - distance / 64.0) * bullet.damage as f32).ceil() as i32,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn soldat_primary_weapon_stats_match_normal_mode() {
        assert_eq!(WEAPONS.len(), 10);
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
        });
        w.step(&BTreeMap::new());
        assert_eq!(w.players[&1].hp, 0);
        assert_eq!(w.players[&1].kills, 0);
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
