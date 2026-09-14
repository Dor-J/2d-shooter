use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
                    self.events.push(Event::Respawn { player: player.id });
                }
                continue;
            }
            let input = inputs.get(&player.id).copied().unwrap_or_default();
            player.last_seq = input.seq;
            player.weapon = input.weapon.min(2);
            if player.cooldown > 0 {
                player.cooldown -= 1;
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
            if input.fire && player.cooldown == 0 {
                let dx = input.aim.x - player.pos.x;
                let dy = input.aim.y - player.pos.y;
                let len = (dx * dx + dy * dy).sqrt();
                if len > 1.0 && len.is_finite() {
                    let (speed, damage, cooldown) = match player.weapon {
                        1 => (850.0, 34, 18),
                        2 => (1100.0, 12, 4),
                        _ => (780.0, 22, 9),
                    };
                    self.projectiles.push(Projectile {
                        id: self.next_projectile,
                        owner: player.id,
                        pos: player.pos,
                        vel: Vec2 {
                            x: dx / len * speed,
                            y: dy / len * speed,
                        },
                        ttl: 90,
                        damage,
                    });
                    self.next_projectile = self.next_projectile.wrapping_add(1);
                    player.cooldown = cooldown;
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
                return false;
            }
            if PLATFORMS.iter().any(|p| {
                bullet.pos.x >= p.x
                    && bullet.pos.x <= p.x + p.w
                    && bullet.pos.y >= p.y
                    && bullet.pos.y <= p.y + p.h
            }) {
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
                    hits.push((bullet.owner, target.id, bullet.damage));
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
                    if self.mode == "team" {
                        if let Some(team) = teams.get(&killer) {
                            self.scores[(*team - 1) as usize] += 1;
                        }
                    }
                    self.events.push(Event::Kill {
                        killer,
                        target: target_id,
                    });
                    if let Some(k) = self.players.get_mut(&killer) {
                        k.kills += 1;
                    }
                }
            }
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
