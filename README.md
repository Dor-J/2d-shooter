# Arena

A browser-based 2D multiplayer shooter inspired by Soldat's pace and presentation. The server is authoritative Rust; the same Rust simulation can compile to WebAssembly for local prediction. Vue 3 handles the lobby and HUD, and WebGL2 renders play.

On touch devices, the match uses a player-following camera. Drag the left pad to move and push it upward to jump, drag the right pad to aim and fire, and hold the center button for the jetpack. The weapon selector and score/chat panel stay outside the thumb zones. Both portrait and landscape layouts honor device safe areas.

Component guides: [web client](apps/web/README.md), [Rust server](apps/api/README.md), [shared crates](crates/README.md), and [deployment](infrastructure/README.md).

## Local development

Install Rust 1.88 with `wasm32-unknown-unknown`, Bun, and `wasm-pack`. Run `bun install` in `apps/web`. In separate terminals, run `cargo run -p server` and `bun run dev` in `apps/web`; open the Vite URL in two tabs. The browser uses authoritative snapshots without Wasm, but prediction requires `wasm-pack build crates/game-core --target web --release --out-dir ../../apps/web/public/wasm --out-name game_core` from the repository root. Run `cargo test --workspace` and `bun run build` to check the code.

## Production container

Set `SITE_ADDRESS` to the public domain and `PUBLIC_ORIGIN` to its `https://` origin, then run `docker compose -f infrastructure/compose.yaml up --build -d`. Caddy terminates TLS and forwards `/ws` to the game server. `/health` and `/metrics` are available for monitoring. Deploy with a pinned image digest and keep the previous digest for rollback. The current server stores match state only in memory; restarting it ends active matches.

## Status

This repository provides a playable foundation, not a production-complete Soldat remake. The first arena, two modes, ten normal-mode primary weapons, four secondary weapons, and frag grenades work at the simulation layer. Production release still requires exact mechanics parity, more content, network/browser/device testing, asset provenance review, and operational hardening. See `docs/gameplay.md` and `docs/provenance.md`.
