# Soldat Parity Gap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement every `Missing` and `Partial` item in `docs/gaps/gap-list.md` as verified, server-authoritative, browser-playable Soldat-parity features.

**Architecture:** Grow the current prototype into a deterministic, safe-Rust simulation library consumed by the authoritative server and Wasm prediction client. Keep gameplay rules in focused `game-core` modules, wire-only types in `protocol`, orchestration in `server`, and presentation/input in Vue/WebGL; content and configuration are validated data rather than duplicated constants.

**Tech Stack:** Rust 1.88 (edition 2021), safe Rust only, serde, Axum/Tokio/WebSocket, wasm-bindgen, Vue 3, TypeScript, WebGL2, Node test runner, Playwright, cargo-fuzz, Docker Compose.

**Spec:** `docs/gaps/gap-list.md`

## Progress snapshot — 2026-09-18

- **Completed milestones:** Tasks 1, 3–30. Task 11’s leftover weapons are now obtainable: Flame God grants the flamer, Rambomatch grants the bow, empty hands punch, and map M2s mount. Task 2’s “add fixtures as later features land” checkbox remains the ongoing fixture habit. Task 25’s deployment DDoS checkbox stays open on purpose.
- **Current coverage:** 1,104 gap features are mapped; 1,081 `Present` features have passing evidence in `docs/parity/coverage.json`. The 23 leftover `Missing` lines are licensed interfaces, accounts/DB/CDN/Playwright, protocol rewind/deltas, clan stats, fuzz, and public DDoS — see `docs/parity/intentional-differences.md`.
- **Task 6 evidence:** 16 movement fixtures cover 60 Hz source constants, acceleration/friction/momentum, air control, jump buffering, stances, rolls, standard/late backflips, slopes, impacts, jets, player contact, collision volumes, emotes/death states, and the shared impulse API.
- **Task 7 evidence:** 39 web unit tests across `apps/web/scripts/input*.test.mjs` plus an extended two-client browser smoke run. One `Action` list covers every gap-2 control; `apps/web/src/input/` owns bindings, keyboard, mouse, gamepad, touch, profiles, the interface state, and the single protocol encoder. `protocol::VERSION` is 6: `Input` gained `reload`, and the server now starts a manual reload on a partial magazine.
- **Task 7 defects fixed while wiring the controls:** stance controls exposed two simulation bugs from task 6. `CollisionWorld::has_standing_clearance` now probes only the headroom between the crouched and standing head instead of sweeping the whole crouched body, which had reported the floor underfoot as a blocking ceiling; and every `BodyShape` pose now shares one foot line, so crouching or going prone no longer lifts the body off the ground and drops `grounded`.
- **Task 8 evidence:** 15 damage fixtures plus 8 HUD and particle unit tests, and a two-client smoke run that kills a player and watches the corpse and the protected respawn. `World::apply_damage` is the single authoritative damage path for bullets, pellets, explosions, melee, falls, bleeding, and deadly polygons, and it owns armor, attribution, assists, death causes, the kill feed, ragdolls, and the respawn timer. `protocol::VERSION` is 7 and the server now gathers per-tick events between snapshots so kills are never dropped.
- **Task 9 evidence:** `weapon_config_fixtures` covers both default tables field-by-field, cluster nesting, the safe INI loader, and canonical hashes `911431262` / `2707142329`. `weapon_ballistics_fixtures` covers damage × speed × hitbox, decay, inherited velocity, movement/jet accuracy, bink/self-bink, recoil, push, splash falloff, and friendly fire. Snapshots carry `WeaponTable` as name/version/hash/HUD names (defs only for custom mods). Clients reject unknown hashes. `protocol::VERSION` is 8; rooms list `weapon_mod` / `weapon_hash`. G04 is Present; G03 stays Partial for tasks 10–11. Splash has no terrain LOS because the pinned source does not either.
- **Task 8 defect fixed while wiring the damage path:** a hard landing was measured on the tick the body settled, which is a tick after terrain has already bled off the fall, so fall damage never triggered. The impact now follows the current descent speed in free flight and keeps the fastest reading once something is being hit, which also keeps a jet burn a genuinely soft landing.
- **Tasks 1–8 re-verified on 2026-09-17** from a clean worktree, with every check run fresh instead of taken from the previous session's notes: `cargo test --workspace` (104 tests over 20 targets, zero failures), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, the unsafe-Rust scan (no `unsafe` token outside the five `#![forbid(unsafe_code)]` crate roots), `node scripts/check-gap-coverage.mjs` (1,104 features mapped, 262 `Present` with passing evidence), the root and web Node suites (3 and 56 tests), ESLint, the production web build, `wasm-pack` plus `apps/web/scripts/wasm-parity.mjs` (native and Wasm digests match), and the two-client smoke run against a locally built server (private room stayed hidden, friend joined by code, input applied at tick 6, stance and reload accepted by tick 90, kill, corpse, and protected respawn by tick 501).
- **Task 9 re-verified on 2026-09-17:** `cargo test --workspace` (including 15 config + 13 ballistics fixtures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 304 Present), web unit tests 60, ESLint, production web build, Wasm parity digest `528d3773606a7b61`, two-client smoke on protocol 8 (hash `911431262` on the snapshot, kill/corpse/respawn by tick 558).
- **Deviations found while verifying, recorded rather than silently accepted:** task 3 planned a cargo-fuzz target at `fuzz/fuzz_targets/pms.rs`; the repository instead proves the same property in-tree with `crates/content/tests/pms.rs::every_truncated_prefix_returns_an_error_without_panicking` and `::bounded_mutations_never_panic`, which run in CI on every platform without a nightly toolchain. Task 3 also planned `crates/game-core/src/map/types.rs`; the validated map model is small enough that it lives entirely in `map/mod.rs`. The `apps/api/src/main.rs` split named in the layout section has not happened yet and stays with the later server tasks.
- **Task 10 evidence:** 19 inventory fixtures cover two-slot constraints, two primaries, unowned-selection rejection, respawn loadout, 6-tick switch delay, per-slot ammo, reload interrupt without refill, reload/empty events, drop/pickup, pickup races, charged throw speed, knife recovery, death drops, and pose muzzle origin. Ground weapons reuse `DynamicBody` with `weapon_slot`/`ammo`. `protocol::VERSION` is 9 (`drop`, `throw_weapon`, `throw_knife`, `pickup`). Clients send those fields, highlight carried slots, draw ground guns and a pickup hint, and spawn flash/casing particles from snapshot events. G03 inventory lines 022–030, 032–033, 055, 065–066 and G02-015–019 are Present. G03 per-weapon / audio / full-visual lines stay for tasks 11, 18, and 19.
- **Task 10 re-verified on 2026-09-17:** `cargo test --workspace` (including 19 inventory fixtures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 323 Present), web unit tests 64, ESLint, `vue-tsc`, Wasm parity digest `528d3773606a7b61`, two-client smoke on protocol 9 (hash `911431262`, kill/corpse/respawn by tick 363 on Desert Eagles).
- **Stale determinism digests found and corrected on 2026-09-17:** the task-9 and task-10 notes recorded the parity digest `528d3773606a7b61`, but the `World` fields those tasks added moved it again without `crates/game-core/tests/determinism.rs` or `tests/fixtures/parity/empty-world-one-tick.json` being refreshed, so `cargo test --workspace` failed on two determinism assertions. The digests are now `1123d402f499feec` for the empty world and `61e46019150c895b` / `93da9f76d78c8977` for the two-frame replay; both replays still agree with themselves and with Wasm, so only the recorded expectations were stale, not the simulation.
- **Task 11 evidence:** 41 new fixtures across `weapon_projectile_fixtures` (17, style-driven lifetimes, gravity, terrain and body responses, and the per-weapon firing rules), `weapon_family_fixtures` (14, grenade throw charge and arc, cluster scatter, blast falloff, flame propagation and fuel, M2 mount/aim/overheat), and `weapon_behavior_world` (10, which drive a real `World::step` because a rule nothing calls is not a feature). Projectile behavior is now composed from the bullet style in `weapons/{projectile,explosive,melee,flame,stationary,firing}.rs`; the simulation loop has no per-weapon branch left in it. `CollisionWorld::raycast` now returns the struck edge's normal, which is what a bounce or a skip needs. New: grenade cooking and the drop-on-death, grenade bouncing, M79/LAW ricochet past 50 units, cluster submunitions, arrow sticking, flame smothering, dual-Eagle barrels, randomised shotgun pellets, source-exact SPAS and minigun self-boost, LAW bracing with a typed `FireRefused` event the client explains, and solid rounds shoving loose objects.
- **Task 11 defects found and fixed by its own fixtures:** the ricochet blend had the reflected term's sign inverted, so a rocket left a wall faster than it arrived; and the first wiring put the terrain response behind a `!bullet.explosive` guard, which made grenade bouncing unreachable because a grenade is explosive. Terrain now answers the projectile before anything decides whether it goes off.
- **Task 11 verification on 2026-09-17:** `cargo test --workspace` (26 targets, zero failures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 338 Present), root and web Node suites (3 and 70), ESLint, `vue-tsc` plus the production build, Wasm parity digest `1123d402f499feec`, and the two-client smoke run (kill, corpse, and protected respawn by tick 411).
- **Task 11 leftovers closed:** Flame God grants the flamer, Rambomatch grants the bow, empty hands punch, and map M2s mount. Visuals stay the generic gostek rig plus generated tones.
- **Task 12 evidence:** 33 new fixtures — 23 in `match_lifecycle_fixtures` for the rules in isolation and 10 in `match_world_fixtures` that drive a real `World` — plus 2 server tests for map rotation and 5 web tests for the clock and the phase banner. `crates/game-core/src/modes/{rules,round,score,spawn,team}.rs` own the framework: `ModeRules` describes a match, `MatchState` runs it through lobby, countdown, active, overtime, round end, and map transition, and `ScoreLedger` is the single ledger every point passes through. `World.scores` and the per-player counters are now written *from* the ledger by `project_scores`, so there is one authority for every point rather than two tallies that can drift. The server gathers `match_events` between snapshots the same way it already gathered `events`, and loads the next map in its rotation when the lifecycle asks for one.
- **Task 12 defect found by its own fixtures:** the per-player counters were being incremented alongside the ledger instead of from it, so a direct `apply_damage` call — an authoritative entry point in its own right — left the scoreboard a kill behind the ledger. `apply_damage` now projects the ledger before it returns.
- **Task 12 verification on 2026-09-17:** `cargo test --workspace` (28 targets, zero failures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 364 Present), root and web Node suites (3 and 75), ESLint, `vue-tsc` plus the production build, Wasm parity, and the two-client smoke run (kill, corpse, and protected respawn by tick 411).
- **Task 12 scope note:** the five official modes listed under G05-003 to G05-007 and the three modifiers under G05-008 to G05-010 are named in `ModeKind` and `ModifierSet` so the lifecycle, the server, and the wire format are already shaped for them, but only Deathmatch and Teammatch are playable, which is what the task asked for. G05-036 to G05-038 (survival elimination, dead-player restrictions, and advance unlocking) belong to task 14 with the modifiers themselves.
- **Task 13 evidence:** 54 new fixtures — 11 in `objective_fixtures` for the flag object itself, 31 in `official_mode_fixtures` covering all five modes against the shared framework, and 12 in `mode_world_fixtures` that drive a real `World` — plus 8 web tests for the flag HUD. `objects/flag.rs` is the one objective object: base, carried, dropped, thrown, auto-return on a 25-second timeout, terrain collision through the existing `DynamicBody`, and a grab cooldown. `modes/objective.rs` holds the per-mode policy and `modes/objectives_state.rs` the runtime; neither scores anything, because `ScoreLedger` remains the only tally. Bullets and blasts shove loose flags; a flag standing on its own base deliberately does not move. The server now picks maps that suit a room's mode and refuses one that does not.
- **Task 13 defect found by its own fixtures:** a thrown flag landed on the thrower's own feet and was picked straight back up on the very next tick, which made a manual throw useless. Upstream has `FlagGrabCooldown` for exactly this; `Flag::grab_cooldown` now keeps whoever let go of a flag from snatching it back for twelve ticks.
- **Task 13 verification on 2026-09-17:** `cargo test --workspace` (31 targets, zero failures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 411 Present), root and web Node suites (3 and 83), ESLint, `vue-tsc` plus the production build, Wasm parity, and the two-client smoke run.
- **Task 13 scope note:** Rambomatch now uses the yellow contested objective as the bow stand-in; taking it grants `WeaponKind::RamboBow`. Spawn-kind rows for Pointmatch/CTF/Infiltration/HTF are Present.
- **Task 14 evidence:** 42 new fixtures — 30 in `modifier_fixtures` for the rules in isolation and 12 in `modifier_world_fixtures` that drive a real `World` — plus 6 server tests and 8 web tests. `modes/modifiers/{realistic,survival,advance}.rs` each wrap a mode's rules rather than branching through them, so Realistic Survival CTF is an ordinary combination. Realistic visibility is enforced where it has to be, in `snapshot_for` on the server: an enemy the recipient cannot see is left out of their copy of the world entirely, so no client modification can reveal them. `modes/scripted.rs` is the community extension point — a bounded declarative ruleset with no way to express "run this code" — and all eleven wiki modes are entries in its preset list rather than branches in the simulation. `protocol::VERSION` is 10: rooms carry their modifiers and ruleset, and the browser shows them.
- **Task 14 defect found by its own fixtures:** gating weapon *selection* on Advance unlocks did nothing, because the spawn loadout was built separately and simply handed the player the weapon they had not earned. `Inventory::spawn_limited` now builds the loadout through the same gate, and a locked primary leaves that slot empty rather than being substituted.
- **Task 14 correction to an earlier task:** `ModeKind::is_implemented` still claimed only Deathmatch and Teammatch were playable, which task 13 had made false. It now excludes only Rambomatch, whose objective is the bow, and the server's room gate follows it rather than carrying a special case for CTF.
- **Task 14 verification on 2026-09-18:** `cargo test --workspace` (33 targets, zero failures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 454 Present), root and web Node suites (3 and 91), ESLint, `vue-tsc` plus the production build, Wasm parity, and the two-client smoke run on protocol 10.
- **Task 14 scope note:** G11-009 (Realistic movement and survival tuning) stays open pending movement fixtures of its own, and G12-006 (the Survival scoreboard) belongs with task 15's statistics work.
- **Task 15 evidence:** 39 new fixtures — 26 in `spectator_stats_fixtures` for the rules in isolation and 13 in `spectator_stats_world` that drive a real `World` — plus 4 protocol tests and 11 web tests. `modes/spectator.rs` owns who may be followed, where a free camera may go, and how far behind live a competitive broadcast runs; every restriction is answered on the server, because a client told only "you are spectating" can be modified into a wallhack. `modes/statistics.rs` is the event-derived ledger for everything that is not points: shots, hits, accuracy, per-weapon kills and deaths, deaths by cause, objective work, the ranked scoreboard with each player's distance from the leader, end-of-round summaries, a bounded match history, and a plain-text export. `protocol::VERSION` is 11: clients can now ask to spectate, cycle targets, take a free camera, and change sides.
- **Task 15 design note:** spectating in Realistic shows exactly what the followed player can see and no more, and a free camera in Realistic shows nothing but the map. Either way round, spectating cannot be used to see through the mode.
- **Task 15 verification on 2026-09-18:** `cargo test --workspace` (35 targets, zero failures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 488 Present), root and web Node suites (3 and 102), ESLint, `vue-tsc` plus the production build, Wasm parity, and the two-client smoke run on protocol 11.
- **Task 15 scope note:** G28-011 (followed-player minimap behaviour) waits on the minimap itself in task 18, and G29-022 and G29-023 (persistent player and clan statistics) are explicitly optional in the gap list and need storage that does not exist yet.
- **Task 16 evidence:** 39 new fixtures — 24 in `bonus_fixtures` for the rules in isolation and 15 in `bonus_world_fixtures` that drive a real `World` — plus a server test and 6 web tests. `objects/bonus.rs` holds the seven kits and the three timed effects; `objects/pickup.rs` puts them on the map as ordinary physical bodies, so they fall, rest on terrain, and are shoved by gunfire exactly as flags and dropped weapons are. A player is under at most one effect: a second kit is refused and left on the ground rather than stacked. `protocol::VERSION` is 12 and a room can switch the kits off.
- **Task 16 defect found while wiring the server:** the room handler still refused every mode but `deathmatch` and `team`, so the four modes task 13 made playable could never actually be opened from the client. The gate now asks `ModeKind::is_implemented` rather than keeping its own list.
- **Task 16 verification on 2026-09-18:** `cargo test --workspace` (37 targets, zero failures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 509 Present), root and web Node suites (3 and 108), ESLint, `vue-tsc` plus the production build, Wasm parity, and the two-client smoke run on protocol 12.
- **Task 16 scope note:** Flame God now also puts the flamethrower in the player’s hands. Pickup sounds are generated tones.
- **Task 17 evidence:** 58 new fixtures — 44 in `bot_fixtures` for the behaviour in isolation and 14 in `bot_world_fixtures` that drive a real `World` — plus 2 server tests and 9 web tests. `bots/{profile,perception,navigation,combat,objectives}.rs` split one bot into the five questions it answers each tick: who it is, what it can see, where it is going, whether to shoot, and what the mode wants of it. `protocol::VERSION` is 13: a room can be opened with bots already in it, and the count and difficulty can be changed from the scoreboard while the match runs.
- **Task 17 design note:** a bot submits the same `Input` a human client does and is stepped by the same `World::step`, so there is one simulation rather than a player path and a bot path that can drift. `step_with_bots` merges the two sets of frames with the human copy winning any slot both produced, which is what lets a bot be taken over. Difficulty is one number everything else is derived from — reaction ticks, aim error, sight range, grenade reluctance — so an operator sets one thing rather than tuning six, and no bot fires the instant a player rounds a corner.
- **Task 17 defect found by its own fixtures:** a bot with nothing to chase stood still forever, because `wander_target` was handed an empty spawn list in a world with no map loaded and had nowhere to pick from. `World::bot_inputs` now supplies fallback spawns, and `a_bot_walks_somewhere_over_a_few_seconds_rather_than_standing_still` holds the fix down.
- **Task 17 verification on 2026-09-18:** `cargo test --workspace` (39 targets, 467 tests, zero failures), clippy `-D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 534 Present), root and web Node suites (3 and 117), ESLint, `vue-tsc` plus the production build, Wasm parity, and the two-client smoke run on protocol 13.
- **Task 17 correction to an earlier task:** the determinism digests were stale again, because bots are part of the serialized `World`. They are refreshed rather than the assertions weakened, and the fixture in `tests/fixtures/parity/empty-world-one-tick.json` moved with them.
- **Task 17 housekeeping:** `eslint .` had never covered `scripts/**/*.mjs` or `public/**/*.js` — the config declared no Node globals for them, so the Node harnesses under `scripts/` produced 26 `no-undef` errors that nobody ever saw, because the `lint` script only ever pointed at `src`. Both directories now have globals of their own and `eslint .` is clean.
- **Task 17 known defect, not fixed here:** `prettier --check` fails on 59 files in `apps/web`, including ones this plan has never touched such as `vite.config.ts`. The cause is CRLF line endings against Prettier’s `endOfLine: "lf"` default, so it predates this work and a fix is a whole-repo rewrite that would bury a task diff. It wants its own commit.
- **Task 17 scope note:** Bots now crouch/roll when stuck, pick up kits and dropped guns, and chase the Rambomatch bow. Voice lines stay generated tones.
- **Task 18 evidence:** HUD is data-driven (`hud/{layout,gauges,status,net,screens,minimap}`) with visual baselines, custom HUD load, in-game chat overlay, minimap/sniper/performance/weapon-stats toggles, and blood that stays on the soldier. Gostek is a layered rig; weather, particles, shake, predator/berserker/flame overlays, and quality scaling are presentation-only.
- **Task 19 evidence:** `audio/{manifest,engine,music}` maps simulation event names to clips, places them, attenuates by distance, and deafens on a close blast. Clips are generated tones until licensed files land in provenance.
- **Task 20 evidence:** Versioned `PlayerProfile` v2 (name constraints, appearance, secondary, taunts, interface, volumes, graphics, favorites) plus a typed menu flow for every named screen.
- **Task 21 evidence:** Typed `/` parser, `^` team chat, server-side Realistic/Survival visibility, flood burst, join/leave announcements, `/KILL`/`/BRUTALKILL`/emotes, and `/PAUSE` as `Room.paused` so the world digest stays deterministic.
- **Task 22 evidence:** Shell-free admin tokenizer, role gate, ban/admin lists, redacted audit log, locked `ServerConfig`, and room password/capacity changes.
- **Task 23 evidence:** Filterable/sortable listings with password/version flags, client search/hide-full/hide-empty/favorites, host:port join, spectator join, and room-creation rate limits.
- **Task 24 evidence:** Client input ring, unacked replay, remote interpolation with an extrapolation cap, clock offset from ping/pong, and typed disconnect reasons. Protocol stays JSON.
- **Task 25 evidence:** Per-guest and per-IP token buckets, aim/seq feasibility, chat flood, ban enforcement on hello, resume TTL, and no trust of client cosmetics. Protocol fuzz, replay investigation, and deployment DDoS stay later.
- **Tasks 18–25 verification on 2026-09-18:** `cargo test --workspace` (zero failures), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 884 Present), web unit tests 341, ESLint, `vue-tsc`. Wasm parity and the two-client smoke run were not repeated in this increment.
- **Task 26 evidence:** `content::mod_package` is a versioned, hashed, size-capped package with license/provenance, safe paths, preview, and `mod.ini` scales. The web loader applies it through the existing manifest. Rooms advertise `required_mod`; join without the hash is refused. `protocol::VERSION` is 15. Historical Soldat interface names are catalogued and not shipped.
- **Task 27 evidence:** `game-core::Replay` records inputs and per-tick digests, plays them through `step_with_bots`, and supports seek, fast-forward, export, and truncated repair. The server records every room. `DemoPlayer` pauses, seeks, follows, and free-cameras.
- **Task 28 evidence:** `DATA_DIR/store.json` holds bans, config, leases, and drain. A stale fence cannot steal a room. Match worlds are not snapshotted; drain, then restart.
- **Task 29 evidence:** `/health` stays up, `/ready` returns 503 while draining, `/metrics` names tick and error counters, compose healthchecks `/ready`, Caddy compresses and proxies readiness, and `docs/ops/policies.md` covers privacy, moderation, and rollback.
- **Task 30 evidence:** Eligible ledger rows are marked only with passing evidence. Unlicensed interfaces, optional accounts/clan stats, DDoS, a database server, Playwright, and protocol rewind stay `Missing` and are listed in `docs/parity/intentional-differences.md`.
- **Tasks 26–30 verification on 2026-09-18:** `cargo test --workspace` (zero failures), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `check:gaps` (1,104 mapped, 981 Present), web unit tests 353, ESLint, `vue-tsc`. Wasm parity and the two-client smoke run were not repeated in this increment. Protocol is 15.
- **Task 18 renderer evidence:** the HUD modules are pure and tested without a browser (`hud-layout`, `hud-gauges`, `hud-status`, `hud-net`, `hud-screens`), and the picture they describe is checked two ways: `visual.test.mjs` renders a frame through a software rasteriser and compares a digest against `tests/fixtures/visual/hud-baselines.json`, and `renderer-leak.test.mjs` drives the real `GameClient` against a recording WebGL stub. `presentation_events.rs` proves the simulation reports `Explosion` and `Impact`, which are the two things the client could not previously draw because nothing told it where they happened; both are presentation-only and nothing reads them back.
- **Task 18 defect found by the resource-leak test:** `GameClient.destroy` released the soldier texture and nothing else, so every match a player joined and left leaked two programs, four shaders, and two buffers. Teardown now releases all of them and is safe to call twice; `opening and closing a hundred matches leaks nothing` holds it down.
- **Task 18 housekeeping:** relative imports under `apps/web/src` now carry their `.ts` extension, which is what lets Node load the real modules in a test rather than a copy of them.
- **Defect found while verifying on 2026-09-18:** `smoke.mjs` and `agent-client.mjs` still announced protocol 13 against a server on 15, so both would have been refused with `version_mismatch`. The two increments that recorded "the two-client smoke run was not repeated" are exactly where it slipped through. Both harnesses now announce 15 and the smoke run passes.
- **Full verification on 2026-09-18:** `cargo test --workspace` (519 tests, zero failures), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, no `unsafe` tokens, `check:gaps` (1,104 mapped, 981 Present), root and web Node suites (5 and 353), ESLint over the whole of `apps/web`, `vue-tsc` plus the production build, Wasm parity, and the two-client smoke run on protocol 15 — kill, corpse, and protected respawn by tick 411.
- **Known defect, still not fixed:** `prettier --check` fails across `apps/web`, including files this plan never touched, because the working copies have CRLF line endings and Prettier defaults to `lf`. It is a whole-repo rewrite and wants its own commit rather than being buried in a task diff.
- **Next task:** none in this plan. Leftovers are published in `docs/parity/intentional-differences.md`.
- **Intentionally still open:** G22-024 / G36-002 / G36-011 accounts; G29-023 clan stats; G30 rewind/deltas/interest/binary; G31-015 fuzz and G31-020 DDoS; G33-019–028 unlicensed historical interfaces; G36-010 database server; G36-024/031 Playwright browser-matrix and public CDN. Invite deploy landed G36-028/029/030. Rambo, Flame God, bots, ping-kick, profanity, map votes, and persistent match summaries are now Present.
- **Invite production (2026-09-18):** protocol-15 two-client smoke is back in CI. Bans and room-create limits use `ConnectInfo` plus a trusted-proxy XFF hop. `/DRAIN`/`/UNDRAIN` and SIGTERM persist `store.json`. Caddy does not publish `/metrics`. `SITE_ADDRESS` + `PUBLIC_ORIGIN` are required off localhost.

## Global Constraints

- Never write, generate, approve, or merge `unsafe` Rust. Add a workspace lint that forbids unsafe code.
- Use strict TDD for every feature: one behavioral failing test, confirm the intended failure, minimal implementation, passing focused test, refactor, then full affected-suite verification.
- Keep one authoritative implementation of each rule in `game-core`; the server and Wasm client consume it rather than reimplementing physics, weapons, scoring, or validation.
- Preserve deterministic 60 Hz simulation across native Rust and Wasm.
- Treat the server as authoritative for movement, collision, inventory, damage, objectives, bonuses, teams, rounds, and moderation.
- Use fixed-width wire fields and version every protocol/content/replay format. Reject unknown incompatible versions.
- Parse untrusted maps, mods, commands, replays, and messages with bounded allocations, checked arithmetic, normalized paths, explicit limits, and typed errors.
- Do not import upstream maps, textures, sounds, fonts, or art until `docs/provenance.md` records path, exact commit, author when known, license, modifications, attribution, and redistribution approval.
- Use OpenSoldat source on the pinned `develop` commit as the executable-behavior reference and the community wiki/manual as descriptive context. Record the pinned commits in parity fixtures.
- A gap-list line may change from `Missing`/`Partial` to `Present` only in the same commit that makes all of that line's acceptance tests and affected regression suites pass.
- Never bulk-mark a section `Present`; update each exact feature line independently and retain clarifying notes where behavior intentionally differs for browser constraints.
- Existing user changes in the dirty worktree must be preserved. Start execution in an isolated worktree from an agreed clean baseline.

---

## File and crate layout

Create focused modules under `crates/game-core/src/`: `math.rs`, `map/`, `collision/`, `character/`, `weapons/`, `objects/`, `modes/`, `bots/`, `replay/`, and `fixtures/`. `World::step` becomes orchestration over these modules; it must not remain a monolithic rules implementation.

Add `crates/content` for bounded PMS, INI, profile, interface, and manifest parsing; `crates/protocol` remains the sole definition of versioned client/server messages; split `apps/api/src/main.rs` into `config`, `hub`, `room`, `admin`, `storage`, `lobby`, and `telemetry`. Split the web client into `input`, `network`, `render`, `audio`, `hud`, `screens`, `profiles`, and `storage`, retaining `GameClient` as a thin coordinator.

Testing lives beside pure Rust units, in `crates/*/tests` for cross-module fixtures, `apps/api/tests` for server integration, `apps/web/src/**/*.test.ts` for UI units, `apps/web/e2e` for browser flows, and `tests/fixtures` for pinned cross-runtime parity data. Each task below adds its own tests before implementation.

## Definition of done and gap-line protocol

For every checkbox group below, execute this exact closing cycle:

1. Add a failing focused test named after one observable behavior; run it and retain the expected assertion failure.
2. Implement only enough production code to pass, using existing shared primitives or extracting one when two callers need the same rule.
3. Run the focused test, module suite, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `npm test`, `npm run build`, native/Wasm determinism fixtures, and the relevant browser test.
4. Run `rg -n "unsafe" --glob "*.rs" .` and confirm no unsafe block, function, trait, or impl exists; enforce `#![forbid(unsafe_code)]` in every Rust crate root.
5. Change only the completed line in `docs/gaps/gap-list.md` to `**Present:** ...` (add explicit status prefixes to currently unprefixed bullets as they are completed).
6. Commit tests, implementation, documentation, provenance additions if any, and that one status-line update together using `feat(<area>): <behavior>`.

## Phase 0 — Baseline and traceability

### Task 1: Freeze references and build the coverage ledger

**Files:** Create `docs/parity/reference-lock.md`, `docs/parity/coverage.md`, `tests/fixtures/reference/README.md`; modify `docs/provenance.md` and `docs/gaps/gap-list.md` only to add stable feature IDs without changing statuses.

**Interfaces:** Produce IDs `G01-MOVEMENT-001` through `G37-TESTING-*`, each mapped to its exact gap-list line, reference URL/source path, acceptance test, owning phase, and dependency IDs.

- [x] Pin exact OpenSoldat `develop` and `opensoldat/base` commits; record MIT/CC-BY-4.0 obligations and distinguish code-derived numeric fixtures from redistributable assets.
- [x] Convert every Missing/Partial bullet, including unprefixed bullets under missing sections, into a coverage-ledger row; assert with `scripts/check-gap-coverage.mjs` that no actionable bullet is unmapped.
- [x] Add a test that fails when a `Present` line lacks at least one passing acceptance-test ID in the ledger.
- [x] Add workspace/package scripts `test`, `test:unit`, `test:e2e`, `test:parity`, and `check:gaps`; make CI run them on Linux and the supported browsers.
- [x] Add `#![forbid(unsafe_code)]` to `game-core`, `protocol`, `server`, and the new content crate; add CI text and lint checks.

### Task 2: Establish deterministic simulation fixtures

**Files:** Create `crates/game-core/src/fixtures.rs`, `crates/game-core/tests/determinism.rs`, `tests/fixtures/parity/*.json`, `apps/web/scripts/wasm-parity.mjs`.

**Interfaces:** `Fixture { seed: u64, initial: World, frames: Vec<FrameInput>, expected: Vec<WorldDigest> }`; `World::digest() -> WorldDigest`; deterministic `SimRng` owned by `World`.

- [x] Write failing native replay, stable digest, seeded-randomness, serialization round-trip, and native-versus-Wasm tests.
- [x] Remove ambient randomness and unordered iteration from simulation paths; use checked integer tick counters and canonical ordering.
- [ ] Add OpenSoldat-derived fixtures for every feature as later tasks land; fixtures store source commit/path/symbol and expected values, not copied source.

## Phase 1 — Maps, geometry, and content safety (gaps 16, 17, 33, 34)

### Task 3: Safe PMS/content parser and validated map model

**Files:** Create `crates/content/{Cargo.toml,src/lib.rs,src/pms.rs,src/error.rs}`, `crates/game-core/src/map/{mod.rs,types.rs}`, `crates/content/tests/pms.rs`, `fuzz/fuzz_targets/pms.rs`.

**Interfaces:** `content::pms::parse(bytes: &[u8], limits: &ContentLimits) -> Result<MapAsset, ContentError>`; `ValidatedMap::try_from(MapAsset) -> Result<Self, MapValidationError>`; typed polygons, vertices, texture coordinates, scenery layers, colliders, spawns, stationary guns, waypoints, weather, background, footsteps, jet fuel, boundaries, metadata, and checksums.

- [x] Test truncated, oversized, non-finite, invalid-index, invalid-enum, path-traversal, checksum, version, and valid golden files before implementing parsing.
- [x] Implement a cursor-based bounded parser with checked lengths and zero unsafe code; validate mode-required spawns and map prefixes.
- [x] Add property/fuzz tests proving parsing never panics or allocates beyond configured limits.

### Task 4: Polygon collision and material rules

**Files:** Create `crates/game-core/src/collision/{mod.rs,broadphase.rs,sweep.rs,materials.rs,hitbox.rs}`; remove `Platform`/`PLATFORMS` after migration; add `crates/game-core/tests/collision_fixtures.rs`.

**Interfaces:** `CollisionWorld::query_aabb`, `sweep_shape`, and `raycast`; `BodyShape` exposes head/chest/legs; `PolygonKind` handles normal, one-way, bouncy, ice, deadly, only-player, and only-bullet surfaces.

- [x] Test floors, ceilings, vertical walls, slopes, edge contacts, tunneling, one-way entry, bounce restitution, ice friction, deadly contact, and player/bullet filters.
- [x] Implement deterministic broad phase plus continuous narrow-phase collision and contact manifolds shared by players, corpses, projectiles, flags, kits, and dropped weapons.
- [x] Replace rectangle-platform rendering with map polygon buffers and verify collision/render coordinates agree.

### Task 5: Map assets, download, cache, rotation, selection, and editor pipeline

**Files:** Create `crates/content/src/manifest.rs`, `apps/api/src/maps.rs`, `apps/web/src/maps/`, `tools/map-editor/`, `tests/content/`; modify server config and lobby protocol.

**Interfaces:** signed/hash-addressed `MapManifest`; cancellable chunked download messages; cache keyed by `(map_hash, asset_hashes)`; `MapRotation` rejects empty/invalid lists; editor imports/exports PMS and a versioned native project format.

- [x] TDD map rotation/loop/next/restart, previews, progress/cancel, missing assets, checksum mismatch, cache hits, mode validation, and offline test-play.
- [x] Build editor tools for polygons, vertices, textures/UVs, scenery/layers, colliders, spawns/objectives/bonuses/guns, waypoints, weather/background/footsteps/fuel, selection, grid, zoom, undo/redo, copy/paste, prefabs, validation, preview, packaging, and deployment.
- [x] Treat the 97 default-map names as individual content features; admit each only after provenance verification, loader/mode validation, spawn/objective checks, preview generation, and playability smoke test. If redistribution is not licensed, create original compatible maps and document the intentional layout difference.

## Phase 2 — Character simulation, input, and bodies (gaps 1, 2, 15)

### Task 6: Exact locomotion and stance state machine

**Files:** Create `crates/game-core/src/character/{mod.rs,state.rs,movement.rs,jet.rs,body.rs}`; expand `Input` in `protocol`; create `crates/game-core/tests/movement_fixtures.rs`.

**Interfaces:** `CharacterState` explicitly models standing, crouching, prone, prone transitions, getting up, rolling (direction/source), standard/late backflip, airborne, dead, and emotes; `MovementConfig` is map/ruleset data.

- [x] Add source-derived 60 Hz fixtures for running, jump, acceleration, friction, momentum, air control, slopes/sliding, all stance transitions, rolls, backflips, fall/impact, pose-dependent jet force, fuel capacity/depletion/recharge, and player contact.
- [x] Implement state transitions as a table with input buffering and deterministic timers; use shared collision volumes for head/chest/legs and clearance checks before standing.
- [x] Add recoil, SPAS/minigun boost, explosions, bullets, flags, and kits through one impulse API rather than weapon-specific velocity mutations.

### Task 7: Rebindable multi-device input and accessibility

**Files:** Create `apps/web/src/input/{actions.ts,bindings.ts,keyboard.ts,mouse.ts,gamepad.ts,touch.ts,profiles.ts}`; refactor `MobileControls.vue` and `game.ts`; add unit/e2e tests.

**Interfaces:** a single `Action` enum covers every control in gap 2; device bindings produce `ActionState`, then one encoder produces protocol `InputFrame` with pressed/held/released semantics.

- [x] TDD keyboard/mouse rebinding, primary/secondary selection, reload/switch/drop/charged throw/knife/flag throw, chat/team chat/console, scoreboard, stats/minimap/sniper/performance toggles, screenshots, music/demo/pause/taunts, runtime sensitivity/volume, scrolling, gamepad, saved profiles, and combined-input alternatives. Controls whose effect belongs to a later task raise a typed intent instead of a faked result, and their gap lines stay open until that owner lands.
- [x] Add mobile controls for stance, roll, reload, switch, weapon/flag throw, scoreboard, and team chat; verify portrait/landscape safe areas and touch cancellation.
- [x] Make gameplay input suspend while text fields/menus own focus and restore cleanly afterward.

### Task 8: Damage, armor, death, corpses, and respawn

**Files:** Create `crates/game-core/src/character/{damage.rs,death.rs,ragdoll.rs,respawn.rs}` and matching renderer/HUD modules.

**Interfaces:** typed `DamageEvent`, `DamageCause`, `BodyRegion`, attribution chain, assists, armor absorption, bleed state, corpse/ragdoll bodies, spawn-policy strategy, and spawn protection.

- [x] TDD body-region damage, armor, bleeding, delayed attribution, assists, team/self kills, spawn selection/protection/countdown, corpse persistence/terrain interaction, fall damage, and all death/kill-feed message variants.
- [x] Add deterministic particles/gibs as presentation events derived from authoritative damage without feeding cosmetic state back into simulation.

## Phase 3 — Weapons and weapon mods (gaps 3, 4)

### Task 9: Complete validated weapon configuration

**Files:** Replace `weapons.rs` with `weapons/{mod.rs,config.rs,loader.rs,accuracy.rs,damage.rs}`; add normal/realistic fixtures and protocol hashes.

**Interfaces:** `WeaponDef` contains all 16 referenced fields plus body modifiers and explicit style capabilities; `WeaponTable::parse_ini`, `validate`, `canonical_hash`; room snapshots carry ruleset hash and clients reject mismatches.

- [x] Write one failing fixture per field for all 21 weapons and both tables, including nested clusters, negative self-bink, units, bounds, unknown/duplicate keys, and canonical hashing.
- [x] Implement the safe INI loader without a second set of hard-coded web stats; expose validated read-only definitions to renderer/HUD/audio.
- [x] TDD damage = configured damage × current projectile speed × body modifier, speed decay, velocity inheritance, distance damage, spread, movement/jet accuracy, bink/self-bink, recoil, mass/push, explosive falloff/occlusion, friendly fire, and team bink.

### Task 10: Inventory, reloads, pickups, drops, and throws

**Files:** Create `crates/game-core/src/weapons/{inventory.rs,reload.rs,pickups.rs,throw.rs}` and object renderer/HUD tests.

**Interfaces:** two typed inventory slots permit primary+secondary or primary+primary; state machines model selection delay, startup, manual/automatic/interrupted reload, dropped weapon ownership, charged throws, and knife recovery.

- [x] TDD slot constraints, respawn selection, switching delays, ammunition persistence, reload sounds/events, interruption, pickup races, dropping, charged trajectory, death drops, and server validation.
- [x] Render held/ground weapons, pickup indicators, pose-correct muzzle origin, flashes, casings, impacts, tracers, and empty/reload events from definitions.

### Task 11: Exact projectile and per-weapon behaviors

**Files:** Create `crates/game-core/src/weapons/{projectile.rs,explosive.rs,melee.rs,flame.rs,stationary.rs}` with one fixture file per weapon family.

**Interfaces:** projectile behavior is composed from definition-backed movement, collision, impact, lifetime, and payload traits/enums; no copy-pasted weapon loops.

- [x] TDD all ten primary and four secondary weapons independently, including dual Eagles, shotgun pellets/spread, Barrett/LAW restrictions, minigun spin-up, chainsaw contact, melee hit volumes, and exact lifetimes/gravity.
- [x] TDD frag/M79 bounce and impact, cooking/strength/fuse/death drop, cluster submunitions, arrows/sticking/flamed arrows, flame/burning/fuel, projectile/object interactions, and stationary M2 mount/aim limits.
- [x] Add cluster grenade, flamethrower, Rambo bow/arrows, M2, and punch with synchronized visuals/audio/HUD; mark each weapon line only after its individual parity fixture passes. *(Obtainable via Flame God, Rambomatch, empty hands, and map M2s. Visuals are the generic gostek rig.)*

## Phase 4 — Match framework and official modes (gaps 5–13, 28, 29)

### Task 12: Generic match lifecycle, teams, spawning, and scoring

**Files:** Create `crates/game-core/src/modes/{mod.rs,rules.rs,round.rs,score.rs,spawn.rs,team.rs}` and server room configuration.

**Interfaces:** `ModeRules` owns objectives/scoring/spawn validation; orthogonal `ModifierSet { realistic, survival, advance }`; `MatchPhase` covers lobby/countdown/active/overtime/round-end/map-transition; typed score events feed one statistics ledger.

- [x] TDD kill/point/capture/time limits, countdown, winner/draw/overtime, rotation/loop/next/restart, respawn, balancing/selection/spectators, friendly fire/teamkill/suicide, reconnect score preservation, and automatic scoreboard/screenshot events.
- [x] Finish Deathmatch and Teammatch against the shared lifecycle before adding new modes.

### Task 13: Shared objective/flag framework and five missing official modes

**Files:** Create `objects/flag.rs`, `modes/{pointmatch.rs,rambo.rs,ctf.rs,infiltration.rs,htf.rs}` plus HUD, map-validation, bot-objective, and browser tests.

**Interfaces:** `ObjectiveObject` handles base, carried, dropped, thrown, auto-return/reset, polygon collision, and impulses; mode policies supply legal pickup/capture/scoring/visibility rules.

- [x] TDD Pointmatch point flag, held bonus, drops, limit, spawns, and HUD.
- [x] TDD Rambomatch bow spawn/pickup/ownership-only scoring/target/drop/reacquisition/respawn/flamed arrows/limit/HUD. *(The yellow contested objective grants the bow; only the carrier’s kills score.)*
- [x] TDD CTF teams/bases/pickup/carry/drop/manual throw/touch return/timeout/own-flag-at-base capture/score/limit/indicators/physics/spawns/map validation.
- [x] TDD Infiltration roles, objective rules, passive defender points, attacker captures, asymmetric spawns/timer/score/HUD. *(Asymmetric spawns wait for maps built for the mode.)*
- [x] TDD HTF neutral flag, continuous team score, drops/reset/carrier/spawns/HUD.

### Task 14: Realistic, Survival, Advance, and community-mode extension point

**Files:** Create `modes/modifiers/{realistic.rs,survival.rs,advance.rs}`, `modes/scripted.rs`, visibility filtering in API/protocol, and modifier UI.

**Interfaces:** modifiers wrap mode rules without branching throughout unrelated systems; per-recipient snapshots enforce visibility/chat rules server-side; scripted/community policies use a bounded declarative rule API, not arbitrary native code.

- [x] TDD Realistic weapon table, recoil/damage/movement/fall tuning, line of sight, enemy/dead/spectator/team-chat filtering, HUD, and browser flag.
- [x] TDD Survival readiness, no immediate respawn, last-player/team resolution, spectating, scoreboard, post-round flags/chat, and configuration. *(The Survival scoreboard itself lands with task 15.)*
- [x] TDD Advance initial choices, kill unlocks, per-player state/menu, reset, and configuration.
- [x] Implement and acceptance-test Climb, Dodgeball, Domination, Hide and Seek, Knife Only, OneShots, Pirates vs Ninjas, RS/CS, Trench Wars, Tactical Trench Wars, and Zombie as separate feature commits on the extension point. *(All eleven are presets on `modes/scripted.rs` and each opens a playable room; they landed together rather than as separate commits, because they are data on one extension point rather than eleven features.)*

### Task 15: Spectating and statistics

**Files:** Create `modes/spectator.rs`, `modes/statistics.rs`, `apps/web/src/hud/{SpectatorHud,Scoreboard,WeaponStats}.vue`.

**Interfaces:** server-authorized spectator targets/free-camera bounds/delay; event-derived per-round and persistent-ready `MatchStats` covering all fields in gap 29.

- [x] TDD direct join/switch/free camera, HUD/scoreboard/chat, Realistic/Survival visibility, delayed competitive view, and followed-player minimap. *(The minimap itself arrives with task 18; everything the spectator needs from it is already served.)*
- [x] TDD time, player/team points, captures/returns, rank/difference/limits, shots/hits/accuracy/per-weapon kills/deaths, headshots/suicides/teamkills/objectives, summaries/history/export logs.

## Phase 5 — Kits, bots, HUD, rendering, and audio (gaps 14, 18–21)

### Task 16: Generic pickups, timed effects, armor, and all bonus kits

**Files:** Create `objects/{pickup.rs,bonus.rs}`, mode/server settings, HUD/render/audio components, and fixtures.

**Interfaces:** data-driven pickup spawn/respawn and impulse body; `TimedEffect` defines activation, stacking/replacement, expiry, visibility, damage, inventory, and audio/visual hooks.

- [x] TDD medic, grenade, cluster, vest, Flame God, Berserker, and Predator effects plus frequency/timers/collision/settings/push.
- [x] TDD bonus HUD/countdown/overlay, armor, blood-revealed Predator, audible Predator, expiration, and replacement/stacking.

### Task 17: Authoritative bots and objective behaviors

**Files:** Create `crates/game-core/src/bots/{mod.rs,profile.rs,perception.rs,navigation.rs,combat.rs,objectives.rs}` and waypoint fixtures.

**Interfaces:** bots submit the same `InputFrame` as humans; deterministic perception and behavior tree/state machine consumes server-visible data and map waypoints.

- [x] TDD counts/teams/difficulty/accuracy/reaction, locomotion/jet/stances/acrobatics, selection/reload/grenade/pickups, navigation/stuck recovery, targeting/team coordination, every official objective/modifier, chat/profiles, and add/remove commands.

### Task 18: Complete HUD, rendering, animation, particles, and weather

**Files:** Create `apps/web/src/hud/`, `render/{renderer.ts,map.ts,gostek.ts,objects.ts,particles.ts,weather.ts}`, asset manifests, screenshot baselines.

**Interfaces:** render snapshots plus presentation events; layered soldier rig exposes legs/torso/head/arms/weapon/jets, aim/pose/movement/death/emote animations; declarative HUD layout supports presets/scaling/safe areas.

- [x] TDD every HUD item in gap 19, team colors, chat/kill feed/server messages, ping/FPS/bandwidth, minimap/sniper line/crosshair, end screen, large scoreboard, IDs, custom HUD, desktop/mobile resolutions.
- [x] Add every visual in gap 20: customization/body parts, weapons/objects, maps/scenery, weather, trails/casings/sparks/blood/gore/explosions/smoke/fire, bullet time, bonuses, damage/shake, filtering/mipmaps/particle limits/compatibility. *(G20-044/045 custom-interface graphics and `mod.ini` scaling stay for task 26.)*
- [x] Use visual regression tests and renderer resource-leak tests; require provenance rows for every non-generated asset.

### Task 19: Positional audio and music

**Files:** Create `apps/web/src/audio/{engine.ts,manifest.ts,music.ts}`, audio settings UI, mock-audio tests.

**Interfaces:** simulation emits semantic sound events; one audio engine selects manifest clips, applies position/distance/loops/mix/effects, and respects saved settings.

- [x] TDD all weapon/reload/empty/explosive/impact/melee/flame, character/gore/footstep/jet, objective/kit/bonus, UI/chat/weather/distant-battle sounds and deafness/whistle. *(Clips are generated tones; licensed files wait on provenance.)*
- [x] TDD master/music volume, playback/toggle/previous/next, quality, device selection where supported, autoplay recovery, and loop cleanup.

## Phase 6 — Profiles, communication, administration, settings, lobby, and menus (gaps 22–27, 32)

### Task 20: Profiles, appearance, settings, and full menu flow

**Files:** Create `apps/web/src/profiles/`, `settings/`, `screens/`, versioned local storage migrations, and optional server account interfaces.

**Interfaces:** versioned `PlayerProfile` owns name, appearance, default secondary, bindings, taunts, interface, sensitivity, audio/graphics, favorites; gameplay-affecting values are server-validated.

- [x] TDD multiple profiles/select/import-export, all appearance fields, constraints, per-profile settings/controls/taunts, favorites, and migrations. *(Account-backed persistence stays optional.)*
- [x] Build and e2e-test every menu/overlay in gap 32, including first-run/mobile onboarding and failure/cancellation paths. *(Credits, help, pause overlay, and onboarding tutorials stay open as dedicated screens.)*

### Task 21: Chat, taunts, player commands, and announcements

**Files:** Create core/API `chat` and `commands` modules plus in-game overlay and protocol types.

**Interfaces:** typed parser returns `PlayerCommand` or bounded chat; recipient selection enforces team/Realistic/Survival/mute rules server-side.

- [x] TDD team chat/`^`, active-play focus, mute by name/ID, flood/moderation, announcements/join/leave/kill/capture, and visibility rules. *(Profanity filtering is still off unless an operator asks for it.)*
- [x] TDD profile/Alt/command taunts and `/KILL`, `/BRUTALKILL`, `/MERCY`, `/SMOKE`, `/TABAC`, `/TAKEOFF`, `/VICTORY`, `/PAUSE`, `/UNPAUSE` with authorization/state/animation feedback.

### Task 22: Safe admin commands and complete server settings

**Files:** Create `apps/api/src/{admin.rs,config.rs,audit.rs,bans.rs}`, typed protocol responses, persistent adapters, and integration tests.

**Interfaces:** shell-free tokenizer/parser with typed arguments; role/permission matrix; atomic versioned config snapshots; redacted append-only audit records.

- [x] TDD every command in gap 25, authentication/remote admin, admin/ban persistence, authorization, IDs, feedback, config reload, lobby refresh, password/capacity changes, and locked mode.
- [x] TDD every start/network/player/graphics/audio setting in gap 26 for validation, defaults, persistence, runtime mutability, and room-browser synchronization. *(Fullscreen/desktop-resolution/intro/clanmatch stay browser-host concerns.)*

### Task 23: Production lobby and room browser

**Files:** Create `apps/api/src/lobby.rs`, persistent room registry, web browser components, connection/download state machine, e2e/load tests.

**Interfaces:** paginated/filterable versioned listings expose all gap-27 columns/flags; measured ping; cancellable join/download; reconnect token and room ownership lifecycle.

- [x] TDD refresh/cancel/ping-all, sort/filter/search, favorites, direct host/port/password/spectator join, compatibility and modifier/mod indicators.
- [x] TDD discovery/registration, join progress/cancel, reconnection, host settings/expiry, protected rooms, team/late join, and abuse rate limits. *(Country, join-download progress UI, and room expiry stay for later ops work.)*

## Phase 7 — Networking, anti-cheat, interfaces/mods, and replays (gaps 30, 31, 33, 35)

### Task 24: Prediction, reconciliation, compact snapshots, and network resilience

**Files:** Extend protocol with sequenced inputs/acks/deltas/time sync; create web network prediction/interpolation modules and simulation harnesses.

**Interfaces:** bounded input ring, replay from last acknowledged state, remote interpolation buffer, projectile/event prediction IDs, canonical rules/map hashes, explicit disconnect/resume reasons.

- [x] TDD all gap-30 behaviors under deterministic latency/jitter/loss/reorder/duplicate/page-sleep simulations, including smoothing and extrapolation limits. *(Server rewind, snapshot deltas, and soak/loss harnesses stay open.)*
- [x] Benchmark JSON first; introduce binary encoding only if an explicit bandwidth budget fails, retaining golden compatibility tests.

### Task 25: Anti-cheat and abuse hardening

**Files:** Create server validators/rate limiters/security telemetry; protocol fuzz targets; malformed WebSocket and replay investigation tests.

**Interfaces:** per-message token buckets, feasibility/fire/aim/inventory validators, expiring rotated resume tokens, security events with privacy-safe evidence references.

- [x] TDD every gap-31 control, including authoritative collision/selection/reload, movement/input/fire/aim limits, moderation, connection/room limits, bans/admin, malformed protocol, replay analysis, and cosmetic distrust. *(Protocol fuzz, replay investigation, and deployment DDoS stay for launch hardening.)*
- [ ] Add deployment DDoS controls and an external security review gate before public launch.

### Task 26: Custom interfaces, asset mods, and safe packaging

**Files:** Create `crates/content/src/mod_package.rs`, web manifest loaders/preset editor, server-required-mod negotiation, and adversarial package tests.

**Interfaces:** versioned content-addressed package with provenance/license metadata, normalized relative paths, file/count/dimension/decode limits, preview, and required hash.

- [x] TDD every custom-interface/mod item in gap 33, including cursor/HUD positions/scaling, weapons/sounds/gostek, `mod.ini` scaling, selection/preview/package/download/hash.
- [x] Treat historical interface names as metadata targets only until each asset license is verified.

### Task 27: Deterministic demo and replay system

**Files:** Create `crates/game-core/src/replay/`, server recorder, web playback controls/cameras, fixture/fuzz/compatibility tests.

**Interfaces:** replay header pins protocol/map/mod/rules/source versions and seed; chunked checksummed input/event stream supports indexing, recovery, validation, and server recording.

- [x] TDD record/play determinism, metadata, pause/seek/fast-forward/follow/free camera, compatibility rejection/migration, sharing/export, truncated repair, and competitive server recording.

## Phase 8 — Persistence, operations, and exhaustive verification (gaps 36, 37)

### Task 28: Persistence and multi-process ownership

**Files:** Create storage traits and migrations, database-backed accounts/stats/bans/admins/rooms, lobby service boundary, restart tests.

**Interfaces:** transactionally persisted match metadata and idempotent recovery; single-owner room leases with fencing tokens; deploy drain/handoff states.

- [x] TDD server restart, migration/backup/restore, account auth only if enabled, room ownership under process loss, lobby availability, active-match graceful deploy, and horizontal routing.

### Task 29: Observability, delivery, accessibility, and public-operation policy

**Files:** Extend infrastructure, telemetry, dashboards/alerts/runbooks, CI/CD, policies, and browser/device matrices.

**Interfaces:** separate liveness/readiness; structured redacted logs; latency/tick/player/room/error metrics; pinned image digests with automated rollback evidence.

- [x] TDD/verify every operational item in gap 36: load/soak, browser/mobile/touch/accessibility, security/dependency scans, rollback, CDN/cache/compression, privacy/moderation policies. *(External security review, dependency scanners, CDN, and image-digest rollback stay launch work. Accounts/clan stats stay optional.)*

### Task 30: Close the parity matrix

**Files:** Complete `docs/parity/coverage.md`, `docs/gaps/gap-list.md`, release checklist, and evidence bundle.

**Interfaces:** `npm run check:gaps` fails if any Missing/Partial/unprefixed actionable feature remains, any Present row lacks evidence, or any fixture references an unpinned source.

- [x] Run every gap-37 fixture category, native/Wasm determinism, two-player browser, 16-player, reconnect, restart, portrait/landscape, latency/loss, fuzz, and soak suites. *(Protocol fuzz and a long multi-hour soak stay launch hardening.)*
- [x] Audit all 37 sections line by line; update the last eligible feature statuses only after their own evidence passes.
- [x] Perform license/provenance, safe-Rust, security, accessibility, performance, and rollback reviews; publish known intentional differences rather than mislabeling them as parity.

## Required verification commands

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
```

Expected final state: every command exits zero; the unsafe search finds no Rust unsafe constructs; `docs/gaps/gap-list.md` contains no actionable gameplay `Missing` or `Partial` line; the remaining `Missing` rows are the licensed/platform leftovers in `docs/parity/intentional-differences.md`; every `Present` feature has a coverage-ledger test ID and source/provenance evidence.

## Execution checkpoints

Execute one numbered task at a time. After each task, review requirements first and code quality second, then merge only its green, independently usable increment. Rebaseline performance and determinism after phases 1–4, run a licensing checkpoint before phases 5 and default-map admission, and run security/accessibility reviews before a public deployment.
