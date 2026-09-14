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
use game_core::{Input, World, TICK_RATE};
use protocol::{ClientMessage, RoomInfo, ServerMessage, VERSION};
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
    world: World,
    inputs: BTreeMap<u32, Input>,
}
struct Hub {
    guests: HashMap<u32, Guest>,
    tokens: HashMap<String, u32>,
    rooms: BTreeMap<u32, Room>,
    next_guest: u32,
    next_room: u32,
}
impl Hub {
    fn new() -> Self {
        let mut hub = Self {
            guests: HashMap::new(),
            tokens: HashMap::new(),
            rooms: BTreeMap::new(),
            next_guest: 1,
            next_room: 2,
        };
        hub.rooms.insert(
            1,
            Room {
                id: 1,
                name: "Arena".into(),
                mode: "deathmatch".into(),
                world: World::new("deathmatch"),
                inputs: BTreeMap::new(),
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
            .map(|r| RoomInfo {
                id: r.id,
                name: r.name.clone(),
                mode: r.mode.clone(),
                players: r.world.players.len(),
                capacity: ROOM_CAPACITY,
            })
            .collect()
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
        self.guests.get_mut(&id).unwrap().room = Some(room_id);
        self.send(
            id,
            &ServerMessage::Joined {
                room: room_id,
                player: id,
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
                if let Some(room_id) = h.guests[&id].room {
                    h.send(
                        id,
                        &ServerMessage::Joined {
                            room: room_id,
                            player: id,
                        },
                    );
                }
            } else {
                send_error(&tx, "hello_required");
            }
            continue;
        }
        let id = identity.unwrap();
        match parsed {
            ClientMessage::Hello { .. } => send_error(&tx, "already_connected"),
            ClientMessage::Rooms => h.send(
                id,
                &ServerMessage::Rooms {
                    rooms: h.room_list(),
                },
            ),
            ClientMessage::CreateRoom { name, mode } => {
                if h.rooms.len() >= 16 {
                    send_error(&tx, "room_limit");
                    continue;
                }
                let name = clean_text(&name, 24);
                if name.is_empty() || (mode != "team" && mode != "deathmatch") {
                    send_error(&tx, "invalid_room");
                    continue;
                }
                let room_id = h.next_room;
                h.next_room += 1;
                h.rooms.insert(
                    room_id,
                    Room {
                        id: room_id,
                        name,
                        mode: mode.clone(),
                        world: World::new(&mode),
                        inputs: BTreeMap::new(),
                    },
                );
                let _ = h.join(id, room_id);
            }
            ClientMessage::JoinRoom { room } => {
                if let Err(code) = h.join(id, room) {
                    send_error(&tx, code);
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
                room.world.step(&room.inputs);
                (
                    room.world.clone(),
                    room.world.players.keys().copied().collect::<Vec<_>>(),
                )
            };
            if world.tick % 3 == 0 {
                for id in participants {
                    h.send(
                        id,
                        &ServerMessage::Snapshot {
                            room: room_id,
                            world: world.clone(),
                        },
                    );
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
