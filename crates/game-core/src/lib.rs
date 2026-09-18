#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub mod bots;
mod character;
pub mod chat;
mod collision;
pub mod commands;
mod fixtures;
mod map;
pub mod modes;
pub mod objects;
pub mod replay;
mod weapons;
pub use bots::{
    chat_line, think as bot_think, Awareness, Bot, BotChatEvent, BotGoal, BotProfile, BotView,
    Contact as BotContact, Difficulty, Movement, Navigation, Objective as BotObjective,
    ProfileError, Trigger, Waypoint,
};
pub use character::{
    absorb, advance_character, apply_impulse, assists, fallback_spawn, multi_kill_label,
    record_attribution, region_multiplier, resolve_player_contact, select_spawn, Absorbed,
    Attribution, Bleed, CharacterState, DamageCause, DamageConfig, DamageEvent, DeathCause,
    Direction, Emote, Impulse, ImpulseSource, ImpulseTarget, KillFeedEntry, MovementConfig,
    MultiKill, Ragdoll, RagdollSegment, RespawnConfig, RollSource, MAX_ATTRIBUTIONS,
};
pub use chat::{censor, may_see as may_see_chat, parse_chat, ChatLine, ChatScope, Flood};
pub use collision::{
    resolve_material_velocity, Aabb, BodyPart, BodyRegion, BodyShape, CollisionMask,
    CollisionPolygon, CollisionWorld, Contact, ContactManifold, DynamicBody, DynamicBodyKind,
    MaterialResponse, PolygonKind, RayHit, ShapeHit, SweepHit,
};
pub use commands::{parse_player_command, CommandError, PlayerCommand};
pub use fixtures::{replay_fixture_json, Fixture, FrameInput, SimRng, WorldDigest};
pub use map::{MapValidationError, ValidatedMap};
pub use modes::{
    assign_team, has_line_of_sight, infiltration_award, leader, limit_reached, policy_for,
    scoreboard, survival_standing, validate_spawns, visibility_between, AdvanceConfig, Award,
    Bearer, FlagAction, FlagLayout, MatchEvent, MatchHistory, MatchLimits, MatchPhase, MatchState,
    MatchStats, MatchSummary, ModeKind, ModeRules, ModifierSet, ObjectivePolicy, ObjectiveRules,
    ObjectiveStat, Objectives, Outcome, PlayerScore, PlayerStats, Rotation, RoundStanding,
    ScoreEvent, ScoreLedger, ScoreboardRow, ScoringPolicy, ScriptedError, ScriptedRules,
    SpawnProblem, SpectateCommand, Spectator, SpectatorView, StatEvent, SurvivalConfig, Survivor,
    TeamChoice, TeamSizes, TimedAward, Unlocked, Viewer, Visibility, WeaponStats, ALPHA, BRAVO,
    CTF_CAPTURE_AWARD, INFILTRATION_CAPTURE_AWARD, NEUTRAL, POINTMATCH_FLAG_MULTIPLIER, SPECTATOR,
};
pub use objects::{
    predator_alpha, BonusConfig, BonusEffect, Flag, FlagEvent, FlagKind, FlagState, KitKind,
    Pickup, Pickups, TimedEffect, BERSERKER_DAMAGE_MULTIPLIER, CLUSTER_GRENADES,
    FLAG_PICKUP_RADIUS, FLAG_TIMEOUT, KIT_RADIUS, PREDATOR_ALPHA, TOUCHDOWN_RADIUS, VEST_ARMOR,
};
pub use replay::{Replay, ReplayChunk, ReplayError, ReplayHeader, MAX_CHUNKS, REPLAY_FORMAT};
pub use weapons::{
    accuracy, aim_dir, barrel_origin, bink_on_hit, boosts_the_shooter, bounce_velocity,
    calculate_bink, can_damage, charged_velocity, cluster_submunitions, collides_with_bodies,
    damage as weapon_damage, degraded_hit_multiply, direct_damage, dropped_weapon,
    explosion_damage, explosion_impulse, explosive, firing, flame, gravity_multiplier,
    grenade_is_armed, impact_response, inaccuracy, is_continuous_contact, max_deviation, melee,
    melee_reach, movement_accuracy, muzzle_origin, muzzle_velocity, nearest_index,
    per_pellet_spread, projectile as projectile_rules, propagated_hit_multiply,
    propagated_velocity, propagates, push_impulse, pushes_objects, recoil_radians, refusal,
    ricochet_velocity, self_bink_on_fire, self_boost, should_bink, stance_spread,
    startup_resets_on_release, stationary, surface_response, swing_segment, throw_arc, throw_frame,
    throw_velocity, BulletStyle, FireRefusal, ImpactResponse, Inventory, NoCollision, ShooterPose,
    StationaryGun, SurfaceResponse, WeaponConfigError, WeaponDef, WeaponKind, WeaponLimits,
    WeaponSlot, WeaponTable, ALL_WEAPONS, ARROW_RESIST_TICKS, CLUSTER_SUBMUNITIONS,
    CONFIGURED_WEAPONS, FIRST_DEGRADE_DISTANCE, GRENADE_SURFACE_COEF, MAX_INACCURACY,
    MAX_THROW_CHARGE, MELEE_RADIUS, OBJECT_PUSH_MULTIPLIER, PICKUP_RADIUS, RICOCHET_MIN_TRAVEL,
    SECOND_DEGRADE_DISTANCE, SELECTABLE_WEAPONS, SWITCH_DELAY_TICKS, THROW_LAST_FRAME,
};

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

#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
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
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Input {
    pub seq: u32,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub jet: bool,
    #[serde(default)]
    pub crouch: bool,
    #[serde(default)]
    pub prone: bool,
    #[serde(default)]
    pub roll: bool,
    #[serde(default)]
    pub emote: Option<Emote>,
    #[serde(default)]
    pub reload: bool,
    pub fire: bool,
    pub throw_grenade: bool,
    pub aim: Vec2,
    pub weapon: u8,
    #[serde(default)]
    pub drop: bool,
    #[serde(default)]
    pub throw_weapon: bool,
    #[serde(default)]
    pub throw_knife: bool,
    #[serde(default)]
    pub pickup: bool,
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
    #[serde(default)]
    pub state: CharacterState,
    #[serde(default)]
    pub facing: Direction,
    #[serde(default)]
    pub previous_input: Input,
    #[serde(default)]
    pub jump_buffer_ticks: u8,
    #[serde(default)]
    pub airborne_ticks: u16,
    #[serde(default)]
    pub last_impact: f32,
    #[serde(default)]
    pub last_impulse: Option<ImpulseSource>,
    pub cooldown: u16,
    pub respawn: u16,
    pub last_seq: u32,
    pub weapon: u8,
    pub ammo: u16,
    pub reload_timer: u16,
    pub startup: u16,
    #[serde(default)]
    pub inventory: Inventory,
    #[serde(default)]
    pub switch_timer: u16,
    #[serde(default)]
    pub throw_charge: u16,
    #[serde(default)]
    pub pending_switch: Option<u8>,
    pub grenades: u8,
    pub grenade_cooldown: u16,
    pub grenade_held: bool,
    /// Ticks the throw key has been held. A cooked grenade is thrown harder.
    #[serde(default)]
    pub grenade_charge: u16,
    /// Cluster grenades in hand, which are thrown in place of frags while any are left.
    #[serde(default)]
    pub cluster_grenades: u8,
    /// Which primaries this player has earned, which only Advance mode ever restricts.
    #[serde(default = "Unlocked::everything")]
    pub unlocked: Unlocked,
    /// The bonus effect this player is under, if any.
    #[serde(default)]
    pub bonus: TimedEffect,
    #[serde(default)]
    pub armor: i32,
    #[serde(default)]
    pub bleed: Option<Bleed>,
    #[serde(default)]
    pub attackers: Vec<Attribution>,
    #[serde(default)]
    pub spawn_protection: u16,
    #[serde(default)]
    pub assists: u32,
    #[serde(default)]
    pub teamkills: u32,
    #[serde(default)]
    pub suicides: u32,
    #[serde(default)]
    pub headshots: u32,
    #[serde(default)]
    pub multi_kill: MultiKill,
    #[serde(default)]
    pub last_death: Option<DeathCause>,
    #[serde(default)]
    pub last_damage_direction: Vec2,
    #[serde(default)]
    pub bink: u16,
    #[serde(default)]
    pub accuracy: f32,
    #[serde(default)]
    pub recoil_aim: f32,
    #[serde(default)]
    pub burst: u16,
}
impl Player {
    pub fn new(id: u32, name: String, team: u8) -> Self {
        let table = WeaponTable::normal();
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
            state: CharacterState::Airborne,
            facing: Direction::Right,
            previous_input: Input::default(),
            jump_buffer_ticks: 0,
            airborne_ticks: 0,
            last_impact: 0.0,
            last_impulse: None,
            cooldown: 0,
            respawn: 0,
            last_seq: 0,
            weapon: 0,
            ammo: u16::from(table.for_slot(0).ammo),
            reload_timer: 0,
            startup: 0,
            inventory: Inventory::spawn(0, &table),
            switch_timer: 0,
            throw_charge: 0,
            pending_switch: None,
            grenades: 2,
            grenade_cooldown: 0,
            grenade_held: false,
            grenade_charge: 0,
            cluster_grenades: 0,
            unlocked: Unlocked::everything(),
            bonus: TimedEffect::default(),
            armor: 0,
            bleed: None,
            attackers: Vec::new(),
            spawn_protection: 0,
            assists: 0,
            teamkills: 0,
            suicides: 0,
            headshots: 0,
            multi_kill: MultiKill::default(),
            last_death: None,
            last_damage_direction: Vec2::default(),
            bink: 0,
            accuracy: 0.0,
            recoil_aim: 0.0,
            burst: 0,
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

fn map_spawn(spawns: &[MapSpawn], id: u32, team: u8) -> Vec2 {
    let matching = spawns
        .iter()
        .filter(|spawn| spawn.team == team || (team == 0 && spawn.team == 0))
        .collect::<Vec<_>>();
    let candidates = if matching.is_empty() {
        spawns.iter().collect::<Vec<_>>()
    } else {
        matching
    };
    candidates
        .get(id as usize % candidates.len().max(1))
        .map_or_else(|| spawn(id, team), |selected| selected.position)
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
    #[serde(default)]
    pub weapon: Option<WeaponKind>,
    #[serde(default)]
    pub origin: Vec2,
    /// Where this projectile last struck something. A rocket only skips off a surface it reached
    /// from far enough away, so it has to remember its last impact rather than only its origin.
    #[serde(default)]
    pub last_impact: Vec2,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawnKind {
    #[default]
    Player,
    Flag,
    Bonus,
    Grenade,
    StationaryGun,
}

impl SpawnKind {
    /// Soldat PMS team numbers for special spawnpoints.
    pub const fn from_pms_team(team: i32) -> Self {
        match team {
            5 | 6 | 14 | 15 => Self::Flag,
            7 => Self::Grenade,
            8..=13 => Self::Bonus,
            16 => Self::StationaryGun,
            _ => Self::Player,
        }
    }

    const fn is_player(kind: &Self) -> bool {
        matches!(kind, Self::Player)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MapSpawn {
    pub position: Vec2,
    pub team: u8,
    #[serde(default, skip_serializing_if = "SpawnKind::is_player")]
    pub kind: SpawnKind,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectileKind {
    Bullet,
    ShotgunPellet,
    M79Grenade,
    FragGrenade,
    Melee,
    LawRocket,
    ThrownKnife,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Shot {
        player: u32,
    },
    Hit {
        target: u32,
        hp: i32,
    },
    Kill {
        killer: u32,
        target: u32,
    },
    Respawn {
        player: u32,
    },
    /// Authoritative damage, for the damage-direction indicator and hit feedback.
    Damage {
        target: u32,
        attacker: Option<u32>,
        amount: i32,
        region: BodyRegion,
        cause: DamageCause,
        direction: Vec2,
    },
    KillFeed(KillFeedEntry),
    /// Presentation only: derived from authoritative damage, never read back by the simulation.
    Blood {
        target: u32,
        position: Vec2,
        direction: Vec2,
        amount: i32,
    },
    Gibs {
        target: u32,
        position: Vec2,
        velocity: Vec2,
    },
    Reload {
        player: u32,
    },
    Empty {
        player: u32,
    },
    /// A kit appeared on the map, for the client to draw and play a sound for.
    KitSpawned {
        kind: KitKind,
    },
    /// Somebody picked a kit up.
    KitTaken {
        player: u32,
        kind: KitKind,
    },
    /// A timed effect ran out.
    BonusExpired {
        player: u32,
        effect: BonusEffect,
    },
    /// The weapon would not fire from the pose it was fired in, so the client can say why rather
    /// than leaving the player wondering whether the shot was lost to the network.
    FireRefused {
        player: u32,
        reason: FireRefusal,
    },
    MuzzleFlash {
        player: u32,
        pos: Vec2,
    },
    Casing {
        player: u32,
        pos: Vec2,
    },
    /// Something went off. Presentation only: the damage it did was already reported as `Damage`,
    /// and the client draws the fireball, the smoke, and the shake from this.
    Explosion {
        pos: Vec2,
        radius: f32,
    },
    /// A round struck terrain rather than a person, with the surface it struck. The client throws
    /// sparks along the normal; nothing reads it back.
    Impact {
        pos: Vec2,
        normal: Vec2,
    },
}

#[derive(Clone, Copy)]
struct PendingHit {
    killer: u32,
    target: u32,
    damage: i32,
    impulse: Impulse,
    region: BodyRegion,
    cause: DamageCause,
    pre_scaled: bool,
    bink_weapon: Option<WeaponKind>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub map_spawns: Vec<MapSpawn>,
    #[serde(default)]
    pub ragdolls: Vec<Ragdoll>,
    pub scores: [u32; 2],
    pub events: Vec<Event>,
    #[serde(default)]
    pub movement: MovementConfig,
    #[serde(default)]
    pub damage: DamageConfig,
    #[serde(default)]
    pub respawn: RespawnConfig,
    #[serde(default)]
    pub rng: SimRng,
    #[serde(default)]
    next_projectile: u32,
    #[serde(skip, default = "default_collision_world")]
    collision: CollisionWorld,
    #[serde(default)]
    pub weapons: WeaponTable,
    /// What kind of match this is and what ends it.
    #[serde(default)]
    pub rules: ModeRules,
    /// Where the match is in its life.
    #[serde(default)]
    pub match_state: MatchState,
    /// The single ledger every point passes through. `scores` and the per-player counters are
    /// projections of it, never a second tally.
    #[serde(default)]
    pub ledger: ScoreLedger,
    /// What the lifecycle decided this tick, for the server to act on.
    #[serde(default)]
    pub match_events: Vec<MatchEvent>,
    /// The flags this mode puts on the map, and what is happening to them.
    #[serde(default)]
    pub objectives: Objectives,
    /// How Survival is configured, when it is switched on.
    #[serde(default)]
    pub survival: SurvivalConfig,
    /// How Advance is configured, when it is switched on.
    #[serde(default)]
    pub advance: AdvanceConfig,
    /// How the current Survival round stands.
    #[serde(default = "RoundStanding::ongoing")]
    pub round_standing: RoundStanding,
    /// What has happened in this match, beside the score: shots, hits, and what killed whom.
    #[serde(default)]
    pub stats: MatchStats,
    /// Who is watching rather than playing, and what each of them is looking at.
    #[serde(default)]
    pub spectators: BTreeMap<u32, Spectator>,
    /// The kits lying on the map.
    #[serde(default)]
    pub pickups: Pickups,
    /// How this server hands kits out.
    #[serde(default)]
    pub bonuses: BonusConfig,
    /// The bots in this match, by the player id each one drives.
    #[serde(default)]
    pub bots: BTreeMap<u32, Bot>,
    #[serde(default)]
    pub friendly_fire: bool,
    /// Bolted M2 guns. Empty worlds omit the field so existing digests stay put.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guns: Vec<StationaryGun>,
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
            map_spawns: Vec::new(),
            ragdolls: Vec::new(),
            scores: [0, 0],
            events: Vec::new(),
            movement: MovementConfig::default(),
            damage: DamageConfig::default(),
            respawn: RespawnConfig::default(),
            rng: SimRng::default(),
            next_projectile: 1,
            collision,
            weapons: WeaponTable::normal(),
            rules: ModeRules::from_mode_id(mode),
            match_state: MatchState::default(),
            ledger: ScoreLedger::default(),
            match_events: Vec::new(),
            objectives: Objectives::default(),
            survival: SurvivalConfig::default(),
            advance: AdvanceConfig::default(),
            round_standing: RoundStanding::Ongoing,
            stats: MatchStats::default(),
            spectators: BTreeMap::new(),
            pickups: Pickups::default(),
            bonuses: BonusConfig::default(),
            bots: BTreeMap::new(),
            friendly_fire: false,
            guns: Vec::new(),
        }
    }

    /// Puts this mode's flags on the map.
    ///
    /// A team's flag goes to that team's spawn; the neutral flag goes to the middle of the
    /// arena. A mode that wants no flags gets none.
    pub fn place_objectives(&mut self) {
        let spawns = self.map_spawns.clone();
        let rules = self.rules;
        self.objectives = Objectives::for_mode(&rules, |kind| match kind {
            FlagKind::Yellow => Some(Vec2 {
                x: WIDTH / 2.0,
                y: HEIGHT / 2.0,
            }),
            team_flag => {
                let team = team_flag.owning_team();
                spawns
                    .iter()
                    .find(|spawn| spawn.team == team)
                    .map(|spawn| spawn.position)
                    .or_else(|| Some(fallback_spawn(u32::from(team), team)))
            }
        });
    }

    pub fn with_map(mode: &str, map: &ValidatedMap) -> Self {
        let mut world = Self::with_collision(mode, CollisionWorld::from_map(map));
        world.movement.fuel_capacity = (map.asset().start_jet.max(0) as f32 / 100.0).max(0.01);
        world.map_spawns = map
            .asset()
            .spawnpoints
            .iter()
            .filter(|spawn| spawn.active)
            .map(|spawn| MapSpawn {
                position: Vec2 {
                    x: spawn.x as f32,
                    y: spawn.y as f32,
                },
                team: spawn.team.clamp(0, u8::MAX as i32) as u8,
                kind: SpawnKind::from_pms_team(spawn.team),
            })
            .collect();
        world.place_objectives();
        world.place_specials();
        world
    }

    /// Bolts M2s and remembers kit/grenade spots from the map's special spawnpoints.
    fn place_specials(&mut self) {
        self.guns = self
            .map_spawns
            .iter()
            .filter(|spawn| spawn.kind == SpawnKind::StationaryGun)
            .map(|spawn| StationaryGun::new(spawn.position))
            .collect();
    }

    /// The yellow flag *is* the bow: whoever holds it has the weapon, everyone else does not.
    fn sync_rambo_bow(&mut self) {
        if self.rules.kind != ModeKind::Rambomatch {
            return;
        }
        let ammo = u16::from(self.weapons.get(WeaponKind::RamboBow).ammo);
        let holders: Vec<u32> = self.objectives.carriers().map(|(_, id)| id).collect();
        let ids: Vec<u32> = self.players.keys().copied().collect();
        for id in ids {
            let rambo = holders.contains(&id);
            let Some(player) = self.players.get_mut(&id) else {
                continue;
            };
            if rambo {
                if !player.inventory.owns(WeaponKind::RamboBow)
                    && !player.inventory.owns(WeaponKind::FlamedArrows)
                {
                    player.inventory.force_equip(WeaponKind::RamboBow, ammo);
                    sync_hands(player);
                }
            } else if player.inventory.owns(WeaponKind::RamboBow)
                || player.inventory.owns(WeaponKind::FlamedArrows)
            {
                player.inventory.take_kind(WeaponKind::RamboBow);
                player.inventory.take_kind(WeaponKind::FlamedArrows);
                sync_hands(player);
            }
        }
    }

    /// The map geometry this world simulates against.
    ///
    /// Flags, corpses, and kits all collide with the same terrain the players do, so anything that
    /// steps a loose body needs to be handed this rather than building its own.
    pub fn collision(&self) -> &CollisionWorld {
        &self.collision
    }

    pub fn digest(&self) -> WorldDigest {
        fixtures::digest(&serde_json::to_vec(self).unwrap_or_default())
    }
    pub fn add_player(&mut self, id: u32, name: String) {
        self.add_player_as(id, name, TeamChoice::Auto);
    }

    /// Adds a player who asked for a particular side, or to spectate.
    ///
    /// A choice that would stack one side is overridden, and a player who was here before keeps
    /// the score they left with rather than starting again at zero.
    pub fn add_player_as(&mut self, id: u32, name: String, choice: TeamChoice) {
        let sizes = TeamSizes::count(self.players.values().map(|player| player.team));
        let team = assign_team(&self.rules, choice, sizes);
        let mut player = Player::new(id, name, team);
        player.pos = map_spawn(&self.map_spawns, id, team);
        player.fuel = self.movement.fuel_capacity;
        let advance = self.rules.modifiers.advance;
        let unlocked = player.unlocked;
        player.inventory = Inventory::spawn_limited(player.weapon, &self.weapons, |kind| {
            !advance || unlocked.has(kind)
        });
        sync_hands(&mut player);
        self.players.insert(id, player);
        self.ledger.ensure(id);
        self.project_scores();
    }

    /// Moves a player to another side mid-match, which is how team switching and spectating work.
    pub fn set_team(&mut self, id: u32, choice: TeamChoice) -> Option<u8> {
        let sizes = TeamSizes::count(
            self.players
                .iter()
                .filter(|(other, _)| **other != id)
                .map(|(_, player)| player.team),
        );
        let team = assign_team(&self.rules, choice, sizes);
        let player = self.players.get_mut(&id)?;
        player.team = team;
        // Changing sides puts a player back on a spawn rather than teleporting them behind enemy
        // lines from wherever they happened to be standing.
        player.pos = map_spawn(&self.map_spawns, id, team);
        player.vel = Vec2::default();
        Some(team)
    }

    /// Advances the kits on the map, hands out new ones, and applies the ones players walk over.
    ///
    /// Every effect a kit grants is applied here and nowhere else, so a player can never end up
    /// under two at once or under one that nothing is counting down.
    fn step_bonuses(&mut self) {
        // Timed effects run down first, so a kit picked up this tick gets its full duration.
        let expired: Vec<(u32, BonusEffect)> = self
            .players
            .iter_mut()
            .filter_map(|(id, player)| player.bonus.step().map(|effect| (*id, effect)))
            .collect();
        for (player, effect) in expired {
            if effect == BonusEffect::FlameGod {
                if let Some(holder) = self.players.get_mut(&player) {
                    holder.inventory.take_kind(WeaponKind::Flamer);
                    sync_hands(holder);
                }
            }
            self.events.push(Event::BonusExpired { player, effect });
        }

        let config = self.bonuses;
        let spawns = self.map_spawns.clone();
        let mut pickups = std::mem::take(&mut self.pickups);
        let spawned = pickups.step(&config, &self.collision, DT, &mut self.rng, |rng| {
            // A kit drops somewhere a player could stand, which is what the spawns already mark.
            if spawns.is_empty() {
                return Some(Vec2 {
                    x: WIDTH / 2.0,
                    y: HEIGHT / 3.0,
                });
            }
            let index = (rng.next_u64() as usize) % spawns.len();
            Some(spawns[index].position)
        });
        self.pickups = pickups;
        if let Some(kind) = spawned {
            self.events.push(Event::KitSpawned { kind });
        }

        // Anybody standing on a kit takes it, if it has anything to give them.
        let standing: Vec<(u32, usize)> = self
            .players
            .values()
            .filter(|player| player.hp > 0 && !modes::team::is_spectator(player.team))
            .filter_map(|player| {
                self.pickups
                    .nearest(player.pos)
                    .map(|index| (player.id, index))
            })
            .collect();
        // Two players on the same kit: the lower id gets it, and the other finds it gone.
        let mut taken: Vec<usize> = Vec::new();
        for (id, index) in standing {
            if taken.contains(&index) {
                continue;
            }
            let Some(kind) = self.pickups.items.get(index).map(|item| item.kind) else {
                continue;
            };
            if !self.apply_kit(id, kind) {
                continue;
            }
            taken.push(index);
            self.events.push(Event::KitTaken { player: id, kind });
        }
        // Remove from the back so the earlier indices stay valid.
        taken.sort_unstable();
        for index in taken.into_iter().rev() {
            self.pickups.take(index);
        }
    }

    /// Gives one kit to one player, and says whether it was any use to them.
    ///
    /// A kit that would do nothing — a medkit for somebody on full health, a second effect while
    /// one is running — is left on the ground for whoever needs it.
    fn apply_kit(&mut self, id: u32, kind: KitKind) -> bool {
        let grenade_cap = self.damage.max_grenades;
        let flamer_ammo = u16::from(self.weapons.get(WeaponKind::Flamer).ammo);
        let Some(player) = self.players.get_mut(&id) else {
            return false;
        };
        match kind {
            KitKind::Medic => {
                if player.hp >= 100 {
                    return false;
                }
                player.hp = 100;
                true
            }
            KitKind::Grenades => {
                if player.grenades >= grenade_cap && player.cluster_grenades == 0 {
                    return false;
                }
                player.grenades = grenade_cap;
                player.cluster_grenades = 0;
                true
            }
            KitKind::ClusterGrenades => {
                player.cluster_grenades = CLUSTER_GRENADES;
                player.grenades = player.grenades.max(CLUSTER_GRENADES);
                true
            }
            KitKind::Vest => {
                if player.armor >= VEST_ARMOR {
                    return false;
                }
                player.armor = VEST_ARMOR;
                true
            }
            KitKind::FlameGod | KitKind::Berserker | KitKind::Predator => {
                let Some(effect) = kind.effect() else {
                    return false;
                };
                if !player.bonus.activate(effect) {
                    return false;
                }
                // Every timed kit also puts you back on your feet, which is what makes them worth
                // running for when you are hurt.
                player.hp = 100;
                if kind == KitKind::FlameGod {
                    player
                        .inventory
                        .force_equip(WeaponKind::Flamer, flamer_ammo);
                    sync_hands(player);
                }
                true
            }
        }
    }

    /// Adds a bot, which joins as an ordinary player driven by the server.
    ///
    /// Returns the id it was given. Bots go through exactly the same joining path as humans, which
    /// is what keeps team assignment, spawning, and the loadout honest for both.
    pub fn add_bot(&mut self, id: u32, profile: BotProfile, choice: TeamChoice) -> u32 {
        let name = profile.name.clone();
        self.add_player_as(id, name, choice);
        self.bots.insert(id, Bot::new(profile));
        id
    }

    /// Removes one bot and the player it was driving.
    pub fn remove_bot(&mut self, id: u32) -> bool {
        if self.bots.remove(&id).is_none() {
            return false;
        }
        self.players.remove(&id);
        true
    }

    /// Removes bots until only `keep` are left, newest first.
    pub fn trim_bots(&mut self, keep: usize) -> usize {
        let mut ids: Vec<u32> = self.bots.keys().copied().collect();
        ids.sort_unstable();
        let mut removed = 0;
        while self.bots.len() > keep {
            let Some(id) = ids.pop() else { break };
            if self.remove_bot(id) {
                removed += 1;
            }
        }
        removed
    }

    /// Whether this player is driven by the server.
    pub fn is_bot(&self, id: u32) -> bool {
        self.bots.contains_key(&id)
    }

    /// Works out what every bot would press this tick.
    ///
    /// The result is merged into the same input map a human's client fills, so from the
    /// simulation's point of view there is no difference between the two.
    pub fn bot_inputs(&mut self) -> BTreeMap<u32, Input> {
        if self.bots.is_empty() {
            return BTreeMap::new();
        }

        let contacts: Vec<BotContact> = self
            .players
            .values()
            .filter(|player| !modes::team::is_spectator(player.team))
            .map(|player| BotContact {
                id: player.id,
                team: player.team,
                pos: player.pos,
                velocity: player.vel,
                alive: player.hp > 0,
            })
            .collect();
        // Somewhere to send a bot that has nothing else to do. A map with no spawn points would
        // otherwise leave every bot standing exactly where it appeared.
        let spawns: Vec<Vec2> = if self.map_spawns.is_empty() {
            vec![
                fallback_spawn(0, 1),
                fallback_spawn(1, 2),
                Vec2 {
                    x: WIDTH / 2.0,
                    y: HEIGHT / 2.0,
                },
            ]
        } else {
            self.map_spawns.iter().map(|spawn| spawn.position).collect()
        };
        let loot: Vec<Vec2> = self
            .pickups
            .items
            .iter()
            .map(|kit| kit.pos())
            .chain(
                self.objects
                    .iter()
                    .filter(|object| object.kind == DynamicBodyKind::DroppedWeapon && object.active)
                    .map(|object| object.pos),
            )
            .collect();
        let rules = self.rules;
        let table = self.weapons.clone();
        let objectives = self.objectives.clone();
        let tick = self.tick;

        let ids: Vec<u32> = self.bots.keys().copied().collect();
        let mut inputs = BTreeMap::new();
        for id in ids {
            let Some(player) = self.players.get(&id) else {
                continue;
            };
            let Some(me) = contacts.iter().find(|contact| contact.id == id).copied() else {
                continue;
            };
            let holding = WeaponKind::from_slot(player.weapon);
            let team_size = contacts
                .iter()
                .filter(|contact| contact.team == player.team)
                .count();
            let view = BotView {
                me,
                fuel: player.fuel,
                ammo: player.ammo,
                magazine: u16::from(table.get(holding).ammo),
                reloading: player.reload_timer > 0,
                grenades: player.grenades,
                holding,
                unlocked: player.unlocked,
                team_size,
            };
            let Some(mut bot) = self.bots.remove(&id) else {
                continue;
            };
            let input = bots::think(
                &mut bot,
                view,
                bots::BotWorld {
                    others: &contacts,
                    rules: &rules,
                    objectives: &objectives,
                    table: &table,
                    collision: &self.collision,
                    spawns: &spawns,
                    loot: &loot,
                },
                &mut self.rng,
                (tick % u64::from(u32::MAX)) as u32,
            );
            self.bots.insert(id, bot);
            inputs.insert(id, input);
        }
        inputs
    }

    /// Steps the world, driving the bots itself.
    ///
    /// A server calls this instead of `step` so bots and humans move on the same tick from the
    /// same code path.
    pub fn step_with_bots(&mut self, inputs: &BTreeMap<u32, Input>) {
        let mut all = self.bot_inputs();
        // A human's input always wins: a player who takes over a bot's slot is not fighting it.
        for (id, input) in inputs {
            all.insert(*id, *input);
        }
        self.step(&all);
    }

    /// The players a spectator may follow right now, lowest id first.
    pub fn followable(&self) -> Vec<u32> {
        modes::spectator::followable(self.players.values().map(|player| (player.id, player.team)))
    }

    /// Points a spectator at somebody, or lets them fly the camera.
    ///
    /// A command from somebody who is not spectating is ignored rather than quietly turning them
    /// into one; joining the spectators is a team change, which has its own path.
    pub fn spectate(&mut self, id: u32, command: SpectateCommand) {
        if !self
            .players
            .get(&id)
            .is_some_and(|player| modes::team::is_spectator(player.team))
        {
            return;
        }
        let followable = self.followable();
        let spectator = self
            .spectators
            .entry(id)
            .or_insert_with(|| Spectator::new(followable.first().copied()));
        modes::spectator::apply(spectator, command, &followable);
    }

    /// Keeps every spectator watching somebody who is still in the match.
    fn retarget_spectators(&mut self) {
        let followable = self.followable();
        // Anybody who has joined the spectators since last tick gets a camera.
        let watching: Vec<u32> = self
            .players
            .values()
            .filter(|player| modes::team::is_spectator(player.team))
            .map(|player| player.id)
            .collect();
        for id in &watching {
            self.spectators
                .entry(*id)
                .or_insert_with(|| Spectator::new(followable.first().copied()));
        }
        // And anybody who has stopped spectating gives theirs up.
        self.spectators.retain(|id, _| watching.contains(id));
        for spectator in self.spectators.values_mut() {
            modes::spectator::retarget_if_gone(spectator, &followable);
        }
    }

    /// Whether the sides are lopsided enough that somebody should be moved.
    pub fn needs_balancing(&self) -> bool {
        self.rules.is_team_mode()
            && modes::team::needs_balancing(TeamSizes::count(
                self.players.values().map(|player| player.team),
            ))
    }

    /// Test helper: put `kind` in the matching slot and select it immediately.
    pub fn equip(&mut self, id: u32, kind: WeaponKind) {
        let ammo = u16::from(self.weapons.get(kind).ammo);
        if let Some(player) = self.players.get_mut(&id) {
            player.inventory.equip(kind, ammo);
            player.switch_timer = 0;
            player.pending_switch = None;
            player.reload_timer = 0;
            player.startup = 0;
            sync_hands(player);
        }
    }
    pub fn step(&mut self, inputs: &BTreeMap<u32, Input>) {
        self.tick += 1;
        self.events.clear();
        self.match_events.clear();
        let map_spawns = self.map_spawns.clone();
        let mode = self.mode.clone();
        let rules_snapshot = self.rules;
        let respawn_config = self.respawn;
        let damage_config = self.damage;
        let opponents = self.opponent_positions();
        // Damage raised inside the player loop is queued and applied afterwards so that every hit
        // takes the same authoritative path through apply_damage.
        let mut pending: Vec<DamageEvent> = Vec::new();
        self.ragdolls
            .retain_mut(|ragdoll| ragdoll.step(&self.collision));
        let lived_projectiles = self.projectiles.len();
        for player in self.players.values_mut() {
            if player.hp <= 0 {
                // A dead player still picks the weapon they will carry back in.
                if let Some(input) = inputs.get(&player.id) {
                    player.last_seq = input.seq;
                    player.weapon = input.weapon.min((SELECTABLE_WEAPONS.len() - 1) as u8);
                }
                if !modes::modifiers::survival::may_respawn(rules_snapshot.modifiers.survival) {
                    // In Survival there is nothing to count down to: they are out for the round.
                    continue;
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
                    player.pos = select_spawn(&map_spawns, &mode, player.team, player.id, &enemies);
                    player.vel = Vec2::default();
                    player.fuel = 1.0;
                    let unlocked = player.unlocked;
                    player.inventory =
                        Inventory::spawn_limited(player.weapon, &self.weapons, |kind| {
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
                    self.events.push(Event::Respawn { player: player.id });
                }
                continue;
            }
            player.spawn_protection = player.spawn_protection.saturating_sub(1);
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
            let input = inputs.get(&player.id).copied().unwrap_or_default();
            player.last_seq = input.seq;
            let selected = input.weapon.min((SELECTABLE_WEAPONS.len() - 1) as u8);
            let selected = if rules_snapshot.modifiers.advance {
                // A locked weapon falls back to one they have rather than being refused outright.
                modes::modifiers::advance::resolve_choice(
                    true,
                    player.unlocked,
                    WeaponKind::from_slot(selected),
                )
                .slot()
                .unwrap_or(selected)
            } else {
                selected
            };
            let selected_kind = WeaponKind::from_slot(selected);
            if let Some(slot) = player.inventory.slot_of(selected_kind) {
                if slot == player.inventory.active && player.pending_switch.is_some() {
                    player.pending_switch = None;
                    player.switch_timer = 0;
                } else if slot != player.inventory.active && player.pending_switch != Some(slot) {
                    weapons::reload::interrupt(&mut player.reload_timer);
                    player.pending_switch = Some(slot);
                    player.switch_timer = SWITCH_DELAY_TICKS;
                    player.startup = 0;
                    player.throw_charge = 0;
                }
            }
            if player.switch_timer > 0 {
                player.switch_timer -= 1;
                if player.switch_timer == 0 {
                    if let Some(slot) = player.pending_switch.take() {
                        player.inventory.active = slot;
                        sync_hands(player);
                        player.startup = 0;
                    }
                }
            }
            let held = held_kind(player);
            let weapon = *self.weapons.get(held);
            if !input.fire {
                player.burst = 0;
                player.recoil_aim *= 0.8;
                player.bink = player.bink.saturating_sub(1);
            }
            player.accuracy = inaccuracy(&weapon, &shooter_pose(player, &input));
            if player.cooldown > 0 {
                player.cooldown -= 1;
            }
            if player.grenade_cooldown > 0 {
                player.grenade_cooldown -= 1;
            }
            if weapons::reload::tick(&mut player.reload_timer) {
                player.ammo = u16::from(weapon.ammo);
                writeback_ammo(player);
            }
            if input.reload && !player.previous_input.reload && player.switch_timer == 0 {
                if let Some(time) = weapons::reload::start(
                    player.ammo,
                    u16::from(weapon.ammo),
                    player.reload_timer,
                    weapon.reload_time,
                ) {
                    player.reload_timer = time;
                    self.events.push(Event::Reload { player: player.id });
                }
            }
            let previous = player.previous_input;
            let has_standing_clearance = !player.state.is_low()
                || self
                    .collision
                    .has_standing_clearance(player.pos, CollisionMask::PLAYER);
            advance_character(player, input, &self.movement, has_standing_clearance);
            let start = player.pos;
            let impact_velocity = player.vel.y.max(0.0);
            let desired = Vec2 {
                // Acceptance evidence: rust:map:bounds
                x: (start.x + player.vel.x * DT).clamp(16.0, WIDTH - 16.0),
                y: (start.y + player.vel.y * DT).clamp(16.0, HEIGHT - 16.0),
            };
            let was_grounded = player.grounded;
            player.grounded = false;
            let contact = self.collision.sweep_shape(
                start,
                desired,
                &character::body::shape_for(player.state),
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
            if !was_grounded
                && player.grounded
                && player.last_impact > damage_config.fall_damage_speed
            {
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
            if input.fire && player.switch_timer == 0 {
                player.startup = player.startup.saturating_add(1);
            } else if !input.fire {
                player.startup = 0;
            }
            let unarmed = player.inventory.active().is_none();
            if input.fire
                && player.switch_timer == 0
                && player.cooldown == 0
                && player.reload_timer == 0
                && player.ammo == 0
                && !unarmed
                && !previous.fire
            {
                self.events.push(Event::Empty { player: player.id });
            }
            if input.fire
                && player.switch_timer == 0
                && player.cooldown == 0
                && player.reload_timer == 0
                && (unarmed || player.ammo > 0)
                && player.startup > weapon.start_up_time
            {
                let dx = input.aim.x - player.pos.x;
                let dy = input.aim.y - player.pos.y;
                let len = (dx * dx + dy * dy).sqrt();
                let pose = shooter_pose(player, &input);
                if len > 1.0 && len.is_finite() && !firing::may_fire(&weapon, &pose) {
                    // Say so once per wind-up, on the tick the shot would otherwise have gone,
                    // rather than every tick the trigger stays down.
                    if player.startup == weapon.start_up_time.saturating_add(1) {
                        self.events.push(Event::FireRefused {
                            player: player.id,
                            reason: refusal(&weapon, &pose).unwrap_or(FireRefusal::NeedsBracing),
                        });
                    }
                } else if len > 1.0 && len.is_finite() {
                    player.bink = self_bink_on_fire(&weapon, player.bink, &pose);
                    player.accuracy = inaccuracy(&weapon, &pose);
                    let recoil = player.recoil_aim;
                    let base_angle = dy.atan2(dx) + recoil;
                    for pellet in 0..weapon.pellets() {
                        let fan = per_pellet_spread(&weapon, pellet, self.rng.next_unit());
                        let jitter =
                            (self.rng.next_unit() * 2.0 - 1.0) * max_deviation(player.accuracy);
                        let angle = base_angle + fan + jitter;
                        let dir = Vec2 {
                            x: angle.cos(),
                            y: angle.sin(),
                        };
                        let muzzle = muzzle_velocity(&weapon, dir, player.vel.scale(DT));
                        let world_vel = muzzle.scale(1.0 / DT);
                        let barrel = barrel_origin(
                            &weapon,
                            muzzle_origin(player.pos, input.aim, player.state, player.facing),
                            dir,
                            pellet,
                        );
                        self.projectiles.push(Projectile {
                            id: self.next_projectile,
                            owner: player.id,
                            pos: barrel,
                            vel: world_vel,
                            ttl: weapon.timeout(),
                            damage: 0,
                            explosive: weapon.bullet_style.is_explosive(),
                            kind: projectile_kind(&weapon),
                            splash_radius: weapon.explosion_radius(),
                            weapon: Some(weapon.kind),
                            origin: barrel,
                            last_impact: barrel,
                        });
                        self.next_projectile = self.next_projectile.wrapping_add(1);
                    }
                    if !unarmed {
                        player.ammo = player.ammo.saturating_sub(1);
                        writeback_ammo(player);
                    }
                    player.cooldown = weapon.fire_interval;
                    if !unarmed && player.ammo == 0 {
                        player.reload_timer = weapon.reload_time;
                    }
                    let barrel = muzzle_origin(player.pos, input.aim, player.state, player.facing);
                    self.events.push(Event::MuzzleFlash {
                        player: player.id,
                        pos: barrel,
                    });
                    self.events.push(Event::Casing {
                        player: player.id,
                        pos: Vec2 {
                            x: player.pos.x - (barrel.x - player.pos.x) * 0.3,
                            y: player.pos.y + 4.0,
                        },
                    });
                    player.recoil_aim += recoil_radians(&weapon, player.burst, &pose);
                    player.burst = player.burst.saturating_add(1);
                    let aim_unit = Vec2 {
                        x: dx / len,
                        y: dy / len,
                    };
                    let source = match weapon.kind {
                        WeaponKind::Spas12 => ImpulseSource::SpasBoost,
                        WeaponKind::Minigun => ImpulseSource::MinigunBoost,
                        _ => ImpulseSource::Recoil,
                    };
                    let boost = if boosts_the_shooter(&weapon) {
                        // A SPAS or minigun blast moves the firer, and players ride it deliberately.
                        self_boost(&weapon, aim_unit, &pose).scale(1.0 / DT)
                    } else {
                        Vec2 {
                            x: -aim_unit.x * 1.5,
                            y: -aim_unit.y * 1.5,
                        }
                    };
                    apply_impulse(
                        player,
                        Impulse {
                            velocity: boost,
                            source,
                        },
                    );
                    self.events.push(Event::Shot { player: player.id });
                    self.stats.record(StatEvent::Shot {
                        player: player.id,
                        weapon: weapon.kind,
                    });
                }
            }
            // Holding the throw key cooks the grenade: the longer it is held the harder it goes,
            // and it only leaves the hand once the key is released or the wind-up is complete.
            if input.throw_grenade && player.grenades > 0 && player.grenade_cooldown == 0 {
                player.grenade_charge = player.grenade_charge.saturating_add(1);
            }
            let wound_up = throw_frame(player.grenade_charge) >= THROW_LAST_FRAME;
            let releasing = player.grenade_held && !input.throw_grenade;
            if (releasing || wound_up) && player.grenades > 0 && player.grenade_cooldown == 0 {
                let dx = input.aim.x - player.pos.x;
                let dy = input.aim.y - player.pos.y;
                let len = (dx * dx + dy * dy).sqrt();
                if len > 1.0 && len.is_finite() {
                    let kind = if player.cluster_grenades > 0 {
                        WeaponKind::ClusterGrenade
                    } else {
                        WeaponKind::FragGrenade
                    };
                    let grenade = *self.weapons.get(kind);
                    let dir = Vec2 {
                        x: dx / len,
                        y: dy / len,
                    };
                    let thrown = throw_velocity(
                        &grenade,
                        dir,
                        player.grenade_charge.saturating_sub(1),
                        player.vel.scale(DT),
                    );
                    self.projectiles.push(Projectile {
                        id: self.next_projectile,
                        owner: player.id,
                        pos: player.pos,
                        vel: thrown.scale(1.0 / DT),
                        ttl: grenade.timeout(),
                        damage: 0,
                        explosive: true,
                        kind: ProjectileKind::FragGrenade,
                        splash_radius: grenade.explosion_radius(),
                        weapon: Some(kind),
                        origin: player.pos,
                        last_impact: player.pos,
                    });
                    self.next_projectile = self.next_projectile.wrapping_add(1);
                    if player.cluster_grenades > 0 {
                        player.cluster_grenades -= 1;
                    }
                    player.grenades -= 1;
                    player.grenade_cooldown = 80;
                    player.grenade_charge = 0;
                }
            }
            if !input.throw_grenade {
                player.grenade_charge = 0;
            }
            player.grenade_held = input.throw_grenade;
            if player.switch_timer == 0 && input.drop && !previous.drop {
                if let Some(slot) = player.inventory.drop_active() {
                    weapons::reload::interrupt(&mut player.reload_timer);
                    player.throw_charge = 0;
                    self.objects
                        .push(dropped_weapon(slot.kind, slot.ammo, player.pos, player.vel));
                    sync_hands(player);
                }
            }
            if player.switch_timer == 0 && input.throw_weapon && player.inventory.active().is_some()
            {
                player.throw_charge = player.throw_charge.saturating_add(1).min(MAX_THROW_CHARGE);
            } else if player.throw_charge > 0 && !input.throw_weapon {
                if let Some(slot) = player.inventory.drop_active() {
                    weapons::reload::interrupt(&mut player.reload_timer);
                    let facing = player.facing.sign();
                    let dir = aim_dir(player.pos, input.aim, facing)
                        .unwrap_or(Vec2 { x: facing, y: 0.0 });
                    let vel = charged_velocity(dir, player.throw_charge, player.vel);
                    self.objects
                        .push(dropped_weapon(slot.kind, slot.ammo, player.pos, vel));
                    sync_hands(player);
                }
                player.throw_charge = 0;
            }
            if player.switch_timer == 0 && input.throw_knife && !previous.throw_knife {
                if let Some(_slot) = player
                    .inventory
                    .take_kind(WeaponKind::CombatKnife)
                    .or_else(|| player.inventory.take_kind(WeaponKind::ThrownKnife))
                {
                    weapons::reload::interrupt(&mut player.reload_timer);
                    if let Some(dir) = aim_dir(player.pos, input.aim, player.facing.sign()) {
                        let knife = *self.weapons.get(WeaponKind::ThrownKnife);
                        let barrel =
                            muzzle_origin(player.pos, input.aim, player.state, player.facing);
                        let muzzle = muzzle_velocity(&knife, dir, player.vel.scale(DT));
                        self.projectiles.push(Projectile {
                            id: self.next_projectile,
                            owner: player.id,
                            pos: barrel,
                            vel: muzzle.scale(1.0 / DT),
                            ttl: knife.timeout(),
                            damage: 0,
                            explosive: false,
                            kind: ProjectileKind::ThrownKnife,
                            splash_radius: 0.0,
                            weapon: Some(WeaponKind::ThrownKnife),
                            origin: barrel,
                            last_impact: barrel,
                        });
                        self.next_projectile = self.next_projectile.wrapping_add(1);
                    }
                    sync_hands(player);
                }
            }
            if player.switch_timer == 0 && input.pickup {
                if let Some(index) = nearest_index(player.pos, &self.objects) {
                    let object = self.objects.remove(index);
                    if let Some(kind) = object.weapon_slot.map(WeaponKind::from_slot) {
                        weapons::reload::interrupt(&mut player.reload_timer);
                        if let Some(displaced) = player.inventory.pickup(WeaponSlot {
                            kind,
                            ammo: object.ammo,
                        }) {
                            self.objects.push(dropped_weapon(
                                displaced.kind,
                                displaced.ammo,
                                player.pos,
                                Vec2::default(),
                            ));
                        }
                        sync_hands(player);
                    }
                }
                let m2_ammo = u16::from(self.weapons.get(WeaponKind::StationaryGun).ammo);
                for gun in &mut self.guns {
                    if gun.mount(player.id, player.pos) {
                        player
                            .inventory
                            .force_equip(WeaponKind::StationaryGun, m2_ammo);
                        sync_hands(player);
                    }
                }
            }
            for gun in &mut self.guns {
                if gun.mounted_by == Some(player.id) && !gun.in_reach(player.pos) {
                    gun.dismount(player.id);
                    player.inventory.take_kind(WeaponKind::StationaryGun);
                    sync_hands(player);
                }
            }
        }
        let player_ids = self.players.keys().copied().collect::<Vec<_>>();
        for (index, left_id) in player_ids.iter().copied().enumerate() {
            for right_id in player_ids.iter().copied().skip(index + 1) {
                if let Some(mut right) = self.players.remove(&right_id) {
                    if let Some(left) = self.players.get_mut(&left_id) {
                        resolve_player_contact(left, &mut right);
                    }
                    self.players.insert(right_id, right);
                }
            }
        }
        for object in &mut self.objects {
            object.step(&self.collision, DT);
        }
        let teams: BTreeMap<u32, u8> = self.players.iter().map(|(&id, p)| (id, p.team)).collect();
        let weapons = self.weapons.clone();
        let friendly_fire = self.friendly_fire;
        let mut hits = Vec::new();
        let mut knife_recoveries = Vec::new();
        // Where a cluster grenade went off, so its pieces can be scattered once the loop is done.
        let mut cluster_pieces: Vec<(u32, Vec2, Vec2)> = Vec::new();
        // Where a blast went off, so loose flags can be shoved once the loop releases them.
        let mut blast_pushes: Vec<(Vec2, f32)> = Vec::new();
        let mut terrain_impacts: Vec<(Vec2, Vec2)> = Vec::new();
        let mut newborns = self.projectiles.split_off(lived_projectiles);
        self.projectiles.retain_mut(|bullet| {
            let previous_pos = bullet.pos;
            let style = bullet
                .weapon
                .map_or(BulletStyle::Plain, |kind| weapons.get(kind).bullet_style);
            let pull = gravity_multiplier(style);
            if pull > 0.0 {
                bullet.vel.y += PROJECTILE_GRAVITY * pull * DT;
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
                    splash_hits(
                        bullet,
                        &self.players,
                        &teams,
                        &weapons,
                        friendly_fire,
                        &mut hits,
                    );
                    blast_pushes.push((bullet.pos, bullet.splash_radius));
                    if matches!(style, BulletStyle::ClusterGrenade) {
                        cluster_pieces.push((bullet.owner, bullet.pos, bullet.vel));
                    }
                }
                if thrown_knife(bullet) {
                    knife_recoveries.push(bullet.pos);
                }
                return false;
            }
            let Some(owner_team) = teams.get(&bullet.owner) else {
                return false;
            };
            let terrain = self
                .collision
                .raycast(previous_pos, bullet.pos, CollisionMask::BULLET)
                .filter(|_| grenade_is_armed(&armed_def(&weapons, bullet), bullet.ttl));
            let platform_hit = terrain.map(|hit| hit.time);
            let mut player_hit: Option<(f32, u32)> = None;
            for target in self.players.values() {
                if target.hp <= 0 || target.id == bullet.owner {
                    continue;
                }
                if !collides_with_bodies(style, bullet.ttl) {
                    continue;
                }
                let same_team = *owner_team != 0 && *owner_team == target.team;
                let allowed = if let Some(kind) = bullet.weapon {
                    can_damage(
                        weapons.get(kind),
                        bullet.owner,
                        target.id,
                        same_team,
                        friendly_fire,
                        false,
                    )
                } else {
                    target.id != bullet.owner && (*owner_team == 0 || *owner_team != target.team)
                };
                if !allowed {
                    continue;
                }
                if let Some(t) = segment_circle_t(previous_pos, bullet.pos, target.pos, 18.0) {
                    if player_hit.is_none_or(|(best, _)| t < best) {
                        player_hit = Some((t, target.id));
                    }
                }
            }
            // Flags, kits, and dropped guns are knocked about by solid rounds passing through
            // them. The shove does not stop the round; it only moves the object.
            if pushes_objects(style) {
                if let Some(def) = bullet.weapon.map(|kind| *weapons.get(kind)) {
                    let shove =
                        push_impulse(&def, bullet.vel.scale(DT)).scale(OBJECT_PUSH_MULTIPLIER / DT);
                    for object in self.objects.iter_mut().filter(|object| object.active) {
                        if segment_circle_t(previous_pos, bullet.pos, object.pos, object.radius)
                            .is_some()
                        {
                            object.vel = object.vel.add(shove);
                            object.grounded = false;
                        }
                    }
                    // Kits are shoved by the same round.
                    for kit in &mut self.pickups.items {
                        if segment_circle_t(previous_pos, bullet.pos, kit.body.pos, kit.body.radius)
                            .is_some()
                        {
                            kit.push(shove);
                        }
                    }
                    // Flags are shoved by the same round, which is what lets a team blast a
                    // dropped flag away from the enemy standing over it.
                    for flag in &mut self.objectives.flags {
                        if flag.state.carrier().is_none()
                            && segment_circle_t(
                                previous_pos,
                                bullet.pos,
                                flag.body.pos,
                                flag.body.radius,
                            )
                            .is_some()
                        {
                            flag.push(shove);
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
                // Terrain does not end every shot. A grenade bounces off it, a rocket reached
                // from far enough away skips along it, an arrow buries itself in it, and a flame
                // gutters out against it. Only once terrain is done with the projectile does it
                // get to wound anybody or go off.
                let struck_terrain =
                    platform_hit.is_some_and(|wall_t| wall_t <= player_t.unwrap_or(f32::INFINITY));
                if struck_terrain {
                    if let Some(hit) = terrain {
                        // Every meeting of round and wall is worth a spark, whatever the round then
                        // does about it — bounce, stick, ricochet, or stop.
                        terrain_impacts.push((bullet.pos, hit.normal));
                        match surface_response(style) {
                            SurfaceResponse::Bounce { restitution } => {
                                bullet.vel = bounce_velocity(bullet.vel, hit.normal, restitution);
                                bullet.pos = nudge(bullet.pos, hit.normal);
                                bullet.last_impact = bullet.pos;
                                return true;
                            }
                            SurfaceResponse::Stick { resist_ticks } => {
                                bullet.vel = Vec2::default();
                                bullet.ttl = bullet.ttl.min(resist_ticks);
                                bullet.pos = nudge(bullet.pos, hit.normal);
                                bullet.last_impact = bullet.pos;
                                return bullet.ttl > 0;
                            }
                            SurfaceResponse::Smother { ticks } => {
                                bullet.ttl = bullet.ttl.min(ticks);
                                return bullet.ttl > 0;
                            }
                            SurfaceResponse::Ricochet { min_travel }
                                if bullet.pos.add(bullet.last_impact.scale(-1.0)).length()
                                    > min_travel =>
                            {
                                bullet.vel = ricochet_velocity(bullet.vel, hit.normal);
                                bullet.pos = nudge(bullet.pos, hit.normal);
                                bullet.last_impact = bullet.pos;
                                return true;
                            }
                            SurfaceResponse::Stop
                            | SurfaceResponse::Explode
                            | SurfaceResponse::Ricochet { .. } => {}
                        }
                    }
                }
                if bullet.explosive {
                    splash_hits(
                        bullet,
                        &self.players,
                        &teams,
                        &weapons,
                        friendly_fire,
                        &mut hits,
                    );
                    blast_pushes.push((bullet.pos, bullet.splash_radius));
                    if matches!(style, BulletStyle::ClusterGrenade) {
                        cluster_pieces.push((bullet.owner, bullet.pos, bullet.vel));
                    }
                } else if player_t
                    .is_some_and(|hit_t| hit_t <= platform_hit.unwrap_or(f32::INFINITY))
                {
                    let target_id = player_hit.unwrap().1;
                    let region = self
                        .players
                        .get(&target_id)
                        .map_or(BodyRegion::Chest, |target| {
                            character::body::shape_for(target.state)
                                .region_at(target.pos, bullet.pos)
                        });
                    let (damage, impulse, pre_scaled) = if let Some(kind) = bullet.weapon {
                        let def = weapons.get(kind);
                        let soldat_speed = bullet.vel.length() * DT;
                        let distance = (bullet.pos.add(bullet.origin.scale(-1.0))).length();
                        let amount = direct_damage(
                            def,
                            degraded_hit_multiply(def, distance),
                            soldat_speed,
                            region,
                        )
                        .round() as i32;
                        (
                            amount,
                            Impulse {
                                velocity: push_impulse(def, bullet.vel.scale(DT)),
                                source: ImpulseSource::Bullet,
                            },
                            true,
                        )
                    } else {
                        let direction_length = bullet.vel.length().max(1.0);
                        (
                            bullet.damage,
                            Impulse {
                                velocity: Vec2 {
                                    x: bullet.vel.x / direction_length * 8.0,
                                    y: bullet.vel.y / direction_length * 8.0,
                                },
                                source: ImpulseSource::Bullet,
                            },
                            false,
                        )
                    };
                    hits.push(PendingHit {
                        killer: bullet.owner,
                        target: target_id,
                        damage,
                        impulse,
                        region,
                        cause: match bullet.kind {
                            ProjectileKind::Melee => DamageCause::Melee,
                            ProjectileKind::ShotgunPellet => DamageCause::Pellet,
                            _ => DamageCause::Bullet,
                        },
                        pre_scaled,
                        bink_weapon: bullet.weapon,
                    });
                } else if thrown_knife(bullet) {
                    knife_recoveries.push(bullet.pos);
                }
                return false;
            }
            true
        });
        self.projectiles.append(&mut newborns);
        for (pos, normal) in terrain_impacts {
            self.events.push(Event::Impact { pos, normal });
        }
        for &(at, radius) in &blast_pushes {
            self.events.push(Event::Explosion {
                pos: at,
                radius: radius.max(1.0),
            });
        }
        for (at, radius) in blast_pushes {
            self.pickups.push_near(
                at,
                radius,
                Vec2 {
                    x: 0.0,
                    y: -EXPLOSION_FLAG_PUSH * 0.5,
                },
            );
            // A blast throws a loose flag away from itself, hardest at the centre.
            for flag in &mut self.objectives.flags {
                if flag.state.carrier().is_some() {
                    continue;
                }
                let away = Vec2 {
                    x: flag.body.pos.x - at.x,
                    y: flag.body.pos.y - at.y,
                };
                let distance = away.length();
                if distance >= radius || radius <= 0.0 {
                    continue;
                }
                let scale = (1.0 - distance / radius) * EXPLOSION_FLAG_PUSH;
                let direction = if distance > f32::EPSILON {
                    Vec2 {
                        x: away.x / distance,
                        y: away.y / distance,
                    }
                } else {
                    Vec2 { x: 0.0, y: -1.0 }
                };
                flag.push(direction.scale(scale));
            }
        }
        for (owner, pos, velocity) in cluster_pieces {
            let piece = *self.weapons.get(WeaponKind::Cluster);
            for scatter in cluster_submunitions(velocity.scale(DT), &mut self.rng) {
                self.projectiles.push(Projectile {
                    id: self.next_projectile,
                    owner,
                    pos,
                    vel: scatter.scale(1.0 / DT),
                    ttl: piece.timeout(),
                    damage: 0,
                    explosive: true,
                    kind: ProjectileKind::FragGrenade,
                    splash_radius: piece.explosion_radius(),
                    weapon: Some(WeaponKind::Cluster),
                    origin: pos,
                    last_impact: pos,
                });
                self.next_projectile = self.next_projectile.wrapping_add(1);
            }
        }
        for pos in knife_recoveries {
            self.objects.push(dropped_weapon(
                WeaponKind::CombatKnife,
                1,
                pos,
                Vec2::default(),
            ));
        }
        for hit in hits {
            if let Some(target) = self.players.get_mut(&hit.target) {
                if target.hp <= 0 || target.spawn_protection > 0 {
                    continue;
                }
                apply_impulse(target, hit.impulse);
                if let Some(kind) = hit.bink_weapon {
                    let victim_id = target.id;
                    let same_team = {
                        let attacker_team = teams.get(&hit.killer).copied().unwrap_or(0);
                        attacker_team != 0 && attacker_team == target.team
                    };
                    if should_bink(
                        self.weapons.get(kind),
                        hit.killer,
                        victim_id,
                        same_team,
                        self.friendly_fire,
                    ) {
                        target.bink = bink_on_hit(self.weapons.get(kind), target.bink);
                    }
                }
            }
            let direction = hit.impulse.velocity;
            self.apply_damage(DamageEvent {
                attacker: Some(hit.killer),
                target: hit.target,
                amount: hit.damage,
                region: hit.region,
                cause: hit.cause,
                direction,
                pre_scaled: hit.pre_scaled,
            });
        }
        for event in pending {
            self.apply_damage(event);
        }

        // The flags move and change hands before the lifecycle looks at the score, so a capture on
        // the last tick of a match still counts towards deciding it.
        let bearers: Vec<Bearer> = self
            .players
            .values()
            .map(|player| Bearer {
                id: player.id,
                team: player.team,
                pos: player.pos,
                velocity: player.vel,
                alive: player.hp > 0,
                throwing: inputs
                    .get(&player.id)
                    .is_some_and(|input| input.throw_weapon || (input.jump && input.crouch)),
                aim: inputs.get(&player.id).map_or(player.pos, |input| input.aim),
            })
            .collect();
        let sizes = TeamSizes::count(self.players.values().map(|player| player.team));
        let rules = self.rules;
        let mut objectives = std::mem::take(&mut self.objectives);
        let awards = objectives.step(
            &rules,
            &self.collision,
            DT,
            &bearers,
            (sizes.alpha, sizes.bravo),
        );
        self.objectives = objectives;
        self.sync_rambo_bow();
        for award in awards {
            let team_of = |id: u32| self.players.get(&id).map_or(0, |player| player.team);
            self.ledger.record(award.into_event(), &rules, team_of);
        }
        for event in &self.objectives.events {
            match event {
                FlagEvent::Captured { by, .. } => self.stats.record(StatEvent::Objective {
                    player: *by,
                    kind: ObjectiveStat::Capture,
                }),
                FlagEvent::Returned { by: Some(by), .. } => {
                    self.stats.record(StatEvent::Objective {
                        player: *by,
                        kind: ObjectiveStat::Return,
                    })
                }
                _ => {}
            }
        }

        // Survival decides its own round: the clock and the score limits are beside the point when
        // the question is who is still breathing.
        if rules.modifiers.survival {
            let survivors: Vec<Survivor> = self
                .players
                .values()
                .map(|player| Survivor {
                    id: player.id,
                    team: player.team,
                    alive: player.hp > 0,
                    spectator: modes::team::is_spectator(player.team),
                })
                .collect();
            self.round_standing = survival_standing(&rules, &survivors);
        } else {
            self.round_standing = RoundStanding::Ongoing;
        }

        self.step_bonuses();
        self.retarget_spectators();

        // The lifecycle runs last, so it sees this tick's scores before deciding the match is over.
        self.project_scores();
        let playing = self
            .players
            .values()
            .filter(|player| !modes::team::is_spectator(player.team))
            .count();
        let rules = self.rules;
        if let Some(event) = self.match_state.step(&rules, &self.ledger, playing) {
            // A round that just ended shows its scoreboard in the same breath.
            if matches!(event, MatchEvent::RoundEnded { .. }) {
                let scoreboard = self.match_state.scoreboard_event();
                self.match_events.push(event);
                self.match_events.push(scoreboard);
            } else {
                self.match_events.push(event);
            }
        }
    }

    /// Restarts the match: a fresh countdown, an empty ledger, and everybody back on a spawn.
    ///
    /// Players keep their place in the room; only the score and the clock are wiped, which is what
    /// makes this a restart rather than a new room.
    pub fn restart_match(&mut self) {
        let rules = self.rules;
        let event = self.match_state.restart(&rules);
        self.objectives.reset();
        self.pickups.clear();
        for player in self.players.values_mut() {
            player.bonus.clear();
        }
        self.ledger = ScoreLedger::default();
        self.stats.clear();
        for id in self.players.keys().copied().collect::<Vec<_>>() {
            self.ledger.ensure(id);
        }
        self.project_scores();
        self.match_events.push(event);
    }

    /// Copies the ledger onto the per-player counters and the team scores.
    ///
    /// Those are what the HUD, the scoreboard, and the wire format read. Writing them from the
    /// ledger rather than alongside it is what keeps one authority for every point in the match.
    fn project_scores(&mut self) {
        for (id, player) in self.players.iter_mut() {
            let score = self.ledger.player(*id);
            player.kills = score.kills;
            player.deaths = score.deaths;
            player.teamkills = score.teamkills;
            player.suicides = score.suicides;
        }
        let teams = self.ledger.teams();
        self.scores = [teams[1].max(0) as u32, teams[2].max(0) as u32];
    }

    /// Living players this one would rather not spawn next to: the other team in team modes and
    /// everybody else otherwise.
    fn opponent_positions(&self) -> BTreeMap<u32, Vec<Vec2>> {
        let team_mode = self.mode == "team";
        self.players
            .keys()
            .map(|&id| {
                let team = self.players.get(&id).map_or(0, |player| player.team);
                let positions = self
                    .players
                    .values()
                    .filter(|other| other.id != id && other.hp > 0)
                    .filter(|other| !team_mode || other.team != team)
                    .map(|other| other.pos)
                    .collect();
                (id, positions)
            })
            .collect()
    }

    /// Carries out a typed player command. The parser already decided what was asked; this decides
    /// whether the world will do it.
    pub fn apply_player_command(&mut self, player: u32, command: PlayerCommand) -> bool {
        match command {
            PlayerCommand::Kill | PlayerCommand::BrutalKill => {
                let Some(target) = self.players.get(&player) else {
                    return false;
                };
                if target.hp <= 0 {
                    return false;
                }
                let amount = if command == PlayerCommand::BrutalKill {
                    self.damage.gib_damage.max(target.hp)
                } else {
                    target.hp
                };
                self.apply_damage(DamageEvent {
                    attacker: Some(player),
                    target: player,
                    amount,
                    region: BodyRegion::Chest,
                    cause: DamageCause::Bullet,
                    direction: Vec2::default(),
                    pre_scaled: true,
                });
                true
            }
            PlayerCommand::Mercy
            | PlayerCommand::Smoke
            | PlayerCommand::Tabac
            | PlayerCommand::Takeoff
            | PlayerCommand::Victory => {
                let Some(emote) = command.emote() else {
                    return false;
                };
                let Some(target) = self.players.get_mut(&player) else {
                    return false;
                };
                if target.hp <= 0 {
                    return false;
                }
                target.state = CharacterState::Emote {
                    emote,
                    ticks_left: 90,
                };
                true
            }
            PlayerCommand::Pause | PlayerCommand::Unpause => false,
        }
    }

    /// The single authoritative damage path. Bullets, explosions, melee, falls, bleeding, and
    /// deadly polygons all arrive here, so armor, attribution, death, and the kill feed can never
    /// disagree about what happened.
    pub fn apply_damage(&mut self, mut event: DamageEvent) {
        // Flame God is exactly what it says: nothing touches them while it lasts.
        if self
            .players
            .get(&event.target)
            .is_some_and(|player| player.bonus.is_invulnerable())
        {
            return;
        }
        // A Berserker hits four times as hard, which is applied once, here, rather than at every
        // place a weapon works out its damage.
        if let Some(attacker) = event.attacker.filter(|id| *id != event.target) {
            let multiplier = self
                .players
                .get(&attacker)
                .map_or(1.0, |player| player.bonus.damage_multiplier());
            if multiplier != 1.0 {
                event.amount = ((event.amount as f32) * multiplier).round() as i32;
            }
        }
        let tick = self.tick;
        let config = self.damage;
        let respawn_config = self.respawn;
        let team_mode = self.mode == "team";
        let teams: BTreeMap<u32, u8> = self
            .players
            .iter()
            .map(|(&id, player)| (id, player.team))
            .collect();

        let Some(target) = self.players.get_mut(&event.target) else {
            return;
        };
        if target.hp <= 0 {
            return;
        }
        let self_inflicted = event.attacker == Some(event.target);
        if target.spawn_protection > 0 && !self_inflicted {
            return;
        }
        let scaled = if event.pre_scaled {
            event.amount.max(0)
        } else {
            ((event.amount.max(0) as f32) * region_multiplier(&config, event.region)).round() as i32
        };
        if scaled <= 0 {
            return;
        }
        let split = absorb(&config, target.armor, scaled);
        target.armor = (target.armor - split.armor).max(0);
        target.hp = (target.hp - split.health).max(0);
        target.last_damage_direction = event.direction;
        if let Some(attacker) = event.attacker.filter(|id| *id != event.target) {
            record_attribution(&mut target.attackers, attacker, tick, split.health);
        }
        if event.cause.draws_blood() && split.health >= config.bleed_threshold && target.hp > 0 {
            target.bleed = Some(Bleed::open(&config, event.attacker));
        }
        let hp = target.hp;
        let position = target.pos;
        let velocity = target.vel;
        let target_team = target.team;
        let attackers = target.attackers.clone();
        let died = hp == 0;
        let gibbed = died && split.health >= config.gib_damage;
        let death_drops = if died {
            Some((target.inventory.take_all(), position, velocity))
        } else {
            None
        };
        // A grenade that was being cooked when its thrower was killed falls out of the dead hand
        // and goes off where it lands, which is what makes trading with a cooked grenade work.
        let dropped_grenade = if died && target.grenade_charge > 0 && target.grenades > 0 {
            target.grenades -= 1;
            target.grenade_charge = 0;
            let kind = if target.cluster_grenades > 0 {
                target.cluster_grenades -= 1;
                WeaponKind::ClusterGrenade
            } else {
                WeaponKind::FragGrenade
            };
            Some((kind, position, velocity))
        } else {
            None
        };
        if died {
            target.respawn = respawn_config.delay_ticks;
            target.bleed = None;
            target.bonus.clear();
            target.state = CharacterState::Dead;
        }

        self.events.push(Event::Damage {
            target: event.target,
            attacker: event.attacker,
            amount: split.health,
            region: event.region,
            cause: event.cause,
            direction: event.direction,
        });
        self.events.push(Event::Hit {
            target: event.target,
            hp,
        });
        if event.cause.draws_blood() {
            self.events.push(Event::Blood {
                target: event.target,
                position,
                direction: event.direction,
                amount: split.health,
            });
        }
        if let Some((slots, pos, vel)) = death_drops {
            for (index, slot) in slots.into_iter().enumerate() {
                let kick = Vec2 {
                    x: if index == 0 { -60.0 } else { 60.0 },
                    y: -120.0,
                };
                self.objects.push(dropped_weapon(
                    slot.kind,
                    slot.ammo,
                    pos,
                    Vec2 {
                        x: vel.x + kick.x,
                        y: vel.y + kick.y,
                    },
                ));
            }
        }
        if let Some((kind, pos, vel)) = dropped_grenade {
            let grenade = *self.weapons.get(kind);
            self.projectiles.push(Projectile {
                id: self.next_projectile,
                owner: event.target,
                pos,
                // It is dropped, not thrown: it keeps the body's momentum and nothing more.
                vel,
                ttl: grenade.timeout(),
                damage: 0,
                explosive: true,
                kind: ProjectileKind::FragGrenade,
                splash_radius: grenade.explosion_radius(),
                weapon: Some(kind),
                origin: pos,
                last_impact: pos,
            });
            self.next_projectile = self.next_projectile.wrapping_add(1);
        }
        if !died {
            return;
        }

        let killer = event.attacker.filter(|id| *id != event.target);
        let teamkill = killer.is_some_and(|id| {
            team_mode && target_team != 0 && teams.get(&id) == Some(&target_team)
        });
        let suicide = killer.is_none() && event.attacker.is_some();
        let assisting = assists(
            &attackers,
            killer,
            event.target,
            tick,
            config.attribution_ticks,
        );
        let headshot = !teamkill && killer.is_some() && event.region == BodyRegion::Head;

        // Every point in the match is booked here and nowhere else.
        let rules = self.rules;
        let team_of = |id: u32| teams.get(&id).copied().unwrap_or(0);
        let score_event = if let Some(id) = killer {
            Some(if teamkill {
                ScoreEvent::TeamKill {
                    killer: id,
                    victim: event.target,
                }
            } else {
                ScoreEvent::Kill {
                    killer: id,
                    victim: event.target,
                }
            })
        } else if suicide {
            Some(ScoreEvent::Suicide {
                player: event.target,
            })
        } else {
            None
        };
        if let Some(score_event) = score_event {
            // Pointmatch pays extra while holding the flag; Rambomatch pays only the bow holder.
            if let ScoreEvent::Kill { killer, victim } = score_event {
                let policy = modes::objective::ObjectiveRules::for_mode(rules.kind);
                let points = policy.kill_points(self.objectives.is_carrying(killer));
                if points == 0 {
                    self.ledger.note_death(victim);
                } else {
                    self.ledger.record(score_event, &rules, team_of);
                    if points > 1 {
                        self.ledger.record(
                            ScoreEvent::Objective {
                                player: killer,
                                team: team_of(killer),
                                points: points - 1,
                            },
                            &rules,
                            team_of,
                        );
                    }
                }
            } else {
                self.ledger.record(score_event, &rules, team_of);
            }
        } else {
            // An environmental death still counts as a death, with nobody to credit.
            self.ledger.ensure(event.target);
        }

        let mut multi = 0;
        if let Some(id) = killer.filter(|_| !teamkill) {
            if let Some(attacker) = self.players.get_mut(&id) {
                if headshot {
                    attacker.headshots += 1;
                }
                multi = attacker.multi_kill.record(tick, config.multi_kill_ticks);
            }
        }
        for assist in &assisting {
            if let Some(helper) = self.players.get_mut(assist) {
                helper.assists += 1;
            }
        }

        let cause = if teamkill {
            DeathCause::TeamKill {
                by: killer.unwrap_or(event.target),
            }
        } else if let Some(id) = killer {
            DeathCause::Killed {
                by: id,
                cause: event.cause,
                region: event.region,
            }
        } else if suicide {
            DeathCause::Suicide(event.cause)
        } else {
            DeathCause::Environment(event.cause)
        };
        if let Some(target) = self.players.get_mut(&event.target) {
            target.last_death = Some(cause);
        }

        self.ragdolls.push(Ragdoll::spawn(
            event.target,
            target_team,
            position,
            velocity,
            event.direction,
            respawn_config.corpse_ticks,
            gibbed,
        ));
        if gibbed {
            self.events.push(Event::Gibs {
                target: event.target,
                position,
                velocity,
            });
        }
        self.events.push(Event::Kill {
            killer: killer.unwrap_or(event.target),
            target: event.target,
        });
        self.events.push(Event::KillFeed(KillFeedEntry {
            killer,
            target: event.target,
            cause: event.cause,
            region: event.region,
            headshot,
            teamkill,
            suicide: killer.is_none(),
            multi,
            assists: assisting,
        }));
        // Statistics are fed by the same events as the score, so the two can never disagree about
        // what happened.
        if let Some(attacker) = event.attacker.filter(|id| *id != event.target) {
            if let Some(weapon) = weapon_of(event.cause, self.players.get(&attacker)) {
                self.stats.record(StatEvent::Hit {
                    player: attacker,
                    weapon,
                    headshot: event.region == BodyRegion::Head,
                });
                if died {
                    self.stats.record(StatEvent::KilledWith {
                        killer: attacker,
                        victim: event.target,
                        weapon,
                    });
                }
            }
        }
        if died {
            self.stats.record(StatEvent::DiedTo {
                victim: event.target,
                cause: event.cause,
            });
        }

        // apply_damage is an authoritative entry point in its own right, so the counters the HUD
        // reads are brought back in line here and not only at the end of a tick.
        self.project_scores();
        if rules.modifiers.advance {
            self.advance_progress(killer, event.target);
        }
    }

    /// Moves a player up or down the Advance ladder after a kill.
    ///
    /// Earning a weapon and losing one are the same rule seen from either end: every configured
    /// number of kills grants one, and every configured number of deaths takes one back.
    fn advance_progress(&mut self, killer: Option<u32>, victim: u32) {
        let config = self.advance;
        if let Some(id) = killer.filter(|id| *id != victim) {
            let kills = self.ledger.player(id).kills;
            if modes::modifiers::advance::kill_earns_unlock(&config, kills) {
                let mut unlocked = self
                    .players
                    .get(&id)
                    .map_or(Unlocked::starting(), |p| p.unlocked);
                if modes::modifiers::advance::unlock_one(&mut unlocked, &mut self.rng).is_some() {
                    if let Some(player) = self.players.get_mut(&id) {
                        player.unlocked = unlocked;
                    }
                }
            }
        }
        let deaths = self.ledger.player(victim).deaths;
        if modes::modifiers::advance::death_costs_unlock(&config, deaths) {
            let mut unlocked = self
                .players
                .get(&victim)
                .map_or(Unlocked::starting(), |p| p.unlocked);
            if modes::modifiers::advance::revoke_one(&mut unlocked, &mut self.rng).is_some() {
                if let Some(player) = self.players.get_mut(&victim) {
                    player.unlocked = unlocked;
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
    weapons: &WeaponTable,
    friendly_fire: bool,
    hits: &mut Vec<PendingHit>,
) {
    for target in players.values() {
        if target.hp <= 0 {
            continue;
        }
        let owner_team = teams.get(&bullet.owner).copied().unwrap_or(0);
        let same_team = owner_team != 0 && owner_team == target.team;
        let allowed = if let Some(kind) = bullet.weapon {
            can_damage(
                weapons.get(kind),
                bullet.owner,
                target.id,
                same_team,
                friendly_fire,
                true,
            )
        } else {
            target.id == bullet.owner || owner_team == 0 || owner_team != target.team
        };
        if !allowed {
            continue;
        }
        let dx = target.pos.x - bullet.pos.x;
        let dy = target.pos.y - bullet.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();
        if distance >= bullet.splash_radius {
            continue;
        }
        let scale = 1.0 - distance / bullet.splash_radius;
        let normal = if distance > f32::EPSILON {
            Vec2 {
                x: dx / distance,
                y: dy / distance,
            }
        } else {
            Vec2 { x: 0.0, y: -1.0 }
        };
        let region = BodyRegion::Chest;
        let (damage, impulse, pre_scaled) = if let Some(kind) = bullet.weapon {
            let def = weapons.get(kind);
            (
                explosion_damage(def, distance, region).round() as i32,
                Impulse {
                    velocity: explosion_impulse(def, normal, distance),
                    source: ImpulseSource::Explosion,
                },
                true,
            )
        } else {
            (
                (scale * bullet.damage as f32).ceil() as i32,
                Impulse {
                    velocity: Vec2 {
                        x: normal.x * 90.0 * scale,
                        y: normal.y * 90.0 * scale,
                    },
                    source: ImpulseSource::Explosion,
                },
                false,
            )
        };
        hits.push(PendingHit {
            killer: bullet.owner,
            target: target.id,
            damage,
            impulse,
            region,
            cause: DamageCause::Explosion,
            pre_scaled,
            bink_weapon: bullet.weapon,
        });
    }
}

fn shooter_pose(player: &Player, input: &Input) -> ShooterPose {
    ShooterPose {
        state: player.state,
        grounded: player.grounded,
        moving: input.left || input.right || player.vel.x.abs() > 20.0,
        jetting: input.jet && player.fuel > 0.01,
        bink: player.bink,
    }
}

fn projectile_kind(def: &WeaponDef) -> ProjectileKind {
    if def.kind == WeaponKind::ThrownKnife {
        return ProjectileKind::ThrownKnife;
    }
    match def.bullet_style {
        BulletStyle::Shotgun => ProjectileKind::ShotgunPellet,
        BulletStyle::M79Grenade => ProjectileKind::M79Grenade,
        BulletStyle::Law => ProjectileKind::LawRocket,
        BulletStyle::FragGrenade | BulletStyle::ClusterGrenade | BulletStyle::Cluster => {
            ProjectileKind::FragGrenade
        }
        style if style.is_melee() => ProjectileKind::Melee,
        _ => ProjectileKind::Bullet,
    }
}

fn thrown_knife(bullet: &Projectile) -> bool {
    bullet.kind == ProjectileKind::ThrownKnife || bullet.weapon == Some(WeaponKind::ThrownKnife)
}

fn held_kind(player: &Player) -> WeaponKind {
    let kind = player
        .inventory
        .active()
        .map(|slot| slot.kind)
        .unwrap_or(WeaponKind::Punch);
    if kind == WeaponKind::RamboBow && player.bonus.is(BonusEffect::FlameGod) {
        return WeaponKind::FlamedArrows;
    }
    kind
}

fn sync_hands(player: &mut Player) {
    if let Some(slot) = player.inventory.active() {
        if let Some(index) = slot.kind.slot() {
            player.weapon = index;
        }
        player.ammo = slot.ammo;
    } else {
        player.ammo = 0;
    }
}

fn writeback_ammo(player: &mut Player) {
    if let Some(slot) = player.inventory.active_mut() {
        slot.ammo = player.ammo;
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

/// How hard a blast throws a loose flag, in world units per second at the centre of the blast.
const EXPLOSION_FLAG_PUSH: f32 = 260.0;

/// How hard gravity pulls a projectile that is subject to it, in world units per second squared.
///
/// Matched to the character gravity the movement config uses, so a grenade and a body fall at the
/// same rate and a thrown arc lands where a player expects it to.
const PROJECTILE_GRAVITY: f32 = 0.12 * 950.0;

/// Lifts a projectile clear of the surface it just struck so the next tick does not re-hit it.
fn nudge(pos: Vec2, normal: Vec2) -> Vec2 {
    Vec2 {
        x: pos.x + normal.x * 0.5,
        y: pos.y + normal.y * 0.5,
    }
}

/// The definition a projectile was fired from, falling back to a plain bullet for older snapshots
/// that predate weapon-tagged projectiles.
fn armed_def(weapons: &WeaponTable, bullet: &Projectile) -> WeaponDef {
    *weapons.get(bullet.weapon.unwrap_or(WeaponKind::Ak74))
}

/// The weapon a damage event should be credited to, for the statistics.
///
/// Only a wound from something a player was holding counts: a fall or a deadly polygon has no
/// weapon behind it, and crediting one would make a scoreboard lie.
fn weapon_of(cause: DamageCause, attacker: Option<&Player>) -> Option<WeaponKind> {
    match cause {
        DamageCause::Bullet | DamageCause::Pellet | DamageCause::Melee | DamageCause::Explosion => {
            attacker.map(|player| WeaponKind::from_slot(player.weapon))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn soldat_primary_weapon_stats_match_normal_mode() {
        let table = WeaponTable::normal();
        assert_eq!(SELECTABLE_WEAPONS[..10].len(), 10);
        assert_eq!(table.for_slot(0).kind.display_name(), "Desert Eagles");
        assert_eq!(
            (
                table.for_slot(0).fire_interval,
                table.for_slot(0).ammo,
                table.for_slot(0).reload_time
            ),
            (24, 7, 87)
        );
        assert_eq!(
            (
                table.for_slot(1).fire_interval,
                table.for_slot(1).ammo,
                table.for_slot(1).reload_time
            ),
            (6, 30, 105)
        );
        assert_eq!(
            (table.for_slot(4).bullet_style, table.for_slot(4).pellets()),
            (BulletStyle::Shotgun, 6)
        );
        assert_eq!(
            (
                table.for_slot(6).bullet_style.is_explosive(),
                table.for_slot(6).ammo
            ),
            (true, 1)
        );
        assert_eq!(
            (
                table.for_slot(7).start_up_time,
                table.for_slot(7).fire_interval
            ),
            (19, 225)
        );
    }
    #[test]
    fn soldat_secondary_weapon_stats_match_normal_mode() {
        let table = WeaponTable::normal();
        assert_eq!(SELECTABLE_WEAPONS.len(), 14);
        assert_eq!(
            (
                table.for_slot(10).kind.display_name(),
                table.for_slot(10).ammo,
                table.for_slot(10).reload_time
            ),
            ("USSOCOM", 14, 60)
        );
        assert_eq!(
            (
                table.for_slot(11).kind.display_name(),
                table.for_slot(11).bullet_style.is_melee()
            ),
            ("Combat Knife", true)
        );
        assert_eq!(
            (
                table.for_slot(12).kind.display_name(),
                table.for_slot(12).fire_interval
            ),
            ("Chainsaw", 2)
        );
        assert_eq!(
            (
                table.for_slot(13).kind.display_name(),
                table.for_slot(13).start_up_time
            ),
            ("LAW", 13)
        );
    }
    #[test]
    fn frag_grenade_consumes_ammo_and_uses_source_radius() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Thrower".into());
        let grenades = w.players[&1].grenades;
        let aim = Vec2 { x: 500.0, y: 430.0 };
        let cooking = BTreeMap::from([(
            1,
            Input {
                throw_grenade: true,
                aim,
                ..Default::default()
            },
        )]);
        let released = BTreeMap::from([(
            1,
            Input {
                aim,
                ..Default::default()
            },
        )]);

        // A grenade is cooked while the key is held and leaves the hand when it is let go.
        w.step(&cooking);
        assert_eq!(w.players[&1].grenades, grenades, "still in hand");
        w.step(&released);
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
            weapon: None,
            origin: Vec2::default(),
            last_impact: Vec2::default(),
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
            weapon: None,
            origin: Vec2::default(),
            last_impact: Vec2::default(),
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
        // Holding the key cooks one grenade and lets it go at full strength. It must not keep
        // throwing the rest of the pouch for as long as the key stays down.
        for _ in 0..100 {
            w.step(&input);
        }
        assert_eq!(w.players[&1].grenades, 1);
    }
    #[test]
    fn melee_projectile_has_short_range() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "Knife".into());
        w.equip(1, WeaponKind::CombatKnife);
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
        let eagles = *WeaponTable::normal().for_slot(0);
        for _ in 0..eagles.reload_time {
            w.step(&BTreeMap::new());
        }
        assert_eq!(w.players[&1].ammo, u16::from(eagles.ammo));
    }
    #[test]
    fn explicit_reload_input_refills_a_partial_magazine_and_ignores_a_full_one() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        let aim = Vec2 {
            x: 1000.0,
            y: 430.0,
        };
        w.players.get_mut(&1).unwrap().cooldown = 0;
        w.step(&BTreeMap::from([(
            1,
            Input {
                fire: true,
                aim,
                ..Default::default()
            },
        )]));
        let eagles = *WeaponTable::normal().for_slot(0);
        assert_eq!(w.players[&1].ammo, u16::from(eagles.ammo) - 1);

        let reload = Input {
            reload: true,
            aim,
            ..Default::default()
        };
        w.step(&BTreeMap::from([(1, reload)]));
        assert_eq!(w.players[&1].reload_timer, eagles.reload_time);
        for _ in 0..eagles.reload_time {
            w.step(&BTreeMap::new());
        }
        assert_eq!(w.players[&1].ammo, u16::from(eagles.ammo));

        w.step(&BTreeMap::from([(1, reload)]));
        assert_eq!(w.players[&1].reload_timer, 0);
    }
    #[test]
    fn a_held_reload_input_starts_only_one_reload() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        let aim = Vec2 {
            x: 1000.0,
            y: 430.0,
        };
        w.players.get_mut(&1).unwrap().cooldown = 0;
        w.step(&BTreeMap::from([(
            1,
            Input {
                fire: true,
                aim,
                ..Default::default()
            },
        )]));
        let reload = Input {
            reload: true,
            aim,
            ..Default::default()
        };
        w.step(&BTreeMap::from([(1, reload)]));
        let started = w.players[&1].reload_timer;
        w.step(&BTreeMap::from([(1, reload)]));
        assert_eq!(w.players[&1].reload_timer, started - 1);
    }
    #[test]
    fn spas_fires_multiple_pellets_for_one_shell() {
        let mut w = World::new("deathmatch");
        w.add_player(1, "A".into());
        w.equip(1, WeaponKind::Spas12);
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
        let spas = *WeaponTable::normal().for_slot(4);
        assert_eq!(w.projectiles.len(), spas.pellets() as usize);
        assert_eq!(w.players[&1].ammo, u16::from(spas.ammo) - 1);
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
            weapon: None,
            origin: Vec2::default(),
            last_impact: Vec2::default(),
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
                weapon: 10,
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
        assert_eq!(w.players[&1].reload_timer, 0);
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
            weapon: None,
            origin: Vec2::default(),
            last_impact: Vec2::default(),
        });
        w.step(&BTreeMap::new());
        assert_eq!(w.players[&1].hp, 0);
        assert_eq!(w.players[&1].kills, 0);
        assert_eq!(w.players[&1].suicides, 1);
        assert_eq!(w.ragdolls.len(), 1);
        for _ in 0..120 {
            w.step(&BTreeMap::new());
        }
        assert!(
            w.ragdolls[0]
                .segments
                .iter()
                .any(|segment| segment.grounded),
            "a corpse settles on the terrain"
        );
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
        assert_eq!(first, WeaponTable::normal().for_slot(0).pellets() as usize);
        assert_eq!(w.projectiles.len(), first);
    }
}
