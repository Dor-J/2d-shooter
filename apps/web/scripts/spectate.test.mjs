// Acceptance evidence for docs/parity/coverage.json:
//   web:spectate:hud — the spectator bar names who is being watched
//   web:spectate:keys — previous/next/free-camera controls
import test from 'node:test'
import assert from 'node:assert/strict'
import { commandForKey, delayLabel, isSpectating, spectatorLabel } from '../src/spectate.ts'

const following = (target) => ({ view: 'Following', target, camera: { x: 0, y: 0 } })
const free = { view: 'FreeCamera', target: null, camera: { x: 0, y: 0 } }

test('a playing client is not spectating', () => {
  assert.equal(isSpectating(undefined), false)
  assert.equal(isSpectating(following(1)), true)
})

test('the bar names whoever is being watched', () => {
  assert.equal(spectatorLabel(following(7), { 7: 'Ada' }), 'Watching Ada')
  assert.equal(spectatorLabel(free, {}), 'Free camera')
  assert.equal(spectatorLabel(following(null), {}), 'Nobody left to watch')
  assert.equal(spectatorLabel(undefined, {}), '')
})

test('an unknown player still reads sensibly', () => {
  const label = spectatorLabel(following(99), {})
  assert.doesNotMatch(label, /undefined/)
  assert.match(label, /a player/)
})

test('the spectate controls map to previous, next, and free camera', () => {
  assert.deepEqual(commandForKey('ArrowRight'), { type: 'next' })
  assert.deepEqual(commandForKey('d'), { type: 'next' })
  assert.deepEqual(commandForKey('ArrowLeft'), { type: 'previous' })
  assert.deepEqual(commandForKey('A'), { type: 'previous' })
  assert.deepEqual(commandForKey('f'), { type: 'free_camera' })
  assert.equal(commandForKey('q'), null, 'other keys mean other things')
})

test('a delayed broadcast says so and a live one says nothing', () => {
  assert.equal(delayLabel(0), '')
  assert.equal(delayLabel(undefined), '')
  assert.equal(delayLabel(600), 'Broadcast delayed 10s')
})
