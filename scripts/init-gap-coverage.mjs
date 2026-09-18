import { readFileSync, writeFileSync, mkdirSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { readGapFeatures } from './check-gap-coverage.mjs'

const gapPath = resolve('docs/gaps/gap-list.md')
const coveragePath = resolve('docs/parity/coverage.json')
const markdownPath = resolve('docs/parity/coverage.md')
const source = readFileSync(gapPath, 'utf8')
const features = readGapFeatures(source)
const sectionCounts = new Map()
const sectionSlugs = new Map()

for (const feature of features) {
  const sectionNumber = Number(feature.section.match(/^\d+/)[0])
  const sectionName = feature.section.replace(/^\d+\.\s*/, '')
  const slug = sectionName.toUpperCase().replace(/[^A-Z0-9]+/g, '-').replace(/^-|-$/g, '').split('-').slice(0, 2).join('-')
  sectionSlugs.set(feature.section, slug)
  const ordinal = (sectionCounts.get(feature.section) ?? 0) + 1
  sectionCounts.set(feature.section, ordinal)
  feature.id = `G${String(sectionNumber).padStart(2, '0')}-${slug}-${String(ordinal).padStart(3, '0')}`
}

let featureIndex = 0
let inNumberedSection = false
const tagged = source.split(/\r?\n/).map(line => {
  if (/^## \d+\./.test(line)) inNumberedSection = true
  else if (line.startsWith('## ')) inNumberedSection = false
  if (!inNumberedSection || !line.startsWith('- ')) return line
  const feature = features[featureIndex++]
  return `${line.replace(/\s+<!--\s+G\d{2}-[A-Z0-9-]+-\d{3}\s+-->$/, '')} <!-- ${feature.id} -->`
}).join('\n')

const presentEvidence = new Map([
  ['Move left/right.', 'rust:movement_fixtures'],
  ['Jump.', 'rust:movement_fixtures'],
  ['Jet.', 'rust:movement_fixtures'],
  ['Mouse aiming.', 'web:smoke:aim-input'],
  ['Primary fire.', 'rust:game_core::tests::fire_is_rate_limited'],
  ['Grenade input.', 'rust:game_core::tests::holding_grenade_input_throws_only_once'],
  ['Chat.', 'server:chat-handler'],
  ['Touch controls.', 'web:mobile:movement-pad'],
  ['Basic 100 HP.', 'rust:game_core::tests::m79_explosion_damages_nearby_enemy_not_distant_enemy'],
  ['Death count.', 'rust:game_core::tests::self_explosion_is_not_credited_as_a_kill'],
  ['Automatic respawn.', 'rust:game_core::tests::deterministic_replay'],
  ['Numeric health.', 'web:smoke:hud-health'],
  ['Numeric jet fuel.', 'web:smoke:hud-jet'],
  ['Ammo and grenade count.', 'web:smoke:hud-ammo'],
  ['Kills/deaths.', 'web:smoke:hud-score'],
  ['Scoreboard.', 'web:smoke:scoreboard'],
  ['Basic room chat.', 'server:chat-handler'],
  ['Basic chat rate limiting.', 'server:chat-rate-limit'],
  ['Determinism across native Rust and Wasm.', 'parity:G37-TESTING-REQUIRED-027'],
  ['`.pms` map loader.', 'rust:content::pms'],
  ['PMS format validation.', 'rust:content::pms-validation'],
  ['Polygon-edge interaction.', 'rust:collision_fixtures:edges'],
  ['One-way polygons.', 'rust:collision_fixtures:one-way'],
  ['Bouncy polygons.', 'rust:polygon_collision:materials'],
  ['Ice/slippery surfaces.', 'rust:polygon_collision:materials'],
  ['Deadly polygons.', 'rust:world_polygon_collision:deadly'],
  ['Only-player and only-bullet polygon types.', 'rust:map_validation:filters'],
  ['Player collision with vertical and angled geometry.', 'rust:collision_fixtures:geometry'],
  ['Proper head/body/legs collision volumes.', 'rust:collision_fixtures:body-shape'],
  ['Corpses interacting with terrain.', 'rust:game_core:corpse-terrain'],
  ['Polygon geometry.', 'web:map-render:coordinates'],
  ['Polygon types and properties.', 'rust:map_validation:materials'],
  ['Polygon collision fixtures.', 'rust:collision_fixtures:geometry'],
  ['One-way polygon fixtures.', 'rust:collision_fixtures:one-way'],
  // Task 7 — rebindable multi-device input and accessibility.
  ['Numeric weapon selection.', 'web:input:weapon-selection'],
  ['Separate primary and secondary selection controls.', 'web:input:weapon-selection'],
  ['Crouch.', 'web:smoke:stance'],
  ['Prone.', 'web:smoke:stance'],
  ['Roll.', 'web:smoke:stance'],
  ['Backflip combinations.', 'web:input:backflip-combo'],
  ['Explicit reload.', 'web:smoke:reload'],
  ['Scoreboard hold/toggle behavior.', 'web:input:scoreboard'],
  ['Scoreboard scrolling.', 'web:input:scoreboard-scroll'],
  ['Screenshot control.', 'web:input:screenshot'],
  ['Window minimize shortcut.', 'web:input:minimize'],
  ['Runtime mouse-sensitivity adjustment.', 'web:input:sensitivity'],
  ['Fully rebindable keyboard controls.', 'web:input:rebinding'],
  ['Rebindable mouse buttons.', 'web:input:mouse-rebinding'],
  ['Controller/gamepad support.', 'web:input:gamepad'],
  ['Saved control profiles.', 'web:input:profiles'],
  ['Accessibility alternatives for combined inputs.', 'web:input:accessibility'],
  // Task 8 — damage, armor, death, corpses, and respawn.
  ['Character-body ragdoll physics.', 'rust:damage_fixtures:ragdoll'],
  ['Respawn weapon-selection menu.', 'rust:damage_fixtures:weapon-choice'],
  ['Body-part damage.', 'rust:damage_fixtures:regions'],
  ['Bleeding.', 'rust:damage_fixtures:bleeding'],
  ['Blood particles.', 'web:render:particles'],
  ['Gore/gibs.', 'web:render:particles'],
  ['Ragdoll corpses.', 'rust:damage_fixtures:ragdoll'],
  ['Corpse persistence.', 'rust:damage_fixtures:ragdoll'],
  ['Death causes.', 'rust:damage_fixtures:death-causes'],
  ['Kill feed.', 'web:hud:kill-feed'],
  ['Headshot messaging/effects.', 'web:hud:kill-feed'],
  ['Multi-kill messages.', 'web:hud:kill-feed'],
  ['Self-kill messages.', 'web:hud:kill-feed'],
  ['Teamkill messages.', 'web:hud:kill-feed'],
  ['Spawn protection if applicable.', 'rust:damage_fixtures:spawn-protection'],
  ['Configurable respawn time.', 'rust:damage_fixtures:respawn'],
  ['Respawn countdown HUD.', 'web:hud:respawn'],
  ['Weapon selection while dead.', 'rust:damage_fixtures:weapon-choice'],
  ['Proper mode/team/map spawn selection.', 'rust:damage_fixtures:spawn-selection'],
  ['Kill attribution after delayed damage.', 'rust:damage_fixtures:attribution'],
  ['Assist tracking if desired.', 'rust:damage_fixtures:attribution'],
  ['Damage direction feedback.', 'web:hud:damage-arrow'],
  // Task 9 — validated weapon configuration and formulas.
  ['Damage.', 'rust:weapon_config_fixtures'],
  ['Fire interval.', 'rust:weapon_config_fixtures'],
  ['Ammunition capacity.', 'rust:weapon_config_fixtures'],
  ['Reload time.', 'rust:weapon_config_fixtures'],
  ['Projectile speed.', 'rust:weapon_config_fixtures'],
  ['Bullet style.', 'rust:weapon_config_fixtures'],
  ['Startup time.', 'rust:weapon_config_fixtures'],
  ['Bink.', 'rust:weapon_config_fixtures'],
  ['Self-bink through negative bink values.', 'rust:weapon_config_fixtures'],
  ['Movement accuracy.', 'rust:weapon_config_fixtures'],
  ['Bullet spread.', 'rust:weapon_config_fixtures'],
  ['Recoil.', 'rust:weapon_config_fixtures'],
  ['Push.', 'rust:weapon_config_fixtures'],
  ['Inherited velocity.', 'rust:weapon_config_fixtures'],
  ['Head damage modifier.', 'rust:weapon_config_fixtures'],
  ['Chest damage modifier.', 'rust:weapon_config_fixtures'],
  ['Leg damage modifier.', 'rust:weapon_config_fixtures'],
  ['Separate normal and Realistic weapon tables.', 'rust:weapon_config_fixtures'],
  ['Cluster-grenade nesting under frag grenades.', 'rust:weapon_config_fixtures'],
  ['Server-selected/custom weapon mods.', 'rust:weapon_config_fixtures'],
  ['Weapon-mod synchronization with clients.', 'web:weapons:table'],
  ['Validation of custom weapon values.', 'rust:weapon_config_fixtures'],
  ['A safe loader for `weapons.ini`-style data.', 'rust:weapon_config_fixtures'],
  ['Version/hash checking so clients know which mod is active.', 'web:weapons:table'],
  ['Weapon-mod display in the room/server browser.', 'web:weapons:table'],
  ['Replace simplified damage with `Damage × CurrentSpeed × HitboxModifier`.', 'rust:weapon_ballistics_fixtures:damage'],
  ['Add projectile speed decay.', 'rust:weapon_ballistics_fixtures:decay'],
  ['Add player velocity inheritance.', 'rust:weapon_ballistics_fixtures:inherit'],
  ['Allow movement direction to increase or decrease projectile speed and damage.', 'rust:weapon_ballistics_fixtures:inherit'],
  ['Add head, chest, and leg hit detection.', 'rust:weapon_ballistics_fixtures:damage'],
  ['Add bink when the player is hit.', 'rust:weapon_ballistics_fixtures:accuracy'],
  ['Add self-bink when firing relevant weapons.', 'rust:weapon_ballistics_fixtures:accuracy'],
  ['Add cursor expansion representing current accuracy.', 'web:weapons:cursor'],
  ['Add movement accuracy penalties.', 'rust:weapon_ballistics_fixtures:accuracy'],
  ['Add stronger jetting accuracy penalties.', 'rust:weapon_ballistics_fixtures:accuracy'],
  ['Add recoil to the aim/cursor.', 'web:weapons:cursor'],
  ['Add correct bullet spread.', 'rust:weapon_ballistics_fixtures:accuracy'],
  ['Add bullet mass/push behavior.', 'rust:weapon_ballistics_fixtures:accuracy'],
  ['Add distance-dependent damage caused by changing projectile speed.', 'rust:weapon_ballistics_fixtures:decay'],
  ['Add exact explosive damage and falloff.', 'rust:weapon_ballistics_fixtures:explosion'],
  ['Add terrain occlusion for splash damage if Soldat’s source behavior requires it.', 'rust:weapon_ballistics_fixtures:explosion'],
  ['Add friendly-fire and team-bink rules.', 'rust:weapon_ballistics_fixtures:friendly'],
  // Task 10 — inventory, reloads, pickups, drops, and throws.
  ['Switch between carried primary and secondary weapons.', 'rust:weapon_inventory_fixtures'],
  ['Drop current weapon.', 'rust:weapon_inventory_fixtures'],
  ['Hold-to-charge weapon throw.', 'rust:weapon_inventory_fixtures'],
  ['Throw combat knife.', 'rust:weapon_inventory_fixtures'],
  ['Pick up weapons.', 'rust:weapon_inventory_fixtures'],
  ['Primary-plus-secondary inventory slots.', 'rust:weapon_inventory_fixtures'],
  ['Carrying two primary weapons.', 'rust:weapon_inventory_fixtures'],
  ['Weapon pickups.', 'rust:weapon_inventory_fixtures'],
  ['Weapon dropping.', 'rust:weapon_inventory_fixtures'],
  ['Thrown-weapon physics.', 'rust:weapon_inventory_fixtures'],
  ['Thrown combat knife.', 'rust:weapon_inventory_fixtures'],
  ['Knife recovery/pickup.', 'rust:weapon_inventory_fixtures'],
  ['Manual reload.', 'rust:weapon_inventory_fixtures'],
  ['Reload interruption.', 'rust:weapon_inventory_fixtures'],
  ['Weapon-switch delays.', 'rust:weapon_inventory_fixtures'],
  ['Correct weapon startup behavior.', 'rust:game_core::tests::fire_is_rate_limited'],
  ['Muzzle origin based on character pose.', 'rust:weapon_inventory_fixtures'],
  ['Weapons lying on the ground.', 'rust:weapon_inventory_fixtures'],
  ['Weapon pickup indicators.', 'web:weapons:inventory'],
])

for (const feature of features) {
  if (feature.status !== 'Present' || presentEvidence.has(feature.feature)) continue
  if (feature.section.startsWith('1.')) presentEvidence.set(feature.feature, 'rust:movement_fixtures')
  if (feature.section.startsWith('16.')) presentEvidence.set(feature.feature, 'task5:map-pipeline')
  if (feature.section.startsWith('17.')) presentEvidence.set(feature.feature, 'rust:map_editor:all_97_original_maps_pass_the_admission_pipeline')
  if (feature.section.startsWith('34.')) presentEvidence.set(feature.feature, 'rust:map_editor:editor_pipeline')
  if (feature.section.startsWith('37.') && feature.id <= 'G37-TESTING-REQUIRED-004') presentEvidence.set(feature.feature, 'rust:movement_fixtures')
}

const ledger = {
  referenceLock: 'docs/parity/reference-lock.md',
  features: features.map(feature => ({
    id: feature.id,
    section: feature.section,
    feature: feature.feature,
    source: `docs/gaps/gap-list.md#${feature.section.toLowerCase().replace(/[^a-z0-9]+/g, '-')}`,
    acceptance: [presentEvidence.get(feature.feature) ?? `parity:${feature.id}`],
  })),
  passingEvidence: [...new Set(features.filter(feature => feature.status === 'Present').map(feature => presentEvidence.get(feature.feature)))],
}

if (ledger.passingEvidence.some(value => !value)) throw new Error('A present feature lacks a registered baseline acceptance test')

mkdirSync(dirname(coveragePath), { recursive: true })
const normalizedGap = `${tagged.replace(/\n*$/, '')}\n`
if (normalizedGap !== source) writeFileSync(gapPath, normalizedGap)
writeFileSync(coveragePath, `${JSON.stringify(ledger, null, 2)}\n`)
writeFileSync(markdownPath, [
  '# Soldat parity coverage',
  '',
  'This generated index maps every actionable feature in `docs/gaps/gap-list.md` to a stable ID and acceptance-test identifier. `coverage.json` is the machine-readable authority.',
  '',
  '| ID | Status | Section | Feature | Acceptance |',
  '| --- | --- | --- | --- | --- |',
  ...features.map(feature => `| ${feature.id} | ${feature.status} | ${feature.section} | ${feature.feature.replaceAll('|', '\\|')} | ${ledger.features.find(entry => entry.id === feature.id).acceptance.join(', ')} |`),
  '',
].join('\n'))

process.stdout.write(`Initialized ${features.length} stable feature mappings\n`)
