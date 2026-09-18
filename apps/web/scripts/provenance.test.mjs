import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync, readdirSync } from 'node:fs'
import { resolve } from 'node:path'
import { assetPath, defaultManifest, importedAssets } from '../src/render/manifest.ts'

const ROOT = resolve(import.meta.dirname, '../../..')
const provenance = readFileSync(resolve(ROOT, 'docs/provenance.md'), 'utf8')
const ART = resolve(ROOT, 'apps/web/public/art')

test('every file shipped as art is accounted for in the provenance record', () => {
  const files = readdirSync(ART)
  assert.ok(files.length > 0, 'there is art to account for')
  for (const file of files) {
    assert.ok(
      provenance.includes(`art/${file}`),
      `${file} has a provenance row saying where it came from`,
    )
  }
})

test('every asset the manifest names is a file that is actually there', () => {
  const manifest = defaultManifest()
  const files = new Set(readdirSync(ART))
  for (const id of Object.keys(manifest.entries)) {
    const path = assetPath(manifest, id)
    assert.ok(path.startsWith('art/'), `${id} lives under the art directory`)
    assert.ok(files.has(path.slice('art/'.length)), `${id} points at a file that exists: ${path}`)
  }
})

test('nothing shipped by default came from outside this project', () => {
  assert.deepEqual(
    importedAssets(defaultManifest()),
    [],
    'an imported asset needs its own provenance row before it can ship',
  )
})

test('the provenance record says what the match visuals are made of', () => {
  assert.match(
    provenance,
    /procedural/i,
    'the record states that the drawn match is procedural rather than imported',
  )
})
