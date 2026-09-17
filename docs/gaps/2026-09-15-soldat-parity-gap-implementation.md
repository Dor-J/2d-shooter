# Soldat Parity Gap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement every `Missing` and `Partial` item in `docs/gaps/gap-list.md` as verified, server-authoritative, browser-playable Soldat-parity features.

**Architecture:** Grow the current prototype into a deterministic, safe-Rust simulation library consumed by the authoritative server and Wasm prediction client. Keep gameplay rules in focused `game-core` modules, wire-only types in `protocol`, orchestration in `server`, and presentation/input in Vue/WebGL; content and configuration are validated data rather than duplicated constants.

**Tech Stack:** Rust 1.88 (edition 2021), safe Rust only, serde, Axum/Tokio/WebSocket, wasm-bindgen, Vue 3, TypeScript, WebGL2, Node test runner, Playwright, cargo-fuzz, Docker Compose.

**Spec:** `docs/gaps/gap-list.md`

## Progress snapshot — 2026-09-17

- **Completed milestones:** Tasks 1 and 3–6. Task 2's deterministic native/Wasm foundation is complete; its intentionally ongoing “add fixtures as later features land” checkbox remains open.
- **Current coverage:** 1,104 gap features are mapped; 223 `Present` features have passing evidence in `docs/parity/coverage.json`.
- **Task 6 evidence:** 13 movement fixtures cover 60 Hz source constants, acceleration/friction/momentum, air control, jump buffering, stances, rolls, standard/late backflips, slopes, impacts, jets, player contact, collision volumes, emotes/death states, and the shared impulse API.
- **Last completed verification:** `cargo test --workspace`, focused determinism and movement suites, Clippy with warnings denied, Rust formatting, web unit tests, production web build, native/Wasm parity, gap coverage, unsafe-Rust scan, and `git diff --check` passed.
- **Next task:** Task 7 — rebindable multi-device input and accessibility. It has not started.
- **Intentionally still open:** visual character animation layers, ragdolls, and automatic bullet/explosion interaction with flag/kit game objects remain assigned to their later owning tasks; their gap lines were not marked `Present` by Task 6.

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

- [ ] TDD keyboard/mouse rebinding, primary/secondary selection, reload/switch/drop/charged throw/knife/flag throw, chat/team chat/console, scoreboard, stats/minimap/sniper/performance toggles, screenshots, music/demo/pause/taunts, runtime sensitivity/volume, scrolling, gamepad, saved profiles, and combined-input alternatives.
- [ ] Add mobile controls for stance, roll, reload, switch, weapon/flag throw, scoreboard, and team chat; verify portrait/landscape safe areas and touch cancellation.
- [ ] Make gameplay input suspend while text fields/menus own focus and restore cleanly afterward.

### Task 8: Damage, armor, death, corpses, and respawn

**Files:** Create `crates/game-core/src/character/{damage.rs,death.rs,ragdoll.rs,respawn.rs}` and matching renderer/HUD modules.

**Interfaces:** typed `DamageEvent`, `DamageCause`, `BodyRegion`, attribution chain, assists, armor absorption, bleed state, corpse/ragdoll bodies, spawn-policy strategy, and spawn protection.

- [ ] TDD body-region damage, armor, bleeding, delayed attribution, assists, team/self kills, spawn selection/protection/countdown, corpse persistence/terrain interaction, fall damage, and all death/kill-feed message variants.
- [ ] Add deterministic particles/gibs as presentation events derived from authoritative damage without feeding cosmetic state back into simulation.

## Phase 3 — Weapons and weapon mods (gaps 3, 4)

### Task 9: Complete validated weapon configuration

**Files:** Replace `weapons.rs` with `weapons/{mod.rs,config.rs,loader.rs,accuracy.rs,damage.rs}`; add normal/realistic fixtures and protocol hashes.

**Interfaces:** `WeaponDef` contains all 16 referenced fields plus body modifiers and explicit style capabilities; `WeaponTable::parse_ini`, `validate`, `canonical_hash`; room snapshots carry ruleset hash and clients reject mismatches.

- [ ] Write one failing fixture per field for all 21 weapons and both tables, including nested clusters, negative self-bink, units, bounds, unknown/duplicate keys, and canonical hashing.
- [ ] Implement the safe INI loader without a second set of hard-coded web stats; expose validated read-only definitions to renderer/HUD/audio.
- [ ] TDD damage = configured damage × current projectile speed × body modifier, speed decay, velocity inheritance, distance damage, spread, movement/jet accuracy, bink/self-bink, recoil, mass/push, explosive falloff/occlusion, friendly fire, and team bink.

### Task 10: Inventory, reloads, pickups, drops, and throws

**Files:** Create `crates/game-core/src/weapons/{inventory.rs,reload.rs,pickups.rs,throw.rs}` and object renderer/HUD tests.

**Interfaces:** two typed inventory slots permit primary+secondary or primary+primary; state machines model selection delay, startup, manual/automatic/interrupted reload, dropped weapon ownership, charged throws, and knife recovery.

- [ ] TDD slot constraints, respawn selection, switching delays, ammunition persistence, reload sounds/events, interruption, pickup races, dropping, charged trajectory, death drops, and server validation.
- [ ] Render held/ground weapons, pickup indicators, pose-correct muzzle origin, flashes, casings, impacts, tracers, and empty/reload events from definitions.

### Task 11: Exact projectile and per-weapon behaviors

**Files:** Create `crates/game-core/src/weapons/{projectile.rs,explosive.rs,melee.rs,flame.rs,stationary.rs}` with one fixture file per weapon family.

**Interfaces:** projectile behavior is composed from definition-backed movement, collision, impact, lifetime, and payload traits/enums; no copy-pasted weapon loops.

- [ ] TDD all ten primary and four secondary weapons independently, including dual Eagles, shotgun pellets/spread, Barrett/LAW restrictions, minigun spin-up, chainsaw contact, melee hit volumes, and exact lifetimes/gravity.
- [ ] TDD frag/M79 bounce and impact, cooking/strength/fuse/death drop, cluster submunitions, arrows/sticking/flamed arrows, flame/burning/fuel, projectile/object interactions, and stationary M2 mount/aim limits.
- [ ] Add cluster grenade, flamethrower, Rambo bow/arrows, M2, and punch with synchronized visuals/audio/HUD; mark each weapon line only after its individual parity fixture passes.

## Phase 4 — Match framework and official modes (gaps 5–13, 28, 29)

### Task 12: Generic match lifecycle, teams, spawning, and scoring

**Files:** Create `crates/game-core/src/modes/{mod.rs,rules.rs,round.rs,score.rs,spawn.rs,team.rs}` and server room configuration.

**Interfaces:** `ModeRules` owns objectives/scoring/spawn validation; orthogonal `ModifierSet { realistic, survival, advance }`; `MatchPhase` covers lobby/countdown/active/overtime/round-end/map-transition; typed score events feed one statistics ledger.

- [ ] TDD kill/point/capture/time limits, countdown, winner/draw/overtime, rotation/loop/next/restart, respawn, balancing/selection/spectators, friendly fire/teamkill/suicide, reconnect score preservation, and automatic scoreboard/screenshot events.
- [ ] Finish Deathmatch and Teammatch against the shared lifecycle before adding new modes.

### Task 13: Shared objective/flag framework and five missing official modes

**Files:** Create `objects/flag.rs`, `modes/{pointmatch.rs,rambo.rs,ctf.rs,infiltration.rs,htf.rs}` plus HUD, map-validation, bot-objective, and browser tests.

**Interfaces:** `ObjectiveObject` handles base, carried, dropped, thrown, auto-return/reset, polygon collision, and impulses; mode policies supply legal pickup/capture/scoring/visibility rules.

- [ ] TDD Pointmatch point flag, held bonus, drops, limit, spawns, and HUD.
- [ ] TDD Rambomatch bow spawn/pickup/ownership-only scoring/target/drop/reacquisition/respawn/flamed arrows/limit/HUD.
- [ ] TDD CTF teams/bases/pickup/carry/drop/manual throw/touch return/timeout/own-flag-at-base capture/score/limit/indicators/physics/spawns/map validation.
- [ ] TDD Infiltration roles, objective rules, passive defender points, attacker captures, asymmetric spawns/timer/score/HUD.
- [ ] TDD HTF neutral flag, continuous team score, drops/reset/carrier/spawns/HUD.

### Task 14: Realistic, Survival, Advance, and community-mode extension point

**Files:** Create `modes/modifiers/{realistic.rs,survival.rs,advance.rs}`, `modes/scripted.rs`, visibility filtering in API/protocol, and modifier UI.

**Interfaces:** modifiers wrap mode rules without branching throughout unrelated systems; per-recipient snapshots enforce visibility/chat rules server-side; scripted/community policies use a bounded declarative rule API, not arbitrary native code.

- [ ] TDD Realistic weapon table, recoil/damage/movement/fall tuning, line of sight, enemy/dead/spectator/team-chat filtering, HUD, and browser flag.
- [ ] TDD Survival readiness, no immediate respawn, last-player/team resolution, spectating, scoreboard, post-round flags/chat, and configuration.
- [ ] TDD Advance initial choices, kill unlocks, per-player state/menu, reset, and configuration.
- [ ] Implement and acceptance-test Climb, Dodgeball, Domination, Hide and Seek, Knife Only, OneShots, Pirates vs Ninjas, RS/CS, Trench Wars, Tactical Trench Wars, and Zombie as separate feature commits on the extension point.

### Task 15: Spectating and statistics

**Files:** Create `modes/spectator.rs`, `modes/statistics.rs`, `apps/web/src/hud/{SpectatorHud,Scoreboard,WeaponStats}.vue`.

**Interfaces:** server-authorized spectator targets/free-camera bounds/delay; event-derived per-round and persistent-ready `MatchStats` covering all fields in gap 29.

- [ ] TDD direct join/switch/free camera, HUD/scoreboard/chat, Realistic/Survival visibility, delayed competitive view, and followed-player minimap.
- [ ] TDD time, player/team points, captures/returns, rank/difference/limits, shots/hits/accuracy/per-weapon kills/deaths, headshots/suicides/teamkills/objectives, summaries/history/export logs.

## Phase 5 — Kits, bots, HUD, rendering, and audio (gaps 14, 18–21)

### Task 16: Generic pickups, timed effects, armor, and all bonus kits

**Files:** Create `objects/{pickup.rs,bonus.rs}`, mode/server settings, HUD/render/audio components, and fixtures.

**Interfaces:** data-driven pickup spawn/respawn and impulse body; `TimedEffect` defines activation, stacking/replacement, expiry, visibility, damage, inventory, and audio/visual hooks.

- [ ] TDD medic, grenade, cluster, vest, Flame God, Berserker, and Predator effects plus frequency/timers/collision/settings/push.
- [ ] TDD bonus HUD/countdown/overlay, armor, blood-revealed Predator, audible Predator, expiration, and replacement/stacking.

### Task 17: Authoritative bots and objective behaviors

**Files:** Create `crates/game-core/src/bots/{mod.rs,profile.rs,perception.rs,navigation.rs,combat.rs,objectives.rs}` and waypoint fixtures.

**Interfaces:** bots submit the same `InputFrame` as humans; deterministic perception and behavior tree/state machine consumes server-visible data and map waypoints.

- [ ] TDD counts/teams/difficulty/accuracy/reaction, locomotion/jet/stances/acrobatics, selection/reload/grenade/pickups, navigation/stuck recovery, targeting/team coordination, every official objective/modifier, chat/profiles, and add/remove commands.

### Task 18: Complete HUD, rendering, animation, particles, and weather

**Files:** Create `apps/web/src/hud/`, `render/{renderer.ts,map.ts,gostek.ts,objects.ts,particles.ts,weather.ts}`, asset manifests, screenshot baselines.

**Interfaces:** render snapshots plus presentation events; layered soldier rig exposes legs/torso/head/arms/weapon/jets, aim/pose/movement/death/emote animations; declarative HUD layout supports presets/scaling/safe areas.

- [ ] TDD every HUD item in gap 19, team colors, chat/kill feed/server messages, ping/FPS/bandwidth, minimap/sniper line/crosshair, end screen, large scoreboard, IDs, custom HUD, desktop/mobile resolutions.
- [ ] Add every visual in gap 20: customization/body parts, weapons/objects, maps/scenery, weather, trails/casings/sparks/blood/gore/explosions/smoke/fire, bullet time, bonuses, damage/shake, filtering/mipmaps/particle limits/compatibility.
- [ ] Use visual regression tests and renderer resource-leak tests; require provenance rows for every non-generated asset.

### Task 19: Positional audio and music

**Files:** Create `apps/web/src/audio/{engine.ts,manifest.ts,music.ts}`, audio settings UI, mock-audio tests.

**Interfaces:** simulation emits semantic sound events; one audio engine selects manifest clips, applies position/distance/loops/mix/effects, and respects saved settings.

- [ ] TDD all weapon/reload/empty/explosive/impact/melee/flame, character/gore/footstep/jet, objective/kit/bonus, UI/chat/weather/distant-battle sounds and deafness/whistle.
- [ ] TDD master/music volume, playback/toggle/previous/next, quality, device selection where supported, autoplay recovery, and loop cleanup.

## Phase 6 — Profiles, communication, administration, settings, lobby, and menus (gaps 22–27, 32)

### Task 20: Profiles, appearance, settings, and full menu flow

**Files:** Create `apps/web/src/profiles/`, `settings/`, `screens/`, versioned local storage migrations, and optional server account interfaces.

**Interfaces:** versioned `PlayerProfile` owns name, appearance, default secondary, bindings, taunts, interface, sensitivity, audio/graphics, favorites; gameplay-affecting values are server-validated.

- [ ] TDD multiple profiles/select/import-export, all appearance fields, constraints, per-profile settings/controls/taunts, favorites, and migrations.
- [ ] Build and e2e-test every menu/overlay in gap 32, including first-run/mobile onboarding and failure/cancellation paths.

### Task 21: Chat, taunts, player commands, and announcements

**Files:** Create core/API `chat` and `commands` modules plus in-game overlay and protocol types.

**Interfaces:** typed parser returns `PlayerCommand` or bounded chat; recipient selection enforces team/Realistic/Survival/mute rules server-side.

- [ ] TDD team chat/`^`, active-play focus, mute by name/ID, flood/moderation, announcements/join/leave/kill/capture, and visibility rules.
- [ ] TDD profile/Alt/command taunts and `/KILL`, `/BRUTALKILL`, `/MERCY`, `/SMOKE`, `/TABAC`, `/TAKEOFF`, `/VICTORY`, `/PAUSE`, `/UNPAUSE` with authorization/state/animation feedback.

### Task 22: Safe admin commands and complete server settings

**Files:** Create `apps/api/src/{admin.rs,config.rs,audit.rs,bans.rs}`, typed protocol responses, persistent adapters, and integration tests.

**Interfaces:** shell-free tokenizer/parser with typed arguments; role/permission matrix; atomic versioned config snapshots; redacted append-only audit records.

- [ ] TDD every command in gap 25, authentication/remote admin, admin/ban persistence, authorization, IDs, feedback, config reload, lobby refresh, password/capacity changes, and locked mode.
- [ ] TDD every start/network/player/graphics/audio setting in gap 26 for validation, defaults, persistence, runtime mutability, and room-browser synchronization.

### Task 23: Production lobby and room browser

**Files:** Create `apps/api/src/lobby.rs`, persistent room registry, web browser components, connection/download state machine, e2e/load tests.

**Interfaces:** paginated/filterable versioned listings expose all gap-27 columns/flags; measured ping; cancellable join/download; reconnect token and room ownership lifecycle.

- [ ] TDD refresh/cancel/ping-all, sort/filter/search, favorites, direct host/port/password/spectator join, compatibility and modifier/mod indicators.
- [ ] TDD discovery/registration, join progress/cancel, reconnection, host settings/expiry, protected rooms, team/late join, and abuse rate limits.

## Phase 7 — Networking, anti-cheat, interfaces/mods, and replays (gaps 30, 31, 33, 35)

### Task 24: Prediction, reconciliation, compact snapshots, and network resilience

**Files:** Extend protocol with sequenced inputs/acks/deltas/time sync; create web network prediction/interpolation modules and simulation harnesses.

**Interfaces:** bounded input ring, replay from last acknowledged state, remote interpolation buffer, projectile/event prediction IDs, canonical rules/map hashes, explicit disconnect/resume reasons.

- [ ] TDD all gap-30 behaviors under deterministic latency/jitter/loss/reorder/duplicate/page-sleep simulations, including smoothing and extrapolation limits.
- [ ] Benchmark JSON first; introduce binary encoding only if an explicit bandwidth budget fails, retaining golden compatibility tests.

### Task 25: Anti-cheat and abuse hardening

**Files:** Create server validators/rate limiters/security telemetry; protocol fuzz targets; malformed WebSocket and replay investigation tests.

**Interfaces:** per-message token buckets, feasibility/fire/aim/inventory validators, expiring rotated resume tokens, security events with privacy-safe evidence references.

- [ ] TDD every gap-31 control, including authoritative collision/selection/reload, movement/input/fire/aim limits, moderation, connection/room limits, bans/admin, malformed protocol, replay analysis, and cosmetic distrust.
- [ ] Add deployment DDoS controls and an external security review gate before public launch.

### Task 26: Custom interfaces, asset mods, and safe packaging

**Files:** Create `crates/content/src/mod_package.rs`, web manifest loaders/preset editor, server-required-mod negotiation, and adversarial package tests.

**Interfaces:** versioned content-addressed package with provenance/license metadata, normalized relative paths, file/count/dimension/decode limits, preview, and required hash.

- [ ] TDD every custom-interface/mod item in gap 33, including cursor/HUD positions/scaling, weapons/sounds/gostek, `mod.ini` scaling, selection/preview/package/download/hash.
- [ ] Treat historical interface names as metadata targets only until each asset license is verified.

### Task 27: Deterministic demo and replay system

**Files:** Create `crates/game-core/src/replay/`, server recorder, web playback controls/cameras, fixture/fuzz/compatibility tests.

**Interfaces:** replay header pins protocol/map/mod/rules/source versions and seed; chunked checksummed input/event stream supports indexing, recovery, validation, and server recording.

- [ ] TDD record/play determinism, metadata, pause/seek/fast-forward/follow/free camera, compatibility rejection/migration, sharing/export, truncated repair, and competitive server recording.

## Phase 8 — Persistence, operations, and exhaustive verification (gaps 36, 37)

### Task 28: Persistence and multi-process ownership

**Files:** Create storage traits and migrations, database-backed accounts/stats/bans/admins/rooms, lobby service boundary, restart tests.

**Interfaces:** transactionally persisted match metadata and idempotent recovery; single-owner room leases with fencing tokens; deploy drain/handoff states.

- [ ] TDD server restart, migration/backup/restore, account auth only if enabled, room ownership under process loss, lobby availability, active-match graceful deploy, and horizontal routing.

### Task 29: Observability, delivery, accessibility, and public-operation policy

**Files:** Extend infrastructure, telemetry, dashboards/alerts/runbooks, CI/CD, policies, and browser/device matrices.

**Interfaces:** separate liveness/readiness; structured redacted logs; latency/tick/player/room/error metrics; pinned image digests with automated rollback evidence.

- [ ] TDD/verify every operational item in gap 36: load/soak, browser/mobile/touch/accessibility, security/dependency scans, rollback, CDN/cache/compression, privacy/moderation policies.

### Task 30: Close the parity matrix

**Files:** Complete `docs/parity/coverage.md`, `docs/gaps/gap-list.md`, release checklist, and evidence bundle.

**Interfaces:** `npm run check:gaps` fails if any Missing/Partial/unprefixed actionable feature remains, any Present row lacks evidence, or any fixture references an unpinned source.

- [ ] Run every gap-37 fixture category, native/Wasm determinism, two-player browser, 16-player, reconnect, restart, portrait/landscape, latency/loss, fuzz, and soak suites.
- [ ] Audit all 37 sections line by line; update the last eligible feature statuses only after their own evidence passes.
- [ ] Perform license/provenance, safe-Rust, security, accessibility, performance, and rollback reviews; publish known intentional differences rather than mislabeling them as parity.

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

Expected final state: every command exits zero; the unsafe search finds no Rust unsafe constructs; `docs/gaps/gap-list.md` contains no actionable `Missing` or `Partial` line; every `Present` feature has a coverage-ledger test ID and source/provenance evidence.

## Execution checkpoints

Execute one numbered task at a time. After each task, review requirements first and code quality second, then merge only its green, independently usable increment. Rebaseline performance and determinism after phases 1–4, run a licensing checkpoint before phases 5 and default-map admission, and run security/accessibility reviews before a public deployment.
