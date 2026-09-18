import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { buildScene } from '../src/render/scene.ts'
import { defaultLayout, minimalLayout, mobileLayout } from '../src/hud/layout.ts'
import { defaultAppearance } from '../src/render/gostek.ts'
import { HEALTH_CRITICAL, HEALTH_FULL } from '../src/hud/gauges.ts'
import { createSurface, differingPixels, digest, fillCentred, fillRect, pixelAt } from './raster.mjs'

const BASELINES = resolve(import.meta.dirname, '../../../tests/fixtures/visual/hud-baselines.json')

function scene({ layout = defaultLayout(), viewport = { width: 1280, height: 720 }, health = 100 } = {}) {
  return buildScene({
    layout,
    viewport,
    soldiers: [
      {
        id: 1,
        pos: { x: 640, y: 400 },
        aim: { x: 760, y: 380 },
        team: 1,
        hp: health,
        pose: 'stand',
        appearance: defaultAppearance(),
        jetting: false,
        moving: false,
      },
      {
        id: 2,
        pos: { x: 420, y: 400 },
        aim: { x: 300, y: 420 },
        team: 2,
        hp: 100,
        pose: 'crouch',
        appearance: defaultAppearance(),
        jetting: true,
        moving: true,
      },
    ],
    local: 1,
    health,
    fuel: 120,
    fuelCapacity: 200,
    ammo: { ammo: 18, magazine: 30, reloadTimer: 0, reloadTicks: 108 },
    tick: 12,
  })
}

function render(draws, width, height) {
  const surface = createSurface(width, height)
  for (const draw of draws) {
    if (draw.centred) fillCentred(surface, draw.x, draw.y, draw.width, draw.height, draw.color)
    else fillRect(surface, draw.x, draw.y, draw.width, draw.height, draw.color)
  }
  return surface
}

const baselines = JSON.parse(readFileSync(BASELINES, 'utf8'))

/// Set UPDATE_VISUAL_BASELINES=1 to re-record after a deliberate change, then read the diff.
const updating = process.env.UPDATE_VISUAL_BASELINES === '1'
const recorded = {}

function check(name, surface) {
  const actual = digest(surface)
  if (updating) {
    recorded[name] = actual
    return
  }
  assert.equal(
    actual,
    baselines[name],
    `${name} no longer draws the same frame; re-record with UPDATE_VISUAL_BASELINES=1 if that was intended`,
  )
}

test('the default frame draws the same picture it did when it was signed off', () => {
  check('default-1280x720', render(scene(), 1280, 720))
})

test('the phone frame draws the same picture it did when it was signed off', () => {
  const viewport = { width: 844, height: 390, safeArea: { top: 0, right: 47, bottom: 21, left: 47 } }
  check('mobile-844x390', render(scene({ layout: mobileLayout(), viewport }), 844, 390))
})

test('the minimal HUD draws the same picture it did when it was signed off', () => {
  check('minimal-1280x720', render(scene({ layout: minimalLayout() }), 1280, 720))
})

test('a wounded frame draws the same picture it did when it was signed off', () => {
  check('wounded-1280x720', render(scene({ health: 18 }), 1280, 720))
})

test('the frame really did change when the health did', () => {
  const healthy = render(scene(), 1280, 720)
  const wounded = render(scene({ health: 18 }), 1280, 720)
  assert.notEqual(digest(healthy), digest(wounded))
  assert.ok(differingPixels(healthy, wounded) > 100, 'and a whole bar of pixels moved, not one')
})

test('the health bar is drawn where the layout says, in the colour the gauge says', () => {
  const surface = render(scene(), 1280, 720)
  const draw = scene().find(entry => entry.id === 'hud-health-fill')
  const sample = pixelAt(surface, draw.x + 4, draw.y + draw.height / 2)
  assert.deepEqual(sample.slice(0, 3), HEALTH_FULL.slice(0, 3).map(channel => Math.round(channel * 255)))

  const hurt = render(scene({ health: 18 }), 1280, 720)
  const hurtSample = pixelAt(hurt, draw.x + 4, draw.y + draw.height / 2)
  assert.deepEqual(
    hurtSample.slice(0, 3),
    HEALTH_CRITICAL.slice(0, 3).map(channel => Math.round(channel * 255)),
  )
})

test('an element the layout switched off leaves no pixels behind', () => {
  const full = scene()
  const minimal = scene({ layout: minimalLayout() })
  assert.ok(full.some(draw => draw.id === 'hud-minimap-track'))
  assert.ok(!minimal.some(draw => draw.id === 'hud-minimap-track'))
})

test('the same scene renders identically twice, so a baseline means something', () => {
  assert.equal(digest(render(scene(), 1280, 720)), digest(render(scene(), 1280, 720)))
})

test('a soldier is drawn on top of the background and the HUD on top of the soldier', () => {
  const draws = scene()
  const soldier = draws.findIndex(draw => draw.id.startsWith('soldier-'))
  const hud = draws.findIndex(draw => draw.id.startsWith('hud-'))
  assert.ok(soldier >= 0 && hud >= 0)
  assert.ok(soldier < hud, 'the HUD is drawn last, so nothing covers it')
})

test.after(() => {
  if (!updating) return
  writeFileSync(BASELINES, `${JSON.stringify({ ...baselines, ...recorded }, null, 2)}\n`)
  process.stdout.write(`re-recorded ${Object.keys(recorded).length} visual baselines\n`)
})
