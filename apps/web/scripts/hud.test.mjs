// Acceptance evidence for docs/parity/coverage.json:
//   web:hud:kill-feed     — kill feed lines for every death variant, with expiry and a cap
//   web:hud:respawn       — respawn countdown and the weapon menu shown while dead
//   web:hud:damage-arrow  — damage direction feedback from authoritative damage events
//   web:render:particles  — blood, gore, and gib particles derived from damage events
import test from 'node:test'
import assert from 'node:assert/strict'
import { KillFeed, describeKill, respawnCountdown, damageArrowAngle } from '../src/hud/feed.ts'
import { ParticleField } from '../src/render/particles.ts'

const names = new Map([
  [1, 'Ace'],
  [2, 'Bystander'],
  [3, 'Helper'],
])
const name = id => names.get(id) ?? `Player ${id}`

function entry(overrides = {}) {
  return {
    killer: 1,
    target: 2,
    cause: 'bullet',
    region: 'chest',
    headshot: false,
    teamkill: false,
    suicide: false,
    multi: 1,
    assists: [],
    ...overrides,
  }
}

test('kill feed lines name the killer, the victim, and what happened', () => {
  assert.equal(describeKill(entry(), name), 'Ace shot Bystander')
  assert.equal(
    describeKill(entry({ headshot: true, region: 'head' }), name),
    'Ace headshot Bystander',
  )
  assert.equal(describeKill(entry({ cause: 'explosion' }), name), 'Ace blew up Bystander')
  assert.equal(describeKill(entry({ teamkill: true }), name), 'Ace team-killed Bystander')
  assert.equal(
    describeKill(entry({ killer: null, suicide: true, cause: 'explosion' }), name),
    'Bystander blew themselves up',
  )
  assert.equal(
    describeKill(entry({ killer: null, suicide: false, cause: 'fall' }), name),
    'Bystander fell to their death',
  )
  assert.equal(
    describeKill(entry({ multi: 3 }), name),
    'Ace shot Bystander (triple kill)',
  )
  assert.equal(
    describeKill(entry({ assists: [3] }), name),
    'Ace shot Bystander (assist: Helper)',
  )
})

test('the kill feed drops old lines and never grows without bound', () => {
  const feed = new KillFeed({ lifetimeMs: 1000, limit: 3 })
  feed.push([entry({ target: 2 })], name, 0)
  feed.push([entry({ target: 3 })], name, 200)
  assert.equal(feed.visible(300).length, 2)
  assert.equal(feed.visible(1100).length, 1, 'the first line expired')
  assert.equal(feed.visible(1500).length, 0)

  for (let i = 0; i < 10; i += 1) feed.push([entry()], name, 3000)
  assert.equal(feed.visible(3000).length, 3, 'the feed is capped')
})

test('the respawn countdown reads in whole seconds and disappears when alive', () => {
  assert.equal(respawnCountdown(120), '2')
  assert.equal(respawnCountdown(61), '2')
  assert.equal(respawnCountdown(60), '1')
  assert.equal(respawnCountdown(1), '1')
  assert.equal(respawnCountdown(0), '')
})

test('the damage arrow points back along the direction the hit came from', () => {
  assert.equal(damageArrowAngle({ x: 1, y: 0 }), 180)
  assert.equal(damageArrowAngle({ x: -1, y: 0 }), 0)
  assert.equal(damageArrowAngle({ x: 0, y: 0 }), null)
})

test('blood is emitted from damage events and scales with the damage taken', () => {
  const field = new ParticleField({ limit: 200 })
  field.emitBlood({ x: 100, y: 100 }, { x: 1, y: 0 }, 5, 1)
  const light = field.particles.length
  field.clear()
  field.emitBlood({ x: 100, y: 100 }, { x: 1, y: 0 }, 60, 1)
  assert.ok(field.particles.length > light, 'a heavier hit sprays more blood')
  assert.ok(field.particles.every(particle => Number.isFinite(particle.x)))
})

test('the same event always produces the same particles', () => {
  const left = new ParticleField({ limit: 200 })
  const right = new ParticleField({ limit: 200 })
  left.emitBlood({ x: 10, y: 20 }, { x: 0, y: -1 }, 30, 42)
  right.emitBlood({ x: 10, y: 20 }, { x: 0, y: -1 }, 30, 42)
  assert.deepEqual(left.particles, right.particles)
})

test('gibs are thrown from the body and particles fade out and are collected', () => {
  const field = new ParticleField({ limit: 200 })
  field.emitGibs({ x: 50, y: 50 }, { x: 40, y: -20 }, 7)
  assert.ok(field.particles.some(particle => particle.kind === 'gib'))
  const before = field.particles.length
  for (let i = 0; i < 600; i += 1) field.step(1 / 60)
  assert.equal(field.particles.length, 0, 'particles do not accumulate forever')
  assert.ok(before > 0)
})

test('muzzle flashes and casings are short-lived presentation particles', () => {
  const field = new ParticleField({ limit: 20 })
  field.emitFlash({ x: 8, y: 9 })
  field.emitCasing({ x: 8, y: 9 })
  assert.ok(field.particles.some(particle => particle.kind === 'flash'))
  assert.ok(field.particles.some(particle => particle.kind === 'casing'))
  for (let i = 0; i < 60; i += 1) field.step(1 / 60)
  assert.equal(field.particles.length, 0)
})

test('the particle field never exceeds its limit', () => {
  const field = new ParticleField({ limit: 24 })
  for (let i = 0; i < 40; i += 1) field.emitBlood({ x: i, y: i }, { x: 1, y: 0 }, 80, i)
  assert.ok(field.particles.length <= 24)
})
