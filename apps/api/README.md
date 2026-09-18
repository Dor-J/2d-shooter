# Rust game server

The `server` Cargo package exposes `/ws` for versioned real-time messages, `/health` for liveness, `/ready` for readiness, and `/metrics` for guest/room/player/tick/error counts. It owns rooms, guest sessions, authoritative match state, chat rate limiting, and the 60 Hz simulation loop. Match worlds stay in memory. Bans, config, and room leases persist to `DATA_DIR/store.json` when that directory is set.

## Run locally

From the repository root:

```sh
cargo run -p server
```

The default bind address is `127.0.0.1:3000`. Set `BIND_ADDR` to change it. Set `ALLOWED_ORIGIN` to an exact browser origin when exposing the server; the production Compose configuration sets it from `PUBLIC_ORIGIN`. An empty `ALLOWED_ORIGIN` skips the check (local `cargo run` only). Set `TRUSTED_PROXIES` to `private` or a comma list of proxy IPs so bans and rate limits use the rightmost `X-Forwarded-For` hop; otherwise the TCP peer is used. TLS terminates at Caddy, which proxies `/ws`, `/health`, and `/ready`. `/metrics` is not published on the public ports. `/DRAIN` and `/UNDRAIN` are admin commands; SIGTERM flushes `store.json`.

## Protocol and behavior

After opening the WebSocket, a client must send `hello` with protocol version `15`, guest name, and an optional resume token. The server responds with `welcome` and a room list. The client may create or join a room, then send numbered input messages. The server validates input and broadcasts snapshots every three simulation ticks (20 Hz). See `crates/protocol/src/lib.rs` for the exact JSON message types.

The default Arena room supports deathmatch. Guests can create the implemented modes with up to 16 players. A disconnected guest has 20 seconds to reconnect with its token. Accounts and clan statistics stay optional; bans and config persist when `DATA_DIR` is set.

## Checks

```sh
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

With the server running, `bun --cwd apps/web run smoke` checks a two-guest join and movement update. This is a development foundation; the server still needs sustained load, abuse, and restart testing before public deployment.
