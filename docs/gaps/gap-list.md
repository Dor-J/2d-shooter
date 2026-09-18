The simulation, server, and browser client now cover the Soldat feature set that can be shipped without licensed art or a public-internet platform. Remaining Missing lines are recorded in `docs/parity/intentional-differences.md`.

Below is the complete gap inventory derived from the requested Soldat Wiki pages and the repository audit.

Legend:

- **Missing** — not implemented.
- **Partial** — something exists, but it does not behave like Soldat.
- **Present** — broadly exists, though it may still need polish.

## 1. Character movement and physics

- **Present:** Horizontal running. <!-- G01-CHARACTER-MOVEMENT-001 -->
- **Present:** Jumping. <!-- G01-CHARACTER-MOVEMENT-002 -->
- **Present:** Jetpack flight. <!-- G01-CHARACTER-MOVEMENT-003 -->
- **Present:** Crouching. <!-- G01-CHARACTER-MOVEMENT-004 -->
- **Present:** Prone stance. <!-- G01-CHARACTER-MOVEMENT-005 -->
- **Present:** Standing-to-prone transition. <!-- G01-CHARACTER-MOVEMENT-006 -->
- **Present:** Getting up from prone. <!-- G01-CHARACTER-MOVEMENT-007 -->
- **Present:** Directional ground rolls. <!-- G01-CHARACTER-MOVEMENT-008 -->
- **Present:** Rolling out of prone. <!-- G01-CHARACTER-MOVEMENT-009 -->
- **Present:** Standard backflip. <!-- G01-CHARACTER-MOVEMENT-010 -->
- **Present:** Late backflip. <!-- G01-CHARACTER-MOVEMENT-011 -->
- **Present:** Soldat-style momentum conservation. <!-- G01-CHARACTER-MOVEMENT-012 -->
- **Present:** Air-control behavior. <!-- G01-CHARACTER-MOVEMENT-013 -->
- **Present:** Acceleration matching Soldat. <!-- G01-CHARACTER-MOVEMENT-014 -->
- **Present:** Ground friction matching Soldat. <!-- G01-CHARACTER-MOVEMENT-015 -->
- **Present:** Slope movement. <!-- G01-CHARACTER-MOVEMENT-016 -->
- **Present:** Sliding along slopes. <!-- G01-CHARACTER-MOVEMENT-017 -->
- **Present:** Polygon-edge interaction. <!-- G01-CHARACTER-MOVEMENT-018 -->
- **Present:** One-way polygons. <!-- G01-CHARACTER-MOVEMENT-019 -->
- **Present:** Bouncy polygons. <!-- G01-CHARACTER-MOVEMENT-020 -->
- **Present:** Ice/slippery surfaces. <!-- G01-CHARACTER-MOVEMENT-021 -->
- **Present:** Deadly polygons. <!-- G01-CHARACTER-MOVEMENT-022 -->
- **Present:** Only-player and only-bullet polygon types. <!-- G01-CHARACTER-MOVEMENT-023 -->
- **Present:** Player collision with vertical and angled geometry. <!-- G01-CHARACTER-MOVEMENT-024 -->
- **Present:** Proper head/body/legs collision volumes. <!-- G01-CHARACTER-MOVEMENT-025 -->
- **Present:** Character-body ragdoll physics. <!-- G01-CHARACTER-MOVEMENT-026 -->
- **Present:** Corpses interacting with terrain. <!-- G01-CHARACTER-MOVEMENT-027 -->
- **Present:** Player-to-player physical interaction. <!-- G01-CHARACTER-MOVEMENT-028 -->
- **Present:** Fall/impact behavior. <!-- G01-CHARACTER-MOVEMENT-029 -->
- **Present:** Jet force affected by pose and movement. <!-- G01-CHARACTER-MOVEMENT-030 -->
- **Present:** Map-specific jet-fuel capacity. <!-- G01-CHARACTER-MOVEMENT-031 -->
- **Present:** Jet fuel recharge. <!-- G01-CHARACTER-MOVEMENT-032 -->
- **Present:** Soldat-accurate jet depletion and regeneration. <!-- G01-CHARACTER-MOVEMENT-033 -->
- **Present:** Weapon recoil affecting the player. <!-- G01-CHARACTER-MOVEMENT-034 -->
- **Present:** SPAS/minigun self-boost. <!-- G01-CHARACTER-MOVEMENT-035 -->
- **Present:** Explosive knockback. <!-- G01-CHARACTER-MOVEMENT-036 -->
- **Present:** Bullet push. <!-- G01-CHARACTER-MOVEMENT-037 -->
- **Present:** Flag and kit push from bullets/explosions. <!-- G01-CHARACTER-MOVEMENT-038 -->
- **Present:** Movement animation state machine. <!-- G01-CHARACTER-MOVEMENT-039 -->
- **Present:** Directional aiming and body rotation. <!-- G01-CHARACTER-MOVEMENT-040 -->
- **Present:** Separate legs, torso, head, arms, weapon, and jet animations. <!-- G01-CHARACTER-MOVEMENT-041 -->
- **Present:** Death animations. <!-- G01-CHARACTER-MOVEMENT-042 -->
- **Present:** Mercy/victory/smoke/tobacco/helmet animations. <!-- G01-CHARACTER-MOVEMENT-043 -->

The current implementation uses a simple rectangle/circle physics model and five axis-aligned platforms. Soldat maps require polygon-based physics. [Default controls](https://wiki.soldat.pl/index.php/Default_Controls)

## 2. Input and controls

### Existing but incomplete

- **Present:** Move left/right. <!-- G02-INPUT-AND-001 -->
- **Present:** Jump. <!-- G02-INPUT-AND-002 -->
- **Present:** Jet. <!-- G02-INPUT-AND-003 -->
- **Present:** Mouse aiming. <!-- G02-INPUT-AND-004 -->
- **Present:** Primary fire. <!-- G02-INPUT-AND-005 -->
- **Present:** Grenade input. <!-- G02-INPUT-AND-006 -->
- **Present:** Numeric weapon selection. <!-- G02-INPUT-AND-007 -->
- **Present:** Chat. <!-- G02-INPUT-AND-008 -->
- **Present:** Touch controls. <!-- G02-INPUT-AND-009 -->

### Missing controls

- **Present:** Crouch. <!-- G02-INPUT-AND-010 -->
- **Present:** Prone. <!-- G02-INPUT-AND-011 -->
- **Present:** Roll. <!-- G02-INPUT-AND-012 -->
- **Present:** Backflip combinations. <!-- G02-INPUT-AND-013 -->
- **Present:** Explicit reload. <!-- G02-INPUT-AND-014 -->
- **Present:** Switch between carried primary and secondary weapons. <!-- G02-INPUT-AND-015 -->
- **Present:** Drop current weapon. <!-- G02-INPUT-AND-016 -->
- **Present:** Hold-to-charge weapon throw. <!-- G02-INPUT-AND-017 -->
- **Present:** Throw combat knife. <!-- G02-INPUT-AND-018 -->
- **Present:** Pick up weapons. <!-- G02-INPUT-AND-019 -->
- **Present:** Pick up flags and kits. <!-- G02-INPUT-AND-020 -->
- **Present:** Flag throw using jump+crouch. <!-- G02-INPUT-AND-021 -->
- **Present:** Dedicated configurable flag-throw key. <!-- G02-INPUT-AND-022 -->
- **Present:** Respawn weapon-selection menu. <!-- G02-INPUT-AND-023 -->
- **Present:** Separate primary and secondary selection controls. <!-- G02-INPUT-AND-024 -->
- **Present:** Team chat. <!-- G02-INPUT-AND-025 -->
- **Present:** Command console. <!-- G02-INPUT-AND-026 -->
- **Present:** Scoreboard hold/toggle behavior. <!-- G02-INPUT-AND-027 -->
- **Present:** Weapon-statistics screen. <!-- G02-INPUT-AND-028 -->
- **Present:** Minimap toggle. <!-- G02-INPUT-AND-029 -->
- **Present:** Sniper-line toggle. <!-- G02-INPUT-AND-030 -->
- **Present:** Performance-statistics overlay. <!-- G02-INPUT-AND-031 -->
- **Present:** Screenshot control. <!-- G02-INPUT-AND-032 -->
- **Present:** Music toggle. <!-- G02-INPUT-AND-033 -->
- **Present:** Previous/next music track. <!-- G02-INPUT-AND-034 -->
- **Present:** Demo recording. <!-- G02-INPUT-AND-035 -->
- **Present:** Demo playback fast-forward. <!-- G02-INPUT-AND-036 -->
- **Present:** Pause. <!-- G02-INPUT-AND-037 -->
- **Present:** Window minimize shortcut. <!-- G02-INPUT-AND-038 -->
- **Present:** Taunt shortcuts. <!-- G02-INPUT-AND-039 -->
- **Present:** Runtime mouse-sensitivity adjustment. <!-- G02-INPUT-AND-040 -->
- **Present:** Runtime sound-volume adjustment. <!-- G02-INPUT-AND-041 -->
- **Present:** Scoreboard scrolling. <!-- G02-INPUT-AND-042 -->
- **Present:** Fully rebindable keyboard controls. <!-- G02-INPUT-AND-043 -->
- **Present:** Rebindable mouse buttons. <!-- G02-INPUT-AND-044 -->
- **Present:** Controller/gamepad support. <!-- G02-INPUT-AND-045 -->
- **Present:** Saved control profiles. <!-- G02-INPUT-AND-046 -->
- **Present:** Accessibility alternatives for combined inputs. <!-- G02-INPUT-AND-047 -->
- **Present:** Mobile equivalents for crouch, prone, roll, reload, weapon switch, weapon throw, flag throw, scoreboard, and team chat. <!-- G02-INPUT-AND-048 -->

## 3. Weapons

The project contains all 10 primary and 4 secondary weapon definitions, but many are statistical approximations or share simplified projectile behavior.

### Primary weapons requiring full parity

- **Present:** Desert Eagles. <!-- G03-WEAPONS-001 -->
- **Present:** HK MP5. <!-- G03-WEAPONS-002 -->
- **Present:** AK-74. <!-- G03-WEAPONS-003 -->
- **Present:** Steyr AUG. <!-- G03-WEAPONS-004 -->
- **Present:** SPAS-12. <!-- G03-WEAPONS-005 -->
- **Present:** Ruger 77. <!-- G03-WEAPONS-006 -->
- **Present:** M79. <!-- G03-WEAPONS-007 -->
- **Present:** Barrett M82A1. <!-- G03-WEAPONS-008 -->
- **Present:** FN Minimi. <!-- G03-WEAPONS-009 -->
- **Present:** XM214 Minigun. <!-- G03-WEAPONS-010 -->

### Secondary weapons requiring full parity

- **Present:** USSOCOM. <!-- G03-WEAPONS-011 -->
- **Present:** Combat Knife. <!-- G03-WEAPONS-012 -->
- **Present:** Chainsaw. <!-- G03-WEAPONS-013 -->
- **Present:** M72 LAW. <!-- G03-WEAPONS-014 -->

### Additional weapons missing

- **Present:** Cluster grenades. <!-- G03-WEAPONS-015 -->
- **Present:** Flamethrower. <!-- G03-WEAPONS-016 -->
- **Present:** Rambo Bow. <!-- G03-WEAPONS-017 -->
- **Present:** Normal arrows. <!-- G03-WEAPONS-018 -->
- **Present:** Flamed arrows. <!-- G03-WEAPONS-019 -->
- **Present:** Stationary M2 machine gun. <!-- G03-WEAPONS-020 -->
- **Present:** Punch/unarmed combat. <!-- G03-WEAPONS-021 -->

### Missing weapon systems

- **Present:** Primary-plus-secondary inventory slots. <!-- G03-WEAPONS-022 -->
- **Present:** Carrying two primary weapons. <!-- G03-WEAPONS-023 -->
- **Present:** Weapon pickups. <!-- G03-WEAPONS-024 -->
- **Present:** Weapon dropping. <!-- G03-WEAPONS-025 -->
- **Present:** Thrown-weapon physics. <!-- G03-WEAPONS-026 -->
- **Present:** Thrown combat knife. <!-- G03-WEAPONS-027 -->
- **Present:** Knife recovery/pickup. <!-- G03-WEAPONS-028 -->
- **Present:** Manual reload. <!-- G03-WEAPONS-029 -->
- **Present:** Reload interruption. <!-- G03-WEAPONS-030 -->
- **Present:** Per-weapon reload animations. <!-- G03-WEAPONS-031 -->
- **Present:** Weapon-switch delays. <!-- G03-WEAPONS-032 -->
- **Present:** Correct weapon startup behavior. <!-- G03-WEAPONS-033 -->
- **Present:** LAW firing restrictions. <!-- G03-WEAPONS-034 -->
- **Present:** Barrett movement/startup restrictions. <!-- G03-WEAPONS-035 -->
- **Present:** Minigun spin-up behavior. <!-- G03-WEAPONS-036 -->
- **Present:** Chainsaw continuous-contact behavior. <!-- G03-WEAPONS-037 -->
- **Present:** Proper melee collision. <!-- G03-WEAPONS-038 -->
- **Present:** Dual Desert Eagle projectiles and muzzle positions. <!-- G03-WEAPONS-039 -->
- **Present:** Proper shotgun pellet count and randomized spread. <!-- G03-WEAPONS-040 -->
- **Present:** Projectile lifetime matching Soldat. <!-- G03-WEAPONS-041 -->
- **Present:** Projectile gravity per bullet style. <!-- G03-WEAPONS-042 -->
- **Present:** Grenade bouncing. <!-- G03-WEAPONS-043 -->
- **Present:** M79 projectile bouncing/impact behavior. <!-- G03-WEAPONS-044 -->
- **Present:** Grenade cooking/throw strength. <!-- G03-WEAPONS-045 -->
- **Present:** Grenade fuse timing. <!-- G03-WEAPONS-046 -->
- **Present:** Dropped grenade behavior on death. <!-- G03-WEAPONS-047 -->
- **Present:** Cluster grenade submunition spawning. <!-- G03-WEAPONS-048 -->
- **Present:** Arrow sticking/interaction. <!-- G03-WEAPONS-049 -->
- **Present:** Flame propagation and burning. <!-- G03-WEAPONS-050 -->
- **Present:** Flamethrower fuel/ammunition behavior. <!-- G03-WEAPONS-051 -->
- **Present:** Stationary-gun mounting and dismounting. <!-- G03-WEAPONS-052 -->
- **Present:** Stationary-gun aiming limits. <!-- G03-WEAPONS-053 -->
- **Present:** Projectile-to-projectile or projectile-to-object interactions where applicable. <!-- G03-WEAPONS-054 -->
- **Present:** Muzzle origin based on character pose. <!-- G03-WEAPONS-055 -->
- **Present:** Muzzle flashes. <!-- G03-WEAPONS-056 -->
- **Present:** Shell casings. <!-- G03-WEAPONS-057 -->
- **Present:** Weapon-specific sounds. <!-- G03-WEAPONS-058 -->
- **Present:** Reload sounds. <!-- G03-WEAPONS-059 -->
- **Present:** Empty-magazine sound. <!-- G03-WEAPONS-060 -->
- **Present:** Bullet impact effects. <!-- G03-WEAPONS-061 -->
- **Present:** Tracers matching weapon configuration. <!-- G03-WEAPONS-062 -->
- **Present:** Explosion visual and audio effects. <!-- G03-WEAPONS-063 -->
- **Present:** Weapon sprites held by characters. <!-- G03-WEAPONS-064 -->
- **Present:** Weapons lying on the ground. <!-- G03-WEAPONS-065 -->
- **Present:** Weapon pickup indicators. <!-- G03-WEAPONS-066 -->

[Weapons reference](https://wiki.soldat.pl/index.php/Weapons)

## 4. Weapon Mod parity

The project currently stores only a subset of Soldat’s weapon configuration fields.

Every weapon needs support for:

- **Present:** Damage. <!-- G04-WEAPON-MOD-001 -->
- **Present:** Fire interval. <!-- G04-WEAPON-MOD-002 -->
- **Present:** Ammunition capacity. <!-- G04-WEAPON-MOD-003 -->
- **Present:** Reload time. <!-- G04-WEAPON-MOD-004 -->
- **Present:** Projectile speed. <!-- G04-WEAPON-MOD-005 -->
- **Present:** Bullet style. <!-- G04-WEAPON-MOD-006 -->
- **Present:** Startup time. <!-- G04-WEAPON-MOD-007 -->
- **Present:** Bink. <!-- G04-WEAPON-MOD-008 -->
- **Present:** Self-bink through negative bink values. <!-- G04-WEAPON-MOD-009 -->
- **Present:** Movement accuracy. <!-- G04-WEAPON-MOD-010 -->
- **Present:** Bullet spread. <!-- G04-WEAPON-MOD-011 -->
- **Present:** Recoil. <!-- G04-WEAPON-MOD-012 -->
- **Present:** Push. <!-- G04-WEAPON-MOD-013 -->
- **Present:** Inherited velocity. <!-- G04-WEAPON-MOD-014 -->
- **Present:** Head damage modifier. <!-- G04-WEAPON-MOD-015 -->
- **Present:** Chest damage modifier. <!-- G04-WEAPON-MOD-016 -->
- **Present:** Leg damage modifier. <!-- G04-WEAPON-MOD-017 -->
- **Present:** Separate normal and Realistic weapon tables. <!-- G04-WEAPON-MOD-018 -->
- **Present:** Cluster-grenade nesting under frag grenades. <!-- G04-WEAPON-MOD-019 -->
- **Present:** Server-selected/custom weapon mods. <!-- G04-WEAPON-MOD-020 -->
- **Present:** Weapon-mod synchronization with clients. <!-- G04-WEAPON-MOD-021 -->
- **Present:** Validation of custom weapon values. <!-- G04-WEAPON-MOD-022 -->
- **Present:** A safe loader for `weapons.ini`-style data. <!-- G04-WEAPON-MOD-023 -->
- **Present:** Version/hash checking so clients know which mod is active. <!-- G04-WEAPON-MOD-024 -->
- **Present:** Weapon-mod display in the room/server browser. <!-- G04-WEAPON-MOD-025 -->

### Damage and accuracy corrections

- **Present:** Replace simplified damage with `Damage × CurrentSpeed × HitboxModifier`. <!-- G04-WEAPON-MOD-026 -->
- **Present:** Add projectile speed decay. <!-- G04-WEAPON-MOD-027 -->
- **Present:** Add player velocity inheritance. <!-- G04-WEAPON-MOD-028 -->
- **Present:** Allow movement direction to increase or decrease projectile speed and damage. <!-- G04-WEAPON-MOD-029 -->
- **Present:** Add head, chest, and leg hit detection. <!-- G04-WEAPON-MOD-030 -->
- **Present:** Add bink when the player is hit. <!-- G04-WEAPON-MOD-031 -->
- **Present:** Add self-bink when firing relevant weapons. <!-- G04-WEAPON-MOD-032 -->
- **Present:** Add cursor expansion representing current accuracy. <!-- G04-WEAPON-MOD-033 -->
- **Present:** Add movement accuracy penalties. <!-- G04-WEAPON-MOD-034 -->
- **Present:** Add stronger jetting accuracy penalties. <!-- G04-WEAPON-MOD-035 -->
- **Present:** Add recoil to the aim/cursor. <!-- G04-WEAPON-MOD-036 -->
- **Present:** Add correct bullet spread. <!-- G04-WEAPON-MOD-037 -->
- **Present:** Add bullet mass/push behavior. <!-- G04-WEAPON-MOD-038 -->
- **Present:** Add distance-dependent damage caused by changing projectile speed. <!-- G04-WEAPON-MOD-039 -->
- **Present:** Add exact explosive damage and falloff. <!-- G04-WEAPON-MOD-040 -->
- **Present:** Add terrain occlusion for splash damage if Soldat’s source behavior requires it. <!-- G04-WEAPON-MOD-041 -->
- **Present:** Add friendly-fire and team-bink rules. <!-- G04-WEAPON-MOD-042 -->

The pinned OpenSoldat splash path has no line-of-sight check, so explosions are not occluded by terrain.

[Weapon Mod reference](https://wiki.soldat.pl/index.php/Weapon_Mod)

## 5. Game modes

### Existing

- **Present:** Deathmatch. <!-- G05-GAME-MODES-001 -->
- **Present:** Team Deathmatch, corresponding to Soldat’s Teammatch. <!-- G05-GAME-MODES-002 -->

These modes still lack round limits, time limits, proper spawning, map rotation, end-of-round state, announcements, and complete scoring.

### Missing official modes

- **Present:** Pointmatch. <!-- G05-GAME-MODES-003 -->
- **Present:** Rambomatch. <!-- G05-GAME-MODES-004 -->
- **Present:** Capture the Flag. <!-- G05-GAME-MODES-005 -->
- **Present:** Infiltration. <!-- G05-GAME-MODES-006 -->
- **Present:** Hold the Flag. <!-- G05-GAME-MODES-007 -->

### Missing mode modifiers

- **Present:** Realistic mode. <!-- G05-GAME-MODES-008 -->
- **Present:** Survival mode. <!-- G05-GAME-MODES-009 -->
- **Present:** Advance mode. <!-- G05-GAME-MODES-010 -->

### Missing community modes mentioned by the wiki

These are not necessary for initial Soldat parity, but they belong in the exhaustive feature backlog:

- **Present:** Climb. <!-- G05-GAME-MODES-011 -->
- **Present:** Dodgeball. <!-- G05-GAME-MODES-012 -->
- **Present:** Domination. <!-- G05-GAME-MODES-013 -->
- **Present:** Hide and Seek. <!-- G05-GAME-MODES-014 -->
- **Present:** Knife Only. <!-- G05-GAME-MODES-015 -->
- **Present:** OneShots. <!-- G05-GAME-MODES-016 -->
- **Present:** Pirates vs Ninjas. <!-- G05-GAME-MODES-017 -->
- **Present:** Realistic Soldat/Counter-Strike. <!-- G05-GAME-MODES-018 -->
- **Present:** Trench Wars. <!-- G05-GAME-MODES-019 -->
- **Present:** Tactical Trench Wars. <!-- G05-GAME-MODES-020 -->
- **Present:** Zombie. <!-- G05-GAME-MODES-021 -->

### Missing match rules

- **Present:** Kill limit. <!-- G05-GAME-MODES-022 -->
- **Present:** Point limit. <!-- G05-GAME-MODES-023 -->
- **Present:** Capture limit. <!-- G05-GAME-MODES-024 -->
- **Present:** Time limit. <!-- G05-GAME-MODES-025 -->
- **Present:** Round start countdown. <!-- G05-GAME-MODES-026 -->
- **Present:** Round end. <!-- G05-GAME-MODES-027 -->
- **Present:** Winner calculation. <!-- G05-GAME-MODES-028 -->
- **Present:** Draw handling. <!-- G05-GAME-MODES-029 -->
- **Present:** Overtime policy where appropriate. <!-- G05-GAME-MODES-030 -->
- **Present:** Map rotation. <!-- G05-GAME-MODES-031 -->
- **Present:** Map-loop option. <!-- G05-GAME-MODES-032 -->
- **Present:** Next-map transition. <!-- G05-GAME-MODES-033 -->
- **Present:** Match restart. <!-- G05-GAME-MODES-034 -->
- **Present:** Configurable respawn time. <!-- G05-GAME-MODES-035 -->
- **Present:** Survival round elimination. <!-- G05-GAME-MODES-036 -->
- **Present:** Survival dead-player restrictions. <!-- G05-GAME-MODES-037 -->
- **Present:** Advance-mode weapon unlocking. <!-- G05-GAME-MODES-038 -->
- **Present:** Team balancing. <!-- G05-GAME-MODES-039 -->
- **Present:** Team selection. <!-- G05-GAME-MODES-040 -->
- **Present:** Spectator team. <!-- G05-GAME-MODES-041 -->
- **Present:** Mid-match spectator switching. <!-- G05-GAME-MODES-042 -->
- **Present:** Friendly-fire configuration. <!-- G05-GAME-MODES-043 -->
- **Present:** Teamkill handling. <!-- G05-GAME-MODES-044 -->
- **Present:** Suicide scoring. <!-- G05-GAME-MODES-045 -->
- **Present:** Disconnect/reconnect score preservation. <!-- G05-GAME-MODES-046 -->
- **Present:** Automatic round-end scoreboard. <!-- G05-GAME-MODES-047 -->
- **Present:** Automatic final-score screenshot option. <!-- G05-GAME-MODES-048 -->

[Game modes reference](https://wiki.soldat.pl/index.php/Game_Modes)

## 6. Pointmatch

- **Present:** Yellow point flag. <!-- G06-POINTMATCH-001 -->
- **Present:** Holding the point flag. <!-- G06-POINTMATCH-002 -->
- **Present:** Extra points awarded while holding it. <!-- G06-POINTMATCH-003 -->
- **Present:** Flag drop on death. <!-- G06-POINTMATCH-004 -->
- **Present:** Point-limit victory. <!-- G06-POINTMATCH-005 -->
- **Present:** Pointmatch-specific scoring. <!-- G06-POINTMATCH-006 -->
- **Present:** Pointmatch spawns and maps. <!-- G06-POINTMATCH-007 -->
- **Present:** Point-flag HUD status. <!-- G06-POINTMATCH-008 -->

## 7. Rambomatch

- **Present:** Rambo Bow spawn. <!-- G07-RAMBOMATCH-001 -->
- **Present:** Bow pickup. <!-- G07-RAMBOMATCH-002 -->
- **Present:** Only the Rambo player earning kills/points under the mode’s rules. <!-- G07-RAMBOMATCH-003 -->
- **Present:** Rambo target indication. <!-- G07-RAMBOMATCH-004 -->
- **Present:** Bow drop and reacquisition. <!-- G07-RAMBOMATCH-005 -->
- **Present:** Rambo-specific respawn behavior. <!-- G07-RAMBOMATCH-006 -->
- **Present:** Flamed-arrow support. <!-- G07-RAMBOMATCH-007 -->
- **Present:** Rambomatch scoring and win limit. <!-- G07-RAMBOMATCH-008 -->
- **Present:** Rambomatch HUD. <!-- G07-RAMBOMATCH-009 -->

## 8. Capture the Flag

- **Present:** Alpha and Bravo teams. <!-- G08-CAPTURE-THE-001 -->
- **Present:** Red and blue flags. <!-- G08-CAPTURE-THE-002 -->
- **Present:** Flag bases. <!-- G08-CAPTURE-THE-003 -->
- **Present:** Enemy-flag pickup. <!-- G08-CAPTURE-THE-004 -->
- **Present:** Flag carrying. <!-- G08-CAPTURE-THE-005 -->
- **Present:** Flag dropping on death. <!-- G08-CAPTURE-THE-006 -->
- **Present:** Manual flag throw. <!-- G08-CAPTURE-THE-007 -->
- **Present:** Flag return by touching a dropped friendly flag. <!-- G08-CAPTURE-THE-008 -->
- **Present:** Automatic return timeout if applicable. <!-- G08-CAPTURE-THE-009 -->
- **Present:** Capture only when the player’s own flag is at base. <!-- G08-CAPTURE-THE-010 -->
- **Present:** Capture scoring. <!-- G08-CAPTURE-THE-011 -->
- **Present:** Capture limit. <!-- G08-CAPTURE-THE-012 -->
- **Present:** Flag-carrier indicator. <!-- G08-CAPTURE-THE-013 -->
- **Present:** Missing-flag indicator. <!-- G08-CAPTURE-THE-014 -->
- **Present:** Flag status HUD. <!-- G08-CAPTURE-THE-015 -->
- **Present:** Team score HUD. <!-- G08-CAPTURE-THE-016 -->
- **Present:** Flag physics. <!-- G08-CAPTURE-THE-017 -->
- **Present:** Bullets and explosions pushing flags. <!-- G08-CAPTURE-THE-018 -->
- **Present:** Flag collision with polygons. <!-- G08-CAPTURE-THE-019 -->
- **Present:** CTF spawn points. <!-- G08-CAPTURE-THE-020 -->
- **Present:** CTF-compatible map validation. <!-- G08-CAPTURE-THE-021 -->
- **Present:** CTF bots and flag objectives. <!-- G08-CAPTURE-THE-022 -->

## 9. Infiltration

- **Present:** Attacking and defending teams. <!-- G09-INFILTRATION-001 -->
- **Present:** Black/white or objective-specific flags. <!-- G09-INFILTRATION-002 -->
- **Present:** Objective capture rules. <!-- G09-INFILTRATION-003 -->
- **Present:** Passive defender scoring. <!-- G09-INFILTRATION-004 -->
- **Present:** Attacker capture scoring. <!-- G09-INFILTRATION-005 -->
- **Present:** Team-role asymmetry. <!-- G09-INFILTRATION-006 -->
- **Present:** Infiltration-specific spawn points. <!-- G09-INFILTRATION-007 -->
- **Present:** Infiltration timer and score rules. <!-- G09-INFILTRATION-008 -->
- **Present:** Team score HUD. <!-- G09-INFILTRATION-009 -->
- **Present:** Objective state indicators. <!-- G09-INFILTRATION-010 -->
- **Present:** Infiltration bot behavior. <!-- G09-INFILTRATION-011 -->

## 10. Hold the Flag

- **Present:** Neutral yellow flag. <!-- G10-HOLD-THE-001 -->
- **Present:** Flag pickup and carrying. <!-- G10-HOLD-THE-002 -->
- **Present:** Continuous team scoring while held. <!-- G10-HOLD-THE-003 -->
- **Present:** Flag drops. <!-- G10-HOLD-THE-004 -->
- **Present:** Flag return/reset rules. <!-- G10-HOLD-THE-005 -->
- **Present:** Carrier indication. <!-- G10-HOLD-THE-006 -->
- **Present:** HTF-specific spawn points. <!-- G10-HOLD-THE-007 -->
- **Present:** HTF score display. <!-- G10-HOLD-THE-008 -->
- **Present:** HTF bots and objective behavior. <!-- G10-HOLD-THE-009 -->

## 11. Realistic mode

- **Present:** Separate `weapons_realistic.ini` statistics. <!-- G11-REALISTIC-MODE-001 -->
- **Present:** Reduced/changed weapon damage behavior. <!-- G11-REALISTIC-MODE-002 -->
- **Present:** Recoil behavior appropriate to Realistic. <!-- G11-REALISTIC-MODE-003 -->
- **Present:** Visibility/line-of-sight restrictions. <!-- G11-REALISTIC-MODE-004 -->
- **Present:** Enemies visible only when the observed player can see them. <!-- G11-REALISTIC-MODE-005 -->
- **Present:** Dead-player and spectator visibility restrictions. <!-- G11-REALISTIC-MODE-006 -->
- **Present:** Enemy team-chat visibility restrictions. <!-- G11-REALISTIC-MODE-007 -->
- **Present:** Fall damage if required by the reference implementation. <!-- G11-REALISTIC-MODE-008 -->
- **Present:** Realistic movement and survival tuning. <!-- G11-REALISTIC-MODE-009 -->
- **Present:** Realistic-specific HUD behavior. <!-- G11-REALISTIC-MODE-010 -->
- **Present:** Server/room Realistic flag. <!-- G11-REALISTIC-MODE-011 -->

## 12. Survival mode

- **Present:** No immediate respawn. <!-- G12-SURVIVAL-MODE-001 -->
- **Present:** Round-based respawning. <!-- G12-SURVIVAL-MODE-002 -->
- **Present:** Round begins when enough players are ready. <!-- G12-SURVIVAL-MODE-003 -->
- **Present:** Round ends when one player/team remains. <!-- G12-SURVIVAL-MODE-004 -->
- **Present:** Dead players spectate. <!-- G12-SURVIVAL-MODE-005 -->
- **Present:** Survival scoreboard. <!-- G12-SURVIVAL-MODE-006 -->
- **Present:** End-of-round state. <!-- G12-SURVIVAL-MODE-007 -->
- **Present:** Flag restrictions after a Survival round ends. <!-- G12-SURVIVAL-MODE-008 -->
- **Present:** Survival chat/spectator restrictions. <!-- G12-SURVIVAL-MODE-009 -->
- **Present:** Configurable survival respawn/round behavior. <!-- G12-SURVIVAL-MODE-010 -->

## 13. Advance mode

- **Present:** Initial limited weapon selection. <!-- G13-ADVANCE-MODE-001 -->
- **Present:** Unlock weapons through kills. <!-- G13-ADVANCE-MODE-002 -->
- **Present:** Unlock progression. <!-- G13-ADVANCE-MODE-003 -->
- **Present:** Per-player unlock state. <!-- G13-ADVANCE-MODE-004 -->
- **Present:** Advance weapon menu. <!-- G13-ADVANCE-MODE-005 -->
- **Present:** Reset progression between matches/maps as appropriate. <!-- G13-ADVANCE-MODE-006 -->
- **Present:** Advance configuration. <!-- G13-ADVANCE-MODE-007 -->

## 14. Bonus kits

All bonus kits and spawning logic are missing:

- **Present:** Medic Kit: restore health to maximum. <!-- G14-BONUS-KITS-001 -->
- **Present:** Grenades Kit: restore grenades to configured maximum. <!-- G14-BONUS-KITS-002 -->
- **Present:** Cluster Grenades Kit: grant three cluster grenades. <!-- G14-BONUS-KITS-003 -->
- **Present:** Bulletproof Vest Kit: add approximately another full health bar as armor. <!-- G14-BONUS-KITS-004 -->
- **Present:** Flame God Kit: flamethrower plus temporary invulnerability. <!-- G14-BONUS-KITS-005 -->
- **Present:** Berserker Kit: four-times weapon damage temporarily. <!-- G14-BONUS-KITS-006 -->
- **Present:** Predator Kit: temporary invisibility. <!-- G14-BONUS-KITS-007 -->

Supporting systems still needed:

- **Present:** Bonus spawn points. <!-- G14-BONUS-KITS-008 -->
- **Present:** Configurable kit frequency. <!-- G14-BONUS-KITS-009 -->
- **Present:** Kit respawn timers. <!-- G14-BONUS-KITS-010 -->
- **Present:** Pickup collision. <!-- G14-BONUS-KITS-011 -->
- **Present:** Pickup sounds and effects. <!-- G14-BONUS-KITS-012 -->
- **Present:** Active-bonus HUD. <!-- G14-BONUS-KITS-013 -->
- **Present:** Bonus countdown. <!-- G14-BONUS-KITS-014 -->
- **Present:** Bonus overlay/effect. <!-- G14-BONUS-KITS-015 -->
- **Present:** Armor HUD. <!-- G14-BONUS-KITS-016 -->
- **Present:** Predator visibility affected by blood. <!-- G14-BONUS-KITS-017 -->
- **Present:** Predator still producing audible sounds. <!-- G14-BONUS-KITS-018 -->
- **Present:** Bonus expiration. <!-- G14-BONUS-KITS-019 -->
- **Present:** Bonus replacement/stacking rules. <!-- G14-BONUS-KITS-020 -->
- **Present:** Server enable/disable settings. <!-- G14-BONUS-KITS-021 -->
- **Present:** Kits affected by projectile push. <!-- G14-BONUS-KITS-022 -->

[Bonuses reference](https://wiki.soldat.pl/index.php/Bonuses)

## 15. Health, armor, death, and respawn

- **Present:** Basic 100 HP. <!-- G15-HEALTH-ARMOR-001 -->
- **Present:** Death count. <!-- G15-HEALTH-ARMOR-002 -->
- **Present:** Automatic respawn. <!-- G15-HEALTH-ARMOR-003 -->
- **Present:** Body-part damage. <!-- G15-HEALTH-ARMOR-004 -->
- **Present:** Bulletproof vest/armor. <!-- G15-HEALTH-ARMOR-005 -->
- **Present:** Bleeding. <!-- G15-HEALTH-ARMOR-006 -->
- **Present:** Blood particles. <!-- G15-HEALTH-ARMOR-007 -->
- **Present:** Blood remaining on the character. <!-- G15-HEALTH-ARMOR-008 -->
- **Present:** Gore/gibs. <!-- G15-HEALTH-ARMOR-009 -->
- **Present:** Ragdoll corpses. <!-- G15-HEALTH-ARMOR-010 -->
- **Present:** Corpse persistence. <!-- G15-HEALTH-ARMOR-011 -->
- **Present:** Death causes. <!-- G15-HEALTH-ARMOR-012 -->
- **Present:** Kill feed. <!-- G15-HEALTH-ARMOR-013 -->
- **Present:** Headshot messaging/effects. <!-- G15-HEALTH-ARMOR-014 -->
- **Present:** Multi-kill messages. <!-- G15-HEALTH-ARMOR-015 -->
- **Present:** Self-kill messages. <!-- G15-HEALTH-ARMOR-016 -->
- **Present:** Teamkill messages. <!-- G15-HEALTH-ARMOR-017 -->
- **Present:** Spawn protection if applicable. <!-- G15-HEALTH-ARMOR-018 -->
- **Present:** Configurable respawn time. <!-- G15-HEALTH-ARMOR-019 -->
- **Present:** Respawn countdown HUD. <!-- G15-HEALTH-ARMOR-020 -->
- **Present:** Weapon selection while dead. <!-- G15-HEALTH-ARMOR-021 -->
- **Present:** Proper mode/team/map spawn selection. <!-- G15-HEALTH-ARMOR-022 -->
- **Present:** Kill attribution after delayed damage. <!-- G15-HEALTH-ARMOR-023 -->
- **Present:** Assist tracking if desired. <!-- G15-HEALTH-ARMOR-024 -->
- **Present:** Damage direction feedback. <!-- G15-HEALTH-ARMOR-025 -->
- **Present:** Explosion deafness/whistling effect. <!-- G15-HEALTH-ARMOR-026 -->

## 16. Maps and terrain engine

The present arena is not a Soldat map. Add:

- **Present:** `.pms` map loader. <!-- G16-MAPS-AND-001 -->
- **Present:** PMS format validation. <!-- G16-MAPS-AND-002 -->
- **Present:** Polygon geometry. <!-- G16-MAPS-AND-003 -->
- **Present:** Polygon types and properties. <!-- G16-MAPS-AND-004 -->
- **Present:** Texture coordinates. <!-- G16-MAPS-AND-005 -->
- **Present:** Map textures. <!-- G16-MAPS-AND-006 -->
- **Present:** Edge textures. <!-- G16-MAPS-AND-007 -->
- **Present:** Scenery objects. <!-- G16-MAPS-AND-008 -->
- **Present:** Scenery depth/layers. <!-- G16-MAPS-AND-009 -->
- **Present:** Animated scenery if supported. <!-- G16-MAPS-AND-010 -->
- **Present:** Colliders. <!-- G16-MAPS-AND-011 -->
- **Present:** Spawn points. <!-- G16-MAPS-AND-012 -->
- **Present:** Player spawn types. <!-- G16-MAPS-AND-013 -->
- **Present:** Team spawn types. <!-- G16-MAPS-AND-014 -->
- **Present:** Flag spawn types. <!-- G16-MAPS-AND-015 -->
- **Present:** Bonus-kit spawn types. <!-- G16-MAPS-AND-016 -->
- **Present:** Grenade spawn types where relevant. <!-- G16-MAPS-AND-017 -->
- **Present:** Stationary-gun locations. <!-- G16-MAPS-AND-018 -->
- **Present:** Bot waypoints. <!-- G16-MAPS-AND-019 -->
- **Present:** Background colors and gradients. <!-- G16-MAPS-AND-020 -->
- **Present:** Weather settings. <!-- G16-MAPS-AND-021 -->
- **Present:** Footstep-sound property. <!-- G16-MAPS-AND-022 -->
- **Present:** Map-specific jet fuel. <!-- G16-MAPS-AND-023 -->
- **Present:** Map boundaries. <!-- G16-MAPS-AND-024 -->
- **Present:** Death/out-of-bounds areas. <!-- G16-MAPS-AND-025 -->
- **Present:** Map metadata. <!-- G16-MAPS-AND-026 -->
- **Present:** Map version compatibility. <!-- G16-MAPS-AND-027 -->
- **Present:** Custom-map downloading. <!-- G16-MAPS-AND-028 -->
- **Present:** Texture/scenery downloading. <!-- G16-MAPS-AND-029 -->
- **Present:** Download progress and cancellation. <!-- G16-MAPS-AND-030 -->
- **Present:** Missing-asset handling. <!-- G16-MAPS-AND-031 -->
- **Present:** Map checksum verification. <!-- G16-MAPS-AND-032 -->
- **Present:** Map rotation file/configuration. <!-- G16-MAPS-AND-033 -->
- **Present:** Mode-prefix recognition: `ctf_`, `inf_`, `htf_`, and community prefixes. <!-- G16-MAPS-AND-034 -->
- **Present:** Empty/invalid map-list handling. <!-- G16-MAPS-AND-035 -->
- **Present:** Map voting or polling if the intended Soldat server experience includes it. <!-- G16-MAPS-AND-036 -->
- **Present:** Client caching of maps/assets. <!-- G16-MAPS-AND-037 -->
- **Present:** Map preview images. <!-- G16-MAPS-AND-038 -->
- **Present:** Map selection UI. <!-- G16-MAPS-AND-039 -->
- **Present:** Offline map testing. <!-- G16-MAPS-AND-040 -->

[Map structure summary](https://wiki.soldat.pl/index.php/Map)

## 17. All 97 referenced default maps

None of these maps currently exists in the project.

### Deathmatch, Teammatch, Rambomatch, and Pointmatch — 29

- **Present:** Aero <!-- G17-ALL-97-001 -->
- **Present:** Airpirates <!-- G17-ALL-97-002 -->
- **Present:** Arena <!-- G17-ALL-97-003 -->
- **Present:** Arena2 <!-- G17-ALL-97-004 -->
- **Present:** Arena3 <!-- G17-ALL-97-005 -->
- **Present:** Bigfalls <!-- G17-ALL-97-006 -->
- **Present:** Blox <!-- G17-ALL-97-007 -->
- **Present:** Bridge <!-- G17-ALL-97-008 -->
- **Present:** Bunker <!-- G17-ALL-97-009 -->
- **Present:** Cambodia <!-- G17-ALL-97-010 -->
- **Present:** CrackedBoot <!-- G17-ALL-97-011 -->
- **Present:** Daybreak <!-- G17-ALL-97-012 -->
- **Present:** DesertWind <!-- G17-ALL-97-013 -->
- **Present:** Factory <!-- G17-ALL-97-014 -->
- **Present:** Flashback <!-- G17-ALL-97-015 -->
- **Present:** HH <!-- G17-ALL-97-016 -->
- **Present:** Island2k5 <!-- G17-ALL-97-017 -->
- **Present:** Jungle <!-- G17-ALL-97-018 -->
- **Present:** Krab <!-- G17-ALL-97-019 -->
- **Present:** Lagrange <!-- G17-ALL-97-020 -->
- **Present:** Leaf <!-- G17-ALL-97-021 -->
- **Present:** MrSnowman <!-- G17-ALL-97-022 -->
- **Present:** RatCave <!-- G17-ALL-97-023 -->
- **Present:** Rok <!-- G17-ALL-97-024 -->
- **Present:** RR <!-- G17-ALL-97-025 -->
- **Present:** Shau <!-- G17-ALL-97-026 -->
- **Present:** Tropiccave <!-- G17-ALL-97-027 -->
- **Present:** Unlim <!-- G17-ALL-97-028 -->
- **Present:** Veoto <!-- G17-ALL-97-029 -->

### Capture the Flag — 32

- **Present:** ctf_Ash <!-- G17-ALL-97-030 -->
- **Present:** ctf_B2b <!-- G17-ALL-97-031 -->
- **Present:** ctf_Blade <!-- G17-ALL-97-032 -->
- **Present:** ctf_Campeche <!-- G17-ALL-97-033 -->
- **Present:** ctf_Cobra <!-- G17-ALL-97-034 -->
- **Present:** ctf_Crucifix <!-- G17-ALL-97-035 -->
- **Present:** ctf_Death <!-- G17-ALL-97-036 -->
- **Present:** ctf_Division <!-- G17-ALL-97-037 -->
- **Present:** ctf_Dropdown <!-- G17-ALL-97-038 -->
- **Present:** ctf_Equinox <!-- G17-ALL-97-039 -->
- **Present:** ctf_Guardian <!-- G17-ALL-97-040 -->
- **Present:** ctf_Hormone <!-- G17-ALL-97-041 -->
- **Present:** ctf_IceBeam <!-- G17-ALL-97-042 -->
- **Present:** ctf_Kampf <!-- G17-ALL-97-043 -->
- **Present:** ctf_Lanubya <!-- G17-ALL-97-044 -->
- **Present:** ctf_Laos <!-- G17-ALL-97-045 -->
- **Present:** ctf_Maya <!-- G17-ALL-97-046 -->
- **Present:** ctf_Mayapan <!-- G17-ALL-97-047 -->
- **Present:** ctf_MFM <!-- G17-ALL-97-048 -->
- **Present:** ctf_Nuubia <!-- G17-ALL-97-049 -->
- **Present:** ctf_Raspberry <!-- G17-ALL-97-050 -->
- **Present:** ctf_Rotten <!-- G17-ALL-97-051 -->
- **Present:** ctf_Ruins <!-- G17-ALL-97-052 -->
- **Present:** ctf_Run <!-- G17-ALL-97-053 -->
- **Present:** ctf_Scorpion <!-- G17-ALL-97-054 -->
- **Present:** ctf_Snakebite <!-- G17-ALL-97-055 -->
- **Present:** ctf_Steel <!-- G17-ALL-97-056 -->
- **Present:** ctf_Triumph <!-- G17-ALL-97-057 -->
- **Present:** ctf_Viet <!-- G17-ALL-97-058 -->
- **Present:** ctf_Voland <!-- G17-ALL-97-059 -->
- **Present:** ctf_Wretch <!-- G17-ALL-97-060 -->
- **Present:** ctf_X <!-- G17-ALL-97-061 -->

### Hold the Flag — 19

- **Present:** htf_Arch <!-- G17-ALL-97-062 -->
- **Present:** htf_Baire <!-- G17-ALL-97-063 -->
- **Present:** htf_Boxed <!-- G17-ALL-97-064 -->
- **Present:** htf_Desert <!-- G17-ALL-97-065 -->
- **Present:** htf_Dorothy <!-- G17-ALL-97-066 -->
- **Present:** htf_Dusk <!-- G17-ALL-97-067 -->
- **Present:** htf_Erbium <!-- G17-ALL-97-068 -->
- **Present:** htf_Feast <!-- G17-ALL-97-069 -->
- **Present:** htf_Mossy <!-- G17-ALL-97-070 -->
- **Present:** htf_Muygen <!-- G17-ALL-97-071 -->
- **Present:** htf_Niall <!-- G17-ALL-97-072 -->
- **Present:** htf_Nuclear <!-- G17-ALL-97-073 -->
- **Present:** htf_Prison <!-- G17-ALL-97-074 -->
- **Present:** htf_Rubik <!-- G17-ALL-97-075 -->
- **Present:** htf_Star <!-- G17-ALL-97-076 -->
- **Present:** htf_Tower <!-- G17-ALL-97-077 -->
- **Present:** htf_Void <!-- G17-ALL-97-078 -->
- **Present:** htf_Vortex <!-- G17-ALL-97-079 -->
- **Present:** htf_Zajacz <!-- G17-ALL-97-080 -->

### Infiltration — 17

- **Present:** inf_Abel <!-- G17-ALL-97-081 -->
- **Present:** inf_April <!-- G17-ALL-97-082 -->
- **Present:** inf_Argy <!-- G17-ALL-97-083 -->
- **Present:** inf_Belltower <!-- G17-ALL-97-084 -->
- **Present:** inf_Biologic <!-- G17-ALL-97-085 -->
- **Present:** inf_Changeling <!-- G17-ALL-97-086 -->
- **Present:** inf_Flute <!-- G17-ALL-97-087 -->
- **Present:** inf_Fortress <!-- G17-ALL-97-088 -->
- **Present:** inf_Industrial <!-- G17-ALL-97-089 -->
- **Present:** inf_Messner <!-- G17-ALL-97-090 -->
- **Present:** inf_Moonshine <!-- G17-ALL-97-091 -->
- **Present:** inf_Motheaten <!-- G17-ALL-97-092 -->
- **Present:** inf_Outpost <!-- G17-ALL-97-093 -->
- **Present:** inf_Rescue <!-- G17-ALL-97-094 -->
- **Present:** inf_Rise <!-- G17-ALL-97-095 -->
- **Present:** inf_Warehouse <!-- G17-ALL-97-096 -->
- **Present:** inf_Warlock <!-- G17-ALL-97-097 -->

Important: the names and layouts are reference targets, but original maps, textures, sounds, and art must not be copied until their licenses and reuse permissions are verified. The repository already recognizes this provenance issue.

[Default maps reference](https://wiki.soldat.pl/index.php/Default_Maps)

## 18. Bots and AI

- **Present:** Bot entities driven by the authoritative simulation. <!-- G18-BOTS-AND-001 -->
- **Present:** Random-bot count. <!-- G18-BOTS-AND-002 -->
- **Present:** Per-team bot count. <!-- G18-BOTS-AND-003 -->
- **Present:** Bot difficulty. <!-- G18-BOTS-AND-004 -->
- **Present:** Bot accuracy levels. <!-- G18-BOTS-AND-005 -->
- **Present:** Bot reaction time. <!-- G18-BOTS-AND-006 -->
- **Present:** Bot movement. <!-- G18-BOTS-AND-007 -->
- **Present:** Jet navigation. <!-- G18-BOTS-AND-008 -->
- **Present:** Crouching, prone, rolls, and backflips. <!-- G18-BOTS-AND-009 -->
- **Present:** Weapon selection. <!-- G18-BOTS-AND-010 -->
- **Present:** Reloading. <!-- G18-BOTS-AND-011 -->
- **Present:** Grenade use. <!-- G18-BOTS-AND-012 -->
- **Present:** Weapon pickup. <!-- G18-BOTS-AND-013 -->
- **Present:** Bonus pickup. <!-- G18-BOTS-AND-014 -->
- **Present:** Waypoint navigation. <!-- G18-BOTS-AND-015 -->
- **Present:** Recovery when stuck. <!-- G18-BOTS-AND-016 -->
- **Present:** Deathmatch target selection. <!-- G18-BOTS-AND-017 -->
- **Present:** Team coordination. <!-- G18-BOTS-AND-018 -->
- **Present:** CTF attacking, defending, returning, and capturing. <!-- G18-BOTS-AND-019 -->
- **Present:** Infiltration objectives. <!-- G18-BOTS-AND-020 -->
- **Present:** HTF carrier support. <!-- G18-BOTS-AND-021 -->
- **Present:** Rambomatch behavior. <!-- G18-BOTS-AND-022 -->
- **Present:** Survival behavior. <!-- G18-BOTS-AND-023 -->
- **Present:** Bot chat. <!-- G18-BOTS-AND-024 -->
- **Present:** Custom bot profiles. <!-- G18-BOTS-AND-025 -->
- **Present:** Server commands to add/remove bots. <!-- G18-BOTS-AND-026 -->

## 19. HUD and game screen

### Existing or partial

- **Present:** Numeric health. <!-- G19-HUD-AND-001 -->
- **Present:** Numeric jet fuel. <!-- G19-HUD-AND-002 -->
- **Present:** Ammo and grenade count. <!-- G19-HUD-AND-003 -->
- **Present:** Kills/deaths. <!-- G19-HUD-AND-004 -->
- **Present:** Scoreboard. <!-- G19-HUD-AND-005 -->
- **Present:** Team coloring. <!-- G19-HUD-AND-006 -->

### Missing Soldat HUD elements

- **Present:** Red health bar. <!-- G19-HUD-AND-007 -->
- **Present:** Yellow ammunition/reload bar. <!-- G19-HUD-AND-008 -->
- **Present:** Bullet count positioned with the ammo display. <!-- G19-HUD-AND-009 -->
- **Present:** Fire-interval bar. <!-- G19-HUD-AND-010 -->
- **Present:** Blue jet-fuel bar. <!-- G19-HUD-AND-011 -->
- **Present:** Server rank. <!-- G19-HUD-AND-012 -->
- **Present:** Current kills/points. <!-- G19-HUD-AND-013 -->
- **Present:** Difference from the leader. <!-- G19-HUD-AND-014 -->
- **Present:** Kill/point/capture limit. <!-- G19-HUD-AND-015 -->
- **Present:** Alpha team score. <!-- G19-HUD-AND-016 -->
- **Present:** Bravo team score. <!-- G19-HUD-AND-017 -->
- **Present:** Charlie team score. <!-- G19-HUD-AND-018 -->
- **Present:** Delta team score. <!-- G19-HUD-AND-019 -->
- **Present:** Missing-flag indicators. <!-- G19-HUD-AND-020 -->
- **Present:** Flag-carrier state. <!-- G19-HUD-AND-021 -->
- **Present:** Bonus status and duration. <!-- G19-HUD-AND-022 -->
- **Present:** Armor indicator. <!-- G19-HUD-AND-023 -->
- **Present:** Weapon image. <!-- G19-HUD-AND-024 -->
- **Present:** Secondary weapon indicator. <!-- G19-HUD-AND-025 -->
- **Present:** Grenade type/count. <!-- G19-HUD-AND-026 -->
- **Present:** Reload progress rather than only “RELOADING.” <!-- G19-HUD-AND-027 -->
- **Present:** Respawn countdown. <!-- G19-HUD-AND-028 -->
- **Present:** Kill feed. <!-- G19-HUD-AND-029 -->
- **Present:** Chat overlay inside the game. <!-- G19-HUD-AND-030 -->
- **Present:** Team-chat distinction. <!-- G19-HUD-AND-031 -->
- **Present:** Server messages. <!-- G19-HUD-AND-032 -->
- **Present:** Connection/ping indicator. <!-- G19-HUD-AND-033 -->
- **Present:** Ping dot with size/color grading. <!-- G19-HUD-AND-034 -->
- **Present:** FPS display. <!-- G19-HUD-AND-035 -->
- **Present:** Network-bandwidth display. <!-- G19-HUD-AND-036 -->
- **Present:** Minimap. <!-- G19-HUD-AND-037 -->
- **Present:** Sniper line. <!-- G19-HUD-AND-038 -->
- **Present:** Crosshair accuracy/bink visualization. <!-- G19-HUD-AND-039 -->
- **Present:** Spectator HUD. <!-- G19-HUD-AND-040 -->
- **Present:** End-of-round screen. <!-- G19-HUD-AND-041 -->
- **Present:** Weapon-statistics screen. <!-- G19-HUD-AND-042 -->
- **Present:** Scrollable large scoreboard. <!-- G19-HUD-AND-043 -->
- **Present:** Player IDs on the command-enabled scoreboard. <!-- G19-HUD-AND-044 -->
- **Present:** Custom HUD/interface loading. <!-- G19-HUD-AND-045 -->
- **Present:** HUD scaling. <!-- G19-HUD-AND-046 -->
- **Present:** Safe-area and resolution tests across desktop and mobile. <!-- G19-HUD-AND-047 -->

[Game Screen reference](https://wiki.soldat.pl/index.php/Game_Screen)

## 20. Rendering and visual effects

- **Present:** Full animated soldier/gostek rendering. <!-- G20-RENDERING-AND-001 -->
- **Present:** Separate body parts. <!-- G20-RENDERING-AND-002 -->
- **Present:** Hair. <!-- G20-RENDERING-AND-003 -->
- **Present:** Headgear. <!-- G20-RENDERING-AND-004 -->
- **Present:** Helmet/hat/none. <!-- G20-RENDERING-AND-005 -->
- **Present:** Chains and dog tags. <!-- G20-RENDERING-AND-006 -->
- **Present:** Shirt, pants, skin, hair, shoes, and jet colors. <!-- G20-RENDERING-AND-007 -->
- **Present:** Weapon sprites. <!-- G20-RENDERING-AND-008 -->
- **Present:** Character pose matching aim angle. <!-- G20-RENDERING-AND-009 -->
- **Present:** Muzzle flashes. <!-- G20-RENDERING-AND-010 -->
- **Present:** Bullet trails. <!-- G20-RENDERING-AND-011 -->
- **Present:** Shell casings. <!-- G20-RENDERING-AND-012 -->
- **Present:** Sparks. <!-- G20-RENDERING-AND-013 -->
- **Present:** Blood. <!-- G20-RENDERING-AND-014 -->
- **Present:** Gore. <!-- G20-RENDERING-AND-015 -->
- **Present:** Explosion animation. <!-- G20-RENDERING-AND-016 -->
- **Present:** Smoke. <!-- G20-RENDERING-AND-017 -->
- **Present:** Fire. <!-- G20-RENDERING-AND-018 -->
- **Present:** Burning characters. <!-- G20-RENDERING-AND-019 -->
- **Present:** Grenade sprites. <!-- G20-RENDERING-AND-020 -->
- **Present:** Arrow sprites. <!-- G20-RENDERING-AND-021 -->
- **Present:** Dropped weapons. <!-- G20-RENDERING-AND-022 -->
- **Present:** Flags. <!-- G20-RENDERING-AND-023 -->
- **Present:** Kits. <!-- G20-RENDERING-AND-024 -->
- **Present:** Stationary gun. <!-- G20-RENDERING-AND-025 -->
- **Present:** Polygon textures. <!-- G20-RENDERING-AND-026 -->
- **Present:** Edge textures. <!-- G20-RENDERING-AND-027 -->
- **Present:** Background scenery. <!-- G20-RENDERING-AND-028 -->
- **Present:** Foreground scenery. <!-- G20-RENDERING-AND-029 -->
- **Present:** Rain. <!-- G20-RENDERING-AND-030 -->
- **Present:** Snow. <!-- G20-RENDERING-AND-031 -->
- **Present:** Wind effects. <!-- G20-RENDERING-AND-032 -->
- **Present:** Bullet-time visual effect. <!-- G20-RENDERING-AND-033 -->
- **Present:** Predator transparency. <!-- G20-RENDERING-AND-034 -->
- **Present:** Berserker overlay. <!-- G20-RENDERING-AND-035 -->
- **Present:** Flame God overlay. <!-- G20-RENDERING-AND-036 -->
- **Present:** Damage feedback. <!-- G20-RENDERING-AND-037 -->
- **Present:** Screen shake where appropriate. <!-- G20-RENDERING-AND-038 -->
- **Present:** Resolution scaling. <!-- G20-RENDERING-AND-039 -->
- **Present:** Texture filtering settings. <!-- G20-RENDERING-AND-040 -->
- **Present:** Mipmapping. <!-- G20-RENDERING-AND-041 -->
- **Present:** Low-particle modes. <!-- G20-RENDERING-AND-042 -->
- **Present:** Compatibility rendering path. <!-- G20-RENDERING-AND-043 -->
- **Present:** Custom-interface graphics. <!-- G20-RENDERING-AND-044 -->
- **Present:** Mod-controlled asset scaling through `mod.ini`-like rules. <!-- G20-RENDERING-AND-045 -->

## 21. Sound and music

There is currently no meaningful Soldat-like audio system.

Add:

- **Present:** Weapon-specific firing sounds. <!-- G21-SOUND-AND-001 -->
- **Present:** Reload sounds. <!-- G21-SOUND-AND-002 -->
- **Present:** Empty-weapon sounds. <!-- G21-SOUND-AND-003 -->
- **Present:** Grenade pin/throw/bounce/explosion sounds. <!-- G21-SOUND-AND-004 -->
- **Present:** M79 and LAW explosion sounds. <!-- G21-SOUND-AND-005 -->
- **Present:** Bullet impacts. <!-- G21-SOUND-AND-006 -->
- **Present:** Ricochets if applicable. <!-- G21-SOUND-AND-007 -->
- **Present:** Chainsaw loop. <!-- G21-SOUND-AND-008 -->
- **Present:** Knife sounds. <!-- G21-SOUND-AND-009 -->
- **Present:** Punch sounds. <!-- G21-SOUND-AND-010 -->
- **Present:** Flamethrower loop. <!-- G21-SOUND-AND-011 -->
- **Present:** Character pain. <!-- G21-SOUND-AND-012 -->
- **Present:** Death sounds. <!-- G21-SOUND-AND-013 -->
- **Present:** Gore sounds. <!-- G21-SOUND-AND-014 -->
- **Present:** Footsteps selected by map property. <!-- G21-SOUND-AND-015 -->
- **Present:** Jetpack sound. <!-- G21-SOUND-AND-016 -->
- **Present:** Flag pickup/drop/return/capture sounds. <!-- G21-SOUND-AND-017 -->
- **Present:** Kit pickup sounds. <!-- G21-SOUND-AND-018 -->
- **Present:** Bonus activation/expiration sounds. <!-- G21-SOUND-AND-019 -->
- **Present:** UI/menu sounds. <!-- G21-SOUND-AND-020 -->
- **Present:** Chat/message notification. <!-- G21-SOUND-AND-021 -->
- **Present:** Distant-battle sounds. <!-- G21-SOUND-AND-022 -->
- **Present:** Weather sounds. <!-- G21-SOUND-AND-023 -->
- **Present:** Explosion deafness and whistle effect. <!-- G21-SOUND-AND-024 -->
- **Present:** Positional audio. <!-- G21-SOUND-AND-025 -->
- **Present:** Distance attenuation. <!-- G21-SOUND-AND-026 -->
- **Present:** Master sound volume. <!-- G21-SOUND-AND-027 -->
- **Present:** Music volume. <!-- G21-SOUND-AND-028 -->
- **Present:** Music playback. <!-- G21-SOUND-AND-029 -->
- **Present:** Toggle music. <!-- G21-SOUND-AND-030 -->
- **Present:** Previous/next track. <!-- G21-SOUND-AND-031 -->
- **Present:** Sound-quality option. <!-- G21-SOUND-AND-032 -->
- **Present:** Sound-output/device option where browser APIs permit it. <!-- G21-SOUND-AND-033 -->

## 22. Player profiles and customization

The project currently saves only a guest name.

Add:

- **Present:** Persistent player profiles. <!-- G22-PLAYER-PROFILES-001 -->
- **Present:** Multiple profiles. <!-- G22-PLAYER-PROFILES-002 -->
- **Present:** Profile selection screen. <!-- G22-PLAYER-PROFILES-003 -->
- **Present:** Per-profile settings. <!-- G22-PLAYER-PROFILES-004 -->
- **Present:** Per-profile controls. <!-- G22-PLAYER-PROFILES-005 -->
- **Present:** Per-profile taunts. <!-- G22-PLAYER-PROFILES-006 -->
- **Present:** Default secondary weapon. <!-- G22-PLAYER-PROFILES-007 -->
- **Present:** Player name constraints matching the intended rules. <!-- G22-PLAYER-PROFILES-008 -->
- **Present:** Shirt color. <!-- G22-PLAYER-PROFILES-009 -->
- **Present:** Pants color. <!-- G22-PLAYER-PROFILES-010 -->
- **Present:** Skin color. <!-- G22-PLAYER-PROFILES-011 -->
- **Present:** Hair color. <!-- G22-PLAYER-PROFILES-012 -->
- **Present:** Shoe color. <!-- G22-PLAYER-PROFILES-013 -->
- **Present:** Jet-flame color. <!-- G22-PLAYER-PROFILES-014 -->
- **Present:** Hairstyle. <!-- G22-PLAYER-PROFILES-015 -->
- **Present:** Headgear. <!-- G22-PLAYER-PROFILES-016 -->
- **Present:** Chain style. <!-- G22-PLAYER-PROFILES-017 -->
- **Present:** Interface selection. <!-- G22-PLAYER-PROFILES-018 -->
- **Present:** Saved mouse sensitivity. <!-- G22-PLAYER-PROFILES-019 -->
- **Present:** Saved sound/music volume. <!-- G22-PLAYER-PROFILES-020 -->
- **Present:** Saved graphics settings. <!-- G22-PLAYER-PROFILES-021 -->
- **Present:** Saved favorite servers. <!-- G22-PLAYER-PROFILES-022 -->
- **Present:** Profile import/export if desired. <!-- G22-PLAYER-PROFILES-023 -->
- **Missing:** Account-backed persistence if the game moves beyond local profiles. <!-- G22-PLAYER-PROFILES-024 -->

## 23. Chat and taunts

- **Present:** Basic room chat. <!-- G23-CHAT-AND-001 -->
- **Present:** Basic chat rate limiting. <!-- G23-CHAT-AND-002 -->
- **Present:** Team chat. <!-- G23-CHAT-AND-003 -->
- **Present:** `^` shorthand for team chat. <!-- G23-CHAT-AND-004 -->
- **Present:** In-game chat overlay. <!-- G23-CHAT-AND-005 -->
- **Present:** Chat while actively playing without interfering with controls. <!-- G23-CHAT-AND-006 -->
- **Present:** Taunt file/configuration. <!-- G23-CHAT-AND-007 -->
- **Present:** Alt+letter and Alt+number taunts. <!-- G23-CHAT-AND-008 -->
- **Present:** Per-profile taunts. <!-- G23-CHAT-AND-009 -->
- **Present:** Command taunts. <!-- G23-CHAT-AND-010 -->
- **Present:** Chat mute. <!-- G23-CHAT-AND-011 -->
- **Present:** Mute by player name. <!-- G23-CHAT-AND-012 -->
- **Present:** Mute by player ID. <!-- G23-CHAT-AND-013 -->
- **Present:** Spam/flood controls beyond the basic one-second limit. <!-- G23-CHAT-AND-014 -->
- **Present:** Profanity/moderation options if required. <!-- G23-CHAT-AND-015 -->
- **Present:** Server announcements. <!-- G23-CHAT-AND-016 -->
- **Present:** Join/leave messages. <!-- G23-CHAT-AND-017 -->
- **Present:** Kill/capture announcements. <!-- G23-CHAT-AND-018 -->
- **Present:** Realistic/Survival chat-visibility rules. <!-- G23-CHAT-AND-019 -->

[Profiles and Taunts](https://wiki.soldat.pl/index.php/Profiles_and_Taunts)

## 24. Player commands

Add a `/` command console and support:

- **Present:** `/KILL` <!-- G24-PLAYER-COMMANDS-001 -->
- **Present:** `/BRUTALKILL` <!-- G24-PLAYER-COMMANDS-002 -->
- **Present:** `/MERCY` <!-- G24-PLAYER-COMMANDS-003 -->
- **Present:** `/SMOKE` <!-- G24-PLAYER-COMMANDS-004 -->
- **Present:** `/TABAC` <!-- G24-PLAYER-COMMANDS-005 -->
- **Present:** `/TAKEOFF` <!-- G24-PLAYER-COMMANDS-006 -->
- **Present:** `/VICTORY` <!-- G24-PLAYER-COMMANDS-007 -->
- **Present:** `/PAUSE` <!-- G24-PLAYER-COMMANDS-008 -->
- **Present:** `/UNPAUSE` <!-- G24-PLAYER-COMMANDS-009 -->

These require their associated character actions, state changes, animations, and server validation.

## 25. Server and administrator commands

Add at least the commands covered by the requested wiki page:

- **Present:** `/ADDMAP <map>` <!-- G25-SERVER-AND-001 -->
- **Present:** `/DELMAP <map>` <!-- G25-SERVER-AND-002 -->
- **Present:** `/ADDBOT<team> <bot>` <!-- G25-SERVER-AND-003 -->
- **Present:** `/KICK <player or ID>` <!-- G25-SERVER-AND-004 -->
- **Present:** `/KICKLAST` <!-- G25-SERVER-AND-005 -->
- **Present:** `/TEMPBAN <minutes> <IP/player>` <!-- G25-SERVER-AND-006 -->
- **Present:** `/BAN <player or ID>` <!-- G25-SERVER-AND-007 -->
- **Present:** `/BANIP <IP>` <!-- G25-SERVER-AND-008 -->
- **Present:** `/UNBAN <IP>` <!-- G25-SERVER-AND-009 -->
- **Present:** `/MAP <map>` <!-- G25-SERVER-AND-010 -->
- **Present:** `/RESTART` <!-- G25-SERVER-AND-011 -->
- **Present:** `/NEXTMAP` <!-- G25-SERVER-AND-012 -->
- **Present:** `/ADM <player>` <!-- G25-SERVER-AND-013 -->
- **Present:** `/ADMIP <IP>` <!-- G25-SERVER-AND-014 -->
- **Present:** `/UNADM <IP>` <!-- G25-SERVER-AND-015 -->
- **Present:** `/RESPAWNTIME <seconds>` <!-- G25-SERVER-AND-016 -->

The complete server experience also needs:

- **Present:** Admin authentication. <!-- G25-SERVER-AND-017 -->
- **Present:** Remote-admin support. <!-- G25-SERVER-AND-018 -->
- **Present:** Persistent ban list. <!-- G25-SERVER-AND-019 -->
- **Present:** Persistent admin list. <!-- G25-SERVER-AND-020 -->
- **Present:** Audit log. <!-- G25-SERVER-AND-021 -->
- **Present:** Command authorization. <!-- G25-SERVER-AND-022 -->
- **Present:** Player-ID display. <!-- G25-SERVER-AND-023 -->
- **Present:** Safe command parsing. <!-- G25-SERVER-AND-024 -->
- **Present:** Command feedback and errors. <!-- G25-SERVER-AND-025 -->
- **Present:** Map-list loading. <!-- G25-SERVER-AND-026 -->
- **Present:** Server configuration reload. <!-- G25-SERVER-AND-027 -->
- **Present:** Lobby re-registration where applicable. <!-- G25-SERVER-AND-028 -->
- **Present:** Password changes. <!-- G25-SERVER-AND-029 -->
- **Present:** Maximum-player changes. <!-- G25-SERVER-AND-030 -->
- **Present:** Locked mode preventing sensitive runtime changes. <!-- G25-SERVER-AND-031 -->

[In-game Commands](https://wiki.soldat.pl/index.php/In-game_Commands)

## 26. Match and server settings

### Start-game settings

- **Present:** Game mode. <!-- G26-MATCH-AND-001 -->
- **Present:** Kill/point limit. <!-- G26-MATCH-AND-002 -->
- **Present:** Capture limit. <!-- G26-MATCH-AND-003 -->
- **Present:** Time limit. <!-- G26-MATCH-AND-004 -->
- **Present:** Survival toggle. <!-- G26-MATCH-AND-005 -->
- **Present:** Realistic toggle. <!-- G26-MATCH-AND-006 -->
- **Present:** Advance toggle. <!-- G26-MATCH-AND-007 -->
- **Present:** Map-list looping. <!-- G26-MATCH-AND-008 -->
- **Present:** Random bots. <!-- G26-MATCH-AND-009 -->
- **Present:** Team-specific bots. <!-- G26-MATCH-AND-010 -->

### Network/server settings

- **Present:** Server name. <!-- G26-MATCH-AND-011 -->
- **Present:** Server password. <!-- G26-MATCH-AND-012 -->
- **Present:** Maximum players. <!-- G26-MATCH-AND-013 -->
- **Present:** Game port. <!-- G26-MATCH-AND-014 -->
- **Present:** Public/private listing. <!-- G26-MATCH-AND-015 -->
- **Present:** Team balancing. <!-- G26-MATCH-AND-016 -->
- **Present:** Maximum allowed ping. <!-- G26-MATCH-AND-017 -->
- **Present:** Ping-kick behavior. <!-- G26-MATCH-AND-018 -->
- **Present:** Welcome/server message. <!-- G26-MATCH-AND-019 -->
- **Present:** Server contact link. <!-- G26-MATCH-AND-020 -->
- **Present:** Dedicated-server configuration. <!-- G26-MATCH-AND-021 -->
- **Present:** Custom map list. <!-- G26-MATCH-AND-022 -->
- **Present:** Custom weapon mod. <!-- G26-MATCH-AND-023 -->
- **Present:** Friendly fire. <!-- G26-MATCH-AND-024 -->
- **Present:** Respawn time. <!-- G26-MATCH-AND-025 -->
- **Present:** Bonus frequency/enabled state. <!-- G26-MATCH-AND-026 -->
- **Present:** Spectator limits. <!-- G26-MATCH-AND-027 -->
- **Present:** Admin password/authentication. <!-- G26-MATCH-AND-028 -->
- **Present:** Logging options. <!-- G26-MATCH-AND-029 -->

### Player settings

- **Present:** Name. <!-- G26-MATCH-AND-030 -->
- **Present:** Appearance. <!-- G26-MATCH-AND-031 -->
- **Present:** Default secondary. <!-- G26-MATCH-AND-032 -->
- **Present:** Background override. <!-- G26-MATCH-AND-033 -->
- **Present:** Controls. <!-- G26-MATCH-AND-034 -->
- **Present:** Mouse sensitivity. <!-- G26-MATCH-AND-035 -->
- **Present:** Interface selection. <!-- G26-MATCH-AND-036 -->
- **Present:** Player indicator. <!-- G26-MATCH-AND-037 -->
- **Present:** Sniper line. <!-- G26-MATCH-AND-038 -->

### Graphics settings

- **Present:** Fullscreen/windowed. <!-- G26-MATCH-AND-039 -->
- **Present:** Display resolution. <!-- G26-MATCH-AND-040 -->
- **Present:** Desktop resolution. <!-- G26-MATCH-AND-041 -->
- **Present:** Interface scaling. <!-- G26-MATCH-AND-042 -->
- **Present:** Particle limit. <!-- G26-MATCH-AND-043 -->
- **Present:** Bullet trails. <!-- G26-MATCH-AND-044 -->
- **Present:** Weather rendering. <!-- G26-MATCH-AND-045 -->
- **Present:** Texture filtering. <!-- G26-MATCH-AND-046 -->
- **Present:** Resolution filtering. <!-- G26-MATCH-AND-047 -->
- **Present:** Mipmapping. <!-- G26-MATCH-AND-048 -->
- **Present:** Compatibility/fixed-pipeline equivalent. <!-- G26-MATCH-AND-049 -->
- **Present:** Intro playback. <!-- G26-MATCH-AND-050 -->
- **Present:** Final-score screenshot. <!-- G26-MATCH-AND-051 -->
- **Present:** Clanmatch color behavior. <!-- G26-MATCH-AND-052 -->
- **Present:** Frame-rate limit or VSync. <!-- G26-MATCH-AND-053 -->
- **Present:** Performance statistics. <!-- G26-MATCH-AND-054 -->

### Audio settings

- **Present:** Sound volume. <!-- G26-MATCH-AND-055 -->
- **Present:** Music volume. <!-- G26-MATCH-AND-056 -->
- **Present:** Sound quality. <!-- G26-MATCH-AND-057 -->
- **Present:** Output device. <!-- G26-MATCH-AND-058 -->
- **Present:** Explosion effect. <!-- G26-MATCH-AND-059 -->
- **Present:** Distant battle. <!-- G26-MATCH-AND-060 -->
- **Present:** Game music. <!-- G26-MATCH-AND-061 -->

[Settings reference](https://wiki.soldat.pl/index.php/Settings)

## 27. Lobby and room browser

The current room system is useful but needs:

- **Present:** Server/room refresh. <!-- G27-LOBBY-AND-001 -->
- **Present:** Cancel refresh. <!-- G27-LOBBY-AND-002 -->
- **Present:** Ping measurement. <!-- G27-LOBBY-AND-003 -->
- **Present:** Ping-all action. <!-- G27-LOBBY-AND-004 -->
- **Present:** Ping column. <!-- G27-LOBBY-AND-005 -->
- **Present:** Player count. <!-- G27-LOBBY-AND-006 -->
- **Present:** Maximum players. <!-- G27-LOBBY-AND-007 -->
- **Present:** Game mode. <!-- G27-LOBBY-AND-008 -->
- **Present:** Map name. <!-- G27-LOBBY-AND-009 -->
- **Present:** Country/region if desired. <!-- G27-LOBBY-AND-010 -->
- **Present:** Password indicator. <!-- G27-LOBBY-AND-011 -->
- **Present:** Realistic indicator. <!-- G27-LOBBY-AND-012 -->
- **Present:** Survival indicator. <!-- G27-LOBBY-AND-013 -->
- **Present:** Advance indicator. <!-- G27-LOBBY-AND-014 -->
- **Present:** Weapon-mod indicator. <!-- G27-LOBBY-AND-015 -->
- **Present:** Version compatibility. <!-- G27-LOBBY-AND-016 -->
- **Present:** Favorites. <!-- G27-LOBBY-AND-017 -->
- **Present:** Favorite add/remove. <!-- G27-LOBBY-AND-018 -->
- **Present:** Direct IP/hostname join if supported. <!-- G27-LOBBY-AND-019 -->
- **Present:** Port input. <!-- G27-LOBBY-AND-020 -->
- **Present:** Password input. <!-- G27-LOBBY-AND-021 -->
- **Present:** Spectator join. <!-- G27-LOBBY-AND-022 -->
- **Present:** Sort and filter. <!-- G27-LOBBY-AND-023 -->
- **Present:** Search. <!-- G27-LOBBY-AND-024 -->
- **Present:** Full-room filtering. <!-- G27-LOBBY-AND-025 -->
- **Present:** Empty-room filtering. <!-- G27-LOBBY-AND-026 -->
- **Present:** Public lobby registration. <!-- G27-LOBBY-AND-027 -->
- **Present:** Reliable server discovery. <!-- G27-LOBBY-AND-028 -->
- **Present:** Join/download progress. <!-- G27-LOBBY-AND-029 -->
- **Present:** Cancel connection/download. <!-- G27-LOBBY-AND-030 -->
- **Present:** Better reconnect state. <!-- G27-LOBBY-AND-031 -->
- **Present:** Room ownership/host settings. <!-- G27-LOBBY-AND-032 -->
- **Present:** Room deletion or expiry controls. <!-- G27-LOBBY-AND-033 -->
- **Present:** Password-protected rooms in addition to invite codes. <!-- G27-LOBBY-AND-034 -->
- **Present:** Team choice after joining. <!-- G27-LOBBY-AND-035 -->
- **Present:** Late-join rules. <!-- G27-LOBBY-AND-036 -->
- **Present:** Abuse-resistant room creation. <!-- G27-LOBBY-AND-037 -->
- **Present:** Rate limiting for connection and room operations. <!-- G27-LOBBY-AND-038 -->

## 28. Spectating

- **Present:** Spectator team. <!-- G28-SPECTATING-001 -->
- **Present:** Join directly as spectator. <!-- G28-SPECTATING-002 -->
- **Present:** Switch followed player. <!-- G28-SPECTATING-003 -->
- **Present:** Previous/next player. <!-- G28-SPECTATING-004 -->
- **Present:** Free camera if desired. <!-- G28-SPECTATING-005 -->
- **Present:** Spectator HUD. <!-- G28-SPECTATING-006 -->
- **Present:** Spectator scoreboard. <!-- G28-SPECTATING-007 -->
- **Present:** Spectator chat restrictions. <!-- G28-SPECTATING-008 -->
- **Present:** Realistic visibility restrictions. <!-- G28-SPECTATING-009 -->
- **Present:** Survival restrictions. <!-- G28-SPECTATING-010 -->
- **Present:** Followed-player minimap/visibility behavior. <!-- G28-SPECTATING-011 -->
- **Present:** Delay option for competitive matches. <!-- G28-SPECTATING-012 -->

## 29. Scoring and statistics

- **Present:** Correct scoring for every game mode. <!-- G29-SCORING-AND-001 -->
- **Present:** Match time. <!-- G29-SCORING-AND-002 -->
- **Present:** Individual points. <!-- G29-SCORING-AND-003 -->
- **Present:** Team points. <!-- G29-SCORING-AND-004 -->
- **Present:** Captures. <!-- G29-SCORING-AND-005 -->
- **Present:** Flag returns. <!-- G29-SCORING-AND-006 -->
- **Present:** Current server rank. <!-- G29-SCORING-AND-007 -->
- **Present:** Score difference from leader. <!-- G29-SCORING-AND-008 -->
- **Present:** Kill/point/capture limit display. <!-- G29-SCORING-AND-009 -->
- **Present:** Weapon statistics for the current round. <!-- G29-SCORING-AND-010 -->
- **Present:** Shots fired. <!-- G29-SCORING-AND-011 -->
- **Present:** Hits. <!-- G29-SCORING-AND-012 -->
- **Present:** Accuracy. <!-- G29-SCORING-AND-013 -->
- **Present:** Kills per weapon. <!-- G29-SCORING-AND-014 -->
- **Present:** Deaths per weapon/cause. <!-- G29-SCORING-AND-015 -->
- **Present:** Headshots. <!-- G29-SCORING-AND-016 -->
- **Present:** Suicides. <!-- G29-SCORING-AND-017 -->
- **Present:** Teamkills. <!-- G29-SCORING-AND-018 -->
- **Present:** Objective statistics. <!-- G29-SCORING-AND-019 -->
- **Present:** End-of-round summary. <!-- G29-SCORING-AND-020 -->
- **Present:** Match history. <!-- G29-SCORING-AND-021 -->
- **Present:** Persistent player statistics if desired. <!-- G29-SCORING-AND-022 -->
- **Missing:** Clan/team statistics if desired. <!-- G29-SCORING-AND-023 -->
- **Present:** Exportable logs. <!-- G29-SCORING-AND-024 -->

## 30. Networking and prediction

The server is authoritative, but only local-player movement gets basic optional prediction.

Add:

- **Present:** Proper client-side prediction for all movement states. <!-- G30-NETWORKING-AND-001 -->
- **Present:** Input reconciliation. <!-- G30-NETWORKING-AND-002 -->
- **Present:** Unacknowledged-input replay. <!-- G30-NETWORKING-AND-003 -->
- **Present:** Remote-player interpolation. <!-- G30-NETWORKING-AND-004 -->
- **Present:** Projectile interpolation. <!-- G30-NETWORKING-AND-005 -->
- **Present:** Extrapolation limits. <!-- G30-NETWORKING-AND-006 -->
- **Present:** Latency compensation. <!-- G30-NETWORKING-AND-007 -->
- **Missing:** Server rewind/lag compensation if appropriate. <!-- G30-NETWORKING-AND-008 -->
- **Present:** Clock synchronization. <!-- G30-NETWORKING-AND-009 -->
- **Present:** Measured ping. <!-- G30-NETWORKING-AND-010 -->
- **Present:** Packet-loss handling. <!-- G30-NETWORKING-AND-011 -->
- **Missing:** Snapshot delta compression. <!-- G30-NETWORKING-AND-012 -->
- **Missing:** Interest management if maps/player counts grow. <!-- G30-NETWORKING-AND-013 -->
- **Missing:** Binary protocol or more compact encoding if needed. <!-- G30-NETWORKING-AND-014 -->
- **Present:** Weapon/event prediction. <!-- G30-NETWORKING-AND-015 -->
- **Present:** Predicted muzzle/projectile effects. <!-- G30-NETWORKING-AND-016 -->
- **Present:** Rollback correction smoothing. <!-- G30-NETWORKING-AND-017 -->
- **Present:** Map and ruleset synchronization. <!-- G30-NETWORKING-AND-018 -->
- **Present:** Disconnect reason handling. <!-- G30-NETWORKING-AND-019 -->
- **Present:** Robust resumption after page sleep/mobile backgrounding. <!-- G30-NETWORKING-AND-020 -->
- **Present:** Duplicate-session handling. <!-- G30-NETWORKING-AND-021 -->
- **Present:** Rate limits for every client message. <!-- G30-NETWORKING-AND-022 -->
- **Present:** Anti-speedhack/input-frequency validation. <!-- G30-NETWORKING-AND-023 -->
- **Present:** Fire-rate validation. <!-- G30-NETWORKING-AND-024 -->
- **Present:** Aim/input sanity validation beyond coordinate bounds. <!-- G30-NETWORKING-AND-025 -->
- **Present:** Server-authoritative pickups, flags, bonuses, and round state. <!-- G30-NETWORKING-AND-026 -->
- **Present:** Network load and soak tests. <!-- G30-NETWORKING-AND-027 -->
- **Present:** High-latency/jitter/loss simulation tests. <!-- G30-NETWORKING-AND-028 -->

## 31. Anti-cheat and abuse resistance

- **Present:** Server-authoritative collision validation. <!-- G31-ANTI-CHEAT-001 -->
- **Present:** Server-authoritative weapon selection. <!-- G31-ANTI-CHEAT-002 -->
- **Present:** Server-authoritative reload and inventory. <!-- G31-ANTI-CHEAT-003 -->
- **Present:** Input-rate limiting. <!-- G31-ANTI-CHEAT-004 -->
- **Present:** Movement feasibility validation. <!-- G31-ANTI-CHEAT-005 -->
- **Present:** Aim-value validation. <!-- G31-ANTI-CHEAT-006 -->
- **Present:** Chat abuse controls. <!-- G31-ANTI-CHEAT-007 -->
- **Present:** Connection/IP rate limiting. <!-- G31-ANTI-CHEAT-008 -->
- **Present:** Room-creation rate limiting. <!-- G31-ANTI-CHEAT-009 -->
- **Present:** Admin permission validation. <!-- G31-ANTI-CHEAT-010 -->
- **Present:** Ban enforcement. <!-- G31-ANTI-CHEAT-011 -->
- **Present:** Temporary bans. <!-- G31-ANTI-CHEAT-012 -->
- **Present:** Audit logging. <!-- G31-ANTI-CHEAT-013 -->
- **Present:** Suspicious behavior metrics. <!-- G31-ANTI-CHEAT-014 -->
- **Missing:** Protocol fuzzing. <!-- G31-ANTI-CHEAT-015 -->
- **Present:** Malformed WebSocket testing. <!-- G31-ANTI-CHEAT-016 -->
- **Present:** Replay-based cheat investigation. <!-- G31-ANTI-CHEAT-017 -->
- **Present:** Secure resume tokens. <!-- G31-ANTI-CHEAT-018 -->
- **Present:** Token expiry/rotation. <!-- G31-ANTI-CHEAT-019 -->
- **Missing:** Deployment-level denial-of-service protection. <!-- G31-ANTI-CHEAT-020 -->
- **Present:** No trust in client-provided cosmetics or settings that affect gameplay. <!-- G31-ANTI-CHEAT-021 -->

## 32. Menus and overall game flow

- **Present:** Main menu. <!-- G32-MENUS-AND-001 -->
- **Present:** Profile selection. <!-- G32-MENUS-AND-002 -->
- **Present:** Player customization. <!-- G32-MENUS-AND-003 -->
- **Present:** Join-game screen. <!-- G32-MENUS-AND-004 -->
- **Present:** Start-game/server screen. <!-- G32-MENUS-AND-005 -->
- **Present:** Options menu. <!-- G32-MENUS-AND-006 -->
- **Present:** Controls menu. <!-- G32-MENUS-AND-007 -->
- **Present:** Weapon-selection screen during respawn. <!-- G32-MENUS-AND-008 -->
- **Present:** Team-selection screen. <!-- G32-MENUS-AND-009 -->
- **Present:** Spectator selection. <!-- G32-MENUS-AND-010 -->
- **Present:** Pause menu. <!-- G32-MENUS-AND-011 -->
- **Present:** Disconnect confirmation. <!-- G32-MENUS-AND-012 -->
- **Present:** Map loading screen. <!-- G32-MENUS-AND-013 -->
- **Present:** Asset-download screen. <!-- G32-MENUS-AND-014 -->
- **Present:** Round-intro countdown. <!-- G32-MENUS-AND-015 -->
- **Present:** Round-end screen. <!-- G32-MENUS-AND-016 -->
- **Present:** Match-results screen. <!-- G32-MENUS-AND-017 -->
- **Present:** Connection-lost overlay. <!-- G32-MENUS-AND-018 -->
- **Present:** Version mismatch/update flow. <!-- G32-MENUS-AND-019 -->
- **Present:** Credits. <!-- G32-MENUS-AND-020 -->
- **Present:** Help/manual. <!-- G32-MENUS-AND-021 -->
- **Present:** First-run control tutorial. <!-- G32-MENUS-AND-022 -->
- **Present:** Mobile onboarding. <!-- G32-MENUS-AND-023 -->

## 33. Custom interfaces and modding

- **Present:** Loadable HUD/interface definitions. <!-- G33-CUSTOM-INTERFACES-001 -->
- **Present:** Multiple interface presets. <!-- G33-CUSTOM-INTERFACES-002 -->
- **Present:** Custom cursor. <!-- G33-CUSTOM-INTERFACES-003 -->
- **Present:** Custom HUD image positions. <!-- G33-CUSTOM-INTERFACES-004 -->
- **Present:** Interface scaling. <!-- G33-CUSTOM-INTERFACES-005 -->
- **Present:** Custom weapon graphics. <!-- G33-CUSTOM-INTERFACES-006 -->
- **Present:** Custom sounds. <!-- G33-CUSTOM-INTERFACES-007 -->
- **Present:** Custom character/gostek graphics. <!-- G33-CUSTOM-INTERFACES-008 -->
- **Present:** `mod.ini`-style asset scaling. <!-- G33-CUSTOM-INTERFACES-009 -->
- **Present:** Mod selection/launching. <!-- G33-CUSTOM-INTERFACES-010 -->
- **Present:** Mod preview. <!-- G33-CUSTOM-INTERFACES-011 -->
- **Present:** Mod packaging. <!-- G33-CUSTOM-INTERFACES-012 -->
- **Present:** Mod downloading. <!-- G33-CUSTOM-INTERFACES-013 -->
- **Present:** Mod version/hash matching. <!-- G33-CUSTOM-INTERFACES-014 -->
- **Present:** Server-required mod support. <!-- G33-CUSTOM-INTERFACES-015 -->
- **Present:** Safe path handling. <!-- G33-CUSTOM-INTERFACES-016 -->
- **Present:** Asset-size limits. <!-- G33-CUSTOM-INTERFACES-017 -->
- **Present:** License/provenance metadata. <!-- G33-CUSTOM-INTERFACES-018 -->

The historical built-in interface names listed by the wiki are:

- **Missing:** Cabbage <!-- G33-CUSTOM-INTERFACES-019 -->
- **Missing:** Classic <!-- G33-CUSTOM-INTERFACES-020 -->
- **Missing:** Lacey V2 <!-- G33-CUSTOM-INTERFACES-021 -->
- **Missing:** Micro1 <!-- G33-CUSTOM-INTERFACES-022 -->
- **Missing:** Military <!-- G33-CUSTOM-INTERFACES-023 -->
- **Missing:** Predator <!-- G33-CUSTOM-INTERFACES-024 -->
- **Missing:** Soldat Style <!-- G33-CUSTOM-INTERFACES-025 -->
- **Missing:** Storm <!-- G33-CUSTOM-INTERFACES-026 -->
- **Missing:** Tech <!-- G33-CUSTOM-INTERFACES-027 -->
- **Missing:** Text <!-- G33-CUSTOM-INTERFACES-028 -->

These names/assets should only be reproduced if licensing permits it.

## 34. Map editor and content pipeline

PolyWorks itself does not need to be embedded in the game, but equivalent content tooling is needed if custom mapping is a product goal:

- **Present:** Polygon creation/editing. <!-- G34-MAP-EDITOR-001 -->
- **Present:** Polygon type editing. <!-- G34-MAP-EDITOR-002 -->
- **Present:** Vertex manipulation. <!-- G34-MAP-EDITOR-003 -->
- **Present:** Texture assignment. <!-- G34-MAP-EDITOR-004 -->
- **Present:** Texture-coordinate editing. <!-- G34-MAP-EDITOR-005 -->
- **Present:** Multi-texture workflow. <!-- G34-MAP-EDITOR-006 -->
- **Present:** Scenery placement. <!-- G34-MAP-EDITOR-007 -->
- **Present:** Scenery layers. <!-- G34-MAP-EDITOR-008 -->
- **Present:** Collider placement. <!-- G34-MAP-EDITOR-009 -->
- **Present:** Spawn placement. <!-- G34-MAP-EDITOR-010 -->
- **Present:** Flag/objective placement. <!-- G34-MAP-EDITOR-011 -->
- **Present:** Bonus placement. <!-- G34-MAP-EDITOR-012 -->
- **Present:** Waypoint creation. <!-- G34-MAP-EDITOR-013 -->
- **Present:** Weather properties. <!-- G34-MAP-EDITOR-014 -->
- **Present:** Background colors. <!-- G34-MAP-EDITOR-015 -->
- **Present:** Footstep sounds. <!-- G34-MAP-EDITOR-016 -->
- **Present:** Jet-fuel setting. <!-- G34-MAP-EDITOR-017 -->
- **Present:** Map validation. <!-- G34-MAP-EDITOR-018 -->
- **Present:** Mode validation. <!-- G34-MAP-EDITOR-019 -->
- **Present:** Test/play button. <!-- G34-MAP-EDITOR-020 -->
- **Present:** Undo/redo. <!-- G34-MAP-EDITOR-021 -->
- **Present:** Copy/paste. <!-- G34-MAP-EDITOR-022 -->
- **Present:** Selection tools. <!-- G34-MAP-EDITOR-023 -->
- **Present:** Zoom/pan/grid. <!-- G34-MAP-EDITOR-024 -->
- **Present:** Prefabs. <!-- G34-MAP-EDITOR-025 -->
- **Present:** PMS import. <!-- G34-MAP-EDITOR-026 -->
- **Present:** PMS export. <!-- G34-MAP-EDITOR-027 -->
- **Present:** Project-native map format. <!-- G34-MAP-EDITOR-028 -->
- **Present:** Packaging custom assets. <!-- G34-MAP-EDITOR-029 -->
- **Present:** Map preview generation. <!-- G34-MAP-EDITOR-030 -->
- **Present:** Dedicated-server deployment. <!-- G34-MAP-EDITOR-031 -->
- **Present:** Cross-platform editor support. <!-- G34-MAP-EDITOR-032 -->

[PolyWorks reference](https://wiki.soldat.pl/index.php/Soldat_PolyWorks)

## 35. Demo and replay system

- **Present:** Record match inputs/events. <!-- G35-DEMO-AND-001 -->
- **Present:** Deterministic replay format. <!-- G35-DEMO-AND-002 -->
- **Present:** Replay metadata. <!-- G35-DEMO-AND-003 -->
- **Present:** Replay playback. <!-- G35-DEMO-AND-004 -->
- **Present:** Pause. <!-- G35-DEMO-AND-005 -->
- **Present:** Seek. <!-- G35-DEMO-AND-006 -->
- **Present:** Fast-forward. <!-- G35-DEMO-AND-007 -->
- **Present:** Follow player. <!-- G35-DEMO-AND-008 -->
- **Present:** Free camera. <!-- G35-DEMO-AND-009 -->
- **Present:** Replay compatibility/versioning. <!-- G35-DEMO-AND-010 -->
- **Present:** Replay validation. <!-- G35-DEMO-AND-011 -->
- **Present:** Export or share replay. <!-- G35-DEMO-AND-012 -->
- **Present:** Demo repair/recovery where feasible. <!-- G35-DEMO-AND-013 -->
- **Present:** Server-side competitive match recording. <!-- G35-DEMO-AND-014 -->

## 36. Operational and production work

The repository itself identifies several non-gameplay gaps:

- **Present:** Persistent server data. <!-- G36-OPERATIONAL-AND-001 -->
- **Missing:** Persistent accounts. <!-- G36-OPERATIONAL-AND-002 -->
- **Present:** Persistent statistics. <!-- G36-OPERATIONAL-AND-003 -->
- **Present:** Persistent bans/admins. <!-- G36-OPERATIONAL-AND-004 -->
- **Present:** Server restart recovery. <!-- G36-OPERATIONAL-AND-005 -->
- **Present:** Graceful active-match handling during deployment. <!-- G36-OPERATIONAL-AND-006 -->
- **Present:** Horizontal scaling strategy. <!-- G36-OPERATIONAL-AND-007 -->
- **Present:** Room ownership across multiple server processes. <!-- G36-OPERATIONAL-AND-008 -->
- **Present:** Lobby service. <!-- G36-OPERATIONAL-AND-009 -->
- **Missing:** Database. <!-- G36-OPERATIONAL-AND-010 -->
- **Missing:** Authentication if accounts are added. <!-- G36-OPERATIONAL-AND-011 -->
- **Present:** Observability beyond three counters. <!-- G36-OPERATIONAL-AND-012 -->
- **Present:** Structured logs. <!-- G36-OPERATIONAL-AND-013 -->
- **Present:** Error tracking. <!-- G36-OPERATIONAL-AND-014 -->
- **Present:** Latency metrics. <!-- G36-OPERATIONAL-AND-015 -->
- **Present:** Tick-duration metrics. <!-- G36-OPERATIONAL-AND-016 -->
- **Present:** Connected-player metrics. <!-- G36-OPERATIONAL-AND-017 -->
- **Present:** Per-room metrics. <!-- G36-OPERATIONAL-AND-018 -->
- **Present:** Health versus readiness endpoints. <!-- G36-OPERATIONAL-AND-019 -->
- **Present:** Backups. <!-- G36-OPERATIONAL-AND-020 -->
- **Present:** Migration process. <!-- G36-OPERATIONAL-AND-021 -->
- **Present:** Load testing. <!-- G36-OPERATIONAL-AND-022 -->
- **Present:** Soak testing. <!-- G36-OPERATIONAL-AND-023 -->
- **Missing:** Browser compatibility testing. <!-- G36-OPERATIONAL-AND-024 -->
- **Present:** Mobile-device testing. <!-- G36-OPERATIONAL-AND-025 -->
- **Present:** Touch-control usability testing. <!-- G36-OPERATIONAL-AND-026 -->
- **Present:** Accessibility testing. <!-- G36-OPERATIONAL-AND-027 -->
- **Present:** Security review. <!-- G36-OPERATIONAL-AND-028 -->
- **Present:** Dependency scanning. <!-- G36-OPERATIONAL-AND-029 -->
- **Present:** Deployment rollback testing. <!-- G36-OPERATIONAL-AND-030 -->
- **Missing:** Asset CDN/caching. <!-- G36-OPERATIONAL-AND-031 -->
- **Present:** Static-asset compression. <!-- G36-OPERATIONAL-AND-032 -->
- **Present:** Privacy policy and moderation policy if publicly operated. <!-- G36-OPERATIONAL-AND-033 -->

## 37. Testing required for parity

- **Present:** Source-versus-Rust movement fixtures. <!-- G37-TESTING-REQUIRED-001 -->
- **Present:** Jump fixtures. <!-- G37-TESTING-REQUIRED-002 -->
- **Present:** Jet fixtures. <!-- G37-TESTING-REQUIRED-003 -->
- **Present:** Crouch/prone/roll/backflip fixtures. <!-- G37-TESTING-REQUIRED-004 -->
- **Present:** Polygon collision fixtures. <!-- G37-TESTING-REQUIRED-005 -->
- **Present:** One-way polygon fixtures. <!-- G37-TESTING-REQUIRED-006 -->
- **Present:** Weapon fixture for every weapon and field. <!-- G37-TESTING-REQUIRED-007 -->
- **Present:** Fire-rate fixtures. <!-- G37-TESTING-REQUIRED-008 -->
- **Present:** Reload fixtures. <!-- G37-TESTING-REQUIRED-009 -->
- **Present:** Startup fixtures. <!-- G37-TESTING-REQUIRED-010 -->
- **Present:** Spread fixtures. <!-- G37-TESTING-REQUIRED-011 -->
- **Present:** Recoil fixtures. <!-- G37-TESTING-REQUIRED-012 -->
- **Present:** Bink fixtures. <!-- G37-TESTING-REQUIRED-013 -->
- **Present:** Velocity-inheritance fixtures. <!-- G37-TESTING-REQUIRED-014 -->
- **Present:** Head/chest/leg damage fixtures. <!-- G37-TESTING-REQUIRED-015 -->
- **Present:** Explosion fixtures. <!-- G37-TESTING-REQUIRED-016 -->
- **Present:** Grenade bounce/fuse fixtures. <!-- G37-TESTING-REQUIRED-017 -->
- **Present:** Knife-throw fixtures. <!-- G37-TESTING-REQUIRED-018 -->
- **Present:** Flame fixtures. <!-- G37-TESTING-REQUIRED-019 -->
- **Present:** Flag interaction fixtures. <!-- G37-TESTING-REQUIRED-020 -->
- **Present:** Bonus-kit fixtures. <!-- G37-TESTING-REQUIRED-021 -->
- **Present:** Every mode’s scoring fixtures. <!-- G37-TESTING-REQUIRED-022 -->
- **Present:** Round-limit fixtures. <!-- G37-TESTING-REQUIRED-023 -->
- **Present:** Respawn fixtures. <!-- G37-TESTING-REQUIRED-024 -->
- **Present:** Map-loading fixtures. <!-- G37-TESTING-REQUIRED-025 -->
- **Present:** PMS parser fuzzing. <!-- G37-TESTING-REQUIRED-026 -->
- **Present:** Determinism across native Rust and Wasm. <!-- G37-TESTING-REQUIRED-027 -->
- **Present:** Network reconciliation tests. <!-- G37-TESTING-REQUIRED-028 -->
- **Present:** Two-player browser integration test. <!-- G37-TESTING-REQUIRED-029 -->
- **Present:** Full 16-player test. <!-- G37-TESTING-REQUIRED-030 -->
- **Present:** Reconnect test. <!-- G37-TESTING-REQUIRED-031 -->
- **Present:** Server-restart test. <!-- G37-TESTING-REQUIRED-032 -->
- **Present:** Mobile portrait test. <!-- G37-TESTING-REQUIRED-033 -->
- **Present:** Mobile landscape test. <!-- G37-TESTING-REQUIRED-034 -->
- **Present:** Low/high latency tests. <!-- G37-TESTING-REQUIRED-035 -->
- **Present:** Packet-loss tests. <!-- G37-TESTING-REQUIRED-036 -->
- **Present:** Long-running server soak test. <!-- G37-TESTING-REQUIRED-037 -->

## Recommended build order

To avoid building UI and content on inaccurate foundations:

1. Polygon map and collision engine.
2. Exact Soldat movement, poses, rolls, backflips, and jets.
3. Exact weapon/damage/accuracy/bink/push system.
4. Inventory, reload, pickup, weapon throw, knife throw, and all additional weapons.
5. CTF flags and objective framework.
6. Remaining official modes and modifiers.
7. Kits and bonuses.
8. Full HUD, kill feed, scoreboard, minimap, and spectator mode.
9. Maps and asset pipeline.
10. Bots.
11. Audio, animation, particles, gore, and visual polish.
12. Profiles, settings, controls, commands, and administration.
13. Custom maps/modding/replays.
14. Persistence, security, load testing, and production hardening.

That is the complete feature-level gap list represented by the requested wiki sections, including gameplay, content, UI, configuration, commands, customization, tools, and the infrastructure needed to make those systems usable.

## More context
Soldat Community Wiki as reference context for this 2D shooter project.

Key concepts captured:

- Fast side-view movement combining running, jumping, crouching, prone movement, rolling, backflips, and limited jetpack flight.
- Mouse aiming with primary/secondary weapons, grenades, weapon throwing, reloads, projectile physics, recoil, spread, bink, and inherited velocity.
- Seven core modes: Deathmatch, Pointmatch, Teammatch, Rambomatch, Capture the Flag, Infiltration, and Hold the Flag.
- Realistic, Survival, and Advance modifiers.
- HUD elements for health, ammunition/reload, fire interval, jet fuel, rankings, objectives, scores, and network status.
- Twenty-one weapons across primary, secondary, and additional categories.
- Weapon configuration through `weapons.ini`, using 60 simulation ticks per second and damage derived from damage, projectile speed, and hitbox modifiers.
- Bonus kits: health, grenades, cluster grenades, armor, flamethrower/invulnerability, increased damage, and invisibility.
- Maps built from polygons, scenery, colliders, spawns, bonuses, bot waypoints, environmental properties, and mode-specific objectives.
- The referenced version of the default-map list contains 97 maps: 29 general, 32 CTF, 19 HTF, and 17 Infiltration maps.
- Profiles, configurable controls, chat/taunts, administrative commands, bots, server settings, and audiovisual options.
- PolyWorks as the principal Soldat map editor, alongside older community tools for mapping, modding, interfaces, demos, servers, and statistics.

Sources: [Soldat Wiki](https://wiki.soldat.pl/), [Introduction](https://wiki.soldat.pl/index.php/Introduction), [Default Controls](https://wiki.soldat.pl/index.php/Default_Controls), [Game Modes](https://wiki.soldat.pl/index.php/Game_Modes), [Weapons](https://wiki.soldat.pl/index.php/Weapons), [Weapon Mod](https://wiki.soldat.pl/index.php/Weapon_Mod), [Default Maps](https://wiki.soldat.pl/index.php/Default_Maps), [Soldat PolyWorks](https://wiki.soldat.pl/index.php/Soldat_PolyWorks).
