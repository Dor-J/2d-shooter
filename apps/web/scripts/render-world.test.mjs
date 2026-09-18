// Acceptance evidence: web:render:objects web:render:map
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  FLAG_COLORS,
  KIT_KINDS,
  bulletTrail,
  droppedWeaponSprite,
  flagSprite,
  kitSprite,
  projectileSprite,
  stationaryGunSprite,
} from '../src/render/objects.ts'
import {
  POLYGON_COLORS,
  edgesOf,
  parallaxPosition,
  polygonColor,
  sceneryFrame,
  sceneryLayers,
  sceneryVisible,
  textureCoords,
} from '../src/render/map.ts'

test('every bonus kit is drawn in a colour of its own', () => {
  const colors = KIT_KINDS.map(kind => kitSprite(kind, { x: 0, y: 0 }).color.join())
  assert.equal(new Set(colors).size, KIT_KINDS.length, 'no two kits look the same')
  assert.equal(KIT_KINDS.length, 7)
})

test('a kit bobs, so it reads as something to pick up rather than scenery', () => {
  const still = kitSprite('Medical', { x: 10, y: 20 }, 0)
  const later = kitSprite('Medical', { x: 10, y: 20 }, 30)
  assert.notEqual(still.y, later.y)
  assert.equal(still.x, later.x, 'and it bobs rather than wandering')
})

test('an unknown kit is still drawn rather than silently skipped', () => {
  const sprite = kitSprite('Mystery', { x: 0, y: 0 })
  assert.equal(sprite.sprite, 'kit-mystery')
  assert.equal(sprite.color.length, 4)
})

test('a carried flag is drawn behind its carrier so a firefight stays readable', () => {
  const carried = flagSprite('Alpha', { x: 0, y: 0 }, 'carried')
  const onBase = flagSprite('Alpha', { x: 0, y: 0 }, 'base')
  assert.ok(carried.layer < onBase.layer)
  assert.deepEqual(carried.color, FLAG_COLORS.Alpha)
})

test('a flag on its base waves and a dropped one lies at an angle', () => {
  assert.notEqual(flagSprite('Bravo', { x: 0, y: 0 }, 'base', 0).y, flagSprite('Bravo', { x: 0, y: 0 }, 'base', 14).y)
  assert.ok(flagSprite('Bravo', { x: 0, y: 0 }, 'dropped').angle > 0)
})

test('a dropped weapon keeps its own sprite id', () => {
  assert.equal(droppedWeaponSprite('Barrett M82A1', { x: 0, y: 0 }).sprite, 'weapon-barrett-m82a1')
})

test('a grenade spins and an arrow points where it is going', () => {
  const grenade = projectileSprite('grenade', { x: 0, y: 0 }, { x: 100, y: 0 }, 0)
  const spun = projectileSprite('grenade', { x: 0, y: 0 }, { x: 100, y: 0 }, 8)
  assert.notEqual(grenade.angle, spun.angle)

  const arrow = projectileSprite('arrow', { x: 0, y: 0 }, { x: 0, y: 100 })
  assert.ok(Math.abs(arrow.angle - Math.PI / 2) < 1e-9)
  const flat = projectileSprite('arrow', { x: 0, y: 0 }, { x: 100, y: 0 })
  assert.equal(flat.angle, 0)
})

test('every projectile kind has a size and a colour of its own', () => {
  const kinds = ['grenade', 'cluster', 'arrow', 'bullet', 'rocket']
  const seen = new Set()
  for (const kind of kinds) {
    const sprite = projectileSprite(kind, { x: 0, y: 0 }, { x: 1, y: 0 })
    assert.ok(sprite.width > 0 && sprite.height > 0, kind)
    seen.add(sprite.color.join())
  }
  assert.equal(seen.size, kinds.length)
})

test('a manned stationary gun looks different from an abandoned one', () => {
  const manned = stationaryGunSprite({ x: 0, y: 0 }, 0.3, true)
  const idle = stationaryGunSprite({ x: 0, y: 0 }, 0.3, false)
  assert.notDeepEqual(manned.color, idle.color)
  assert.equal(manned.angle, 0.3)
})

test('a bullet trail is derived from the round rather than remembered between frames', () => {
  const trail = bulletTrail({ x: 100, y: 50 }, { x: 600, y: 0 }, 2)
  assert.equal(trail.to.x, 100)
  assert.ok(trail.from.x < trail.to.x, 'the streak is behind the round')
  const still = bulletTrail({ x: 100, y: 50 }, { x: 0, y: 0 })
  assert.deepEqual(still.from, still.to, 'a round going nowhere leaves no streak')
})

test('every polygon kind has a colour, and the dangerous ones do not look like the safe ones', () => {
  for (const kind of Object.keys(POLYGON_COLORS)) {
    assert.equal(polygonColor(kind).length, 4)
  }
  assert.notDeepEqual(polygonColor('Deadly'), polygonColor('Normal'))
  assert.notDeepEqual(polygonColor('Ice'), polygonColor('Bouncy'))
  assert.deepEqual(polygonColor('Unheard of'), POLYGON_COLORS.Normal)
})

test('a texture is projected from the world, so two polygons that meet show no seam', () => {
  const left = { kind: 'Normal', vertices: [{ x: 0, y: 0 }, { x: 128, y: 0 }, { x: 0, y: 128 }] }
  const right = { kind: 'Normal', vertices: [{ x: 128, y: 0 }, { x: 256, y: 0 }, { x: 128, y: 128 }] }
  const leftEdge = textureCoords(left, 128)[1]
  const rightEdge = textureCoords(right, 128)[0]
  assert.deepEqual(leftEdge, rightEdge)
  assert.deepEqual(textureCoords(left, 128)[0], { u: 0, v: 0 })
})

test('only the lit edges of a polygon are drawn, so ground reads as ground', () => {
  const polygon = {
    kind: 'Normal',
    vertices: [
      { x: 0, y: 0 },
      { x: 100, y: 0 },
      { x: 100, y: 40 },
      { x: 0, y: 40 },
    ],
  }
  const edges = edgesOf(polygon, { x: 0, y: -1 })
  assert.ok(edges.length > 0 && edges.length < 4, 'some edges face the light and some do not')
  for (const edge of edges) assert.ok(edge.width > 0)
})

test('a degenerate edge is skipped rather than dividing by its own length', () => {
  const polygon = { kind: 'Normal', vertices: [{ x: 5, y: 5 }, { x: 5, y: 5 }, { x: 5, y: 5 }] }
  assert.deepEqual(edgesOf(polygon), [])
})

test('distant scenery moves less than the camera and foreground moves more', () => {
  const far = { sprite: 'hill', x: 100, y: 100, width: 50, height: 50, angle: 0, parallax: 0.3, foreground: false, alpha: 1 }
  const near = { ...far, parallax: 1.4, foreground: true }
  const camera = { x: 100, y: 0 }
  assert.equal(parallaxPosition(far, camera).x, 70)
  assert.equal(parallaxPosition(near, camera).x, -40)
})

test('scenery is split into what is drawn behind the soldiers and what is drawn in front', () => {
  const pieces = [
    { sprite: 'a', x: 0, y: 0, width: 1, height: 1, angle: 0, parallax: 0.8, foreground: false, alpha: 1 },
    { sprite: 'b', x: 0, y: 0, width: 1, height: 1, angle: 0, parallax: 0.2, foreground: false, alpha: 1 },
    { sprite: 'c', x: 0, y: 0, width: 1, height: 1, angle: 0, parallax: 1.2, foreground: true, alpha: 1 },
  ]
  const { background, foreground } = sceneryLayers(pieces)
  assert.deepEqual(
    background.map(piece => piece.sprite),
    ['b', 'a'],
    'and the furthest away is drawn first',
  )
  assert.deepEqual(
    foreground.map(piece => piece.sprite),
    ['c'],
  )
})

test('scenery off the side of the view is not drawn at all', () => {
  const piece = { sprite: 'tree', x: 40, y: 40, width: 20, height: 20, angle: 0, parallax: 1, foreground: false, alpha: 1 }
  const view = { width: 800, height: 600 }
  assert.equal(sceneryVisible(piece, { x: 0, y: 0 }, view), true)
  assert.equal(sceneryVisible(piece, { x: 4000, y: 0 }, view), false)
})

test('animated scenery advances through its sheet', () => {
  const piece = { sprite: 'fan', x: 0, y: 0, width: 8, height: 8, angle: 0, parallax: 1, foreground: false, alpha: 1, frames: 4, period: 2 }
  assert.equal(sceneryFrame(piece, 0), 0)
  assert.equal(sceneryFrame(piece, 2), 1)
  assert.equal(sceneryFrame(piece, 8), 0)
})
