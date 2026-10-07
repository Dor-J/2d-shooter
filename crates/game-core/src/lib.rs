#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub mod anim;
pub mod bots;
mod character;
pub mod chat;
mod collision;
pub mod commands;
mod events;
mod fixtures;
mod input;
mod map;
mod math;
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
    MultiKill, Player, Ragdoll, RagdollSegment, RespawnConfig, RollSource, MAX_ATTRIBUTIONS,
};
pub use chat::{censor, may_see as may_see_chat, parse_chat, ChatLine, ChatScope, Flood};
pub use collision::{
    resolve_material_velocity, Aabb, BodyPart, BodyRegion, BodyShape, CollisionMask,
    CollisionPolygon, CollisionWorld, Contact, ContactManifold, DynamicBody, DynamicBodyKind,
    MaterialResponse, PolygonKind, RayHit, ShapeHit, SweepHit,
};
pub use commands::{parse_player_command, CommandError, PlayerCommand};
pub use events::Event;
pub use fixtures::{replay_fixture_json, Fixture, FrameInput, SimRng, WorldDigest};
pub use input::Input;
use map::map_spawn;
pub use map::{MapSpawn, MapValidationError, SpawnKind, ValidatedMap};
pub use math::Vec2;
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
    throw_velocity, BulletStyle, FireRefusal, ImpactResponse, Inventory, NoCollision, Projectile,
    ProjectileKind, ShooterPose, StationaryGun, SurfaceResponse, WeaponConfigError, WeaponDef,
    WeaponKind, WeaponLimits, WeaponSlot, WeaponTable, ALL_WEAPONS, ARROW_RESIST_TICKS,
    CLUSTER_SUBMUNITIONS, CONFIGURED_WEAPONS, FIRST_DEGRADE_DISTANCE, GRENADE_SURFACE_COEF,
    MAX_INACCURACY, MAX_THROW_CHARGE, MELEE_RADIUS, OBJECT_PUSH_MULTIPLIER, PICKUP_RADIUS,
    RICOCHET_MIN_TRAVEL, SECOND_DEGRADE_DISTANCE, SELECTABLE_WEAPONS, SWITCH_DELAY_TICKS,
    THROW_LAST_FRAME,
};
use weapons::{held_kind, shooter_pose, sync_hands, writeback_ammo};

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
        map::place_objectives(self);
    }

    pub fn with_map(mode: &str, map: &ValidatedMap) -> Self {
        map::with_map(mode, map)
    }

    /// Bolts M2s and remembers kit/grenade spots from the map's special spawnpoints.
    fn place_specials(&mut self) {
        map::place_specials(self);
    }

    /// The yellow flag *is* the bow: whoever holds it has the weapon, everyone else does not.
    fn sync_rambo_bow(&mut self) {
        modes::sync_rambo_bow(self);
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
        objects::step_bonuses(self);
    }

    /// Gives one kit to one player, and says whether it was any use to them.
    ///
    /// A kit that would do nothing — a medkit for somebody on full health, a second effect while
    /// one is running — is left on the ground for whoever needs it.
    fn apply_kit(&mut self, id: u32, kind: KitKind) -> bool {
        objects::apply_kit(self, id, kind)
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
        bots::bot_inputs(self)
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
        modes::spectator::followable_players(self)
    }

    /// Points a spectator at somebody, or lets them fly the camera.
    ///
    /// A command from somebody who is not spectating is ignored rather than quietly turning them
    /// into one; joining the spectators is a team change, which has its own path.
    pub fn spectate(&mut self, id: u32, command: SpectateCommand) {
        modes::spectator::spectate(self, id, command);
    }

    /// Keeps every spectator watching somebody who is still in the match.
    fn retarget_spectators(&mut self) {
        modes::spectator::retarget_spectators(self);
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
                character::tick::corpse(
                    player,
                    inputs,
                    &rules_snapshot,
                    &respawn_config,
                    &opponents,
                    &map_spawns,
                    &mode,
                    &self.weapons,
                    &mut self.events,
                );
                continue;
            }
            player.spawn_protection = player.spawn_protection.saturating_sub(1);
            character::tick::bleed(player, &mut pending);
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
            character::tick::movement(
                player,
                input,
                &self.collision,
                &self.movement,
                &damage_config,
                &mut pending,
            );
            weapons::trigger::trigger(
                player,
                input,
                previous,
                weapon,
                &mut self.events,
                &mut self.guns,
                &mut self.next_projectile,
                &mut self.objects,
                &mut self.projectiles,
                &mut self.rng,
                &mut self.stats,
                &self.weapons,
            );
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
        let mut newborns = self.projectiles.split_off(lived_projectiles);
        weapons::fly(self, &mut newborns);
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
        modes::round::restart_match(self);
    }

    /// Copies the ledger onto the per-player counters and the team scores.
    ///
    /// Those are what the HUD, the scoreboard, and the wire format read. Writing them from the
    /// ledger rather than alongside it is what keeps one authority for every point in the match.
    fn project_scores(&mut self) {
        modes::score::project_scores(self);
    }

    /// Living players this one would rather not spawn next to: the other team in team modes and
    /// everybody else otherwise.
    fn opponent_positions(&self) -> BTreeMap<u32, Vec<Vec2>> {
        character::respawn::opponent_positions(self)
    }

    /// Carries out a typed player command. The parser already decided what was asked; this decides
    /// whether the world will do it.
    pub fn apply_player_command(&mut self, player: u32, command: PlayerCommand) -> bool {
        commands::apply_player_command(self, player, command)
    }

    /// The single authoritative damage path. Bullets, explosions, melee, falls, bleeding, and
    /// deadly polygons all arrive here, so armor, attribution, death, and the kill feed can never
    /// disagree about what happened.
    pub fn apply_damage(&mut self, event: DamageEvent) {
        character::apply_damage(self, event);
    }

    /// Moves a player up or down the Advance ladder after a kill.
    ///
    /// Earning a weapon and losing one are the same rule seen from either end: every configured
    /// number of kills grants one, and every configured number of deaths takes one back.
    fn advance_progress(&mut self, killer: Option<u32>, victim: u32) {
        modes::modifiers::advance::advance_progress(self, killer, victim);
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm;

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
