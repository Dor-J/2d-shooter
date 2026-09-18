import test from 'node:test'
import assert from 'node:assert/strict'
import {
  AMMO_COLOR,
  ARMOR_COLOR,
  CRITICAL_HEALTH,
  HEALTH_CRITICAL,
  HEALTH_FULL,
  JET_COLOR,
  JET_EMPTY_COLOR,
  RELOAD_COLOR,
  TEAM_NAMES,
  ammoGauge,
  armorGauge,
  bulletCountText,
  damageVignette,
  fireIntervalGauge,
  gaugeFillRect,
  healthGauge,
  jetGauge,
  playerTint,
  teamColor,
  teamName,
} from '../src/hud/gauges.ts'

test('the health bar empties with health and never leaves its range', () => {
  assert.equal(healthGauge(100).fill, 1)
  assert.equal(healthGauge(50).fill, 0.5)
  assert.equal(healthGauge(0).fill, 0)
  assert.equal(healthGauge(-40).fill, 0, 'an overkill does not push it negative')
  assert.equal(healthGauge(150).fill, 1, 'and a medkit overfill does not push it past full')
})

test('the health bar turns red once a player is nearly dead', () => {
  assert.deepEqual(healthGauge(100).color, HEALTH_FULL)
  assert.deepEqual(healthGauge(20).color, HEALTH_CRITICAL)
  assert.deepEqual(healthGauge(CRITICAL_HEALTH * 100).color, HEALTH_CRITICAL)
})

test('the health bar says its number out loud for a screen reader', () => {
  assert.equal(healthGauge(73).label, '73 health')
})

test('an empty jet tank is greyed rather than hidden, so the player can see why they are not rising', () => {
  assert.deepEqual(jetGauge(100, 100).color, JET_COLOR)
  assert.deepEqual(jetGauge(0, 100).color, JET_EMPTY_COLOR)
  assert.equal(jetGauge(25, 100).fill, 0.25)
  assert.equal(jetGauge(1, 0).fill, 0, 'a mode with no jets does not divide by zero')
})

test('the ammo bar shows rounds left when the weapon is loaded', () => {
  const gauge = ammoGauge({ ammo: 15, magazine: 30, reloadTimer: 0, reloadTicks: 108 })
  assert.equal(gauge.fill, 0.5)
  assert.deepEqual(gauge.color, AMMO_COLOR)
  assert.equal(gauge.label, '15 of 30 rounds')
})

test('the ammo bar becomes a reload bar that fills as the reload finishes', () => {
  const started = ammoGauge({ ammo: 0, magazine: 30, reloadTimer: 108, reloadTicks: 108 })
  const nearly = ammoGauge({ ammo: 0, magazine: 30, reloadTimer: 27, reloadTicks: 108 })
  assert.equal(started.fill, 0)
  assert.equal(nearly.fill, 0.75)
  assert.deepEqual(nearly.color, RELOAD_COLOR)
  assert.equal(nearly.label, 'reloading 75%')
})

test('the bullet count says EMPTY rather than a bare zero', () => {
  assert.equal(bulletCountText({ ammo: 12, magazine: 30, reloadTimer: 0, reloadTicks: 0 }), '12 / 30')
  assert.equal(bulletCountText({ ammo: 0, magazine: 30, reloadTimer: 0, reloadTicks: 0 }), 'EMPTY')
  assert.equal(bulletCountText({ ammo: 0, magazine: 30, reloadTimer: 40, reloadTicks: 108 }), '— / —')
})

test('the fire-interval bar fills up as the weapon cycles, so a burst can be timed', () => {
  assert.equal(fireIntervalGauge(0, 10).fill, 1)
  assert.equal(fireIntervalGauge(10, 10).fill, 0)
  assert.equal(fireIntervalGauge(5, 10).fill, 0.5)
  assert.equal(fireIntervalGauge(0, 10).label, 'ready')
  assert.equal(fireIntervalGauge(3, 10).label, 'weapon cycling')
})

test('armor is shown only when there is some', () => {
  assert.equal(armorGauge(0), null)
  assert.equal(armorGauge(-1), null)
  const gauge = armorGauge(50)
  assert.equal(gauge.fill, 0.5)
  assert.deepEqual(gauge.color, ARMOR_COLOR)
  assert.equal(gauge.label, '50 armor')
})

test('a gauge fills its placed element from the left, inside a border', () => {
  const placed = { id: 'health', x: 100, y: 200, width: 240, height: 18, layer: 0 }
  const full = gaugeFillRect(healthGauge(100), placed)
  assert.equal(full.x, 102)
  assert.equal(full.y, 202)
  assert.equal(full.width, 236)
  assert.equal(full.height, 14)
  assert.equal(gaugeFillRect(healthGauge(50), placed).width, 118)
  assert.equal(gaugeFillRect(healthGauge(0), placed).width, 0)
})

test('all four teams have a colour and a name', () => {
  for (const team of [1, 2, 3, 4]) {
    assert.equal(teamColor(team).length, 4)
    assert.ok(TEAM_NAMES[team])
  }
  assert.equal(teamName(1), 'Alpha')
  assert.equal(teamName(4), 'Delta')
  assert.equal(teamName(0), 'No team')
  assert.notDeepEqual(teamColor(1), teamColor(2))
  assert.notDeepEqual(teamColor(3), teamColor(4))
})

test('an unknown team falls back rather than drawing nothing', () => {
  assert.deepEqual(teamColor(99), teamColor(0))
  assert.deepEqual(teamColor(undefined), teamColor(0))
})

test('your own soldier is brightened so you can find yourself in a crowd', () => {
  const mine = playerTint(1, true)
  const theirs = playerTint(1, false)
  assert.ok(mine[0] >= theirs[0] && mine[1] >= theirs[1])
  assert.notDeepEqual(mine, theirs)
  for (const channel of mine) assert.ok(channel <= 1, 'and never past white')
})

test('the screen bleeds when you are hit and keeps bleeding while you are nearly dead', () => {
  assert.ok(damageVignette(100, 0) > 0.5, 'a fresh hit shows')
  assert.equal(damageVignette(100, 2), 0, 'and fades')
  assert.ok(damageVignette(10, 5) > 0, 'but low health keeps telling you')
  assert.equal(damageVignette(0, 5), 0, 'a corpse is not warned about its health')
  assert.ok(damageVignette(100, 0) <= 1)
})
