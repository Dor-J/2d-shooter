# Gameplay rules and parity status

The normal-mode primary and secondary weapon catalog in `crates/game-core/src/weapons.rs` is transcribed from [OpenSoldat `shared/Weapons.pas`](https://github.com/soldat/soldat/blob/develop/shared/Weapons.pas). Names, magazine sizes, fire intervals, reload durations, startup delays, projectile speed values, and hit multipliers match that source. A 60 Hz authoritative server applies weapon choice, firing, ammo consumption, reload, shotgun pellets, explosive weapons, and frag grenades. The browser compiles the same simulation to WebAssembly for local prediction.

Damage conversion, SPAS pellet count and spread, explosion falloff, collision geometry, movement acceleration, jets, and match rules are **not yet exact Soldat ports**. Soldat's damage uses projectile and body-part mechanics beyond the catalog values; this game currently derives a simplified amount from source speed and hit multiplier. Source explosion radii are used for the M79 (64) and frag grenade (85), but this game's 1,200 × 700 arena uses its own scale. The source has separate realistic-mode values, bonus weapons, cluster grenades, thrown knives, and more game modes that are not implemented here.

## Controls

Every control in `docs/gaps/gap-list.md` section 2 is one entry in the `Action` list in
`apps/web/src/input/actions.ts`. Keyboard, mouse, gamepad, and touch devices resolve their bindings
to those actions, a single encoder turns the resulting action state into one protocol input frame,
and client-only controls raise typed interface intents instead. Bindings, mouse sensitivity, audio
levels, and hold-to-toggle preferences live in versioned control profiles that can be created,
renamed, exported, imported, and switched at runtime. Gameplay input is suspended whenever a text
field or overlay owns focus, and every device releases what it holds so no control can stick down.

Combined inputs always have a single-control alternative: a backflip is crouch plus jump in the air
or the dedicated backflip key, and a flag throw is crouch plus jump on the ground or the dedicated
flag-throw key. Any hold control can be switched to a toggle for one-handed play.

Two controls differ from the original on purpose because a browser tab cannot do what the desktop
build does. A page cannot minimise itself, so the minimize control sends the match to the background
and releases input instead. Mouse aiming is absolute — the cursor is the crosshair — so the mouse
sensitivity setting scales stick and pad aiming rather than cursor movement.

## Damage and death

Every unit of damage is one `DamageEvent` handled by `World::apply_damage`, so bullets, pellets,
explosions, melee, falls, bleeding, and deadly polygons all share one path. That path scales damage
by the body region that was hit, spends armor before health, opens a bleeding wound on a heavy hit,
records who hurt the player and when, and resolves the death: the killer, assists inside the
attribution window, headshots, team kills, suicides, multi-kill streaks, and the kill-feed line.
Death leaves a three-segment ragdoll that falls, settles on the terrain, and is cleared after a
configured lifetime; a big enough hit gibs the body instead. Respawn picks a spawn point by mode,
team, and distance from the players you would rather not land next to, and grants a short period of
spawn protection. Blood and gore reach the client as presentation events derived from that same
authoritative damage; nothing cosmetic is ever read back into the simulation.

Region multipliers, bleeding rates, fall-damage thresholds, and armor absorption are this project's
own values in `DamageConfig`, not yet a port of Soldat's per-projectile damage model.

Before claiming gameplay parity, port the relevant routines from `shared/mechanics`, `shared/PolyMap.pas`, `shared/Game.pas`, and client/server rule paths, then create source-versus-Rust replay fixtures for movement, collisions, every weapon style, damage zones, flags, and respawn. Exact original map and art reuse also requires the asset provenance review in `docs/provenance.md`.
