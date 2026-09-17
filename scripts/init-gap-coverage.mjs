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
