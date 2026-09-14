# Rust game server

The `server` Cargo package exposes `/ws` for versioned real-time messages, `/health` for liveness, and `/metrics` for basic counts. It owns rooms, guest sessions, authoritative match state, chat rate limiting, and the 60 Hz simulation loop. Match state is in memory and is lost on restart.

## Run locally

From the repository root:

```sh
cargo run -p server
```

The default bind address is `127.0.0.1:3000`. Set `BIND_ADDR` to change it. Set `ALLOWED_ORIGIN` to an exact browser origin when exposing the server; the production Compose configuration sets it from `PUBLIC_ORIGIN`. TLS terminates at Caddy, which proxies `/ws` to this service.

## Protocol and behavior

After opening the WebSocket, a client must send `hello` with protocol version `1`, guest name, and an optional resume token. The server responds with `welcome` and a room list. The client may create or join a room, then send numbered input messages. The server validates input and broadcasts snapshots every three simulation ticks (20 Hz). See `crates/protocol/src/lib.rs` for the exact JSON message types.

The default Arena room supports deathmatch. Guests can create deathmatch or team-deathmatch rooms with up to 16 players. A disconnected guest has 20 seconds to reconnect with its token. There are no persistent accounts, statistics, or moderation database yet.

## Checks

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

With the server running, `npm --prefix apps/web run smoke` checks a two-guest join and movement update. This is a development foundation; the server still needs sustained load, abuse, and restart testing before public deployment.
