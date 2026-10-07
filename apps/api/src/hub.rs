use super::maps::{MapService, TransferEvent};
use super::room::{
    advance_map, default_rotation, fill_with_bots, map_suits_mode, snapshot_for, Room, RoomRequest,
    ROOM_CAPACITY,
};
use super::{admin, audit, bans, config, lobby, security, storage};
use game_core::{
    MatchEvent, ModeKind, ModeRules, Replay, ReplayHeader, ValidatedMap, WeaponTable, World,
    TICK_RATE,
};
use map_editor::{original_default_map, DEFAULT_MAPS};
use protocol::{RoomInfo, ServerMessage, WireModifiers, VERSION};
use std::{
    collections::{BTreeMap, HashMap},
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, Mutex};

pub(crate) struct Guest {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) token: String,
    pub(crate) sender: Option<Sender>,
    pub(crate) room: Option<u32>,
    pub(crate) disconnected: Option<Instant>,
    pub(crate) last_chat: Instant,
    pub(crate) last_input: u32,
    pub(crate) muted: std::collections::HashSet<u32>,
    pub(crate) admin: bool,
    pub(crate) flood: game_core::Flood,
    pub(crate) ip: IpAddr,
    pub(crate) last_ping: Instant,
}

pub(crate) struct Hub {
    pub(crate) guests: HashMap<u32, Guest>,
    pub(crate) tokens: HashMap<String, u32>,
    pub(crate) rooms: BTreeMap<u32, Room>,
    pub(crate) next_guest: u32,
    next_room: u32,
    pub(crate) maps: MapService,
    pub(crate) map_transfers: BTreeMap<u64, u32>,
    pub(crate) bans: bans::BanList,
    pub(crate) audit: audit::AuditLog,
    pub(crate) config: config::ServerConfig,
    pub(crate) limits: security::RateLimits,
    pub(crate) store: storage::Store,
    pub(crate) drain: bool,
    pub(crate) tick_ms: u64,
    pub(crate) errors: u64,
    owner: String,
}

impl Hub {
    pub(crate) fn new() -> Self {
        let maps = std::env::var_os("MAP_PACKAGES_DIR")
            .map(std::path::PathBuf::from)
            .map(|path| MapService::load_directory(&path).expect("load signed map packages"))
            .unwrap_or_default();
        let default_map = ValidatedMap::try_from(original_default_map(DEFAULT_MAPS[0]))
            .expect("built-in map must validate");
        let store = std::env::var_os("DATA_DIR")
            .map(|_| storage::load_file(&storage::store_path()))
            .unwrap_or_default();
        let mut config = config::ServerConfig::default();
        store.config.apply(&mut config);
        let world = World::with_map("deathmatch", &default_map);
        let mut hub = Self {
            guests: HashMap::new(),
            tokens: HashMap::new(),
            rooms: BTreeMap::new(),
            next_guest: 1,
            next_room: 2,
            maps,
            map_transfers: BTreeMap::new(),
            bans: if store.bans.is_empty() {
                bans::BanList::default()
            } else {
                bans::BanList::restore(&store.bans)
            },
            audit: audit::AuditLog::default(),
            config,
            limits: security::RateLimits::default(),
            store: store.clone(),
            drain: store.drain,
            tick_ms: 0,
            errors: 0,
            owner: std::env::var("SERVER_ID").unwrap_or_else(|_| "api-1".into()),
        };
        hub.rooms.insert(
            1,
            Room {
                id: 1,
                name: "Arena".into(),
                mode: "deathmatch".into(),
                code: "ARENA1".into(),
                public: true,
                replay: Replay::new(ReplayHeader::new(VERSION, "Aero", &world, 1)),
                world,
                inputs: BTreeMap::new(),
                pending_events: Vec::new(),
                pending_match_events: Vec::new(),
                rotation: default_rotation("Aero", ModeKind::Deathmatch),
                ruleset: None,
                map: "Aero".into(),
                password: String::new(),
                paused: false,
                capacity: ROOM_CAPACITY,
                required_mod: String::new(),
                votes: std::collections::HashMap::new(),
            },
        );
        if let Ok(name) = std::env::var("SERVER_NAME") {
            let _ = hub.config.set_name(name);
        }
        let _ = hub.store.claim(1, hub.owner.clone(), 1);
        hub
    }
    fn persist(&mut self) {
        self.store.bans = self.bans.persist();
        self.store.config = storage::ConfigBlob::from(&self.config);
        self.store.drain = self.drain;
        if std::env::var_os("DATA_DIR").is_some() {
            let _ = storage::save_file(&storage::store_path(), &self.store);
        }
    }
    /// Flushes bans/config. The dying process refuses joins; an operator `/DRAIN` survives restart.
    pub(crate) fn persist_for_shutdown(&mut self) {
        let keep_drain = self.drain;
        self.drain = true;
        self.persist();
        self.store.drain = keep_drain;
        if std::env::var_os("DATA_DIR").is_some() {
            let _ = storage::save_file(&storage::store_path(), &self.store);
        }
    }
    pub(crate) fn send(&self, id: u32, msg: &ServerMessage) {
        if let Some(tx) = self.guests.get(&id).and_then(|g| g.sender.as_ref()) {
            if let Ok(raw) = serde_json::to_string(msg) {
                let _ = tx.send(raw);
            }
        }
    }
    pub(crate) fn room_list(&self) -> Vec<RoomInfo> {
        let rooms: Vec<RoomInfo> = self
            .rooms
            .values()
            .filter(|r| r.public)
            .map(|r| RoomInfo {
                id: r.id,
                name: r.name.clone(),
                mode: r.mode.clone(),
                players: r.world.players.len(),
                capacity: r.capacity,
                map: r.map.clone(),
                weapon_mod: r.world.weapons.name().to_string(),
                weapon_hash: r.world.weapons.canonical_hash(),
                modifiers: WireModifiers {
                    realistic: r.world.rules.modifiers.realistic,
                    survival: r.world.rules.modifiers.survival,
                    advance: r.world.rules.modifiers.advance,
                },
                ruleset: r.ruleset.clone(),
                password: !r.password.is_empty(),
                version: VERSION,
                required_mod: (!r.required_mod.is_empty()).then(|| r.required_mod.clone()),
                region: Some(std::env::var("SERVER_REGION").unwrap_or_else(|_| "invite".into())),
            })
            .collect();
        lobby::sort_rooms(
            lobby::filter_rooms(&rooms, &lobby::LobbyQuery::default()),
            lobby::SortKey::Players,
        )
    }
    pub(crate) fn requires_mod(
        &self,
        room_id: u32,
        offered: Option<&str>,
    ) -> Result<(), &'static str> {
        let required = &self.rooms.get(&room_id).ok_or("room_missing")?.required_mod;
        if required.is_empty() || required == offered.unwrap_or("") {
            Ok(())
        } else {
            Err("mod_required")
        }
    }
    pub(crate) fn create_room(&mut self, request: RoomRequest) -> Result<u32, &'static str> {
        if self.drain {
            return Err("draining");
        }
        let RoomRequest {
            name,
            mode,
            public,
            requested_map,
            weapon_mod,
            modifiers,
            ruleset,
            bonuses,
            bots,
            bot_difficulty,
            password: request_password,
            required_mod,
        } = request;
        // A community ruleset brings its own base mode and modifiers, so it is resolved first and
        // then treated exactly like any other room.
        let scripted = match ruleset.as_deref() {
            Some(id) => {
                let found =
                    game_core::modes::scripted::presets::by_id(id).ok_or("unknown_ruleset")?;
                found.validate().map_err(|_| "invalid_ruleset")?;
                Some(found)
            }
            None => None,
        };
        let mode = scripted
            .as_ref()
            .map_or(mode, |rules| rules.base.id().to_string());
        let kind = ModeKind::from_id(&mode);
        if !kind.is_implemented() {
            // A mode whose objective nobody can pick up yet is refused up front rather than
            // opening a room nobody can actually play in.
            return Err("mode_unavailable");
        }
        let entry = requested_map
            .as_deref()
            .and_then(|name| DEFAULT_MAPS.iter().find(|entry| entry.name == name))
            .or_else(|| {
                DEFAULT_MAPS
                    .iter()
                    .find(|entry| map_suits_mode(entry.mode, kind))
            })
            .ok_or("map_missing")?;
        if !map_suits_mode(entry.mode, kind) {
            return Err("map_mode_mismatch");
        }
        let map =
            ValidatedMap::try_from(original_default_map(*entry)).map_err(|_| "invalid_map")?;
        let room_id = self.next_room;
        self.next_room += 1;
        let code = loop {
            let candidate = uuid::Uuid::new_v4().simple().to_string()[..6].to_uppercase();
            if self.rooms.values().all(|room| room.code != candidate) {
                break candidate;
            }
        };
        let mut world = World::with_map(&mode, &map);
        world.rules = match &scripted {
            Some(rules) => rules.to_mode_rules(),
            None => {
                let mut base = ModeRules::new(kind);
                if let Some(wanted) = modifiers {
                    base.modifiers = game_core::ModifierSet {
                        realistic: wanted.realistic,
                        survival: wanted.survival,
                        advance: wanted.advance,
                    };
                }
                base
            }
        };
        // Advance starts everybody at the bottom of the ladder rather than with the armoury.
        // A server may switch the kits off entirely; on is the default.
        if bonuses == Some(false) {
            world.bonuses = game_core::BonusConfig::none();
        }
        world.place_objectives();
        world.weapons = match weapon_mod.as_deref() {
            Some("realistic") | Some("Realistic mod") => WeaponTable::realistic(),
            // A Realistic room uses the Realistic table by default; that is what the modifier is.
            _ if world.rules.modifiers.realistic => WeaponTable::realistic(),
            _ => WeaponTable::normal(),
        };
        if world.rules.modifiers.realistic {
            world.movement = game_core::MovementConfig::realistic();
        }
        // Bots fill the room before anybody joins, so a player never walks into an empty match.
        if let Some(wanted) = bots.filter(|count| *count > 0) {
            let difficulty =
                game_core::Difficulty::from_id(bot_difficulty.as_deref().unwrap_or("normal"));
            fill_with_bots(&mut world, usize::from(wanted), difficulty);
        }
        let map_name = entry.name;
        self.rooms.insert(
            room_id,
            Room {
                id: room_id,
                name,
                mode: mode.clone(),
                code,
                public,
                replay: Replay::new(ReplayHeader::new(
                    VERSION,
                    map_name,
                    &world,
                    u64::from(room_id),
                )),
                world,
                inputs: BTreeMap::new(),
                pending_events: Vec::new(),
                pending_match_events: Vec::new(),
                rotation: default_rotation(map_name, kind),
                ruleset: scripted.as_ref().map(|rules| rules.name.clone()),
                map: map_name.into(),
                password: request_password,
                paused: false,
                capacity: ROOM_CAPACITY,
                required_mod,
                votes: std::collections::HashMap::new(),
            },
        );
        let fence = u64::from(room_id);
        let _ = self.store.claim(room_id, self.owner.clone(), fence);
        Ok(room_id)
    }
    pub(crate) fn room_id_by_code(&self, code: &str) -> Option<u32> {
        let code = code.trim().to_uppercase();
        self.rooms
            .values()
            .find(|room| room.code == code)
            .map(|room| room.id)
    }
    pub(crate) fn leave(&mut self, id: u32) {
        let name = self.guests.get(&id).map(|guest| guest.name.clone());
        if let Some(room_id) = self.guests.get_mut(&id).and_then(|g| g.room.take()) {
            if let Some(room) = self.rooms.get_mut(&room_id) {
                room.world.players.remove(&id);
                room.inputs.remove(&id);
            }
            if let Some(name) = name {
                announce(self, room_id, format!("{name} left"));
            }
        }
    }
    pub(crate) fn join(&mut self, id: u32, room_id: u32) -> Result<(), &'static str> {
        if self.drain {
            return Err("draining");
        }
        let capacity = self.rooms.get(&room_id).ok_or("room_missing")?.capacity;
        if self
            .rooms
            .get(&room_id)
            .ok_or("room_missing")?
            .world
            .players
            .len()
            >= capacity
        {
            return Err("room_full");
        }
        self.leave(id);
        let name = self.guests[&id].name.clone();
        let room = self.rooms.get_mut(&room_id).unwrap();
        room.world.add_player(id, name.clone());
        let joined_name = room.name.clone();
        let joined_code = room.code.clone();
        let joined_public = room.public;
        self.guests.get_mut(&id).unwrap().room = Some(room_id);
        self.send(
            id,
            &ServerMessage::Joined {
                room: room_id,
                player: id,
                name: joined_name,
                code: joined_code,
                public: joined_public,
            },
        );
        announce(self, room_id, format!("{name} joined"));
        Ok(())
    }
}

pub(crate) type Sender = mpsc::UnboundedSender<String>;

pub(crate) type Shared = Arc<Mutex<Hub>>;

pub(crate) fn apply_admin(hub: &mut Hub, actor: u32, command: admin::AdminCommand) {
    match command {
        admin::AdminCommand::Kick { target } => {
            if let Some(id) = resolve_target(hub, &target) {
                hub.send(
                    id,
                    &ServerMessage::Kicked {
                        reason: "kicked".into(),
                    },
                );
                hub.leave(id);
            }
        }
        admin::AdminCommand::Ban { target } => {
            if let Some(id) = resolve_target(hub, &target) {
                if let Some(ip) = hub.guests.get(&id).map(|guest| guest.ip) {
                    hub.bans.ban(ip, None, "ban");
                }
                hub.send(
                    id,
                    &ServerMessage::Kicked {
                        reason: "banned".into(),
                    },
                );
                hub.leave(id);
            }
        }
        admin::AdminCommand::BanIp { ip } => hub.bans.ban(ip, None, "banip"),
        admin::AdminCommand::TempBan { minutes, target } => {
            if let Some(id) = resolve_target(hub, &target) {
                if let Some(ip) = hub.guests.get(&id).map(|guest| guest.ip) {
                    hub.bans.ban(ip, Some(minutes), "tempban");
                }
                hub.send(
                    id,
                    &ServerMessage::Kicked {
                        reason: "banned".into(),
                    },
                );
                hub.leave(id);
            }
            hub.audit
                .record(actor.to_string(), "tempban", minutes.to_string());
        }
        admin::AdminCommand::Unban { ip } => {
            hub.bans.unban(ip);
        }
        admin::AdminCommand::Restart | admin::AdminCommand::NextMap => {
            if let Some(room_id) = hub.guests.get(&actor).and_then(|guest| guest.room) {
                if let Some(room) = hub.rooms.get_mut(&room_id) {
                    room.world.match_state.phase = game_core::MatchPhase::MapTransition;
                }
            }
        }
        admin::AdminCommand::Map { map } => {
            if let Some(room_id) = hub.guests.get(&actor).and_then(|guest| guest.room) {
                if let Some(room) = hub.rooms.get_mut(&room_id) {
                    room.map = map;
                }
            }
        }
        admin::AdminCommand::RespawnTime { seconds } => {
            let _ = hub.config.set_respawn(seconds);
        }
        admin::AdminCommand::Password { value } => {
            if let Some(room_id) = hub.guests.get(&actor).and_then(|guest| guest.room) {
                if let Some(room) = hub.rooms.get_mut(&room_id) {
                    room.password = value.clone();
                }
            }
            let _ = hub.config.set_password(value);
        }
        admin::AdminCommand::MaxPlayers { count } => {
            if let Some(room_id) = hub.guests.get(&actor).and_then(|guest| guest.room) {
                if let Some(room) = hub.rooms.get_mut(&room_id) {
                    room.capacity = usize::from(count);
                }
            }
            let _ = hub.config.set_max_players(count);
        }
        admin::AdminCommand::Adm { target } => {
            if let Some(id) = resolve_target(hub, &target) {
                hub.guests.get_mut(&id).unwrap().admin = true;
                hub.bans.add_admin_id(id);
            }
        }
        admin::AdminCommand::AdmIp { ip } => hub.bans.add_admin(ip),
        admin::AdminCommand::Unadm { ip } => {
            hub.bans.remove_admin(ip);
        }
        admin::AdminCommand::AddBot { name, .. } => {
            if let Some(room_id) = hub.guests.get(&actor).and_then(|guest| guest.room) {
                if let Some(room) = hub.rooms.get_mut(&room_id) {
                    let wanted = room.world.bots.len() + 1;
                    fill_with_bots(&mut room.world, wanted, game_core::Difficulty::Normal);
                    let _ = name;
                }
            }
        }
        admin::AdminCommand::KickLast => {
            if let Some(id) = hub.guests.keys().copied().filter(|&id| id != actor).max() {
                hub.send(
                    id,
                    &ServerMessage::Kicked {
                        reason: "kicked".into(),
                    },
                );
                hub.leave(id);
            }
        }
        admin::AdminCommand::AddMap { map } | admin::AdminCommand::DelMap { map } => {
            hub.audit.record(actor.to_string(), "maplist", map);
        }
        admin::AdminCommand::Drain => {
            hub.drain = true;
            hub.audit.record(actor.to_string(), "drain", "on");
        }
        admin::AdminCommand::Undrain => {
            hub.drain = false;
            hub.audit.record(actor.to_string(), "drain", "off");
        }
        admin::AdminCommand::Vote { map } => {
            if let Some(room_id) = hub.guests.get(&actor).and_then(|guest| guest.room) {
                if let Some(room) = hub.rooms.get_mut(&room_id) {
                    room.votes.insert(actor, map.clone());
                    let needed = room.world.players.len().div_ceil(2).max(1);
                    let tally = room.votes.values().filter(|choice| *choice == &map).count();
                    if tally >= needed {
                        room.map = map;
                        room.world.match_state.phase = game_core::MatchPhase::MapTransition;
                        room.votes.clear();
                    }
                }
            }
        }
    }
    let _ = hub.bans.persist();
    let _ = hub.audit.last();
    hub.persist();
}

fn resolve_target(hub: &Hub, target: &admin::Target) -> Option<u32> {
    match target {
        admin::Target::Id(id) => hub.guests.contains_key(id).then_some(*id),
        admin::Target::Name(name) => hub
            .guests
            .values()
            .find(|guest| guest.name.eq_ignore_ascii_case(name))
            .map(|guest| guest.id),
    }
}

fn announce(hub: &Hub, room_id: u32, text: String) {
    for guest in hub
        .guests
        .values()
        .filter(|guest| guest.room == Some(room_id))
    {
        hub.send(
            guest.id,
            &ServerMessage::Chat {
                player: 0,
                name: "server".into(),
                text: text.clone(),
                scope: "server".into(),
            },
        );
    }
}

pub(crate) async fn tick_loop(hub: Shared) {
    let mut interval = tokio::time::interval(Duration::from_secs_f64(1.0 / TICK_RATE as f64));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        let started = Instant::now();
        let mut h = hub.lock().await;
        for guest in h.guests.values_mut() {
            guest.flood.tick();
        }
        // Acceptance evidence: rust:ping:kick
        if h.config.kick_on_ping {
            let limit = Duration::from_millis(u64::from(h.config.max_ping_ms.max(1)) * 3);
            let late: Vec<u32> = h
                .guests
                .iter()
                .filter(|(_, g)| g.disconnected.is_none() && g.last_ping.elapsed() > limit)
                .map(|(&id, _)| id)
                .collect();
            for id in late {
                h.send(
                    id,
                    &ServerMessage::Kicked {
                        reason: "ping".into(),
                    },
                );
                h.leave(id);
            }
        }
        let expired: Vec<u32> = h
            .guests
            .iter()
            .filter_map(|(&id, g)| {
                g.disconnected
                    .filter(|t| t.elapsed() >= security::resume_ttl())
                    .map(|_| id)
            })
            .collect();
        for id in expired {
            h.leave(id);
            if let Some(g) = h.guests.remove(&id) {
                h.tokens.remove(&g.token);
            }
        }
        let room_ids: Vec<u32> = h.rooms.keys().copied().collect();
        let mut finished = Vec::new();
        for room_id in room_ids {
            let (world, participants) = {
                let room = h.rooms.get_mut(&room_id).unwrap();
                if !room.paused {
                    room.world.step_with_bots(&room.inputs);
                    let _ = room.replay.record(&room.world, &room.inputs);
                }
                room.pending_events
                    .extend(room.world.events.iter().cloned());
                room.pending_match_events
                    .extend(room.world.match_events.iter().copied());
                // A round that has run its course loads the next map in the rotation and starts
                // the whole lifecycle again, which is what makes a server keep going by itself.
                if room
                    .world
                    .match_events
                    .iter()
                    .any(|event| matches!(event, MatchEvent::RoundEnded { .. }))
                {
                    finished.push(format!(
                        "{} {} tick={}",
                        room.map, room.mode, room.world.tick
                    ));
                }
                if room
                    .world
                    .match_events
                    .iter()
                    .any(|event| matches!(event, MatchEvent::NextMap))
                {
                    advance_map(room);
                }
                let mut snapshot = room.world.clone();
                snapshot.events = std::mem::take(&mut room.pending_events);
                snapshot.match_events = std::mem::take(&mut room.pending_match_events);
                let participants = room.world.players.keys().copied().collect::<Vec<_>>();
                if snapshot.tick % 3 != 0 {
                    // Not a snapshot tick: put the events back and wait for the next one.
                    room.pending_events = snapshot.events;
                    (room.world.clone(), participants)
                } else {
                    (snapshot, participants)
                }
            };
            if world.tick % 3 == 0 {
                for id in participants {
                    h.send(
                        id,
                        &ServerMessage::Snapshot {
                            room: room_id,
                            world: Box::new(snapshot_for(&world, id)),
                        },
                    );
                }
            }
        }
        if !finished.is_empty() {
            for line in finished {
                h.store.note_match(&line);
            }
            h.persist();
        }
        let transfers = h
            .map_transfers
            .iter()
            .map(|(&transfer, &owner)| (transfer, owner))
            .collect::<Vec<_>>();
        for (transfer, owner) in transfers {
            match h.maps.next(transfer) {
                Ok(TransferEvent::Chunk {
                    path,
                    offset,
                    total,
                    bytes,
                    ..
                }) => h.send(
                    owner,
                    &ServerMessage::MapDownloadChunk {
                        transfer,
                        path,
                        offset,
                        total,
                        bytes,
                    },
                ),
                Ok(TransferEvent::Complete { .. }) => {
                    h.map_transfers.remove(&transfer);
                    h.send(owner, &ServerMessage::MapDownloadComplete { transfer });
                }
                Ok(TransferEvent::Cancelled { .. }) | Err(_) => {
                    h.map_transfers.remove(&transfer);
                }
            }
        }
        if h.rooms.len() > 1 {
            let empty: Vec<u32> = h
                .rooms
                .iter()
                .filter_map(|(&id, room)| {
                    (id != 1 && room.world.players.is_empty() && room.world.tick > 3600)
                        .then_some(id)
                })
                .collect();
            for id in empty {
                h.store.release(id, u64::from(id));
                h.rooms.remove(&id);
            }
        }
        h.tick_ms = started.elapsed().as_millis() as u64;
    }
}
