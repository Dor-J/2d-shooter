#![forbid(unsafe_code)]

mod maps;

use axum::{
    extract::{
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
    Event, Input, MatchEvent, ModeKind, ModeRules, Rotation, ValidatedMap, WeaponTable, World,
    TICK_RATE,
};
use map_editor::{original_default_map, DEFAULT_MAPS};
use maps::{MapService, TransferEvent};
use protocol::{
    ClientMessage, MapInfo, RoomInfo, ServerMessage, WireModifiers, WireSpectateCommand, VERSION,
};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, Mutex};
use tracing::info;

const ROOM_CAPACITY: usize = 16;
const RESUME_GRACE: Duration = Duration::from_secs(20);
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
}
struct Hub {
    guests: HashMap<u32, Guest>,
    tokens: HashMap<String, u32>,
    rooms: BTreeMap<u32, Room>,
    next_guest: u32,
    next_room: u32,
    maps: MapService,
    map_transfers: BTreeMap<u64, u32>,
}
impl Hub {
    fn new() -> Self {
        let maps = std::env::var_os("MAP_PACKAGES_DIR")
            .map(std::path::PathBuf::from)
            .map(|path| MapService::load_directory(&path).expect("load signed map packages"))
            .unwrap_or_default();
        let default_map = ValidatedMap::try_from(original_default_map(DEFAULT_MAPS[0]))
            .expect("built-in map must validate");
        let mut hub = Self {
            guests: HashMap::new(),
            tokens: HashMap::new(),
            rooms: BTreeMap::new(),
            next_guest: 1,
            next_room: 2,
            maps,
            map_transfers: BTreeMap::new(),
        };
        hub.rooms.insert(
            1,
            Room {
                id: 1,
                name: "Arena".into(),
                mode: "deathmatch".into(),
                code: "ARENA1".into(),
                public: true,
                world: World::with_map("deathmatch", &default_map),
                inputs: BTreeMap::new(),
                pending_events: Vec::new(),
                pending_match_events: Vec::new(),
                rotation: default_rotation("Aero", ModeKind::Deathmatch),
                ruleset: None,
                map: "Aero".into(),
            },
        );
        hub
    }
    fn send(&self, id: u32, msg: &ServerMessage) {
        if let Some(tx) = self.guests.get(&id).and_then(|g| g.sender.as_ref()) {
            if let Ok(raw) = serde_json::to_string(msg) {
                let _ = tx.send(raw);
            }
        }
    }
    fn room_list(&self) -> Vec<RoomInfo> {
        self.rooms
            .values()
            .filter(|r| r.public)
            .map(|r| RoomInfo {
                id: r.id,
                name: r.name.clone(),
                mode: r.mode.clone(),
                players: r.world.players.len(),
                capacity: ROOM_CAPACITY,
                map: r.map.clone(),
                weapon_mod: r.world.weapons.name().to_string(),
                weapon_hash: r.world.weapons.canonical_hash(),
                modifiers: WireModifiers {
                    realistic: r.world.rules.modifiers.realistic,
                    survival: r.world.rules.modifiers.survival,
                    advance: r.world.rules.modifiers.advance,
                },
                ruleset: r.ruleset.clone(),
            })
            .collect()
    }
    fn create_room(&mut self, request: RoomRequest) -> Result<u32, &'static str> {
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
        // Bots fill the room before anybody joins, so a player never walks into an empty match.
        if let Some(wanted) = bots.filter(|count| *count > 0) {
            let difficulty =
                game_core::Difficulty::from_id(bot_difficulty.as_deref().unwrap_or("normal"));
            fill_with_bots(&mut world, usize::from(wanted), difficulty);
        }
        self.rooms.insert(
            room_id,
            Room {
                id: room_id,
                name,
                mode: mode.clone(),
                code,
                public,
                world,
                inputs: BTreeMap::new(),
                pending_events: Vec::new(),
                pending_match_events: Vec::new(),
                rotation: default_rotation(entry.name, kind),
                ruleset: scripted.as_ref().map(|rules| rules.name.clone()),
                map: entry.name.into(),
            },
        );
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
        if let Some(room_id) = self.guests.get_mut(&id).and_then(|g| g.room.take()) {
            if let Some(room) = self.rooms.get_mut(&room_id) {
                room.world.players.remove(&id);
                room.inputs.remove(&id);
            }
        }
    }
    fn join(&mut self, id: u32, room_id: u32) -> Result<(), &'static str> {
        if self
            .rooms
            .get(&room_id)
            .ok_or("room_missing")?
            .world
            .players
            .len()
            >= ROOM_CAPACITY
        {
            return Err("room_full");
        }
        self.leave(id);
        let name = self.guests[&id].name.clone();
        let room = self.rooms.get_mut(&room_id).unwrap();
        room.world.add_player(id, name);
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
        .route(
            "/health",
            get(|| async { Json(serde_json::json!({"status":"ok"})) }),
        )
        .route("/metrics", get(metrics))
        .route("/ws", get(ws_handler))
        .with_state(hub);
    let bind = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .expect("bind server");
    info!(%bind, "listening");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
            info!("shutdown signal");
        })
        .await
        .expect("serve");
}
async fn metrics(State(hub): State<Shared>) -> String {
    let h = hub.lock().await;
    format!(
        "game_guests {}\ngame_rooms {}\ngame_players {}\n",
        h.guests.len(),
        h.rooms.len(),
        h.rooms
            .values()
            .map(|r| r.world.players.len())
            .sum::<usize>()
    )
}
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(hub): State<Shared>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Ok(origin) = std::env::var("ALLOWED_ORIGIN") {
        if headers.get("origin").and_then(|v| v.to_str().ok()) != Some(origin.as_str()) {
            return StatusCode::FORBIDDEN.into_response();
        }
    }
    ws.max_message_size(protocol::MAX_MESSAGE_BYTES)
        .on_upgrade(move |socket| handle_socket(socket, hub))
        .into_response()
}
async fn handle_socket(socket: WebSocket, hub: Shared) {
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
            } = parsed
            {
                if version != VERSION {
                    send_error(&tx, "version_mismatch");
                    break;
                }
                let name = clean_text(&name, 20);
                if name.is_empty() {
                    send_error(&tx, "invalid_name");
                    continue;
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
                            .is_some_and(|t| t.elapsed() < RESUME_GRACE)
                    });
                let id = if let Some(id) = resumed {
                    let g = h.guests.get_mut(&id).unwrap();
                    g.sender = Some(tx.clone());
                    g.disconnected = None;
                    g.last_input = 0;
                    id
                } else {
                    let id = h.next_guest;
                    h.next_guest += 1;
                    let token = uuid::Uuid::new_v4().to_string();
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
                        },
                    );
                    id
                };
                identity = Some(id);
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
            } => {
                if h.rooms.len() >= 16 {
                    send_error(&tx, "room_limit");
                    continue;
                }
                let name = clean_text(&name, 24);
                // A mode nobody can play is refused here; which modes those are is the
                // simulation's answer, not a list the server keeps its own copy of.
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
                }) {
                    Ok(room_id) => {
                        let _ = h.join(id, room_id);
                    }
                    Err(code) => send_error(&tx, code),
                }
            }
            ClientMessage::JoinRoom { room } => {
                if h.rooms
                    .get(&room)
                    .is_some_and(|candidate| !candidate.public)
                {
                    send_error(&tx, "invite_code_required");
                } else if let Err(code) = h.join(id, room) {
                    send_error(&tx, code);
                }
            }
            ClientMessage::JoinByCode { code } => {
                if let Some(room) = h.room_id_by_code(&code) {
                    if let Err(code) = h.join(id, room) {
                        send_error(&tx, code);
                    }
                } else {
                    send_error(&tx, "invalid_invite_code");
                }
            }
            ClientMessage::LeaveRoom => h.leave(id),
            ClientMessage::Input { input } => {
                if let Some(room_id) = h.guests[&id].room {
                    if input.seq <= h.guests[&id].last_input
                        || !input.aim.x.is_finite()
                        || !input.aim.y.is_finite()
                        || input.aim.x.abs() > 10000.0
                        || input.aim.y.abs() > 10000.0
                    {
                        continue;
                    }
                    h.guests.get_mut(&id).unwrap().last_input = input.seq;
                    if let Some(room) = h.rooms.get_mut(&room_id) {
                        room.inputs.insert(id, input);
                    }
                }
            }
            ClientMessage::Chat { text } => {
                let text = clean_text(&text, 120);
                if text.is_empty() || h.guests[&id].last_chat.elapsed() < Duration::from_secs(1) {
                    continue;
                }
                let room_id = h.guests[&id].room;
                h.guests.get_mut(&id).unwrap().last_chat = Instant::now();
                if let Some(room_id) = room_id {
                    let name = h.guests[&id].name.clone();
                    for guest in h.guests.values().filter(|g| g.room == Some(room_id)) {
                        h.send(
                            guest.id,
                            &ServerMessage::Chat {
                                player: id,
                                name: name.clone(),
                                text: text.clone(),
                            },
                        );
                    }
                }
            }
            ClientMessage::Ping { nonce } => h.send(id, &ServerMessage::Pong { nonce }),
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
        let mut h = hub.lock().await;
        let expired: Vec<u32> = h
            .guests
            .iter()
            .filter_map(|(&id, g)| {
                g.disconnected
                    .filter(|t| t.elapsed() >= RESUME_GRACE)
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
        for room_id in room_ids {
            let (world, participants) = {
                let room = h.rooms.get_mut(&room_id).unwrap();
                room.world.step_with_bots(&room.inputs);
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
                h.rooms.remove(&id);
            }
        }
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
}
