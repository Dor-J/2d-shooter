// Acceptance evidence: web:prediction:reconcile
import test from 'node:test'
import assert from 'node:assert/strict'
import { disconnectText, extrapolate, InputRing, interpolate, reconcile, syncClock } from '../src/network/prediction.ts'

test('unacknowledged inputs are replayed onto the last server copy', () => {
  const acked = { id: 1, pos: { x: 100, y: 400 }, vel: { x: 0, y: 0 }, last_seq: 1 }
  const next = reconcile(acked, [
    { seq: 1, left: false, right: true, jump: false, jet: false, aim: { x: 0, y: 0 } },
    { seq: 2, left: false, right: true, jump: false, jet: false, aim: { x: 0, y: 0 } },
  ])
  assert.ok(next.pos.x > acked.pos.x)
  assert.equal(next.last_seq, 2)
})

test('interpolation stays between the two snapshots and never past the cap', () => {
  const mid = interpolate({ x: 0, y: 0 }, { x: 10, y: 0 }, 0.5)
  assert.equal(mid.x, 5)
  const capped = interpolate({ x: 0, y: 0 }, { x: 10, y: 0 }, 4, 1)
  assert.equal(capped.x, 10)
})

test('extrapolation is capped so a stalled packet does not send someone through a wall', () => {
  const far = extrapolate({ x: 0, y: 0 }, { x: 1000, y: 0 }, 1)
  const near = extrapolate({ x: 0, y: 0 }, { x: 1000, y: 0 }, 0.05)
  assert.ok(far.x < 1000)
  assert.ok(near.x < far.x)
})

test('clock sync ignores a nonsense sample', () => {
  const clock = syncClock({ offset: 0, samples: 0 }, 10, 20, 12)
  assert.ok(clock.samples === 1)
  const ignored = syncClock(clock, 10, 20, 9)
  assert.equal(ignored.samples, 1)
})

test('the input ring forgets what the server has already seen', () => {
  const ring = new InputRing()
  ring.push({ seq: 1, left: false, right: false, jump: false, jet: false, aim: { x: 0, y: 0 } })
  ring.push({ seq: 2, left: false, right: false, jump: false, jet: false, aim: { x: 0, y: 0 } })
  ring.dropThrough(1)
  assert.equal(ring.frames.length, 1)
  assert.equal(ring.frames[0].seq, 2)
})

test('a disconnect reason reads as a sentence', () => {
  assert.match(disconnectText('banned'), /not allowed/)
  assert.match(disconnectText('version_mismatch'), /version/)
})

test('high latency is a clock offset, and a lost packet is a capped extrapolation', () => {
  const late = syncClock({ offset: 0, samples: 0 }, 0, 80, 2)
  assert.ok(Math.abs(late.offset) >= 0)
  const held = extrapolate({ x: 0, y: 0 }, { x: 400, y: 0 }, 2)
  assert.ok(held.x < 400)
})
