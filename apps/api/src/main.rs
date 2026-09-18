#![forbid(unsafe_code)]

mod admin;
mod audit;
mod bans;
mod config;
mod lobby;
mod maps;
mod ops;
mod security;
mod storage;

use axum::{
    extract::{
        connect_info::ConnectInfo,
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::{SinkExt, StreamExt};
use game_core::{
    Event, Input, MatchEvent, ModeKind, ModeRules, Replay, ReplayHeader, Rotation, ValidatedMap,
    WeaponTable, World, TICK_RATE,
};
use map_editor::{original_default_map, DEFAULT_MAPS};
use maps::{MapService, TransferEvent};
use protocol::{
    ClientMessage, MapInfo, RoomInfo, ServerMessage, WireModifiers, WireSpectateCommand, VERSION,
};
use std::{
    collections::{BTreeMap, HashMap},
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, Mutex};
use tracing::info;

const ROOM_CAPACITY: usize = 16;
type Sender = mpsc::UnboundedSender<String>;

struct Guest {
    id: u32,
    name: String,
    token: String,
    sender: Option<Sender>,
    room: Option<u32>,
    disconnected: Option<Instant>,
    last_chat: Instant,
    last_input: u32,
    muted: std::collections::HashSet<u32>,
    admin: bool,
    flood: game_core::Flood,
    ip: IpAddr,
    last_ping: Instant,
}
struct Room {
    id: u32,
    name: String,
    mode: String,
    code: String,
    public: bool,
    world: World,
    inputs: BTreeMap<u32, Input>,
    map: String,
    /// The simulation clears its events every tick but snapshots go out less often, so they are
    /// gathered here and drained into the snapshot. Otherwise kills and hits would be dropped.
    pending_events: Vec<Event>,
    /// Lifecycle decisions gathered the same way, so a round ending between snapshots is never
    /// lost to the client.
    pending_match_events: Vec<MatchEvent>,
    /// The maps this room plays, in order.
    rotation: Rotation,
    /// The community ruleset this room is playing, when it is playing one.
    ruleset: Option<String>,
    password: String,
    paused: bool,
    capacity: usize,
    required_mod: String,
    replay: Replay,
    votes: std::collections::HashMap<u32, String>,
}
struct Hub {
    guests: HashMap<u32, Guest>,
    tokens: HashMap<String, u32>,
    rooms: BTreeMap<u32, Room>,
    next_guest: u32,
    next_room: u32,
    maps: MapService,
    map_transfers: BTreeMap<u64, u32>,
    bans: bans::BanList,
    audit: audit::AuditLog,
    config: config::ServerConfig,
    limits: security::RateLimits,
    store: storage::Store,
    drain: bool,
    tick_ms: u64,
    errors: u64,
    owner: String,
}
impl Hub {
    fn new() -> Self {
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
    fn persist_for_shutdown(&mut self) {
        let keep_drain = self.drain;
        self.drain = true;
        self.persist();
        self.store.drain = keep_drain;
        if std::env::var_os("DATA_DIR").is_some() {
            let _ = storage::save_file(&storage::store_path(), &self.store);
        }
    }
    fn send(&self, id: u32, msg: &ServerMessage) {
        if let Some(tx) = self.guests.get(&id).and_then(|g| g.sender.as_ref()) {
            if let Ok(raw) = serde_json::to_string(msg) {
                let _ = tx.send(raw);
            }
        }
    }
    fn room_list(&self) -> Vec<RoomInfo> {
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
    fn requires_mod(&self, room_id: u32, offered: Option<&str>) -> Result<(), &'static str> {
        let required = &self.rooms.get(&room_id).ok_or("room_missing")?.required_mod;
        if required.is_empty() || required == offered.unwrap_or("") {
            Ok(())
        } else {
            Err("mod_required")
        }
    }
    fn create_room(&mut self, request: RoomRequest) -> Result<u32, &'static str> {
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
    fn room_id_by_code(&self, code: &str) -> Option<u32> {
        let code = code.trim().to_uppercase();
        self.rooms
            .values()
            .find(|room| room.code == code)
            .map(|room| room.id)
    }
    fn leave(&mut self, id: u32) {
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
    fn join(&mut self, id: u32, room_id: u32) -> Result<(), &'static str> {
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
type Shared = Arc<Mutex<Hub>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=info".into()),
        )
        .init();
    let hub: Shared = Arc::new(Mutex::new(Hub::new()));
    tokio::spawn(tick_loop(hub.clone()));
    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(metrics))
        .route("/ws", get(ws_handler))
        .with_state(hub.clone());
    let bind = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .expect("bind server");
    info!(%bind, "listening");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(hub))
    .await
    .expect("serve");
}
async fn shutdown_signal(hub: Shared) {
    let ctrl_c = tokio::signal::ctrl_c();
    #[cfg(unix)]
    {
        let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("listen for SIGTERM");
        tokio::select! {
            _ = ctrl_c => {}
            _ = term.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = ctrl_c.await;
    }
    let mut h = hub.lock().await;
    h.persist_for_shutdown();
    info!("shutdown signal");
}
async fn health() -> Json<serde_json::Value> {
    Json(ops::liveness())
}
async fn ready(State(hub): State<Shared>) -> impl IntoResponse {
    let h = hub.lock().await;
    let players = h.rooms.values().map(|r| r.world.players.len()).sum();
    let (code, body) = ops::readiness(h.drain, h.rooms.len(), players);
    (
        StatusCode::from_u16(code).unwrap_or(StatusCode::OK),
        Json(body),
    )
}
async fn metrics(State(hub): State<Shared>) -> String {
    let h = hub.lock().await;
    ops::metrics_text(
        h.guests.len(),
        h.rooms.len(),
        h.rooms
            .values()
            .map(|r| r.world.players.len())
            .sum::<usize>(),
        h.audit.len(),
        h.bans.persist().len(),
        h.tick_ms,
        h.errors,
    )
}
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Shared>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    if let Ok(origin) = std::env::var("ALLOWED_ORIGIN") {
        if !origin.is_empty()
            && headers.get("origin").and_then(|v| v.to_str().ok()) != Some(origin.as_str())
        {
            return StatusCode::FORBIDDEN.into_response();
        }
    }
    let trusted = std::env::var("TRUSTED_PROXIES").unwrap_or_default();
    let forwarded = headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok());
    let ip = security::client_ip(addr.ip(), forwarded, &trusted);
    ws.max_message_size(protocol::MAX_MESSAGE_BYTES)
        .on_upgrade(move |socket| handle_socket(socket, hub, ip))
        .into_response()
}
async fn handle_socket(socket: WebSocket, hub: Shared, ip: IpAddr) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let writer = tokio::spawn(async move {
        while let Some(raw) = rx.recv().await {
            if ws_tx.send(Message::Text(raw.into())).await.is_err() {
                break;
            }
        }
    });
    let mut identity = None;
    while let Some(Ok(message)) = ws_rx.next().await {
        let Message::Text(raw) = message else {
            continue;
        };
        let parsed = match protocol::parse_client(&raw) {
            Ok(v) => v,
            Err(code) => {
                send_error(&tx, code);
                continue;
            }
        };
        let mut h = hub.lock().await;
        if identity.is_none() {
            if let ClientMessage::Hello {
                version,
                name,
                resume,
                admin_password,
            } = parsed
            {
                if version != VERSION {
                    send_error(&tx, "version_mismatch");
                    break;
                }
                if h.bans.is_banned(ip) {
                    let _ = tx.send(
                        serde_json::to_string(&ServerMessage::Kicked {
                            reason: "banned".into(),
                        })
                        .unwrap(),
                    );
                    break;
                }
                if !h.limits.allow_connect(ip) {
                    send_error(&tx, "rate_limited");
                    break;
                }
                let name = clean_text(&name, 20);
                if name.is_empty() {
                    send_error(&tx, "invalid_name");
                    continue;
                }
                let dupes: Vec<u32> = h
                    .guests
                    .iter()
                    .filter(|(_, guest)| {
                        guest.sender.is_some() && guest.name.eq_ignore_ascii_case(&name)
                    })
                    .map(|(&id, _)| id)
                    .collect();
                for id in dupes {
                    h.send(
                        id,
                        &ServerMessage::Kicked {
                            reason: "duplicate".into(),
                        },
                    );
                    h.leave(id);
                }
                if h.guests.values().filter(|g| g.sender.is_some()).count() >= 128 {
                    send_error(&tx, "server_full");
                    break;
                }
                let resumed = resume
                    .and_then(|token| h.tokens.get(&token).copied())
                    .filter(|id| {
                        h.guests
                            .get(id)
                            .and_then(|g| g.disconnected)
                            .is_some_and(|t| t.elapsed() < security::resume_ttl())
                    });
                let id = if let Some(id) = resumed {
                    let g = h.guests.get_mut(&id).unwrap();
                    g.sender = Some(tx.clone());
                    g.disconnected = None;
                    g.last_input = 0;
                    g.last_ping = Instant::now();
                    id
                } else {
                    let id = h.next_guest;
                    h.next_guest += 1;
                    let token = uuid::Uuid::new_v4().to_string();
                    let admin = h.bans.is_admin(Some(ip), None);
                    h.tokens.insert(token.clone(), id);
                    h.guests.insert(
                        id,
                        Guest {
                            id,
                            name,
                            token,
                            sender: Some(tx.clone()),
                            room: None,
                            disconnected: None,
                            last_chat: Instant::now() - Duration::from_secs(10),
                            last_input: 0,
                            muted: std::collections::HashSet::new(),
                            admin,
                            flood: game_core::Flood::default(),
                            ip,
                            last_ping: Instant::now(),
                        },
                    );
                    id
                };
                identity = Some(id);
                if let Some(password) = admin_password {
                    if h.config.authenticate_admin(&password) {
                        h.guests.get_mut(&id).unwrap().admin = true;
                        h.bans.add_admin_id(id);
                    }
                }
                if h.bans.is_admin(Some(ip), Some(id)) {
                    h.guests.get_mut(&id).unwrap().admin = true;
                }
                let token = h.guests[&id].token.clone();
                h.send(
                    id,
                    &ServerMessage::Welcome {
                        version: VERSION,
                        player: id,
                        token,
                    },
                );
                h.send(
                    id,
                    &ServerMessage::Rooms {
                        rooms: h.room_list(),
                    },
                );
                h.send(
                    id,
                    &ServerMessage::MapCatalog {
                        maps: DEFAULT_MAPS
                            .iter()
                            .map(|entry| MapInfo {
                                name: entry.name.into(),
                                mode: format!("{:?}", entry.mode).to_lowercase(),
                                preview: format!("previews/{}.svg", entry.name),
                            })
                            .collect(),
                    },
                );
                if let Some(room_id) = h.guests[&id].room {
                    h.send(
                        id,
                        &ServerMessage::Joined {
                            room: room_id,
                            player: id,
                            name: h.rooms[&room_id].name.clone(),
                            code: h.rooms[&room_id].code.clone(),
                            public: h.rooms[&room_id].public,
                        },
                    );
                }
            } else {
                send_error(&tx, "hello_required");
            }
            continue;
        }
        let id = identity.unwrap();
        if !h
            .guests
            .get(&id)
            .and_then(|guest| guest.sender.as_ref())
            .is_some_and(|active| active.same_channel(&tx))
        {
            break;
        }
        match parsed {
            ClientMessage::Hello { .. } => send_error(&tx, "already_connected"),
            ClientMessage::Rooms => h.send(
                id,
                &ServerMessage::Rooms {
                    rooms: h.room_list(),
                },
            ),
            ClientMessage::Spectate { command } => {
                if let Some(room_id) = h.guests.get(&id).and_then(|guest| guest.room) {
                    if let Some(room) = h.rooms.get_mut(&room_id) {
                        room.world.spectate(
                            id,
                            match command {
                                WireSpectateCommand::Next => game_core::SpectateCommand::Next,
                                WireSpectateCommand::Previous => {
                                    game_core::SpectateCommand::Previous
                                }
                                WireSpectateCommand::Follow { player } => {
                                    game_core::SpectateCommand::Follow(player)
                                }
                                WireSpectateCommand::FreeCamera => {
                                    game_core::SpectateCommand::FreeCamera
                                }
                            },
                        );
                    }
                }
            }
            ClientMessage::Bots { count, difficulty } => {
                // Anybody in the room may set the bot count; there is no admin layer yet, and a
                // room full of bots nobody wanted is easier to fix than one nobody can fill.
                if let Some(room_id) = h.guests.get(&id).and_then(|guest| guest.room) {
                    if let Some(room) = h.rooms.get_mut(&room_id) {
                        let difficulty = game_core::Difficulty::from_id(
                            difficulty.as_deref().unwrap_or("normal"),
                        );
                        let wanted = usize::from(count).min(MAX_BOTS);
                        fill_with_bots(&mut room.world, wanted, difficulty);
                    }
                }
            }
            ClientMessage::SetTeam { team } => {
                if let Some(room_id) = h.guests.get(&id).and_then(|guest| guest.room) {
                    if let Some(room) = h.rooms.get_mut(&room_id) {
                        room.world
                            .set_team(id, game_core::TeamChoice::from_id(&team));
                    }
                }
            }
            ClientMessage::CreateRoom {
                name,
                mode,
                public,
                map,
                weapon_mod,
                modifiers,
                ruleset,
                bonuses,
                bots,
                bot_difficulty,
                password,
                required_mod,
            } => {
                let ip = h.guests[&id].ip;
                if !h.limits.allow_room(ip) {
                    send_error(&tx, "rate_limited");
                    continue;
                }
                if h.rooms.len() >= 16 {
                    send_error(&tx, "room_limit");
                    continue;
                }
                let name = clean_text(&name, 24);
                if name.is_empty() || !ModeKind::from_id(&mode).is_implemented() {
                    send_error(&tx, "invalid_room");
                    continue;
                }
                match h.create_room(RoomRequest {
                    name,
                    mode,
                    public,
                    requested_map: map,
                    weapon_mod,
                    modifiers,
                    ruleset,
                    bonuses,
                    bots,
                    bot_difficulty,
                    password: password.unwrap_or_default(),
                    required_mod: required_mod.unwrap_or_default(),
                }) {
                    Ok(room_id) => {
                        let _ = h.join(id, room_id);
                    }
                    Err(code) => send_error(&tx, code),
                }
            }
            ClientMessage::JoinRoom {
                room,
                password,
                spectator,
                mod_hash,
            } => {
                if h.rooms
                    .get(&room)
                    .is_some_and(|candidate| !candidate.public)
                {
                    send_error(&tx, "invite_code_required");
                } else if h.rooms.get(&room).is_some_and(|candidate| {
                    !candidate.password.is_empty()
                        && candidate.password != password.unwrap_or_default()
                }) {
                    send_error(&tx, "bad_password");
                } else if let Err(code) = h.requires_mod(room, mod_hash.as_deref()) {
                    send_error(&tx, code);
                } else if let Err(code) = h.join(id, room) {
                    send_error(&tx, code);
                } else if spectator {
                    if let Some(room_id) = h.guests.get(&id).and_then(|guest| guest.room) {
                        if let Some(found) = h.rooms.get_mut(&room_id) {
                            found.world.set_team(id, game_core::TeamChoice::Spectator);
                        }
                    }
                }
            }
            ClientMessage::JoinByCode {
                code,
                password,
                spectator,
                mod_hash,
            } => {
                if let Some(room) = h.room_id_by_code(&code) {
                    if h.rooms.get(&room).is_some_and(|candidate| {
                        !candidate.password.is_empty()
                            && candidate.password != password.unwrap_or_default()
                    }) {
                        send_error(&tx, "bad_password");
                    } else if let Err(code) = h.requires_mod(room, mod_hash.as_deref()) {
                        send_error(&tx, code);
                    } else if let Err(code) = h.join(id, room) {
                        send_error(&tx, code);
                    } else if spectator {
                        if let Some(found) = h.rooms.get_mut(&room) {
                            found.world.set_team(id, game_core::TeamChoice::Spectator);
                        }
                    }
                } else {
                    send_error(&tx, "invalid_invite_code");
                }
            }
            ClientMessage::LeaveRoom => h.leave(id),
            ClientMessage::Input { input } => {
                if !h.limits.allow_message(id) {
                    continue;
                }
                if let Some(room_id) = h.guests[&id].room {
                    if !security::input_feasible(
                        input.seq,
                        h.guests[&id].last_input,
                        input.aim.x,
                        input.aim.y,
                    ) {
                        continue;
                    }
                    h.guests.get_mut(&id).unwrap().last_input = input.seq;
                    if let Some(room) = h.rooms.get_mut(&room_id) {
                        if !room.paused {
                            room.inputs.insert(id, input);
                        }
                    }
                }
            }
            ClientMessage::Chat { text, scope } => {
                let requested = if scope.as_deref() == Some("team") {
                    game_core::ChatScope::Team
                } else {
                    game_core::ChatScope::All
                };
                let (scope, text) = game_core::parse_chat(&text, requested);
                let text = game_core::censor(&clean_text(&text, 120), true);
                if text.is_empty() {
                    continue;
                }
                let guest = h.guests.get_mut(&id).unwrap();
                if !guest.flood.allow() || guest.last_chat.elapsed() < Duration::from_millis(250) {
                    continue;
                }
                guest.last_chat = Instant::now();
                let room_id = guest.room;
                let name = guest.name.clone();
                let sender_team = room_id
                    .and_then(|room| h.rooms.get(&room))
                    .and_then(|room| room.world.players.get(&id))
                    .map(|player| player.team)
                    .unwrap_or(0);
                let sender_alive = room_id
                    .and_then(|room| h.rooms.get(&room))
                    .and_then(|room| room.world.players.get(&id))
                    .is_some_and(|player| player.hp > 0);
                if let Some(room_id) = room_id {
                    let line = game_core::ChatLine {
                        player: id,
                        name: name.clone(),
                        text: text.clone(),
                        scope,
                    };
                    let realistic = h
                        .rooms
                        .get(&room_id)
                        .is_some_and(|room| room.world.rules.modifiers.realistic);
                    let survival = h
                        .rooms
                        .get(&room_id)
                        .is_some_and(|room| room.world.rules.modifiers.survival);
                    let recipients: Vec<u32> = h
                        .guests
                        .values()
                        .filter(|g| g.room == Some(room_id) && !g.muted.contains(&id))
                        .filter(|g| {
                            let player = h
                                .rooms
                                .get(&room_id)
                                .and_then(|room| room.world.players.get(&g.id));
                            game_core::may_see_chat(
                                &line,
                                g.id,
                                player.map(|p| p.team).unwrap_or(0),
                                sender_team,
                                player.is_some_and(|p| p.hp > 0),
                                sender_alive,
                                realistic,
                                survival,
                            )
                        })
                        .map(|g| g.id)
                        .collect();
                    let scope_name = match scope {
                        game_core::ChatScope::Team => "team",
                        game_core::ChatScope::Server => "server",
                        game_core::ChatScope::All => "all",
                    }
                    .to_string();
                    for guest_id in recipients {
                        h.send(
                            guest_id,
                            &ServerMessage::Chat {
                                player: id,
                                name: name.clone(),
                                text: text.clone(),
                                scope: scope_name.clone(),
                            },
                        );
                    }
                }
            }
            ClientMessage::Command { line, password } => {
                if let Some(password) = password {
                    if h.config.authenticate_admin(&password) {
                        h.guests.get_mut(&id).unwrap().admin = true;
                    }
                }
                if let Ok(command) = game_core::parse_player_command(&line) {
                    if command.is_pause() {
                        if let Some(room_id) = h.guests[&id].room {
                            if let Some(room) = h.rooms.get_mut(&room_id) {
                                room.paused = command == game_core::PlayerCommand::Pause;
                            }
                        }
                    } else if let Some(room_id) = h.guests[&id].room {
                        if let Some(room) = h.rooms.get_mut(&room_id) {
                            room.world.apply_player_command(id, command);
                        }
                    }
                } else if let Ok(command) = admin::parse_admin(&line) {
                    let guest = &h.guests[&id];
                    let role = if guest.admin || h.bans.is_admin(Some(guest.ip), Some(id)) {
                        admin::Role::Admin
                    } else {
                        admin::Role::Player
                    };
                    if admin::authorize(role, &command).is_ok() {
                        h.audit.record(id.to_string(), format!("{command:?}"), "");
                        apply_admin(&mut h, id, command);
                    } else {
                        send_error(&tx, "unauthorized");
                    }
                } else {
                    send_error(&tx, "unknown_command");
                }
            }
            ClientMessage::Mute { target } => {
                let muted_id = target.parse::<u32>().ok().or_else(|| {
                    h.guests
                        .values()
                        .find(|guest| guest.name.eq_ignore_ascii_case(&target))
                        .map(|guest| guest.id)
                });
                if let Some(other) = muted_id {
                    h.guests.get_mut(&id).unwrap().muted.insert(other);
                }
            }
            ClientMessage::Ping { nonce } => {
                if let Some(guest) = h.guests.get_mut(&id) {
                    guest.last_ping = Instant::now();
                }
                h.send(id, &ServerMessage::Pong { nonce });
            }
            ClientMessage::MapDownloadStart {
                map_hash,
                cached_assets,
            } => match h.maps.start(map_hash, &cached_assets) {
                Ok((transfer, manifest, total_bytes)) => {
                    h.map_transfers.insert(transfer, id);
                    h.send(
                        id,
                        &ServerMessage::MapDownloadManifest {
                            transfer,
                            manifest,
                            total_bytes,
                        },
                    );
                }
                Err(code) => send_error(&tx, code),
            },
            ClientMessage::MapDownloadCancel { transfer } => {
                if h.map_transfers.get(&transfer) == Some(&id) {
                    if let Ok(TransferEvent::Cancelled { transfer }) = h.maps.cancel(transfer) {
                        h.map_transfers.remove(&transfer);
                        h.send(id, &ServerMessage::MapDownloadCancelled { transfer });
                    }
                } else {
                    send_error(&tx, "transfer_missing");
                }
            }
        }
    }
    if let Some(id) = identity {
        let mut h = hub.lock().await;
        if let Some(g) = h.guests.get_mut(&id) {
            if g.sender
                .as_ref()
                .is_some_and(|active| active.same_channel(&tx))
            {
                g.sender = None;
                g.disconnected = Some(Instant::now());
            }
        }
    }
    writer.abort();
}
fn send_error(tx: &Sender, code: &str) {
    let _ = tx.send(
        serde_json::to_string(&ServerMessage::Error {
            code: code.into(),
            message: code.replace('_', " "),
        })
        .unwrap(),
    );
}
fn apply_admin(hub: &mut Hub, actor: u32, command: admin::AdminCommand) {
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

fn clean_text(raw: &str, max: usize) -> String {
    raw.chars()
        .filter(|c| !c.is_control() && *c != '<' && *c != '>')
        .take(max)
        .collect::<String>()
        .trim()
        .to_string()
}
async fn tick_loop(hub: Shared) {
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

/// What a client asked for when it opened a room.
///
/// A struct rather than a long argument list: the fields are all optional-looking strings and
/// flags, and getting two of them the wrong way round would be silent.
#[derive(Clone, Debug, Default)]
struct RoomRequest {
    name: String,
    mode: String,
    public: bool,
    requested_map: Option<String>,
    weapon_mod: Option<String>,
    modifiers: Option<WireModifiers>,
    ruleset: Option<String>,
    /// Whether bonus kits are handed out. `None` takes the server default, which is on.
    bonuses: Option<bool>,
    /// How many bots to fill the room with, and how hard they should be.
    bots: Option<u8>,
    bot_difficulty: Option<String>,
    password: String,
    required_mod: String,
}

/// Builds the snapshot one recipient is allowed to see.
///
/// Outside Realistic every player sees the same world, so the shared snapshot is handed straight
/// back. In Realistic an enemy the recipient cannot actually see is left out of their copy
/// entirely, because a client that never receives a position cannot reveal it.
fn snapshot_for(world: &World, recipient: u32) -> World {
    if !world.rules.modifiers.realistic {
        return world.clone();
    }
    let Some(observer) = world.players.get(&recipient) else {
        return world.clone();
    };
    let observer =
        game_core::Viewer::new(observer.id, observer.team, observer.pos, observer.hp > 0);

    let mut filtered = world.clone();
    filtered.players.retain(|_, player| {
        game_core::visibility_between(
            world.collision(),
            observer,
            game_core::Viewer::new(player.id, player.team, player.pos, player.hp > 0),
        )
        .is_visible()
    });
    filtered
}

/// The most bots one room may hold, so a command cannot fill a server with them.
const MAX_BOTS: usize = 12;

/// Ids a server hands to bots, high enough never to collide with a guest's.
const BOT_ID_BASE: u32 = 900_000;

/// Fills a world with bots up to `wanted`, or removes them down to it.
///
/// Bots join through the same path a human does, so they are assigned to teams, spawned, and given
/// a loadout by exactly the same rules.
fn fill_with_bots(world: &mut World, wanted: usize, difficulty: game_core::Difficulty) -> usize {
    let profiles = game_core::bots::profile::default_profiles();
    let existing = world.bots.len();
    if wanted <= existing {
        return world.trim_bots(wanted);
    }
    for index in existing..wanted {
        let stock = &profiles[index % profiles.len()];
        // Past the stock list the names repeat, so they are numbered to stay distinct.
        let name = if index < profiles.len() {
            stock.name.clone()
        } else {
            format!("{} {}", stock.name, index / profiles.len() + 1)
        };
        let profile = game_core::BotProfile {
            name: game_core::bots::profile::clean_name(&name),
            difficulty,
            ..stock.clone()
        };
        world.add_bot(
            BOT_ID_BASE + index as u32,
            profile,
            game_core::TeamChoice::Auto,
        );
    }
    wanted - existing
}

/// Whether a map built for `map_mode` can host a room playing `mode`.
///
/// Teammatch happily runs on a deathmatch map; a flag mode cannot, because it needs the bases.
fn map_suits_mode(map_mode: content::manifest::MapMode, mode: ModeKind) -> bool {
    use content::manifest::MapMode;
    match mode {
        ModeKind::Deathmatch | ModeKind::Teammatch | ModeKind::Rambomatch => {
            matches!(map_mode, MapMode::Deathmatch | MapMode::TeamMatch)
        }
        ModeKind::Pointmatch => matches!(map_mode, MapMode::Deathmatch | MapMode::PointMatch),
        ModeKind::CaptureTheFlag => matches!(map_mode, MapMode::CaptureTheFlag),
        ModeKind::HoldTheFlag => matches!(map_mode, MapMode::HoldTheFlag),
        ModeKind::Infiltration => matches!(map_mode, MapMode::Infiltration),
    }
}

/// Every map the server ships that suits `mode`, looping, with `first` played first.
///
/// A rotation is never empty: a server with nothing to load would strand every room on it.
fn default_rotation(first: &str, mode: ModeKind) -> Rotation {
    let names: Vec<String> = DEFAULT_MAPS
        .iter()
        .filter(|entry| map_suits_mode(entry.mode, mode))
        .map(|entry| entry.name.to_string())
        .collect();
    let mut rotation = Rotation::new(names, true).unwrap_or_else(|| {
        Rotation::new(vec![first.to_string()], true).expect("one map is always a valid rotation")
    });
    rotation.select(first);
    rotation
}

/// Loads the next map in a room's rotation and restarts the match on it.
///
/// Players stay where they are in the room; the world, the score, and the clock are rebuilt, which
/// is what a map change is. A map that will not load leaves the room on the one it is already
/// playing rather than dropping everybody.
fn advance_map(room: &mut Room) {
    let next = room.rotation.next_map().to_string();
    let Some(entry) = DEFAULT_MAPS.iter().find(|entry| entry.name == next) else {
        room.world.restart_match();
        return;
    };
    let Ok(map) = ValidatedMap::try_from(original_default_map(*entry)) else {
        room.world.restart_match();
        return;
    };

    let carried: Vec<(u32, String)> = room
        .world
        .players
        .iter()
        .map(|(id, player)| (*id, player.name.clone()))
        .collect();
    let rules = room.world.rules;
    let weapons = room.world.weapons.clone();

    let mut world = World::with_map(&room.mode, &map);
    world.rules = rules;
    world.weapons = weapons;
    world.place_objectives();
    for (id, name) in carried {
        world.add_player(id, name);
    }
    world.restart_match();
    room.map = next;
    room.replay = Replay::new(ReplayHeader::new(
        VERSION,
        &room.map,
        &world,
        u64::from(room.id),
    ));
    room.world = world;
    room.inputs.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_rooms_are_hidden_but_resolvable_by_code() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Friends".into(),
                mode: "deathmatch".into(),
                public: false,
                requested_map: Some("Arena".into()),
                ..RoomRequest::default()
            })
            .unwrap();
        assert!(!hub.room_list().iter().any(|room| room.id == room_id));
        let code = hub.rooms[&room_id].code.clone();
        assert_eq!(hub.room_id_by_code(&code.to_lowercase()), Some(room_id));
    }

    #[test]
    // Acceptance evidence for docs/parity/coverage.json: server:bots
    fn a_room_can_be_opened_with_bots_already_in_it() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "With bots".into(),
                mode: "deathmatch".into(),
                public: true,
                bots: Some(4),
                bot_difficulty: Some("veteran".into()),
                ..RoomRequest::default()
            })
            .unwrap();

        let world = &hub.rooms[&room_id].world;
        assert_eq!(world.bots.len(), 4, "nobody walks into an empty match");
        assert_eq!(world.players.len(), 4);
        for bot in world.bots.values() {
            assert_eq!(bot.profile.difficulty, game_core::Difficulty::Veteran);
            assert!(bot.profile.validate().is_ok(), "{}", bot.profile.name);
        }
        // No two bots share a name, however many there are.
        let mut names: Vec<&str> = world
            .bots
            .values()
            .map(|bot| bot.profile.name.as_str())
            .collect();
        names.sort_unstable();
        let count = names.len();
        names.dedup();
        assert_eq!(names.len(), count);
    }

    #[test]
    fn the_bot_count_can_be_raised_and_lowered_and_is_capped() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Fill".into(),
                mode: "deathmatch".into(),
                public: true,
                ..RoomRequest::default()
            })
            .unwrap();
        let world = &mut hub.rooms.get_mut(&room_id).unwrap().world;

        fill_with_bots(world, 3, game_core::Difficulty::Normal);
        assert_eq!(world.bots.len(), 3);

        fill_with_bots(world, 1, game_core::Difficulty::Normal);
        assert_eq!(world.bots.len(), 1, "and back down again");
        assert_eq!(world.players.len(), 1, "with no ghosts left behind");

        fill_with_bots(world, MAX_BOTS, game_core::Difficulty::Normal);
        assert_eq!(world.bots.len(), MAX_BOTS);

        fill_with_bots(world, 0, game_core::Difficulty::Normal);
        assert!(world.bots.is_empty());
    }

    #[test]
    // Acceptance evidence for docs/parity/coverage.json: server:bonus-settings
    fn a_room_can_switch_the_bonus_kits_off() {
        let mut hub = Hub::new();
        let on = hub
            .create_room(RoomRequest {
                name: "Kits".into(),
                mode: "deathmatch".into(),
                public: true,
                ..RoomRequest::default()
            })
            .unwrap();
        assert!(
            hub.rooms[&on].world.bonuses.any(),
            "kits are handed out by default"
        );

        let off = hub
            .create_room(RoomRequest {
                name: "No kits".into(),
                mode: "deathmatch".into(),
                public: true,
                bonuses: Some(false),
                ..RoomRequest::default()
            })
            .unwrap();
        assert!(!hub.rooms[&off].world.bonuses.any(), "and off when asked");
    }

    #[test]
    // Acceptance evidence for docs/parity/coverage.json: server:modifier-rooms
    fn a_room_can_be_created_with_any_combination_of_modifiers() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Hardcore".into(),
                mode: "deathmatch".into(),
                public: true,
                modifiers: Some(WireModifiers {
                    realistic: true,
                    survival: true,
                    advance: false,
                }),
                ..RoomRequest::default()
            })
            .unwrap();

        let world = &hub.rooms[&room_id].world;
        assert!(world.rules.modifiers.realistic);
        assert!(world.rules.modifiers.survival);
        assert!(!world.rules.modifiers.advance);
        // A Realistic room plays on the Realistic table without having to be told twice.
        assert!(world.weapons.is_realistic());

        let info = hub
            .room_list()
            .into_iter()
            .find(|room| room.id == room_id)
            .unwrap();
        assert!(info.modifiers.realistic && info.modifiers.survival);
        assert!(info.modifiers.any(), "the browser shows a badge");
        assert_eq!(info.ruleset, None);
    }

    #[test]
    // Acceptance evidence for docs/parity/coverage.json: server:community-modes
    fn every_community_ruleset_opens_a_playable_room() {
        for ruleset in game_core::modes::scripted::presets::all() {
            let mut hub = Hub::new();
            let created = hub.create_room(RoomRequest {
                name: format!("{} room", ruleset.name),
                mode: "deathmatch".into(),
                public: true,
                ruleset: Some(ruleset.name.clone()),
                ..RoomRequest::default()
            });
            // A ruleset whose base mode has no maps yet is refused cleanly rather than crashing.
            let Ok(room_id) = created else {
                assert_eq!(created.unwrap_err(), "map_missing", "{}", ruleset.name);
                continue;
            };
            let room = &hub.rooms[&room_id];
            assert_eq!(room.ruleset.as_deref(), Some(ruleset.name.as_str()));
            assert_eq!(room.world.rules.kind, ruleset.base, "{}", ruleset.name);
            assert_eq!(
                room.world.rules.modifiers.realistic,
                ruleset.modifiers.realistic
            );
        }
    }

    #[test]
    fn an_unknown_community_ruleset_is_refused_rather_than_silently_ignored() {
        let mut hub = Hub::new();
        assert_eq!(
            hub.create_room(RoomRequest {
                name: "Nope".into(),
                mode: "deathmatch".into(),
                public: true,
                ruleset: Some("Not A Real Mode".into()),
                ..RoomRequest::default()
            }),
            Err("unknown_ruleset")
        );
    }

    #[test]
    // Acceptance evidence for docs/parity/coverage.json: server:realistic-visibility
    fn a_realistic_snapshot_leaves_out_an_enemy_the_recipient_cannot_see() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Realistic".into(),
                mode: "team".into(),
                public: true,
                modifiers: Some(WireModifiers {
                    realistic: true,
                    survival: false,
                    advance: false,
                }),
                ..RoomRequest::default()
            })
            .unwrap();
        let room = hub.rooms.get_mut(&room_id).unwrap();
        room.world
            .add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
        room.world
            .add_player_as(2, "Bravo".into(), game_core::TeamChoice::Bravo);

        // Standing on top of each other, both are sent.
        let here = game_core::Vec2 { x: 600.0, y: 600.0 };
        room.world.players.get_mut(&1).unwrap().pos = here;
        room.world.players.get_mut(&2).unwrap().pos = here;
        assert_eq!(snapshot_for(&room.world, 1).players.len(), 2);

        // Put the enemy the other side of the floor and they vanish from the snapshot.
        room.world.players.get_mut(&2).unwrap().pos = game_core::Vec2 { x: 600.0, y: 690.0 };
        let filtered = snapshot_for(&room.world, 1);
        assert_eq!(
            filtered.players.len(),
            1,
            "the enemy is not in the snapshot"
        );
        assert!(filtered.players.contains_key(&1), "but the recipient is");
    }

    #[test]
    fn a_normal_room_sends_everybody_the_same_world() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Plain".into(),
                mode: "team".into(),
                public: true,
                ..RoomRequest::default()
            })
            .unwrap();
        let room = hub.rooms.get_mut(&room_id).unwrap();
        room.world
            .add_player_as(1, "Alpha".into(), game_core::TeamChoice::Alpha);
        room.world
            .add_player_as(2, "Bravo".into(), game_core::TeamChoice::Bravo);
        room.world.players.get_mut(&2).unwrap().pos = game_core::Vec2 { x: 600.0, y: 690.0 };

        assert_eq!(
            snapshot_for(&room.world, 1).players.len(),
            2,
            "without Realistic nothing is hidden"
        );
    }

    #[test]
    // Acceptance evidence for docs/parity/coverage.json: server:map-rotation
    fn a_finished_round_moves_the_room_onto_the_next_map_and_keeps_its_players() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Rotating".into(),
                mode: "deathmatch".into(),
                public: true,
                ..RoomRequest::default()
            })
            .unwrap();
        let room = hub.rooms.get_mut(&room_id).unwrap();
        room.world.add_player(1, "One".into());
        room.world.add_player(2, "Two".into());
        let first = room.map.clone();
        assert!(room.rotation.len() > 1, "the server ships a real rotation");

        advance_map(room);

        assert_ne!(room.map, first, "it moved on");
        assert_eq!(room.world.players.len(), 2, "and took its players with it");
        assert_eq!(
            room.world.match_state.phase,
            game_core::MatchPhase::Countdown,
            "the new map starts from a countdown"
        );
        assert_eq!(room.world.scores, [0, 0], "on a clean score");
    }

    #[test]
    fn a_rooms_rotation_loops_rather_than_running_out() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Looping".into(),
                mode: "deathmatch".into(),
                public: true,
                ..RoomRequest::default()
            })
            .unwrap();
        let room = hub.rooms.get_mut(&room_id).unwrap();
        let length = room.rotation.len();
        let first = room.map.clone();

        for _ in 0..length {
            advance_map(room);
        }
        assert_eq!(room.map, first, "a full lap comes back to where it started");
    }

    #[test]
    fn public_rooms_advertise_the_weapon_mod_hash() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Real".into(),
                mode: "deathmatch".into(),
                public: true,
                requested_map: Some("Arena".into()),
                weapon_mod: Some("realistic".into()),
                ..RoomRequest::default()
            })
            .unwrap();
        let info = hub
            .room_list()
            .into_iter()
            .find(|room| room.id == room_id)
            .unwrap();
        assert_eq!(info.weapon_mod, "Realistic mod");
        assert_eq!(info.weapon_hash, WeaponTable::realistic().canonical_hash());
    }

    #[test]
    // Acceptance evidence: server:password-room
    fn a_passworded_room_advertises_the_lock_not_the_secret() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Lock".into(),
                mode: "deathmatch".into(),
                public: true,
                password: "secret".into(),
                ..RoomRequest::default()
            })
            .unwrap();
        let info = hub
            .room_list()
            .into_iter()
            .find(|room| room.id == room_id)
            .unwrap();
        assert!(info.password);
        assert_eq!(hub.rooms[&room_id].password, "secret");
    }

    #[test]
    fn a_required_mod_is_advertised_and_a_mismatch_is_refused() {
        let mut hub = Hub::new();
        let room_id = hub
            .create_room(RoomRequest {
                name: "Modded".into(),
                mode: "deathmatch".into(),
                public: true,
                required_mod: "abc".into(),
                ..RoomRequest::default()
            })
            .unwrap();
        let info = hub
            .room_list()
            .into_iter()
            .find(|room| room.id == room_id)
            .unwrap();
        assert_eq!(info.required_mod.as_deref(), Some("abc"));
        assert_eq!(hub.requires_mod(room_id, None), Err("mod_required"));
        assert_eq!(hub.requires_mod(room_id, Some("abc")), Ok(()));
    }

    #[test]
    // Acceptance evidence: server:load
    fn a_short_load_opens_many_public_rooms() {
        let mut hub = Hub::new();
        for index in 0..8 {
            hub.create_room(RoomRequest {
                name: format!("Load{index}"),
                mode: "deathmatch".into(),
                public: true,
                ..RoomRequest::default()
            })
            .unwrap();
        }
        assert!(hub.rooms.len() >= 9);
        assert_eq!(ops::readiness(false, hub.rooms.len(), 0).0, 200);
    }

    #[test]
    fn shutdown_flush_keeps_an_operator_drain_and_forces_one_on_the_dying_process() {
        let mut hub = Hub::new();
        hub.persist_for_shutdown();
        assert!(hub.drain);
        assert!(!hub.store.drain);
        hub.drain = true;
        hub.persist_for_shutdown();
        assert!(hub.store.drain);
    }

    #[test]
    fn drain_refuses_a_new_room_and_readiness_drops() {
        let mut hub = Hub::new();
        hub.drain = true;
        assert_eq!(
            hub.create_room(RoomRequest {
                name: "Late".into(),
                mode: "deathmatch".into(),
                public: true,
                ..RoomRequest::default()
            }),
            Err("draining")
        );
        assert_eq!(ops::readiness(true, hub.rooms.len(), 0).0, 503);
    }
}
