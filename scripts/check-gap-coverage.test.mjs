import assert from 'node:assert/strict'
import { execFileSync, spawnSync } from 'node:child_process'
import { mkdtempSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import test from 'node:test'

const checker = resolve('scripts/check-gap-coverage.mjs')

function fixture(gaps, coverage) {
  const directory = mkdtempSync(join(tmpdir(), 'arena-gap-coverage-'))
  const gapPath = join(directory, 'gaps.md')
  const coveragePath = join(directory, 'coverage.json')
  writeFileSync(gapPath, gaps)
  writeFileSync(coveragePath, JSON.stringify(coverage))
  return { gapPath, coveragePath }
}

test('accepts complete mappings and passing evidence for present features', () => {
  const { gapPath, coveragePath } = fixture(
    '# Inventory\n\n## 1. Movement\n\n- **Missing:** Crouching.\n- **Present:** Jumping.\n',
    {
      features: [
        { id: 'G01-MOVEMENT-001', section: '1. Movement', feature: 'Crouching.', acceptance: ['future/crouching.test'] },
        { id: 'G01-MOVEMENT-002', section: '1. Movement', feature: 'Jumping.', acceptance: ['evidence/jumping.test'] },
      ],
      passingEvidence: ['evidence/jumping.test'],
    },
  )

  const output = execFileSync(process.execPath, [checker, gapPath, coveragePath], { encoding: 'utf8' })

  assert.match(output, /2 features mapped; 1 present feature has passing evidence/)
})

test('rejects an actionable gap that is absent from the ledger', () => {
  const { gapPath, coveragePath } = fixture(
    '# Inventory\n\n## 1. Movement\n\n- **Missing:** Crouching.\n- Air control.\n',
    {
      features: [
        { id: 'G01-MOVEMENT-001', section: '1. Movement', feature: 'Crouching.', acceptance: ['future/crouching.test'] },
      ],
      passingEvidence: [],
    },
  )

  const result = spawnSync(process.execPath, [checker, gapPath, coveragePath], { encoding: 'utf8' })

  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /Unmapped feature.*Air control\./)
})

test('rejects a present feature whose only passing evidence is a placeholder generator', () => {
  const placeholder = fixture(
    '# Inventory\n\n## 1. Movement\n\n- **Present:** Running.\n',
    {
      features: [
        { id: 'G01-MOVEMENT-001', section: '1. Movement', feature: 'Running.', acceptance: ['rust:movement_fixtures'] },
      ],
      passingEvidence: ['rust:movement_fixtures'],
    },
  )
  const failed = spawnSync(process.execPath, [checker, placeholder.gapPath, placeholder.coveragePath], { encoding: 'utf8' })

  assert.notEqual(failed.status, 0)
  assert.match(failed.stderr, /Present feature has only placeholder evidence: 1\. Movement \/ Running\./)

  const maps = fixture(
    '# Inventory\n\n## 17. Maps\n\n- **Present:** ctf_Ash. <!-- G17-ALL-97-001 -->\n',
    {
      features: [
        {
          id: 'G17-ALL-97-001',
          section: '17. Maps',
          feature: 'ctf_Ash.',
          acceptance: ['rust:map_editor:all_97_original_maps_pass_the_admission_pipeline'],
        },
      ],
      passingEvidence: ['rust:map_editor:all_97_original_maps_pass_the_admission_pipeline'],
    },
  )
  const mapsFailed = spawnSync(process.execPath, [checker, maps.gapPath, maps.coveragePath], { encoding: 'utf8' })
  assert.notEqual(mapsFailed.status, 0)
  assert.match(mapsFailed.stderr, /Present feature has only placeholder evidence: 17\. Maps \/ ctf_Ash\./)

  const unpassedRealId = fixture(
    '# Inventory\n\n## 20. Rendering\n\n- **Present:** Character sprites.\n',
    {
      features: [
        {
          id: 'G20-RENDERING-AND-001',
          section: '20. Rendering',
          feature: 'Character sprites.',
          acceptance: ['parity:G20-RENDERING-AND-001', 'web:render:gostek'],
        },
      ],
      passingEvidence: ['web:render:gostek'],
    },
  )
  const unpassed = spawnSync(process.execPath, [checker, unpassedRealId.gapPath, unpassedRealId.coveragePath], { encoding: 'utf8' })
  assert.notEqual(unpassed.status, 0)
  assert.match(unpassed.stderr, /Present feature has only placeholder evidence: 20\. Rendering \/ Character sprites\./)

  const real = fixture(
    '# Inventory\n\n## 4. Weapons\n\n- **Present:** Damage.\n- **Present:** Editor pipeline.\n- **Present:** Map pipeline.\n- **Present:** Reload animations.\n',
    {
      features: [
        { id: 'G04-WEAPON-MOD-001', section: '4. Weapons', feature: 'Damage.', acceptance: ['rust:weapon_config_fixtures'] },
        { id: 'G34-MAP-EDITOR-001', section: '4. Weapons', feature: 'Editor pipeline.', acceptance: ['rust:map_editor:editor_pipeline'] },
        { id: 'G16-MAPS-AND-001', section: '4. Weapons', feature: 'Map pipeline.', acceptance: ['task5:map-pipeline'] },
        {
          id: 'G03-WEAPONS-031',
          section: '4. Weapons',
          feature: 'Reload animations.',
          acceptance: ['web:render:gostek', 'rust:weapon_inventory_fixtures'],
        },
      ],
      passingEvidence: ['rust:weapon_config_fixtures', 'rust:map_editor:editor_pipeline', 'task5:map-pipeline', 'web:render:gostek', 'rust:weapon_inventory_fixtures'],
    },
  )
  const passed = execFileSync(process.execPath, [checker, real.gapPath, real.coveragePath], { encoding: 'utf8' })

  assert.match(passed, /4 features mapped; 4 present features have passing evidence/)
})

test('rejects a present feature without passing acceptance evidence', () => {
  const { gapPath, coveragePath } = fixture(
    '# Inventory\n\n## 1. Movement\n\n- **Present:** Jumping.\n',
    {
      features: [
        { id: 'G01-MOVEMENT-001', section: '1. Movement', feature: 'Jumping.', acceptance: ['evidence/jumping.test'] },
      ],
      passingEvidence: [],
    },
  )

  const result = spawnSync(process.execPath, [checker, gapPath, coveragePath], { encoding: 'utf8' })

  assert.notEqual(result.status, 0)
  assert.match(result.stderr, /Present feature lacks passing evidence.*Jumping\./)
})
