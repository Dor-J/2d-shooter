import test from 'node:test'
import assert from 'node:assert/strict'
import { ParticleField } from '../src/render/particles.ts'

const origin = { x: 100, y: 100 }

function kinds(field) {
  return new Set(field.particles.map(particle => particle.kind))
}

test('a hit sprays blood, and a heavier hit sprays more of it', () => {
  const light = new ParticleField()
  light.emitBlood(origin, { x: 1, y: 0 }, 10, 1)
  const heavy = new ParticleField()
  heavy.emitBlood(origin, { x: 1, y: 0 }, 60, 1)
  assert.ok(heavy.particles.length > light.particles.length)
})

test('a round striking terrain throws sparks away from the surface', () => {
  const field = new ParticleField()
  field.emitSparks(origin, { x: 0, y: -1 }, 3)
  assert.ok(field.particles.length > 0)
  assert.deepEqual(kinds(field), new Set(['spark']))
  const upward = field.particles.filter(particle => particle.vy < 0)
  assert.ok(upward.length > field.particles.length / 2, 'most go the way the surface faces')
})

test('smoke rises and fire climbs, where blood falls', () => {
  const field = new ParticleField()
  field.emitSmoke(origin, 5)
  field.emitFire(origin, 6)
  const before = field.particles.map(particle => particle.y)
  for (let i = 0; i < 6; i += 1) field.step(1 / 60)
  const after = field.particles.map(particle => particle.y)
  assert.ok(after.length > 0)
  for (let i = 0; i < after.length; i += 1) assert.ok(after[i] < before[i], 'it went up')

  const falling = new ParticleField()
  falling.emitBlood(origin, { x: 1, y: 0 }, 40, 2)
  const startY = falling.particles[0].y
  for (let i = 0; i < 30; i += 1) falling.step(1 / 60)
  assert.ok(falling.particles.length === 0 || falling.particles[0].y > startY)
})

test('wind pushes smoke sideways', () => {
  const still = new ParticleField()
  still.emitSmoke(origin, 9, 0)
  const blown = new ParticleField()
  blown.emitSmoke(origin, 9, 300)
  const drift = particles => particles.reduce((sum, particle) => sum + particle.vx, 0)
  assert.ok(drift(blown.particles) > drift(still.particles))
})

test('an explosion is a core, a ring of fire, and the smoke it leaves', () => {
  const field = new ParticleField()
  field.emitExplosion(origin, 40, 11)
  const kind = kinds(field)
  assert.ok(kind.has('explosion'))
  assert.ok(kind.has('fire'))
  assert.ok(kind.has('smoke'))
  const core = field.particles.find(particle => particle.kind === 'explosion')
  assert.equal(core.size, 40, 'the core is as wide as the blast')
})

test('a bigger blast throws its fire further', () => {
  const small = new ParticleField()
  small.emitExplosion(origin, 20, 4)
  const large = new ParticleField()
  large.emitExplosion(origin, 80, 4)
  const speed = field =>
    Math.max(
      ...field.particles
        .filter(particle => particle.kind === 'fire')
        .map(particle => Math.hypot(particle.vx, particle.vy)),
    )
  assert.ok(speed(large) > speed(small))
})

test('a streak is gone within a couple of frames', () => {
  const field = new ParticleField()
  field.emitTrail(origin, { x: 600, y: 0 })
  assert.equal(field.particles.length, 1)
  for (let i = 0; i < 6; i += 1) field.step(1 / 60)
  assert.equal(field.particles.length, 0)
})

test('the low-particle setting thins every spray but never silences one', () => {
  const full = new ParticleField({ budget: 1 })
  const low = new ParticleField({ budget: 0.25 })
  for (const field of [full, low]) {
    field.emitBlood(origin, { x: 1, y: 0 }, 60, 1)
    field.emitSparks(origin, { x: 0, y: -1 }, 1)
    field.emitSmoke(origin, 1)
  }
  assert.ok(low.particles.length < full.particles.length)
  assert.ok(low.particles.length > 0, 'a hit the player cannot see is a gameplay change')
})

test('particles switched off entirely emit nothing at all', () => {
  const field = new ParticleField({ budget: 0 })
  field.emitBlood(origin, { x: 1, y: 0 }, 60, 1)
  field.emitSparks(origin, { x: 0, y: -1 }, 1)
  field.emitTrail(origin, { x: 1, y: 0 })
  field.emitExplosion(origin, 40, 1)
  assert.equal(field.particles.filter(particle => particle.kind !== 'explosion').length, 0)
})

test('the field never grows past its limit, however busy the fight', () => {
  const field = new ParticleField({ limit: 50 })
  for (let i = 0; i < 40; i += 1) {
    field.emitExplosion(origin, 40, i)
    field.emitBlood(origin, { x: 1, y: 0 }, 60, i)
  }
  assert.ok(field.particles.length <= 50)
})

test('the same seed makes the same spray, so a replay looks the same', () => {
  const run = () => {
    const field = new ParticleField()
    field.emitBlood(origin, { x: 1, y: 0 }, 40, 77)
    field.emitSparks(origin, { x: 0, y: -1 }, 77)
    field.emitExplosion(origin, 30, 77)
    for (let i = 0; i < 10; i += 1) field.step(1 / 60)
    return field.particles.map(particle => [particle.kind, particle.x, particle.y])
  }
  assert.deepEqual(run(), run())
})

test('a step of no time changes nothing and clearing empties the field', () => {
  const field = new ParticleField()
  field.emitFire(origin, 1)
  const before = structuredClone(field.particles)
  field.step(0)
  field.step(Number.NaN)
  assert.deepEqual(field.particles, before)
  field.clear()
  assert.deepEqual(field.particles, [])
})
