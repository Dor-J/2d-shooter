# Production and Soldat Parity Backlog

**Date:** 2026-10-07
**Status:** Open
**Supersedes:** nothing. Complements `docs/gaps/gap-list.md` and `docs/gaps/2026-09-15-soldat-parity-gap-implementation.md`, whose ledger reports 1,082 `Present` lines but counts placeholder content (for example the 97 template maps) as present.

This document lists every item that still has to be implemented or refactored before the game can be called (a) production-ready and (b) at full parity with Soldat. Every gap below was verified against the repository on the date above and against the references in [§ 15](#15-references). Items that could not be verified are marked **Needs verification** rather than asserted.

---

## Contents

1. [How to read this document](#1-how-to-read-this-document)
2. [Executive summary](#2-executive-summary)
3. [Already at parity — do not redo](#3-already-at-parity--do-not-redo)
4. [Movement and physics engine](#4-movement-and-physics-engine)
5. [Controls and input defaults](#5-controls-and-input-defaults)
6. [Skeletal animation system](#6-skeletal-animation-system)
7. [Art assets](#7-art-assets)
8. [Maps](#8-maps)
9. [Weapons and combat](#9-weapons-and-combat)
10. [Audio](#10-audio)
11. [HUD and interfaces](#11-hud-and-interfaces)
12. [Game modes](#12-game-modes)
13. [Networking, security, and operations](#13-networking-security-and-operations)
14. [Code health and documentation](#14-code-health-and-documentation)
15. [References](#15-references)
16. [Decisions required](#16-decisions-required)
17. [Release gates](#17-release-gates)

---

## 1. How to read this document

| Priority | Meaning |
|---|---|
| **P0** | Blocks production launch or makes the game feel unlike Soldat. |
| **P1** | Visible parity gap; ship-blocking for a "full parity" claim but not for an invite beta. |
| **P2** | Polish, optional product work, or internal health. |

Each item has an ID (`MOV-01`, `NET-03`, …), the **current state** with file references, the **reference** behaviour with a quotation or source location, the **work** required, and **acceptance criteria**. An item is done only when its acceptance criteria pass in CI and the matching `gap-list.md` line is updated in the same commit, per the existing gap-line protocol.

Source pins used throughout:

- OpenSoldat engine: `opensoldat/opensoldat` `develop` @ `c7596cdca32416cb66339b339105eb1e07e7fbbf` (MIT).
- OpenSoldat content: `opensoldat/base` @ `5b6e5bef23f5c0d58fb1d4d887b9b94ebcf799b4` (CC BY 4.0).

---

## 2. Executive summary

| Area | Parity today | Priority | Section |
|---|---|---|---|
| Weapon statistics (Normal + Realistic) | **At parity** | — | §3 |
| Bonus kits (7) | **At parity** | — | §3 |
| Mode and modifier names | Present; behaviour depth needs per-mode audit | P1 | §12 |
| Movement physics | **Not at parity** — invented constants, scripted roll/backflip | **P0** | §4 |
| Controls | **Not at parity** — defaults and combos differ | **P0** | §5 |
| Animation | **Missing** — 10 procedural poses vs 50 skeletal animations | **P0** | §6 |
| Art | **Missing** — procedural rectangles | **P0** | §7 |
| Maps | **Placeholder** — 97 copies of one template | **P0** | §8 |
| Audio | **Placeholder** — generated tones | P1 | §10 |
| HUD / interfaces | Mostly present; 10 interface packs missing | P1 | §11 |
| Networking for production | Lag compensation, deltas, fuzzing missing | **P0** | §13 |
| Operations / security | External review, DDoS, browser matrix missing | **P0** | §13 |

`cargo test --workspace` passes with zero failures as of 2026-10-07. Passing tests prove the current rules are internally consistent; they do not prove Soldat parity where the rules themselves were invented (§4).

---

## 3. Already at parity — do not redo

These were checked line by line and need no work beyond keeping their fixtures green.

- **Weapon roster.** The wiki: *"There are overall 21 different weapons split into 3 groups"* ([Weapons](https://wiki.soldat.pl/index.php/Weapons)). `crates/game-core/src/weapons/tables.rs` has 20 entries with cluster grenades nested inside the grenade entry, matching *"16 values for each one of the weapons (besides cluster grenades which is nested within frag grenades)"* ([Weapon Mod](https://wiki.soldat.pl/index.php/Weapon_Mod)).
- **Weapon values.** Match `opensoldat/base` `server/configs/weapons.ini` exactly. Spot-checked: Barret M82A1 (`Damage=4.45`, `FireInterval=225`, `Ammo=10`, `ReloadTime=70`, `Speed=55`, `StartUpTime=19`, `Bink=65`), XM214 Minigun (`0.468`, `3`, `100`, `480`, `StartUpTime=25`, `BulletSpread=0.3`), FN Minimi (`0.85`, `9`, `50`, `250`), USSOCOM (`1.49`, `10`, `14`, `60`).
- **Damage formula.** *"[Damage] \* [CurrentSpeed] \* [HitboxModifier]"* and *"CurrentSpeed = Speed + PlayerSpeed \* Inherited Velocity"* ([Weapon Mod](https://wiki.soldat.pl/index.php/Weapon_Mod)).
- **Bonus kits.** `crates/game-core/src/objects/bonus.rs` matches [Bonuses](https://wiki.soldat.pl/index.php/Bonuses) and `Constants.pas`:

| Kit | Wiki | Source constant | Code |
|---|---|---|---|
| Flame God | *"Duration: 10 seconds"* | `FLAMERBONUSTIME = 600` | `600` |
| Berserker | *"four times stronger. Duration: 15 seconds"* | `BERSERKERBONUSTIME = 900` | `900`, `4.0×` |
| Predator | *"Duration: 25 seconds"* | `PREDATORBONUSTIME = 1500`, `PREDATORALPHA = 5` | `1500`, alpha `5` |
| Vest | *"addition of 100% health points"* | `DEFAULTVEST = 100` | `100` |
| Cluster | *"3 grenades"* | `CLUSTER_GRENADES = 3` | `3` |

- **LAW stance rule.** Source (`Sprites.pas` ~L4243) fires the LAW when on ground and in `Crouch` (frame > 13), `CrouchRun`, `CrouchRunBack`, or `Prone` (frame > 23). `weapons/firing.rs::law_is_braced` accepts crouching or prone, which is correct in principle. Fandom's *"Can be used only from the crouch position"* is incomplete. The frame gating is tracked in `WPN-04`.
- **Jet fuel from map.** `lib.rs` L643 derives fuel capacity from the map's `start_jet`.
- **HUD bar semantics** for health (red), ammo/reload (yellow), and jet fuel (blue) match *"Red: Health / Yellow: Ammunition or Reload Time / … / Blue: Jet Fuel"* (fandom *Game Screen*). Fire-interval colour is tracked in `HUD-01`.

---

## 4. Movement and physics engine

> Soldat's identity is its movement. This is the single largest parity gap and the highest-risk refactor.

### MOV-01 — Replace invented locomotion with the source particle model · **P0** · Refactor

**Current state.** `crates/game-core/src/character/movement.rs::MovementConfig::soldat_default` drives the simulation with hand-tuned pixel-per-second values: `gravity: 950.0`, `max_run_speed: 270.0`, `jump_speed: 390.0`, `ground_acceleration: 1050.0`, `air_acceleration: 350.0`, `jet_acceleration: 1400.0`, `ground_friction: 0.82`. The fields `source_run_speed: 0.118`, `source_jump_speed: 0.66`, `source_jet_speed: 0.10` exist but are read only by `tests/movement_fixtures.rs` (asserting they equal themselves). They never influence motion.

**Reference.** OpenSoldat `shared/Constants.pas`:

```text
RUNSPEED       = 0.118;
RUNSPEEDUP     = RUNSPEED / 6;
FLYSPEED       = 0.03;
JUMPSPEED      = 0.66;
CROUCHRUNSPEED = RUNSPEED / 0.6;
PRONESPEED     = RUNSPEED * 4.0;
ROLLSPEED      = RUNSPEED / 1.2;
JUMPDIRSPEED   = 0.30;
JETSPEED       = 0.10;
PARA_SPEED     = -0.5 * 0.06; // Grav
```

These are per-tick **forces** applied to a particle system (`SpriteParts.Forces[...]`, `SpriteParts.Velocity[...]` in `shared/mechanics/Control.pas` and `Sprites.pas`), not velocities. Example, `Control.pas` ~L1634: `Spriteparts.Forces[SpriteC.Num].X := SpriteC.Direction * ROLLSPEED`.

**Work.**

1. Port the sprite particle integration (forces → velocity → position, with the source's damping and gravity) into `character/` as the only locomotion path. Keep it deterministic and safe Rust.
2. Apply every force constant above from the place the source applies it (`Control.pas` run, jump, side-jump, roll, jet, prone branches).
3. Delete `max_run_speed`, `jump_speed`, `ground_acceleration`, `air_acceleration`, `jet_acceleration`, `max_jet_speed`, `ground_friction`, `roll_speed`, `backflip_speed`, and the dead `source_*` fields once the port lands.
4. Re-derive `MovementConfig::realistic()`; its current values (`max_run_speed = 230`, `jump_speed = 350`, `air_acceleration = 280`) are not taken from any source. **Needs verification** of whether Realistic changes locomotion at all in OpenSoldat.
5. Refresh determinism digests (`tests/fixtures/parity/*.json`, `crates/game-core/tests/determinism.rs`) and Wasm parity.

**Acceptance.**

- Golden trajectories recorded from an OpenSoldat build (same map, same input sequence, ≥ 300 ticks) match native and Wasm within a stated tolerance for: standing jump, side jump, run-to-stop, jet climb, prone slide, roll, backflip.
- No locomotion constant exists in the codebase that is not traceable to a named source symbol.

### MOV-02 — Animation-frame-driven movement state · **P0** · Refactor · depends on ANI-01

**Current state.** `CharacterState` is a hand-written table with fixed timers: `roll_ticks: 15`, `backflip_ticks: 15`, `prone_transition_ticks: 26`, `get_up_ticks: 16`, `late_backflip_after_ticks: 12`.

**Reference.** The source gates movement on the current legs/body animation and its frame: `LegsAnimation.ID = Prone.ID and LegsAnimation.CurrFrame >= 24` (roll out of prone, `Control.pas` ~L1668), `Crouch` frame > 13 for the LAW (`Sprites.pas` ~L4245), roll ending in `Control.pas` ~L2068–2110 (*"Gracefully end a roll animation"*, *"Was probably a backflip"*).

**Work.** Drive state from the animation player built in `ANI-01` (current animation ID + frame) instead of tick counters. Remove the fixed timers.

**Acceptance.** Fixture per transition (stand→crouch, stand→prone, prone→stand, prone→roll, run→roll, side-jump→backflip, roll→fall) asserting the same frame at which the source permits the next action.

### MOV-03 — Correct roll trigger · **P0**

**Current state.** Roll is a dedicated `roll` action bound to `C` (`apps/web/src/input/bindings.ts` L27).

**Reference.** Wiki: *"Roll - [Directional + S] - Rolls in a particular direction. Can be performed while standing, or out of the prone position"* ([Default Controls](https://wiki.soldat.pl/index.php/Default_Controls)). Source, `Control.pas` ~L1659: `else if SpriteC.Control.Right and SpriteC.Control.Down then … if SpriteC.OnGround then // roll to the side`, allowed from `Run`, `RunBack`, `Fall`, `ProneMove`, or `Prone` frame ≥ 24.

**Work.** Trigger roll from direction + crouch on ground, with the source's animation preconditions. Choose `Roll` vs `RollBack` from facing (`SpriteC.Direction`), as the source does. Remove the default `roll` binding (keep it as an optional accessibility binding if desired).

**Acceptance.** Holding `D` then pressing `S` while running produces a roll; pressing `C` with default bindings does nothing.

### MOV-04 — Correct backflip trigger and late backflip · **P0**

**Current state.** `apps/web/src/input/encoder.ts::backflipRequested` returns true for the `backflip` key (`Z`) or for `airborne && jump && crouch`. In `movement.rs`, a roll press while airborne becomes `Backflip` with a fixed speed.

**Reference.** Wiki: *"Backflip - Jump backwards and press the Jet button"*; *"Late Backflip - If you are making a jump to the left/right side and keep W pressed, then you can make a backflip in air at any time"* ([Default Controls](https://wiki.soldat.pl/index.php/Default_Controls)). Tutorial: *"While facing backward (relative to your Side Jump direction) and holding the jump button mid-air during the jump, click the jet button."* Source, `Control.pas` ~L350:

```text
if SpriteC.Control.Jetpack and
   (((SpriteC.LegsAnimation.ID = JumpSide.ID) and
     (((SpriteC.Direction = -1) and SpriteC.Control.Right) or
      ((SpriteC.Direction = 1) and SpriteC.Control.Left) or
      PlayerPressedLeftRight)) or
    ((SpriteC.LegsAnimation.ID = RollBack.ID) and SpriteC.Control.Up)) then
  … BodyApplyAnimation(RollBack, 1); LegsApplyAnimation(RollBack, 1);
```

**Work.** Backflip = jet pressed during `JumpSide` while facing opposite to the held direction, continuing while jump is held. Remove jump+crouch-in-air as a backflip trigger (jump+crouch is the flag throw, §5). Remove the default `backflip` binding.

**Acceptance.** Fixtures for basic, reverse, late, and inverse backflip using only move/jump/jet/aim inputs.

### MOV-05 — Parachutes · **P0**

**Current state.** No parachute code anywhere in `crates/` or `apps/`.

**Reference.** Tutorial: *"Parachutes automatically activate in maps where the spawn point is well above the ground. Parachutes cannot be summoned. … you can disengage parachute at any time by pressing the jet button."* Source: `TSprite.Parachute` is called on respawn (`Sprites.pas` ~L3763). It raycasts `PARA_DISTANCE = 500` below the spawn and, if no ground is found within `PARA_DISTANCE - 10`, creates `OBJECT_PARACHUTE = 23` held by the player (`Sprites.pas` ~L3835–3870). Descent uses `PARA_SPEED = -0.5 * 0.06`.

**Work.** Add parachute as a held object: spawn check, descent force, left/right steering, jet to detach, rendering (shirt-coloured canopy as in source), protocol field, bot handling.

**Acceptance.** A spawn ≥ 500 units above ground deploys a parachute; one below does not; jet detaches; descent speed matches source fixture.

### MOV-06 — Emergent movement techniques acceptance suite · **P0** · depends on MOV-01..04

**Current state.** Not reproducible, because roll and backflip are scripted with fixed velocities.

**Reference.** [Soldat's Comprehensive Movement Tutorial](https://wiki.soldat.pl/index.php/Soldat%27s_Comprehensive_Movement_Tutorial):

- Kick jump: *"Hold a directional button, and tap the jump button (default: [W]) every time you touch the ground."*
- Cannonball: *"Cannonball is one of the fastest horizontal motions in Soldat."*
- Superman: *"just lie down and start jetting"*.
- Prone-cancel: *"the move that ties every movement in Soldat together"*.
- Jet roll, glide roll, hard roll, incline roll, mini-flip, micro-flip, prone gliding, poly catching, ice sliding (*"simply lay down on ice and move"*), Spas boost (*"Each shot from a spas boosts your player's model"*).

**Work.** No feature code: these must emerge from MOV-01..04. Build an input-script fixture per technique on a purpose-built test map (the tutorial's practice maps `ctf_jumptut`, `ctf_learntheflip`, `freetime12`, `kz_Climbtut` are a guide).

**Acceptance.** Each technique's scripted input reproduces the source trajectory class (e.g. cannonball horizontal speed > run speed; superman lateral speed > held-direction airborne speed; prone-cancel stands faster than the get-up animation).

### MOV-07 — Collision behaviour that movement depends on · **P1** · Needs verification

The tutorial relies on collision quirks: *"Due to the way collision works in Soldat, jumping upon landing facing forward is faster than facing backwards … (~5% increase on average)"*, poly-corner catching, *"polybugged (launched)"* on corners. Verify `collision/` reproduces these once MOV-01 lands; add fixtures or record intentional differences.

---

## 5. Controls and input defaults

### INP-01 — Match Soldat default bindings · **P0**

**Current state** (`apps/web/src/input/bindings.ts`) vs **reference** ([Default Controls](https://wiki.soldat.pl/index.php/Default_Controls), soldat.pl):

| Action | Soldat default | Repo default | Change |
|---|---|---|---|
| Jump | `[W]` | Space, W, ArrowUp | Keep W first; Space optional |
| Jet | `[RMB]` | Left/Right Shift | **Move to RMB** |
| Throw grenade | `[E]` | E, **RMB** | Remove RMB |
| Throw weapon / knife | `[F]` (*"the more you hold, the further it throws"*) | `T` (throw), `V` (knife), `G` (drop) | **Move to F**; one charged action |
| Pick up | not a bound key in the wiki list | `F` | **Needs verification** whether pickup is automatic in source; free `F` |
| Switch to secondary | `[Q]` | Q | OK |
| Reload | `[R]` | R | OK |
| Chat | `[T]` | Enter | **Move to T** |
| Team chat | `[Y]` | Y | OK |
| Console | `[/]` | Backquote | **Move to /** |
| Flag throw | `[W+S]` (*"Hold jump key + Crouch key"*) | `B` (+ jump+crouch on ground) | Keep combo, drop `B` default |
| Roll / Backflip | combos (§4) | `C` / `Z` | Remove defaults |
| Primary weapon select | `[1]`…`[0]` | Digit1…Digit0 | OK |
| Secondary weapon select | `[CTRL+1]`…`[CTRL+4]` | `P` (single action) | **Add Ctrl+1..4** |
| Scoreboard | `[F1]` | Tab | **Add F1** |
| Weapon stats / sniper line | `[F2]` / `[ALT+F2]` | F4 / L | **Move** |
| Minimap / performance stats | `[F3]` / `[ALT+F3]` | N / F3 | **Move** |
| Screenshot | `[F4]` | F2 | **Move** |
| Music toggle / prev / next | `[F5]` / `[F6]` / `[F7]` | F5 / F6 / F7 | OK (see INP-02) |
| Demo record / fast-forward | `[F8]` | F8 / F9 | Merge to F8 |
| Minimize | `[F9]` | F10 | **Move** (see INP-02) |
| Pause | `[F10]` | Pause | **Move to F10** |
| Taunts | `[ALT] + A,B,C… or 1,2,3…0` | H, J, K, U (four emotes) | **Add Alt+key taunt slots** |
| Mouse sensitivity ±5% | `[CTRL+ALT] + [+/-]` | `]` / `[` | **Move** |
| Volume ±10% | `[SHIFT] + [+/-]` | `=` / `-` | **Move** |
| Scoreboard scroll | `[Pg Up/Pg Down]` | PageUp/PageDown, wheel | OK |
| Game menu | `[ESC]` | — | **Needs verification** |

Note: the fandom *Default Controls* page disagrees with soldat.pl on F2/F3 (fandom: *"Toggle minimap / Show/hide sniper line [F2] / [ALT + F2]"*, *"View statistics / Hide player names [F3] / [ALT + F3]"*). See [§16](#16-decisions-required).

**Acceptance.** `apps/web/scripts/input.test.mjs` asserts the full default table above; `BINDINGS_VERSION` bumps and `profiles.ts::migrateProfile` resets only untouched defaults.

### INP-02 — Browser-reserved keys · **P0**

F5 (reload page), Ctrl+1..4 (switch browser tab), Ctrl+W, F11, and Alt combinations are reserved by browsers. Use `preventDefault` while the canvas has focus and the [Keyboard Lock API](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard_API) in fullscreen where supported. Document fallbacks where a key cannot be captured (e.g. Ctrl+W, and Keyboard Lock on non-Chromium browsers).

**Acceptance.** Browser test per engine confirms which Soldat defaults are captured; uncapturable ones have documented alternates shown in the Controls screen.

### INP-03 — Mobile controls follow the new action model · **P1** · depends on MOV-03/04

`MobileControls.vue` exposes roll and stance buttons. After MOV-03/04, roll and backflip are combos; touch needs explicit buttons that synthesize the combo (direction+crouch; jet during side jump).

---

## 6. Skeletal animation system

### ANI-01 — POA animation loader and player · **P0** · New

**Current state.** `apps/web/src/render/gostek.ts` defines 10 poses (`stand, walk, run, crouch, prone, roll, jet, fall, dead, emote`). The only motion is a sine leg swing (`WALK_PERIOD = 24`). The simulation has no animation concept.

**Reference.** `opensoldat/base` ships 50 animation files in `shared/anims/` (plus `template.poa`). They are skeletal and are used by both simulation (`LegsAnimation`, `BodyAnimation`, `CurrFrame`) and rendering:

```text
barret, bezbroni, biega, biegaranny, biegatyl, bije, celuje, celujeodrzut, change, chat,
cieszy, cigar, clipin, clipout, firemode, gora, goraodrzut, kolba, krocze, kuca, kucaidzie,
kucaidzietyl, laduje, lezy, lezyidzie, match, odrzut, odrzut2, pistolet, rucha, rzuca, samo,
samo2, shotgun, skok, skokdolobrot, skokdolobrottyl, skokwbok, slideback, smoke, spada,
stoi, strzala, stunt, szcza, takeoff, wipe, wstaje, wyrzuca, zmienbron
```

**Work.**

1. Bounded, safe `.poa` parser in `crates/content` (same limits/typed-error pattern as `pms.rs`), with truncation and mutation tests.
2. Animation player in `game-core` (ID, frame, speed, loop, legs/body split) shared by simulation and Wasm.
3. Map every source animation ID (`Run`, `RunBack`, `Stand`, `Crouch`, `CrouchRun`, `CrouchRunBack`, `Prone`, `ProneMove`, `Roll`, `RollBack`, `JumpSide`, `Fall`, reload, change weapon, throw, punch, stab, aim, recoil, emotes, …) to its `.poa` file. **Needs verification**: the ID→file table lives in the OpenSoldat source (`Anims` loading code).
4. Renderer draws the skeleton from snapshot animation ID + frame.

**Acceptance.** All 50 files parse; a golden test asserts skeleton point positions for selected frames; `GameClient` renders every animation without leaks (`renderer-leak.test.mjs`).

### ANI-02 — Per-action animations · **P0** · depends on ANI-01

Wire reload (`clipin`, `clipout`, `laduje`), change weapon (`zmienbron`), throw (`rzuca`, `wyrzuca`), punch/stab, aim and recoil (`celuje`, `celujeodrzut`, `odrzut`, `odrzut2`), weapon-specific holds (`barret`, `pistolet`, `shotgun`, `bezbroni`), the emotes behind `/SMOKE`, `/TABAC`, `/TAKEOFF`, `/VICTORY`, `/MERCY` (`smoke`, `cigar`, `takeoff`, `cieszy`, `match`), and the movement set. Remove `Emote { ticks_left: 90 }` in `movement.rs` in favour of animation length.

---

## 7. Art assets

### ART-01 — Asset strategy decision · **P0** · Decision

**Current state.** In-match visuals are procedural (`docs/provenance.md`: *"Match visuals remain procedural WebGL shapes"*). The soldier is about 13 coloured rectangles (`gostek.ts::buildRig`); every weapon is drawn as the same `24 × 5` bar (`add('weapon', …, 24, 5, [0.2, 0.19, 0.17, 1], angle)`). `docs/parity/intentional-differences.md` lists art under **"Unlicensed content"**.

**Correction.** `opensoldat/base` is licensed **CC BY 4.0** (`LICENSE.txt`: *"Attribution 4.0 International"*), which permits redistribution with attribution. The real blocker is that its credits are self-described as incomplete (README: *"See an incomplete listing of credits in Credits.md"*), so per-asset authorship must be established before shipping. Original Soldat 1.x content outside `opensoldat/base` is a separate question.

**Options.** (a) Import `opensoldat/base` assets with per-asset attribution review; (b) commission original assets at Soldat's skeleton and sprite dimensions; (c) a mix. See [§16](#16-decisions-required).

### ART-02 — Asset import pipeline · **P0** · depends on ART-01

Volume in `opensoldat/base` to cover:

| Folder | Files |
|---|---|
| `shared/gostek-gfx` | 126 |
| `shared/weapons-gfx` | 106 |
| `shared/sparks-gfx` | 72 |
| `shared/scenery-gfx` | 447 |
| `shared/textures` | 89 |
| `shared/interface-gfx` | 50 |
| `shared/custom-interfaces` | 186 |
| `shared/objects-gfx` | 2 |
| `shared/sfx` | 167 (§10) |

**Work.** Extend `render/manifest.ts` and `importedAssets()` with a provenance row per file (path, upstream commit, author, licence, modifications, displayed attribution) as `docs/provenance.md` already requires; generate texture atlases; extend `provenance.test.mjs` to fail on any imported file without a complete row; add an in-game credits screen (also an open item from Task 20).

### ART-03 — Gostek rendering from sprites · **P0** · depends on ANI-01, ART-02

Replace rectangles with per-part sprites on the skeleton: legs, thighs, feet, arms, hands, torso, head, hair styles, headgear, chains, jets, and team/appearance colourisation. Keep `Appearance` and the existing customisation fields.

### ART-04 — Per-weapon sprites, muzzle, casings, projectiles · **P0** · depends on ART-02

One held sprite and one ground sprite per weapon (ground weapons correspond to `OBJECT_USSOCOM = 4` … `OBJECT_STATIONARY_GUN = 27`), per-weapon muzzle offsets, casing sprites, and projectile sprites (bullet, pellet, M79 grenade, LAW missile, arrow, flame arrow, flame, knife, cluster fragments).

### ART-05 — Objects, kits, flags, parachute · **P1** · depends on ART-02

Sprites for the three flags (`OBJECT_ALPHA_FLAG`, `OBJECT_BRAVO_FLAG`, `OBJECT_POINTMATCH_FLAG`), seven kits (`OBJECT_MEDICAL_KIT` … `OBJECT_CLUSTER_KIT`), the Rambo bow object (`OBJECT_RAMBO_BOW = 15`), and the parachute (`OBJECT_PARACHUTE = 23`).

### ART-06 — Sparks, gore, scenery, weather · **P1** · depends on ART-02

Replace procedural particles with `sparks-gfx`, and render map scenery from `scenery-gfx` (§8).

### ART-07 — Remove or repurpose generated placeholders · **P2**

`apps/web/public/art/soldier.png` (loaded in `game.ts` L172) and `battlefield.png` (CSS backdrop in `style.css` L79) are ImageGen placeholders. Remove them from the match once ART-03 and MAP-02 land; keep `arena-key-art.png` for the landing page if desired.

---

## 8. Maps

### MAP-01 — Replace the 97 template maps · **P0**

**Current state.** `tools/map-editor/src/catalog.rs::original_default_map` builds every one of the 97 maps from one template: a `1200 × 700` floor made of two triangles plus two sloped platforms at `390 + seed % 100`, empty `scenery`, `props`, and `colliders`, a `generated.ppm` texture, and two waypoints. Only the names are real.

**Reference.** [Default Maps](https://wiki.soldat.pl/index.php/Default_Maps): *"Deathmatch Teammatch Rambomatch & Pointmatch Maps: 29 / Capture the Flag Maps: 32 / Hold the Flag Maps: 19 / Infiltration Maps: 17 / Overall Default Maps: 97"*. `opensoldat/base` `shared/maps/` contains 99 `.pms` files and 1 `.txt`.

**Work.** Depending on ART-01: load the real `.pms` files through the existing `content::pms` parser and validation, or author original maps with comparable size, flow, spawn layout, and objective placement. Either way, each map is admitted individually (provenance row, mode validation, spawn/objective checks, preview, bot waypoint smoke test), as Task 5 already specifies.

**Acceptance.** No map ships from `original_default_map`; each of the 97 names maps to a unique layout; `gap-list.md` map lines are re-marked only per map.

### MAP-02 — Map rendering fidelity · **P0** · depends on MAP-01, ART-02

Polygon texture UVs and vertex colours, background gradient, scenery layers (front/back), and polygon types (normal, one-way, bouncy, ice, deadly, only-player, only-bullet), checked visually against the source renderer.

### MAP-03 — Bot waypoints on real maps · **P1** · depends on MAP-01

The template provides two waypoints. Real `.pms` files carry waypoint graphs (see [Map Waypoints](https://wiki.soldat.pl/index.php/Map_Waypoints)). Verify `bots/navigation.rs` on every admitted map.

### MAP-04 — Infiltration asymmetric spawns · **P1** · depends on MAP-01

The plan doc records *"Asymmetric spawns wait for maps built for the mode."* Close once the 17 `inf_` maps exist.

---

## 9. Weapons and combat

### WPN-01 — Barrett sniper mode · **P0**

**Current state.** No scope, zoom, or sniper camera code exists (`rg -i "scope|zoom|sniper" crates` finds only chat scope, a bot profile name, and a statistics fixture).

**Reference.** Fandom (official description): *"This sniper weapon has incredible power. … Sniper mode available if you crouch or go prone."*

**Work.** Implement the source's crouch/prone camera behaviour for the Barrett (**Needs verification**: exact camera offset rules in the OpenSoldat client source). The camera is client-side, but Realistic visibility must stay server-enforced (`snapshot_for`).

### WPN-02 — Rambo bow as a real object · **P1**

**Current state.** `lib.rs` L673: *"The yellow flag \*is\* the bow"* — Rambomatch reuses the yellow objective flag as a stand-in.

**Reference.** The source has a dedicated `OBJECT_RAMBO_BOW = 15` and `BOW_INTEREST_TIME = SECOND * 41 + 40`. Fandom: *"The famous Bow of John Rambo available in Rambomatch mode. … You can change to exploding arrows with the change weapon key."*

**Work.** Replace the flag stand-in with a bow object (spawn, pickup, drop, respawn), apply `BOW_INTEREST_TIME` to bot interest, and confirm that the change-weapon key swaps Rambo Bow ↔ Flamed Arrows (`lib.rs` L686–697 handles ownership; **Needs verification** of the swap input).

### WPN-03 — Weapon-specific animations and holds · **P0** · depends on ANI-02, ART-04

Covered by ANI-02 and ART-04; listed here so the weapon owner signs off on each weapon individually (21 sign-offs).

### WPN-04 — LAW and stance timing from animation frames · **P1** · depends on MOV-02

The source gates the LAW on `Crouch` frame > 13 and `Prone` frame > 23. The repo checks only the state. Move to frame gating.

### WPN-05 — Predator audibility · **P2** · Decision

The wikis disagree. soldat.pl: *"Enemies can still hear sounds you make"*. Fandom: *"their gunfire remains inaudible"*. The repo follows soldat.pl (`bonus.rs::predator_is_audible() == true`). Resolve against the source `SFX` code path and record the result.

### WPN-06 — Weapon-mod hash compatibility with OpenSoldat · **P2** · Needs verification

`weapon_config_fixtures` asserts canonical hashes `911431262` / `2707142329`. Confirm these equal the values produced by OpenSoldat `CreateWMChecksum` for the same tables; if they are a project-specific hash, document that.

---

## 10. Audio

### AUD-01 — Real sound effects · **P1** · depends on ART-01

**Current state.** `apps/web/src/audio/manifest.ts` defines 26 generated clips. One `fire` clip (`kind: 'shot', hz: 220, seconds: 0.08`) plays for every weapon.

**Reference.** `opensoldat/base` `shared/sfx` has 167 files. The source defines per-event IDs (e.g. `SFX_GRENADE_THROW = 19`, `SFX_GRENADE_EXPLOSION = 20`, `SFX_GRENADE_BOUNCE = 21`, `SFX_CLUSTERGRENADE = 62`, `SFX_GRENADE_PULLOUT = 64`, `SFX_DIST_GRENADE = 80`).

**Work.** Replace `ClipId` with the source SFX ID set; per-weapon fire, reload, empty, and distant variants; roll (`SFX_ROLL` is played in `Control.pas`), jets, footsteps per map `steps`, kits, flags, gore. Keep the existing positional engine, distance attenuation, and deafness.

**Acceptance.** Each simulation event maps to a source SFX ID; no generated tone remains in a production build.

### AUD-02 — Music · **P2**

Playback controls exist (`audio/music.ts`). Add licensed tracks or keep music off by default.

---

## 11. HUD and interfaces

### HUD-01 — Fire-interval bar colour · **P2**

`hud/gauges.ts` L33 sets `FIRE_INTERVAL_COLOR = [0.78, 0.78, 0.82, 1]` (silver). Reference: *"Small Gold: Fire Interval Time"* (fandom *Game Screen*). `layout.ts` L161 also hides `fire-interval` in one preset; confirm that matches the default Soldat HUD.

### HUD-02 — Ten classic interface packs · **P1** · depends on ART-01

`gap-list.md` `G33-019`…`G33-028` (Cabbage, Classic, Lacey V2, Micro1, Military, Predator, Soldat Style, Storm, Tech, Text) are `Missing`. `opensoldat/base` `shared/custom-interfaces` has 186 files. Import through the existing `content::mod_package` path, with attribution.

### HUD-03 — Player status block · **P2** · Needs verification

Reference: *"The top green number indicates the player's rank … The middle red number shows the player's total kills/points and the difference from the leading player. The purple number represents the game's Kill Limit"*, plus a ping-coloured lag-o-meter dot. Verify colours and placement in `hud/status.ts`.

### HUD-04 — Dedicated screens still open · **P1**

From Task 20: *"Credits, help, pause overlay, and onboarding tutorials stay open as dedicated screens."* Credits becomes mandatory once CC BY assets ship (ART-02).

---

## 12. Game modes

### MOD-01 — Behavioural audit per official mode · **P1** · Needs verification

All seven modes and three modifiers exist and are tested against the repo's own rules. Audit each against the source (scoring, limits, respawn, flag rules, timers such as `FLAG_TIMEOUT = SECOND * 25`, `SURVIVAL_RESPAWNTIME = SECOND * 5`, `CAPTURECTFMESSAGEWAIT = SECOND * 7`) and add a source-pinned fixture per rule that differs.

### MOD-02 — Community modes as separate acceptance targets · **P2**

Task 14 notes that all eleven (*"Climb, Dodgeball, Domination, Hide and Seek, Knife Only, OneShots, Pirates vs Ninjas, RS/CS, Trench Wars, Tactical Trench Wars, and Zombie"*) *"landed together"* as presets in `modes/scripted.rs`. These were community scripts with no single canonical ruleset; write a one-paragraph spec per mode with its source (script or forum post) and an acceptance test per spec.

### MOD-03 — Realistic movement and Survival scoreboard · **P1**

The plan doc leaves `G11-009` (*"Realistic movement and survival tuning stays open pending movement fixtures of its own"*) open. Resolve with MOV-01 step 4.

---

## 13. Networking, security, and operations

These match the remaining `Missing` lines in `gap-list.md` plus blockers recorded in `README.md` and `docs/ops/`.

### NET-01 — Server rewind / lag compensation · **P0** (`G30-008`)

Hit registration for a fast hitscan-heavy shooter needs rewind to the shooter's view time. Implement bounded history per player, rewind window cap, and anti-abuse limits. **Acceptance:** under simulated 100 ms RTT, a shot on a strafing target registers when it hits on the shooter's client.

### NET-02 — Snapshot delta compression · **P0** (`G30-012`)

**Current state:** full JSON snapshots. Add per-client baseline acks and deltas. **Acceptance:** measured bandwidth per client at 16 players stays within a written budget.

### NET-03 — Interest management · **P1** (`G30-013`)

Needed once real maps (MAP-01) are larger than the `1200 × 700` template.

### NET-04 — Binary encoding · **P2** (`G30-014`)

Only if NET-02 misses the budget; keep golden compatibility tests.

### NET-05 — Soak, loss, and 16-player load · **P0**

The plan doc says *"Protocol fuzz and a long multi-hour soak stay launch hardening."* Run multi-hour soak and loss/jitter suites in CI or as a release job.

### SEC-01 — Protocol fuzzing · **P0** (`G31-015`)

Add a `cargo-fuzz` (or in-tree bounded-mutation) harness for every `ClientMessage` variant, mirroring `crates/content/tests/pms.rs`.

### SEC-02 — Deployment DDoS protection · **P0** (`G31-020`)

Required before open-internet launch: an edge provider or WAF in front of Caddy, connection caps, and WebSocket upgrade rate limits.

### SEC-03 — External security review · **P0**

Task 25 leaves *"Add deployment DDoS controls and an external security review gate before public launch"* unchecked.

### OPS-01 — Browser compatibility matrix · **P0** (`G36-024`)

Playwright across Chromium, Firefox, and WebKit, including mobile emulation, touch, and the key-capture checks from INP-02.

### OPS-02 — Asset CDN and caching · **P1** (`G36-031`)

Becomes P0 once ART-02 adds hundreds of files: content-hashed URLs, immutable cache headers, compression.

### OPS-03 — Match continuity across restarts · **P1**

README: *"Match state is in memory; restarting ends active matches."* Either accept that (document `/DRAIN` as the only deploy path, which is current behaviour) or snapshot worlds on drain.

### OPS-04 — Self-host fonts · **P2**

`docs/provenance.md`: *"Font loading uses Google Fonts; replace or self-host before a release that needs offline operation."*

### OPS-05 — Image digest rollback evidence · **P1**

Task 29: *"External security review, dependency scanners, CDN, and image-digest rollback stay launch work."* Automate digest recording (`scripts/record-image-digests.mjs`) in the release workflow and test a rollback.

### OPS-06 — Dependency scanning · **P1**

Add `cargo audit` / `cargo deny` and a JS dependency audit to CI.

### ACC-01 — Accounts, authentication, database, clan stats · **P2** · Optional

`G22-024`, `G36-002`, `G36-010`, `G36-011`, `G29-023`. Only if the product needs persistent identity. The current `DATA_DIR/store.json` is adequate for an invite beta.

---

## 14. Code health and documentation

### REF-01 — Split `game-core/src/lib.rs` · **P1** · Refactor

3,234 lines. The plan's architecture rule says *"`World::step` becomes orchestration over these modules; it must not remain a monolithic rules implementation."* Move mode glue (e.g. `sync_rambo_bow`), map loading, and per-tick orchestration into their modules.

### REF-02 — Split `apps/api/src/main.rs` · **P1** · Refactor

1,970 lines. The plan named `config`, `hub`, `room`, `admin`, `storage`, `lobby`, `telemetry`; `admin`, `audit`, `bans`, `config`, `lobby`, `maps`, `ops`, `security`, `storage` exist, but room/hub handling still lives in `main.rs`.

### REF-03 — Line endings and Prettier · **P2**

Known defect since Task 17: `prettier --check` fails across `apps/web` because of CRLF line endings. Add `.gitattributes` (`* text=auto eol=lf`), renormalise in a dedicated commit, and make `prettier --check` a CI gate.

### DOC-01 — Correct the parity ledger · **P0**

`gap-list.md` marks lines `Present` that are placeholders (97 maps, art, animations, movement fixtures that assert invented constants). Re-audit and downgrade them to `Partial` with a reason, so `check:gaps` stops overstating parity. Extend `scripts/check-gap-coverage.mjs` to reject `Present` on rows whose evidence is a placeholder generator.

### DOC-02 — Correct `intentional-differences.md` · **P1**

Replace *"Unlicensed content"* for `opensoldat/base` assets with the accurate status (CC BY 4.0, attribution review pending), and add the movement and controls differences until §4–5 land.

### DOC-03 — README status · **P2**

The README's status says *"The first arena, two modes, ten normal-mode primary weapons, four secondary weapons, and frag grenades work"*, which is stale (seven modes, three modifiers, and all 21 weapons now exist). Rewrite it after DOC-01.

### DOC-04 — Agent instruction files · **P2**

`AGENTS.md` is empty and `CALUDE.md` (typo of `CLAUDE.md`) contains only `@AGENTS.md`. Fill `AGENTS.md` with the verification commands and constraints from the plan doc, and rename the file.

---

## 15. References

- OpenSoldat engine, MIT: <https://github.com/opensoldat/opensoldat> @ `c7596cdca32416cb66339b339105eb1e07e7fbbf`. Files cited: `shared/Constants.pas`, `shared/mechanics/Control.pas`, `shared/mechanics/Sprites.pas`.
- OpenSoldat base content, CC BY 4.0: <https://github.com/opensoldat/base> @ `5b6e5bef23f5c0d58fb1d4d887b9b94ebcf799b4`. Files cited: `server/configs/weapons.ini`, `shared/anims/`, `shared/maps/`, `shared/sfx/`, `LICENSE.txt`, `README.md`.
- Soldat Community Wiki: [Main Page](https://wiki.soldat.pl/), [Weapons](https://wiki.soldat.pl/index.php/Weapons), [Weapon Mod](https://wiki.soldat.pl/index.php/Weapon_Mod), [Bonuses](https://wiki.soldat.pl/index.php/Bonuses), [Game Modes](https://wiki.soldat.pl/index.php/Game_Modes), [Default Controls](https://wiki.soldat.pl/index.php/Default_Controls), [Default Maps](https://wiki.soldat.pl/index.php/Default_Maps), [Soldat's Comprehensive Movement Tutorial](https://wiki.soldat.pl/index.php/Soldat%27s_Comprehensive_Movement_Tutorial), [Map Waypoints](https://wiki.soldat.pl/index.php/Map_Waypoints).
- Soldat Fandom Wiki: [Soldat Wiki](https://soldat.fandom.com/wiki/Soldat_Wiki) — pages *Default Controls*, *Game Screen*, *Barret M82A1*, *M72 LAW*, *Rambo Bow*, *Predator*, *Berserker*, *Flame God* (retrieved through the MediaWiki API).
- Neither wiki publishes per-weapon numeric stat tables; numeric weapon parity is checked against `weapons.ini`, which the Weapon Mod page names as the source (*"done by editing the weapons.ini and weapons_realistic.ini"*).

---

## 16. Decisions required

| # | Decision | Options | Blocks |
|---|---|---|---|
| D1 | Asset source | Import `opensoldat/base` (CC BY 4.0, attribution review) / commission originals / mix | ART-*, MAP-01, AUD-01, HUD-02 |
| D2 | F2/F3 mapping | soldat.pl (F2 weapon stats, F3 minimap) / fandom (F2 minimap, F3 stats) | INP-01 |
| D3 | Browser-reserved keys | Capture where possible + alternates / alternates only | INP-02 |
| D4 | Keep dedicated roll/backflip keys as optional bindings? | Yes (accessibility) / No (pure parity) | MOV-03, MOV-04 |
| D5 | Predator audibility | Follow source after checking | WPN-05 |
| D6 | Accounts and database | Defer / build | ACC-01 |
| D7 | Match continuity on deploy | Drain-only (current) / world snapshots | OPS-03 |

---

## 17. Release gates

**Invite beta** requires: MOV-01…06, INP-01, INP-02, ANI-01, ANI-02, ART-01…04, MAP-01, MAP-02, WPN-01, NET-01, NET-02, NET-05, SEC-01, OPS-01, DOC-01.

**Public launch** additionally requires: SEC-02, SEC-03, OPS-02, OPS-05, OPS-06, AUD-01, HUD-02, MAP-03, MAP-04, MOD-01.

**"Full Soldat parity" claim** additionally requires every P1 item, plus a signed-off comparison video of each movement technique in §4 MOV-06 against an OpenSoldat build.

For every gate, the existing verification commands in `docs/gaps/2026-09-15-soldat-parity-gap-implementation.md` (*Required verification commands*) must exit zero, plus the browser matrix (OPS-01) and soak (NET-05).
