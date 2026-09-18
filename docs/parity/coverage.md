# Soldat parity coverage

This generated index maps every actionable feature in `docs/gaps/gap-list.md` to a stable ID and acceptance-test identifier. `coverage.json` is the machine-readable authority.

| ID | Status | Section | Feature | Acceptance |
| --- | --- | --- | --- | --- |
| G01-CHARACTER-MOVEMENT-001 | Present | 1. Character movement and physics | Horizontal running. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-002 | Present | 1. Character movement and physics | Jumping. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-003 | Present | 1. Character movement and physics | Jetpack flight. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-004 | Present | 1. Character movement and physics | Crouching. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-005 | Present | 1. Character movement and physics | Prone stance. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-006 | Present | 1. Character movement and physics | Standing-to-prone transition. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-007 | Present | 1. Character movement and physics | Getting up from prone. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-008 | Present | 1. Character movement and physics | Directional ground rolls. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-009 | Present | 1. Character movement and physics | Rolling out of prone. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-010 | Present | 1. Character movement and physics | Standard backflip. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-011 | Present | 1. Character movement and physics | Late backflip. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-012 | Present | 1. Character movement and physics | Soldat-style momentum conservation. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-013 | Present | 1. Character movement and physics | Air-control behavior. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-014 | Present | 1. Character movement and physics | Acceleration matching Soldat. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-015 | Present | 1. Character movement and physics | Ground friction matching Soldat. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-016 | Present | 1. Character movement and physics | Slope movement. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-017 | Present | 1. Character movement and physics | Sliding along slopes. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-018 | Present | 1. Character movement and physics | Polygon-edge interaction. | rust:collision_fixtures:edges |
| G01-CHARACTER-MOVEMENT-019 | Present | 1. Character movement and physics | One-way polygons. | rust:collision_fixtures:one-way |
| G01-CHARACTER-MOVEMENT-020 | Present | 1. Character movement and physics | Bouncy polygons. | rust:polygon_collision:materials |
| G01-CHARACTER-MOVEMENT-021 | Present | 1. Character movement and physics | Ice/slippery surfaces. | rust:polygon_collision:materials |
| G01-CHARACTER-MOVEMENT-022 | Present | 1. Character movement and physics | Deadly polygons. | rust:world_polygon_collision:deadly |
| G01-CHARACTER-MOVEMENT-023 | Present | 1. Character movement and physics | Only-player and only-bullet polygon types. | rust:map_validation:filters |
| G01-CHARACTER-MOVEMENT-024 | Present | 1. Character movement and physics | Player collision with vertical and angled geometry. | rust:collision_fixtures:geometry |
| G01-CHARACTER-MOVEMENT-025 | Present | 1. Character movement and physics | Proper head/body/legs collision volumes. | rust:collision_fixtures:body-shape |
| G01-CHARACTER-MOVEMENT-026 | Present | 1. Character movement and physics | Character-body ragdoll physics. | rust:damage_fixtures:ragdoll |
| G01-CHARACTER-MOVEMENT-027 | Present | 1. Character movement and physics | Corpses interacting with terrain. | rust:game_core:corpse-terrain |
| G01-CHARACTER-MOVEMENT-028 | Present | 1. Character movement and physics | Player-to-player physical interaction. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-029 | Present | 1. Character movement and physics | Fall/impact behavior. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-030 | Present | 1. Character movement and physics | Jet force affected by pose and movement. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-031 | Present | 1. Character movement and physics | Map-specific jet-fuel capacity. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-032 | Present | 1. Character movement and physics | Jet fuel recharge. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-033 | Present | 1. Character movement and physics | Soldat-accurate jet depletion and regeneration. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-034 | Present | 1. Character movement and physics | Weapon recoil affecting the player. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-035 | Present | 1. Character movement and physics | SPAS/minigun self-boost. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-036 | Present | 1. Character movement and physics | Explosive knockback. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-037 | Present | 1. Character movement and physics | Bullet push. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-038 | Missing | 1. Character movement and physics | Flag and kit push from bullets/explosions. | parity:G01-CHARACTER-MOVEMENT-038 |
| G01-CHARACTER-MOVEMENT-039 | Present | 1. Character movement and physics | Movement animation state machine. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-040 | Missing | 1. Character movement and physics | Directional aiming and body rotation. | parity:G01-CHARACTER-MOVEMENT-040 |
| G01-CHARACTER-MOVEMENT-041 | Missing | 1. Character movement and physics | Separate legs, torso, head, arms, weapon, and jet animations. | parity:G01-CHARACTER-MOVEMENT-041 |
| G01-CHARACTER-MOVEMENT-042 | Missing | 1. Character movement and physics | Death animations. | parity:G01-CHARACTER-MOVEMENT-042 |
| G01-CHARACTER-MOVEMENT-043 | Missing | 1. Character movement and physics | Mercy/victory/smoke/tobacco/helmet animations. | parity:G01-CHARACTER-MOVEMENT-043 |
| G02-INPUT-AND-001 | Present | 2. Input and controls | Move left/right. | rust:movement_fixtures |
| G02-INPUT-AND-002 | Present | 2. Input and controls | Jump. | rust:movement_fixtures |
| G02-INPUT-AND-003 | Present | 2. Input and controls | Jet. | rust:movement_fixtures |
| G02-INPUT-AND-004 | Present | 2. Input and controls | Mouse aiming. | web:smoke:aim-input |
| G02-INPUT-AND-005 | Present | 2. Input and controls | Primary fire. | rust:game_core::tests::fire_is_rate_limited |
| G02-INPUT-AND-006 | Present | 2. Input and controls | Grenade input. | rust:game_core::tests::holding_grenade_input_throws_only_once |
| G02-INPUT-AND-007 | Present | 2. Input and controls | Numeric weapon selection. | web:input:weapon-selection |
| G02-INPUT-AND-008 | Present | 2. Input and controls | Chat. | server:chat-handler |
| G02-INPUT-AND-009 | Present | 2. Input and controls | Touch controls. | web:mobile:movement-pad |
| G02-INPUT-AND-010 | Present | 2. Input and controls | Crouch. | web:smoke:stance |
| G02-INPUT-AND-011 | Present | 2. Input and controls | Prone. | web:smoke:stance |
| G02-INPUT-AND-012 | Present | 2. Input and controls | Roll. | web:smoke:stance |
| G02-INPUT-AND-013 | Present | 2. Input and controls | Backflip combinations. | web:input:backflip-combo |
| G02-INPUT-AND-014 | Present | 2. Input and controls | Explicit reload. | web:smoke:reload |
| G02-INPUT-AND-015 | Present | 2. Input and controls | Switch between carried primary and secondary weapons. | rust:weapon_inventory_fixtures |
| G02-INPUT-AND-016 | Present | 2. Input and controls | Drop current weapon. | rust:weapon_inventory_fixtures |
| G02-INPUT-AND-017 | Present | 2. Input and controls | Hold-to-charge weapon throw. | rust:weapon_inventory_fixtures |
| G02-INPUT-AND-018 | Present | 2. Input and controls | Throw combat knife. | rust:weapon_inventory_fixtures |
| G02-INPUT-AND-019 | Present | 2. Input and controls | Pick up weapons. | rust:weapon_inventory_fixtures |
| G02-INPUT-AND-020 | Missing | 2. Input and controls | Pick up flags and kits. | parity:G02-INPUT-AND-020 |
| G02-INPUT-AND-021 | Missing | 2. Input and controls | Flag throw using jump+crouch. | parity:G02-INPUT-AND-021 |
| G02-INPUT-AND-022 | Missing | 2. Input and controls | Dedicated configurable flag-throw key. | parity:G02-INPUT-AND-022 |
| G02-INPUT-AND-023 | Present | 2. Input and controls | Respawn weapon-selection menu. | rust:damage_fixtures:weapon-choice |
| G02-INPUT-AND-024 | Present | 2. Input and controls | Separate primary and secondary selection controls. | web:input:weapon-selection |
| G02-INPUT-AND-025 | Missing | 2. Input and controls | Team chat. | parity:G02-INPUT-AND-025 |
| G02-INPUT-AND-026 | Missing | 2. Input and controls | Command console. | parity:G02-INPUT-AND-026 |
| G02-INPUT-AND-027 | Present | 2. Input and controls | Scoreboard hold/toggle behavior. | web:input:scoreboard |
| G02-INPUT-AND-028 | Missing | 2. Input and controls | Weapon-statistics screen. | parity:G02-INPUT-AND-028 |
| G02-INPUT-AND-029 | Missing | 2. Input and controls | Minimap toggle. | parity:G02-INPUT-AND-029 |
| G02-INPUT-AND-030 | Missing | 2. Input and controls | Sniper-line toggle. | parity:G02-INPUT-AND-030 |
| G02-INPUT-AND-031 | Missing | 2. Input and controls | Performance-statistics overlay. | parity:G02-INPUT-AND-031 |
| G02-INPUT-AND-032 | Present | 2. Input and controls | Screenshot control. | web:input:screenshot |
| G02-INPUT-AND-033 | Missing | 2. Input and controls | Music toggle. | parity:G02-INPUT-AND-033 |
| G02-INPUT-AND-034 | Missing | 2. Input and controls | Previous/next music track. | parity:G02-INPUT-AND-034 |
| G02-INPUT-AND-035 | Missing | 2. Input and controls | Demo recording. | parity:G02-INPUT-AND-035 |
| G02-INPUT-AND-036 | Missing | 2. Input and controls | Demo playback fast-forward. | parity:G02-INPUT-AND-036 |
| G02-INPUT-AND-037 | Missing | 2. Input and controls | Pause. | parity:G02-INPUT-AND-037 |
| G02-INPUT-AND-038 | Present | 2. Input and controls | Window minimize shortcut. | web:input:minimize |
| G02-INPUT-AND-039 | Missing | 2. Input and controls | Taunt shortcuts. | parity:G02-INPUT-AND-039 |
| G02-INPUT-AND-040 | Present | 2. Input and controls | Runtime mouse-sensitivity adjustment. | web:input:sensitivity |
| G02-INPUT-AND-041 | Missing | 2. Input and controls | Runtime sound-volume adjustment. | parity:G02-INPUT-AND-041 |
| G02-INPUT-AND-042 | Present | 2. Input and controls | Scoreboard scrolling. | web:input:scoreboard-scroll |
| G02-INPUT-AND-043 | Present | 2. Input and controls | Fully rebindable keyboard controls. | web:input:rebinding |
| G02-INPUT-AND-044 | Present | 2. Input and controls | Rebindable mouse buttons. | web:input:mouse-rebinding |
| G02-INPUT-AND-045 | Present | 2. Input and controls | Controller/gamepad support. | web:input:gamepad |
| G02-INPUT-AND-046 | Present | 2. Input and controls | Saved control profiles. | web:input:profiles |
| G02-INPUT-AND-047 | Present | 2. Input and controls | Accessibility alternatives for combined inputs. | web:input:accessibility |
| G02-INPUT-AND-048 | Missing | 2. Input and controls | Mobile equivalents for crouch, prone, roll, reload, weapon switch, weapon throw, flag throw, scoreboard, and team chat. | parity:G02-INPUT-AND-048 |
| G03-WEAPONS-001 | Partial | 3. Weapons | Desert Eagles. | parity:G03-WEAPONS-001 |
| G03-WEAPONS-002 | Partial | 3. Weapons | HK MP5. | parity:G03-WEAPONS-002 |
| G03-WEAPONS-003 | Partial | 3. Weapons | AK-74. | parity:G03-WEAPONS-003 |
| G03-WEAPONS-004 | Partial | 3. Weapons | Steyr AUG. | parity:G03-WEAPONS-004 |
| G03-WEAPONS-005 | Partial | 3. Weapons | SPAS-12. | parity:G03-WEAPONS-005 |
| G03-WEAPONS-006 | Partial | 3. Weapons | Ruger 77. | parity:G03-WEAPONS-006 |
| G03-WEAPONS-007 | Partial | 3. Weapons | M79. | parity:G03-WEAPONS-007 |
| G03-WEAPONS-008 | Partial | 3. Weapons | Barrett M82A1. | parity:G03-WEAPONS-008 |
| G03-WEAPONS-009 | Partial | 3. Weapons | FN Minimi. | parity:G03-WEAPONS-009 |
| G03-WEAPONS-010 | Partial | 3. Weapons | XM214 Minigun. | parity:G03-WEAPONS-010 |
| G03-WEAPONS-011 | Partial | 3. Weapons | USSOCOM. | parity:G03-WEAPONS-011 |
| G03-WEAPONS-012 | Partial | 3. Weapons | Combat Knife. | parity:G03-WEAPONS-012 |
| G03-WEAPONS-013 | Partial | 3. Weapons | Chainsaw. | parity:G03-WEAPONS-013 |
| G03-WEAPONS-014 | Partial | 3. Weapons | M72 LAW. | parity:G03-WEAPONS-014 |
| G03-WEAPONS-015 | Missing | 3. Weapons | Cluster grenades. | parity:G03-WEAPONS-015 |
| G03-WEAPONS-016 | Missing | 3. Weapons | Flamethrower. | parity:G03-WEAPONS-016 |
| G03-WEAPONS-017 | Missing | 3. Weapons | Rambo Bow. | parity:G03-WEAPONS-017 |
| G03-WEAPONS-018 | Missing | 3. Weapons | Normal arrows. | parity:G03-WEAPONS-018 |
| G03-WEAPONS-019 | Missing | 3. Weapons | Flamed arrows. | parity:G03-WEAPONS-019 |
| G03-WEAPONS-020 | Missing | 3. Weapons | Stationary M2 machine gun. | parity:G03-WEAPONS-020 |
| G03-WEAPONS-021 | Missing | 3. Weapons | Punch/unarmed combat. | parity:G03-WEAPONS-021 |
| G03-WEAPONS-022 | Present | 3. Weapons | Primary-plus-secondary inventory slots. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-023 | Present | 3. Weapons | Carrying two primary weapons. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-024 | Present | 3. Weapons | Weapon pickups. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-025 | Present | 3. Weapons | Weapon dropping. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-026 | Present | 3. Weapons | Thrown-weapon physics. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-027 | Present | 3. Weapons | Thrown combat knife. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-028 | Present | 3. Weapons | Knife recovery/pickup. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-029 | Present | 3. Weapons | Manual reload. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-030 | Present | 3. Weapons | Reload interruption. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-031 | Missing | 3. Weapons | Per-weapon reload animations. | parity:G03-WEAPONS-031 |
| G03-WEAPONS-032 | Present | 3. Weapons | Weapon-switch delays. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-033 | Present | 3. Weapons | Correct weapon startup behavior. | rust:game_core::tests::fire_is_rate_limited |
| G03-WEAPONS-034 | Missing | 3. Weapons | LAW firing restrictions. | parity:G03-WEAPONS-034 |
| G03-WEAPONS-035 | Missing | 3. Weapons | Barrett movement/startup restrictions. | parity:G03-WEAPONS-035 |
| G03-WEAPONS-036 | Missing | 3. Weapons | Minigun spin-up behavior. | parity:G03-WEAPONS-036 |
| G03-WEAPONS-037 | Missing | 3. Weapons | Chainsaw continuous-contact behavior. | parity:G03-WEAPONS-037 |
| G03-WEAPONS-038 | Missing | 3. Weapons | Proper melee collision. | parity:G03-WEAPONS-038 |
| G03-WEAPONS-039 | Missing | 3. Weapons | Dual Desert Eagle projectiles and muzzle positions. | parity:G03-WEAPONS-039 |
| G03-WEAPONS-040 | Missing | 3. Weapons | Proper shotgun pellet count and randomized spread. | parity:G03-WEAPONS-040 |
| G03-WEAPONS-041 | Missing | 3. Weapons | Projectile lifetime matching Soldat. | parity:G03-WEAPONS-041 |
| G03-WEAPONS-042 | Missing | 3. Weapons | Projectile gravity per bullet style. | parity:G03-WEAPONS-042 |
| G03-WEAPONS-043 | Missing | 3. Weapons | Grenade bouncing. | parity:G03-WEAPONS-043 |
| G03-WEAPONS-044 | Missing | 3. Weapons | M79 projectile bouncing/impact behavior. | parity:G03-WEAPONS-044 |
| G03-WEAPONS-045 | Missing | 3. Weapons | Grenade cooking/throw strength. | parity:G03-WEAPONS-045 |
| G03-WEAPONS-046 | Missing | 3. Weapons | Grenade fuse timing. | parity:G03-WEAPONS-046 |
| G03-WEAPONS-047 | Missing | 3. Weapons | Dropped grenade behavior on death. | parity:G03-WEAPONS-047 |
| G03-WEAPONS-048 | Missing | 3. Weapons | Cluster grenade submunition spawning. | parity:G03-WEAPONS-048 |
| G03-WEAPONS-049 | Missing | 3. Weapons | Arrow sticking/interaction. | parity:G03-WEAPONS-049 |
| G03-WEAPONS-050 | Missing | 3. Weapons | Flame propagation and burning. | parity:G03-WEAPONS-050 |
| G03-WEAPONS-051 | Missing | 3. Weapons | Flamethrower fuel/ammunition behavior. | parity:G03-WEAPONS-051 |
| G03-WEAPONS-052 | Missing | 3. Weapons | Stationary-gun mounting and dismounting. | parity:G03-WEAPONS-052 |
| G03-WEAPONS-053 | Missing | 3. Weapons | Stationary-gun aiming limits. | parity:G03-WEAPONS-053 |
| G03-WEAPONS-054 | Missing | 3. Weapons | Projectile-to-projectile or projectile-to-object interactions where applicable. | parity:G03-WEAPONS-054 |
| G03-WEAPONS-055 | Present | 3. Weapons | Muzzle origin based on character pose. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-056 | Missing | 3. Weapons | Muzzle flashes. | parity:G03-WEAPONS-056 |
| G03-WEAPONS-057 | Missing | 3. Weapons | Shell casings. | parity:G03-WEAPONS-057 |
| G03-WEAPONS-058 | Missing | 3. Weapons | Weapon-specific sounds. | parity:G03-WEAPONS-058 |
| G03-WEAPONS-059 | Missing | 3. Weapons | Reload sounds. | parity:G03-WEAPONS-059 |
| G03-WEAPONS-060 | Missing | 3. Weapons | Empty-magazine sound. | parity:G03-WEAPONS-060 |
| G03-WEAPONS-061 | Missing | 3. Weapons | Bullet impact effects. | parity:G03-WEAPONS-061 |
| G03-WEAPONS-062 | Missing | 3. Weapons | Tracers matching weapon configuration. | parity:G03-WEAPONS-062 |
| G03-WEAPONS-063 | Missing | 3. Weapons | Explosion visual and audio effects. | parity:G03-WEAPONS-063 |
| G03-WEAPONS-064 | Missing | 3. Weapons | Weapon sprites held by characters. | parity:G03-WEAPONS-064 |
| G03-WEAPONS-065 | Present | 3. Weapons | Weapons lying on the ground. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-066 | Present | 3. Weapons | Weapon pickup indicators. | web:weapons:inventory |
| G04-WEAPON-MOD-001 | Present | 4. Weapon Mod parity | Damage. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-002 | Present | 4. Weapon Mod parity | Fire interval. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-003 | Present | 4. Weapon Mod parity | Ammunition capacity. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-004 | Present | 4. Weapon Mod parity | Reload time. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-005 | Present | 4. Weapon Mod parity | Projectile speed. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-006 | Present | 4. Weapon Mod parity | Bullet style. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-007 | Present | 4. Weapon Mod parity | Startup time. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-008 | Present | 4. Weapon Mod parity | Bink. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-009 | Present | 4. Weapon Mod parity | Self-bink through negative bink values. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-010 | Present | 4. Weapon Mod parity | Movement accuracy. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-011 | Present | 4. Weapon Mod parity | Bullet spread. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-012 | Present | 4. Weapon Mod parity | Recoil. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-013 | Present | 4. Weapon Mod parity | Push. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-014 | Present | 4. Weapon Mod parity | Inherited velocity. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-015 | Present | 4. Weapon Mod parity | Head damage modifier. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-016 | Present | 4. Weapon Mod parity | Chest damage modifier. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-017 | Present | 4. Weapon Mod parity | Leg damage modifier. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-018 | Present | 4. Weapon Mod parity | Separate normal and Realistic weapon tables. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-019 | Present | 4. Weapon Mod parity | Cluster-grenade nesting under frag grenades. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-020 | Present | 4. Weapon Mod parity | Server-selected/custom weapon mods. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-021 | Present | 4. Weapon Mod parity | Weapon-mod synchronization with clients. | web:weapons:table |
| G04-WEAPON-MOD-022 | Present | 4. Weapon Mod parity | Validation of custom weapon values. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-023 | Present | 4. Weapon Mod parity | A safe loader for `weapons.ini`-style data. | rust:weapon_config_fixtures |
| G04-WEAPON-MOD-024 | Present | 4. Weapon Mod parity | Version/hash checking so clients know which mod is active. | web:weapons:table |
| G04-WEAPON-MOD-025 | Present | 4. Weapon Mod parity | Weapon-mod display in the room/server browser. | web:weapons:table |
| G04-WEAPON-MOD-026 | Present | 4. Weapon Mod parity | Replace simplified damage with `Damage × CurrentSpeed × HitboxModifier`. | rust:weapon_ballistics_fixtures:damage |
| G04-WEAPON-MOD-027 | Present | 4. Weapon Mod parity | Add projectile speed decay. | rust:weapon_ballistics_fixtures:decay |
| G04-WEAPON-MOD-028 | Present | 4. Weapon Mod parity | Add player velocity inheritance. | rust:weapon_ballistics_fixtures:inherit |
| G04-WEAPON-MOD-029 | Present | 4. Weapon Mod parity | Allow movement direction to increase or decrease projectile speed and damage. | rust:weapon_ballistics_fixtures:inherit |
| G04-WEAPON-MOD-030 | Present | 4. Weapon Mod parity | Add head, chest, and leg hit detection. | rust:weapon_ballistics_fixtures:damage |
| G04-WEAPON-MOD-031 | Present | 4. Weapon Mod parity | Add bink when the player is hit. | rust:weapon_ballistics_fixtures:accuracy |
| G04-WEAPON-MOD-032 | Present | 4. Weapon Mod parity | Add self-bink when firing relevant weapons. | rust:weapon_ballistics_fixtures:accuracy |
| G04-WEAPON-MOD-033 | Present | 4. Weapon Mod parity | Add cursor expansion representing current accuracy. | web:weapons:cursor |
| G04-WEAPON-MOD-034 | Present | 4. Weapon Mod parity | Add movement accuracy penalties. | rust:weapon_ballistics_fixtures:accuracy |
| G04-WEAPON-MOD-035 | Present | 4. Weapon Mod parity | Add stronger jetting accuracy penalties. | rust:weapon_ballistics_fixtures:accuracy |
| G04-WEAPON-MOD-036 | Present | 4. Weapon Mod parity | Add recoil to the aim/cursor. | web:weapons:cursor |
| G04-WEAPON-MOD-037 | Present | 4. Weapon Mod parity | Add correct bullet spread. | rust:weapon_ballistics_fixtures:accuracy |
| G04-WEAPON-MOD-038 | Present | 4. Weapon Mod parity | Add bullet mass/push behavior. | rust:weapon_ballistics_fixtures:accuracy |
| G04-WEAPON-MOD-039 | Present | 4. Weapon Mod parity | Add distance-dependent damage caused by changing projectile speed. | rust:weapon_ballistics_fixtures:decay |
| G04-WEAPON-MOD-040 | Present | 4. Weapon Mod parity | Add exact explosive damage and falloff. | rust:weapon_ballistics_fixtures:explosion |
| G04-WEAPON-MOD-041 | Present | 4. Weapon Mod parity | Add terrain occlusion for splash damage if Soldat’s source behavior requires it. | rust:weapon_ballistics_fixtures:explosion |
| G04-WEAPON-MOD-042 | Present | 4. Weapon Mod parity | Add friendly-fire and team-bink rules. | rust:weapon_ballistics_fixtures:friendly |
| G05-GAME-MODES-001 | Partial | 5. Game modes | Deathmatch. | parity:G05-GAME-MODES-001 |
| G05-GAME-MODES-002 | Partial | 5. Game modes | Team Deathmatch, corresponding to Soldat’s Teammatch. | parity:G05-GAME-MODES-002 |
| G05-GAME-MODES-003 | Missing | 5. Game modes | Pointmatch. | parity:G05-GAME-MODES-003 |
| G05-GAME-MODES-004 | Missing | 5. Game modes | Rambomatch. | parity:G05-GAME-MODES-004 |
| G05-GAME-MODES-005 | Missing | 5. Game modes | Capture the Flag. | parity:G05-GAME-MODES-005 |
| G05-GAME-MODES-006 | Missing | 5. Game modes | Infiltration. | parity:G05-GAME-MODES-006 |
| G05-GAME-MODES-007 | Missing | 5. Game modes | Hold the Flag. | parity:G05-GAME-MODES-007 |
| G05-GAME-MODES-008 | Missing | 5. Game modes | Realistic mode. | parity:G05-GAME-MODES-008 |
| G05-GAME-MODES-009 | Missing | 5. Game modes | Survival mode. | parity:G05-GAME-MODES-009 |
| G05-GAME-MODES-010 | Missing | 5. Game modes | Advance mode. | parity:G05-GAME-MODES-010 |
| G05-GAME-MODES-011 | Missing | 5. Game modes | Climb. | parity:G05-GAME-MODES-011 |
| G05-GAME-MODES-012 | Missing | 5. Game modes | Dodgeball. | parity:G05-GAME-MODES-012 |
| G05-GAME-MODES-013 | Missing | 5. Game modes | Domination. | parity:G05-GAME-MODES-013 |
| G05-GAME-MODES-014 | Missing | 5. Game modes | Hide and Seek. | parity:G05-GAME-MODES-014 |
| G05-GAME-MODES-015 | Missing | 5. Game modes | Knife Only. | parity:G05-GAME-MODES-015 |
| G05-GAME-MODES-016 | Missing | 5. Game modes | OneShots. | parity:G05-GAME-MODES-016 |
| G05-GAME-MODES-017 | Missing | 5. Game modes | Pirates vs Ninjas. | parity:G05-GAME-MODES-017 |
| G05-GAME-MODES-018 | Missing | 5. Game modes | Realistic Soldat/Counter-Strike. | parity:G05-GAME-MODES-018 |
| G05-GAME-MODES-019 | Missing | 5. Game modes | Trench Wars. | parity:G05-GAME-MODES-019 |
| G05-GAME-MODES-020 | Missing | 5. Game modes | Tactical Trench Wars. | parity:G05-GAME-MODES-020 |
| G05-GAME-MODES-021 | Missing | 5. Game modes | Zombie. | parity:G05-GAME-MODES-021 |
| G05-GAME-MODES-022 | Missing | 5. Game modes | Kill limit. | parity:G05-GAME-MODES-022 |
| G05-GAME-MODES-023 | Missing | 5. Game modes | Point limit. | parity:G05-GAME-MODES-023 |
| G05-GAME-MODES-024 | Missing | 5. Game modes | Capture limit. | parity:G05-GAME-MODES-024 |
| G05-GAME-MODES-025 | Missing | 5. Game modes | Time limit. | parity:G05-GAME-MODES-025 |
| G05-GAME-MODES-026 | Missing | 5. Game modes | Round start countdown. | parity:G05-GAME-MODES-026 |
| G05-GAME-MODES-027 | Missing | 5. Game modes | Round end. | parity:G05-GAME-MODES-027 |
| G05-GAME-MODES-028 | Missing | 5. Game modes | Winner calculation. | parity:G05-GAME-MODES-028 |
| G05-GAME-MODES-029 | Missing | 5. Game modes | Draw handling. | parity:G05-GAME-MODES-029 |
| G05-GAME-MODES-030 | Missing | 5. Game modes | Overtime policy where appropriate. | parity:G05-GAME-MODES-030 |
| G05-GAME-MODES-031 | Missing | 5. Game modes | Map rotation. | parity:G05-GAME-MODES-031 |
| G05-GAME-MODES-032 | Missing | 5. Game modes | Map-loop option. | parity:G05-GAME-MODES-032 |
| G05-GAME-MODES-033 | Missing | 5. Game modes | Next-map transition. | parity:G05-GAME-MODES-033 |
| G05-GAME-MODES-034 | Missing | 5. Game modes | Match restart. | parity:G05-GAME-MODES-034 |
| G05-GAME-MODES-035 | Missing | 5. Game modes | Configurable respawn time. | rust:damage_fixtures:respawn |
| G05-GAME-MODES-036 | Missing | 5. Game modes | Survival round elimination. | parity:G05-GAME-MODES-036 |
| G05-GAME-MODES-037 | Missing | 5. Game modes | Survival dead-player restrictions. | parity:G05-GAME-MODES-037 |
| G05-GAME-MODES-038 | Missing | 5. Game modes | Advance-mode weapon unlocking. | parity:G05-GAME-MODES-038 |
| G05-GAME-MODES-039 | Missing | 5. Game modes | Team balancing. | parity:G05-GAME-MODES-039 |
| G05-GAME-MODES-040 | Missing | 5. Game modes | Team selection. | parity:G05-GAME-MODES-040 |
| G05-GAME-MODES-041 | Missing | 5. Game modes | Spectator team. | parity:G05-GAME-MODES-041 |
| G05-GAME-MODES-042 | Missing | 5. Game modes | Mid-match spectator switching. | parity:G05-GAME-MODES-042 |
| G05-GAME-MODES-043 | Missing | 5. Game modes | Friendly-fire configuration. | parity:G05-GAME-MODES-043 |
| G05-GAME-MODES-044 | Missing | 5. Game modes | Teamkill handling. | parity:G05-GAME-MODES-044 |
| G05-GAME-MODES-045 | Missing | 5. Game modes | Suicide scoring. | parity:G05-GAME-MODES-045 |
| G05-GAME-MODES-046 | Missing | 5. Game modes | Disconnect/reconnect score preservation. | parity:G05-GAME-MODES-046 |
| G05-GAME-MODES-047 | Missing | 5. Game modes | Automatic round-end scoreboard. | parity:G05-GAME-MODES-047 |
| G05-GAME-MODES-048 | Missing | 5. Game modes | Automatic final-score screenshot option. | parity:G05-GAME-MODES-048 |
| G06-POINTMATCH-001 | Missing | 6. Pointmatch | Yellow point flag. | parity:G06-POINTMATCH-001 |
| G06-POINTMATCH-002 | Missing | 6. Pointmatch | Holding the point flag. | parity:G06-POINTMATCH-002 |
| G06-POINTMATCH-003 | Missing | 6. Pointmatch | Extra points awarded while holding it. | parity:G06-POINTMATCH-003 |
| G06-POINTMATCH-004 | Missing | 6. Pointmatch | Flag drop on death. | parity:G06-POINTMATCH-004 |
| G06-POINTMATCH-005 | Missing | 6. Pointmatch | Point-limit victory. | parity:G06-POINTMATCH-005 |
| G06-POINTMATCH-006 | Missing | 6. Pointmatch | Pointmatch-specific scoring. | parity:G06-POINTMATCH-006 |
| G06-POINTMATCH-007 | Missing | 6. Pointmatch | Pointmatch spawns and maps. | parity:G06-POINTMATCH-007 |
| G06-POINTMATCH-008 | Missing | 6. Pointmatch | Point-flag HUD status. | parity:G06-POINTMATCH-008 |
| G07-RAMBOMATCH-001 | Missing | 7. Rambomatch | Rambo Bow spawn. | parity:G07-RAMBOMATCH-001 |
| G07-RAMBOMATCH-002 | Missing | 7. Rambomatch | Bow pickup. | parity:G07-RAMBOMATCH-002 |
| G07-RAMBOMATCH-003 | Missing | 7. Rambomatch | Only the Rambo player earning kills/points under the mode’s rules. | parity:G07-RAMBOMATCH-003 |
| G07-RAMBOMATCH-004 | Missing | 7. Rambomatch | Rambo target indication. | parity:G07-RAMBOMATCH-004 |
| G07-RAMBOMATCH-005 | Missing | 7. Rambomatch | Bow drop and reacquisition. | parity:G07-RAMBOMATCH-005 |
| G07-RAMBOMATCH-006 | Missing | 7. Rambomatch | Rambo-specific respawn behavior. | parity:G07-RAMBOMATCH-006 |
| G07-RAMBOMATCH-007 | Missing | 7. Rambomatch | Flamed-arrow support. | parity:G07-RAMBOMATCH-007 |
| G07-RAMBOMATCH-008 | Missing | 7. Rambomatch | Rambomatch scoring and win limit. | parity:G07-RAMBOMATCH-008 |
| G07-RAMBOMATCH-009 | Missing | 7. Rambomatch | Rambomatch HUD. | parity:G07-RAMBOMATCH-009 |
| G08-CAPTURE-THE-001 | Missing | 8. Capture the Flag | Alpha and Bravo teams. | parity:G08-CAPTURE-THE-001 |
| G08-CAPTURE-THE-002 | Missing | 8. Capture the Flag | Red and blue flags. | parity:G08-CAPTURE-THE-002 |
| G08-CAPTURE-THE-003 | Missing | 8. Capture the Flag | Flag bases. | parity:G08-CAPTURE-THE-003 |
| G08-CAPTURE-THE-004 | Missing | 8. Capture the Flag | Enemy-flag pickup. | parity:G08-CAPTURE-THE-004 |
| G08-CAPTURE-THE-005 | Missing | 8. Capture the Flag | Flag carrying. | parity:G08-CAPTURE-THE-005 |
| G08-CAPTURE-THE-006 | Missing | 8. Capture the Flag | Flag dropping on death. | parity:G08-CAPTURE-THE-006 |
| G08-CAPTURE-THE-007 | Missing | 8. Capture the Flag | Manual flag throw. | parity:G08-CAPTURE-THE-007 |
| G08-CAPTURE-THE-008 | Missing | 8. Capture the Flag | Flag return by touching a dropped friendly flag. | parity:G08-CAPTURE-THE-008 |
| G08-CAPTURE-THE-009 | Missing | 8. Capture the Flag | Automatic return timeout if applicable. | parity:G08-CAPTURE-THE-009 |
| G08-CAPTURE-THE-010 | Missing | 8. Capture the Flag | Capture only when the player’s own flag is at base. | parity:G08-CAPTURE-THE-010 |
| G08-CAPTURE-THE-011 | Missing | 8. Capture the Flag | Capture scoring. | parity:G08-CAPTURE-THE-011 |
| G08-CAPTURE-THE-012 | Missing | 8. Capture the Flag | Capture limit. | parity:G08-CAPTURE-THE-012 |
| G08-CAPTURE-THE-013 | Missing | 8. Capture the Flag | Flag-carrier indicator. | parity:G08-CAPTURE-THE-013 |
| G08-CAPTURE-THE-014 | Missing | 8. Capture the Flag | Missing-flag indicator. | parity:G08-CAPTURE-THE-014 |
| G08-CAPTURE-THE-015 | Missing | 8. Capture the Flag | Flag status HUD. | parity:G08-CAPTURE-THE-015 |
| G08-CAPTURE-THE-016 | Missing | 8. Capture the Flag | Team score HUD. | parity:G08-CAPTURE-THE-016 |
| G08-CAPTURE-THE-017 | Missing | 8. Capture the Flag | Flag physics. | parity:G08-CAPTURE-THE-017 |
| G08-CAPTURE-THE-018 | Missing | 8. Capture the Flag | Bullets and explosions pushing flags. | parity:G08-CAPTURE-THE-018 |
| G08-CAPTURE-THE-019 | Missing | 8. Capture the Flag | Flag collision with polygons. | parity:G08-CAPTURE-THE-019 |
| G08-CAPTURE-THE-020 | Missing | 8. Capture the Flag | CTF spawn points. | parity:G08-CAPTURE-THE-020 |
| G08-CAPTURE-THE-021 | Missing | 8. Capture the Flag | CTF-compatible map validation. | parity:G08-CAPTURE-THE-021 |
| G08-CAPTURE-THE-022 | Missing | 8. Capture the Flag | CTF bots and flag objectives. | parity:G08-CAPTURE-THE-022 |
| G09-INFILTRATION-001 | Missing | 9. Infiltration | Attacking and defending teams. | parity:G09-INFILTRATION-001 |
| G09-INFILTRATION-002 | Missing | 9. Infiltration | Black/white or objective-specific flags. | parity:G09-INFILTRATION-002 |
| G09-INFILTRATION-003 | Missing | 9. Infiltration | Objective capture rules. | parity:G09-INFILTRATION-003 |
| G09-INFILTRATION-004 | Missing | 9. Infiltration | Passive defender scoring. | parity:G09-INFILTRATION-004 |
| G09-INFILTRATION-005 | Missing | 9. Infiltration | Attacker capture scoring. | parity:G09-INFILTRATION-005 |
| G09-INFILTRATION-006 | Missing | 9. Infiltration | Team-role asymmetry. | parity:G09-INFILTRATION-006 |
| G09-INFILTRATION-007 | Missing | 9. Infiltration | Infiltration-specific spawn points. | parity:G09-INFILTRATION-007 |
| G09-INFILTRATION-008 | Missing | 9. Infiltration | Infiltration timer and score rules. | parity:G09-INFILTRATION-008 |
| G09-INFILTRATION-009 | Missing | 9. Infiltration | Team score HUD. | parity:G09-INFILTRATION-009 |
| G09-INFILTRATION-010 | Missing | 9. Infiltration | Objective state indicators. | parity:G09-INFILTRATION-010 |
| G09-INFILTRATION-011 | Missing | 9. Infiltration | Infiltration bot behavior. | parity:G09-INFILTRATION-011 |
| G10-HOLD-THE-001 | Missing | 10. Hold the Flag | Neutral yellow flag. | parity:G10-HOLD-THE-001 |
| G10-HOLD-THE-002 | Missing | 10. Hold the Flag | Flag pickup and carrying. | parity:G10-HOLD-THE-002 |
| G10-HOLD-THE-003 | Missing | 10. Hold the Flag | Continuous team scoring while held. | parity:G10-HOLD-THE-003 |
| G10-HOLD-THE-004 | Missing | 10. Hold the Flag | Flag drops. | parity:G10-HOLD-THE-004 |
| G10-HOLD-THE-005 | Missing | 10. Hold the Flag | Flag return/reset rules. | parity:G10-HOLD-THE-005 |
| G10-HOLD-THE-006 | Missing | 10. Hold the Flag | Carrier indication. | parity:G10-HOLD-THE-006 |
| G10-HOLD-THE-007 | Missing | 10. Hold the Flag | HTF-specific spawn points. | parity:G10-HOLD-THE-007 |
| G10-HOLD-THE-008 | Missing | 10. Hold the Flag | HTF score display. | parity:G10-HOLD-THE-008 |
| G10-HOLD-THE-009 | Missing | 10. Hold the Flag | HTF bots and objective behavior. | parity:G10-HOLD-THE-009 |
| G11-REALISTIC-MODE-001 | Missing | 11. Realistic mode | Separate `weapons_realistic.ini` statistics. | parity:G11-REALISTIC-MODE-001 |
| G11-REALISTIC-MODE-002 | Missing | 11. Realistic mode | Reduced/changed weapon damage behavior. | parity:G11-REALISTIC-MODE-002 |
| G11-REALISTIC-MODE-003 | Missing | 11. Realistic mode | Recoil behavior appropriate to Realistic. | parity:G11-REALISTIC-MODE-003 |
| G11-REALISTIC-MODE-004 | Missing | 11. Realistic mode | Visibility/line-of-sight restrictions. | parity:G11-REALISTIC-MODE-004 |
| G11-REALISTIC-MODE-005 | Missing | 11. Realistic mode | Enemies visible only when the observed player can see them. | parity:G11-REALISTIC-MODE-005 |
| G11-REALISTIC-MODE-006 | Missing | 11. Realistic mode | Dead-player and spectator visibility restrictions. | parity:G11-REALISTIC-MODE-006 |
| G11-REALISTIC-MODE-007 | Missing | 11. Realistic mode | Enemy team-chat visibility restrictions. | parity:G11-REALISTIC-MODE-007 |
| G11-REALISTIC-MODE-008 | Missing | 11. Realistic mode | Fall damage if required by the reference implementation. | parity:G11-REALISTIC-MODE-008 |
| G11-REALISTIC-MODE-009 | Missing | 11. Realistic mode | Realistic movement and survival tuning. | parity:G11-REALISTIC-MODE-009 |
| G11-REALISTIC-MODE-010 | Missing | 11. Realistic mode | Realistic-specific HUD behavior. | parity:G11-REALISTIC-MODE-010 |
| G11-REALISTIC-MODE-011 | Missing | 11. Realistic mode | Server/room Realistic flag. | parity:G11-REALISTIC-MODE-011 |
| G12-SURVIVAL-MODE-001 | Missing | 12. Survival mode | No immediate respawn. | parity:G12-SURVIVAL-MODE-001 |
| G12-SURVIVAL-MODE-002 | Missing | 12. Survival mode | Round-based respawning. | parity:G12-SURVIVAL-MODE-002 |
| G12-SURVIVAL-MODE-003 | Missing | 12. Survival mode | Round begins when enough players are ready. | parity:G12-SURVIVAL-MODE-003 |
| G12-SURVIVAL-MODE-004 | Missing | 12. Survival mode | Round ends when one player/team remains. | parity:G12-SURVIVAL-MODE-004 |
| G12-SURVIVAL-MODE-005 | Missing | 12. Survival mode | Dead players spectate. | parity:G12-SURVIVAL-MODE-005 |
| G12-SURVIVAL-MODE-006 | Missing | 12. Survival mode | Survival scoreboard. | parity:G12-SURVIVAL-MODE-006 |
| G12-SURVIVAL-MODE-007 | Missing | 12. Survival mode | End-of-round state. | parity:G12-SURVIVAL-MODE-007 |
| G12-SURVIVAL-MODE-008 | Missing | 12. Survival mode | Flag restrictions after a Survival round ends. | parity:G12-SURVIVAL-MODE-008 |
| G12-SURVIVAL-MODE-009 | Missing | 12. Survival mode | Survival chat/spectator restrictions. | parity:G12-SURVIVAL-MODE-009 |
| G12-SURVIVAL-MODE-010 | Missing | 12. Survival mode | Configurable survival respawn/round behavior. | parity:G12-SURVIVAL-MODE-010 |
| G13-ADVANCE-MODE-001 | Missing | 13. Advance mode | Initial limited weapon selection. | parity:G13-ADVANCE-MODE-001 |
| G13-ADVANCE-MODE-002 | Missing | 13. Advance mode | Unlock weapons through kills. | parity:G13-ADVANCE-MODE-002 |
| G13-ADVANCE-MODE-003 | Missing | 13. Advance mode | Unlock progression. | parity:G13-ADVANCE-MODE-003 |
| G13-ADVANCE-MODE-004 | Missing | 13. Advance mode | Per-player unlock state. | parity:G13-ADVANCE-MODE-004 |
| G13-ADVANCE-MODE-005 | Missing | 13. Advance mode | Advance weapon menu. | parity:G13-ADVANCE-MODE-005 |
| G13-ADVANCE-MODE-006 | Missing | 13. Advance mode | Reset progression between matches/maps as appropriate. | parity:G13-ADVANCE-MODE-006 |
| G13-ADVANCE-MODE-007 | Missing | 13. Advance mode | Advance configuration. | parity:G13-ADVANCE-MODE-007 |
| G14-BONUS-KITS-001 | Missing | 14. Bonus kits | Medic Kit: restore health to maximum. | parity:G14-BONUS-KITS-001 |
| G14-BONUS-KITS-002 | Missing | 14. Bonus kits | Grenades Kit: restore grenades to configured maximum. | parity:G14-BONUS-KITS-002 |
| G14-BONUS-KITS-003 | Missing | 14. Bonus kits | Cluster Grenades Kit: grant three cluster grenades. | parity:G14-BONUS-KITS-003 |
| G14-BONUS-KITS-004 | Missing | 14. Bonus kits | Bulletproof Vest Kit: add approximately another full health bar as armor. | parity:G14-BONUS-KITS-004 |
| G14-BONUS-KITS-005 | Missing | 14. Bonus kits | Flame God Kit: flamethrower plus temporary invulnerability. | parity:G14-BONUS-KITS-005 |
| G14-BONUS-KITS-006 | Missing | 14. Bonus kits | Berserker Kit: four-times weapon damage temporarily. | parity:G14-BONUS-KITS-006 |
| G14-BONUS-KITS-007 | Missing | 14. Bonus kits | Predator Kit: temporary invisibility. | parity:G14-BONUS-KITS-007 |
| G14-BONUS-KITS-008 | Missing | 14. Bonus kits | Bonus spawn points. | parity:G14-BONUS-KITS-008 |
| G14-BONUS-KITS-009 | Missing | 14. Bonus kits | Configurable kit frequency. | parity:G14-BONUS-KITS-009 |
| G14-BONUS-KITS-010 | Missing | 14. Bonus kits | Kit respawn timers. | parity:G14-BONUS-KITS-010 |
| G14-BONUS-KITS-011 | Missing | 14. Bonus kits | Pickup collision. | parity:G14-BONUS-KITS-011 |
| G14-BONUS-KITS-012 | Missing | 14. Bonus kits | Pickup sounds and effects. | parity:G14-BONUS-KITS-012 |
| G14-BONUS-KITS-013 | Missing | 14. Bonus kits | Active-bonus HUD. | parity:G14-BONUS-KITS-013 |
| G14-BONUS-KITS-014 | Missing | 14. Bonus kits | Bonus countdown. | parity:G14-BONUS-KITS-014 |
| G14-BONUS-KITS-015 | Missing | 14. Bonus kits | Bonus overlay/effect. | parity:G14-BONUS-KITS-015 |
| G14-BONUS-KITS-016 | Missing | 14. Bonus kits | Armor HUD. | parity:G14-BONUS-KITS-016 |
| G14-BONUS-KITS-017 | Missing | 14. Bonus kits | Predator visibility affected by blood. | parity:G14-BONUS-KITS-017 |
| G14-BONUS-KITS-018 | Missing | 14. Bonus kits | Predator still producing audible sounds. | parity:G14-BONUS-KITS-018 |
| G14-BONUS-KITS-019 | Missing | 14. Bonus kits | Bonus expiration. | parity:G14-BONUS-KITS-019 |
| G14-BONUS-KITS-020 | Missing | 14. Bonus kits | Bonus replacement/stacking rules. | parity:G14-BONUS-KITS-020 |
| G14-BONUS-KITS-021 | Missing | 14. Bonus kits | Server enable/disable settings. | parity:G14-BONUS-KITS-021 |
| G14-BONUS-KITS-022 | Missing | 14. Bonus kits | Kits affected by projectile push. | parity:G14-BONUS-KITS-022 |
| G15-HEALTH-ARMOR-001 | Present | 15. Health, armor, death, and respawn | Basic 100 HP. | rust:game_core::tests::m79_explosion_damages_nearby_enemy_not_distant_enemy |
| G15-HEALTH-ARMOR-002 | Present | 15. Health, armor, death, and respawn | Death count. | rust:game_core::tests::self_explosion_is_not_credited_as_a_kill |
| G15-HEALTH-ARMOR-003 | Present | 15. Health, armor, death, and respawn | Automatic respawn. | rust:game_core::tests::deterministic_replay |
| G15-HEALTH-ARMOR-004 | Present | 15. Health, armor, death, and respawn | Body-part damage. | rust:damage_fixtures:regions |
| G15-HEALTH-ARMOR-005 | Missing | 15. Health, armor, death, and respawn | Bulletproof vest/armor. | parity:G15-HEALTH-ARMOR-005 |
| G15-HEALTH-ARMOR-006 | Present | 15. Health, armor, death, and respawn | Bleeding. | rust:damage_fixtures:bleeding |
| G15-HEALTH-ARMOR-007 | Present | 15. Health, armor, death, and respawn | Blood particles. | web:render:particles |
| G15-HEALTH-ARMOR-008 | Missing | 15. Health, armor, death, and respawn | Blood remaining on the character. | parity:G15-HEALTH-ARMOR-008 |
| G15-HEALTH-ARMOR-009 | Present | 15. Health, armor, death, and respawn | Gore/gibs. | web:render:particles |
| G15-HEALTH-ARMOR-010 | Present | 15. Health, armor, death, and respawn | Ragdoll corpses. | rust:damage_fixtures:ragdoll |
| G15-HEALTH-ARMOR-011 | Present | 15. Health, armor, death, and respawn | Corpse persistence. | rust:damage_fixtures:ragdoll |
| G15-HEALTH-ARMOR-012 | Present | 15. Health, armor, death, and respawn | Death causes. | rust:damage_fixtures:death-causes |
| G15-HEALTH-ARMOR-013 | Present | 15. Health, armor, death, and respawn | Kill feed. | web:hud:kill-feed |
| G15-HEALTH-ARMOR-014 | Present | 15. Health, armor, death, and respawn | Headshot messaging/effects. | web:hud:kill-feed |
| G15-HEALTH-ARMOR-015 | Present | 15. Health, armor, death, and respawn | Multi-kill messages. | web:hud:kill-feed |
| G15-HEALTH-ARMOR-016 | Present | 15. Health, armor, death, and respawn | Self-kill messages. | web:hud:kill-feed |
| G15-HEALTH-ARMOR-017 | Present | 15. Health, armor, death, and respawn | Teamkill messages. | web:hud:kill-feed |
| G15-HEALTH-ARMOR-018 | Present | 15. Health, armor, death, and respawn | Spawn protection if applicable. | rust:damage_fixtures:spawn-protection |
| G15-HEALTH-ARMOR-019 | Present | 15. Health, armor, death, and respawn | Configurable respawn time. | rust:damage_fixtures:respawn |
| G15-HEALTH-ARMOR-020 | Present | 15. Health, armor, death, and respawn | Respawn countdown HUD. | web:hud:respawn |
| G15-HEALTH-ARMOR-021 | Present | 15. Health, armor, death, and respawn | Weapon selection while dead. | rust:damage_fixtures:weapon-choice |
| G15-HEALTH-ARMOR-022 | Present | 15. Health, armor, death, and respawn | Proper mode/team/map spawn selection. | rust:damage_fixtures:spawn-selection |
| G15-HEALTH-ARMOR-023 | Present | 15. Health, armor, death, and respawn | Kill attribution after delayed damage. | rust:damage_fixtures:attribution |
| G15-HEALTH-ARMOR-024 | Present | 15. Health, armor, death, and respawn | Assist tracking if desired. | rust:damage_fixtures:attribution |
| G15-HEALTH-ARMOR-025 | Present | 15. Health, armor, death, and respawn | Damage direction feedback. | web:hud:damage-arrow |
| G15-HEALTH-ARMOR-026 | Missing | 15. Health, armor, death, and respawn | Explosion deafness/whistling effect. | parity:G15-HEALTH-ARMOR-026 |
| G16-MAPS-AND-001 | Present | 16. Maps and terrain engine | `.pms` map loader. | rust:content::pms |
| G16-MAPS-AND-002 | Present | 16. Maps and terrain engine | PMS format validation. | rust:content::pms-validation |
| G16-MAPS-AND-003 | Present | 16. Maps and terrain engine | Polygon geometry. | web:map-render:coordinates |
| G16-MAPS-AND-004 | Present | 16. Maps and terrain engine | Polygon types and properties. | rust:map_validation:materials |
| G16-MAPS-AND-005 | Present | 16. Maps and terrain engine | Texture coordinates. | task5:map-pipeline |
| G16-MAPS-AND-006 | Present | 16. Maps and terrain engine | Map textures. | task5:map-pipeline |
| G16-MAPS-AND-007 | Missing | 16. Maps and terrain engine | Edge textures. | parity:G16-MAPS-AND-007 |
| G16-MAPS-AND-008 | Present | 16. Maps and terrain engine | Scenery objects. | task5:map-pipeline |
| G16-MAPS-AND-009 | Present | 16. Maps and terrain engine | Scenery depth/layers. | task5:map-pipeline |
| G16-MAPS-AND-010 | Missing | 16. Maps and terrain engine | Animated scenery if supported. | parity:G16-MAPS-AND-010 |
| G16-MAPS-AND-011 | Present | 16. Maps and terrain engine | Colliders. | task5:map-pipeline |
| G16-MAPS-AND-012 | Present | 16. Maps and terrain engine | Spawn points. | task5:map-pipeline |
| G16-MAPS-AND-013 | Present | 16. Maps and terrain engine | Player spawn types. | task5:map-pipeline |
| G16-MAPS-AND-014 | Present | 16. Maps and terrain engine | Team spawn types. | task5:map-pipeline |
| G16-MAPS-AND-015 | Missing | 16. Maps and terrain engine | Flag spawn types. | parity:G16-MAPS-AND-015 |
| G16-MAPS-AND-016 | Missing | 16. Maps and terrain engine | Bonus-kit spawn types. | parity:G16-MAPS-AND-016 |
| G16-MAPS-AND-017 | Missing | 16. Maps and terrain engine | Grenade spawn types where relevant. | parity:G16-MAPS-AND-017 |
| G16-MAPS-AND-018 | Missing | 16. Maps and terrain engine | Stationary-gun locations. | parity:G16-MAPS-AND-018 |
| G16-MAPS-AND-019 | Present | 16. Maps and terrain engine | Bot waypoints. | task5:map-pipeline |
| G16-MAPS-AND-020 | Present | 16. Maps and terrain engine | Background colors and gradients. | task5:map-pipeline |
| G16-MAPS-AND-021 | Present | 16. Maps and terrain engine | Weather settings. | task5:map-pipeline |
| G16-MAPS-AND-022 | Present | 16. Maps and terrain engine | Footstep-sound property. | task5:map-pipeline |
| G16-MAPS-AND-023 | Present | 16. Maps and terrain engine | Map-specific jet fuel. | task5:map-pipeline |
| G16-MAPS-AND-024 | Missing | 16. Maps and terrain engine | Map boundaries. | parity:G16-MAPS-AND-024 |
| G16-MAPS-AND-025 | Present | 16. Maps and terrain engine | Death/out-of-bounds areas. | task5:map-pipeline |
| G16-MAPS-AND-026 | Present | 16. Maps and terrain engine | Map metadata. | task5:map-pipeline |
| G16-MAPS-AND-027 | Present | 16. Maps and terrain engine | Map version compatibility. | task5:map-pipeline |
| G16-MAPS-AND-028 | Present | 16. Maps and terrain engine | Custom-map downloading. | task5:map-pipeline |
| G16-MAPS-AND-029 | Present | 16. Maps and terrain engine | Texture/scenery downloading. | task5:map-pipeline |
| G16-MAPS-AND-030 | Present | 16. Maps and terrain engine | Download progress and cancellation. | task5:map-pipeline |
| G16-MAPS-AND-031 | Present | 16. Maps and terrain engine | Missing-asset handling. | task5:map-pipeline |
| G16-MAPS-AND-032 | Present | 16. Maps and terrain engine | Map checksum verification. | task5:map-pipeline |
| G16-MAPS-AND-033 | Present | 16. Maps and terrain engine | Map rotation file/configuration. | task5:map-pipeline |
| G16-MAPS-AND-034 | Present | 16. Maps and terrain engine | Mode-prefix recognition: `ctf_`, `inf_`, `htf_`, and community prefixes. | task5:map-pipeline |
| G16-MAPS-AND-035 | Present | 16. Maps and terrain engine | Empty/invalid map-list handling. | task5:map-pipeline |
| G16-MAPS-AND-036 | Missing | 16. Maps and terrain engine | Map voting or polling if the intended Soldat server experience includes it. | parity:G16-MAPS-AND-036 |
| G16-MAPS-AND-037 | Present | 16. Maps and terrain engine | Client caching of maps/assets. | task5:map-pipeline |
| G16-MAPS-AND-038 | Present | 16. Maps and terrain engine | Map preview images. | task5:map-pipeline |
| G16-MAPS-AND-039 | Present | 16. Maps and terrain engine | Map selection UI. | task5:map-pipeline |
| G16-MAPS-AND-040 | Present | 16. Maps and terrain engine | Offline map testing. | task5:map-pipeline |
| G17-ALL-97-001 | Present | 17. All 97 referenced default maps | Aero | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-002 | Present | 17. All 97 referenced default maps | Airpirates | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-003 | Present | 17. All 97 referenced default maps | Arena | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-004 | Present | 17. All 97 referenced default maps | Arena2 | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-005 | Present | 17. All 97 referenced default maps | Arena3 | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-006 | Present | 17. All 97 referenced default maps | Bigfalls | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-007 | Present | 17. All 97 referenced default maps | Blox | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-008 | Present | 17. All 97 referenced default maps | Bridge | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-009 | Present | 17. All 97 referenced default maps | Bunker | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-010 | Present | 17. All 97 referenced default maps | Cambodia | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-011 | Present | 17. All 97 referenced default maps | CrackedBoot | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-012 | Present | 17. All 97 referenced default maps | Daybreak | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-013 | Present | 17. All 97 referenced default maps | DesertWind | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-014 | Present | 17. All 97 referenced default maps | Factory | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-015 | Present | 17. All 97 referenced default maps | Flashback | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-016 | Present | 17. All 97 referenced default maps | HH | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-017 | Present | 17. All 97 referenced default maps | Island2k5 | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-018 | Present | 17. All 97 referenced default maps | Jungle | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-019 | Present | 17. All 97 referenced default maps | Krab | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-020 | Present | 17. All 97 referenced default maps | Lagrange | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-021 | Present | 17. All 97 referenced default maps | Leaf | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-022 | Present | 17. All 97 referenced default maps | MrSnowman | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-023 | Present | 17. All 97 referenced default maps | RatCave | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-024 | Present | 17. All 97 referenced default maps | Rok | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-025 | Present | 17. All 97 referenced default maps | RR | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-026 | Present | 17. All 97 referenced default maps | Shau | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-027 | Present | 17. All 97 referenced default maps | Tropiccave | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-028 | Present | 17. All 97 referenced default maps | Unlim | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-029 | Present | 17. All 97 referenced default maps | Veoto | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-030 | Present | 17. All 97 referenced default maps | ctf_Ash | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-031 | Present | 17. All 97 referenced default maps | ctf_B2b | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-032 | Present | 17. All 97 referenced default maps | ctf_Blade | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-033 | Present | 17. All 97 referenced default maps | ctf_Campeche | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-034 | Present | 17. All 97 referenced default maps | ctf_Cobra | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-035 | Present | 17. All 97 referenced default maps | ctf_Crucifix | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-036 | Present | 17. All 97 referenced default maps | ctf_Death | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-037 | Present | 17. All 97 referenced default maps | ctf_Division | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-038 | Present | 17. All 97 referenced default maps | ctf_Dropdown | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-039 | Present | 17. All 97 referenced default maps | ctf_Equinox | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-040 | Present | 17. All 97 referenced default maps | ctf_Guardian | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-041 | Present | 17. All 97 referenced default maps | ctf_Hormone | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-042 | Present | 17. All 97 referenced default maps | ctf_IceBeam | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-043 | Present | 17. All 97 referenced default maps | ctf_Kampf | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-044 | Present | 17. All 97 referenced default maps | ctf_Lanubya | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-045 | Present | 17. All 97 referenced default maps | ctf_Laos | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-046 | Present | 17. All 97 referenced default maps | ctf_Maya | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-047 | Present | 17. All 97 referenced default maps | ctf_Mayapan | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-048 | Present | 17. All 97 referenced default maps | ctf_MFM | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-049 | Present | 17. All 97 referenced default maps | ctf_Nuubia | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-050 | Present | 17. All 97 referenced default maps | ctf_Raspberry | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-051 | Present | 17. All 97 referenced default maps | ctf_Rotten | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-052 | Present | 17. All 97 referenced default maps | ctf_Ruins | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-053 | Present | 17. All 97 referenced default maps | ctf_Run | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-054 | Present | 17. All 97 referenced default maps | ctf_Scorpion | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-055 | Present | 17. All 97 referenced default maps | ctf_Snakebite | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-056 | Present | 17. All 97 referenced default maps | ctf_Steel | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-057 | Present | 17. All 97 referenced default maps | ctf_Triumph | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-058 | Present | 17. All 97 referenced default maps | ctf_Viet | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-059 | Present | 17. All 97 referenced default maps | ctf_Voland | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-060 | Present | 17. All 97 referenced default maps | ctf_Wretch | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-061 | Present | 17. All 97 referenced default maps | ctf_X | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-062 | Present | 17. All 97 referenced default maps | htf_Arch | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-063 | Present | 17. All 97 referenced default maps | htf_Baire | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-064 | Present | 17. All 97 referenced default maps | htf_Boxed | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-065 | Present | 17. All 97 referenced default maps | htf_Desert | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-066 | Present | 17. All 97 referenced default maps | htf_Dorothy | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-067 | Present | 17. All 97 referenced default maps | htf_Dusk | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-068 | Present | 17. All 97 referenced default maps | htf_Erbium | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-069 | Present | 17. All 97 referenced default maps | htf_Feast | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-070 | Present | 17. All 97 referenced default maps | htf_Mossy | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-071 | Present | 17. All 97 referenced default maps | htf_Muygen | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-072 | Present | 17. All 97 referenced default maps | htf_Niall | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-073 | Present | 17. All 97 referenced default maps | htf_Nuclear | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-074 | Present | 17. All 97 referenced default maps | htf_Prison | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-075 | Present | 17. All 97 referenced default maps | htf_Rubik | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-076 | Present | 17. All 97 referenced default maps | htf_Star | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-077 | Present | 17. All 97 referenced default maps | htf_Tower | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-078 | Present | 17. All 97 referenced default maps | htf_Void | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-079 | Present | 17. All 97 referenced default maps | htf_Vortex | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-080 | Present | 17. All 97 referenced default maps | htf_Zajacz | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-081 | Present | 17. All 97 referenced default maps | inf_Abel | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-082 | Present | 17. All 97 referenced default maps | inf_April | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-083 | Present | 17. All 97 referenced default maps | inf_Argy | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-084 | Present | 17. All 97 referenced default maps | inf_Belltower | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-085 | Present | 17. All 97 referenced default maps | inf_Biologic | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-086 | Present | 17. All 97 referenced default maps | inf_Changeling | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-087 | Present | 17. All 97 referenced default maps | inf_Flute | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-088 | Present | 17. All 97 referenced default maps | inf_Fortress | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-089 | Present | 17. All 97 referenced default maps | inf_Industrial | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-090 | Present | 17. All 97 referenced default maps | inf_Messner | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-091 | Present | 17. All 97 referenced default maps | inf_Moonshine | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-092 | Present | 17. All 97 referenced default maps | inf_Motheaten | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-093 | Present | 17. All 97 referenced default maps | inf_Outpost | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-094 | Present | 17. All 97 referenced default maps | inf_Rescue | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-095 | Present | 17. All 97 referenced default maps | inf_Rise | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-096 | Present | 17. All 97 referenced default maps | inf_Warehouse | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G17-ALL-97-097 | Present | 17. All 97 referenced default maps | inf_Warlock | rust:map_editor:all_97_original_maps_pass_the_admission_pipeline |
| G18-BOTS-AND-001 | Missing | 18. Bots and AI | Bot entities driven by the authoritative simulation. | parity:G18-BOTS-AND-001 |
| G18-BOTS-AND-002 | Missing | 18. Bots and AI | Random-bot count. | parity:G18-BOTS-AND-002 |
| G18-BOTS-AND-003 | Missing | 18. Bots and AI | Per-team bot count. | parity:G18-BOTS-AND-003 |
| G18-BOTS-AND-004 | Missing | 18. Bots and AI | Bot difficulty. | parity:G18-BOTS-AND-004 |
| G18-BOTS-AND-005 | Missing | 18. Bots and AI | Bot accuracy levels. | parity:G18-BOTS-AND-005 |
| G18-BOTS-AND-006 | Missing | 18. Bots and AI | Bot reaction time. | parity:G18-BOTS-AND-006 |
| G18-BOTS-AND-007 | Missing | 18. Bots and AI | Bot movement. | parity:G18-BOTS-AND-007 |
| G18-BOTS-AND-008 | Missing | 18. Bots and AI | Jet navigation. | parity:G18-BOTS-AND-008 |
| G18-BOTS-AND-009 | Missing | 18. Bots and AI | Crouching, prone, rolls, and backflips. | parity:G18-BOTS-AND-009 |
| G18-BOTS-AND-010 | Missing | 18. Bots and AI | Weapon selection. | parity:G18-BOTS-AND-010 |
| G18-BOTS-AND-011 | Missing | 18. Bots and AI | Reloading. | parity:G18-BOTS-AND-011 |
| G18-BOTS-AND-012 | Missing | 18. Bots and AI | Grenade use. | parity:G18-BOTS-AND-012 |
| G18-BOTS-AND-013 | Missing | 18. Bots and AI | Weapon pickup. | parity:G18-BOTS-AND-013 |
| G18-BOTS-AND-014 | Missing | 18. Bots and AI | Bonus pickup. | parity:G18-BOTS-AND-014 |
| G18-BOTS-AND-015 | Missing | 18. Bots and AI | Waypoint navigation. | parity:G18-BOTS-AND-015 |
| G18-BOTS-AND-016 | Missing | 18. Bots and AI | Recovery when stuck. | parity:G18-BOTS-AND-016 |
| G18-BOTS-AND-017 | Missing | 18. Bots and AI | Deathmatch target selection. | parity:G18-BOTS-AND-017 |
| G18-BOTS-AND-018 | Missing | 18. Bots and AI | Team coordination. | parity:G18-BOTS-AND-018 |
| G18-BOTS-AND-019 | Missing | 18. Bots and AI | CTF attacking, defending, returning, and capturing. | parity:G18-BOTS-AND-019 |
| G18-BOTS-AND-020 | Missing | 18. Bots and AI | Infiltration objectives. | parity:G18-BOTS-AND-020 |
| G18-BOTS-AND-021 | Missing | 18. Bots and AI | HTF carrier support. | parity:G18-BOTS-AND-021 |
| G18-BOTS-AND-022 | Missing | 18. Bots and AI | Rambomatch behavior. | parity:G18-BOTS-AND-022 |
| G18-BOTS-AND-023 | Missing | 18. Bots and AI | Survival behavior. | parity:G18-BOTS-AND-023 |
| G18-BOTS-AND-024 | Missing | 18. Bots and AI | Bot chat. | parity:G18-BOTS-AND-024 |
| G18-BOTS-AND-025 | Missing | 18. Bots and AI | Custom bot profiles. | parity:G18-BOTS-AND-025 |
| G18-BOTS-AND-026 | Missing | 18. Bots and AI | Server commands to add/remove bots. | parity:G18-BOTS-AND-026 |
| G19-HUD-AND-001 | Present | 19. HUD and game screen | Numeric health. | web:smoke:hud-health |
| G19-HUD-AND-002 | Present | 19. HUD and game screen | Numeric jet fuel. | web:smoke:hud-jet |
| G19-HUD-AND-003 | Present | 19. HUD and game screen | Ammo and grenade count. | web:smoke:hud-ammo |
| G19-HUD-AND-004 | Present | 19. HUD and game screen | Kills/deaths. | web:smoke:hud-score |
| G19-HUD-AND-005 | Present | 19. HUD and game screen | Scoreboard. | web:smoke:scoreboard |
| G19-HUD-AND-006 | Partial | 19. HUD and game screen | Team coloring. | parity:G19-HUD-AND-006 |
| G19-HUD-AND-007 | Missing | 19. HUD and game screen | Red health bar. | parity:G19-HUD-AND-007 |
| G19-HUD-AND-008 | Missing | 19. HUD and game screen | Yellow ammunition/reload bar. | parity:G19-HUD-AND-008 |
| G19-HUD-AND-009 | Missing | 19. HUD and game screen | Bullet count positioned with the ammo display. | parity:G19-HUD-AND-009 |
| G19-HUD-AND-010 | Missing | 19. HUD and game screen | Fire-interval bar. | parity:G19-HUD-AND-010 |
| G19-HUD-AND-011 | Missing | 19. HUD and game screen | Blue jet-fuel bar. | parity:G19-HUD-AND-011 |
| G19-HUD-AND-012 | Missing | 19. HUD and game screen | Server rank. | parity:G19-HUD-AND-012 |
| G19-HUD-AND-013 | Missing | 19. HUD and game screen | Current kills/points. | parity:G19-HUD-AND-013 |
| G19-HUD-AND-014 | Missing | 19. HUD and game screen | Difference from the leader. | parity:G19-HUD-AND-014 |
| G19-HUD-AND-015 | Missing | 19. HUD and game screen | Kill/point/capture limit. | parity:G19-HUD-AND-015 |
| G19-HUD-AND-016 | Missing | 19. HUD and game screen | Alpha team score. | parity:G19-HUD-AND-016 |
| G19-HUD-AND-017 | Missing | 19. HUD and game screen | Bravo team score. | parity:G19-HUD-AND-017 |
| G19-HUD-AND-018 | Missing | 19. HUD and game screen | Charlie team score. | parity:G19-HUD-AND-018 |
| G19-HUD-AND-019 | Missing | 19. HUD and game screen | Delta team score. | parity:G19-HUD-AND-019 |
| G19-HUD-AND-020 | Missing | 19. HUD and game screen | Missing-flag indicators. | parity:G19-HUD-AND-020 |
| G19-HUD-AND-021 | Missing | 19. HUD and game screen | Flag-carrier state. | parity:G19-HUD-AND-021 |
| G19-HUD-AND-022 | Missing | 19. HUD and game screen | Bonus status and duration. | parity:G19-HUD-AND-022 |
| G19-HUD-AND-023 | Missing | 19. HUD and game screen | Armor indicator. | parity:G19-HUD-AND-023 |
| G19-HUD-AND-024 | Missing | 19. HUD and game screen | Weapon image. | parity:G19-HUD-AND-024 |
| G19-HUD-AND-025 | Missing | 19. HUD and game screen | Secondary weapon indicator. | parity:G19-HUD-AND-025 |
| G19-HUD-AND-026 | Missing | 19. HUD and game screen | Grenade type/count. | parity:G19-HUD-AND-026 |
| G19-HUD-AND-027 | Missing | 19. HUD and game screen | Reload progress rather than only “RELOADING.” | parity:G19-HUD-AND-027 |
| G19-HUD-AND-028 | Missing | 19. HUD and game screen | Respawn countdown. | parity:G19-HUD-AND-028 |
| G19-HUD-AND-029 | Missing | 19. HUD and game screen | Kill feed. | web:hud:kill-feed |
| G19-HUD-AND-030 | Missing | 19. HUD and game screen | Chat overlay inside the game. | parity:G19-HUD-AND-030 |
| G19-HUD-AND-031 | Missing | 19. HUD and game screen | Team-chat distinction. | parity:G19-HUD-AND-031 |
| G19-HUD-AND-032 | Missing | 19. HUD and game screen | Server messages. | parity:G19-HUD-AND-032 |
| G19-HUD-AND-033 | Missing | 19. HUD and game screen | Connection/ping indicator. | parity:G19-HUD-AND-033 |
| G19-HUD-AND-034 | Missing | 19. HUD and game screen | Ping dot with size/color grading. | parity:G19-HUD-AND-034 |
| G19-HUD-AND-035 | Missing | 19. HUD and game screen | FPS display. | parity:G19-HUD-AND-035 |
| G19-HUD-AND-036 | Missing | 19. HUD and game screen | Network-bandwidth display. | parity:G19-HUD-AND-036 |
| G19-HUD-AND-037 | Missing | 19. HUD and game screen | Minimap. | parity:G19-HUD-AND-037 |
| G19-HUD-AND-038 | Missing | 19. HUD and game screen | Sniper line. | parity:G19-HUD-AND-038 |
| G19-HUD-AND-039 | Missing | 19. HUD and game screen | Crosshair accuracy/bink visualization. | parity:G19-HUD-AND-039 |
| G19-HUD-AND-040 | Missing | 19. HUD and game screen | Spectator HUD. | parity:G19-HUD-AND-040 |
| G19-HUD-AND-041 | Missing | 19. HUD and game screen | End-of-round screen. | parity:G19-HUD-AND-041 |
| G19-HUD-AND-042 | Missing | 19. HUD and game screen | Weapon-statistics screen. | parity:G19-HUD-AND-042 |
| G19-HUD-AND-043 | Missing | 19. HUD and game screen | Scrollable large scoreboard. | parity:G19-HUD-AND-043 |
| G19-HUD-AND-044 | Missing | 19. HUD and game screen | Player IDs on the command-enabled scoreboard. | parity:G19-HUD-AND-044 |
| G19-HUD-AND-045 | Missing | 19. HUD and game screen | Custom HUD/interface loading. | parity:G19-HUD-AND-045 |
| G19-HUD-AND-046 | Missing | 19. HUD and game screen | HUD scaling. | parity:G19-HUD-AND-046 |
| G19-HUD-AND-047 | Missing | 19. HUD and game screen | Safe-area and resolution tests across desktop and mobile. | parity:G19-HUD-AND-047 |
| G20-RENDERING-AND-001 | Missing | 20. Rendering and visual effects | Full animated soldier/gostek rendering. | parity:G20-RENDERING-AND-001 |
| G20-RENDERING-AND-002 | Missing | 20. Rendering and visual effects | Separate body parts. | parity:G20-RENDERING-AND-002 |
| G20-RENDERING-AND-003 | Missing | 20. Rendering and visual effects | Hair. | parity:G20-RENDERING-AND-003 |
| G20-RENDERING-AND-004 | Missing | 20. Rendering and visual effects | Headgear. | parity:G20-RENDERING-AND-004 |
| G20-RENDERING-AND-005 | Missing | 20. Rendering and visual effects | Helmet/hat/none. | parity:G20-RENDERING-AND-005 |
| G20-RENDERING-AND-006 | Missing | 20. Rendering and visual effects | Chains and dog tags. | parity:G20-RENDERING-AND-006 |
| G20-RENDERING-AND-007 | Missing | 20. Rendering and visual effects | Shirt, pants, skin, hair, shoes, and jet colors. | parity:G20-RENDERING-AND-007 |
| G20-RENDERING-AND-008 | Missing | 20. Rendering and visual effects | Weapon sprites. | parity:G20-RENDERING-AND-008 |
| G20-RENDERING-AND-009 | Missing | 20. Rendering and visual effects | Character pose matching aim angle. | parity:G20-RENDERING-AND-009 |
| G20-RENDERING-AND-010 | Missing | 20. Rendering and visual effects | Muzzle flashes. | parity:G20-RENDERING-AND-010 |
| G20-RENDERING-AND-011 | Missing | 20. Rendering and visual effects | Bullet trails. | parity:G20-RENDERING-AND-011 |
| G20-RENDERING-AND-012 | Missing | 20. Rendering and visual effects | Shell casings. | parity:G20-RENDERING-AND-012 |
| G20-RENDERING-AND-013 | Missing | 20. Rendering and visual effects | Sparks. | parity:G20-RENDERING-AND-013 |
| G20-RENDERING-AND-014 | Missing | 20. Rendering and visual effects | Blood. | parity:G20-RENDERING-AND-014 |
| G20-RENDERING-AND-015 | Missing | 20. Rendering and visual effects | Gore. | parity:G20-RENDERING-AND-015 |
| G20-RENDERING-AND-016 | Missing | 20. Rendering and visual effects | Explosion animation. | parity:G20-RENDERING-AND-016 |
| G20-RENDERING-AND-017 | Missing | 20. Rendering and visual effects | Smoke. | parity:G20-RENDERING-AND-017 |
| G20-RENDERING-AND-018 | Missing | 20. Rendering and visual effects | Fire. | parity:G20-RENDERING-AND-018 |
| G20-RENDERING-AND-019 | Missing | 20. Rendering and visual effects | Burning characters. | parity:G20-RENDERING-AND-019 |
| G20-RENDERING-AND-020 | Missing | 20. Rendering and visual effects | Grenade sprites. | parity:G20-RENDERING-AND-020 |
| G20-RENDERING-AND-021 | Missing | 20. Rendering and visual effects | Arrow sprites. | parity:G20-RENDERING-AND-021 |
| G20-RENDERING-AND-022 | Missing | 20. Rendering and visual effects | Dropped weapons. | parity:G20-RENDERING-AND-022 |
| G20-RENDERING-AND-023 | Missing | 20. Rendering and visual effects | Flags. | parity:G20-RENDERING-AND-023 |
| G20-RENDERING-AND-024 | Missing | 20. Rendering and visual effects | Kits. | parity:G20-RENDERING-AND-024 |
| G20-RENDERING-AND-025 | Missing | 20. Rendering and visual effects | Stationary gun. | parity:G20-RENDERING-AND-025 |
| G20-RENDERING-AND-026 | Missing | 20. Rendering and visual effects | Polygon textures. | parity:G20-RENDERING-AND-026 |
| G20-RENDERING-AND-027 | Missing | 20. Rendering and visual effects | Edge textures. | parity:G20-RENDERING-AND-027 |
| G20-RENDERING-AND-028 | Missing | 20. Rendering and visual effects | Background scenery. | parity:G20-RENDERING-AND-028 |
| G20-RENDERING-AND-029 | Missing | 20. Rendering and visual effects | Foreground scenery. | parity:G20-RENDERING-AND-029 |
| G20-RENDERING-AND-030 | Missing | 20. Rendering and visual effects | Rain. | parity:G20-RENDERING-AND-030 |
| G20-RENDERING-AND-031 | Missing | 20. Rendering and visual effects | Snow. | parity:G20-RENDERING-AND-031 |
| G20-RENDERING-AND-032 | Missing | 20. Rendering and visual effects | Wind effects. | parity:G20-RENDERING-AND-032 |
| G20-RENDERING-AND-033 | Missing | 20. Rendering and visual effects | Bullet-time visual effect. | parity:G20-RENDERING-AND-033 |
| G20-RENDERING-AND-034 | Missing | 20. Rendering and visual effects | Predator transparency. | parity:G20-RENDERING-AND-034 |
| G20-RENDERING-AND-035 | Missing | 20. Rendering and visual effects | Berserker overlay. | parity:G20-RENDERING-AND-035 |
| G20-RENDERING-AND-036 | Missing | 20. Rendering and visual effects | Flame God overlay. | parity:G20-RENDERING-AND-036 |
| G20-RENDERING-AND-037 | Missing | 20. Rendering and visual effects | Damage feedback. | parity:G20-RENDERING-AND-037 |
| G20-RENDERING-AND-038 | Missing | 20. Rendering and visual effects | Screen shake where appropriate. | parity:G20-RENDERING-AND-038 |
| G20-RENDERING-AND-039 | Missing | 20. Rendering and visual effects | Resolution scaling. | parity:G20-RENDERING-AND-039 |
| G20-RENDERING-AND-040 | Missing | 20. Rendering and visual effects | Texture filtering settings. | parity:G20-RENDERING-AND-040 |
| G20-RENDERING-AND-041 | Missing | 20. Rendering and visual effects | Mipmapping. | parity:G20-RENDERING-AND-041 |
| G20-RENDERING-AND-042 | Missing | 20. Rendering and visual effects | Low-particle modes. | parity:G20-RENDERING-AND-042 |
| G20-RENDERING-AND-043 | Missing | 20. Rendering and visual effects | Compatibility rendering path. | parity:G20-RENDERING-AND-043 |
| G20-RENDERING-AND-044 | Missing | 20. Rendering and visual effects | Custom-interface graphics. | parity:G20-RENDERING-AND-044 |
| G20-RENDERING-AND-045 | Missing | 20. Rendering and visual effects | Mod-controlled asset scaling through `mod.ini`-like rules. | parity:G20-RENDERING-AND-045 |
| G21-SOUND-AND-001 | Missing | 21. Sound and music | Weapon-specific firing sounds. | parity:G21-SOUND-AND-001 |
| G21-SOUND-AND-002 | Missing | 21. Sound and music | Reload sounds. | parity:G21-SOUND-AND-002 |
| G21-SOUND-AND-003 | Missing | 21. Sound and music | Empty-weapon sounds. | parity:G21-SOUND-AND-003 |
| G21-SOUND-AND-004 | Missing | 21. Sound and music | Grenade pin/throw/bounce/explosion sounds. | parity:G21-SOUND-AND-004 |
| G21-SOUND-AND-005 | Missing | 21. Sound and music | M79 and LAW explosion sounds. | parity:G21-SOUND-AND-005 |
| G21-SOUND-AND-006 | Missing | 21. Sound and music | Bullet impacts. | parity:G21-SOUND-AND-006 |
| G21-SOUND-AND-007 | Missing | 21. Sound and music | Ricochets if applicable. | parity:G21-SOUND-AND-007 |
| G21-SOUND-AND-008 | Missing | 21. Sound and music | Chainsaw loop. | parity:G21-SOUND-AND-008 |
| G21-SOUND-AND-009 | Missing | 21. Sound and music | Knife sounds. | parity:G21-SOUND-AND-009 |
| G21-SOUND-AND-010 | Missing | 21. Sound and music | Punch sounds. | parity:G21-SOUND-AND-010 |
| G21-SOUND-AND-011 | Missing | 21. Sound and music | Flamethrower loop. | parity:G21-SOUND-AND-011 |
| G21-SOUND-AND-012 | Missing | 21. Sound and music | Character pain. | parity:G21-SOUND-AND-012 |
| G21-SOUND-AND-013 | Missing | 21. Sound and music | Death sounds. | parity:G21-SOUND-AND-013 |
| G21-SOUND-AND-014 | Missing | 21. Sound and music | Gore sounds. | parity:G21-SOUND-AND-014 |
| G21-SOUND-AND-015 | Missing | 21. Sound and music | Footsteps selected by map property. | parity:G21-SOUND-AND-015 |
| G21-SOUND-AND-016 | Missing | 21. Sound and music | Jetpack sound. | parity:G21-SOUND-AND-016 |
| G21-SOUND-AND-017 | Missing | 21. Sound and music | Flag pickup/drop/return/capture sounds. | parity:G21-SOUND-AND-017 |
| G21-SOUND-AND-018 | Missing | 21. Sound and music | Kit pickup sounds. | parity:G21-SOUND-AND-018 |
| G21-SOUND-AND-019 | Missing | 21. Sound and music | Bonus activation/expiration sounds. | parity:G21-SOUND-AND-019 |
| G21-SOUND-AND-020 | Missing | 21. Sound and music | UI/menu sounds. | parity:G21-SOUND-AND-020 |
| G21-SOUND-AND-021 | Missing | 21. Sound and music | Chat/message notification. | parity:G21-SOUND-AND-021 |
| G21-SOUND-AND-022 | Missing | 21. Sound and music | Distant-battle sounds. | parity:G21-SOUND-AND-022 |
| G21-SOUND-AND-023 | Missing | 21. Sound and music | Weather sounds. | parity:G21-SOUND-AND-023 |
| G21-SOUND-AND-024 | Missing | 21. Sound and music | Explosion deafness and whistle effect. | parity:G21-SOUND-AND-024 |
| G21-SOUND-AND-025 | Missing | 21. Sound and music | Positional audio. | parity:G21-SOUND-AND-025 |
| G21-SOUND-AND-026 | Missing | 21. Sound and music | Distance attenuation. | parity:G21-SOUND-AND-026 |
| G21-SOUND-AND-027 | Missing | 21. Sound and music | Master sound volume. | parity:G21-SOUND-AND-027 |
| G21-SOUND-AND-028 | Missing | 21. Sound and music | Music volume. | parity:G21-SOUND-AND-028 |
| G21-SOUND-AND-029 | Missing | 21. Sound and music | Music playback. | parity:G21-SOUND-AND-029 |
| G21-SOUND-AND-030 | Missing | 21. Sound and music | Toggle music. | parity:G21-SOUND-AND-030 |
| G21-SOUND-AND-031 | Missing | 21. Sound and music | Previous/next track. | parity:G21-SOUND-AND-031 |
| G21-SOUND-AND-032 | Missing | 21. Sound and music | Sound-quality option. | parity:G21-SOUND-AND-032 |
| G21-SOUND-AND-033 | Missing | 21. Sound and music | Sound-output/device option where browser APIs permit it. | parity:G21-SOUND-AND-033 |
| G22-PLAYER-PROFILES-001 | Missing | 22. Player profiles and customization | Persistent player profiles. | parity:G22-PLAYER-PROFILES-001 |
| G22-PLAYER-PROFILES-002 | Missing | 22. Player profiles and customization | Multiple profiles. | parity:G22-PLAYER-PROFILES-002 |
| G22-PLAYER-PROFILES-003 | Missing | 22. Player profiles and customization | Profile selection screen. | parity:G22-PLAYER-PROFILES-003 |
| G22-PLAYER-PROFILES-004 | Missing | 22. Player profiles and customization | Per-profile settings. | parity:G22-PLAYER-PROFILES-004 |
| G22-PLAYER-PROFILES-005 | Missing | 22. Player profiles and customization | Per-profile controls. | parity:G22-PLAYER-PROFILES-005 |
| G22-PLAYER-PROFILES-006 | Missing | 22. Player profiles and customization | Per-profile taunts. | parity:G22-PLAYER-PROFILES-006 |
| G22-PLAYER-PROFILES-007 | Missing | 22. Player profiles and customization | Default secondary weapon. | parity:G22-PLAYER-PROFILES-007 |
| G22-PLAYER-PROFILES-008 | Missing | 22. Player profiles and customization | Player name constraints matching the intended rules. | parity:G22-PLAYER-PROFILES-008 |
| G22-PLAYER-PROFILES-009 | Missing | 22. Player profiles and customization | Shirt color. | parity:G22-PLAYER-PROFILES-009 |
| G22-PLAYER-PROFILES-010 | Missing | 22. Player profiles and customization | Pants color. | parity:G22-PLAYER-PROFILES-010 |
| G22-PLAYER-PROFILES-011 | Missing | 22. Player profiles and customization | Skin color. | parity:G22-PLAYER-PROFILES-011 |
| G22-PLAYER-PROFILES-012 | Missing | 22. Player profiles and customization | Hair color. | parity:G22-PLAYER-PROFILES-012 |
| G22-PLAYER-PROFILES-013 | Missing | 22. Player profiles and customization | Shoe color. | parity:G22-PLAYER-PROFILES-013 |
| G22-PLAYER-PROFILES-014 | Missing | 22. Player profiles and customization | Jet-flame color. | parity:G22-PLAYER-PROFILES-014 |
| G22-PLAYER-PROFILES-015 | Missing | 22. Player profiles and customization | Hairstyle. | parity:G22-PLAYER-PROFILES-015 |
| G22-PLAYER-PROFILES-016 | Missing | 22. Player profiles and customization | Headgear. | parity:G22-PLAYER-PROFILES-016 |
| G22-PLAYER-PROFILES-017 | Missing | 22. Player profiles and customization | Chain style. | parity:G22-PLAYER-PROFILES-017 |
| G22-PLAYER-PROFILES-018 | Missing | 22. Player profiles and customization | Interface selection. | parity:G22-PLAYER-PROFILES-018 |
| G22-PLAYER-PROFILES-019 | Missing | 22. Player profiles and customization | Saved mouse sensitivity. | parity:G22-PLAYER-PROFILES-019 |
| G22-PLAYER-PROFILES-020 | Missing | 22. Player profiles and customization | Saved sound/music volume. | parity:G22-PLAYER-PROFILES-020 |
| G22-PLAYER-PROFILES-021 | Missing | 22. Player profiles and customization | Saved graphics settings. | parity:G22-PLAYER-PROFILES-021 |
| G22-PLAYER-PROFILES-022 | Missing | 22. Player profiles and customization | Saved favorite servers. | parity:G22-PLAYER-PROFILES-022 |
| G22-PLAYER-PROFILES-023 | Missing | 22. Player profiles and customization | Profile import/export if desired. | parity:G22-PLAYER-PROFILES-023 |
| G22-PLAYER-PROFILES-024 | Missing | 22. Player profiles and customization | Account-backed persistence if the game moves beyond local profiles. | parity:G22-PLAYER-PROFILES-024 |
| G23-CHAT-AND-001 | Present | 23. Chat and taunts | Basic room chat. | server:chat-handler |
| G23-CHAT-AND-002 | Present | 23. Chat and taunts | Basic chat rate limiting. | server:chat-rate-limit |
| G23-CHAT-AND-003 | Missing | 23. Chat and taunts | Team chat. | parity:G23-CHAT-AND-003 |
| G23-CHAT-AND-004 | Missing | 23. Chat and taunts | `^` shorthand for team chat. | parity:G23-CHAT-AND-004 |
| G23-CHAT-AND-005 | Missing | 23. Chat and taunts | In-game chat overlay. | parity:G23-CHAT-AND-005 |
| G23-CHAT-AND-006 | Missing | 23. Chat and taunts | Chat while actively playing without interfering with controls. | parity:G23-CHAT-AND-006 |
| G23-CHAT-AND-007 | Missing | 23. Chat and taunts | Taunt file/configuration. | parity:G23-CHAT-AND-007 |
| G23-CHAT-AND-008 | Missing | 23. Chat and taunts | Alt+letter and Alt+number taunts. | parity:G23-CHAT-AND-008 |
| G23-CHAT-AND-009 | Missing | 23. Chat and taunts | Per-profile taunts. | parity:G23-CHAT-AND-009 |
| G23-CHAT-AND-010 | Missing | 23. Chat and taunts | Command taunts. | parity:G23-CHAT-AND-010 |
| G23-CHAT-AND-011 | Missing | 23. Chat and taunts | Chat mute. | parity:G23-CHAT-AND-011 |
| G23-CHAT-AND-012 | Missing | 23. Chat and taunts | Mute by player name. | parity:G23-CHAT-AND-012 |
| G23-CHAT-AND-013 | Missing | 23. Chat and taunts | Mute by player ID. | parity:G23-CHAT-AND-013 |
| G23-CHAT-AND-014 | Missing | 23. Chat and taunts | Spam/flood controls beyond the basic one-second limit. | parity:G23-CHAT-AND-014 |
| G23-CHAT-AND-015 | Missing | 23. Chat and taunts | Profanity/moderation options if required. | parity:G23-CHAT-AND-015 |
| G23-CHAT-AND-016 | Missing | 23. Chat and taunts | Server announcements. | parity:G23-CHAT-AND-016 |
| G23-CHAT-AND-017 | Missing | 23. Chat and taunts | Join/leave messages. | parity:G23-CHAT-AND-017 |
| G23-CHAT-AND-018 | Missing | 23. Chat and taunts | Kill/capture announcements. | parity:G23-CHAT-AND-018 |
| G23-CHAT-AND-019 | Missing | 23. Chat and taunts | Realistic/Survival chat-visibility rules. | parity:G23-CHAT-AND-019 |
| G24-PLAYER-COMMANDS-001 | Missing | 24. Player commands | `/KILL` | parity:G24-PLAYER-COMMANDS-001 |
| G24-PLAYER-COMMANDS-002 | Missing | 24. Player commands | `/BRUTALKILL` | parity:G24-PLAYER-COMMANDS-002 |
| G24-PLAYER-COMMANDS-003 | Missing | 24. Player commands | `/MERCY` | parity:G24-PLAYER-COMMANDS-003 |
| G24-PLAYER-COMMANDS-004 | Missing | 24. Player commands | `/SMOKE` | parity:G24-PLAYER-COMMANDS-004 |
| G24-PLAYER-COMMANDS-005 | Missing | 24. Player commands | `/TABAC` | parity:G24-PLAYER-COMMANDS-005 |
| G24-PLAYER-COMMANDS-006 | Missing | 24. Player commands | `/TAKEOFF` | parity:G24-PLAYER-COMMANDS-006 |
| G24-PLAYER-COMMANDS-007 | Missing | 24. Player commands | `/VICTORY` | parity:G24-PLAYER-COMMANDS-007 |
| G24-PLAYER-COMMANDS-008 | Missing | 24. Player commands | `/PAUSE` | parity:G24-PLAYER-COMMANDS-008 |
| G24-PLAYER-COMMANDS-009 | Missing | 24. Player commands | `/UNPAUSE` | parity:G24-PLAYER-COMMANDS-009 |
| G25-SERVER-AND-001 | Missing | 25. Server and administrator commands | `/ADDMAP <map>` | parity:G25-SERVER-AND-001 |
| G25-SERVER-AND-002 | Missing | 25. Server and administrator commands | `/DELMAP <map>` | parity:G25-SERVER-AND-002 |
| G25-SERVER-AND-003 | Missing | 25. Server and administrator commands | `/ADDBOT<team> <bot>` | parity:G25-SERVER-AND-003 |
| G25-SERVER-AND-004 | Missing | 25. Server and administrator commands | `/KICK <player or ID>` | parity:G25-SERVER-AND-004 |
| G25-SERVER-AND-005 | Missing | 25. Server and administrator commands | `/KICKLAST` | parity:G25-SERVER-AND-005 |
| G25-SERVER-AND-006 | Missing | 25. Server and administrator commands | `/TEMPBAN <minutes> <IP/player>` | parity:G25-SERVER-AND-006 |
| G25-SERVER-AND-007 | Missing | 25. Server and administrator commands | `/BAN <player or ID>` | parity:G25-SERVER-AND-007 |
| G25-SERVER-AND-008 | Missing | 25. Server and administrator commands | `/BANIP <IP>` | parity:G25-SERVER-AND-008 |
| G25-SERVER-AND-009 | Missing | 25. Server and administrator commands | `/UNBAN <IP>` | parity:G25-SERVER-AND-009 |
| G25-SERVER-AND-010 | Missing | 25. Server and administrator commands | `/MAP <map>` | parity:G25-SERVER-AND-010 |
| G25-SERVER-AND-011 | Missing | 25. Server and administrator commands | `/RESTART` | parity:G25-SERVER-AND-011 |
| G25-SERVER-AND-012 | Missing | 25. Server and administrator commands | `/NEXTMAP` | parity:G25-SERVER-AND-012 |
| G25-SERVER-AND-013 | Missing | 25. Server and administrator commands | `/ADM <player>` | parity:G25-SERVER-AND-013 |
| G25-SERVER-AND-014 | Missing | 25. Server and administrator commands | `/ADMIP <IP>` | parity:G25-SERVER-AND-014 |
| G25-SERVER-AND-015 | Missing | 25. Server and administrator commands | `/UNADM <IP>` | parity:G25-SERVER-AND-015 |
| G25-SERVER-AND-016 | Missing | 25. Server and administrator commands | `/RESPAWNTIME <seconds>` | parity:G25-SERVER-AND-016 |
| G25-SERVER-AND-017 | Missing | 25. Server and administrator commands | Admin authentication. | parity:G25-SERVER-AND-017 |
| G25-SERVER-AND-018 | Missing | 25. Server and administrator commands | Remote-admin support. | parity:G25-SERVER-AND-018 |
| G25-SERVER-AND-019 | Missing | 25. Server and administrator commands | Persistent ban list. | parity:G25-SERVER-AND-019 |
| G25-SERVER-AND-020 | Missing | 25. Server and administrator commands | Persistent admin list. | parity:G25-SERVER-AND-020 |
| G25-SERVER-AND-021 | Missing | 25. Server and administrator commands | Audit log. | parity:G25-SERVER-AND-021 |
| G25-SERVER-AND-022 | Missing | 25. Server and administrator commands | Command authorization. | parity:G25-SERVER-AND-022 |
| G25-SERVER-AND-023 | Missing | 25. Server and administrator commands | Player-ID display. | parity:G25-SERVER-AND-023 |
| G25-SERVER-AND-024 | Missing | 25. Server and administrator commands | Safe command parsing. | parity:G25-SERVER-AND-024 |
| G25-SERVER-AND-025 | Missing | 25. Server and administrator commands | Command feedback and errors. | parity:G25-SERVER-AND-025 |
| G25-SERVER-AND-026 | Missing | 25. Server and administrator commands | Map-list loading. | parity:G25-SERVER-AND-026 |
| G25-SERVER-AND-027 | Missing | 25. Server and administrator commands | Server configuration reload. | parity:G25-SERVER-AND-027 |
| G25-SERVER-AND-028 | Missing | 25. Server and administrator commands | Lobby re-registration where applicable. | parity:G25-SERVER-AND-028 |
| G25-SERVER-AND-029 | Missing | 25. Server and administrator commands | Password changes. | parity:G25-SERVER-AND-029 |
| G25-SERVER-AND-030 | Missing | 25. Server and administrator commands | Maximum-player changes. | parity:G25-SERVER-AND-030 |
| G25-SERVER-AND-031 | Missing | 25. Server and administrator commands | Locked mode preventing sensitive runtime changes. | parity:G25-SERVER-AND-031 |
| G26-MATCH-AND-001 | Missing | 26. Match and server settings | Game mode. | parity:G26-MATCH-AND-001 |
| G26-MATCH-AND-002 | Missing | 26. Match and server settings | Kill/point limit. | parity:G26-MATCH-AND-002 |
| G26-MATCH-AND-003 | Missing | 26. Match and server settings | Capture limit. | parity:G26-MATCH-AND-003 |
| G26-MATCH-AND-004 | Missing | 26. Match and server settings | Time limit. | parity:G26-MATCH-AND-004 |
| G26-MATCH-AND-005 | Missing | 26. Match and server settings | Survival toggle. | parity:G26-MATCH-AND-005 |
| G26-MATCH-AND-006 | Missing | 26. Match and server settings | Realistic toggle. | parity:G26-MATCH-AND-006 |
| G26-MATCH-AND-007 | Missing | 26. Match and server settings | Advance toggle. | parity:G26-MATCH-AND-007 |
| G26-MATCH-AND-008 | Missing | 26. Match and server settings | Map-list looping. | parity:G26-MATCH-AND-008 |
| G26-MATCH-AND-009 | Missing | 26. Match and server settings | Random bots. | parity:G26-MATCH-AND-009 |
| G26-MATCH-AND-010 | Missing | 26. Match and server settings | Team-specific bots. | parity:G26-MATCH-AND-010 |
| G26-MATCH-AND-011 | Missing | 26. Match and server settings | Server name. | parity:G26-MATCH-AND-011 |
| G26-MATCH-AND-012 | Missing | 26. Match and server settings | Server password. | parity:G26-MATCH-AND-012 |
| G26-MATCH-AND-013 | Missing | 26. Match and server settings | Maximum players. | parity:G26-MATCH-AND-013 |
| G26-MATCH-AND-014 | Missing | 26. Match and server settings | Game port. | parity:G26-MATCH-AND-014 |
| G26-MATCH-AND-015 | Missing | 26. Match and server settings | Public/private listing. | parity:G26-MATCH-AND-015 |
| G26-MATCH-AND-016 | Missing | 26. Match and server settings | Team balancing. | parity:G26-MATCH-AND-016 |
| G26-MATCH-AND-017 | Missing | 26. Match and server settings | Maximum allowed ping. | parity:G26-MATCH-AND-017 |
| G26-MATCH-AND-018 | Missing | 26. Match and server settings | Ping-kick behavior. | parity:G26-MATCH-AND-018 |
| G26-MATCH-AND-019 | Missing | 26. Match and server settings | Welcome/server message. | parity:G26-MATCH-AND-019 |
| G26-MATCH-AND-020 | Missing | 26. Match and server settings | Server contact link. | parity:G26-MATCH-AND-020 |
| G26-MATCH-AND-021 | Missing | 26. Match and server settings | Dedicated-server configuration. | parity:G26-MATCH-AND-021 |
| G26-MATCH-AND-022 | Missing | 26. Match and server settings | Custom map list. | parity:G26-MATCH-AND-022 |
| G26-MATCH-AND-023 | Missing | 26. Match and server settings | Custom weapon mod. | parity:G26-MATCH-AND-023 |
| G26-MATCH-AND-024 | Missing | 26. Match and server settings | Friendly fire. | parity:G26-MATCH-AND-024 |
| G26-MATCH-AND-025 | Missing | 26. Match and server settings | Respawn time. | parity:G26-MATCH-AND-025 |
| G26-MATCH-AND-026 | Missing | 26. Match and server settings | Bonus frequency/enabled state. | parity:G26-MATCH-AND-026 |
| G26-MATCH-AND-027 | Missing | 26. Match and server settings | Spectator limits. | parity:G26-MATCH-AND-027 |
| G26-MATCH-AND-028 | Missing | 26. Match and server settings | Admin password/authentication. | parity:G26-MATCH-AND-028 |
| G26-MATCH-AND-029 | Missing | 26. Match and server settings | Logging options. | parity:G26-MATCH-AND-029 |
| G26-MATCH-AND-030 | Missing | 26. Match and server settings | Name. | parity:G26-MATCH-AND-030 |
| G26-MATCH-AND-031 | Missing | 26. Match and server settings | Appearance. | parity:G26-MATCH-AND-031 |
| G26-MATCH-AND-032 | Missing | 26. Match and server settings | Default secondary. | parity:G26-MATCH-AND-032 |
| G26-MATCH-AND-033 | Missing | 26. Match and server settings | Background override. | parity:G26-MATCH-AND-033 |
| G26-MATCH-AND-034 | Missing | 26. Match and server settings | Controls. | parity:G26-MATCH-AND-034 |
| G26-MATCH-AND-035 | Missing | 26. Match and server settings | Mouse sensitivity. | parity:G26-MATCH-AND-035 |
| G26-MATCH-AND-036 | Missing | 26. Match and server settings | Interface selection. | parity:G26-MATCH-AND-036 |
| G26-MATCH-AND-037 | Missing | 26. Match and server settings | Player indicator. | parity:G26-MATCH-AND-037 |
| G26-MATCH-AND-038 | Missing | 26. Match and server settings | Sniper line. | parity:G26-MATCH-AND-038 |
| G26-MATCH-AND-039 | Missing | 26. Match and server settings | Fullscreen/windowed. | parity:G26-MATCH-AND-039 |
| G26-MATCH-AND-040 | Missing | 26. Match and server settings | Display resolution. | parity:G26-MATCH-AND-040 |
| G26-MATCH-AND-041 | Missing | 26. Match and server settings | Desktop resolution. | parity:G26-MATCH-AND-041 |
| G26-MATCH-AND-042 | Missing | 26. Match and server settings | Interface scaling. | parity:G26-MATCH-AND-042 |
| G26-MATCH-AND-043 | Missing | 26. Match and server settings | Particle limit. | parity:G26-MATCH-AND-043 |
| G26-MATCH-AND-044 | Missing | 26. Match and server settings | Bullet trails. | parity:G26-MATCH-AND-044 |
| G26-MATCH-AND-045 | Missing | 26. Match and server settings | Weather rendering. | parity:G26-MATCH-AND-045 |
| G26-MATCH-AND-046 | Missing | 26. Match and server settings | Texture filtering. | parity:G26-MATCH-AND-046 |
| G26-MATCH-AND-047 | Missing | 26. Match and server settings | Resolution filtering. | parity:G26-MATCH-AND-047 |
| G26-MATCH-AND-048 | Missing | 26. Match and server settings | Mipmapping. | parity:G26-MATCH-AND-048 |
| G26-MATCH-AND-049 | Missing | 26. Match and server settings | Compatibility/fixed-pipeline equivalent. | parity:G26-MATCH-AND-049 |
| G26-MATCH-AND-050 | Missing | 26. Match and server settings | Intro playback. | parity:G26-MATCH-AND-050 |
| G26-MATCH-AND-051 | Missing | 26. Match and server settings | Final-score screenshot. | parity:G26-MATCH-AND-051 |
| G26-MATCH-AND-052 | Missing | 26. Match and server settings | Clanmatch color behavior. | parity:G26-MATCH-AND-052 |
| G26-MATCH-AND-053 | Missing | 26. Match and server settings | Frame-rate limit or VSync. | parity:G26-MATCH-AND-053 |
| G26-MATCH-AND-054 | Missing | 26. Match and server settings | Performance statistics. | parity:G26-MATCH-AND-054 |
| G26-MATCH-AND-055 | Missing | 26. Match and server settings | Sound volume. | parity:G26-MATCH-AND-055 |
| G26-MATCH-AND-056 | Missing | 26. Match and server settings | Music volume. | parity:G26-MATCH-AND-056 |
| G26-MATCH-AND-057 | Missing | 26. Match and server settings | Sound quality. | parity:G26-MATCH-AND-057 |
| G26-MATCH-AND-058 | Missing | 26. Match and server settings | Output device. | parity:G26-MATCH-AND-058 |
| G26-MATCH-AND-059 | Missing | 26. Match and server settings | Explosion effect. | parity:G26-MATCH-AND-059 |
| G26-MATCH-AND-060 | Missing | 26. Match and server settings | Distant battle. | parity:G26-MATCH-AND-060 |
| G26-MATCH-AND-061 | Missing | 26. Match and server settings | Game music. | parity:G26-MATCH-AND-061 |
| G27-LOBBY-AND-001 | Missing | 27. Lobby and room browser | Server/room refresh. | parity:G27-LOBBY-AND-001 |
| G27-LOBBY-AND-002 | Missing | 27. Lobby and room browser | Cancel refresh. | parity:G27-LOBBY-AND-002 |
| G27-LOBBY-AND-003 | Missing | 27. Lobby and room browser | Ping measurement. | parity:G27-LOBBY-AND-003 |
| G27-LOBBY-AND-004 | Missing | 27. Lobby and room browser | Ping-all action. | parity:G27-LOBBY-AND-004 |
| G27-LOBBY-AND-005 | Missing | 27. Lobby and room browser | Ping column. | parity:G27-LOBBY-AND-005 |
| G27-LOBBY-AND-006 | Missing | 27. Lobby and room browser | Player count. | parity:G27-LOBBY-AND-006 |
| G27-LOBBY-AND-007 | Missing | 27. Lobby and room browser | Maximum players. | parity:G27-LOBBY-AND-007 |
| G27-LOBBY-AND-008 | Missing | 27. Lobby and room browser | Game mode. | parity:G27-LOBBY-AND-008 |
| G27-LOBBY-AND-009 | Missing | 27. Lobby and room browser | Map name. | parity:G27-LOBBY-AND-009 |
| G27-LOBBY-AND-010 | Missing | 27. Lobby and room browser | Country/region if desired. | parity:G27-LOBBY-AND-010 |
| G27-LOBBY-AND-011 | Missing | 27. Lobby and room browser | Password indicator. | parity:G27-LOBBY-AND-011 |
| G27-LOBBY-AND-012 | Missing | 27. Lobby and room browser | Realistic indicator. | parity:G27-LOBBY-AND-012 |
| G27-LOBBY-AND-013 | Missing | 27. Lobby and room browser | Survival indicator. | parity:G27-LOBBY-AND-013 |
| G27-LOBBY-AND-014 | Missing | 27. Lobby and room browser | Advance indicator. | parity:G27-LOBBY-AND-014 |
| G27-LOBBY-AND-015 | Missing | 27. Lobby and room browser | Weapon-mod indicator. | parity:G27-LOBBY-AND-015 |
| G27-LOBBY-AND-016 | Missing | 27. Lobby and room browser | Version compatibility. | parity:G27-LOBBY-AND-016 |
| G27-LOBBY-AND-017 | Missing | 27. Lobby and room browser | Favorites. | parity:G27-LOBBY-AND-017 |
| G27-LOBBY-AND-018 | Missing | 27. Lobby and room browser | Favorite add/remove. | parity:G27-LOBBY-AND-018 |
| G27-LOBBY-AND-019 | Missing | 27. Lobby and room browser | Direct IP/hostname join if supported. | parity:G27-LOBBY-AND-019 |
| G27-LOBBY-AND-020 | Missing | 27. Lobby and room browser | Port input. | parity:G27-LOBBY-AND-020 |
| G27-LOBBY-AND-021 | Missing | 27. Lobby and room browser | Password input. | parity:G27-LOBBY-AND-021 |
| G27-LOBBY-AND-022 | Missing | 27. Lobby and room browser | Spectator join. | parity:G27-LOBBY-AND-022 |
| G27-LOBBY-AND-023 | Missing | 27. Lobby and room browser | Sort and filter. | parity:G27-LOBBY-AND-023 |
| G27-LOBBY-AND-024 | Missing | 27. Lobby and room browser | Search. | parity:G27-LOBBY-AND-024 |
| G27-LOBBY-AND-025 | Missing | 27. Lobby and room browser | Full-room filtering. | parity:G27-LOBBY-AND-025 |
| G27-LOBBY-AND-026 | Missing | 27. Lobby and room browser | Empty-room filtering. | parity:G27-LOBBY-AND-026 |
| G27-LOBBY-AND-027 | Missing | 27. Lobby and room browser | Public lobby registration. | parity:G27-LOBBY-AND-027 |
| G27-LOBBY-AND-028 | Missing | 27. Lobby and room browser | Reliable server discovery. | parity:G27-LOBBY-AND-028 |
| G27-LOBBY-AND-029 | Missing | 27. Lobby and room browser | Join/download progress. | parity:G27-LOBBY-AND-029 |
| G27-LOBBY-AND-030 | Missing | 27. Lobby and room browser | Cancel connection/download. | parity:G27-LOBBY-AND-030 |
| G27-LOBBY-AND-031 | Missing | 27. Lobby and room browser | Better reconnect state. | parity:G27-LOBBY-AND-031 |
| G27-LOBBY-AND-032 | Missing | 27. Lobby and room browser | Room ownership/host settings. | parity:G27-LOBBY-AND-032 |
| G27-LOBBY-AND-033 | Missing | 27. Lobby and room browser | Room deletion or expiry controls. | parity:G27-LOBBY-AND-033 |
| G27-LOBBY-AND-034 | Missing | 27. Lobby and room browser | Password-protected rooms in addition to invite codes. | parity:G27-LOBBY-AND-034 |
| G27-LOBBY-AND-035 | Missing | 27. Lobby and room browser | Team choice after joining. | parity:G27-LOBBY-AND-035 |
| G27-LOBBY-AND-036 | Missing | 27. Lobby and room browser | Late-join rules. | parity:G27-LOBBY-AND-036 |
| G27-LOBBY-AND-037 | Missing | 27. Lobby and room browser | Abuse-resistant room creation. | parity:G27-LOBBY-AND-037 |
| G27-LOBBY-AND-038 | Missing | 27. Lobby and room browser | Rate limiting for connection and room operations. | parity:G27-LOBBY-AND-038 |
| G28-SPECTATING-001 | Missing | 28. Spectating | Spectator team. | parity:G28-SPECTATING-001 |
| G28-SPECTATING-002 | Missing | 28. Spectating | Join directly as spectator. | parity:G28-SPECTATING-002 |
| G28-SPECTATING-003 | Missing | 28. Spectating | Switch followed player. | parity:G28-SPECTATING-003 |
| G28-SPECTATING-004 | Missing | 28. Spectating | Previous/next player. | parity:G28-SPECTATING-004 |
| G28-SPECTATING-005 | Missing | 28. Spectating | Free camera if desired. | parity:G28-SPECTATING-005 |
| G28-SPECTATING-006 | Missing | 28. Spectating | Spectator HUD. | parity:G28-SPECTATING-006 |
| G28-SPECTATING-007 | Missing | 28. Spectating | Spectator scoreboard. | parity:G28-SPECTATING-007 |
| G28-SPECTATING-008 | Missing | 28. Spectating | Spectator chat restrictions. | parity:G28-SPECTATING-008 |
| G28-SPECTATING-009 | Missing | 28. Spectating | Realistic visibility restrictions. | parity:G28-SPECTATING-009 |
| G28-SPECTATING-010 | Missing | 28. Spectating | Survival restrictions. | parity:G28-SPECTATING-010 |
| G28-SPECTATING-011 | Missing | 28. Spectating | Followed-player minimap/visibility behavior. | parity:G28-SPECTATING-011 |
| G28-SPECTATING-012 | Missing | 28. Spectating | Delay option for competitive matches. | parity:G28-SPECTATING-012 |
| G29-SCORING-AND-001 | Missing | 29. Scoring and statistics | Correct scoring for every game mode. | parity:G29-SCORING-AND-001 |
| G29-SCORING-AND-002 | Missing | 29. Scoring and statistics | Match time. | parity:G29-SCORING-AND-002 |
| G29-SCORING-AND-003 | Missing | 29. Scoring and statistics | Individual points. | parity:G29-SCORING-AND-003 |
| G29-SCORING-AND-004 | Missing | 29. Scoring and statistics | Team points. | parity:G29-SCORING-AND-004 |
| G29-SCORING-AND-005 | Missing | 29. Scoring and statistics | Captures. | parity:G29-SCORING-AND-005 |
| G29-SCORING-AND-006 | Missing | 29. Scoring and statistics | Flag returns. | parity:G29-SCORING-AND-006 |
| G29-SCORING-AND-007 | Missing | 29. Scoring and statistics | Current server rank. | parity:G29-SCORING-AND-007 |
| G29-SCORING-AND-008 | Missing | 29. Scoring and statistics | Score difference from leader. | parity:G29-SCORING-AND-008 |
| G29-SCORING-AND-009 | Missing | 29. Scoring and statistics | Kill/point/capture limit display. | parity:G29-SCORING-AND-009 |
| G29-SCORING-AND-010 | Missing | 29. Scoring and statistics | Weapon statistics for the current round. | parity:G29-SCORING-AND-010 |
| G29-SCORING-AND-011 | Missing | 29. Scoring and statistics | Shots fired. | parity:G29-SCORING-AND-011 |
| G29-SCORING-AND-012 | Missing | 29. Scoring and statistics | Hits. | parity:G29-SCORING-AND-012 |
| G29-SCORING-AND-013 | Missing | 29. Scoring and statistics | Accuracy. | parity:G29-SCORING-AND-013 |
| G29-SCORING-AND-014 | Missing | 29. Scoring and statistics | Kills per weapon. | parity:G29-SCORING-AND-014 |
| G29-SCORING-AND-015 | Missing | 29. Scoring and statistics | Deaths per weapon/cause. | parity:G29-SCORING-AND-015 |
| G29-SCORING-AND-016 | Missing | 29. Scoring and statistics | Headshots. | parity:G29-SCORING-AND-016 |
| G29-SCORING-AND-017 | Missing | 29. Scoring and statistics | Suicides. | parity:G29-SCORING-AND-017 |
| G29-SCORING-AND-018 | Missing | 29. Scoring and statistics | Teamkills. | parity:G29-SCORING-AND-018 |
| G29-SCORING-AND-019 | Missing | 29. Scoring and statistics | Objective statistics. | parity:G29-SCORING-AND-019 |
| G29-SCORING-AND-020 | Missing | 29. Scoring and statistics | End-of-round summary. | parity:G29-SCORING-AND-020 |
| G29-SCORING-AND-021 | Missing | 29. Scoring and statistics | Match history. | parity:G29-SCORING-AND-021 |
| G29-SCORING-AND-022 | Missing | 29. Scoring and statistics | Persistent player statistics if desired. | parity:G29-SCORING-AND-022 |
| G29-SCORING-AND-023 | Missing | 29. Scoring and statistics | Clan/team statistics if desired. | parity:G29-SCORING-AND-023 |
| G29-SCORING-AND-024 | Missing | 29. Scoring and statistics | Exportable logs. | parity:G29-SCORING-AND-024 |
| G30-NETWORKING-AND-001 | Missing | 30. Networking and prediction | Proper client-side prediction for all movement states. | parity:G30-NETWORKING-AND-001 |
| G30-NETWORKING-AND-002 | Missing | 30. Networking and prediction | Input reconciliation. | parity:G30-NETWORKING-AND-002 |
| G30-NETWORKING-AND-003 | Missing | 30. Networking and prediction | Unacknowledged-input replay. | parity:G30-NETWORKING-AND-003 |
| G30-NETWORKING-AND-004 | Missing | 30. Networking and prediction | Remote-player interpolation. | parity:G30-NETWORKING-AND-004 |
| G30-NETWORKING-AND-005 | Missing | 30. Networking and prediction | Projectile interpolation. | parity:G30-NETWORKING-AND-005 |
| G30-NETWORKING-AND-006 | Missing | 30. Networking and prediction | Extrapolation limits. | parity:G30-NETWORKING-AND-006 |
| G30-NETWORKING-AND-007 | Missing | 30. Networking and prediction | Latency compensation. | parity:G30-NETWORKING-AND-007 |
| G30-NETWORKING-AND-008 | Missing | 30. Networking and prediction | Server rewind/lag compensation if appropriate. | parity:G30-NETWORKING-AND-008 |
| G30-NETWORKING-AND-009 | Missing | 30. Networking and prediction | Clock synchronization. | parity:G30-NETWORKING-AND-009 |
| G30-NETWORKING-AND-010 | Missing | 30. Networking and prediction | Measured ping. | parity:G30-NETWORKING-AND-010 |
| G30-NETWORKING-AND-011 | Missing | 30. Networking and prediction | Packet-loss handling. | parity:G30-NETWORKING-AND-011 |
| G30-NETWORKING-AND-012 | Missing | 30. Networking and prediction | Snapshot delta compression. | parity:G30-NETWORKING-AND-012 |
| G30-NETWORKING-AND-013 | Missing | 30. Networking and prediction | Interest management if maps/player counts grow. | parity:G30-NETWORKING-AND-013 |
| G30-NETWORKING-AND-014 | Missing | 30. Networking and prediction | Binary protocol or more compact encoding if needed. | parity:G30-NETWORKING-AND-014 |
| G30-NETWORKING-AND-015 | Missing | 30. Networking and prediction | Weapon/event prediction. | parity:G30-NETWORKING-AND-015 |
| G30-NETWORKING-AND-016 | Missing | 30. Networking and prediction | Predicted muzzle/projectile effects. | parity:G30-NETWORKING-AND-016 |
| G30-NETWORKING-AND-017 | Missing | 30. Networking and prediction | Rollback correction smoothing. | parity:G30-NETWORKING-AND-017 |
| G30-NETWORKING-AND-018 | Missing | 30. Networking and prediction | Map and ruleset synchronization. | parity:G30-NETWORKING-AND-018 |
| G30-NETWORKING-AND-019 | Missing | 30. Networking and prediction | Disconnect reason handling. | parity:G30-NETWORKING-AND-019 |
| G30-NETWORKING-AND-020 | Missing | 30. Networking and prediction | Robust resumption after page sleep/mobile backgrounding. | parity:G30-NETWORKING-AND-020 |
| G30-NETWORKING-AND-021 | Missing | 30. Networking and prediction | Duplicate-session handling. | parity:G30-NETWORKING-AND-021 |
| G30-NETWORKING-AND-022 | Missing | 30. Networking and prediction | Rate limits for every client message. | parity:G30-NETWORKING-AND-022 |
| G30-NETWORKING-AND-023 | Missing | 30. Networking and prediction | Anti-speedhack/input-frequency validation. | parity:G30-NETWORKING-AND-023 |
| G30-NETWORKING-AND-024 | Missing | 30. Networking and prediction | Fire-rate validation. | parity:G30-NETWORKING-AND-024 |
| G30-NETWORKING-AND-025 | Missing | 30. Networking and prediction | Aim/input sanity validation beyond coordinate bounds. | parity:G30-NETWORKING-AND-025 |
| G30-NETWORKING-AND-026 | Missing | 30. Networking and prediction | Server-authoritative pickups, flags, bonuses, and round state. | parity:G30-NETWORKING-AND-026 |
| G30-NETWORKING-AND-027 | Missing | 30. Networking and prediction | Network load and soak tests. | parity:G30-NETWORKING-AND-027 |
| G30-NETWORKING-AND-028 | Missing | 30. Networking and prediction | High-latency/jitter/loss simulation tests. | parity:G30-NETWORKING-AND-028 |
| G31-ANTI-CHEAT-001 | Missing | 31. Anti-cheat and abuse resistance | Server-authoritative collision validation. | parity:G31-ANTI-CHEAT-001 |
| G31-ANTI-CHEAT-002 | Missing | 31. Anti-cheat and abuse resistance | Server-authoritative weapon selection. | parity:G31-ANTI-CHEAT-002 |
| G31-ANTI-CHEAT-003 | Missing | 31. Anti-cheat and abuse resistance | Server-authoritative reload and inventory. | parity:G31-ANTI-CHEAT-003 |
| G31-ANTI-CHEAT-004 | Missing | 31. Anti-cheat and abuse resistance | Input-rate limiting. | parity:G31-ANTI-CHEAT-004 |
| G31-ANTI-CHEAT-005 | Missing | 31. Anti-cheat and abuse resistance | Movement feasibility validation. | parity:G31-ANTI-CHEAT-005 |
| G31-ANTI-CHEAT-006 | Missing | 31. Anti-cheat and abuse resistance | Aim-value validation. | parity:G31-ANTI-CHEAT-006 |
| G31-ANTI-CHEAT-007 | Missing | 31. Anti-cheat and abuse resistance | Chat abuse controls. | parity:G31-ANTI-CHEAT-007 |
| G31-ANTI-CHEAT-008 | Missing | 31. Anti-cheat and abuse resistance | Connection/IP rate limiting. | parity:G31-ANTI-CHEAT-008 |
| G31-ANTI-CHEAT-009 | Missing | 31. Anti-cheat and abuse resistance | Room-creation rate limiting. | parity:G31-ANTI-CHEAT-009 |
| G31-ANTI-CHEAT-010 | Missing | 31. Anti-cheat and abuse resistance | Admin permission validation. | parity:G31-ANTI-CHEAT-010 |
| G31-ANTI-CHEAT-011 | Missing | 31. Anti-cheat and abuse resistance | Ban enforcement. | parity:G31-ANTI-CHEAT-011 |
| G31-ANTI-CHEAT-012 | Missing | 31. Anti-cheat and abuse resistance | Temporary bans. | parity:G31-ANTI-CHEAT-012 |
| G31-ANTI-CHEAT-013 | Missing | 31. Anti-cheat and abuse resistance | Audit logging. | parity:G31-ANTI-CHEAT-013 |
| G31-ANTI-CHEAT-014 | Missing | 31. Anti-cheat and abuse resistance | Suspicious behavior metrics. | parity:G31-ANTI-CHEAT-014 |
| G31-ANTI-CHEAT-015 | Missing | 31. Anti-cheat and abuse resistance | Protocol fuzzing. | parity:G31-ANTI-CHEAT-015 |
| G31-ANTI-CHEAT-016 | Missing | 31. Anti-cheat and abuse resistance | Malformed WebSocket testing. | parity:G31-ANTI-CHEAT-016 |
| G31-ANTI-CHEAT-017 | Missing | 31. Anti-cheat and abuse resistance | Replay-based cheat investigation. | parity:G31-ANTI-CHEAT-017 |
| G31-ANTI-CHEAT-018 | Missing | 31. Anti-cheat and abuse resistance | Secure resume tokens. | parity:G31-ANTI-CHEAT-018 |
| G31-ANTI-CHEAT-019 | Missing | 31. Anti-cheat and abuse resistance | Token expiry/rotation. | parity:G31-ANTI-CHEAT-019 |
| G31-ANTI-CHEAT-020 | Missing | 31. Anti-cheat and abuse resistance | Deployment-level denial-of-service protection. | parity:G31-ANTI-CHEAT-020 |
| G31-ANTI-CHEAT-021 | Missing | 31. Anti-cheat and abuse resistance | No trust in client-provided cosmetics or settings that affect gameplay. | parity:G31-ANTI-CHEAT-021 |
| G32-MENUS-AND-001 | Missing | 32. Menus and overall game flow | Main menu. | parity:G32-MENUS-AND-001 |
| G32-MENUS-AND-002 | Missing | 32. Menus and overall game flow | Profile selection. | parity:G32-MENUS-AND-002 |
| G32-MENUS-AND-003 | Missing | 32. Menus and overall game flow | Player customization. | parity:G32-MENUS-AND-003 |
| G32-MENUS-AND-004 | Missing | 32. Menus and overall game flow | Join-game screen. | parity:G32-MENUS-AND-004 |
| G32-MENUS-AND-005 | Missing | 32. Menus and overall game flow | Start-game/server screen. | parity:G32-MENUS-AND-005 |
| G32-MENUS-AND-006 | Missing | 32. Menus and overall game flow | Options menu. | parity:G32-MENUS-AND-006 |
| G32-MENUS-AND-007 | Missing | 32. Menus and overall game flow | Controls menu. | parity:G32-MENUS-AND-007 |
| G32-MENUS-AND-008 | Missing | 32. Menus and overall game flow | Weapon-selection screen during respawn. | parity:G32-MENUS-AND-008 |
| G32-MENUS-AND-009 | Missing | 32. Menus and overall game flow | Team-selection screen. | parity:G32-MENUS-AND-009 |
| G32-MENUS-AND-010 | Missing | 32. Menus and overall game flow | Spectator selection. | parity:G32-MENUS-AND-010 |
| G32-MENUS-AND-011 | Missing | 32. Menus and overall game flow | Pause menu. | parity:G32-MENUS-AND-011 |
| G32-MENUS-AND-012 | Missing | 32. Menus and overall game flow | Disconnect confirmation. | parity:G32-MENUS-AND-012 |
| G32-MENUS-AND-013 | Missing | 32. Menus and overall game flow | Map loading screen. | parity:G32-MENUS-AND-013 |
| G32-MENUS-AND-014 | Missing | 32. Menus and overall game flow | Asset-download screen. | parity:G32-MENUS-AND-014 |
| G32-MENUS-AND-015 | Missing | 32. Menus and overall game flow | Round-intro countdown. | parity:G32-MENUS-AND-015 |
| G32-MENUS-AND-016 | Missing | 32. Menus and overall game flow | Round-end screen. | parity:G32-MENUS-AND-016 |
| G32-MENUS-AND-017 | Missing | 32. Menus and overall game flow | Match-results screen. | parity:G32-MENUS-AND-017 |
| G32-MENUS-AND-018 | Missing | 32. Menus and overall game flow | Connection-lost overlay. | parity:G32-MENUS-AND-018 |
| G32-MENUS-AND-019 | Missing | 32. Menus and overall game flow | Version mismatch/update flow. | parity:G32-MENUS-AND-019 |
| G32-MENUS-AND-020 | Missing | 32. Menus and overall game flow | Credits. | parity:G32-MENUS-AND-020 |
| G32-MENUS-AND-021 | Missing | 32. Menus and overall game flow | Help/manual. | parity:G32-MENUS-AND-021 |
| G32-MENUS-AND-022 | Missing | 32. Menus and overall game flow | First-run control tutorial. | parity:G32-MENUS-AND-022 |
| G32-MENUS-AND-023 | Missing | 32. Menus and overall game flow | Mobile onboarding. | parity:G32-MENUS-AND-023 |
| G33-CUSTOM-INTERFACES-001 | Missing | 33. Custom interfaces and modding | Loadable HUD/interface definitions. | parity:G33-CUSTOM-INTERFACES-001 |
| G33-CUSTOM-INTERFACES-002 | Missing | 33. Custom interfaces and modding | Multiple interface presets. | parity:G33-CUSTOM-INTERFACES-002 |
| G33-CUSTOM-INTERFACES-003 | Missing | 33. Custom interfaces and modding | Custom cursor. | parity:G33-CUSTOM-INTERFACES-003 |
| G33-CUSTOM-INTERFACES-004 | Missing | 33. Custom interfaces and modding | Custom HUD image positions. | parity:G33-CUSTOM-INTERFACES-004 |
| G33-CUSTOM-INTERFACES-005 | Missing | 33. Custom interfaces and modding | Interface scaling. | parity:G33-CUSTOM-INTERFACES-005 |
| G33-CUSTOM-INTERFACES-006 | Missing | 33. Custom interfaces and modding | Custom weapon graphics. | parity:G33-CUSTOM-INTERFACES-006 |
| G33-CUSTOM-INTERFACES-007 | Missing | 33. Custom interfaces and modding | Custom sounds. | parity:G33-CUSTOM-INTERFACES-007 |
| G33-CUSTOM-INTERFACES-008 | Missing | 33. Custom interfaces and modding | Custom character/gostek graphics. | parity:G33-CUSTOM-INTERFACES-008 |
| G33-CUSTOM-INTERFACES-009 | Missing | 33. Custom interfaces and modding | `mod.ini`-style asset scaling. | parity:G33-CUSTOM-INTERFACES-009 |
| G33-CUSTOM-INTERFACES-010 | Missing | 33. Custom interfaces and modding | Mod selection/launching. | parity:G33-CUSTOM-INTERFACES-010 |
| G33-CUSTOM-INTERFACES-011 | Missing | 33. Custom interfaces and modding | Mod preview. | parity:G33-CUSTOM-INTERFACES-011 |
| G33-CUSTOM-INTERFACES-012 | Missing | 33. Custom interfaces and modding | Mod packaging. | parity:G33-CUSTOM-INTERFACES-012 |
| G33-CUSTOM-INTERFACES-013 | Missing | 33. Custom interfaces and modding | Mod downloading. | parity:G33-CUSTOM-INTERFACES-013 |
| G33-CUSTOM-INTERFACES-014 | Missing | 33. Custom interfaces and modding | Mod version/hash matching. | parity:G33-CUSTOM-INTERFACES-014 |
| G33-CUSTOM-INTERFACES-015 | Missing | 33. Custom interfaces and modding | Server-required mod support. | parity:G33-CUSTOM-INTERFACES-015 |
| G33-CUSTOM-INTERFACES-016 | Missing | 33. Custom interfaces and modding | Safe path handling. | parity:G33-CUSTOM-INTERFACES-016 |
| G33-CUSTOM-INTERFACES-017 | Missing | 33. Custom interfaces and modding | Asset-size limits. | parity:G33-CUSTOM-INTERFACES-017 |
| G33-CUSTOM-INTERFACES-018 | Missing | 33. Custom interfaces and modding | License/provenance metadata. | parity:G33-CUSTOM-INTERFACES-018 |
| G33-CUSTOM-INTERFACES-019 | Missing | 33. Custom interfaces and modding | Cabbage | parity:G33-CUSTOM-INTERFACES-019 |
| G33-CUSTOM-INTERFACES-020 | Missing | 33. Custom interfaces and modding | Classic | parity:G33-CUSTOM-INTERFACES-020 |
| G33-CUSTOM-INTERFACES-021 | Missing | 33. Custom interfaces and modding | Lacey V2 | parity:G33-CUSTOM-INTERFACES-021 |
| G33-CUSTOM-INTERFACES-022 | Missing | 33. Custom interfaces and modding | Micro1 | parity:G33-CUSTOM-INTERFACES-022 |
| G33-CUSTOM-INTERFACES-023 | Missing | 33. Custom interfaces and modding | Military | parity:G33-CUSTOM-INTERFACES-023 |
| G33-CUSTOM-INTERFACES-024 | Missing | 33. Custom interfaces and modding | Predator | parity:G33-CUSTOM-INTERFACES-024 |
| G33-CUSTOM-INTERFACES-025 | Missing | 33. Custom interfaces and modding | Soldat Style | parity:G33-CUSTOM-INTERFACES-025 |
| G33-CUSTOM-INTERFACES-026 | Missing | 33. Custom interfaces and modding | Storm | parity:G33-CUSTOM-INTERFACES-026 |
| G33-CUSTOM-INTERFACES-027 | Missing | 33. Custom interfaces and modding | Tech | parity:G33-CUSTOM-INTERFACES-027 |
| G33-CUSTOM-INTERFACES-028 | Missing | 33. Custom interfaces and modding | Text | parity:G33-CUSTOM-INTERFACES-028 |
| G34-MAP-EDITOR-001 | Present | 34. Map editor and content pipeline | Polygon creation/editing. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-002 | Present | 34. Map editor and content pipeline | Polygon type editing. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-003 | Present | 34. Map editor and content pipeline | Vertex manipulation. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-004 | Present | 34. Map editor and content pipeline | Texture assignment. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-005 | Present | 34. Map editor and content pipeline | Texture-coordinate editing. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-006 | Present | 34. Map editor and content pipeline | Multi-texture workflow. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-007 | Present | 34. Map editor and content pipeline | Scenery placement. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-008 | Present | 34. Map editor and content pipeline | Scenery layers. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-009 | Present | 34. Map editor and content pipeline | Collider placement. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-010 | Present | 34. Map editor and content pipeline | Spawn placement. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-011 | Present | 34. Map editor and content pipeline | Flag/objective placement. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-012 | Present | 34. Map editor and content pipeline | Bonus placement. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-013 | Present | 34. Map editor and content pipeline | Waypoint creation. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-014 | Present | 34. Map editor and content pipeline | Weather properties. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-015 | Present | 34. Map editor and content pipeline | Background colors. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-016 | Present | 34. Map editor and content pipeline | Footstep sounds. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-017 | Present | 34. Map editor and content pipeline | Jet-fuel setting. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-018 | Present | 34. Map editor and content pipeline | Map validation. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-019 | Present | 34. Map editor and content pipeline | Mode validation. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-020 | Present | 34. Map editor and content pipeline | Test/play button. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-021 | Present | 34. Map editor and content pipeline | Undo/redo. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-022 | Present | 34. Map editor and content pipeline | Copy/paste. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-023 | Present | 34. Map editor and content pipeline | Selection tools. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-024 | Present | 34. Map editor and content pipeline | Zoom/pan/grid. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-025 | Present | 34. Map editor and content pipeline | Prefabs. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-026 | Present | 34. Map editor and content pipeline | PMS import. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-027 | Present | 34. Map editor and content pipeline | PMS export. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-028 | Present | 34. Map editor and content pipeline | Project-native map format. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-029 | Present | 34. Map editor and content pipeline | Packaging custom assets. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-030 | Present | 34. Map editor and content pipeline | Map preview generation. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-031 | Present | 34. Map editor and content pipeline | Dedicated-server deployment. | rust:map_editor:editor_pipeline |
| G34-MAP-EDITOR-032 | Present | 34. Map editor and content pipeline | Cross-platform editor support. | rust:map_editor:editor_pipeline |
| G35-DEMO-AND-001 | Missing | 35. Demo and replay system | Record match inputs/events. | parity:G35-DEMO-AND-001 |
| G35-DEMO-AND-002 | Missing | 35. Demo and replay system | Deterministic replay format. | parity:G35-DEMO-AND-002 |
| G35-DEMO-AND-003 | Missing | 35. Demo and replay system | Replay metadata. | parity:G35-DEMO-AND-003 |
| G35-DEMO-AND-004 | Missing | 35. Demo and replay system | Replay playback. | parity:G35-DEMO-AND-004 |
| G35-DEMO-AND-005 | Missing | 35. Demo and replay system | Pause. | parity:G35-DEMO-AND-005 |
| G35-DEMO-AND-006 | Missing | 35. Demo and replay system | Seek. | parity:G35-DEMO-AND-006 |
| G35-DEMO-AND-007 | Missing | 35. Demo and replay system | Fast-forward. | parity:G35-DEMO-AND-007 |
| G35-DEMO-AND-008 | Missing | 35. Demo and replay system | Follow player. | parity:G35-DEMO-AND-008 |
| G35-DEMO-AND-009 | Missing | 35. Demo and replay system | Free camera. | parity:G35-DEMO-AND-009 |
| G35-DEMO-AND-010 | Missing | 35. Demo and replay system | Replay compatibility/versioning. | parity:G35-DEMO-AND-010 |
| G35-DEMO-AND-011 | Missing | 35. Demo and replay system | Replay validation. | parity:G35-DEMO-AND-011 |
| G35-DEMO-AND-012 | Missing | 35. Demo and replay system | Export or share replay. | parity:G35-DEMO-AND-012 |
| G35-DEMO-AND-013 | Missing | 35. Demo and replay system | Demo repair/recovery where feasible. | parity:G35-DEMO-AND-013 |
| G35-DEMO-AND-014 | Missing | 35. Demo and replay system | Server-side competitive match recording. | parity:G35-DEMO-AND-014 |
| G36-OPERATIONAL-AND-001 | Missing | 36. Operational and production work | Persistent server data. | parity:G36-OPERATIONAL-AND-001 |
| G36-OPERATIONAL-AND-002 | Missing | 36. Operational and production work | Persistent accounts. | parity:G36-OPERATIONAL-AND-002 |
| G36-OPERATIONAL-AND-003 | Missing | 36. Operational and production work | Persistent statistics. | parity:G36-OPERATIONAL-AND-003 |
| G36-OPERATIONAL-AND-004 | Missing | 36. Operational and production work | Persistent bans/admins. | parity:G36-OPERATIONAL-AND-004 |
| G36-OPERATIONAL-AND-005 | Missing | 36. Operational and production work | Server restart recovery. | parity:G36-OPERATIONAL-AND-005 |
| G36-OPERATIONAL-AND-006 | Missing | 36. Operational and production work | Graceful active-match handling during deployment. | parity:G36-OPERATIONAL-AND-006 |
| G36-OPERATIONAL-AND-007 | Missing | 36. Operational and production work | Horizontal scaling strategy. | parity:G36-OPERATIONAL-AND-007 |
| G36-OPERATIONAL-AND-008 | Missing | 36. Operational and production work | Room ownership across multiple server processes. | parity:G36-OPERATIONAL-AND-008 |
| G36-OPERATIONAL-AND-009 | Missing | 36. Operational and production work | Lobby service. | parity:G36-OPERATIONAL-AND-009 |
| G36-OPERATIONAL-AND-010 | Missing | 36. Operational and production work | Database. | parity:G36-OPERATIONAL-AND-010 |
| G36-OPERATIONAL-AND-011 | Missing | 36. Operational and production work | Authentication if accounts are added. | parity:G36-OPERATIONAL-AND-011 |
| G36-OPERATIONAL-AND-012 | Missing | 36. Operational and production work | Observability beyond three counters. | parity:G36-OPERATIONAL-AND-012 |
| G36-OPERATIONAL-AND-013 | Missing | 36. Operational and production work | Structured logs. | parity:G36-OPERATIONAL-AND-013 |
| G36-OPERATIONAL-AND-014 | Missing | 36. Operational and production work | Error tracking. | parity:G36-OPERATIONAL-AND-014 |
| G36-OPERATIONAL-AND-015 | Missing | 36. Operational and production work | Latency metrics. | parity:G36-OPERATIONAL-AND-015 |
| G36-OPERATIONAL-AND-016 | Missing | 36. Operational and production work | Tick-duration metrics. | parity:G36-OPERATIONAL-AND-016 |
| G36-OPERATIONAL-AND-017 | Missing | 36. Operational and production work | Connected-player metrics. | parity:G36-OPERATIONAL-AND-017 |
| G36-OPERATIONAL-AND-018 | Missing | 36. Operational and production work | Per-room metrics. | parity:G36-OPERATIONAL-AND-018 |
| G36-OPERATIONAL-AND-019 | Missing | 36. Operational and production work | Health versus readiness endpoints. | parity:G36-OPERATIONAL-AND-019 |
| G36-OPERATIONAL-AND-020 | Missing | 36. Operational and production work | Backups. | parity:G36-OPERATIONAL-AND-020 |
| G36-OPERATIONAL-AND-021 | Missing | 36. Operational and production work | Migration process. | parity:G36-OPERATIONAL-AND-021 |
| G36-OPERATIONAL-AND-022 | Missing | 36. Operational and production work | Load testing. | parity:G36-OPERATIONAL-AND-022 |
| G36-OPERATIONAL-AND-023 | Missing | 36. Operational and production work | Soak testing. | parity:G36-OPERATIONAL-AND-023 |
| G36-OPERATIONAL-AND-024 | Missing | 36. Operational and production work | Browser compatibility testing. | parity:G36-OPERATIONAL-AND-024 |
| G36-OPERATIONAL-AND-025 | Missing | 36. Operational and production work | Mobile-device testing. | parity:G36-OPERATIONAL-AND-025 |
| G36-OPERATIONAL-AND-026 | Missing | 36. Operational and production work | Touch-control usability testing. | parity:G36-OPERATIONAL-AND-026 |
| G36-OPERATIONAL-AND-027 | Missing | 36. Operational and production work | Accessibility testing. | parity:G36-OPERATIONAL-AND-027 |
| G36-OPERATIONAL-AND-028 | Missing | 36. Operational and production work | Security review. | parity:G36-OPERATIONAL-AND-028 |
| G36-OPERATIONAL-AND-029 | Missing | 36. Operational and production work | Dependency scanning. | parity:G36-OPERATIONAL-AND-029 |
| G36-OPERATIONAL-AND-030 | Missing | 36. Operational and production work | Deployment rollback testing. | parity:G36-OPERATIONAL-AND-030 |
| G36-OPERATIONAL-AND-031 | Missing | 36. Operational and production work | Asset CDN/caching. | parity:G36-OPERATIONAL-AND-031 |
| G36-OPERATIONAL-AND-032 | Missing | 36. Operational and production work | Static-asset compression. | parity:G36-OPERATIONAL-AND-032 |
| G36-OPERATIONAL-AND-033 | Missing | 36. Operational and production work | Privacy policy and moderation policy if publicly operated. | parity:G36-OPERATIONAL-AND-033 |
| G37-TESTING-REQUIRED-001 | Present | 37. Testing required for parity | Source-versus-Rust movement fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-002 | Present | 37. Testing required for parity | Jump fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-003 | Present | 37. Testing required for parity | Jet fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-004 | Present | 37. Testing required for parity | Crouch/prone/roll/backflip fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-005 | Present | 37. Testing required for parity | Polygon collision fixtures. | rust:collision_fixtures:geometry |
| G37-TESTING-REQUIRED-006 | Present | 37. Testing required for parity | One-way polygon fixtures. | rust:collision_fixtures:one-way |
| G37-TESTING-REQUIRED-007 | Missing | 37. Testing required for parity | Weapon fixture for every weapon and field. | parity:G37-TESTING-REQUIRED-007 |
| G37-TESTING-REQUIRED-008 | Missing | 37. Testing required for parity | Fire-rate fixtures. | parity:G37-TESTING-REQUIRED-008 |
| G37-TESTING-REQUIRED-009 | Missing | 37. Testing required for parity | Reload fixtures. | parity:G37-TESTING-REQUIRED-009 |
| G37-TESTING-REQUIRED-010 | Missing | 37. Testing required for parity | Startup fixtures. | parity:G37-TESTING-REQUIRED-010 |
| G37-TESTING-REQUIRED-011 | Missing | 37. Testing required for parity | Spread fixtures. | parity:G37-TESTING-REQUIRED-011 |
| G37-TESTING-REQUIRED-012 | Missing | 37. Testing required for parity | Recoil fixtures. | parity:G37-TESTING-REQUIRED-012 |
| G37-TESTING-REQUIRED-013 | Missing | 37. Testing required for parity | Bink fixtures. | parity:G37-TESTING-REQUIRED-013 |
| G37-TESTING-REQUIRED-014 | Missing | 37. Testing required for parity | Velocity-inheritance fixtures. | parity:G37-TESTING-REQUIRED-014 |
| G37-TESTING-REQUIRED-015 | Missing | 37. Testing required for parity | Head/chest/leg damage fixtures. | parity:G37-TESTING-REQUIRED-015 |
| G37-TESTING-REQUIRED-016 | Missing | 37. Testing required for parity | Explosion fixtures. | parity:G37-TESTING-REQUIRED-016 |
| G37-TESTING-REQUIRED-017 | Missing | 37. Testing required for parity | Grenade bounce/fuse fixtures. | parity:G37-TESTING-REQUIRED-017 |
| G37-TESTING-REQUIRED-018 | Missing | 37. Testing required for parity | Knife-throw fixtures. | parity:G37-TESTING-REQUIRED-018 |
| G37-TESTING-REQUIRED-019 | Missing | 37. Testing required for parity | Flame fixtures. | parity:G37-TESTING-REQUIRED-019 |
| G37-TESTING-REQUIRED-020 | Missing | 37. Testing required for parity | Flag interaction fixtures. | parity:G37-TESTING-REQUIRED-020 |
| G37-TESTING-REQUIRED-021 | Missing | 37. Testing required for parity | Bonus-kit fixtures. | parity:G37-TESTING-REQUIRED-021 |
| G37-TESTING-REQUIRED-022 | Missing | 37. Testing required for parity | Every mode’s scoring fixtures. | parity:G37-TESTING-REQUIRED-022 |
| G37-TESTING-REQUIRED-023 | Missing | 37. Testing required for parity | Round-limit fixtures. | parity:G37-TESTING-REQUIRED-023 |
| G37-TESTING-REQUIRED-024 | Missing | 37. Testing required for parity | Respawn fixtures. | parity:G37-TESTING-REQUIRED-024 |
| G37-TESTING-REQUIRED-025 | Missing | 37. Testing required for parity | Map-loading fixtures. | parity:G37-TESTING-REQUIRED-025 |
| G37-TESTING-REQUIRED-026 | Missing | 37. Testing required for parity | PMS parser fuzzing. | parity:G37-TESTING-REQUIRED-026 |
| G37-TESTING-REQUIRED-027 | Present | 37. Testing required for parity | Determinism across native Rust and Wasm. | parity:G37-TESTING-REQUIRED-027 |
| G37-TESTING-REQUIRED-028 | Missing | 37. Testing required for parity | Network reconciliation tests. | parity:G37-TESTING-REQUIRED-028 |
| G37-TESTING-REQUIRED-029 | Missing | 37. Testing required for parity | Two-player browser integration test. | parity:G37-TESTING-REQUIRED-029 |
| G37-TESTING-REQUIRED-030 | Missing | 37. Testing required for parity | Full 16-player test. | parity:G37-TESTING-REQUIRED-030 |
| G37-TESTING-REQUIRED-031 | Missing | 37. Testing required for parity | Reconnect test. | parity:G37-TESTING-REQUIRED-031 |
| G37-TESTING-REQUIRED-032 | Missing | 37. Testing required for parity | Server-restart test. | parity:G37-TESTING-REQUIRED-032 |
| G37-TESTING-REQUIRED-033 | Missing | 37. Testing required for parity | Mobile portrait test. | parity:G37-TESTING-REQUIRED-033 |
| G37-TESTING-REQUIRED-034 | Missing | 37. Testing required for parity | Mobile landscape test. | parity:G37-TESTING-REQUIRED-034 |
| G37-TESTING-REQUIRED-035 | Missing | 37. Testing required for parity | Low/high latency tests. | parity:G37-TESTING-REQUIRED-035 |
| G37-TESTING-REQUIRED-036 | Missing | 37. Testing required for parity | Packet-loss tests. | parity:G37-TESTING-REQUIRED-036 |
| G37-TESTING-REQUIRED-037 | Missing | 37. Testing required for parity | Long-running server soak test. | parity:G37-TESTING-REQUIRED-037 |
