# Web client

Vue 3 and TypeScript provide the guest-name screen, room browser, match HUD, chat, and touch controls. `src/game.ts` owns the WebGL2 canvas, keyboard/pointer input, snapshot rendering, and optional Rust/Wasm prediction. The browser connects to the Rust server through `/ws`; Vite proxies that path to `127.0.0.1:3000` during development.

## Run locally

From this directory:

```sh
bun install
bun run dev
```

Open the URL Vite prints, normally `http://localhost:5173`. Start the server from the repository root with `cargo run -p server` before connecting. Open a second browser tab with a different guest name to test multiplayer. If WebGL2 is unavailable, the match view reports an error.

To enable browser-side prediction, run this from the repository root before starting Vite:

```sh
wasm-pack build crates/game-core --target web --release --out-dir ../../apps/web/public/wasm --out-name game_core
```

The generated `public/wasm` files are ignored by Git. Without them, the client still displays authoritative server snapshots, but local movement prediction is unavailable.

## Checks

```sh
bun run lint
bun run build
```

Husky installs repository-level hooks during `bun install` when `.git` is present: pre-commit runs lint; pre-push runs build. `bun run smoke` checks a running server by connecting two guest sockets and sending movement input. It does not test browser rendering.

## Controls and limits

Desktop: A/D or arrows to move, Space/W to jump, Shift for jet, mouse to aim/fire, 1–3 to select a weapon. On touch devices, on-screen buttons provide movement, jump, jet, and fire; drag on the arena to aim. Current touch controls are basic and have not been validated across mobile devices. The WebGL view uses procedural placeholder shapes rather than imported Soldat art.
