# Agent instructions

Browser Soldat-like shooter. Rust crates are `game-core` (simulation), `protocol`, and `content`. The server in `apps/api` is authoritative. The Vue 3 / WebGL client in `apps/web` predicts in Wasm.

## Simulation and authority

- One authoritative simulation lives in `game-core`. The server and the Wasm client consume it. Physics, weapons, scoring, and validation stay there.
- The server is authoritative for movement, collision, inventory, damage, objectives, bonuses, teams, rounds, and moderation.
- The Wasm client predicts with that simulation, then reconciles to the server.
- Keep the simulation deterministic at 60 Hz on native Rust and on Wasm.
- Use fixed-width wire fields. Version every protocol, content, and replay format. Reject unknown incompatible versions.

## Unsafe Rust

Never write unsafe Rust. The workspace forbids unsafe code: every crate root has `#![forbid(unsafe_code)]`.

## Bounded parsers

Parse untrusted maps, mods, commands, replays, and messages with bounded allocations, checked arithmetic, normalized paths, explicit limits, and typed errors.

## Assets

Do not import upstream art, sounds, maps, or animations. Original assets only.

Numeric fixtures may record upstream constants with commit, path, and symbol. Do not copy upstream source, configuration files, or binaries into this repository.

`docs/provenance.md` is asset law. It records path, exact commit, author when known, license, modifications, attribution, and redistribution approval.

## Gap list

A gap-list line becomes Present only in the same commit as passing evidence: that line's acceptance tests and the affected regression suites. Never bulk-mark a section Present. Update each feature line on its own, and keep a note where browser constraints intentionally differ.

## Dirty worktree

Preserve unrelated dirty worktree changes. Leave files this task did not change unstaged and unedited.

## Verification

Run from the repository root. Every command must exit zero. The unsafe search must find no Rust unsafe block, function, trait, or impl.

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p game-core --test determinism
wasm-pack test --headless --chrome crates/game-core
npm --prefix apps/web test
npm --prefix apps/web run test:e2e
npm --prefix apps/web run test:parity
npm --prefix apps/web run build
npm run check:gaps
rg -n "unsafe" --glob "*.rs" .
docker compose -f infrastructure/compose.yaml config
npx --prefix apps/web prettier --check .
```

## How to change behavior

Use strict TDD: one behavioral failing test, confirm the intended failure, minimal implementation, passing focused test, refactor, then the verification commands above.

Use OpenSoldat source on the pinned `develop` commit in `docs/provenance.md` as the executable-behavior reference. The community wiki and manual are descriptive context. Record pinned commits in parity fixtures.

## Pointers

- `docs/gaps/2026-10-07-production-parity-backlog.md` — open backlog
- `docs/gaps/gap-list.md` — feature ledger
- `docs/provenance.md` — asset law
