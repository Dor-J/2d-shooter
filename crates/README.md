# Shared Rust crates

`game-core` contains the fixed-tick world simulation: player movement, jet fuel, platform collision, three weapons, projectile hits, team scoring, death, and respawn. It has no network or rendering dependency. The server runs it natively; `wasm-pack` compiles the same crate for browser-side movement prediction.

`protocol` defines version `1` of the JSON WebSocket messages shared with the server. The TypeScript client currently mirrors the relevant message shapes in `apps/web/src/game.ts` and `apps/web/src/App.vue`; keep these aligned when changing the protocol. Incompatible versions are rejected at handshake.

From the repository root:

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
wasm-pack build crates/game-core --target web --release --out-dir ../../apps/web/public/wasm --out-name game_core
```

The world advances at 60 ticks per second. Rules and current tuning values are documented in `docs/gameplay.md`. Deterministic replay, platform landing, weapon cooldown, and malformed protocol messages have unit tests. The current map uses rectangular platforms; polygon maps and fuller Soldat-style content remain future work.
