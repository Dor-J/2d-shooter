// Acceptance evidence: web:replay:player web:replay:camera
import test from 'node:test'
import assert from 'node:assert/strict'
import { DemoPlayer, DemoRecorder } from '../src/replay/player.ts'

const file = {
  header: {
    format: 1,
    protocol: 15,
    map: 'Aero',
    mode: 'deathmatch',
    weapon_hash: 1,
    seed: 7,
    source_revision: 'test',
  },
  chunks: [
    { tick: 1, inputs: {}, checksum: 'aa' },
    { tick: 2, inputs: {}, checksum: 'bb' },
    { tick: 3, inputs: {}, checksum: 'cc' },
    { tick: 4, inputs: {}, checksum: 'dd' },
  ],
}

test('pause holds the tick and seek/fast-forward move it', () => {
  const player = DemoPlayer.open(file, 15, 1)
  assert.ok(player instanceof DemoPlayer)
  player.pause()
  assert.equal(player.step(2), 1)
  player.resume()
  assert.equal(player.fastForward(2), 3)
  assert.equal(player.seek(2), 2)
})

test('follow and free camera are distinct modes, and export round-trips', () => {
  const player = DemoPlayer.open(file, 15, 1)
  assert.ok(player instanceof DemoPlayer)
  player.follow(3)
  assert.deepEqual(player.camera, { mode: 'follow', player: 3 })
  player.freeCamera(10, 20)
  assert.equal(player.camera.mode, 'free')
  const copy = DemoPlayer.open(player.export(), 15, 1)
  assert.ok(copy instanceof DemoPlayer)
  assert.equal(copy.file.chunks.length, 4)
})

test('a wrong protocol is refused and a blank tail can be repaired', () => {
  assert.deepEqual(DemoPlayer.open(file, 14, 1), { error: 'protocol' })
  const broken = { ...file, chunks: [...file.chunks, { tick: 5, inputs: {}, checksum: '' }] }
  const repaired = DemoPlayer.repair(broken)
  assert.ok('chunks' in repaired)
  assert.equal(repaired.chunks.length, 4)
})

test('a recorder writes the ticks it saw', () => {
  const recorder = new DemoRecorder()
  recorder.record(1, 'aa')
  recorder.record(2, 'bb')
  const saved = recorder.toFile(file.header)
  assert.equal(saved.chunks.length, 2)
  const player = DemoPlayer.open(saved, 15, 1)
  assert.ok(player instanceof DemoPlayer)
  assert.equal(player.fastForward(4), 2)
})
