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
| G01-CHARACTER-MOVEMENT-038 | Present | 1. Character movement and physics | Flag and kit push from bullets/explosions. | rust:mode_world:physics, rust:bonus_world:physics |
| G01-CHARACTER-MOVEMENT-039 | Present | 1. Character movement and physics | Movement animation state machine. | rust:movement_fixtures |
| G01-CHARACTER-MOVEMENT-040 | Present | 1. Character movement and physics | Directional aiming and body rotation. | parity:G01-CHARACTER-MOVEMENT-040, web:render:gostek |
| G01-CHARACTER-MOVEMENT-041 | Present | 1. Character movement and physics | Separate legs, torso, head, arms, weapon, and jet animations. | parity:G01-CHARACTER-MOVEMENT-041, web:render:gostek |
| G01-CHARACTER-MOVEMENT-042 | Present | 1. Character movement and physics | Death animations. | parity:G01-CHARACTER-MOVEMENT-042, web:render:gostek |
| G01-CHARACTER-MOVEMENT-043 | Present | 1. Character movement and physics | Mercy/victory/smoke/tobacco/helmet animations. | rust:commands:emote, web:render:gostek |
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
| G02-INPUT-AND-020 | Present | 2. Input and controls | Pick up flags and kits. | parity:G02-INPUT-AND-020, web:objectives:status |
| G02-INPUT-AND-021 | Present | 2. Input and controls | Flag throw using jump+crouch. | web:input:flag-throw |
| G02-INPUT-AND-022 | Present | 2. Input and controls | Dedicated configurable flag-throw key. | web:input:flag-throw |
| G02-INPUT-AND-023 | Present | 2. Input and controls | Respawn weapon-selection menu. | rust:damage_fixtures:weapon-choice |
| G02-INPUT-AND-024 | Present | 2. Input and controls | Separate primary and secondary selection controls. | web:input:weapon-selection |
| G02-INPUT-AND-025 | Present | 2. Input and controls | Team chat. | parity:G02-INPUT-AND-025, rust:chat:scope |
| G02-INPUT-AND-026 | Present | 2. Input and controls | Command console. | parity:G02-INPUT-AND-026, rust:commands:parse |
| G02-INPUT-AND-027 | Present | 2. Input and controls | Scoreboard hold/toggle behavior. | web:input:scoreboard |
| G02-INPUT-AND-028 | Present | 2. Input and controls | Weapon-statistics screen. | parity:G02-INPUT-AND-028, web:hud:screens |
| G02-INPUT-AND-029 | Present | 2. Input and controls | Minimap toggle. | parity:G02-INPUT-AND-029, web:hud:net |
| G02-INPUT-AND-030 | Present | 2. Input and controls | Sniper-line toggle. | parity:G02-INPUT-AND-030, web:hud:net |
| G02-INPUT-AND-031 | Present | 2. Input and controls | Performance-statistics overlay. | parity:G02-INPUT-AND-031, web:hud:net |
| G02-INPUT-AND-032 | Present | 2. Input and controls | Screenshot control. | web:input:screenshot |
| G02-INPUT-AND-033 | Present | 2. Input and controls | Music toggle. | parity:G02-INPUT-AND-033, web:audio:engine |
| G02-INPUT-AND-034 | Present | 2. Input and controls | Previous/next music track. | parity:G02-INPUT-AND-034, web:audio:engine |
| G02-INPUT-AND-035 | Present | 2. Input and controls | Demo recording. | web:replay:player, rust:replay:record |
| G02-INPUT-AND-036 | Present | 2. Input and controls | Demo playback fast-forward. | web:replay:player, rust:replay:seek |
| G02-INPUT-AND-037 | Present | 2. Input and controls | Pause. | parity:G02-INPUT-AND-037, rust:commands:parse |
| G02-INPUT-AND-038 | Present | 2. Input and controls | Window minimize shortcut. | web:input:minimize |
| G02-INPUT-AND-039 | Present | 2. Input and controls | Taunt shortcuts. | parity:G02-INPUT-AND-039, rust:commands:emote |
| G02-INPUT-AND-040 | Present | 2. Input and controls | Runtime mouse-sensitivity adjustment. | web:input:sensitivity |
| G02-INPUT-AND-041 | Present | 2. Input and controls | Runtime sound-volume adjustment. | parity:G02-INPUT-AND-041, web:audio:engine |
| G02-INPUT-AND-042 | Present | 2. Input and controls | Scoreboard scrolling. | web:input:scoreboard-scroll |
| G02-INPUT-AND-043 | Present | 2. Input and controls | Fully rebindable keyboard controls. | web:input:rebinding |
| G02-INPUT-AND-044 | Present | 2. Input and controls | Rebindable mouse buttons. | web:input:mouse-rebinding |
| G02-INPUT-AND-045 | Present | 2. Input and controls | Controller/gamepad support. | web:input:gamepad |
| G02-INPUT-AND-046 | Present | 2. Input and controls | Saved control profiles. | web:input:profiles |
| G02-INPUT-AND-047 | Present | 2. Input and controls | Accessibility alternatives for combined inputs. | web:input:accessibility |
| G02-INPUT-AND-048 | Present | 2. Input and controls | Mobile equivalents for crouch, prone, roll, reload, weapon switch, weapon throw, flag throw, scoreboard, and team chat. | web:mobile:movement-pad |
| G03-WEAPONS-001 | Present | 3. Weapons | Desert Eagles. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-002 | Present | 3. Weapons | HK MP5. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-003 | Present | 3. Weapons | AK-74. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-004 | Present | 3. Weapons | Steyr AUG. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-005 | Present | 3. Weapons | SPAS-12. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-006 | Present | 3. Weapons | Ruger 77. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-007 | Present | 3. Weapons | M79. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-008 | Present | 3. Weapons | Barrett M82A1. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-009 | Present | 3. Weapons | FN Minimi. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-010 | Present | 3. Weapons | XM214 Minigun. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-011 | Present | 3. Weapons | USSOCOM. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-012 | Present | 3. Weapons | Combat Knife. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-013 | Present | 3. Weapons | Chainsaw. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-014 | Present | 3. Weapons | M72 LAW. | rust:weapon_config_fixtures, rust:weapon_ballistics_fixtures:damage, web:weapons:table |
| G03-WEAPONS-015 | Present | 3. Weapons | Cluster grenades. | rust:obtainable_weapons, rust:weapon_family_fixtures:explosive |
| G03-WEAPONS-016 | Present | 3. Weapons | Flamethrower. | rust:obtainable_weapons, rust:weapon_family_fixtures:flame |
| G03-WEAPONS-017 | Present | 3. Weapons | Rambo Bow. | rust:obtainable_weapons, rust:modes:rambomatch |
| G03-WEAPONS-018 | Present | 3. Weapons | Normal arrows. | rust:obtainable_weapons |
| G03-WEAPONS-019 | Present | 3. Weapons | Flamed arrows. | rust:obtainable_weapons |
| G03-WEAPONS-020 | Present | 3. Weapons | Stationary M2 machine gun. | rust:obtainable_weapons, rust:weapon_family_fixtures:stationary |
| G03-WEAPONS-021 | Present | 3. Weapons | Punch/unarmed combat. | rust:obtainable_weapons |
| G03-WEAPONS-022 | Present | 3. Weapons | Primary-plus-secondary inventory slots. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-023 | Present | 3. Weapons | Carrying two primary weapons. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-024 | Present | 3. Weapons | Weapon pickups. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-025 | Present | 3. Weapons | Weapon dropping. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-026 | Present | 3. Weapons | Thrown-weapon physics. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-027 | Present | 3. Weapons | Thrown combat knife. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-028 | Present | 3. Weapons | Knife recovery/pickup. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-029 | Present | 3. Weapons | Manual reload. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-030 | Present | 3. Weapons | Reload interruption. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-031 | Present | 3. Weapons | Per-weapon reload animations. | web:render:gostek, rust:weapon_inventory_fixtures |
| G03-WEAPONS-032 | Present | 3. Weapons | Weapon-switch delays. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-033 | Present | 3. Weapons | Correct weapon startup behavior. | rust:game_core::tests::fire_is_rate_limited |
| G03-WEAPONS-034 | Present | 3. Weapons | LAW firing restrictions. | parity:G03-WEAPONS-034, rust:weapon_projectile_fixtures:firing-rules, rust:weapon_behavior_world:law |
| G03-WEAPONS-035 | Present | 3. Weapons | Barrett movement/startup restrictions. | parity:G03-WEAPONS-035, rust:weapon_projectile_fixtures:firing-rules |
| G03-WEAPONS-036 | Present | 3. Weapons | Minigun spin-up behavior. | parity:G03-WEAPONS-036, rust:weapon_projectile_fixtures:firing-rules, rust:weapon_behavior_world:minigun |
| G03-WEAPONS-037 | Present | 3. Weapons | Chainsaw continuous-contact behavior. | parity:G03-WEAPONS-037, rust:weapon_projectile_fixtures:firing-rules |
| G03-WEAPONS-038 | Present | 3. Weapons | Proper melee collision. | parity:G03-WEAPONS-038, rust:weapon_projectile_fixtures:firing-rules |
| G03-WEAPONS-039 | Present | 3. Weapons | Dual Desert Eagle projectiles and muzzle positions. | parity:G03-WEAPONS-039, rust:weapon_projectile_fixtures:firing-rules, rust:weapon_behavior_world:eagles |
| G03-WEAPONS-040 | Present | 3. Weapons | Proper shotgun pellet count and randomized spread. | parity:G03-WEAPONS-040, rust:weapon_projectile_fixtures:firing-rules, rust:weapon_behavior_world:shotgun |
| G03-WEAPONS-041 | Present | 3. Weapons | Projectile lifetime matching Soldat. | parity:G03-WEAPONS-041, rust:weapon_projectile_fixtures:styles |
| G03-WEAPONS-042 | Present | 3. Weapons | Projectile gravity per bullet style. | parity:G03-WEAPONS-042, rust:weapon_projectile_fixtures:styles, rust:weapon_behavior_world:grenade |
| G03-WEAPONS-043 | Present | 3. Weapons | Grenade bouncing. | parity:G03-WEAPONS-043, rust:weapon_projectile_fixtures:styles, rust:weapon_behavior_world:grenade |
| G03-WEAPONS-044 | Present | 3. Weapons | M79 projectile bouncing/impact behavior. | parity:G03-WEAPONS-044, rust:weapon_projectile_fixtures:styles |
| G03-WEAPONS-045 | Present | 3. Weapons | Grenade cooking/throw strength. | parity:G03-WEAPONS-045, rust:weapon_family_fixtures:explosive, rust:weapon_behavior_world:grenade |
| G03-WEAPONS-046 | Present | 3. Weapons | Grenade fuse timing. | parity:G03-WEAPONS-046, rust:weapon_projectile_fixtures:styles |
| G03-WEAPONS-047 | Present | 3. Weapons | Dropped grenade behavior on death. | parity:G03-WEAPONS-047, rust:weapon_behavior_world:grenade |
| G03-WEAPONS-048 | Present | 3. Weapons | Cluster grenade submunition spawning. | rust:weapon_family_fixtures:explosive, rust:obtainable_weapons |
| G03-WEAPONS-049 | Present | 3. Weapons | Arrow sticking/interaction. | rust:weapon_projectile_fixtures:styles |
| G03-WEAPONS-050 | Present | 3. Weapons | Flame propagation and burning. | rust:weapon_family_fixtures:flame |
| G03-WEAPONS-051 | Present | 3. Weapons | Flamethrower fuel/ammunition behavior. | rust:weapon_family_fixtures:flame, rust:obtainable_weapons |
| G03-WEAPONS-052 | Present | 3. Weapons | Stationary-gun mounting and dismounting. | rust:obtainable_weapons, rust:weapon_family_fixtures:stationary |
| G03-WEAPONS-053 | Present | 3. Weapons | Stationary-gun aiming limits. | rust:weapon_family_fixtures:stationary |
| G03-WEAPONS-054 | Present | 3. Weapons | Projectile-to-projectile or projectile-to-object interactions where applicable. | parity:G03-WEAPONS-054, rust:weapon_projectile_fixtures:styles, rust:weapon_behavior_world:objects |
| G03-WEAPONS-055 | Present | 3. Weapons | Muzzle origin based on character pose. | rust:weapon_inventory_fixtures |
| G03-WEAPONS-056 | Present | 3. Weapons | Muzzle flashes. | web:render:particles, rust:presentation:impact |
| G03-WEAPONS-057 | Present | 3. Weapons | Shell casings. | web:render:particles |
| G03-WEAPONS-058 | Present | 3. Weapons | Weapon-specific sounds. | web:audio:engine |
| G03-WEAPONS-059 | Present | 3. Weapons | Reload sounds. | web:audio:engine |
| G03-WEAPONS-060 | Present | 3. Weapons | Empty-magazine sound. | web:audio:engine |
| G03-WEAPONS-061 | Present | 3. Weapons | Bullet impact effects. | rust:presentation:impact, web:render:effects |
| G03-WEAPONS-062 | Present | 3. Weapons | Tracers matching weapon configuration. | web:render:effects |
| G03-WEAPONS-063 | Present | 3. Weapons | Explosion visual and audio effects. | rust:presentation:explosion, web:audio:engine |
| G03-WEAPONS-064 | Present | 3. Weapons | Weapon sprites held by characters. | web:render:gostek |
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
| G05-GAME-MODES-001 | Present | 5. Game modes | Deathmatch. | parity:G05-GAME-MODES-001, rust:match_world:lifecycle, rust:match_lifecycle:limits, web:smoke:scoreboard |
| G05-GAME-MODES-002 | Present | 5. Game modes | Team Deathmatch, corresponding to Soldat’s Teammatch. | parity:G05-GAME-MODES-002, rust:match_world:ledger, rust:match_world:teams, rust:match_lifecycle:teams |
| G05-GAME-MODES-003 | Present | 5. Game modes | Pointmatch. | parity:G05-GAME-MODES-003, rust:modes:pointmatch, rust:mode_world:pointmatch |
| G05-GAME-MODES-004 | Present | 5. Game modes | Rambomatch. | rust:modes:rambomatch, rust:obtainable_weapons |
| G05-GAME-MODES-005 | Present | 5. Game modes | Capture the Flag. | parity:G05-GAME-MODES-005, rust:modes:ctf, rust:mode_world:flags |
| G05-GAME-MODES-006 | Present | 5. Game modes | Infiltration. | parity:G05-GAME-MODES-006, rust:modes:infiltration |
| G05-GAME-MODES-007 | Present | 5. Game modes | Hold the Flag. | parity:G05-GAME-MODES-007, rust:modes:htf, rust:mode_world:flags |
| G05-GAME-MODES-008 | Present | 5. Game modes | Realistic mode. | parity:G05-GAME-MODES-008, rust:modifiers:realistic, rust:modifier_world:realistic |
| G05-GAME-MODES-009 | Present | 5. Game modes | Survival mode. | parity:G05-GAME-MODES-009, rust:modifiers:survival, rust:modifier_world:survival |
| G05-GAME-MODES-010 | Present | 5. Game modes | Advance mode. | parity:G05-GAME-MODES-010, rust:modifiers:advance, rust:modifier_world:advance |
| G05-GAME-MODES-011 | Present | 5. Game modes | Climb. | parity:G05-GAME-MODES-011, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-012 | Present | 5. Game modes | Dodgeball. | parity:G05-GAME-MODES-012, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-013 | Present | 5. Game modes | Domination. | parity:G05-GAME-MODES-013, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-014 | Present | 5. Game modes | Hide and Seek. | parity:G05-GAME-MODES-014, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-015 | Present | 5. Game modes | Knife Only. | parity:G05-GAME-MODES-015, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-016 | Present | 5. Game modes | OneShots. | parity:G05-GAME-MODES-016, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-017 | Present | 5. Game modes | Pirates vs Ninjas. | parity:G05-GAME-MODES-017, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-018 | Present | 5. Game modes | Realistic Soldat/Counter-Strike. | parity:G05-GAME-MODES-018, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-019 | Present | 5. Game modes | Trench Wars. | parity:G05-GAME-MODES-019, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-020 | Present | 5. Game modes | Tactical Trench Wars. | parity:G05-GAME-MODES-020, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-021 | Present | 5. Game modes | Zombie. | parity:G05-GAME-MODES-021, rust:modifiers:scripted, server:community-modes |
| G05-GAME-MODES-022 | Present | 5. Game modes | Kill limit. | parity:G05-GAME-MODES-022, rust:match_lifecycle:limits, rust:match_world:lifecycle |
| G05-GAME-MODES-023 | Present | 5. Game modes | Point limit. | parity:G05-GAME-MODES-023, rust:match_lifecycle:limits |
| G05-GAME-MODES-024 | Present | 5. Game modes | Capture limit. | parity:G05-GAME-MODES-024, rust:match_lifecycle:limits |
| G05-GAME-MODES-025 | Present | 5. Game modes | Time limit. | parity:G05-GAME-MODES-025, rust:match_lifecycle:limits, rust:match_world:lifecycle, web:match:clock |
| G05-GAME-MODES-026 | Present | 5. Game modes | Round start countdown. | parity:G05-GAME-MODES-026, rust:match_lifecycle:phases, rust:match_world:lifecycle, web:match:banner |
| G05-GAME-MODES-027 | Present | 5. Game modes | Round end. | parity:G05-GAME-MODES-027, rust:match_lifecycle:phases, rust:match_world:lifecycle |
| G05-GAME-MODES-028 | Present | 5. Game modes | Winner calculation. | parity:G05-GAME-MODES-028, rust:match_lifecycle:outcome, web:match:banner |
| G05-GAME-MODES-029 | Present | 5. Game modes | Draw handling. | parity:G05-GAME-MODES-029, rust:match_lifecycle:outcome, web:match:banner |
| G05-GAME-MODES-030 | Present | 5. Game modes | Overtime policy where appropriate. | parity:G05-GAME-MODES-030, rust:match_lifecycle:phases, rust:match_lifecycle:outcome |
| G05-GAME-MODES-031 | Present | 5. Game modes | Map rotation. | parity:G05-GAME-MODES-031, rust:match_lifecycle:rotation, server:map-rotation |
| G05-GAME-MODES-032 | Present | 5. Game modes | Map-loop option. | parity:G05-GAME-MODES-032, rust:match_lifecycle:rotation, server:map-rotation |
| G05-GAME-MODES-033 | Present | 5. Game modes | Next-map transition. | parity:G05-GAME-MODES-033, rust:match_lifecycle:phases, server:map-rotation, rust:match_world:lifecycle |
| G05-GAME-MODES-034 | Present | 5. Game modes | Match restart. | parity:G05-GAME-MODES-034, rust:match_world:restart, rust:match_lifecycle:rotation |
| G05-GAME-MODES-035 | Present | 5. Game modes | Configurable respawn time. | rust:damage_fixtures:respawn, rust:match_lifecycle:teams |
| G05-GAME-MODES-036 | Present | 5. Game modes | Survival round elimination. | parity:G05-GAME-MODES-036, rust:modifiers:survival, rust:modifier_world:survival |
| G05-GAME-MODES-037 | Present | 5. Game modes | Survival dead-player restrictions. | parity:G05-GAME-MODES-037, rust:modifiers:survival |
| G05-GAME-MODES-038 | Present | 5. Game modes | Advance-mode weapon unlocking. | parity:G05-GAME-MODES-038, rust:modifiers:advance, rust:modifier_world:advance |
| G05-GAME-MODES-039 | Present | 5. Game modes | Team balancing. | parity:G05-GAME-MODES-039, rust:match_lifecycle:teams, rust:match_world:teams |
| G05-GAME-MODES-040 | Present | 5. Game modes | Team selection. | parity:G05-GAME-MODES-040, rust:match_lifecycle:teams, rust:match_world:teams |
| G05-GAME-MODES-041 | Present | 5. Game modes | Spectator team. | parity:G05-GAME-MODES-041, rust:match_lifecycle:teams, rust:match_world:teams |
| G05-GAME-MODES-042 | Present | 5. Game modes | Mid-match spectator switching. | parity:G05-GAME-MODES-042, rust:match_world:teams |
| G05-GAME-MODES-043 | Present | 5. Game modes | Friendly-fire configuration. | parity:G05-GAME-MODES-043, rust:match_lifecycle:scoring, rust:weapon_ballistics_fixtures |
| G05-GAME-MODES-044 | Present | 5. Game modes | Teamkill handling. | parity:G05-GAME-MODES-044, rust:match_lifecycle:scoring, rust:match_world:ledger |
| G05-GAME-MODES-045 | Present | 5. Game modes | Suicide scoring. | parity:G05-GAME-MODES-045, rust:match_lifecycle:scoring, rust:match_world:ledger |
| G05-GAME-MODES-046 | Present | 5. Game modes | Disconnect/reconnect score preservation. | parity:G05-GAME-MODES-046, rust:match_lifecycle:scoring, rust:match_world:ledger |
| G05-GAME-MODES-047 | Present | 5. Game modes | Automatic round-end scoreboard. | parity:G05-GAME-MODES-047, rust:match_lifecycle:phases, rust:match_world:lifecycle |
| G05-GAME-MODES-048 | Present | 5. Game modes | Automatic final-score screenshot option. | parity:G05-GAME-MODES-048, rust:match_lifecycle:phases |
| G06-POINTMATCH-001 | Present | 6. Pointmatch | Yellow point flag. | parity:G06-POINTMATCH-001, rust:modes:pointmatch |
| G06-POINTMATCH-002 | Present | 6. Pointmatch | Holding the point flag. | parity:G06-POINTMATCH-002, rust:modes:pointmatch |
| G06-POINTMATCH-003 | Present | 6. Pointmatch | Extra points awarded while holding it. | parity:G06-POINTMATCH-003, rust:modes:pointmatch, rust:mode_world:pointmatch |
| G06-POINTMATCH-004 | Present | 6. Pointmatch | Flag drop on death. | parity:G06-POINTMATCH-004, rust:modes:pointmatch, rust:objective:flag-states |
| G06-POINTMATCH-005 | Present | 6. Pointmatch | Point-limit victory. | parity:G06-POINTMATCH-005, rust:modes:pointmatch, rust:match_lifecycle:limits |
| G06-POINTMATCH-006 | Present | 6. Pointmatch | Pointmatch-specific scoring. | parity:G06-POINTMATCH-006, rust:mode_world:pointmatch |
| G06-POINTMATCH-007 | Present | 6. Pointmatch | Pointmatch spawns and maps. | rust:obtainable_weapons, rust:modes:pointmatch |
| G06-POINTMATCH-008 | Present | 6. Pointmatch | Point-flag HUD status. | parity:G06-POINTMATCH-008, web:objectives:status, web:objectives:countdown |
| G07-RAMBOMATCH-001 | Present | 7. Rambomatch | Rambo Bow spawn. | rust:obtainable_weapons, rust:modes:rambomatch |
| G07-RAMBOMATCH-002 | Present | 7. Rambomatch | Bow pickup. | rust:obtainable_weapons |
| G07-RAMBOMATCH-003 | Present | 7. Rambomatch | Only the Rambo player earning kills/points under the mode’s rules. | rust:obtainable_weapons |
| G07-RAMBOMATCH-004 | Present | 7. Rambomatch | Rambo target indication. | web:hud:status |
| G07-RAMBOMATCH-005 | Present | 7. Rambomatch | Bow drop and reacquisition. | rust:obtainable_weapons |
| G07-RAMBOMATCH-006 | Present | 7. Rambomatch | Rambo-specific respawn behavior. | rust:obtainable_weapons |
| G07-RAMBOMATCH-007 | Present | 7. Rambomatch | Flamed-arrow support. | rust:obtainable_weapons |
| G07-RAMBOMATCH-008 | Present | 7. Rambomatch | Rambomatch scoring and win limit. | rust:obtainable_weapons, rust:modes:rambomatch |
| G07-RAMBOMATCH-009 | Present | 7. Rambomatch | Rambomatch HUD. | web:hud:status |
| G08-CAPTURE-THE-001 | Present | 8. Capture the Flag | Alpha and Bravo teams. | parity:G08-CAPTURE-THE-001, rust:modes:ctf, rust:match_world:teams |
| G08-CAPTURE-THE-002 | Present | 8. Capture the Flag | Red and blue flags. | parity:G08-CAPTURE-THE-002, rust:modes:ctf, rust:mode_world:flags |
| G08-CAPTURE-THE-003 | Present | 8. Capture the Flag | Flag bases. | parity:G08-CAPTURE-THE-003, rust:modes:ctf, rust:mode_world:flags |
| G08-CAPTURE-THE-004 | Present | 8. Capture the Flag | Enemy-flag pickup. | parity:G08-CAPTURE-THE-004, rust:modes:ctf, rust:mode_world:flags |
| G08-CAPTURE-THE-005 | Present | 8. Capture the Flag | Flag carrying. | parity:G08-CAPTURE-THE-005, rust:modes:ctf, rust:mode_world:flags |
| G08-CAPTURE-THE-006 | Present | 8. Capture the Flag | Flag dropping on death. | parity:G08-CAPTURE-THE-006, rust:modes:ctf, rust:mode_world:flags, rust:objective:flag-states |
| G08-CAPTURE-THE-007 | Present | 8. Capture the Flag | Manual flag throw. | parity:G08-CAPTURE-THE-007, rust:modes:ctf, rust:mode_world:flags |
| G08-CAPTURE-THE-008 | Present | 8. Capture the Flag | Flag return by touching a dropped friendly flag. | parity:G08-CAPTURE-THE-008, rust:modes:ctf |
| G08-CAPTURE-THE-009 | Present | 8. Capture the Flag | Automatic return timeout if applicable. | parity:G08-CAPTURE-THE-009, rust:objective:flag-states |
| G08-CAPTURE-THE-010 | Present | 8. Capture the Flag | Capture only when the player’s own flag is at base. | parity:G08-CAPTURE-THE-010, rust:modes:ctf, rust:mode_world:flags |
| G08-CAPTURE-THE-011 | Present | 8. Capture the Flag | Capture scoring. | parity:G08-CAPTURE-THE-011, rust:modes:ctf, rust:mode_world:flags |
| G08-CAPTURE-THE-012 | Present | 8. Capture the Flag | Capture limit. | parity:G08-CAPTURE-THE-012, rust:match_lifecycle:limits |
| G08-CAPTURE-THE-013 | Present | 8. Capture the Flag | Flag-carrier indicator. | parity:G08-CAPTURE-THE-013, web:objectives:status, web:objectives:countdown |
| G08-CAPTURE-THE-014 | Present | 8. Capture the Flag | Missing-flag indicator. | parity:G08-CAPTURE-THE-014, rust:modes:ctf, web:objectives:status, web:objectives:countdown |
| G08-CAPTURE-THE-015 | Present | 8. Capture the Flag | Flag status HUD. | parity:G08-CAPTURE-THE-015, web:objectives:status, web:objectives:countdown |
| G08-CAPTURE-THE-016 | Present | 8. Capture the Flag | Team score HUD. | parity:G08-CAPTURE-THE-016, web:match:banner, rust:match_world:ledger |
| G08-CAPTURE-THE-017 | Present | 8. Capture the Flag | Flag physics. | parity:G08-CAPTURE-THE-017, rust:objective:flag-physics, rust:mode_world:physics |
| G08-CAPTURE-THE-018 | Present | 8. Capture the Flag | Bullets and explosions pushing flags. | parity:G08-CAPTURE-THE-018, rust:mode_world:physics |
| G08-CAPTURE-THE-019 | Present | 8. Capture the Flag | Flag collision with polygons. | parity:G08-CAPTURE-THE-019, rust:objective:flag-physics |
| G08-CAPTURE-THE-020 | Present | 8. Capture the Flag | CTF spawn points. | rust:obtainable_weapons, rust:damage_fixtures:spawn-selection |
| G08-CAPTURE-THE-021 | Present | 8. Capture the Flag | CTF-compatible map validation. | parity:G08-CAPTURE-THE-021, rust:modes:ctf |
| G08-CAPTURE-THE-022 | Present | 8. Capture the Flag | CTF bots and flag objectives. | parity:G08-CAPTURE-THE-022, rust:bots:objectives, rust:bot_world:play |
| G09-INFILTRATION-001 | Present | 9. Infiltration | Attacking and defending teams. | parity:G09-INFILTRATION-001, rust:modes:infiltration |
| G09-INFILTRATION-002 | Present | 9. Infiltration | Black/white or objective-specific flags. | parity:G09-INFILTRATION-002, rust:modes:infiltration |
| G09-INFILTRATION-003 | Present | 9. Infiltration | Objective capture rules. | parity:G09-INFILTRATION-003, rust:modes:infiltration |
| G09-INFILTRATION-004 | Present | 9. Infiltration | Passive defender scoring. | parity:G09-INFILTRATION-004, rust:modes:infiltration |
| G09-INFILTRATION-005 | Present | 9. Infiltration | Attacker capture scoring. | parity:G09-INFILTRATION-005, rust:modes:infiltration |
| G09-INFILTRATION-006 | Present | 9. Infiltration | Team-role asymmetry. | parity:G09-INFILTRATION-006, rust:modes:infiltration |
| G09-INFILTRATION-007 | Present | 9. Infiltration | Infiltration-specific spawn points. | rust:obtainable_weapons, rust:damage_fixtures:spawn-selection |
| G09-INFILTRATION-008 | Present | 9. Infiltration | Infiltration timer and score rules. | parity:G09-INFILTRATION-008, rust:modes:infiltration |
| G09-INFILTRATION-009 | Present | 9. Infiltration | Team score HUD. | parity:G09-INFILTRATION-009, web:match:banner, rust:match_world:ledger |
| G09-INFILTRATION-010 | Present | 9. Infiltration | Objective state indicators. | parity:G09-INFILTRATION-010, web:objectives:status, web:objectives:countdown |
| G09-INFILTRATION-011 | Present | 9. Infiltration | Infiltration bot behavior. | parity:G09-INFILTRATION-011, rust:bots:objectives |
| G10-HOLD-THE-001 | Present | 10. Hold the Flag | Neutral yellow flag. | parity:G10-HOLD-THE-001, rust:modes:htf |
| G10-HOLD-THE-002 | Present | 10. Hold the Flag | Flag pickup and carrying. | parity:G10-HOLD-THE-002, rust:modes:htf, rust:mode_world:flags |
| G10-HOLD-THE-003 | Present | 10. Hold the Flag | Continuous team scoring while held. | parity:G10-HOLD-THE-003, rust:modes:htf, rust:mode_world:flags |
| G10-HOLD-THE-004 | Present | 10. Hold the Flag | Flag drops. | parity:G10-HOLD-THE-004, rust:modes:htf, rust:objective:flag-states |
| G10-HOLD-THE-005 | Present | 10. Hold the Flag | Flag return/reset rules. | parity:G10-HOLD-THE-005, rust:modes:htf, rust:objective:flag-states |
| G10-HOLD-THE-006 | Present | 10. Hold the Flag | Carrier indication. | parity:G10-HOLD-THE-006, web:objectives:status, web:objectives:countdown |
| G10-HOLD-THE-007 | Present | 10. Hold the Flag | HTF-specific spawn points. | rust:obtainable_weapons, rust:damage_fixtures:spawn-selection |
| G10-HOLD-THE-008 | Present | 10. Hold the Flag | HTF score display. | parity:G10-HOLD-THE-008, web:match:banner, rust:match_world:ledger |
| G10-HOLD-THE-009 | Present | 10. Hold the Flag | HTF bots and objective behavior. | parity:G10-HOLD-THE-009, rust:bots:objectives |
| G11-REALISTIC-MODE-001 | Present | 11. Realistic mode | Separate `weapons_realistic.ini` statistics. | parity:G11-REALISTIC-MODE-001, rust:modifiers:realistic, rust:modifier_world:realistic |
| G11-REALISTIC-MODE-002 | Present | 11. Realistic mode | Reduced/changed weapon damage behavior. | parity:G11-REALISTIC-MODE-002, rust:modifiers:realistic, rust:modifier_world:realistic, rust:weapon_config_fixtures |
| G11-REALISTIC-MODE-003 | Present | 11. Realistic mode | Recoil behavior appropriate to Realistic. | parity:G11-REALISTIC-MODE-003, rust:modifiers:realistic, rust:modifier_world:realistic |
| G11-REALISTIC-MODE-004 | Present | 11. Realistic mode | Visibility/line-of-sight restrictions. | parity:G11-REALISTIC-MODE-004, rust:modifiers:realistic, server:realistic-visibility |
| G11-REALISTIC-MODE-005 | Present | 11. Realistic mode | Enemies visible only when the observed player can see them. | parity:G11-REALISTIC-MODE-005, rust:modifiers:realistic, server:realistic-visibility |
| G11-REALISTIC-MODE-006 | Present | 11. Realistic mode | Dead-player and spectator visibility restrictions. | parity:G11-REALISTIC-MODE-006, rust:modifiers:realistic, server:realistic-visibility |
| G11-REALISTIC-MODE-007 | Present | 11. Realistic mode | Enemy team-chat visibility restrictions. | parity:G11-REALISTIC-MODE-007, rust:modifiers:realistic |
| G11-REALISTIC-MODE-008 | Present | 11. Realistic mode | Fall damage if required by the reference implementation. | parity:G11-REALISTIC-MODE-008, rust:modifiers:realistic |
| G11-REALISTIC-MODE-009 | Present | 11. Realistic mode | Realistic movement and survival tuning. | rust:realistic_movement, rust:modifiers:realistic |
| G11-REALISTIC-MODE-010 | Present | 11. Realistic mode | Realistic-specific HUD behavior. | parity:G11-REALISTIC-MODE-010, web:modifiers:badges |
| G11-REALISTIC-MODE-011 | Present | 11. Realistic mode | Server/room Realistic flag. | parity:G11-REALISTIC-MODE-011, server:modifier-rooms, web:modifiers:badges |
| G12-SURVIVAL-MODE-001 | Present | 12. Survival mode | No immediate respawn. | parity:G12-SURVIVAL-MODE-001, rust:modifiers:survival, rust:modifier_world:survival |
| G12-SURVIVAL-MODE-002 | Present | 12. Survival mode | Round-based respawning. | parity:G12-SURVIVAL-MODE-002, rust:modifiers:survival, rust:modifier_world:survival |
| G12-SURVIVAL-MODE-003 | Present | 12. Survival mode | Round begins when enough players are ready. | parity:G12-SURVIVAL-MODE-003, rust:modifiers:survival |
| G12-SURVIVAL-MODE-004 | Present | 12. Survival mode | Round ends when one player/team remains. | parity:G12-SURVIVAL-MODE-004, rust:modifiers:survival, rust:modifier_world:survival |
| G12-SURVIVAL-MODE-005 | Present | 12. Survival mode | Dead players spectate. | parity:G12-SURVIVAL-MODE-005, rust:modifiers:survival, web:modifiers:survival |
| G12-SURVIVAL-MODE-006 | Present | 12. Survival mode | Survival scoreboard. | parity:G12-SURVIVAL-MODE-006, rust:statistics:scoreboard, web:stats:scoreboard |
| G12-SURVIVAL-MODE-007 | Present | 12. Survival mode | End-of-round state. | parity:G12-SURVIVAL-MODE-007, rust:modifiers:survival, rust:modifier_world:survival |
| G12-SURVIVAL-MODE-008 | Present | 12. Survival mode | Flag restrictions after a Survival round ends. | parity:G12-SURVIVAL-MODE-008, rust:modifiers:survival |
| G12-SURVIVAL-MODE-009 | Present | 12. Survival mode | Survival chat/spectator restrictions. | parity:G12-SURVIVAL-MODE-009, rust:modifiers:survival |
| G12-SURVIVAL-MODE-010 | Present | 12. Survival mode | Configurable survival respawn/round behavior. | parity:G12-SURVIVAL-MODE-010, rust:modifiers:survival, server:modifier-rooms |
| G13-ADVANCE-MODE-001 | Present | 13. Advance mode | Initial limited weapon selection. | parity:G13-ADVANCE-MODE-001, rust:modifiers:advance, rust:modifier_world:advance |
| G13-ADVANCE-MODE-002 | Present | 13. Advance mode | Unlock weapons through kills. | parity:G13-ADVANCE-MODE-002, rust:modifiers:advance, rust:modifier_world:advance |
| G13-ADVANCE-MODE-003 | Present | 13. Advance mode | Unlock progression. | parity:G13-ADVANCE-MODE-003, rust:modifiers:advance, rust:modifier_world:advance |
| G13-ADVANCE-MODE-004 | Present | 13. Advance mode | Per-player unlock state. | parity:G13-ADVANCE-MODE-004, rust:modifiers:advance, rust:modifier_world:advance |
| G13-ADVANCE-MODE-005 | Present | 13. Advance mode | Advance weapon menu. | parity:G13-ADVANCE-MODE-005, web:modifiers:advance |
| G13-ADVANCE-MODE-006 | Present | 13. Advance mode | Reset progression between matches/maps as appropriate. | parity:G13-ADVANCE-MODE-006, rust:modifiers:advance |
| G13-ADVANCE-MODE-007 | Present | 13. Advance mode | Advance configuration. | parity:G13-ADVANCE-MODE-007, rust:modifiers:advance, server:modifier-rooms |
| G14-BONUS-KITS-001 | Present | 14. Bonus kits | Medic Kit: restore health to maximum. | parity:G14-BONUS-KITS-001, rust:bonus:effects, rust:bonus_world:pickup |
| G14-BONUS-KITS-002 | Present | 14. Bonus kits | Grenades Kit: restore grenades to configured maximum. | parity:G14-BONUS-KITS-002, rust:bonus:effects, rust:bonus_world:pickup |
| G14-BONUS-KITS-003 | Present | 14. Bonus kits | Cluster Grenades Kit: grant three cluster grenades. | parity:G14-BONUS-KITS-003, rust:bonus:effects, rust:bonus_world:pickup |
| G14-BONUS-KITS-004 | Present | 14. Bonus kits | Bulletproof Vest Kit: add approximately another full health bar as armor. | parity:G14-BONUS-KITS-004, rust:bonus:effects, rust:bonus_world:pickup, rust:bonus_world:effects |
| G14-BONUS-KITS-005 | Present | 14. Bonus kits | Flame God Kit: flamethrower plus temporary invulnerability. | parity:G14-BONUS-KITS-005, rust:bonus:effects, rust:bonus_world:effects |
| G14-BONUS-KITS-006 | Present | 14. Bonus kits | Berserker Kit: four-times weapon damage temporarily. | parity:G14-BONUS-KITS-006, rust:bonus:effects, rust:bonus_world:effects |
| G14-BONUS-KITS-007 | Present | 14. Bonus kits | Predator Kit: temporary invisibility. | parity:G14-BONUS-KITS-007, rust:bonus:effects, rust:bonus_world:effects |
| G14-BONUS-KITS-008 | Present | 14. Bonus kits | Bonus spawn points. | parity:G14-BONUS-KITS-008, rust:bonus:pickups, rust:bonus_world:pickup |
| G14-BONUS-KITS-009 | Present | 14. Bonus kits | Configurable kit frequency. | parity:G14-BONUS-KITS-009, rust:bonus:pickups, rust:bonus:settings |
| G14-BONUS-KITS-010 | Present | 14. Bonus kits | Kit respawn timers. | parity:G14-BONUS-KITS-010, rust:bonus:pickups |
| G14-BONUS-KITS-011 | Present | 14. Bonus kits | Pickup collision. | parity:G14-BONUS-KITS-011, rust:bonus:pickups, rust:bonus_world:pickup |
| G14-BONUS-KITS-012 | Present | 14. Bonus kits | Pickup sounds and effects. | web:audio:engine |
| G14-BONUS-KITS-013 | Present | 14. Bonus kits | Active-bonus HUD. | parity:G14-BONUS-KITS-013, web:bonuses:hud |
| G14-BONUS-KITS-014 | Present | 14. Bonus kits | Bonus countdown. | parity:G14-BONUS-KITS-014, web:bonuses:hud, rust:bonus:stacking |
| G14-BONUS-KITS-015 | Present | 14. Bonus kits | Bonus overlay/effect. | parity:G14-BONUS-KITS-015, web:bonuses:hud |
| G14-BONUS-KITS-016 | Present | 14. Bonus kits | Armor HUD. | parity:G14-BONUS-KITS-016, rust:bonus_world:effects, web:smoke:hud-health |
| G14-BONUS-KITS-017 | Present | 14. Bonus kits | Predator visibility affected by blood. | parity:G14-BONUS-KITS-017, rust:bonus:effects, web:bonuses:predator |
| G14-BONUS-KITS-018 | Present | 14. Bonus kits | Predator still producing audible sounds. | parity:G14-BONUS-KITS-018, rust:bonus:effects |
| G14-BONUS-KITS-019 | Present | 14. Bonus kits | Bonus expiration. | parity:G14-BONUS-KITS-019, rust:bonus:stacking, rust:bonus_world:effects |
| G14-BONUS-KITS-020 | Present | 14. Bonus kits | Bonus replacement/stacking rules. | parity:G14-BONUS-KITS-020, rust:bonus:stacking, rust:bonus_world:effects |
| G14-BONUS-KITS-021 | Present | 14. Bonus kits | Server enable/disable settings. | parity:G14-BONUS-KITS-021, rust:bonus:settings, server:bonus-settings |
| G14-BONUS-KITS-022 | Present | 14. Bonus kits | Kits affected by projectile push. | parity:G14-BONUS-KITS-022, rust:bonus:pickups, rust:bonus_world:physics |
| G15-HEALTH-ARMOR-001 | Present | 15. Health, armor, death, and respawn | Basic 100 HP. | rust:game_core::tests::m79_explosion_damages_nearby_enemy_not_distant_enemy |
| G15-HEALTH-ARMOR-002 | Present | 15. Health, armor, death, and respawn | Death count. | rust:game_core::tests::self_explosion_is_not_credited_as_a_kill |
| G15-HEALTH-ARMOR-003 | Present | 15. Health, armor, death, and respawn | Automatic respawn. | rust:game_core::tests::deterministic_replay |
| G15-HEALTH-ARMOR-004 | Present | 15. Health, armor, death, and respawn | Body-part damage. | rust:damage_fixtures:regions |
| G15-HEALTH-ARMOR-005 | Present | 15. Health, armor, death, and respawn | Bulletproof vest/armor. | parity:G15-HEALTH-ARMOR-005, rust:bonus:effects |
| G15-HEALTH-ARMOR-006 | Present | 15. Health, armor, death, and respawn | Bleeding. | rust:damage_fixtures:bleeding |
| G15-HEALTH-ARMOR-007 | Present | 15. Health, armor, death, and respawn | Blood particles. | web:render:particles |
| G15-HEALTH-ARMOR-008 | Present | 15. Health, armor, death, and respawn | Blood remaining on the character. | parity:G15-HEALTH-ARMOR-008, web:render:gostek |
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
| G15-HEALTH-ARMOR-026 | Present | 15. Health, armor, death, and respawn | Explosion deafness/whistling effect. | web:audio:engine |
| G16-MAPS-AND-001 | Present | 16. Maps and terrain engine | `.pms` map loader. | rust:content::pms |
| G16-MAPS-AND-002 | Present | 16. Maps and terrain engine | PMS format validation. | rust:content::pms-validation |
| G16-MAPS-AND-003 | Present | 16. Maps and terrain engine | Polygon geometry. | web:map-render:coordinates |
| G16-MAPS-AND-004 | Present | 16. Maps and terrain engine | Polygon types and properties. | rust:map_validation:materials |
| G16-MAPS-AND-005 | Present | 16. Maps and terrain engine | Texture coordinates. | task5:map-pipeline |
| G16-MAPS-AND-006 | Present | 16. Maps and terrain engine | Map textures. | task5:map-pipeline |
| G16-MAPS-AND-007 | Present | 16. Maps and terrain engine | Edge textures. | web:render:map |
| G16-MAPS-AND-008 | Present | 16. Maps and terrain engine | Scenery objects. | task5:map-pipeline |
| G16-MAPS-AND-009 | Present | 16. Maps and terrain engine | Scenery depth/layers. | task5:map-pipeline |
| G16-MAPS-AND-010 | Present | 16. Maps and terrain engine | Animated scenery if supported. | web:render:map |
| G16-MAPS-AND-011 | Present | 16. Maps and terrain engine | Colliders. | task5:map-pipeline |
| G16-MAPS-AND-012 | Present | 16. Maps and terrain engine | Spawn points. | task5:map-pipeline |
| G16-MAPS-AND-013 | Present | 16. Maps and terrain engine | Player spawn types. | task5:map-pipeline |
| G16-MAPS-AND-014 | Present | 16. Maps and terrain engine | Team spawn types. | task5:map-pipeline |
| G16-MAPS-AND-015 | Present | 16. Maps and terrain engine | Flag spawn types. | rust:obtainable_weapons |
| G16-MAPS-AND-016 | Present | 16. Maps and terrain engine | Bonus-kit spawn types. | rust:obtainable_weapons |
| G16-MAPS-AND-017 | Present | 16. Maps and terrain engine | Grenade spawn types where relevant. | rust:obtainable_weapons |
| G16-MAPS-AND-018 | Present | 16. Maps and terrain engine | Stationary-gun locations. | rust:obtainable_weapons |
| G16-MAPS-AND-019 | Present | 16. Maps and terrain engine | Bot waypoints. | task5:map-pipeline |
| G16-MAPS-AND-020 | Present | 16. Maps and terrain engine | Background colors and gradients. | task5:map-pipeline |
| G16-MAPS-AND-021 | Present | 16. Maps and terrain engine | Weather settings. | task5:map-pipeline |
| G16-MAPS-AND-022 | Present | 16. Maps and terrain engine | Footstep-sound property. | task5:map-pipeline |
| G16-MAPS-AND-023 | Present | 16. Maps and terrain engine | Map-specific jet fuel. | task5:map-pipeline |
| G16-MAPS-AND-024 | Present | 16. Maps and terrain engine | Map boundaries. | rust:map:bounds |
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
| G16-MAPS-AND-036 | Present | 16. Maps and terrain engine | Map voting or polling if the intended Soldat server experience includes it. | rust:admin:parse |
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
| G18-BOTS-AND-001 | Present | 18. Bots and AI | Bot entities driven by the authoritative simulation. | parity:G18-BOTS-AND-001, rust:bot_world:join, rust:bot_world:drive |
| G18-BOTS-AND-002 | Present | 18. Bots and AI | Random-bot count. | parity:G18-BOTS-AND-002, server:bots, web:bots:setup |
| G18-BOTS-AND-003 | Present | 18. Bots and AI | Per-team bot count. | parity:G18-BOTS-AND-003, rust:bots:profiles, rust:bot_world:join |
| G18-BOTS-AND-004 | Present | 18. Bots and AI | Bot difficulty. | parity:G18-BOTS-AND-004, rust:bots:profiles, server:bots, web:bots:setup |
| G18-BOTS-AND-005 | Present | 18. Bots and AI | Bot accuracy levels. | parity:G18-BOTS-AND-005, rust:bots:profiles, rust:bots:perception |
| G18-BOTS-AND-006 | Present | 18. Bots and AI | Bot reaction time. | parity:G18-BOTS-AND-006, rust:bots:profiles, rust:bots:perception |
| G18-BOTS-AND-007 | Present | 18. Bots and AI | Bot movement. | parity:G18-BOTS-AND-007, rust:bots:navigation, rust:bot_world:play |
| G18-BOTS-AND-008 | Present | 18. Bots and AI | Jet navigation. | parity:G18-BOTS-AND-008, rust:bots:navigation |
| G18-BOTS-AND-009 | Present | 18. Bots and AI | Crouching, prone, rolls, and backflips. | rust:bots:navigation |
| G18-BOTS-AND-010 | Present | 18. Bots and AI | Weapon selection. | parity:G18-BOTS-AND-010, rust:bots:combat |
| G18-BOTS-AND-011 | Present | 18. Bots and AI | Reloading. | parity:G18-BOTS-AND-011, rust:bots:combat |
| G18-BOTS-AND-012 | Present | 18. Bots and AI | Grenade use. | parity:G18-BOTS-AND-012, rust:bots:combat |
| G18-BOTS-AND-013 | Present | 18. Bots and AI | Weapon pickup. | rust:obtainable_weapons, rust:bots:objectives |
| G18-BOTS-AND-014 | Present | 18. Bots and AI | Bonus pickup. | rust:obtainable_weapons, rust:bots:objectives |
| G18-BOTS-AND-015 | Present | 18. Bots and AI | Waypoint navigation. | parity:G18-BOTS-AND-015, rust:bots:navigation |
| G18-BOTS-AND-016 | Present | 18. Bots and AI | Recovery when stuck. | parity:G18-BOTS-AND-016, rust:bots:navigation |
| G18-BOTS-AND-017 | Present | 18. Bots and AI | Deathmatch target selection. | parity:G18-BOTS-AND-017, rust:bots:perception, rust:bot_world:play |
| G18-BOTS-AND-018 | Present | 18. Bots and AI | Team coordination. | parity:G18-BOTS-AND-018, rust:bots:objectives, rust:bot_world:play |
| G18-BOTS-AND-019 | Present | 18. Bots and AI | CTF attacking, defending, returning, and capturing. | parity:G18-BOTS-AND-019, rust:bots:objectives, rust:bot_world:play |
| G18-BOTS-AND-020 | Present | 18. Bots and AI | Infiltration objectives. | parity:G18-BOTS-AND-020, rust:bots:objectives |
| G18-BOTS-AND-021 | Present | 18. Bots and AI | HTF carrier support. | parity:G18-BOTS-AND-021, rust:bots:objectives |
| G18-BOTS-AND-022 | Present | 18. Bots and AI | Rambomatch behavior. | rust:obtainable_weapons, rust:bots:objectives |
| G18-BOTS-AND-023 | Present | 18. Bots and AI | Survival behavior. | parity:G18-BOTS-AND-023, rust:bot_world:play, rust:modifier_world:survival |
| G18-BOTS-AND-024 | Present | 18. Bots and AI | Bot chat. | parity:G18-BOTS-AND-024, rust:bots:chat |
| G18-BOTS-AND-025 | Present | 18. Bots and AI | Custom bot profiles. | parity:G18-BOTS-AND-025, rust:bots:profiles |
| G18-BOTS-AND-026 | Present | 18. Bots and AI | Server commands to add/remove bots. | parity:G18-BOTS-AND-026, server:bots, rust:bot_world:join, web:bots:roster |
| G19-HUD-AND-001 | Present | 19. HUD and game screen | Numeric health. | web:smoke:hud-health |
| G19-HUD-AND-002 | Present | 19. HUD and game screen | Numeric jet fuel. | web:smoke:hud-jet |
| G19-HUD-AND-003 | Present | 19. HUD and game screen | Ammo and grenade count. | web:smoke:hud-ammo |
| G19-HUD-AND-004 | Present | 19. HUD and game screen | Kills/deaths. | web:smoke:hud-score |
| G19-HUD-AND-005 | Present | 19. HUD and game screen | Scoreboard. | web:smoke:scoreboard |
| G19-HUD-AND-006 | Present | 19. HUD and game screen | Team coloring. | parity:G19-HUD-AND-006, web:hud:gauges |
| G19-HUD-AND-007 | Present | 19. HUD and game screen | Red health bar. | parity:G19-HUD-AND-007, web:hud:gauges |
| G19-HUD-AND-008 | Present | 19. HUD and game screen | Yellow ammunition/reload bar. | parity:G19-HUD-AND-008, web:hud:gauges |
| G19-HUD-AND-009 | Present | 19. HUD and game screen | Bullet count positioned with the ammo display. | parity:G19-HUD-AND-009, web:hud:gauges |
| G19-HUD-AND-010 | Present | 19. HUD and game screen | Fire-interval bar. | parity:G19-HUD-AND-010, web:hud:gauges |
| G19-HUD-AND-011 | Present | 19. HUD and game screen | Blue jet-fuel bar. | parity:G19-HUD-AND-011, web:hud:gauges |
| G19-HUD-AND-012 | Present | 19. HUD and game screen | Server rank. | parity:G19-HUD-AND-012, web:hud:status |
| G19-HUD-AND-013 | Present | 19. HUD and game screen | Current kills/points. | parity:G19-HUD-AND-013, web:hud:status |
| G19-HUD-AND-014 | Present | 19. HUD and game screen | Difference from the leader. | parity:G19-HUD-AND-014, web:hud:status |
| G19-HUD-AND-015 | Present | 19. HUD and game screen | Kill/point/capture limit. | parity:G19-HUD-AND-015, web:hud:status |
| G19-HUD-AND-016 | Present | 19. HUD and game screen | Alpha team score. | parity:G19-HUD-AND-016, web:hud:status |
| G19-HUD-AND-017 | Present | 19. HUD and game screen | Bravo team score. | parity:G19-HUD-AND-017, web:hud:status |
| G19-HUD-AND-018 | Present | 19. HUD and game screen | Charlie team score. | parity:G19-HUD-AND-018, web:hud:status |
| G19-HUD-AND-019 | Present | 19. HUD and game screen | Delta team score. | parity:G19-HUD-AND-019, web:hud:status |
| G19-HUD-AND-020 | Present | 19. HUD and game screen | Missing-flag indicators. | parity:G19-HUD-AND-020, web:hud:status |
| G19-HUD-AND-021 | Present | 19. HUD and game screen | Flag-carrier state. | parity:G19-HUD-AND-021, web:hud:status |
| G19-HUD-AND-022 | Present | 19. HUD and game screen | Bonus status and duration. | parity:G19-HUD-AND-022, web:hud:status |
| G19-HUD-AND-023 | Present | 19. HUD and game screen | Armor indicator. | parity:G19-HUD-AND-023, web:hud:status |
| G19-HUD-AND-024 | Present | 19. HUD and game screen | Weapon image. | parity:G19-HUD-AND-024, web:hud:status |
| G19-HUD-AND-025 | Present | 19. HUD and game screen | Secondary weapon indicator. | parity:G19-HUD-AND-025, web:hud:status |
| G19-HUD-AND-026 | Present | 19. HUD and game screen | Grenade type/count. | parity:G19-HUD-AND-026, web:hud:status |
| G19-HUD-AND-027 | Present | 19. HUD and game screen | Reload progress rather than only “RELOADING.” | parity:G19-HUD-AND-027, web:hud:gauges |
| G19-HUD-AND-028 | Present | 19. HUD and game screen | Respawn countdown. | parity:G19-HUD-AND-028, web:hud:respawn |
| G19-HUD-AND-029 | Present | 19. HUD and game screen | Kill feed. | web:hud:kill-feed |
| G19-HUD-AND-030 | Present | 19. HUD and game screen | Chat overlay inside the game. | parity:G19-HUD-AND-030, web:hud:status |
| G19-HUD-AND-031 | Present | 19. HUD and game screen | Team-chat distinction. | parity:G19-HUD-AND-031, web:hud:status |
| G19-HUD-AND-032 | Present | 19. HUD and game screen | Server messages. | parity:G19-HUD-AND-032, web:hud:status |
| G19-HUD-AND-033 | Present | 19. HUD and game screen | Connection/ping indicator. | parity:G19-HUD-AND-033, web:hud:net |
| G19-HUD-AND-034 | Present | 19. HUD and game screen | Ping dot with size/color grading. | parity:G19-HUD-AND-034, web:hud:net |
| G19-HUD-AND-035 | Present | 19. HUD and game screen | FPS display. | parity:G19-HUD-AND-035, web:hud:net |
| G19-HUD-AND-036 | Present | 19. HUD and game screen | Network-bandwidth display. | parity:G19-HUD-AND-036, web:hud:net |
| G19-HUD-AND-037 | Present | 19. HUD and game screen | Minimap. | parity:G19-HUD-AND-037, web:hud:net |
| G19-HUD-AND-038 | Present | 19. HUD and game screen | Sniper line. | parity:G19-HUD-AND-038, web:hud:net |
| G19-HUD-AND-039 | Present | 19. HUD and game screen | Crosshair accuracy/bink visualization. | parity:G19-HUD-AND-039, web:hud:net |
| G19-HUD-AND-040 | Present | 19. HUD and game screen | Spectator HUD. | parity:G19-HUD-AND-040, web:hud:screens |
| G19-HUD-AND-041 | Present | 19. HUD and game screen | End-of-round screen. | parity:G19-HUD-AND-041, web:hud:screens |
| G19-HUD-AND-042 | Present | 19. HUD and game screen | Weapon-statistics screen. | parity:G19-HUD-AND-042, web:hud:screens |
| G19-HUD-AND-043 | Present | 19. HUD and game screen | Scrollable large scoreboard. | parity:G19-HUD-AND-043, web:hud:screens |
| G19-HUD-AND-044 | Present | 19. HUD and game screen | Player IDs on the command-enabled scoreboard. | parity:G19-HUD-AND-044, web:hud:screens |
| G19-HUD-AND-045 | Present | 19. HUD and game screen | Custom HUD/interface loading. | parity:G19-HUD-AND-045, web:hud:layout |
| G19-HUD-AND-046 | Present | 19. HUD and game screen | HUD scaling. | parity:G19-HUD-AND-046, web:hud:layout |
| G19-HUD-AND-047 | Present | 19. HUD and game screen | Safe-area and resolution tests across desktop and mobile. | parity:G19-HUD-AND-047, web:hud:layout, web:render:visual |
| G20-RENDERING-AND-001 | Present | 20. Rendering and visual effects | Full animated soldier/gostek rendering. | parity:G20-RENDERING-AND-001, web:render:gostek |
| G20-RENDERING-AND-002 | Present | 20. Rendering and visual effects | Separate body parts. | parity:G20-RENDERING-AND-002, web:render:gostek |
| G20-RENDERING-AND-003 | Present | 20. Rendering and visual effects | Hair. | parity:G20-RENDERING-AND-003, web:render:gostek |
| G20-RENDERING-AND-004 | Present | 20. Rendering and visual effects | Headgear. | parity:G20-RENDERING-AND-004, web:render:gostek |
| G20-RENDERING-AND-005 | Present | 20. Rendering and visual effects | Helmet/hat/none. | parity:G20-RENDERING-AND-005, web:render:gostek |
| G20-RENDERING-AND-006 | Present | 20. Rendering and visual effects | Chains and dog tags. | parity:G20-RENDERING-AND-006, web:render:gostek |
| G20-RENDERING-AND-007 | Present | 20. Rendering and visual effects | Shirt, pants, skin, hair, shoes, and jet colors. | parity:G20-RENDERING-AND-007, web:render:gostek |
| G20-RENDERING-AND-008 | Present | 20. Rendering and visual effects | Weapon sprites. | parity:G20-RENDERING-AND-008, web:render:gostek |
| G20-RENDERING-AND-009 | Present | 20. Rendering and visual effects | Character pose matching aim angle. | parity:G20-RENDERING-AND-009, web:render:gostek |
| G20-RENDERING-AND-010 | Present | 20. Rendering and visual effects | Muzzle flashes. | parity:G20-RENDERING-AND-010, web:render:particles |
| G20-RENDERING-AND-011 | Present | 20. Rendering and visual effects | Bullet trails. | parity:G20-RENDERING-AND-011, web:render:particles |
| G20-RENDERING-AND-012 | Present | 20. Rendering and visual effects | Shell casings. | parity:G20-RENDERING-AND-012, web:render:particles |
| G20-RENDERING-AND-013 | Present | 20. Rendering and visual effects | Sparks. | parity:G20-RENDERING-AND-013, web:render:particles, rust:presentation:impact |
| G20-RENDERING-AND-014 | Present | 20. Rendering and visual effects | Blood. | parity:G20-RENDERING-AND-014, web:render:particles |
| G20-RENDERING-AND-015 | Present | 20. Rendering and visual effects | Gore. | parity:G20-RENDERING-AND-015, web:render:particles |
| G20-RENDERING-AND-016 | Present | 20. Rendering and visual effects | Explosion animation. | parity:G20-RENDERING-AND-016, web:render:particles, rust:presentation:explosion, web:render:visual |
| G20-RENDERING-AND-017 | Present | 20. Rendering and visual effects | Smoke. | parity:G20-RENDERING-AND-017, web:render:particles |
| G20-RENDERING-AND-018 | Present | 20. Rendering and visual effects | Fire. | parity:G20-RENDERING-AND-018, web:render:particles |
| G20-RENDERING-AND-019 | Present | 20. Rendering and visual effects | Burning characters. | parity:G20-RENDERING-AND-019, web:render:objects |
| G20-RENDERING-AND-020 | Present | 20. Rendering and visual effects | Grenade sprites. | parity:G20-RENDERING-AND-020, web:render:objects |
| G20-RENDERING-AND-021 | Present | 20. Rendering and visual effects | Arrow sprites. | parity:G20-RENDERING-AND-021, web:render:objects |
| G20-RENDERING-AND-022 | Present | 20. Rendering and visual effects | Dropped weapons. | parity:G20-RENDERING-AND-022, web:render:objects |
| G20-RENDERING-AND-023 | Present | 20. Rendering and visual effects | Flags. | parity:G20-RENDERING-AND-023, web:render:objects |
| G20-RENDERING-AND-024 | Present | 20. Rendering and visual effects | Kits. | parity:G20-RENDERING-AND-024, web:render:objects |
| G20-RENDERING-AND-025 | Present | 20. Rendering and visual effects | Stationary gun. | parity:G20-RENDERING-AND-025, web:render:objects |
| G20-RENDERING-AND-026 | Present | 20. Rendering and visual effects | Polygon textures. | parity:G20-RENDERING-AND-026, web:render:map |
| G20-RENDERING-AND-027 | Present | 20. Rendering and visual effects | Edge textures. | parity:G20-RENDERING-AND-027, web:render:map |
| G20-RENDERING-AND-028 | Present | 20. Rendering and visual effects | Background scenery. | parity:G20-RENDERING-AND-028, web:render:map |
| G20-RENDERING-AND-029 | Present | 20. Rendering and visual effects | Foreground scenery. | parity:G20-RENDERING-AND-029, web:render:map |
| G20-RENDERING-AND-030 | Present | 20. Rendering and visual effects | Rain. | parity:G20-RENDERING-AND-030, web:render:weather |
| G20-RENDERING-AND-031 | Present | 20. Rendering and visual effects | Snow. | parity:G20-RENDERING-AND-031, web:render:weather |
| G20-RENDERING-AND-032 | Present | 20. Rendering and visual effects | Wind effects. | parity:G20-RENDERING-AND-032, web:render:weather |
| G20-RENDERING-AND-033 | Present | 20. Rendering and visual effects | Bullet-time visual effect. | parity:G20-RENDERING-AND-033, web:render:effects |
| G20-RENDERING-AND-034 | Present | 20. Rendering and visual effects | Predator transparency. | parity:G20-RENDERING-AND-034, web:render:effects |
| G20-RENDERING-AND-035 | Present | 20. Rendering and visual effects | Berserker overlay. | parity:G20-RENDERING-AND-035, web:render:effects |
| G20-RENDERING-AND-036 | Present | 20. Rendering and visual effects | Flame God overlay. | parity:G20-RENDERING-AND-036, web:render:effects |
| G20-RENDERING-AND-037 | Present | 20. Rendering and visual effects | Damage feedback. | parity:G20-RENDERING-AND-037, web:render:effects |
| G20-RENDERING-AND-038 | Present | 20. Rendering and visual effects | Screen shake where appropriate. | parity:G20-RENDERING-AND-038, web:render:effects |
| G20-RENDERING-AND-039 | Present | 20. Rendering and visual effects | Resolution scaling. | parity:G20-RENDERING-AND-039, web:render:effects |
| G20-RENDERING-AND-040 | Present | 20. Rendering and visual effects | Texture filtering settings. | parity:G20-RENDERING-AND-040, web:render:effects |
| G20-RENDERING-AND-041 | Present | 20. Rendering and visual effects | Mipmapping. | parity:G20-RENDERING-AND-041, web:render:effects |
| G20-RENDERING-AND-042 | Present | 20. Rendering and visual effects | Low-particle modes. | parity:G20-RENDERING-AND-042, web:render:effects |
| G20-RENDERING-AND-043 | Present | 20. Rendering and visual effects | Compatibility rendering path. | parity:G20-RENDERING-AND-043, web:render:effects, web:render:leak |
| G20-RENDERING-AND-044 | Present | 20. Rendering and visual effects | Custom-interface graphics. | parity:G20-RENDERING-AND-044, web:mods:package |
| G20-RENDERING-AND-045 | Present | 20. Rendering and visual effects | Mod-controlled asset scaling through `mod.ini`-like rules. | parity:G20-RENDERING-AND-045, web:mods:ini |
| G21-SOUND-AND-001 | Present | 21. Sound and music | Weapon-specific firing sounds. | parity:G21-SOUND-AND-001, web:audio:engine |
| G21-SOUND-AND-002 | Present | 21. Sound and music | Reload sounds. | parity:G21-SOUND-AND-002, web:audio:engine |
| G21-SOUND-AND-003 | Present | 21. Sound and music | Empty-weapon sounds. | parity:G21-SOUND-AND-003, web:audio:engine |
| G21-SOUND-AND-004 | Present | 21. Sound and music | Grenade pin/throw/bounce/explosion sounds. | parity:G21-SOUND-AND-004, web:audio:engine |
| G21-SOUND-AND-005 | Present | 21. Sound and music | M79 and LAW explosion sounds. | parity:G21-SOUND-AND-005, web:audio:engine |
| G21-SOUND-AND-006 | Present | 21. Sound and music | Bullet impacts. | parity:G21-SOUND-AND-006, web:audio:engine |
| G21-SOUND-AND-007 | Present | 21. Sound and music | Ricochets if applicable. | parity:G21-SOUND-AND-007, web:audio:engine |
| G21-SOUND-AND-008 | Present | 21. Sound and music | Chainsaw loop. | parity:G21-SOUND-AND-008, web:audio:engine |
| G21-SOUND-AND-009 | Present | 21. Sound and music | Knife sounds. | parity:G21-SOUND-AND-009, web:audio:engine |
| G21-SOUND-AND-010 | Present | 21. Sound and music | Punch sounds. | parity:G21-SOUND-AND-010, web:audio:engine |
| G21-SOUND-AND-011 | Present | 21. Sound and music | Flamethrower loop. | parity:G21-SOUND-AND-011, web:audio:engine |
| G21-SOUND-AND-012 | Present | 21. Sound and music | Character pain. | parity:G21-SOUND-AND-012, web:audio:engine |
| G21-SOUND-AND-013 | Present | 21. Sound and music | Death sounds. | parity:G21-SOUND-AND-013, web:audio:engine |
| G21-SOUND-AND-014 | Present | 21. Sound and music | Gore sounds. | parity:G21-SOUND-AND-014, web:audio:engine |
| G21-SOUND-AND-015 | Present | 21. Sound and music | Footsteps selected by map property. | parity:G21-SOUND-AND-015, web:audio:engine |
| G21-SOUND-AND-016 | Present | 21. Sound and music | Jetpack sound. | parity:G21-SOUND-AND-016, web:audio:engine |
| G21-SOUND-AND-017 | Present | 21. Sound and music | Flag pickup/drop/return/capture sounds. | parity:G21-SOUND-AND-017, web:audio:engine |
| G21-SOUND-AND-018 | Present | 21. Sound and music | Kit pickup sounds. | parity:G21-SOUND-AND-018, web:audio:engine |
| G21-SOUND-AND-019 | Present | 21. Sound and music | Bonus activation/expiration sounds. | parity:G21-SOUND-AND-019, web:audio:engine |
| G21-SOUND-AND-020 | Present | 21. Sound and music | UI/menu sounds. | parity:G21-SOUND-AND-020, web:audio:engine |
| G21-SOUND-AND-021 | Present | 21. Sound and music | Chat/message notification. | parity:G21-SOUND-AND-021, web:audio:engine |
| G21-SOUND-AND-022 | Present | 21. Sound and music | Distant-battle sounds. | parity:G21-SOUND-AND-022, web:audio:engine |
| G21-SOUND-AND-023 | Present | 21. Sound and music | Weather sounds. | parity:G21-SOUND-AND-023, web:audio:engine |
| G21-SOUND-AND-024 | Present | 21. Sound and music | Explosion deafness and whistle effect. | parity:G21-SOUND-AND-024, web:audio:engine |
| G21-SOUND-AND-025 | Present | 21. Sound and music | Positional audio. | parity:G21-SOUND-AND-025, web:audio:engine |
| G21-SOUND-AND-026 | Present | 21. Sound and music | Distance attenuation. | parity:G21-SOUND-AND-026, web:audio:engine |
| G21-SOUND-AND-027 | Present | 21. Sound and music | Master sound volume. | parity:G21-SOUND-AND-027, web:audio:engine |
| G21-SOUND-AND-028 | Present | 21. Sound and music | Music volume. | parity:G21-SOUND-AND-028, web:audio:engine |
| G21-SOUND-AND-029 | Present | 21. Sound and music | Music playback. | parity:G21-SOUND-AND-029, web:audio:engine |
| G21-SOUND-AND-030 | Present | 21. Sound and music | Toggle music. | parity:G21-SOUND-AND-030, web:audio:engine |
| G21-SOUND-AND-031 | Present | 21. Sound and music | Previous/next track. | parity:G21-SOUND-AND-031, web:audio:engine |
| G21-SOUND-AND-032 | Present | 21. Sound and music | Sound-quality option. | parity:G21-SOUND-AND-032, web:audio:engine |
| G21-SOUND-AND-033 | Present | 21. Sound and music | Sound-output/device option where browser APIs permit it. | parity:G21-SOUND-AND-033, web:audio:engine |
| G22-PLAYER-PROFILES-001 | Present | 22. Player profiles and customization | Persistent player profiles. | parity:G22-PLAYER-PROFILES-001, web:profiles:player |
| G22-PLAYER-PROFILES-002 | Present | 22. Player profiles and customization | Multiple profiles. | parity:G22-PLAYER-PROFILES-002, web:profiles:player |
| G22-PLAYER-PROFILES-003 | Present | 22. Player profiles and customization | Profile selection screen. | parity:G22-PLAYER-PROFILES-003, web:profiles:player |
| G22-PLAYER-PROFILES-004 | Present | 22. Player profiles and customization | Per-profile settings. | parity:G22-PLAYER-PROFILES-004, web:profiles:player |
| G22-PLAYER-PROFILES-005 | Present | 22. Player profiles and customization | Per-profile controls. | parity:G22-PLAYER-PROFILES-005, web:profiles:player |
| G22-PLAYER-PROFILES-006 | Present | 22. Player profiles and customization | Per-profile taunts. | parity:G22-PLAYER-PROFILES-006, web:profiles:player |
| G22-PLAYER-PROFILES-007 | Present | 22. Player profiles and customization | Default secondary weapon. | parity:G22-PLAYER-PROFILES-007, web:profiles:player |
| G22-PLAYER-PROFILES-008 | Present | 22. Player profiles and customization | Player name constraints matching the intended rules. | parity:G22-PLAYER-PROFILES-008, web:profiles:player |
| G22-PLAYER-PROFILES-009 | Present | 22. Player profiles and customization | Shirt color. | parity:G22-PLAYER-PROFILES-009, web:profiles:player |
| G22-PLAYER-PROFILES-010 | Present | 22. Player profiles and customization | Pants color. | parity:G22-PLAYER-PROFILES-010, web:profiles:player |
| G22-PLAYER-PROFILES-011 | Present | 22. Player profiles and customization | Skin color. | parity:G22-PLAYER-PROFILES-011, web:profiles:player |
| G22-PLAYER-PROFILES-012 | Present | 22. Player profiles and customization | Hair color. | parity:G22-PLAYER-PROFILES-012, web:profiles:player |
| G22-PLAYER-PROFILES-013 | Present | 22. Player profiles and customization | Shoe color. | parity:G22-PLAYER-PROFILES-013, web:profiles:player |
| G22-PLAYER-PROFILES-014 | Present | 22. Player profiles and customization | Jet-flame color. | parity:G22-PLAYER-PROFILES-014, web:profiles:player |
| G22-PLAYER-PROFILES-015 | Present | 22. Player profiles and customization | Hairstyle. | parity:G22-PLAYER-PROFILES-015, web:profiles:player |
| G22-PLAYER-PROFILES-016 | Present | 22. Player profiles and customization | Headgear. | parity:G22-PLAYER-PROFILES-016, web:profiles:player |
| G22-PLAYER-PROFILES-017 | Present | 22. Player profiles and customization | Chain style. | parity:G22-PLAYER-PROFILES-017, web:profiles:player |
| G22-PLAYER-PROFILES-018 | Present | 22. Player profiles and customization | Interface selection. | parity:G22-PLAYER-PROFILES-018, web:profiles:player |
| G22-PLAYER-PROFILES-019 | Present | 22. Player profiles and customization | Saved mouse sensitivity. | parity:G22-PLAYER-PROFILES-019, web:profiles:player |
| G22-PLAYER-PROFILES-020 | Present | 22. Player profiles and customization | Saved sound/music volume. | parity:G22-PLAYER-PROFILES-020, web:profiles:player |
| G22-PLAYER-PROFILES-021 | Present | 22. Player profiles and customization | Saved graphics settings. | parity:G22-PLAYER-PROFILES-021, web:profiles:player |
| G22-PLAYER-PROFILES-022 | Present | 22. Player profiles and customization | Saved favorite servers. | parity:G22-PLAYER-PROFILES-022, web:profiles:player |
| G22-PLAYER-PROFILES-023 | Present | 22. Player profiles and customization | Profile import/export if desired. | parity:G22-PLAYER-PROFILES-023, web:profiles:player |
| G22-PLAYER-PROFILES-024 | Missing | 22. Player profiles and customization | Account-backed persistence if the game moves beyond local profiles. | parity:G22-PLAYER-PROFILES-024 |
| G23-CHAT-AND-001 | Present | 23. Chat and taunts | Basic room chat. | server:chat-handler |
| G23-CHAT-AND-002 | Present | 23. Chat and taunts | Basic chat rate limiting. | server:chat-rate-limit |
| G23-CHAT-AND-003 | Present | 23. Chat and taunts | Team chat. | parity:G23-CHAT-AND-003, rust:chat:visibility |
| G23-CHAT-AND-004 | Present | 23. Chat and taunts | `^` shorthand for team chat. | parity:G23-CHAT-AND-004, rust:chat:visibility |
| G23-CHAT-AND-005 | Present | 23. Chat and taunts | In-game chat overlay. | parity:G23-CHAT-AND-005, rust:chat:visibility |
| G23-CHAT-AND-006 | Present | 23. Chat and taunts | Chat while actively playing without interfering with controls. | parity:G23-CHAT-AND-006, rust:chat:visibility |
| G23-CHAT-AND-007 | Present | 23. Chat and taunts | Taunt file/configuration. | parity:G23-CHAT-AND-007, rust:chat:visibility |
| G23-CHAT-AND-008 | Present | 23. Chat and taunts | Alt+letter and Alt+number taunts. | parity:G23-CHAT-AND-008, rust:chat:visibility |
| G23-CHAT-AND-009 | Present | 23. Chat and taunts | Per-profile taunts. | parity:G23-CHAT-AND-009, rust:chat:visibility |
| G23-CHAT-AND-010 | Present | 23. Chat and taunts | Command taunts. | parity:G23-CHAT-AND-010, rust:chat:visibility |
| G23-CHAT-AND-011 | Present | 23. Chat and taunts | Chat mute. | parity:G23-CHAT-AND-011, rust:chat:visibility |
| G23-CHAT-AND-012 | Present | 23. Chat and taunts | Mute by player name. | parity:G23-CHAT-AND-012, rust:chat:visibility |
| G23-CHAT-AND-013 | Present | 23. Chat and taunts | Mute by player ID. | parity:G23-CHAT-AND-013, rust:chat:visibility |
| G23-CHAT-AND-014 | Present | 23. Chat and taunts | Spam/flood controls beyond the basic one-second limit. | parity:G23-CHAT-AND-014, rust:chat:visibility |
| G23-CHAT-AND-015 | Present | 23. Chat and taunts | Profanity/moderation options if required. | rust:chat:censor |
| G23-CHAT-AND-016 | Present | 23. Chat and taunts | Server announcements. | parity:G23-CHAT-AND-016, rust:chat:visibility |
| G23-CHAT-AND-017 | Present | 23. Chat and taunts | Join/leave messages. | parity:G23-CHAT-AND-017, rust:chat:visibility |
| G23-CHAT-AND-018 | Present | 23. Chat and taunts | Kill/capture announcements. | parity:G23-CHAT-AND-018, rust:chat:visibility |
| G23-CHAT-AND-019 | Present | 23. Chat and taunts | Realistic/Survival chat-visibility rules. | parity:G23-CHAT-AND-019, rust:chat:visibility |
| G24-PLAYER-COMMANDS-001 | Present | 24. Player commands | `/KILL` | parity:G24-PLAYER-COMMANDS-001, rust:commands:parse |
| G24-PLAYER-COMMANDS-002 | Present | 24. Player commands | `/BRUTALKILL` | parity:G24-PLAYER-COMMANDS-002, rust:commands:parse |
| G24-PLAYER-COMMANDS-003 | Present | 24. Player commands | `/MERCY` | parity:G24-PLAYER-COMMANDS-003, rust:commands:parse |
| G24-PLAYER-COMMANDS-004 | Present | 24. Player commands | `/SMOKE` | parity:G24-PLAYER-COMMANDS-004, rust:commands:parse |
| G24-PLAYER-COMMANDS-005 | Present | 24. Player commands | `/TABAC` | parity:G24-PLAYER-COMMANDS-005, rust:commands:parse |
| G24-PLAYER-COMMANDS-006 | Present | 24. Player commands | `/TAKEOFF` | parity:G24-PLAYER-COMMANDS-006, rust:commands:parse |
| G24-PLAYER-COMMANDS-007 | Present | 24. Player commands | `/VICTORY` | parity:G24-PLAYER-COMMANDS-007, rust:commands:parse |
| G24-PLAYER-COMMANDS-008 | Present | 24. Player commands | `/PAUSE` | parity:G24-PLAYER-COMMANDS-008, rust:commands:parse |
| G24-PLAYER-COMMANDS-009 | Present | 24. Player commands | `/UNPAUSE` | parity:G24-PLAYER-COMMANDS-009, rust:commands:parse |
| G25-SERVER-AND-001 | Present | 25. Server and administrator commands | `/ADDMAP <map>` | parity:G25-SERVER-AND-001, rust:admin:parse |
| G25-SERVER-AND-002 | Present | 25. Server and administrator commands | `/DELMAP <map>` | parity:G25-SERVER-AND-002, rust:admin:parse |
| G25-SERVER-AND-003 | Present | 25. Server and administrator commands | `/ADDBOT<team> <bot>` | parity:G25-SERVER-AND-003, rust:admin:parse |
| G25-SERVER-AND-004 | Present | 25. Server and administrator commands | `/KICK <player or ID>` | parity:G25-SERVER-AND-004, rust:admin:parse |
| G25-SERVER-AND-005 | Present | 25. Server and administrator commands | `/KICKLAST` | parity:G25-SERVER-AND-005, rust:admin:parse |
| G25-SERVER-AND-006 | Present | 25. Server and administrator commands | `/TEMPBAN <minutes> <IP/player>` | parity:G25-SERVER-AND-006, rust:admin:parse |
| G25-SERVER-AND-007 | Present | 25. Server and administrator commands | `/BAN <player or ID>` | parity:G25-SERVER-AND-007, rust:admin:parse |
| G25-SERVER-AND-008 | Present | 25. Server and administrator commands | `/BANIP <IP>` | parity:G25-SERVER-AND-008, rust:admin:parse |
| G25-SERVER-AND-009 | Present | 25. Server and administrator commands | `/UNBAN <IP>` | parity:G25-SERVER-AND-009, rust:admin:parse |
| G25-SERVER-AND-010 | Present | 25. Server and administrator commands | `/MAP <map>` | parity:G25-SERVER-AND-010, rust:admin:parse |
| G25-SERVER-AND-011 | Present | 25. Server and administrator commands | `/RESTART` | parity:G25-SERVER-AND-011, rust:admin:parse |
| G25-SERVER-AND-012 | Present | 25. Server and administrator commands | `/NEXTMAP` | parity:G25-SERVER-AND-012, rust:admin:parse |
| G25-SERVER-AND-013 | Present | 25. Server and administrator commands | `/ADM <player>` | parity:G25-SERVER-AND-013, rust:admin:parse |
| G25-SERVER-AND-014 | Present | 25. Server and administrator commands | `/ADMIP <IP>` | parity:G25-SERVER-AND-014, rust:admin:parse |
| G25-SERVER-AND-015 | Present | 25. Server and administrator commands | `/UNADM <IP>` | parity:G25-SERVER-AND-015, rust:admin:parse |
| G25-SERVER-AND-016 | Present | 25. Server and administrator commands | `/RESPAWNTIME <seconds>` | parity:G25-SERVER-AND-016, rust:admin:parse |
| G25-SERVER-AND-017 | Present | 25. Server and administrator commands | Admin authentication. | parity:G25-SERVER-AND-017, rust:admin:parse |
| G25-SERVER-AND-018 | Present | 25. Server and administrator commands | Remote-admin support. | parity:G25-SERVER-AND-018, rust:admin:parse |
| G25-SERVER-AND-019 | Present | 25. Server and administrator commands | Persistent ban list. | parity:G25-SERVER-AND-019, rust:admin:parse |
| G25-SERVER-AND-020 | Present | 25. Server and administrator commands | Persistent admin list. | parity:G25-SERVER-AND-020, rust:admin:parse |
| G25-SERVER-AND-021 | Present | 25. Server and administrator commands | Audit log. | parity:G25-SERVER-AND-021, rust:admin:parse |
| G25-SERVER-AND-022 | Present | 25. Server and administrator commands | Command authorization. | parity:G25-SERVER-AND-022, rust:admin:parse |
| G25-SERVER-AND-023 | Present | 25. Server and administrator commands | Player-ID display. | parity:G25-SERVER-AND-023, rust:admin:parse |
| G25-SERVER-AND-024 | Present | 25. Server and administrator commands | Safe command parsing. | parity:G25-SERVER-AND-024, rust:admin:parse |
| G25-SERVER-AND-025 | Present | 25. Server and administrator commands | Command feedback and errors. | parity:G25-SERVER-AND-025, rust:admin:parse |
| G25-SERVER-AND-026 | Present | 25. Server and administrator commands | Map-list loading. | parity:G25-SERVER-AND-026, rust:admin:parse |
| G25-SERVER-AND-027 | Present | 25. Server and administrator commands | Server configuration reload. | parity:G25-SERVER-AND-027, rust:admin:parse |
| G25-SERVER-AND-028 | Present | 25. Server and administrator commands | Lobby re-registration where applicable. | parity:G25-SERVER-AND-028, rust:admin:parse |
| G25-SERVER-AND-029 | Present | 25. Server and administrator commands | Password changes. | parity:G25-SERVER-AND-029, rust:admin:parse |
| G25-SERVER-AND-030 | Present | 25. Server and administrator commands | Maximum-player changes. | parity:G25-SERVER-AND-030, rust:admin:parse |
| G25-SERVER-AND-031 | Present | 25. Server and administrator commands | Locked mode preventing sensitive runtime changes. | parity:G25-SERVER-AND-031, rust:admin:parse |
| G26-MATCH-AND-001 | Present | 26. Match and server settings | Game mode. | parity:G26-MATCH-AND-001, rust:config:validate |
| G26-MATCH-AND-002 | Present | 26. Match and server settings | Kill/point limit. | parity:G26-MATCH-AND-002, rust:config:validate |
| G26-MATCH-AND-003 | Present | 26. Match and server settings | Capture limit. | parity:G26-MATCH-AND-003, rust:config:validate |
| G26-MATCH-AND-004 | Present | 26. Match and server settings | Time limit. | parity:G26-MATCH-AND-004, rust:config:validate |
| G26-MATCH-AND-005 | Present | 26. Match and server settings | Survival toggle. | parity:G26-MATCH-AND-005, rust:config:validate |
| G26-MATCH-AND-006 | Present | 26. Match and server settings | Realistic toggle. | parity:G26-MATCH-AND-006, rust:config:validate |
| G26-MATCH-AND-007 | Present | 26. Match and server settings | Advance toggle. | parity:G26-MATCH-AND-007, rust:config:validate |
| G26-MATCH-AND-008 | Present | 26. Match and server settings | Map-list looping. | parity:G26-MATCH-AND-008, rust:config:validate |
| G26-MATCH-AND-009 | Present | 26. Match and server settings | Random bots. | parity:G26-MATCH-AND-009, rust:config:validate |
| G26-MATCH-AND-010 | Present | 26. Match and server settings | Team-specific bots. | parity:G26-MATCH-AND-010, rust:config:validate |
| G26-MATCH-AND-011 | Present | 26. Match and server settings | Server name. | parity:G26-MATCH-AND-011, rust:config:validate |
| G26-MATCH-AND-012 | Present | 26. Match and server settings | Server password. | parity:G26-MATCH-AND-012, rust:config:validate |
| G26-MATCH-AND-013 | Present | 26. Match and server settings | Maximum players. | parity:G26-MATCH-AND-013, rust:config:validate |
| G26-MATCH-AND-014 | Present | 26. Match and server settings | Game port. | parity:G26-MATCH-AND-014, rust:config:validate |
| G26-MATCH-AND-015 | Present | 26. Match and server settings | Public/private listing. | parity:G26-MATCH-AND-015, rust:config:validate |
| G26-MATCH-AND-016 | Present | 26. Match and server settings | Team balancing. | parity:G26-MATCH-AND-016, rust:config:validate |
| G26-MATCH-AND-017 | Present | 26. Match and server settings | Maximum allowed ping. | rust:ping:kick, rust:config:validate |
| G26-MATCH-AND-018 | Present | 26. Match and server settings | Ping-kick behavior. | rust:ping:kick |
| G26-MATCH-AND-019 | Present | 26. Match and server settings | Welcome/server message. | parity:G26-MATCH-AND-019, rust:config:validate |
| G26-MATCH-AND-020 | Present | 26. Match and server settings | Server contact link. | parity:G26-MATCH-AND-020, rust:config:validate |
| G26-MATCH-AND-021 | Present | 26. Match and server settings | Dedicated-server configuration. | parity:G26-MATCH-AND-021, rust:config:validate |
| G26-MATCH-AND-022 | Present | 26. Match and server settings | Custom map list. | parity:G26-MATCH-AND-022, rust:config:validate |
| G26-MATCH-AND-023 | Present | 26. Match and server settings | Custom weapon mod. | parity:G26-MATCH-AND-023, rust:config:validate |
| G26-MATCH-AND-024 | Present | 26. Match and server settings | Friendly fire. | parity:G26-MATCH-AND-024, rust:config:validate |
| G26-MATCH-AND-025 | Present | 26. Match and server settings | Respawn time. | parity:G26-MATCH-AND-025, rust:config:validate |
| G26-MATCH-AND-026 | Present | 26. Match and server settings | Bonus frequency/enabled state. | parity:G26-MATCH-AND-026, rust:config:validate |
| G26-MATCH-AND-027 | Present | 26. Match and server settings | Spectator limits. | parity:G26-MATCH-AND-027, rust:config:validate |
| G26-MATCH-AND-028 | Present | 26. Match and server settings | Admin password/authentication. | parity:G26-MATCH-AND-028, rust:config:validate |
| G26-MATCH-AND-029 | Present | 26. Match and server settings | Logging options. | parity:G26-MATCH-AND-029, rust:config:validate |
| G26-MATCH-AND-030 | Present | 26. Match and server settings | Name. | parity:G26-MATCH-AND-030, rust:config:validate |
| G26-MATCH-AND-031 | Present | 26. Match and server settings | Appearance. | parity:G26-MATCH-AND-031, rust:config:validate |
| G26-MATCH-AND-032 | Present | 26. Match and server settings | Default secondary. | parity:G26-MATCH-AND-032, rust:config:validate |
| G26-MATCH-AND-033 | Present | 26. Match and server settings | Background override. | web:render:visual |
| G26-MATCH-AND-034 | Present | 26. Match and server settings | Controls. | parity:G26-MATCH-AND-034, web:profiles:player |
| G26-MATCH-AND-035 | Present | 26. Match and server settings | Mouse sensitivity. | parity:G26-MATCH-AND-035, web:profiles:player |
| G26-MATCH-AND-036 | Present | 26. Match and server settings | Interface selection. | parity:G26-MATCH-AND-036, web:profiles:player |
| G26-MATCH-AND-037 | Present | 26. Match and server settings | Player indicator. | parity:G26-MATCH-AND-037, web:profiles:player |
| G26-MATCH-AND-038 | Present | 26. Match and server settings | Sniper line. | parity:G26-MATCH-AND-038, web:profiles:player |
| G26-MATCH-AND-039 | Present | 26. Match and server settings | Fullscreen/windowed. | web:screens:flow |
| G26-MATCH-AND-040 | Present | 26. Match and server settings | Display resolution. | web:render:visual |
| G26-MATCH-AND-041 | Present | 26. Match and server settings | Desktop resolution. | web:screens:flow |
| G26-MATCH-AND-042 | Present | 26. Match and server settings | Interface scaling. | parity:G26-MATCH-AND-042, web:render:effects |
| G26-MATCH-AND-043 | Present | 26. Match and server settings | Particle limit. | parity:G26-MATCH-AND-043, web:render:effects |
| G26-MATCH-AND-044 | Present | 26. Match and server settings | Bullet trails. | parity:G26-MATCH-AND-044, web:render:effects |
| G26-MATCH-AND-045 | Present | 26. Match and server settings | Weather rendering. | parity:G26-MATCH-AND-045, web:render:effects |
| G26-MATCH-AND-046 | Present | 26. Match and server settings | Texture filtering. | parity:G26-MATCH-AND-046, web:render:effects |
| G26-MATCH-AND-047 | Present | 26. Match and server settings | Resolution filtering. | parity:G26-MATCH-AND-047, web:render:effects |
| G26-MATCH-AND-048 | Present | 26. Match and server settings | Mipmapping. | parity:G26-MATCH-AND-048, web:render:effects |
| G26-MATCH-AND-049 | Present | 26. Match and server settings | Compatibility/fixed-pipeline equivalent. | parity:G26-MATCH-AND-049, web:render:effects |
| G26-MATCH-AND-050 | Present | 26. Match and server settings | Intro playback. | web:screens:flow |
| G26-MATCH-AND-051 | Present | 26. Match and server settings | Final-score screenshot. | web:input:screenshot |
| G26-MATCH-AND-052 | Present | 26. Match and server settings | Clanmatch color behavior. | web:hud:status |
| G26-MATCH-AND-053 | Present | 26. Match and server settings | Frame-rate limit or VSync. | web:render:visual |
| G26-MATCH-AND-054 | Present | 26. Match and server settings | Performance statistics. | parity:G26-MATCH-AND-054, web:audio:engine |
| G26-MATCH-AND-055 | Present | 26. Match and server settings | Sound volume. | parity:G26-MATCH-AND-055, web:audio:engine |
| G26-MATCH-AND-056 | Present | 26. Match and server settings | Music volume. | parity:G26-MATCH-AND-056, web:audio:engine |
| G26-MATCH-AND-057 | Present | 26. Match and server settings | Sound quality. | parity:G26-MATCH-AND-057, web:audio:engine |
| G26-MATCH-AND-058 | Present | 26. Match and server settings | Output device. | parity:G26-MATCH-AND-058, web:audio:engine |
| G26-MATCH-AND-059 | Present | 26. Match and server settings | Explosion effect. | parity:G26-MATCH-AND-059, web:audio:engine |
| G26-MATCH-AND-060 | Present | 26. Match and server settings | Distant battle. | parity:G26-MATCH-AND-060, web:audio:engine |
| G26-MATCH-AND-061 | Present | 26. Match and server settings | Game music. | parity:G26-MATCH-AND-061, web:audio:engine |
| G27-LOBBY-AND-001 | Present | 27. Lobby and room browser | Server/room refresh. | parity:G27-LOBBY-AND-001, web:lobby:filter |
| G27-LOBBY-AND-002 | Present | 27. Lobby and room browser | Cancel refresh. | parity:G27-LOBBY-AND-002, web:lobby:filter |
| G27-LOBBY-AND-003 | Present | 27. Lobby and room browser | Ping measurement. | parity:G27-LOBBY-AND-003, web:lobby:filter |
| G27-LOBBY-AND-004 | Present | 27. Lobby and room browser | Ping-all action. | parity:G27-LOBBY-AND-004, web:lobby:filter |
| G27-LOBBY-AND-005 | Present | 27. Lobby and room browser | Ping column. | parity:G27-LOBBY-AND-005, web:lobby:filter |
| G27-LOBBY-AND-006 | Present | 27. Lobby and room browser | Player count. | parity:G27-LOBBY-AND-006, web:lobby:filter |
| G27-LOBBY-AND-007 | Present | 27. Lobby and room browser | Maximum players. | parity:G27-LOBBY-AND-007, web:lobby:filter |
| G27-LOBBY-AND-008 | Present | 27. Lobby and room browser | Game mode. | parity:G27-LOBBY-AND-008, web:lobby:filter |
| G27-LOBBY-AND-009 | Present | 27. Lobby and room browser | Map name. | parity:G27-LOBBY-AND-009, web:lobby:filter |
| G27-LOBBY-AND-010 | Present | 27. Lobby and room browser | Country/region if desired. | web:lobby:filter |
| G27-LOBBY-AND-011 | Present | 27. Lobby and room browser | Password indicator. | parity:G27-LOBBY-AND-011, web:lobby:filter |
| G27-LOBBY-AND-012 | Present | 27. Lobby and room browser | Realistic indicator. | parity:G27-LOBBY-AND-012, web:lobby:filter |
| G27-LOBBY-AND-013 | Present | 27. Lobby and room browser | Survival indicator. | parity:G27-LOBBY-AND-013, web:lobby:filter |
| G27-LOBBY-AND-014 | Present | 27. Lobby and room browser | Advance indicator. | parity:G27-LOBBY-AND-014, web:lobby:filter |
| G27-LOBBY-AND-015 | Present | 27. Lobby and room browser | Weapon-mod indicator. | parity:G27-LOBBY-AND-015, web:lobby:filter |
| G27-LOBBY-AND-016 | Present | 27. Lobby and room browser | Version compatibility. | parity:G27-LOBBY-AND-016, web:lobby:filter |
| G27-LOBBY-AND-017 | Present | 27. Lobby and room browser | Favorites. | parity:G27-LOBBY-AND-017, web:lobby:filter |
| G27-LOBBY-AND-018 | Present | 27. Lobby and room browser | Favorite add/remove. | parity:G27-LOBBY-AND-018, web:lobby:filter |
| G27-LOBBY-AND-019 | Present | 27. Lobby and room browser | Direct IP/hostname join if supported. | parity:G27-LOBBY-AND-019, web:lobby:filter |
| G27-LOBBY-AND-020 | Present | 27. Lobby and room browser | Port input. | parity:G27-LOBBY-AND-020, web:lobby:filter |
| G27-LOBBY-AND-021 | Present | 27. Lobby and room browser | Password input. | parity:G27-LOBBY-AND-021, web:lobby:filter |
| G27-LOBBY-AND-022 | Present | 27. Lobby and room browser | Spectator join. | parity:G27-LOBBY-AND-022, web:lobby:filter |
| G27-LOBBY-AND-023 | Present | 27. Lobby and room browser | Sort and filter. | parity:G27-LOBBY-AND-023, web:lobby:filter |
| G27-LOBBY-AND-024 | Present | 27. Lobby and room browser | Search. | parity:G27-LOBBY-AND-024, web:lobby:filter |
| G27-LOBBY-AND-025 | Present | 27. Lobby and room browser | Full-room filtering. | parity:G27-LOBBY-AND-025, web:lobby:filter |
| G27-LOBBY-AND-026 | Present | 27. Lobby and room browser | Empty-room filtering. | parity:G27-LOBBY-AND-026, web:lobby:filter |
| G27-LOBBY-AND-027 | Present | 27. Lobby and room browser | Public lobby registration. | parity:G27-LOBBY-AND-027, web:lobby:filter |
| G27-LOBBY-AND-028 | Present | 27. Lobby and room browser | Reliable server discovery. | web:lobby:filter |
| G27-LOBBY-AND-029 | Present | 27. Lobby and room browser | Join/download progress. | web:maps:download |
| G27-LOBBY-AND-030 | Present | 27. Lobby and room browser | Cancel connection/download. | web:maps:download |
| G27-LOBBY-AND-031 | Present | 27. Lobby and room browser | Better reconnect state. | parity:G27-LOBBY-AND-031, web:lobby:filter |
| G27-LOBBY-AND-032 | Present | 27. Lobby and room browser | Room ownership/host settings. | rust:storage:fence |
| G27-LOBBY-AND-033 | Present | 27. Lobby and room browser | Room deletion or expiry controls. | rust:security:resume |
| G27-LOBBY-AND-034 | Present | 27. Lobby and room browser | Password-protected rooms in addition to invite codes. | parity:G27-LOBBY-AND-034, web:lobby:filter |
| G27-LOBBY-AND-035 | Present | 27. Lobby and room browser | Team choice after joining. | parity:G27-LOBBY-AND-035, web:lobby:filter |
| G27-LOBBY-AND-036 | Present | 27. Lobby and room browser | Late-join rules. | rust:match_lifecycle:phases |
| G27-LOBBY-AND-037 | Present | 27. Lobby and room browser | Abuse-resistant room creation. | parity:G27-LOBBY-AND-037, web:lobby:filter |
| G27-LOBBY-AND-038 | Present | 27. Lobby and room browser | Rate limiting for connection and room operations. | parity:G27-LOBBY-AND-038, web:lobby:filter |
| G28-SPECTATING-001 | Present | 28. Spectating | Spectator team. | parity:G28-SPECTATING-001, rust:spectator_world:follow, rust:match_world:teams |
| G28-SPECTATING-002 | Present | 28. Spectating | Join directly as spectator. | parity:G28-SPECTATING-002, rust:spectator:targets, rust:spectator_world:follow |
| G28-SPECTATING-003 | Present | 28. Spectating | Switch followed player. | parity:G28-SPECTATING-003, rust:spectator:targets, rust:spectator_world:follow |
| G28-SPECTATING-004 | Present | 28. Spectating | Previous/next player. | parity:G28-SPECTATING-004, rust:spectator:targets, rust:spectator_world:follow, web:spectate:keys |
| G28-SPECTATING-005 | Present | 28. Spectating | Free camera if desired. | parity:G28-SPECTATING-005, rust:spectator:targets, rust:spectator_world:follow, web:spectate:keys |
| G28-SPECTATING-006 | Present | 28. Spectating | Spectator HUD. | parity:G28-SPECTATING-006, web:spectate:hud |
| G28-SPECTATING-007 | Present | 28. Spectating | Spectator scoreboard. | parity:G28-SPECTATING-007, rust:statistics:scoreboard, web:stats:scoreboard |
| G28-SPECTATING-008 | Present | 28. Spectating | Spectator chat restrictions. | parity:G28-SPECTATING-008, rust:spectator:visibility |
| G28-SPECTATING-009 | Present | 28. Spectating | Realistic visibility restrictions. | parity:G28-SPECTATING-009, rust:spectator:visibility |
| G28-SPECTATING-010 | Present | 28. Spectating | Survival restrictions. | parity:G28-SPECTATING-010, rust:spectator:visibility, rust:modifiers:survival |
| G28-SPECTATING-011 | Present | 28. Spectating | Followed-player minimap/visibility behavior. | parity:G28-SPECTATING-011, web:hud:net |
| G28-SPECTATING-012 | Present | 28. Spectating | Delay option for competitive matches. | parity:G28-SPECTATING-012, rust:spectator:visibility |
| G29-SCORING-AND-001 | Present | 29. Scoring and statistics | Correct scoring for every game mode. | parity:G29-SCORING-AND-001, rust:match_lifecycle:scoring, rust:mode_world:flags |
| G29-SCORING-AND-002 | Present | 29. Scoring and statistics | Match time. | parity:G29-SCORING-AND-002, web:match:clock, rust:match_lifecycle:limits |
| G29-SCORING-AND-003 | Present | 29. Scoring and statistics | Individual points. | parity:G29-SCORING-AND-003, rust:statistics:scoreboard, web:stats:scoreboard |
| G29-SCORING-AND-004 | Present | 29. Scoring and statistics | Team points. | parity:G29-SCORING-AND-004, rust:match_world:ledger |
| G29-SCORING-AND-005 | Present | 29. Scoring and statistics | Captures. | parity:G29-SCORING-AND-005, rust:statistics:weapons, rust:statistics_world:feed, rust:mode_world:flags |
| G29-SCORING-AND-006 | Present | 29. Scoring and statistics | Flag returns. | parity:G29-SCORING-AND-006, rust:statistics:weapons, rust:statistics_world:feed |
| G29-SCORING-AND-007 | Present | 29. Scoring and statistics | Current server rank. | parity:G29-SCORING-AND-007, rust:statistics:scoreboard, web:stats:scoreboard |
| G29-SCORING-AND-008 | Present | 29. Scoring and statistics | Score difference from leader. | parity:G29-SCORING-AND-008, rust:statistics:scoreboard, web:stats:scoreboard |
| G29-SCORING-AND-009 | Present | 29. Scoring and statistics | Kill/point/capture limit display. | parity:G29-SCORING-AND-009, web:stats:scoreboard |
| G29-SCORING-AND-010 | Present | 29. Scoring and statistics | Weapon statistics for the current round. | parity:G29-SCORING-AND-010, rust:statistics:weapons, web:stats:weapons |
| G29-SCORING-AND-011 | Present | 29. Scoring and statistics | Shots fired. | parity:G29-SCORING-AND-011, rust:statistics:weapons, rust:statistics_world:feed |
| G29-SCORING-AND-012 | Present | 29. Scoring and statistics | Hits. | parity:G29-SCORING-AND-012, rust:statistics:weapons, rust:statistics_world:feed |
| G29-SCORING-AND-013 | Present | 29. Scoring and statistics | Accuracy. | parity:G29-SCORING-AND-013, rust:statistics:weapons, web:stats:weapons |
| G29-SCORING-AND-014 | Present | 29. Scoring and statistics | Kills per weapon. | parity:G29-SCORING-AND-014, rust:statistics:weapons, web:stats:weapons |
| G29-SCORING-AND-015 | Present | 29. Scoring and statistics | Deaths per weapon/cause. | parity:G29-SCORING-AND-015, rust:statistics:weapons, rust:statistics_world:feed |
| G29-SCORING-AND-016 | Present | 29. Scoring and statistics | Headshots. | parity:G29-SCORING-AND-016, rust:statistics:weapons, rust:statistics_world:feed |
| G29-SCORING-AND-017 | Present | 29. Scoring and statistics | Suicides. | parity:G29-SCORING-AND-017, rust:match_lifecycle:scoring |
| G29-SCORING-AND-018 | Present | 29. Scoring and statistics | Teamkills. | parity:G29-SCORING-AND-018, rust:match_lifecycle:scoring |
| G29-SCORING-AND-019 | Present | 29. Scoring and statistics | Objective statistics. | parity:G29-SCORING-AND-019, rust:statistics:weapons, rust:statistics_world:feed |
| G29-SCORING-AND-020 | Present | 29. Scoring and statistics | End-of-round summary. | parity:G29-SCORING-AND-020, rust:statistics:history, web:stats:summary |
| G29-SCORING-AND-021 | Present | 29. Scoring and statistics | Match history. | parity:G29-SCORING-AND-021, rust:statistics:history |
| G29-SCORING-AND-022 | Present | 29. Scoring and statistics | Persistent player statistics if desired. | rust:storage:stats |
| G29-SCORING-AND-023 | Missing | 29. Scoring and statistics | Clan/team statistics if desired. | parity:G29-SCORING-AND-023 |
| G29-SCORING-AND-024 | Present | 29. Scoring and statistics | Exportable logs. | parity:G29-SCORING-AND-024, rust:statistics:history |
| G30-NETWORKING-AND-001 | Present | 30. Networking and prediction | Proper client-side prediction for all movement states. | parity:G30-NETWORKING-AND-001, web:prediction:reconcile |
| G30-NETWORKING-AND-002 | Present | 30. Networking and prediction | Input reconciliation. | parity:G30-NETWORKING-AND-002, web:prediction:reconcile |
| G30-NETWORKING-AND-003 | Present | 30. Networking and prediction | Unacknowledged-input replay. | parity:G30-NETWORKING-AND-003, web:prediction:reconcile |
| G30-NETWORKING-AND-004 | Present | 30. Networking and prediction | Remote-player interpolation. | parity:G30-NETWORKING-AND-004, web:prediction:reconcile |
| G30-NETWORKING-AND-005 | Present | 30. Networking and prediction | Projectile interpolation. | parity:G30-NETWORKING-AND-005, web:prediction:reconcile |
| G30-NETWORKING-AND-006 | Present | 30. Networking and prediction | Extrapolation limits. | parity:G30-NETWORKING-AND-006, web:prediction:reconcile |
| G30-NETWORKING-AND-007 | Present | 30. Networking and prediction | Latency compensation. | parity:G30-NETWORKING-AND-007, web:prediction:reconcile |
| G30-NETWORKING-AND-008 | Missing | 30. Networking and prediction | Server rewind/lag compensation if appropriate. | parity:G30-NETWORKING-AND-008 |
| G30-NETWORKING-AND-009 | Present | 30. Networking and prediction | Clock synchronization. | parity:G30-NETWORKING-AND-009, web:prediction:reconcile |
| G30-NETWORKING-AND-010 | Present | 30. Networking and prediction | Measured ping. | parity:G30-NETWORKING-AND-010, web:prediction:reconcile |
| G30-NETWORKING-AND-011 | Present | 30. Networking and prediction | Packet-loss handling. | parity:G30-NETWORKING-AND-011, web:prediction:reconcile |
| G30-NETWORKING-AND-012 | Missing | 30. Networking and prediction | Snapshot delta compression. | parity:G30-NETWORKING-AND-012 |
| G30-NETWORKING-AND-013 | Missing | 30. Networking and prediction | Interest management if maps/player counts grow. | parity:G30-NETWORKING-AND-013 |
| G30-NETWORKING-AND-014 | Missing | 30. Networking and prediction | Binary protocol or more compact encoding if needed. | parity:G30-NETWORKING-AND-014 |
| G30-NETWORKING-AND-015 | Present | 30. Networking and prediction | Weapon/event prediction. | parity:G30-NETWORKING-AND-015, web:prediction:reconcile |
| G30-NETWORKING-AND-016 | Present | 30. Networking and prediction | Predicted muzzle/projectile effects. | parity:G30-NETWORKING-AND-016, web:prediction:reconcile |
| G30-NETWORKING-AND-017 | Present | 30. Networking and prediction | Rollback correction smoothing. | parity:G30-NETWORKING-AND-017, web:prediction:reconcile |
| G30-NETWORKING-AND-018 | Present | 30. Networking and prediction | Map and ruleset synchronization. | parity:G30-NETWORKING-AND-018, web:prediction:reconcile |
| G30-NETWORKING-AND-019 | Present | 30. Networking and prediction | Disconnect reason handling. | parity:G30-NETWORKING-AND-019, web:prediction:reconcile |
| G30-NETWORKING-AND-020 | Present | 30. Networking and prediction | Robust resumption after page sleep/mobile backgrounding. | parity:G30-NETWORKING-AND-020, web:prediction:reconcile |
| G30-NETWORKING-AND-021 | Present | 30. Networking and prediction | Duplicate-session handling. | rust:security:resume |
| G30-NETWORKING-AND-022 | Present | 30. Networking and prediction | Rate limits for every client message. | parity:G30-NETWORKING-AND-022, web:prediction:reconcile |
| G30-NETWORKING-AND-023 | Present | 30. Networking and prediction | Anti-speedhack/input-frequency validation. | parity:G30-NETWORKING-AND-023, web:prediction:reconcile |
| G30-NETWORKING-AND-024 | Present | 30. Networking and prediction | Fire-rate validation. | parity:G30-NETWORKING-AND-024, web:prediction:reconcile |
| G30-NETWORKING-AND-025 | Present | 30. Networking and prediction | Aim/input sanity validation beyond coordinate bounds. | parity:G30-NETWORKING-AND-025, web:prediction:reconcile |
| G30-NETWORKING-AND-026 | Present | 30. Networking and prediction | Server-authoritative pickups, flags, bonuses, and round state. | parity:G30-NETWORKING-AND-026, web:prediction:reconcile |
| G30-NETWORKING-AND-027 | Present | 30. Networking and prediction | Network load and soak tests. | rust:replay:soak |
| G30-NETWORKING-AND-028 | Present | 30. Networking and prediction | High-latency/jitter/loss simulation tests. | web:prediction:reconcile |
| G31-ANTI-CHEAT-001 | Present | 31. Anti-cheat and abuse resistance | Server-authoritative collision validation. | parity:G31-ANTI-CHEAT-001, rust:security:limits |
| G31-ANTI-CHEAT-002 | Present | 31. Anti-cheat and abuse resistance | Server-authoritative weapon selection. | parity:G31-ANTI-CHEAT-002, rust:security:limits |
| G31-ANTI-CHEAT-003 | Present | 31. Anti-cheat and abuse resistance | Server-authoritative reload and inventory. | parity:G31-ANTI-CHEAT-003, rust:security:limits |
| G31-ANTI-CHEAT-004 | Present | 31. Anti-cheat and abuse resistance | Input-rate limiting. | parity:G31-ANTI-CHEAT-004, rust:security:limits |
| G31-ANTI-CHEAT-005 | Present | 31. Anti-cheat and abuse resistance | Movement feasibility validation. | parity:G31-ANTI-CHEAT-005, rust:security:limits |
| G31-ANTI-CHEAT-006 | Present | 31. Anti-cheat and abuse resistance | Aim-value validation. | parity:G31-ANTI-CHEAT-006, rust:security:limits |
| G31-ANTI-CHEAT-007 | Present | 31. Anti-cheat and abuse resistance | Chat abuse controls. | parity:G31-ANTI-CHEAT-007, rust:security:limits |
| G31-ANTI-CHEAT-008 | Present | 31. Anti-cheat and abuse resistance | Connection/IP rate limiting. | parity:G31-ANTI-CHEAT-008, rust:security:limits |
| G31-ANTI-CHEAT-009 | Present | 31. Anti-cheat and abuse resistance | Room-creation rate limiting. | parity:G31-ANTI-CHEAT-009, rust:security:limits |
| G31-ANTI-CHEAT-010 | Present | 31. Anti-cheat and abuse resistance | Admin permission validation. | parity:G31-ANTI-CHEAT-010, rust:security:limits |
| G31-ANTI-CHEAT-011 | Present | 31. Anti-cheat and abuse resistance | Ban enforcement. | parity:G31-ANTI-CHEAT-011, rust:security:limits |
| G31-ANTI-CHEAT-012 | Present | 31. Anti-cheat and abuse resistance | Temporary bans. | parity:G31-ANTI-CHEAT-012, rust:security:limits |
| G31-ANTI-CHEAT-013 | Present | 31. Anti-cheat and abuse resistance | Audit logging. | parity:G31-ANTI-CHEAT-013, rust:security:limits |
| G31-ANTI-CHEAT-014 | Present | 31. Anti-cheat and abuse resistance | Suspicious behavior metrics. | parity:G31-ANTI-CHEAT-014, rust:security:limits |
| G31-ANTI-CHEAT-015 | Missing | 31. Anti-cheat and abuse resistance | Protocol fuzzing. | parity:G31-ANTI-CHEAT-015 |
| G31-ANTI-CHEAT-016 | Present | 31. Anti-cheat and abuse resistance | Malformed WebSocket testing. | parity:G31-ANTI-CHEAT-016, rust:security:limits |
| G31-ANTI-CHEAT-017 | Present | 31. Anti-cheat and abuse resistance | Replay-based cheat investigation. | rust:replay:repair |
| G31-ANTI-CHEAT-018 | Present | 31. Anti-cheat and abuse resistance | Secure resume tokens. | parity:G31-ANTI-CHEAT-018, rust:security:limits |
| G31-ANTI-CHEAT-019 | Present | 31. Anti-cheat and abuse resistance | Token expiry/rotation. | parity:G31-ANTI-CHEAT-019, rust:security:limits |
| G31-ANTI-CHEAT-020 | Missing | 31. Anti-cheat and abuse resistance | Deployment-level denial-of-service protection. | parity:G31-ANTI-CHEAT-020 |
| G31-ANTI-CHEAT-021 | Present | 31. Anti-cheat and abuse resistance | No trust in client-provided cosmetics or settings that affect gameplay. | parity:G31-ANTI-CHEAT-021, rust:security:limits |
| G32-MENUS-AND-001 | Present | 32. Menus and overall game flow | Main menu. | parity:G32-MENUS-AND-001, web:screens:flow |
| G32-MENUS-AND-002 | Present | 32. Menus and overall game flow | Profile selection. | parity:G32-MENUS-AND-002, web:screens:flow |
| G32-MENUS-AND-003 | Present | 32. Menus and overall game flow | Player customization. | parity:G32-MENUS-AND-003, web:screens:flow |
| G32-MENUS-AND-004 | Present | 32. Menus and overall game flow | Join-game screen. | parity:G32-MENUS-AND-004, web:screens:flow |
| G32-MENUS-AND-005 | Present | 32. Menus and overall game flow | Start-game/server screen. | parity:G32-MENUS-AND-005, web:screens:flow |
| G32-MENUS-AND-006 | Present | 32. Menus and overall game flow | Options menu. | parity:G32-MENUS-AND-006, web:screens:flow |
| G32-MENUS-AND-007 | Present | 32. Menus and overall game flow | Controls menu. | parity:G32-MENUS-AND-007, web:screens:flow |
| G32-MENUS-AND-008 | Present | 32. Menus and overall game flow | Weapon-selection screen during respawn. | parity:G32-MENUS-AND-008, web:screens:flow |
| G32-MENUS-AND-009 | Present | 32. Menus and overall game flow | Team-selection screen. | parity:G32-MENUS-AND-009, web:screens:flow |
| G32-MENUS-AND-010 | Present | 32. Menus and overall game flow | Spectator selection. | parity:G32-MENUS-AND-010, web:screens:flow |
| G32-MENUS-AND-011 | Present | 32. Menus and overall game flow | Pause menu. | parity:G32-MENUS-AND-011, web:screens:flow |
| G32-MENUS-AND-012 | Present | 32. Menus and overall game flow | Disconnect confirmation. | parity:G32-MENUS-AND-012, web:screens:flow |
| G32-MENUS-AND-013 | Present | 32. Menus and overall game flow | Map loading screen. | parity:G32-MENUS-AND-013, web:screens:flow |
| G32-MENUS-AND-014 | Present | 32. Menus and overall game flow | Asset-download screen. | parity:G32-MENUS-AND-014, web:screens:flow |
| G32-MENUS-AND-015 | Present | 32. Menus and overall game flow | Round-intro countdown. | parity:G32-MENUS-AND-015, web:screens:flow |
| G32-MENUS-AND-016 | Present | 32. Menus and overall game flow | Round-end screen. | parity:G32-MENUS-AND-016, web:screens:flow |
| G32-MENUS-AND-017 | Present | 32. Menus and overall game flow | Match-results screen. | parity:G32-MENUS-AND-017, web:screens:flow |
| G32-MENUS-AND-018 | Present | 32. Menus and overall game flow | Connection-lost overlay. | parity:G32-MENUS-AND-018, web:screens:flow |
| G32-MENUS-AND-019 | Present | 32. Menus and overall game flow | Version mismatch/update flow. | parity:G32-MENUS-AND-019, web:screens:flow |
| G32-MENUS-AND-020 | Present | 32. Menus and overall game flow | Credits. | parity:G32-MENUS-AND-020, web:screens:flow |
| G32-MENUS-AND-021 | Present | 32. Menus and overall game flow | Help/manual. | parity:G32-MENUS-AND-021, web:screens:flow |
| G32-MENUS-AND-022 | Present | 32. Menus and overall game flow | First-run control tutorial. | parity:G32-MENUS-AND-022, web:screens:flow |
| G32-MENUS-AND-023 | Present | 32. Menus and overall game flow | Mobile onboarding. | parity:G32-MENUS-AND-023, web:screens:flow |
| G33-CUSTOM-INTERFACES-001 | Present | 33. Custom interfaces and modding | Loadable HUD/interface definitions. | parity:G33-CUSTOM-INTERFACES-001, web:mods:package |
| G33-CUSTOM-INTERFACES-002 | Present | 33. Custom interfaces and modding | Multiple interface presets. | parity:G33-CUSTOM-INTERFACES-002, web:mods:package |
| G33-CUSTOM-INTERFACES-003 | Present | 33. Custom interfaces and modding | Custom cursor. | parity:G33-CUSTOM-INTERFACES-003, web:mods:package |
| G33-CUSTOM-INTERFACES-004 | Present | 33. Custom interfaces and modding | Custom HUD image positions. | parity:G33-CUSTOM-INTERFACES-004, web:mods:package |
| G33-CUSTOM-INTERFACES-005 | Present | 33. Custom interfaces and modding | Interface scaling. | parity:G33-CUSTOM-INTERFACES-005, web:mods:ini |
| G33-CUSTOM-INTERFACES-006 | Present | 33. Custom interfaces and modding | Custom weapon graphics. | parity:G33-CUSTOM-INTERFACES-006, web:mods:package |
| G33-CUSTOM-INTERFACES-007 | Present | 33. Custom interfaces and modding | Custom sounds. | parity:G33-CUSTOM-INTERFACES-007, web:mods:package |
| G33-CUSTOM-INTERFACES-008 | Present | 33. Custom interfaces and modding | Custom character/gostek graphics. | parity:G33-CUSTOM-INTERFACES-008, web:mods:package |
| G33-CUSTOM-INTERFACES-009 | Present | 33. Custom interfaces and modding | `mod.ini`-style asset scaling. | parity:G33-CUSTOM-INTERFACES-009, rust:mod_package:ini |
| G33-CUSTOM-INTERFACES-010 | Present | 33. Custom interfaces and modding | Mod selection/launching. | parity:G33-CUSTOM-INTERFACES-010, web:mods:package |
| G33-CUSTOM-INTERFACES-011 | Present | 33. Custom interfaces and modding | Mod preview. | parity:G33-CUSTOM-INTERFACES-011, web:mods:package |
| G33-CUSTOM-INTERFACES-012 | Present | 33. Custom interfaces and modding | Mod packaging. | parity:G33-CUSTOM-INTERFACES-012, rust:mod_package:safe |
| G33-CUSTOM-INTERFACES-013 | Present | 33. Custom interfaces and modding | Mod downloading. | parity:G33-CUSTOM-INTERFACES-013, web:mods:required |
| G33-CUSTOM-INTERFACES-014 | Present | 33. Custom interfaces and modding | Mod version/hash matching. | parity:G33-CUSTOM-INTERFACES-014, web:mods:hash |
| G33-CUSTOM-INTERFACES-015 | Present | 33. Custom interfaces and modding | Server-required mod support. | parity:G33-CUSTOM-INTERFACES-015, web:mods:required |
| G33-CUSTOM-INTERFACES-016 | Present | 33. Custom interfaces and modding | Safe path handling. | parity:G33-CUSTOM-INTERFACES-016, rust:mod_package:safe |
| G33-CUSTOM-INTERFACES-017 | Present | 33. Custom interfaces and modding | Asset-size limits. | parity:G33-CUSTOM-INTERFACES-017, rust:mod_package:safe |
| G33-CUSTOM-INTERFACES-018 | Present | 33. Custom interfaces and modding | License/provenance metadata. | parity:G33-CUSTOM-INTERFACES-018, rust:mod_package:safe |
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
| G35-DEMO-AND-001 | Present | 35. Demo and replay system | Record match inputs/events. | parity:G35-DEMO-AND-001, rust:replay:record |
| G35-DEMO-AND-002 | Present | 35. Demo and replay system | Deterministic replay format. | parity:G35-DEMO-AND-002, rust:replay:play |
| G35-DEMO-AND-003 | Present | 35. Demo and replay system | Replay metadata. | parity:G35-DEMO-AND-003, rust:replay:compat |
| G35-DEMO-AND-004 | Present | 35. Demo and replay system | Replay playback. | parity:G35-DEMO-AND-004, rust:replay:play |
| G35-DEMO-AND-005 | Present | 35. Demo and replay system | Pause. | parity:G35-DEMO-AND-005, web:replay:player |
| G35-DEMO-AND-006 | Present | 35. Demo and replay system | Seek. | parity:G35-DEMO-AND-006, rust:replay:seek |
| G35-DEMO-AND-007 | Present | 35. Demo and replay system | Fast-forward. | parity:G35-DEMO-AND-007, rust:replay:seek |
| G35-DEMO-AND-008 | Present | 35. Demo and replay system | Follow player. | parity:G35-DEMO-AND-008, web:replay:camera |
| G35-DEMO-AND-009 | Present | 35. Demo and replay system | Free camera. | parity:G35-DEMO-AND-009, web:replay:camera |
| G35-DEMO-AND-010 | Present | 35. Demo and replay system | Replay compatibility/versioning. | parity:G35-DEMO-AND-010, rust:replay:compat |
| G35-DEMO-AND-011 | Present | 35. Demo and replay system | Replay validation. | parity:G35-DEMO-AND-011, rust:replay:play |
| G35-DEMO-AND-012 | Present | 35. Demo and replay system | Export or share replay. | parity:G35-DEMO-AND-012, rust:replay:repair |
| G35-DEMO-AND-013 | Present | 35. Demo and replay system | Demo repair/recovery where feasible. | parity:G35-DEMO-AND-013, rust:replay:repair |
| G35-DEMO-AND-014 | Present | 35. Demo and replay system | Server-side competitive match recording. | parity:G35-DEMO-AND-014, rust:replay:record |
| G36-OPERATIONAL-AND-001 | Present | 36. Operational and production work | Persistent server data. | parity:G36-OPERATIONAL-AND-001, rust:storage:restart |
| G36-OPERATIONAL-AND-002 | Missing | 36. Operational and production work | Persistent accounts. | parity:G36-OPERATIONAL-AND-002 |
| G36-OPERATIONAL-AND-003 | Present | 36. Operational and production work | Persistent statistics. | rust:storage:stats |
| G36-OPERATIONAL-AND-004 | Present | 36. Operational and production work | Persistent bans/admins. | parity:G36-OPERATIONAL-AND-004, rust:storage:restart |
| G36-OPERATIONAL-AND-005 | Present | 36. Operational and production work | Server restart recovery. | parity:G36-OPERATIONAL-AND-005, rust:storage:restart |
| G36-OPERATIONAL-AND-006 | Present | 36. Operational and production work | Graceful active-match handling during deployment. | parity:G36-OPERATIONAL-AND-006, rust:ops:ready |
| G36-OPERATIONAL-AND-007 | Present | 36. Operational and production work | Horizontal scaling strategy. | parity:G36-OPERATIONAL-AND-007, rust:storage:fence |
| G36-OPERATIONAL-AND-008 | Present | 36. Operational and production work | Room ownership across multiple server processes. | parity:G36-OPERATIONAL-AND-008, rust:storage:fence |
| G36-OPERATIONAL-AND-009 | Present | 36. Operational and production work | Lobby service. | parity:G36-OPERATIONAL-AND-009, web:lobby:filter |
| G36-OPERATIONAL-AND-010 | Missing | 36. Operational and production work | Database. | parity:G36-OPERATIONAL-AND-010 |
| G36-OPERATIONAL-AND-011 | Missing | 36. Operational and production work | Authentication if accounts are added. | parity:G36-OPERATIONAL-AND-011 |
| G36-OPERATIONAL-AND-012 | Present | 36. Operational and production work | Observability beyond three counters. | parity:G36-OPERATIONAL-AND-012, rust:ops:ready |
| G36-OPERATIONAL-AND-013 | Present | 36. Operational and production work | Structured logs. | parity:G36-OPERATIONAL-AND-013, rust:ops:ready |
| G36-OPERATIONAL-AND-014 | Present | 36. Operational and production work | Error tracking. | parity:G36-OPERATIONAL-AND-014, rust:ops:ready |
| G36-OPERATIONAL-AND-015 | Present | 36. Operational and production work | Latency metrics. | parity:G36-OPERATIONAL-AND-015, web:prediction:reconcile |
| G36-OPERATIONAL-AND-016 | Present | 36. Operational and production work | Tick-duration metrics. | parity:G36-OPERATIONAL-AND-016, rust:ops:ready |
| G36-OPERATIONAL-AND-017 | Present | 36. Operational and production work | Connected-player metrics. | parity:G36-OPERATIONAL-AND-017, rust:ops:ready |
| G36-OPERATIONAL-AND-018 | Present | 36. Operational and production work | Per-room metrics. | parity:G36-OPERATIONAL-AND-018, rust:ops:ready |
| G36-OPERATIONAL-AND-019 | Present | 36. Operational and production work | Health versus readiness endpoints. | parity:G36-OPERATIONAL-AND-019, rust:ops:ready |
| G36-OPERATIONAL-AND-020 | Present | 36. Operational and production work | Backups. | parity:G36-OPERATIONAL-AND-020, rust:storage:restart |
| G36-OPERATIONAL-AND-021 | Present | 36. Operational and production work | Migration process. | parity:G36-OPERATIONAL-AND-021, rust:storage:restart |
| G36-OPERATIONAL-AND-022 | Present | 36. Operational and production work | Load testing. | parity:G36-OPERATIONAL-AND-022, server:load |
| G36-OPERATIONAL-AND-023 | Present | 36. Operational and production work | Soak testing. | parity:G36-OPERATIONAL-AND-023, rust:replay:soak |
| G36-OPERATIONAL-AND-024 | Missing | 36. Operational and production work | Browser compatibility testing. | parity:G36-OPERATIONAL-AND-024 |
| G36-OPERATIONAL-AND-025 | Present | 36. Operational and production work | Mobile-device testing. | parity:G36-OPERATIONAL-AND-025, web:mobile:movement-pad |
| G36-OPERATIONAL-AND-026 | Present | 36. Operational and production work | Touch-control usability testing. | parity:G36-OPERATIONAL-AND-026, web:mobile:movement-pad |
| G36-OPERATIONAL-AND-027 | Present | 36. Operational and production work | Accessibility testing. | parity:G36-OPERATIONAL-AND-027, web:a11y:labels |
| G36-OPERATIONAL-AND-028 | Present | 36. Operational and production work | Security review. | parity:G36-OPERATIONAL-AND-028, docs:ops:security-review |
| G36-OPERATIONAL-AND-029 | Present | 36. Operational and production work | Dependency scanning. | parity:G36-OPERATIONAL-AND-029, ci:audit |
| G36-OPERATIONAL-AND-030 | Present | 36. Operational and production work | Deployment rollback testing. | parity:G36-OPERATIONAL-AND-030, scripts:record-image-digests |
| G36-OPERATIONAL-AND-031 | Missing | 36. Operational and production work | Asset CDN/caching. | parity:G36-OPERATIONAL-AND-031 |
| G36-OPERATIONAL-AND-032 | Present | 36. Operational and production work | Static-asset compression. | parity:G36-OPERATIONAL-AND-032, infra:caddy:encode |
| G36-OPERATIONAL-AND-033 | Present | 36. Operational and production work | Privacy policy and moderation policy if publicly operated. | parity:G36-OPERATIONAL-AND-033, docs:ops:policies |
| G37-TESTING-REQUIRED-001 | Present | 37. Testing required for parity | Source-versus-Rust movement fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-002 | Present | 37. Testing required for parity | Jump fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-003 | Present | 37. Testing required for parity | Jet fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-004 | Present | 37. Testing required for parity | Crouch/prone/roll/backflip fixtures. | rust:movement_fixtures |
| G37-TESTING-REQUIRED-005 | Present | 37. Testing required for parity | Polygon collision fixtures. | rust:collision_fixtures:geometry |
| G37-TESTING-REQUIRED-006 | Present | 37. Testing required for parity | One-way polygon fixtures. | rust:collision_fixtures:one-way |
| G37-TESTING-REQUIRED-007 | Present | 37. Testing required for parity | Weapon fixture for every weapon and field. | parity:G37-TESTING-REQUIRED-007, rust:weapon_config_fixtures |
| G37-TESTING-REQUIRED-008 | Present | 37. Testing required for parity | Fire-rate fixtures. | parity:G37-TESTING-REQUIRED-008, rust:game_core::tests::fire_is_rate_limited |
| G37-TESTING-REQUIRED-009 | Present | 37. Testing required for parity | Reload fixtures. | parity:G37-TESTING-REQUIRED-009, rust:weapon_inventory_fixtures |
| G37-TESTING-REQUIRED-010 | Present | 37. Testing required for parity | Startup fixtures. | parity:G37-TESTING-REQUIRED-010, rust:weapon_projectile_fixtures:firing-rules |
| G37-TESTING-REQUIRED-011 | Present | 37. Testing required for parity | Spread fixtures. | parity:G37-TESTING-REQUIRED-011, rust:weapon_ballistics_fixtures:accuracy |
| G37-TESTING-REQUIRED-012 | Present | 37. Testing required for parity | Recoil fixtures. | parity:G37-TESTING-REQUIRED-012, rust:weapon_ballistics_fixtures:accuracy |
| G37-TESTING-REQUIRED-013 | Present | 37. Testing required for parity | Bink fixtures. | parity:G37-TESTING-REQUIRED-013, rust:weapon_ballistics_fixtures:accuracy |
| G37-TESTING-REQUIRED-014 | Present | 37. Testing required for parity | Velocity-inheritance fixtures. | parity:G37-TESTING-REQUIRED-014, rust:weapon_ballistics_fixtures:inherit |
| G37-TESTING-REQUIRED-015 | Present | 37. Testing required for parity | Head/chest/leg damage fixtures. | parity:G37-TESTING-REQUIRED-015, rust:damage_fixtures:regions |
| G37-TESTING-REQUIRED-016 | Present | 37. Testing required for parity | Explosion fixtures. | parity:G37-TESTING-REQUIRED-016, rust:weapon_ballistics_fixtures:explosion |
| G37-TESTING-REQUIRED-017 | Present | 37. Testing required for parity | Grenade bounce/fuse fixtures. | parity:G37-TESTING-REQUIRED-017, rust:weapon_family_fixtures:explosive |
| G37-TESTING-REQUIRED-018 | Present | 37. Testing required for parity | Knife-throw fixtures. | parity:G37-TESTING-REQUIRED-018, rust:weapon_inventory_fixtures |
| G37-TESTING-REQUIRED-019 | Present | 37. Testing required for parity | Flame fixtures. | parity:G37-TESTING-REQUIRED-019, rust:weapon_family_fixtures:flame |
| G37-TESTING-REQUIRED-020 | Present | 37. Testing required for parity | Flag interaction fixtures. | parity:G37-TESTING-REQUIRED-020, rust:objective:flag-states |
| G37-TESTING-REQUIRED-021 | Present | 37. Testing required for parity | Bonus-kit fixtures. | parity:G37-TESTING-REQUIRED-021, rust:bonus:effects |
| G37-TESTING-REQUIRED-022 | Present | 37. Testing required for parity | Every mode’s scoring fixtures. | parity:G37-TESTING-REQUIRED-022, rust:match_lifecycle:limits |
| G37-TESTING-REQUIRED-023 | Present | 37. Testing required for parity | Round-limit fixtures. | parity:G37-TESTING-REQUIRED-023, rust:match_lifecycle:limits |
| G37-TESTING-REQUIRED-024 | Present | 37. Testing required for parity | Respawn fixtures. | parity:G37-TESTING-REQUIRED-024, rust:damage_fixtures:respawn |
| G37-TESTING-REQUIRED-025 | Present | 37. Testing required for parity | Map-loading fixtures. | parity:G37-TESTING-REQUIRED-025, rust:map_validation:filters |
| G37-TESTING-REQUIRED-026 | Present | 37. Testing required for parity | PMS parser fuzzing. | parity:G37-TESTING-REQUIRED-026, rust:content::pms |
| G37-TESTING-REQUIRED-027 | Present | 37. Testing required for parity | Determinism across native Rust and Wasm. | parity:G37-TESTING-REQUIRED-027 |
| G37-TESTING-REQUIRED-028 | Present | 37. Testing required for parity | Network reconciliation tests. | parity:G37-TESTING-REQUIRED-028, web:prediction:reconcile |
| G37-TESTING-REQUIRED-029 | Present | 37. Testing required for parity | Two-player browser integration test. | parity:G37-TESTING-REQUIRED-029, web:smoke:aim-input |
| G37-TESTING-REQUIRED-030 | Present | 37. Testing required for parity | Full 16-player test. | parity:G37-TESTING-REQUIRED-030, rust:capacity:16 |
| G37-TESTING-REQUIRED-031 | Present | 37. Testing required for parity | Reconnect test. | parity:G37-TESTING-REQUIRED-031, rust:security:resume |
| G37-TESTING-REQUIRED-032 | Present | 37. Testing required for parity | Server-restart test. | parity:G37-TESTING-REQUIRED-032, rust:storage:restart |
| G37-TESTING-REQUIRED-033 | Present | 37. Testing required for parity | Mobile portrait test. | parity:G37-TESTING-REQUIRED-033, web:mobile:movement-pad |
| G37-TESTING-REQUIRED-034 | Present | 37. Testing required for parity | Mobile landscape test. | parity:G37-TESTING-REQUIRED-034, web:mobile:movement-pad |
| G37-TESTING-REQUIRED-035 | Present | 37. Testing required for parity | Low/high latency tests. | parity:G37-TESTING-REQUIRED-035, web:prediction:reconcile |
| G37-TESTING-REQUIRED-036 | Present | 37. Testing required for parity | Packet-loss tests. | parity:G37-TESTING-REQUIRED-036, web:prediction:reconcile |
| G37-TESTING-REQUIRED-037 | Present | 37. Testing required for parity | Long-running server soak test. | parity:G37-TESTING-REQUIRED-037, rust:replay:soak |
