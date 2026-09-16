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
