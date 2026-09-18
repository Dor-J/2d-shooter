// Acceptance evidence: web:profiles:player
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  constrainName,
  createPlayer,
  exportPlayer,
  importPlayer,
  migratePlayer,
  PlayerStore,
  toggleFavorite,
} from '../src/profiles/player.ts'

test('a name is trimmed and refused when it is empty, too long, or full of controls', () => {
  assert.deepEqual(constrainName('  Ace  '), { name: 'Ace' })
  assert.deepEqual(constrainName(''), { error: 'empty' })
  assert.deepEqual(constrainName('x'.repeat(21)), { error: 'too-long' })
  assert.deepEqual(constrainName('bad\u0001name'), { error: 'controls' })
})

test('a new profile has every appearance field and a default secondary', () => {
  const profile = createPlayer('Dor')
  assert.equal(profile.name, 'Dor')
  assert.equal(profile.secondary, 10)
  assert.ok(profile.appearance.shirt)
  assert.equal(profile.favorites.length, 0)
  assert.equal(profile.taunts.a, 'Nice shot')
})

test('an old save without the new fields still loads', () => {
  const migrated = migratePlayer({ name: 'Old', settings: { sensitivity: 2 } })
  assert.equal(migrated.name, 'Old')
  assert.equal(migrated.sensitivity, 2)
  assert.equal(migrated.audio.master, 0.8)
})

test('importing gives a fresh id so it never overwrites the one already here', () => {
  const original = createPlayer('Ace')
  const imported = importPlayer(exportPlayer(original))
  assert.ok(imported)
  assert.equal(imported.name, 'Ace')
  assert.notEqual(imported.id, original.id)
  assert.equal(importPlayer('not json'), null)
})

test('favorites toggle on and off', () => {
  assert.deepEqual(toggleFavorite([], 'ARENA1'), ['ARENA1'])
  assert.deepEqual(toggleFavorite(['ARENA1'], 'ARENA1'), [])
})

test('the store keeps more than one profile and remembers which is active', () => {
  const memory = new Map()
  const storage = {
    getItem: key => memory.get(key) ?? null,
    setItem: (key, value) => memory.set(key, value),
  }
  const store = new PlayerStore(storage)
  store.create('Two')
  assert.equal(store.list().length, 2)
  const second = store.list()[1]
  store.select(second.id)
  assert.equal(store.active().name, 'Two')
  store.update(second.id, profile => ({ ...profile, name: 'Renamed' }))
  assert.equal(store.active().name, 'Renamed')
})
