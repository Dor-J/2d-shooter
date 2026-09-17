The game currently has a solid prototype foundation, but it is still far from full Soldat parity. It has one rectangular arena, basic running/jumping/jetting, 14 selectable weapons, frag grenades, health/death/respawn, Deathmatch, Team Deathmatch, rooms, chat, a scoreboard, mobile controls, and server authority.

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
- **Missing:** Character-body ragdoll physics. <!-- G01-CHARACTER-MOVEMENT-026 -->
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
- **Missing:** Flag and kit push from bullets/explosions. <!-- G01-CHARACTER-MOVEMENT-038 -->
- **Present:** Movement animation state machine. <!-- G01-CHARACTER-MOVEMENT-039 -->
- **Missing:** Directional aiming and body rotation. <!-- G01-CHARACTER-MOVEMENT-040 -->
- **Missing:** Separate legs, torso, head, arms, weapon, and jet animations. <!-- G01-CHARACTER-MOVEMENT-041 -->
- **Missing:** Death animations. <!-- G01-CHARACTER-MOVEMENT-042 -->
- **Missing:** Mercy/victory/smoke/tobacco/helmet animations. <!-- G01-CHARACTER-MOVEMENT-043 -->

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
- Switch between carried primary and secondary weapons. <!-- G02-INPUT-AND-015 -->
- Drop current weapon. <!-- G02-INPUT-AND-016 -->
- Hold-to-charge weapon throw. <!-- G02-INPUT-AND-017 -->
- Throw combat knife. <!-- G02-INPUT-AND-018 -->
- Pick up weapons. <!-- G02-INPUT-AND-019 -->
- Pick up flags and kits. <!-- G02-INPUT-AND-020 -->
- Flag throw using jump+crouch. <!-- G02-INPUT-AND-021 -->
- Dedicated configurable flag-throw key. <!-- G02-INPUT-AND-022 -->
- Respawn weapon-selection menu. <!-- G02-INPUT-AND-023 -->
- **Present:** Separate primary and secondary selection controls. <!-- G02-INPUT-AND-024 -->
- Team chat. <!-- G02-INPUT-AND-025 -->
- Command console. <!-- G02-INPUT-AND-026 -->
- **Present:** Scoreboard hold/toggle behavior. <!-- G02-INPUT-AND-027 -->
- Weapon-statistics screen. <!-- G02-INPUT-AND-028 -->
- Minimap toggle. <!-- G02-INPUT-AND-029 -->
- Sniper-line toggle. <!-- G02-INPUT-AND-030 -->
- Performance-statistics overlay. <!-- G02-INPUT-AND-031 -->
- **Present:** Screenshot control. <!-- G02-INPUT-AND-032 -->
- Music toggle. <!-- G02-INPUT-AND-033 -->
- Previous/next music track. <!-- G02-INPUT-AND-034 -->
- Demo recording. <!-- G02-INPUT-AND-035 -->
- Demo playback fast-forward. <!-- G02-INPUT-AND-036 -->
- Pause. <!-- G02-INPUT-AND-037 -->
- **Present:** Window minimize shortcut. <!-- G02-INPUT-AND-038 -->
- Taunt shortcuts. <!-- G02-INPUT-AND-039 -->
- **Present:** Runtime mouse-sensitivity adjustment. <!-- G02-INPUT-AND-040 -->
- Runtime sound-volume adjustment. <!-- G02-INPUT-AND-041 -->
- **Present:** Scoreboard scrolling. <!-- G02-INPUT-AND-042 -->
- **Present:** Fully rebindable keyboard controls. <!-- G02-INPUT-AND-043 -->
- **Present:** Rebindable mouse buttons. <!-- G02-INPUT-AND-044 -->
- **Present:** Controller/gamepad support. <!-- G02-INPUT-AND-045 -->
- **Present:** Saved control profiles. <!-- G02-INPUT-AND-046 -->
- **Present:** Accessibility alternatives for combined inputs. <!-- G02-INPUT-AND-047 -->
- Mobile equivalents for crouch, prone, roll, reload, weapon switch, weapon throw, flag throw, scoreboard, and team chat. <!-- G02-INPUT-AND-048 -->

## 3. Weapons

The project contains all 10 primary and 4 secondary weapon definitions, but many are statistical approximations or share simplified projectile behavior.

### Primary weapons requiring full parity

- **Partial:** Desert Eagles. <!-- G03-WEAPONS-001 -->
- **Partial:** HK MP5. <!-- G03-WEAPONS-002 -->
- **Partial:** AK-74. <!-- G03-WEAPONS-003 -->
- **Partial:** Steyr AUG. <!-- G03-WEAPONS-004 -->
- **Partial:** SPAS-12. <!-- G03-WEAPONS-005 -->
- **Partial:** Ruger 77. <!-- G03-WEAPONS-006 -->
- **Partial:** M79. <!-- G03-WEAPONS-007 -->
- **Partial:** Barrett M82A1. <!-- G03-WEAPONS-008 -->
- **Partial:** FN Minimi. <!-- G03-WEAPONS-009 -->
- **Partial:** XM214 Minigun. <!-- G03-WEAPONS-010 -->

### Secondary weapons requiring full parity

- **Partial:** USSOCOM. <!-- G03-WEAPONS-011 -->
- **Partial:** Combat Knife. <!-- G03-WEAPONS-012 -->
- **Partial:** Chainsaw. <!-- G03-WEAPONS-013 -->
- **Partial:** M72 LAW. <!-- G03-WEAPONS-014 -->

### Additional weapons missing

- Cluster grenades. <!-- G03-WEAPONS-015 -->
- Flamethrower. <!-- G03-WEAPONS-016 -->
- Rambo Bow. <!-- G03-WEAPONS-017 -->
- Normal arrows. <!-- G03-WEAPONS-018 -->
- Flamed arrows. <!-- G03-WEAPONS-019 -->
- Stationary M2 machine gun. <!-- G03-WEAPONS-020 -->
- Punch/unarmed combat. <!-- G03-WEAPONS-021 -->

### Missing weapon systems

- Primary-plus-secondary inventory slots. <!-- G03-WEAPONS-022 -->
- Carrying two primary weapons. <!-- G03-WEAPONS-023 -->
- Weapon pickups. <!-- G03-WEAPONS-024 -->
- Weapon dropping. <!-- G03-WEAPONS-025 -->
- Thrown-weapon physics. <!-- G03-WEAPONS-026 -->
- Thrown combat knife. <!-- G03-WEAPONS-027 -->
- Knife recovery/pickup. <!-- G03-WEAPONS-028 -->
- Manual reload. <!-- G03-WEAPONS-029 -->
- Reload interruption. <!-- G03-WEAPONS-030 -->
- Per-weapon reload animations. <!-- G03-WEAPONS-031 -->
- Weapon-switch delays. <!-- G03-WEAPONS-032 -->
- Correct weapon startup behavior. <!-- G03-WEAPONS-033 -->
- LAW firing restrictions. <!-- G03-WEAPONS-034 -->
- Barrett movement/startup restrictions. <!-- G03-WEAPONS-035 -->
- Minigun spin-up behavior. <!-- G03-WEAPONS-036 -->
- Chainsaw continuous-contact behavior. <!-- G03-WEAPONS-037 -->
- Proper melee collision. <!-- G03-WEAPONS-038 -->
- Dual Desert Eagle projectiles and muzzle positions. <!-- G03-WEAPONS-039 -->
- Proper shotgun pellet count and randomized spread. <!-- G03-WEAPONS-040 -->
- Projectile lifetime matching Soldat. <!-- G03-WEAPONS-041 -->
- Projectile gravity per bullet style. <!-- G03-WEAPONS-042 -->
- Grenade bouncing. <!-- G03-WEAPONS-043 -->
- M79 projectile bouncing/impact behavior. <!-- G03-WEAPONS-044 -->
- Grenade cooking/throw strength. <!-- G03-WEAPONS-045 -->
- Grenade fuse timing. <!-- G03-WEAPONS-046 -->
- Dropped grenade behavior on death. <!-- G03-WEAPONS-047 -->
- Cluster grenade submunition spawning. <!-- G03-WEAPONS-048 -->
- Arrow sticking/interaction. <!-- G03-WEAPONS-049 -->
- Flame propagation and burning. <!-- G03-WEAPONS-050 -->
- Flamethrower fuel/ammunition behavior. <!-- G03-WEAPONS-051 -->
- Stationary-gun mounting and dismounting. <!-- G03-WEAPONS-052 -->
- Stationary-gun aiming limits. <!-- G03-WEAPONS-053 -->
- Projectile-to-projectile or projectile-to-object interactions where applicable. <!-- G03-WEAPONS-054 -->
- Muzzle origin based on character pose. <!-- G03-WEAPONS-055 -->
- Muzzle flashes. <!-- G03-WEAPONS-056 -->
- Shell casings. <!-- G03-WEAPONS-057 -->
- Weapon-specific sounds. <!-- G03-WEAPONS-058 -->
- Reload sounds. <!-- G03-WEAPONS-059 -->
- Empty-magazine sound. <!-- G03-WEAPONS-060 -->
- Bullet impact effects. <!-- G03-WEAPONS-061 -->
- Tracers matching weapon configuration. <!-- G03-WEAPONS-062 -->
- Explosion visual and audio effects. <!-- G03-WEAPONS-063 -->
- Weapon sprites held by characters. <!-- G03-WEAPONS-064 -->
- Weapons lying on the ground. <!-- G03-WEAPONS-065 -->
- Weapon pickup indicators. <!-- G03-WEAPONS-066 -->

[Weapons reference](https://wiki.soldat.pl/index.php/Weapons)

## 4. Weapon Mod parity

The project currently stores only a subset of Soldat’s weapon configuration fields.

Every weapon needs support for:

- Damage. <!-- G04-WEAPON-MOD-001 -->
- Fire interval. <!-- G04-WEAPON-MOD-002 -->
- Ammunition capacity. <!-- G04-WEAPON-MOD-003 -->
- Reload time. <!-- G04-WEAPON-MOD-004 -->
- Projectile speed. <!-- G04-WEAPON-MOD-005 -->
- Bullet style. <!-- G04-WEAPON-MOD-006 -->
- Startup time. <!-- G04-WEAPON-MOD-007 -->
- Bink. <!-- G04-WEAPON-MOD-008 -->
- Self-bink through negative bink values. <!-- G04-WEAPON-MOD-009 -->
- Movement accuracy. <!-- G04-WEAPON-MOD-010 -->
- Bullet spread. <!-- G04-WEAPON-MOD-011 -->
- Recoil. <!-- G04-WEAPON-MOD-012 -->
- Push. <!-- G04-WEAPON-MOD-013 -->
- Inherited velocity. <!-- G04-WEAPON-MOD-014 -->
- Head damage modifier. <!-- G04-WEAPON-MOD-015 -->
- Chest damage modifier. <!-- G04-WEAPON-MOD-016 -->
- Leg damage modifier. <!-- G04-WEAPON-MOD-017 -->
- Separate normal and Realistic weapon tables. <!-- G04-WEAPON-MOD-018 -->
- Cluster-grenade nesting under frag grenades. <!-- G04-WEAPON-MOD-019 -->
- Server-selected/custom weapon mods. <!-- G04-WEAPON-MOD-020 -->
- Weapon-mod synchronization with clients. <!-- G04-WEAPON-MOD-021 -->
- Validation of custom weapon values. <!-- G04-WEAPON-MOD-022 -->
- A safe loader for `weapons.ini`-style data. <!-- G04-WEAPON-MOD-023 -->
- Version/hash checking so clients know which mod is active. <!-- G04-WEAPON-MOD-024 -->
- Weapon-mod display in the room/server browser. <!-- G04-WEAPON-MOD-025 -->

### Damage and accuracy corrections

- Replace simplified damage with `Damage × CurrentSpeed × HitboxModifier`. <!-- G04-WEAPON-MOD-026 -->
- Add projectile speed decay. <!-- G04-WEAPON-MOD-027 -->
- Add player velocity inheritance. <!-- G04-WEAPON-MOD-028 -->
- Allow movement direction to increase or decrease projectile speed and damage. <!-- G04-WEAPON-MOD-029 -->
- Add head, chest, and leg hit detection. <!-- G04-WEAPON-MOD-030 -->
- Add bink when the player is hit. <!-- G04-WEAPON-MOD-031 -->
- Add self-bink when firing relevant weapons. <!-- G04-WEAPON-MOD-032 -->
- Add cursor expansion representing current accuracy. <!-- G04-WEAPON-MOD-033 -->
- Add movement accuracy penalties. <!-- G04-WEAPON-MOD-034 -->
- Add stronger jetting accuracy penalties. <!-- G04-WEAPON-MOD-035 -->
- Add recoil to the aim/cursor. <!-- G04-WEAPON-MOD-036 -->
- Add correct bullet spread. <!-- G04-WEAPON-MOD-037 -->
- Add bullet mass/push behavior. <!-- G04-WEAPON-MOD-038 -->
- Add distance-dependent damage caused by changing projectile speed. <!-- G04-WEAPON-MOD-039 -->
- Add exact explosive damage and falloff. <!-- G04-WEAPON-MOD-040 -->
- Add terrain occlusion for splash damage if Soldat’s source behavior requires it. <!-- G04-WEAPON-MOD-041 -->
- Add friendly-fire and team-bink rules. <!-- G04-WEAPON-MOD-042 -->

[Weapon Mod reference](https://wiki.soldat.pl/index.php/Weapon_Mod)

## 5. Game modes

### Existing

- **Partial:** Deathmatch. <!-- G05-GAME-MODES-001 -->
- **Partial:** Team Deathmatch, corresponding to Soldat’s Teammatch. <!-- G05-GAME-MODES-002 -->

These modes still lack round limits, time limits, proper spawning, map rotation, end-of-round state, announcements, and complete scoring.

### Missing official modes

- Pointmatch. <!-- G05-GAME-MODES-003 -->
- Rambomatch. <!-- G05-GAME-MODES-004 -->
- Capture the Flag. <!-- G05-GAME-MODES-005 -->
- Infiltration. <!-- G05-GAME-MODES-006 -->
- Hold the Flag. <!-- G05-GAME-MODES-007 -->

### Missing mode modifiers

- Realistic mode. <!-- G05-GAME-MODES-008 -->
- Survival mode. <!-- G05-GAME-MODES-009 -->
- Advance mode. <!-- G05-GAME-MODES-010 -->

### Missing community modes mentioned by the wiki

These are not necessary for initial Soldat parity, but they belong in the exhaustive feature backlog:

- Climb. <!-- G05-GAME-MODES-011 -->
- Dodgeball. <!-- G05-GAME-MODES-012 -->
- Domination. <!-- G05-GAME-MODES-013 -->
- Hide and Seek. <!-- G05-GAME-MODES-014 -->
- Knife Only. <!-- G05-GAME-MODES-015 -->
- OneShots. <!-- G05-GAME-MODES-016 -->
- Pirates vs Ninjas. <!-- G05-GAME-MODES-017 -->
- Realistic Soldat/Counter-Strike. <!-- G05-GAME-MODES-018 -->
- Trench Wars. <!-- G05-GAME-MODES-019 -->
- Tactical Trench Wars. <!-- G05-GAME-MODES-020 -->
- Zombie. <!-- G05-GAME-MODES-021 -->

### Missing match rules

- Kill limit. <!-- G05-GAME-MODES-022 -->
- Point limit. <!-- G05-GAME-MODES-023 -->
- Capture limit. <!-- G05-GAME-MODES-024 -->
- Time limit. <!-- G05-GAME-MODES-025 -->
- Round start countdown. <!-- G05-GAME-MODES-026 -->
- Round end. <!-- G05-GAME-MODES-027 -->
- Winner calculation. <!-- G05-GAME-MODES-028 -->
- Draw handling. <!-- G05-GAME-MODES-029 -->
- Overtime policy where appropriate. <!-- G05-GAME-MODES-030 -->
- Map rotation. <!-- G05-GAME-MODES-031 -->
- Map-loop option. <!-- G05-GAME-MODES-032 -->
- Next-map transition. <!-- G05-GAME-MODES-033 -->
- Match restart. <!-- G05-GAME-MODES-034 -->
- Configurable respawn time. <!-- G05-GAME-MODES-035 -->
- Survival round elimination. <!-- G05-GAME-MODES-036 -->
- Survival dead-player restrictions. <!-- G05-GAME-MODES-037 -->
- Advance-mode weapon unlocking. <!-- G05-GAME-MODES-038 -->
- Team balancing. <!-- G05-GAME-MODES-039 -->
- Team selection. <!-- G05-GAME-MODES-040 -->
- Spectator team. <!-- G05-GAME-MODES-041 -->
- Mid-match spectator switching. <!-- G05-GAME-MODES-042 -->
- Friendly-fire configuration. <!-- G05-GAME-MODES-043 -->
- Teamkill handling. <!-- G05-GAME-MODES-044 -->
- Suicide scoring. <!-- G05-GAME-MODES-045 -->
- Disconnect/reconnect score preservation. <!-- G05-GAME-MODES-046 -->
- Automatic round-end scoreboard. <!-- G05-GAME-MODES-047 -->
- Automatic final-score screenshot option. <!-- G05-GAME-MODES-048 -->

[Game modes reference](https://wiki.soldat.pl/index.php/Game_Modes)

## 6. Pointmatch

- Yellow point flag. <!-- G06-POINTMATCH-001 -->
- Holding the point flag. <!-- G06-POINTMATCH-002 -->
- Extra points awarded while holding it. <!-- G06-POINTMATCH-003 -->
- Flag drop on death. <!-- G06-POINTMATCH-004 -->
- Point-limit victory. <!-- G06-POINTMATCH-005 -->
- Pointmatch-specific scoring. <!-- G06-POINTMATCH-006 -->
- Pointmatch spawns and maps. <!-- G06-POINTMATCH-007 -->
- Point-flag HUD status. <!-- G06-POINTMATCH-008 -->

## 7. Rambomatch

- Rambo Bow spawn. <!-- G07-RAMBOMATCH-001 -->
- Bow pickup. <!-- G07-RAMBOMATCH-002 -->
- Only the Rambo player earning kills/points under the mode’s rules. <!-- G07-RAMBOMATCH-003 -->
- Rambo target indication. <!-- G07-RAMBOMATCH-004 -->
- Bow drop and reacquisition. <!-- G07-RAMBOMATCH-005 -->
- Rambo-specific respawn behavior. <!-- G07-RAMBOMATCH-006 -->
- Flamed-arrow support. <!-- G07-RAMBOMATCH-007 -->
- Rambomatch scoring and win limit. <!-- G07-RAMBOMATCH-008 -->
- Rambomatch HUD. <!-- G07-RAMBOMATCH-009 -->

## 8. Capture the Flag

- Alpha and Bravo teams. <!-- G08-CAPTURE-THE-001 -->
- Red and blue flags. <!-- G08-CAPTURE-THE-002 -->
- Flag bases. <!-- G08-CAPTURE-THE-003 -->
- Enemy-flag pickup. <!-- G08-CAPTURE-THE-004 -->
- Flag carrying. <!-- G08-CAPTURE-THE-005 -->
- Flag dropping on death. <!-- G08-CAPTURE-THE-006 -->
- Manual flag throw. <!-- G08-CAPTURE-THE-007 -->
- Flag return by touching a dropped friendly flag. <!-- G08-CAPTURE-THE-008 -->
- Automatic return timeout if applicable. <!-- G08-CAPTURE-THE-009 -->
- Capture only when the player’s own flag is at base. <!-- G08-CAPTURE-THE-010 -->
- Capture scoring. <!-- G08-CAPTURE-THE-011 -->
- Capture limit. <!-- G08-CAPTURE-THE-012 -->
- Flag-carrier indicator. <!-- G08-CAPTURE-THE-013 -->
- Missing-flag indicator. <!-- G08-CAPTURE-THE-014 -->
- Flag status HUD. <!-- G08-CAPTURE-THE-015 -->
- Team score HUD. <!-- G08-CAPTURE-THE-016 -->
- Flag physics. <!-- G08-CAPTURE-THE-017 -->
- Bullets and explosions pushing flags. <!-- G08-CAPTURE-THE-018 -->
- Flag collision with polygons. <!-- G08-CAPTURE-THE-019 -->
- CTF spawn points. <!-- G08-CAPTURE-THE-020 -->
- CTF-compatible map validation. <!-- G08-CAPTURE-THE-021 -->
- CTF bots and flag objectives. <!-- G08-CAPTURE-THE-022 -->

## 9. Infiltration

- Attacking and defending teams. <!-- G09-INFILTRATION-001 -->
- Black/white or objective-specific flags. <!-- G09-INFILTRATION-002 -->
- Objective capture rules. <!-- G09-INFILTRATION-003 -->
- Passive defender scoring. <!-- G09-INFILTRATION-004 -->
- Attacker capture scoring. <!-- G09-INFILTRATION-005 -->
- Team-role asymmetry. <!-- G09-INFILTRATION-006 -->
- Infiltration-specific spawn points. <!-- G09-INFILTRATION-007 -->
- Infiltration timer and score rules. <!-- G09-INFILTRATION-008 -->
- Team score HUD. <!-- G09-INFILTRATION-009 -->
- Objective state indicators. <!-- G09-INFILTRATION-010 -->
- Infiltration bot behavior. <!-- G09-INFILTRATION-011 -->

## 10. Hold the Flag

- Neutral yellow flag. <!-- G10-HOLD-THE-001 -->
- Flag pickup and carrying. <!-- G10-HOLD-THE-002 -->
- Continuous team scoring while held. <!-- G10-HOLD-THE-003 -->
- Flag drops. <!-- G10-HOLD-THE-004 -->
- Flag return/reset rules. <!-- G10-HOLD-THE-005 -->
- Carrier indication. <!-- G10-HOLD-THE-006 -->
- HTF-specific spawn points. <!-- G10-HOLD-THE-007 -->
- HTF score display. <!-- G10-HOLD-THE-008 -->
- HTF bots and objective behavior. <!-- G10-HOLD-THE-009 -->

## 11. Realistic mode

- Separate `weapons_realistic.ini` statistics. <!-- G11-REALISTIC-MODE-001 -->
- Reduced/changed weapon damage behavior. <!-- G11-REALISTIC-MODE-002 -->
- Recoil behavior appropriate to Realistic. <!-- G11-REALISTIC-MODE-003 -->
- Visibility/line-of-sight restrictions. <!-- G11-REALISTIC-MODE-004 -->
- Enemies visible only when the observed player can see them. <!-- G11-REALISTIC-MODE-005 -->
- Dead-player and spectator visibility restrictions. <!-- G11-REALISTIC-MODE-006 -->
- Enemy team-chat visibility restrictions. <!-- G11-REALISTIC-MODE-007 -->
- Fall damage if required by the reference implementation. <!-- G11-REALISTIC-MODE-008 -->
- Realistic movement and survival tuning. <!-- G11-REALISTIC-MODE-009 -->
- Realistic-specific HUD behavior. <!-- G11-REALISTIC-MODE-010 -->
- Server/room Realistic flag. <!-- G11-REALISTIC-MODE-011 -->

## 12. Survival mode

- No immediate respawn. <!-- G12-SURVIVAL-MODE-001 -->
- Round-based respawning. <!-- G12-SURVIVAL-MODE-002 -->
- Round begins when enough players are ready. <!-- G12-SURVIVAL-MODE-003 -->
- Round ends when one player/team remains. <!-- G12-SURVIVAL-MODE-004 -->
- Dead players spectate. <!-- G12-SURVIVAL-MODE-005 -->
- Survival scoreboard. <!-- G12-SURVIVAL-MODE-006 -->
- End-of-round state. <!-- G12-SURVIVAL-MODE-007 -->
- Flag restrictions after a Survival round ends. <!-- G12-SURVIVAL-MODE-008 -->
- Survival chat/spectator restrictions. <!-- G12-SURVIVAL-MODE-009 -->
- Configurable survival respawn/round behavior. <!-- G12-SURVIVAL-MODE-010 -->

## 13. Advance mode

- Initial limited weapon selection. <!-- G13-ADVANCE-MODE-001 -->
- Unlock weapons through kills. <!-- G13-ADVANCE-MODE-002 -->
- Unlock progression. <!-- G13-ADVANCE-MODE-003 -->
- Per-player unlock state. <!-- G13-ADVANCE-MODE-004 -->
- Advance weapon menu. <!-- G13-ADVANCE-MODE-005 -->
- Reset progression between matches/maps as appropriate. <!-- G13-ADVANCE-MODE-006 -->
- Advance configuration. <!-- G13-ADVANCE-MODE-007 -->

## 14. Bonus kits

All bonus kits and spawning logic are missing:

- Medic Kit: restore health to maximum. <!-- G14-BONUS-KITS-001 -->
- Grenades Kit: restore grenades to configured maximum. <!-- G14-BONUS-KITS-002 -->
- Cluster Grenades Kit: grant three cluster grenades. <!-- G14-BONUS-KITS-003 -->
- Bulletproof Vest Kit: add approximately another full health bar as armor. <!-- G14-BONUS-KITS-004 -->
- Flame God Kit: flamethrower plus temporary invulnerability. <!-- G14-BONUS-KITS-005 -->
- Berserker Kit: four-times weapon damage temporarily. <!-- G14-BONUS-KITS-006 -->
- Predator Kit: temporary invisibility. <!-- G14-BONUS-KITS-007 -->

Supporting systems still needed:

- Bonus spawn points. <!-- G14-BONUS-KITS-008 -->
- Configurable kit frequency. <!-- G14-BONUS-KITS-009 -->
- Kit respawn timers. <!-- G14-BONUS-KITS-010 -->
- Pickup collision. <!-- G14-BONUS-KITS-011 -->
- Pickup sounds and effects. <!-- G14-BONUS-KITS-012 -->
- Active-bonus HUD. <!-- G14-BONUS-KITS-013 -->
- Bonus countdown. <!-- G14-BONUS-KITS-014 -->
- Bonus overlay/effect. <!-- G14-BONUS-KITS-015 -->
- Armor HUD. <!-- G14-BONUS-KITS-016 -->
- Predator visibility affected by blood. <!-- G14-BONUS-KITS-017 -->
- Predator still producing audible sounds. <!-- G14-BONUS-KITS-018 -->
- Bonus expiration. <!-- G14-BONUS-KITS-019 -->
- Bonus replacement/stacking rules. <!-- G14-BONUS-KITS-020 -->
- Server enable/disable settings. <!-- G14-BONUS-KITS-021 -->
- Kits affected by projectile push. <!-- G14-BONUS-KITS-022 -->

[Bonuses reference](https://wiki.soldat.pl/index.php/Bonuses)

## 15. Health, armor, death, and respawn

- **Present:** Basic 100 HP. <!-- G15-HEALTH-ARMOR-001 -->
- **Present:** Death count. <!-- G15-HEALTH-ARMOR-002 -->
- **Present:** Automatic respawn. <!-- G15-HEALTH-ARMOR-003 -->
- **Missing:** Body-part damage. <!-- G15-HEALTH-ARMOR-004 -->
- **Missing:** Bulletproof vest/armor. <!-- G15-HEALTH-ARMOR-005 -->
- **Missing:** Bleeding. <!-- G15-HEALTH-ARMOR-006 -->
- **Missing:** Blood particles. <!-- G15-HEALTH-ARMOR-007 -->
- **Missing:** Blood remaining on the character. <!-- G15-HEALTH-ARMOR-008 -->
- **Missing:** Gore/gibs. <!-- G15-HEALTH-ARMOR-009 -->
- **Missing:** Ragdoll corpses. <!-- G15-HEALTH-ARMOR-010 -->
- **Missing:** Corpse persistence. <!-- G15-HEALTH-ARMOR-011 -->
- **Missing:** Death causes. <!-- G15-HEALTH-ARMOR-012 -->
- **Missing:** Kill feed. <!-- G15-HEALTH-ARMOR-013 -->
- **Missing:** Headshot messaging/effects. <!-- G15-HEALTH-ARMOR-014 -->
- **Missing:** Multi-kill messages. <!-- G15-HEALTH-ARMOR-015 -->
- **Missing:** Self-kill messages. <!-- G15-HEALTH-ARMOR-016 -->
- **Missing:** Teamkill messages. <!-- G15-HEALTH-ARMOR-017 -->
- **Missing:** Spawn protection if applicable. <!-- G15-HEALTH-ARMOR-018 -->
- **Missing:** Configurable respawn time. <!-- G15-HEALTH-ARMOR-019 -->
- **Missing:** Respawn countdown HUD. <!-- G15-HEALTH-ARMOR-020 -->
- **Missing:** Weapon selection while dead. <!-- G15-HEALTH-ARMOR-021 -->
- **Missing:** Proper mode/team/map spawn selection. <!-- G15-HEALTH-ARMOR-022 -->
- **Missing:** Kill attribution after delayed damage. <!-- G15-HEALTH-ARMOR-023 -->
- **Missing:** Assist tracking if desired. <!-- G15-HEALTH-ARMOR-024 -->
- **Missing:** Damage direction feedback. <!-- G15-HEALTH-ARMOR-025 -->
- **Missing:** Explosion deafness/whistling effect. <!-- G15-HEALTH-ARMOR-026 -->

## 16. Maps and terrain engine

The present arena is not a Soldat map. Add:

- **Present:** `.pms` map loader. <!-- G16-MAPS-AND-001 -->
- **Present:** PMS format validation. <!-- G16-MAPS-AND-002 -->
- **Present:** Polygon geometry. <!-- G16-MAPS-AND-003 -->
- **Present:** Polygon types and properties. <!-- G16-MAPS-AND-004 -->
- **Present:** Texture coordinates. <!-- G16-MAPS-AND-005 -->
- **Present:** Map textures. <!-- G16-MAPS-AND-006 -->
- Edge textures. <!-- G16-MAPS-AND-007 -->
- **Present:** Scenery objects. <!-- G16-MAPS-AND-008 -->
- **Present:** Scenery depth/layers. <!-- G16-MAPS-AND-009 -->
- Animated scenery if supported. <!-- G16-MAPS-AND-010 -->
- **Present:** Colliders. <!-- G16-MAPS-AND-011 -->
- **Present:** Spawn points. <!-- G16-MAPS-AND-012 -->
- **Present:** Player spawn types. <!-- G16-MAPS-AND-013 -->
- **Present:** Team spawn types. <!-- G16-MAPS-AND-014 -->
- Flag spawn types. <!-- G16-MAPS-AND-015 -->
- Bonus-kit spawn types. <!-- G16-MAPS-AND-016 -->
- Grenade spawn types where relevant. <!-- G16-MAPS-AND-017 -->
- Stationary-gun locations. <!-- G16-MAPS-AND-018 -->
- **Present:** Bot waypoints. <!-- G16-MAPS-AND-019 -->
- **Present:** Background colors and gradients. <!-- G16-MAPS-AND-020 -->
- **Present:** Weather settings. <!-- G16-MAPS-AND-021 -->
- **Present:** Footstep-sound property. <!-- G16-MAPS-AND-022 -->
- **Present:** Map-specific jet fuel. <!-- G16-MAPS-AND-023 -->
- Map boundaries. <!-- G16-MAPS-AND-024 -->
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
- Map voting or polling if the intended Soldat server experience includes it. <!-- G16-MAPS-AND-036 -->
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

- Bot entities driven by the authoritative simulation. <!-- G18-BOTS-AND-001 -->
- Random-bot count. <!-- G18-BOTS-AND-002 -->
- Per-team bot count. <!-- G18-BOTS-AND-003 -->
- Bot difficulty. <!-- G18-BOTS-AND-004 -->
- Bot accuracy levels. <!-- G18-BOTS-AND-005 -->
- Bot reaction time. <!-- G18-BOTS-AND-006 -->
- Bot movement. <!-- G18-BOTS-AND-007 -->
- Jet navigation. <!-- G18-BOTS-AND-008 -->
- Crouching, prone, rolls, and backflips. <!-- G18-BOTS-AND-009 -->
- Weapon selection. <!-- G18-BOTS-AND-010 -->
- Reloading. <!-- G18-BOTS-AND-011 -->
- Grenade use. <!-- G18-BOTS-AND-012 -->
- Weapon pickup. <!-- G18-BOTS-AND-013 -->
- Bonus pickup. <!-- G18-BOTS-AND-014 -->
- Waypoint navigation. <!-- G18-BOTS-AND-015 -->
- Recovery when stuck. <!-- G18-BOTS-AND-016 -->
- Deathmatch target selection. <!-- G18-BOTS-AND-017 -->
- Team coordination. <!-- G18-BOTS-AND-018 -->
- CTF attacking, defending, returning, and capturing. <!-- G18-BOTS-AND-019 -->
- Infiltration objectives. <!-- G18-BOTS-AND-020 -->
- HTF carrier support. <!-- G18-BOTS-AND-021 -->
- Rambomatch behavior. <!-- G18-BOTS-AND-022 -->
- Survival behavior. <!-- G18-BOTS-AND-023 -->
- Bot chat. <!-- G18-BOTS-AND-024 -->
- Custom bot profiles. <!-- G18-BOTS-AND-025 -->
- Server commands to add/remove bots. <!-- G18-BOTS-AND-026 -->

## 19. HUD and game screen

### Existing or partial

- **Present:** Numeric health. <!-- G19-HUD-AND-001 -->
- **Present:** Numeric jet fuel. <!-- G19-HUD-AND-002 -->
- **Present:** Ammo and grenade count. <!-- G19-HUD-AND-003 -->
- **Present:** Kills/deaths. <!-- G19-HUD-AND-004 -->
- **Present:** Scoreboard. <!-- G19-HUD-AND-005 -->
- **Partial:** Team coloring. <!-- G19-HUD-AND-006 -->

### Missing Soldat HUD elements

- Red health bar. <!-- G19-HUD-AND-007 -->
- Yellow ammunition/reload bar. <!-- G19-HUD-AND-008 -->
- Bullet count positioned with the ammo display. <!-- G19-HUD-AND-009 -->
- Fire-interval bar. <!-- G19-HUD-AND-010 -->
- Blue jet-fuel bar. <!-- G19-HUD-AND-011 -->
- Server rank. <!-- G19-HUD-AND-012 -->
- Current kills/points. <!-- G19-HUD-AND-013 -->
- Difference from the leader. <!-- G19-HUD-AND-014 -->
- Kill/point/capture limit. <!-- G19-HUD-AND-015 -->
- Alpha team score. <!-- G19-HUD-AND-016 -->
- Bravo team score. <!-- G19-HUD-AND-017 -->
- Charlie team score. <!-- G19-HUD-AND-018 -->
- Delta team score. <!-- G19-HUD-AND-019 -->
- Missing-flag indicators. <!-- G19-HUD-AND-020 -->
- Flag-carrier state. <!-- G19-HUD-AND-021 -->
- Bonus status and duration. <!-- G19-HUD-AND-022 -->
- Armor indicator. <!-- G19-HUD-AND-023 -->
- Weapon image. <!-- G19-HUD-AND-024 -->
- Secondary weapon indicator. <!-- G19-HUD-AND-025 -->
- Grenade type/count. <!-- G19-HUD-AND-026 -->
- Reload progress rather than only “RELOADING.” <!-- G19-HUD-AND-027 -->
- Respawn countdown. <!-- G19-HUD-AND-028 -->
- Kill feed. <!-- G19-HUD-AND-029 -->
- Chat overlay inside the game. <!-- G19-HUD-AND-030 -->
- Team-chat distinction. <!-- G19-HUD-AND-031 -->
- Server messages. <!-- G19-HUD-AND-032 -->
- Connection/ping indicator. <!-- G19-HUD-AND-033 -->
- Ping dot with size/color grading. <!-- G19-HUD-AND-034 -->
- FPS display. <!-- G19-HUD-AND-035 -->
- Network-bandwidth display. <!-- G19-HUD-AND-036 -->
- Minimap. <!-- G19-HUD-AND-037 -->
- Sniper line. <!-- G19-HUD-AND-038 -->
- Crosshair accuracy/bink visualization. <!-- G19-HUD-AND-039 -->
- Spectator HUD. <!-- G19-HUD-AND-040 -->
- End-of-round screen. <!-- G19-HUD-AND-041 -->
- Weapon-statistics screen. <!-- G19-HUD-AND-042 -->
- Scrollable large scoreboard. <!-- G19-HUD-AND-043 -->
- Player IDs on the command-enabled scoreboard. <!-- G19-HUD-AND-044 -->
- Custom HUD/interface loading. <!-- G19-HUD-AND-045 -->
- HUD scaling. <!-- G19-HUD-AND-046 -->
- Safe-area and resolution tests across desktop and mobile. <!-- G19-HUD-AND-047 -->

[Game Screen reference](https://wiki.soldat.pl/index.php/Game_Screen)

## 20. Rendering and visual effects

- Full animated soldier/gostek rendering. <!-- G20-RENDERING-AND-001 -->
- Separate body parts. <!-- G20-RENDERING-AND-002 -->
- Hair. <!-- G20-RENDERING-AND-003 -->
- Headgear. <!-- G20-RENDERING-AND-004 -->
- Helmet/hat/none. <!-- G20-RENDERING-AND-005 -->
- Chains and dog tags. <!-- G20-RENDERING-AND-006 -->
- Shirt, pants, skin, hair, shoes, and jet colors. <!-- G20-RENDERING-AND-007 -->
- Weapon sprites. <!-- G20-RENDERING-AND-008 -->
- Character pose matching aim angle. <!-- G20-RENDERING-AND-009 -->
- Muzzle flashes. <!-- G20-RENDERING-AND-010 -->
- Bullet trails. <!-- G20-RENDERING-AND-011 -->
- Shell casings. <!-- G20-RENDERING-AND-012 -->
- Sparks. <!-- G20-RENDERING-AND-013 -->
- Blood. <!-- G20-RENDERING-AND-014 -->
- Gore. <!-- G20-RENDERING-AND-015 -->
- Explosion animation. <!-- G20-RENDERING-AND-016 -->
- Smoke. <!-- G20-RENDERING-AND-017 -->
- Fire. <!-- G20-RENDERING-AND-018 -->
- Burning characters. <!-- G20-RENDERING-AND-019 -->
- Grenade sprites. <!-- G20-RENDERING-AND-020 -->
- Arrow sprites. <!-- G20-RENDERING-AND-021 -->
- Dropped weapons. <!-- G20-RENDERING-AND-022 -->
- Flags. <!-- G20-RENDERING-AND-023 -->
- Kits. <!-- G20-RENDERING-AND-024 -->
- Stationary gun. <!-- G20-RENDERING-AND-025 -->
- Polygon textures. <!-- G20-RENDERING-AND-026 -->
- Edge textures. <!-- G20-RENDERING-AND-027 -->
- Background scenery. <!-- G20-RENDERING-AND-028 -->
- Foreground scenery. <!-- G20-RENDERING-AND-029 -->
- Rain. <!-- G20-RENDERING-AND-030 -->
- Snow. <!-- G20-RENDERING-AND-031 -->
- Wind effects. <!-- G20-RENDERING-AND-032 -->
- Bullet-time visual effect. <!-- G20-RENDERING-AND-033 -->
- Predator transparency. <!-- G20-RENDERING-AND-034 -->
- Berserker overlay. <!-- G20-RENDERING-AND-035 -->
- Flame God overlay. <!-- G20-RENDERING-AND-036 -->
- Damage feedback. <!-- G20-RENDERING-AND-037 -->
- Screen shake where appropriate. <!-- G20-RENDERING-AND-038 -->
- Resolution scaling. <!-- G20-RENDERING-AND-039 -->
- Texture filtering settings. <!-- G20-RENDERING-AND-040 -->
- Mipmapping. <!-- G20-RENDERING-AND-041 -->
- Low-particle modes. <!-- G20-RENDERING-AND-042 -->
- Compatibility rendering path. <!-- G20-RENDERING-AND-043 -->
- Custom-interface graphics. <!-- G20-RENDERING-AND-044 -->
- Mod-controlled asset scaling through `mod.ini`-like rules. <!-- G20-RENDERING-AND-045 -->

## 21. Sound and music

There is currently no meaningful Soldat-like audio system.

Add:

- Weapon-specific firing sounds. <!-- G21-SOUND-AND-001 -->
- Reload sounds. <!-- G21-SOUND-AND-002 -->
- Empty-weapon sounds. <!-- G21-SOUND-AND-003 -->
- Grenade pin/throw/bounce/explosion sounds. <!-- G21-SOUND-AND-004 -->
- M79 and LAW explosion sounds. <!-- G21-SOUND-AND-005 -->
- Bullet impacts. <!-- G21-SOUND-AND-006 -->
- Ricochets if applicable. <!-- G21-SOUND-AND-007 -->
- Chainsaw loop. <!-- G21-SOUND-AND-008 -->
- Knife sounds. <!-- G21-SOUND-AND-009 -->
- Punch sounds. <!-- G21-SOUND-AND-010 -->
- Flamethrower loop. <!-- G21-SOUND-AND-011 -->
- Character pain. <!-- G21-SOUND-AND-012 -->
- Death sounds. <!-- G21-SOUND-AND-013 -->
- Gore sounds. <!-- G21-SOUND-AND-014 -->
- Footsteps selected by map property. <!-- G21-SOUND-AND-015 -->
- Jetpack sound. <!-- G21-SOUND-AND-016 -->
- Flag pickup/drop/return/capture sounds. <!-- G21-SOUND-AND-017 -->
- Kit pickup sounds. <!-- G21-SOUND-AND-018 -->
- Bonus activation/expiration sounds. <!-- G21-SOUND-AND-019 -->
- UI/menu sounds. <!-- G21-SOUND-AND-020 -->
- Chat/message notification. <!-- G21-SOUND-AND-021 -->
- Distant-battle sounds. <!-- G21-SOUND-AND-022 -->
- Weather sounds. <!-- G21-SOUND-AND-023 -->
- Explosion deafness and whistle effect. <!-- G21-SOUND-AND-024 -->
- Positional audio. <!-- G21-SOUND-AND-025 -->
- Distance attenuation. <!-- G21-SOUND-AND-026 -->
- Master sound volume. <!-- G21-SOUND-AND-027 -->
- Music volume. <!-- G21-SOUND-AND-028 -->
- Music playback. <!-- G21-SOUND-AND-029 -->
- Toggle music. <!-- G21-SOUND-AND-030 -->
- Previous/next track. <!-- G21-SOUND-AND-031 -->
- Sound-quality option. <!-- G21-SOUND-AND-032 -->
- Sound-output/device option where browser APIs permit it. <!-- G21-SOUND-AND-033 -->

## 22. Player profiles and customization

The project currently saves only a guest name.

Add:

- Persistent player profiles. <!-- G22-PLAYER-PROFILES-001 -->
- Multiple profiles. <!-- G22-PLAYER-PROFILES-002 -->
- Profile selection screen. <!-- G22-PLAYER-PROFILES-003 -->
- Per-profile settings. <!-- G22-PLAYER-PROFILES-004 -->
- Per-profile controls. <!-- G22-PLAYER-PROFILES-005 -->
- Per-profile taunts. <!-- G22-PLAYER-PROFILES-006 -->
- Default secondary weapon. <!-- G22-PLAYER-PROFILES-007 -->
- Player name constraints matching the intended rules. <!-- G22-PLAYER-PROFILES-008 -->
- Shirt color. <!-- G22-PLAYER-PROFILES-009 -->
- Pants color. <!-- G22-PLAYER-PROFILES-010 -->
- Skin color. <!-- G22-PLAYER-PROFILES-011 -->
- Hair color. <!-- G22-PLAYER-PROFILES-012 -->
- Shoe color. <!-- G22-PLAYER-PROFILES-013 -->
- Jet-flame color. <!-- G22-PLAYER-PROFILES-014 -->
- Hairstyle. <!-- G22-PLAYER-PROFILES-015 -->
- Headgear. <!-- G22-PLAYER-PROFILES-016 -->
- Chain style. <!-- G22-PLAYER-PROFILES-017 -->
- Interface selection. <!-- G22-PLAYER-PROFILES-018 -->
- Saved mouse sensitivity. <!-- G22-PLAYER-PROFILES-019 -->
- Saved sound/music volume. <!-- G22-PLAYER-PROFILES-020 -->
- Saved graphics settings. <!-- G22-PLAYER-PROFILES-021 -->
- Saved favorite servers. <!-- G22-PLAYER-PROFILES-022 -->
- Profile import/export if desired. <!-- G22-PLAYER-PROFILES-023 -->
- Account-backed persistence if the game moves beyond local profiles. <!-- G22-PLAYER-PROFILES-024 -->

## 23. Chat and taunts

- **Present:** Basic room chat. <!-- G23-CHAT-AND-001 -->
- **Present:** Basic chat rate limiting. <!-- G23-CHAT-AND-002 -->
- **Missing:** Team chat. <!-- G23-CHAT-AND-003 -->
- **Missing:** `^` shorthand for team chat. <!-- G23-CHAT-AND-004 -->
- **Missing:** In-game chat overlay. <!-- G23-CHAT-AND-005 -->
- **Missing:** Chat while actively playing without interfering with controls. <!-- G23-CHAT-AND-006 -->
- **Missing:** Taunt file/configuration. <!-- G23-CHAT-AND-007 -->
- **Missing:** Alt+letter and Alt+number taunts. <!-- G23-CHAT-AND-008 -->
- **Missing:** Per-profile taunts. <!-- G23-CHAT-AND-009 -->
- **Missing:** Command taunts. <!-- G23-CHAT-AND-010 -->
- **Missing:** Chat mute. <!-- G23-CHAT-AND-011 -->
- **Missing:** Mute by player name. <!-- G23-CHAT-AND-012 -->
- **Missing:** Mute by player ID. <!-- G23-CHAT-AND-013 -->
- **Missing:** Spam/flood controls beyond the basic one-second limit. <!-- G23-CHAT-AND-014 -->
- **Missing:** Profanity/moderation options if required. <!-- G23-CHAT-AND-015 -->
- **Missing:** Server announcements. <!-- G23-CHAT-AND-016 -->
- **Missing:** Join/leave messages. <!-- G23-CHAT-AND-017 -->
- **Missing:** Kill/capture announcements. <!-- G23-CHAT-AND-018 -->
- **Missing:** Realistic/Survival chat-visibility rules. <!-- G23-CHAT-AND-019 -->

[Profiles and Taunts](https://wiki.soldat.pl/index.php/Profiles_and_Taunts)

## 24. Player commands

Add a `/` command console and support:

- `/KILL` <!-- G24-PLAYER-COMMANDS-001 -->
- `/BRUTALKILL` <!-- G24-PLAYER-COMMANDS-002 -->
- `/MERCY` <!-- G24-PLAYER-COMMANDS-003 -->
- `/SMOKE` <!-- G24-PLAYER-COMMANDS-004 -->
- `/TABAC` <!-- G24-PLAYER-COMMANDS-005 -->
- `/TAKEOFF` <!-- G24-PLAYER-COMMANDS-006 -->
- `/VICTORY` <!-- G24-PLAYER-COMMANDS-007 -->
- `/PAUSE` <!-- G24-PLAYER-COMMANDS-008 -->
- `/UNPAUSE` <!-- G24-PLAYER-COMMANDS-009 -->

These require their associated character actions, state changes, animations, and server validation.

## 25. Server and administrator commands

Add at least the commands covered by the requested wiki page:

- `/ADDMAP <map>` <!-- G25-SERVER-AND-001 -->
- `/DELMAP <map>` <!-- G25-SERVER-AND-002 -->
- `/ADDBOT<team> <bot>` <!-- G25-SERVER-AND-003 -->
- `/KICK <player or ID>` <!-- G25-SERVER-AND-004 -->
- `/KICKLAST` <!-- G25-SERVER-AND-005 -->
- `/TEMPBAN <minutes> <IP/player>` <!-- G25-SERVER-AND-006 -->
- `/BAN <player or ID>` <!-- G25-SERVER-AND-007 -->
- `/BANIP <IP>` <!-- G25-SERVER-AND-008 -->
- `/UNBAN <IP>` <!-- G25-SERVER-AND-009 -->
- `/MAP <map>` <!-- G25-SERVER-AND-010 -->
- `/RESTART` <!-- G25-SERVER-AND-011 -->
- `/NEXTMAP` <!-- G25-SERVER-AND-012 -->
- `/ADM <player>` <!-- G25-SERVER-AND-013 -->
- `/ADMIP <IP>` <!-- G25-SERVER-AND-014 -->
- `/UNADM <IP>` <!-- G25-SERVER-AND-015 -->
- `/RESPAWNTIME <seconds>` <!-- G25-SERVER-AND-016 -->

The complete server experience also needs:

- Admin authentication. <!-- G25-SERVER-AND-017 -->
- Remote-admin support. <!-- G25-SERVER-AND-018 -->
- Persistent ban list. <!-- G25-SERVER-AND-019 -->
- Persistent admin list. <!-- G25-SERVER-AND-020 -->
- Audit log. <!-- G25-SERVER-AND-021 -->
- Command authorization. <!-- G25-SERVER-AND-022 -->
- Player-ID display. <!-- G25-SERVER-AND-023 -->
- Safe command parsing. <!-- G25-SERVER-AND-024 -->
- Command feedback and errors. <!-- G25-SERVER-AND-025 -->
- Map-list loading. <!-- G25-SERVER-AND-026 -->
- Server configuration reload. <!-- G25-SERVER-AND-027 -->
- Lobby re-registration where applicable. <!-- G25-SERVER-AND-028 -->
- Password changes. <!-- G25-SERVER-AND-029 -->
- Maximum-player changes. <!-- G25-SERVER-AND-030 -->
- Locked mode preventing sensitive runtime changes. <!-- G25-SERVER-AND-031 -->

[In-game Commands](https://wiki.soldat.pl/index.php/In-game_Commands)

## 26. Match and server settings

### Start-game settings

- Game mode. <!-- G26-MATCH-AND-001 -->
- Kill/point limit. <!-- G26-MATCH-AND-002 -->
- Capture limit. <!-- G26-MATCH-AND-003 -->
- Time limit. <!-- G26-MATCH-AND-004 -->
- Survival toggle. <!-- G26-MATCH-AND-005 -->
- Realistic toggle. <!-- G26-MATCH-AND-006 -->
- Advance toggle. <!-- G26-MATCH-AND-007 -->
- Map-list looping. <!-- G26-MATCH-AND-008 -->
- Random bots. <!-- G26-MATCH-AND-009 -->
- Team-specific bots. <!-- G26-MATCH-AND-010 -->

### Network/server settings

- Server name. <!-- G26-MATCH-AND-011 -->
- Server password. <!-- G26-MATCH-AND-012 -->
- Maximum players. <!-- G26-MATCH-AND-013 -->
- Game port. <!-- G26-MATCH-AND-014 -->
- Public/private listing. <!-- G26-MATCH-AND-015 -->
- Team balancing. <!-- G26-MATCH-AND-016 -->
- Maximum allowed ping. <!-- G26-MATCH-AND-017 -->
- Ping-kick behavior. <!-- G26-MATCH-AND-018 -->
- Welcome/server message. <!-- G26-MATCH-AND-019 -->
- Server contact link. <!-- G26-MATCH-AND-020 -->
- Dedicated-server configuration. <!-- G26-MATCH-AND-021 -->
- Custom map list. <!-- G26-MATCH-AND-022 -->
- Custom weapon mod. <!-- G26-MATCH-AND-023 -->
- Friendly fire. <!-- G26-MATCH-AND-024 -->
- Respawn time. <!-- G26-MATCH-AND-025 -->
- Bonus frequency/enabled state. <!-- G26-MATCH-AND-026 -->
- Spectator limits. <!-- G26-MATCH-AND-027 -->
- Admin password/authentication. <!-- G26-MATCH-AND-028 -->
- Logging options. <!-- G26-MATCH-AND-029 -->

### Player settings

- Name. <!-- G26-MATCH-AND-030 -->
- Appearance. <!-- G26-MATCH-AND-031 -->
- Default secondary. <!-- G26-MATCH-AND-032 -->
- Background override. <!-- G26-MATCH-AND-033 -->
- Controls. <!-- G26-MATCH-AND-034 -->
- Mouse sensitivity. <!-- G26-MATCH-AND-035 -->
- Interface selection. <!-- G26-MATCH-AND-036 -->
- Player indicator. <!-- G26-MATCH-AND-037 -->
- Sniper line. <!-- G26-MATCH-AND-038 -->

### Graphics settings

- Fullscreen/windowed. <!-- G26-MATCH-AND-039 -->
- Display resolution. <!-- G26-MATCH-AND-040 -->
- Desktop resolution. <!-- G26-MATCH-AND-041 -->
- Interface scaling. <!-- G26-MATCH-AND-042 -->
- Particle limit. <!-- G26-MATCH-AND-043 -->
- Bullet trails. <!-- G26-MATCH-AND-044 -->
- Weather rendering. <!-- G26-MATCH-AND-045 -->
- Texture filtering. <!-- G26-MATCH-AND-046 -->
- Resolution filtering. <!-- G26-MATCH-AND-047 -->
- Mipmapping. <!-- G26-MATCH-AND-048 -->
- Compatibility/fixed-pipeline equivalent. <!-- G26-MATCH-AND-049 -->
- Intro playback. <!-- G26-MATCH-AND-050 -->
- Final-score screenshot. <!-- G26-MATCH-AND-051 -->
- Clanmatch color behavior. <!-- G26-MATCH-AND-052 -->
- Frame-rate limit or VSync. <!-- G26-MATCH-AND-053 -->
- Performance statistics. <!-- G26-MATCH-AND-054 -->

### Audio settings

- Sound volume. <!-- G26-MATCH-AND-055 -->
- Music volume. <!-- G26-MATCH-AND-056 -->
- Sound quality. <!-- G26-MATCH-AND-057 -->
- Output device. <!-- G26-MATCH-AND-058 -->
- Explosion effect. <!-- G26-MATCH-AND-059 -->
- Distant battle. <!-- G26-MATCH-AND-060 -->
- Game music. <!-- G26-MATCH-AND-061 -->

[Settings reference](https://wiki.soldat.pl/index.php/Settings)

## 27. Lobby and room browser

The current room system is useful but needs:

- Server/room refresh. <!-- G27-LOBBY-AND-001 -->
- Cancel refresh. <!-- G27-LOBBY-AND-002 -->
- Ping measurement. <!-- G27-LOBBY-AND-003 -->
- Ping-all action. <!-- G27-LOBBY-AND-004 -->
- Ping column. <!-- G27-LOBBY-AND-005 -->
- Player count. <!-- G27-LOBBY-AND-006 -->
- Maximum players. <!-- G27-LOBBY-AND-007 -->
- Game mode. <!-- G27-LOBBY-AND-008 -->
- Map name. <!-- G27-LOBBY-AND-009 -->
- Country/region if desired. <!-- G27-LOBBY-AND-010 -->
- Password indicator. <!-- G27-LOBBY-AND-011 -->
- Realistic indicator. <!-- G27-LOBBY-AND-012 -->
- Survival indicator. <!-- G27-LOBBY-AND-013 -->
- Advance indicator. <!-- G27-LOBBY-AND-014 -->
- Weapon-mod indicator. <!-- G27-LOBBY-AND-015 -->
- Version compatibility. <!-- G27-LOBBY-AND-016 -->
- Favorites. <!-- G27-LOBBY-AND-017 -->
- Favorite add/remove. <!-- G27-LOBBY-AND-018 -->
- Direct IP/hostname join if supported. <!-- G27-LOBBY-AND-019 -->
- Port input. <!-- G27-LOBBY-AND-020 -->
- Password input. <!-- G27-LOBBY-AND-021 -->
- Spectator join. <!-- G27-LOBBY-AND-022 -->
- Sort and filter. <!-- G27-LOBBY-AND-023 -->
- Search. <!-- G27-LOBBY-AND-024 -->
- Full-room filtering. <!-- G27-LOBBY-AND-025 -->
- Empty-room filtering. <!-- G27-LOBBY-AND-026 -->
- Public lobby registration. <!-- G27-LOBBY-AND-027 -->
- Reliable server discovery. <!-- G27-LOBBY-AND-028 -->
- Join/download progress. <!-- G27-LOBBY-AND-029 -->
- Cancel connection/download. <!-- G27-LOBBY-AND-030 -->
- Better reconnect state. <!-- G27-LOBBY-AND-031 -->
- Room ownership/host settings. <!-- G27-LOBBY-AND-032 -->
- Room deletion or expiry controls. <!-- G27-LOBBY-AND-033 -->
- Password-protected rooms in addition to invite codes. <!-- G27-LOBBY-AND-034 -->
- Team choice after joining. <!-- G27-LOBBY-AND-035 -->
- Late-join rules. <!-- G27-LOBBY-AND-036 -->
- Abuse-resistant room creation. <!-- G27-LOBBY-AND-037 -->
- Rate limiting for connection and room operations. <!-- G27-LOBBY-AND-038 -->

## 28. Spectating

- Spectator team. <!-- G28-SPECTATING-001 -->
- Join directly as spectator. <!-- G28-SPECTATING-002 -->
- Switch followed player. <!-- G28-SPECTATING-003 -->
- Previous/next player. <!-- G28-SPECTATING-004 -->
- Free camera if desired. <!-- G28-SPECTATING-005 -->
- Spectator HUD. <!-- G28-SPECTATING-006 -->
- Spectator scoreboard. <!-- G28-SPECTATING-007 -->
- Spectator chat restrictions. <!-- G28-SPECTATING-008 -->
- Realistic visibility restrictions. <!-- G28-SPECTATING-009 -->
- Survival restrictions. <!-- G28-SPECTATING-010 -->
- Followed-player minimap/visibility behavior. <!-- G28-SPECTATING-011 -->
- Delay option for competitive matches. <!-- G28-SPECTATING-012 -->

## 29. Scoring and statistics

- Correct scoring for every game mode. <!-- G29-SCORING-AND-001 -->
- Match time. <!-- G29-SCORING-AND-002 -->
- Individual points. <!-- G29-SCORING-AND-003 -->
- Team points. <!-- G29-SCORING-AND-004 -->
- Captures. <!-- G29-SCORING-AND-005 -->
- Flag returns. <!-- G29-SCORING-AND-006 -->
- Current server rank. <!-- G29-SCORING-AND-007 -->
- Score difference from leader. <!-- G29-SCORING-AND-008 -->
- Kill/point/capture limit display. <!-- G29-SCORING-AND-009 -->
- Weapon statistics for the current round. <!-- G29-SCORING-AND-010 -->
- Shots fired. <!-- G29-SCORING-AND-011 -->
- Hits. <!-- G29-SCORING-AND-012 -->
- Accuracy. <!-- G29-SCORING-AND-013 -->
- Kills per weapon. <!-- G29-SCORING-AND-014 -->
- Deaths per weapon/cause. <!-- G29-SCORING-AND-015 -->
- Headshots. <!-- G29-SCORING-AND-016 -->
- Suicides. <!-- G29-SCORING-AND-017 -->
- Teamkills. <!-- G29-SCORING-AND-018 -->
- Objective statistics. <!-- G29-SCORING-AND-019 -->
- End-of-round summary. <!-- G29-SCORING-AND-020 -->
- Match history. <!-- G29-SCORING-AND-021 -->
- Persistent player statistics if desired. <!-- G29-SCORING-AND-022 -->
- Clan/team statistics if desired. <!-- G29-SCORING-AND-023 -->
- Exportable logs. <!-- G29-SCORING-AND-024 -->

## 30. Networking and prediction

The server is authoritative, but only local-player movement gets basic optional prediction.

Add:

- Proper client-side prediction for all movement states. <!-- G30-NETWORKING-AND-001 -->
- Input reconciliation. <!-- G30-NETWORKING-AND-002 -->
- Unacknowledged-input replay. <!-- G30-NETWORKING-AND-003 -->
- Remote-player interpolation. <!-- G30-NETWORKING-AND-004 -->
- Projectile interpolation. <!-- G30-NETWORKING-AND-005 -->
- Extrapolation limits. <!-- G30-NETWORKING-AND-006 -->
- Latency compensation. <!-- G30-NETWORKING-AND-007 -->
- Server rewind/lag compensation if appropriate. <!-- G30-NETWORKING-AND-008 -->
- Clock synchronization. <!-- G30-NETWORKING-AND-009 -->
- Measured ping. <!-- G30-NETWORKING-AND-010 -->
- Packet-loss handling. <!-- G30-NETWORKING-AND-011 -->
- Snapshot delta compression. <!-- G30-NETWORKING-AND-012 -->
- Interest management if maps/player counts grow. <!-- G30-NETWORKING-AND-013 -->
- Binary protocol or more compact encoding if needed. <!-- G30-NETWORKING-AND-014 -->
- Weapon/event prediction. <!-- G30-NETWORKING-AND-015 -->
- Predicted muzzle/projectile effects. <!-- G30-NETWORKING-AND-016 -->
- Rollback correction smoothing. <!-- G30-NETWORKING-AND-017 -->
- Map and ruleset synchronization. <!-- G30-NETWORKING-AND-018 -->
- Disconnect reason handling. <!-- G30-NETWORKING-AND-019 -->
- Robust resumption after page sleep/mobile backgrounding. <!-- G30-NETWORKING-AND-020 -->
- Duplicate-session handling. <!-- G30-NETWORKING-AND-021 -->
- Rate limits for every client message. <!-- G30-NETWORKING-AND-022 -->
- Anti-speedhack/input-frequency validation. <!-- G30-NETWORKING-AND-023 -->
- Fire-rate validation. <!-- G30-NETWORKING-AND-024 -->
- Aim/input sanity validation beyond coordinate bounds. <!-- G30-NETWORKING-AND-025 -->
- Server-authoritative pickups, flags, bonuses, and round state. <!-- G30-NETWORKING-AND-026 -->
- Network load and soak tests. <!-- G30-NETWORKING-AND-027 -->
- High-latency/jitter/loss simulation tests. <!-- G30-NETWORKING-AND-028 -->

## 31. Anti-cheat and abuse resistance

- Server-authoritative collision validation. <!-- G31-ANTI-CHEAT-001 -->
- Server-authoritative weapon selection. <!-- G31-ANTI-CHEAT-002 -->
- Server-authoritative reload and inventory. <!-- G31-ANTI-CHEAT-003 -->
- Input-rate limiting. <!-- G31-ANTI-CHEAT-004 -->
- Movement feasibility validation. <!-- G31-ANTI-CHEAT-005 -->
- Aim-value validation. <!-- G31-ANTI-CHEAT-006 -->
- Chat abuse controls. <!-- G31-ANTI-CHEAT-007 -->
- Connection/IP rate limiting. <!-- G31-ANTI-CHEAT-008 -->
- Room-creation rate limiting. <!-- G31-ANTI-CHEAT-009 -->
- Admin permission validation. <!-- G31-ANTI-CHEAT-010 -->
- Ban enforcement. <!-- G31-ANTI-CHEAT-011 -->
- Temporary bans. <!-- G31-ANTI-CHEAT-012 -->
- Audit logging. <!-- G31-ANTI-CHEAT-013 -->
- Suspicious behavior metrics. <!-- G31-ANTI-CHEAT-014 -->
- Protocol fuzzing. <!-- G31-ANTI-CHEAT-015 -->
- Malformed WebSocket testing. <!-- G31-ANTI-CHEAT-016 -->
- Replay-based cheat investigation. <!-- G31-ANTI-CHEAT-017 -->
- Secure resume tokens. <!-- G31-ANTI-CHEAT-018 -->
- Token expiry/rotation. <!-- G31-ANTI-CHEAT-019 -->
- Deployment-level denial-of-service protection. <!-- G31-ANTI-CHEAT-020 -->
- No trust in client-provided cosmetics or settings that affect gameplay. <!-- G31-ANTI-CHEAT-021 -->

## 32. Menus and overall game flow

- Main menu. <!-- G32-MENUS-AND-001 -->
- Profile selection. <!-- G32-MENUS-AND-002 -->
- Player customization. <!-- G32-MENUS-AND-003 -->
- Join-game screen. <!-- G32-MENUS-AND-004 -->
- Start-game/server screen. <!-- G32-MENUS-AND-005 -->
- Options menu. <!-- G32-MENUS-AND-006 -->
- Controls menu. <!-- G32-MENUS-AND-007 -->
- Weapon-selection screen during respawn. <!-- G32-MENUS-AND-008 -->
- Team-selection screen. <!-- G32-MENUS-AND-009 -->
- Spectator selection. <!-- G32-MENUS-AND-010 -->
- Pause menu. <!-- G32-MENUS-AND-011 -->
- Disconnect confirmation. <!-- G32-MENUS-AND-012 -->
- Map loading screen. <!-- G32-MENUS-AND-013 -->
- Asset-download screen. <!-- G32-MENUS-AND-014 -->
- Round-intro countdown. <!-- G32-MENUS-AND-015 -->
- Round-end screen. <!-- G32-MENUS-AND-016 -->
- Match-results screen. <!-- G32-MENUS-AND-017 -->
- Connection-lost overlay. <!-- G32-MENUS-AND-018 -->
- Version mismatch/update flow. <!-- G32-MENUS-AND-019 -->
- Credits. <!-- G32-MENUS-AND-020 -->
- Help/manual. <!-- G32-MENUS-AND-021 -->
- First-run control tutorial. <!-- G32-MENUS-AND-022 -->
- Mobile onboarding. <!-- G32-MENUS-AND-023 -->

## 33. Custom interfaces and modding

- Loadable HUD/interface definitions. <!-- G33-CUSTOM-INTERFACES-001 -->
- Multiple interface presets. <!-- G33-CUSTOM-INTERFACES-002 -->
- Custom cursor. <!-- G33-CUSTOM-INTERFACES-003 -->
- Custom HUD image positions. <!-- G33-CUSTOM-INTERFACES-004 -->
- Interface scaling. <!-- G33-CUSTOM-INTERFACES-005 -->
- Custom weapon graphics. <!-- G33-CUSTOM-INTERFACES-006 -->
- Custom sounds. <!-- G33-CUSTOM-INTERFACES-007 -->
- Custom character/gostek graphics. <!-- G33-CUSTOM-INTERFACES-008 -->
- `mod.ini`-style asset scaling. <!-- G33-CUSTOM-INTERFACES-009 -->
- Mod selection/launching. <!-- G33-CUSTOM-INTERFACES-010 -->
- Mod preview. <!-- G33-CUSTOM-INTERFACES-011 -->
- Mod packaging. <!-- G33-CUSTOM-INTERFACES-012 -->
- Mod downloading. <!-- G33-CUSTOM-INTERFACES-013 -->
- Mod version/hash matching. <!-- G33-CUSTOM-INTERFACES-014 -->
- Server-required mod support. <!-- G33-CUSTOM-INTERFACES-015 -->
- Safe path handling. <!-- G33-CUSTOM-INTERFACES-016 -->
- Asset-size limits. <!-- G33-CUSTOM-INTERFACES-017 -->
- License/provenance metadata. <!-- G33-CUSTOM-INTERFACES-018 -->

The historical built-in interface names listed by the wiki are:

- Cabbage <!-- G33-CUSTOM-INTERFACES-019 -->
- Classic <!-- G33-CUSTOM-INTERFACES-020 -->
- Lacey V2 <!-- G33-CUSTOM-INTERFACES-021 -->
- Micro1 <!-- G33-CUSTOM-INTERFACES-022 -->
- Military <!-- G33-CUSTOM-INTERFACES-023 -->
- Predator <!-- G33-CUSTOM-INTERFACES-024 -->
- Soldat Style <!-- G33-CUSTOM-INTERFACES-025 -->
- Storm <!-- G33-CUSTOM-INTERFACES-026 -->
- Tech <!-- G33-CUSTOM-INTERFACES-027 -->
- Text <!-- G33-CUSTOM-INTERFACES-028 -->

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

- Record match inputs/events. <!-- G35-DEMO-AND-001 -->
- Deterministic replay format. <!-- G35-DEMO-AND-002 -->
- Replay metadata. <!-- G35-DEMO-AND-003 -->
- Replay playback. <!-- G35-DEMO-AND-004 -->
- Pause. <!-- G35-DEMO-AND-005 -->
- Seek. <!-- G35-DEMO-AND-006 -->
- Fast-forward. <!-- G35-DEMO-AND-007 -->
- Follow player. <!-- G35-DEMO-AND-008 -->
- Free camera. <!-- G35-DEMO-AND-009 -->
- Replay compatibility/versioning. <!-- G35-DEMO-AND-010 -->
- Replay validation. <!-- G35-DEMO-AND-011 -->
- Export or share replay. <!-- G35-DEMO-AND-012 -->
- Demo repair/recovery where feasible. <!-- G35-DEMO-AND-013 -->
- Server-side competitive match recording. <!-- G35-DEMO-AND-014 -->

## 36. Operational and production work

The repository itself identifies several non-gameplay gaps:

- Persistent server data. <!-- G36-OPERATIONAL-AND-001 -->
- Persistent accounts. <!-- G36-OPERATIONAL-AND-002 -->
- Persistent statistics. <!-- G36-OPERATIONAL-AND-003 -->
- Persistent bans/admins. <!-- G36-OPERATIONAL-AND-004 -->
- Server restart recovery. <!-- G36-OPERATIONAL-AND-005 -->
- Graceful active-match handling during deployment. <!-- G36-OPERATIONAL-AND-006 -->
- Horizontal scaling strategy. <!-- G36-OPERATIONAL-AND-007 -->
- Room ownership across multiple server processes. <!-- G36-OPERATIONAL-AND-008 -->
- Lobby service. <!-- G36-OPERATIONAL-AND-009 -->
- Database. <!-- G36-OPERATIONAL-AND-010 -->
- Authentication if accounts are added. <!-- G36-OPERATIONAL-AND-011 -->
- Observability beyond three counters. <!-- G36-OPERATIONAL-AND-012 -->
- Structured logs. <!-- G36-OPERATIONAL-AND-013 -->
- Error tracking. <!-- G36-OPERATIONAL-AND-014 -->
- Latency metrics. <!-- G36-OPERATIONAL-AND-015 -->
- Tick-duration metrics. <!-- G36-OPERATIONAL-AND-016 -->
- Connected-player metrics. <!-- G36-OPERATIONAL-AND-017 -->
- Per-room metrics. <!-- G36-OPERATIONAL-AND-018 -->
- Health versus readiness endpoints. <!-- G36-OPERATIONAL-AND-019 -->
- Backups. <!-- G36-OPERATIONAL-AND-020 -->
- Migration process. <!-- G36-OPERATIONAL-AND-021 -->
- Load testing. <!-- G36-OPERATIONAL-AND-022 -->
- Soak testing. <!-- G36-OPERATIONAL-AND-023 -->
- Browser compatibility testing. <!-- G36-OPERATIONAL-AND-024 -->
- Mobile-device testing. <!-- G36-OPERATIONAL-AND-025 -->
- Touch-control usability testing. <!-- G36-OPERATIONAL-AND-026 -->
- Accessibility testing. <!-- G36-OPERATIONAL-AND-027 -->
- Security review. <!-- G36-OPERATIONAL-AND-028 -->
- Dependency scanning. <!-- G36-OPERATIONAL-AND-029 -->
- Deployment rollback testing. <!-- G36-OPERATIONAL-AND-030 -->
- Asset CDN/caching. <!-- G36-OPERATIONAL-AND-031 -->
- Static-asset compression. <!-- G36-OPERATIONAL-AND-032 -->
- Privacy policy and moderation policy if publicly operated. <!-- G36-OPERATIONAL-AND-033 -->

## 37. Testing required for parity

- **Present:** Source-versus-Rust movement fixtures. <!-- G37-TESTING-REQUIRED-001 -->
- **Present:** Jump fixtures. <!-- G37-TESTING-REQUIRED-002 -->
- **Present:** Jet fixtures. <!-- G37-TESTING-REQUIRED-003 -->
- **Present:** Crouch/prone/roll/backflip fixtures. <!-- G37-TESTING-REQUIRED-004 -->
- **Present:** Polygon collision fixtures. <!-- G37-TESTING-REQUIRED-005 -->
- **Present:** One-way polygon fixtures. <!-- G37-TESTING-REQUIRED-006 -->
- Weapon fixture for every weapon and field. <!-- G37-TESTING-REQUIRED-007 -->
- Fire-rate fixtures. <!-- G37-TESTING-REQUIRED-008 -->
- Reload fixtures. <!-- G37-TESTING-REQUIRED-009 -->
- Startup fixtures. <!-- G37-TESTING-REQUIRED-010 -->
- Spread fixtures. <!-- G37-TESTING-REQUIRED-011 -->
- Recoil fixtures. <!-- G37-TESTING-REQUIRED-012 -->
- Bink fixtures. <!-- G37-TESTING-REQUIRED-013 -->
- Velocity-inheritance fixtures. <!-- G37-TESTING-REQUIRED-014 -->
- Head/chest/leg damage fixtures. <!-- G37-TESTING-REQUIRED-015 -->
- Explosion fixtures. <!-- G37-TESTING-REQUIRED-016 -->
- Grenade bounce/fuse fixtures. <!-- G37-TESTING-REQUIRED-017 -->
- Knife-throw fixtures. <!-- G37-TESTING-REQUIRED-018 -->
- Flame fixtures. <!-- G37-TESTING-REQUIRED-019 -->
- Flag interaction fixtures. <!-- G37-TESTING-REQUIRED-020 -->
- Bonus-kit fixtures. <!-- G37-TESTING-REQUIRED-021 -->
- Every mode’s scoring fixtures. <!-- G37-TESTING-REQUIRED-022 -->
- Round-limit fixtures. <!-- G37-TESTING-REQUIRED-023 -->
- Respawn fixtures. <!-- G37-TESTING-REQUIRED-024 -->
- Map-loading fixtures. <!-- G37-TESTING-REQUIRED-025 -->
- PMS parser fuzzing. <!-- G37-TESTING-REQUIRED-026 -->
- **Present:** Determinism across native Rust and Wasm. <!-- G37-TESTING-REQUIRED-027 -->
- Network reconciliation tests. <!-- G37-TESTING-REQUIRED-028 -->
- Two-player browser integration test. <!-- G37-TESTING-REQUIRED-029 -->
- Full 16-player test. <!-- G37-TESTING-REQUIRED-030 -->
- Reconnect test. <!-- G37-TESTING-REQUIRED-031 -->
- Server-restart test. <!-- G37-TESTING-REQUIRED-032 -->
- Mobile portrait test. <!-- G37-TESTING-REQUIRED-033 -->
- Mobile landscape test. <!-- G37-TESTING-REQUIRED-034 -->
- Low/high latency tests. <!-- G37-TESTING-REQUIRED-035 -->
- Packet-loss tests. <!-- G37-TESTING-REQUIRED-036 -->
- Long-running server soak test. <!-- G37-TESTING-REQUIRED-037 -->

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
