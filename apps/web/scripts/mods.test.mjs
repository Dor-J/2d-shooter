// Acceptance evidence: web:mods:package web:mods:hash web:mods:ini web:mods:required
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  HISTORICAL_INTERFACES,
  INTERFACE_PRESETS,
  applyPackage,
  downloadPackage,
  historicalInterface,
  isSafePath,
  packageHash,
  parseModIni,
  parsePackage,
  requireHash,
  requiredModMatch,
} from '../src/mods/package.ts'
import { defaultManifest } from '../src/render/manifest.ts'

const neon = {
  format_version: 1,
  meta: { name: 'neon', license: 'MIT', provenance: 'generated-for-this-project', preview: 'preview.png' },
  files: { 'soldier.png': 'px' },
  scales: { soldier: 1.25 },
  cursor: 'cross.png',
  hud: { name: 'neon', provides: ['hud-health', 'hud-crosshair'], positions: { 'hud-health': { x: 8, y: 8 } } },
}

test('a safe package hashes and a wrong hash is refused', async () => {
  const parsed = parsePackage(neon)
  assert.ok('package' in parsed)
  const hash = await packageHash(parsed.package)
  assert.equal(await requireHash(parsed.package, hash), true)
  assert.equal(await requireHash(parsed.package, 'deadbeef'), 'hash')
})

test('a path that escapes or a missing license is refused', () => {
  assert.equal(isSafePath('../secret'), false)
  assert.ok('error' in parsePackage({ ...neon, files: { '../x': '1' } }))
  assert.ok('error' in parsePackage({ ...neon, meta: { ...neon.meta, license: '' } }))
})

test('mod.ini scales apply and a cheat size does not', () => {
  const scales = parseModIni('# c\nsoldier=1.5\n')
  assert.ok('scales' in scales)
  assert.equal(scales.scales.soldier, 1.5)
  assert.ok('error' in parseModIni('soldier=99'))
})

test('a package can replace stock art and the three HUD presets stay named', () => {
  const parsed = parsePackage(neon)
  assert.ok('package' in parsed)
  const applied = applyPackage(parsed.package, defaultManifest())
  assert.ok('manifest' in applied)
  assert.ok(INTERFACE_PRESETS.includes('default'))
})

test('historical interface names are catalogued and not treated as shipped assets', () => {
  assert.equal(HISTORICAL_INTERFACES.length, 10)
  assert.equal(historicalInterface('Classic'), true)
  assert.equal(historicalInterface('neon'), false)
})

test('a required-mod hash must match before join, and download yields the same hash', async () => {
  const loaded = await downloadPackage(JSON.stringify(neon))
  assert.ok('hash' in loaded)
  assert.equal(requiredModMatch(loaded.hash, loaded.hash), true)
  assert.equal(requiredModMatch(loaded.hash, 'nope'), false)
  assert.equal(requiredModMatch(undefined, undefined), true)
})
