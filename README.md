# Arena

A browser-based 2D multiplayer shooter inspired by Soldat's pace and presentation. The server is authoritative Rust; the same Rust simulation can compile to WebAssembly for local prediction. Vue 3 handles the lobby and HUD, and WebGL2 renders play.

Controls are fully rebindable across keyboard, mouse, and gamepad, and they are stored in control profiles you can create, export, and import. Open **Controls** in the header to rebind anything, set mouse sensitivity, or switch a hold control to a toggle. The defaults are A/D to move, Space to jump, S to crouch, X to go prone, C to roll, Shift for the jetpack, R to reload, E or the right mouse button for grenades, Tab for the scoreboard, and 1–0 for weapons.

On touch devices, the match uses a player-following camera. Drag the left pad to move and push it upward to jump, drag the right pad to aim and fire, and use the on-screen buttons for the jetpack, grenades, stance, roll, reload, weapon switch, weapon and flag throws, the scoreboard, and team chat. Both portrait and landscape layouts honor device safe areas, and a cancelled touch releases everything it was holding.

Component guides: [web client](apps/web/README.md), [Rust server](apps/api/README.md), [shared crates](crates/README.md), and [deployment](infrastructure/README.md).

## Local development

Install Rust 1.88 with `wasm32-unknown-unknown`, Bun, and `wasm-pack`. Run `bun install` in `apps/web`. In separate terminals, run `cargo run -p server` and `bun run dev` in `apps/web`; open the Vite URL in two tabs. The browser uses authoritative snapshots without Wasm, but prediction requires `wasm-pack build crates/game-core --target web --release --out-dir ../../apps/web/public/wasm --out-name game_core` from the repository root. Run `cargo test --workspace` and `bun run build` to check the code.

## Production container

Set `SITE_ADDRESS` to the public domain and `PUBLIC_ORIGIN` to its `https://` origin (both required off localhost), then run `docker compose -f infrastructure/compose.yaml up --build -d`. Caddy terminates TLS and forwards `/ws`, `/health`, and `/ready`. `/metrics` stays on the docker network. Record image digests with `node scripts/record-image-digests.mjs` and keep the previous digest for rollback. Match state is in memory; restarting ends active matches. `/DRAIN` then restart is the operator path.

## Status

This repository provides a playable foundation, not a production-complete Soldat remake. The first arena, two modes, ten normal-mode primary weapons, four secondary weapons, and frag grenades work at the simulation layer. Production release still requires exact mechanics parity, more content, network/browser/device testing, asset provenance review, and operational hardening. See `docs/gameplay.md` and `docs/provenance.md`.
