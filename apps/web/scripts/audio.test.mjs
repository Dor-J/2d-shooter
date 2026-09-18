// Acceptance evidence: web:audio:engine
import test from 'node:test'
import assert from 'node:assert/strict'
import { AudioEngine, attenuation, clampVolume, defaultAudioSettings, panOf } from '../src/audio/engine.ts'
import { clipFor, CLIPS } from '../src/audio/manifest.ts'
import { applyMusicCommand, defaultMusic, stepTrack, trackName, TRACKS } from '../src/audio/music.ts'

test('every named event maps to a clip, and an unknown one maps to nothing', () => {
  assert.equal(clipFor('Shot')?.id, 'fire')
  assert.equal(clipFor('Reload')?.id, 'reload')
  assert.equal(clipFor('Empty')?.id, 'empty')
  assert.equal(clipFor('Explosion')?.id, 'explosion')
  assert.equal(clipFor('KitTaken')?.id, 'kit')
  assert.equal(clipFor('Nonsense'), null)
})

test('volume stays inside 0–1', () => {
  assert.equal(clampVolume(1.4), 1)
  assert.equal(clampVolume(-2), 0)
  assert.equal(clampVolume(Number.NaN), 0)
})

test('a sound next to the listener is loud and one far away is silent', () => {
  assert.equal(attenuation({ x: 0, y: 0 }, { x: 0, y: 0 }), 1)
  assert.equal(attenuation({ x: 900, y: 0 }, { x: 0, y: 0 }), 0)
  assert.ok(attenuation({ x: 450, y: 0 }, { x: 0, y: 0 }) > 0.4)
})

test('a sound to the right pans right', () => {
  assert.ok(panOf({ x: 200, y: 0 }, { x: 0, y: 0 }) > 0)
  assert.ok(panOf({ x: -200, y: 0 }, { x: 0, y: 0 }) < 0)
})

test('the engine records what it would play rather than needing a speaker', () => {
  const audio = new AudioEngine()
  const heard = audio.hear('Shot', { at: { x: 10, y: 0 }, listener: { x: 0, y: 0 } })
  assert.equal(heard.clip, 'fire')
  assert.ok(heard.volume > 0)
  assert.equal(audio.played.length, 1)
})

test('a weapon with its own voice uses that voice', () => {
  const audio = new AudioEngine()
  assert.equal(audio.hear('Shot', { weapon: 'Chainsaw' }).clip, 'chainsaw')
  assert.equal(audio.hear('Shot', { weapon: 'Combat Knife' }).clip, 'knife')
  assert.equal(audio.hear('Shot', { weapon: 'Fist' }).clip, 'punch')
})

test('master volume of zero is silence', () => {
  const audio = new AudioEngine({ ...defaultAudioSettings(), master: 0 })
  assert.equal(audio.hear('Shot'), null)
})

test('a close explosion takes hearing away and leaves a whistle as it comes back', () => {
  const audio = new AudioEngine()
  audio.hear('Explosion', { at: { x: 0, y: 0 }, listener: { x: 0, y: 0 } })
  assert.ok(audio.deafness > 0)
  const during = audio.hear('Shot', { listener: { x: 0, y: 0 } })
  audio.step(2)
  assert.equal(audio.deafness, 0)
  const after = audio.hear('Shot', { listener: { x: 0, y: 0 } })
  assert.ok(after.volume > during.volume)
})

test('music previous and next wrap, and mute is not a volume of zero', () => {
  assert.equal(TRACKS.length, 4)
  assert.equal(trackName(0), 'Track 1')
  assert.equal(stepTrack(0, -1), 3)
  assert.equal(stepTrack(3, 1), 0)
  const muted = applyMusicCommand(defaultMusic(), { enabled: false })
  assert.equal(muted.playing, false)
  assert.equal(muted.volume, 0.5, 'the saved volume comes back when they unmute')
})

test('every clip the manifest names actually exists', () => {
  for (const clip of Object.values(CLIPS)) {
    assert.ok(clip.hz > 0)
    assert.ok(clip.seconds > 0)
  }
})
