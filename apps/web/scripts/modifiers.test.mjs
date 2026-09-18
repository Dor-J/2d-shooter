// Acceptance evidence for docs/parity/coverage.json:
//   web:modifiers:badges   — the room browser shows which modifiers a room runs
//   web:modifiers:survival — a dead survival player is told they are out, not counted down
//   web:modifiers:advance  — the weapon menu shows what has been unlocked
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  advanceProgress,
  anyModifier,
  COMMUNITY_MODES,
  deathMessage,
  hudIsMinimal,
  isUnlocked,
  modifierBadges,
  modifierSummary,
  NO_MODIFIERS,
} from '../src/modifiers.ts'

test('a plain room shows no modifier badges', () => {
  assert.equal(anyModifier(NO_MODIFIERS), false)
  assert.equal(anyModifier(undefined), false)
  assert.deepEqual(modifierBadges({ modifiers: NO_MODIFIERS }), [])
  assert.equal(modifierSummary({ modifiers: NO_MODIFIERS }), 'Standard rules')
})

test('each modifier gets its own badge, in a stable order', () => {
  const all = { realistic: true, survival: true, advance: true }
  assert.ok(anyModifier(all))
  assert.deepEqual(modifierBadges({ modifiers: all }), ['Realistic', 'Survival', 'Advance'])
  assert.equal(modifierSummary({ modifiers: all }), 'Realistic · Survival · Advance')
})

test('a community room is named by its ruleset first', () => {
  const badges = modifierBadges({
    ruleset: 'RS/CS',
    modifiers: { realistic: true, survival: true, advance: false },
  })
  assert.equal(badges[0], 'RS/CS')
  assert.deepEqual(badges, ['RS/CS', 'Realistic', 'Survival'])
})

test('all eleven community modes are offered', () => {
  assert.equal(COMMUNITY_MODES.length, 11)
  assert.ok(COMMUNITY_MODES.includes('Zombie'))
  assert.ok(COMMUNITY_MODES.includes('Climb'))
  assert.equal(new Set(COMMUNITY_MODES).size, 11, 'no duplicates')
})

test('realistic drops the HUD back to its minimal form', () => {
  assert.ok(hudIsMinimal({ realistic: true, survival: false, advance: false }))
  assert.ok(!hudIsMinimal(NO_MODIFIERS))
  assert.ok(!hudIsMinimal(undefined))
})

test('a dead survival player is told they are out rather than counted down', () => {
  const survival = { realistic: false, survival: true, advance: false }
  assert.equal(deathMessage(survival, '5'), 'You are out for this round')
  assert.equal(deathMessage(NO_MODIFIERS, '5'), 'Respawn in 5')
  assert.equal(deathMessage(NO_MODIFIERS, ''), 'Respawning')
  assert.equal(deathMessage(NO_MODIFIERS, 0), 'Respawning')
})

test('the advance menu counts what has been unlocked', () => {
  assert.equal(advanceProgress(0), '0/10 weapons unlocked')
  assert.equal(advanceProgress(0b1), '1/10 weapons unlocked')
  assert.equal(advanceProgress(0b1010), '2/10 weapons unlocked')
  assert.equal(advanceProgress(0b1111111111), '10/10 weapons unlocked')
  assert.equal(advanceProgress(undefined), '0/10 weapons unlocked')
})

test('advance greys out a primary the player has not earned but never a secondary', () => {
  const advance = { realistic: false, survival: false, advance: true }
  assert.ok(!isUnlocked(advance, 0, 0), 'no primaries earned')
  assert.ok(isUnlocked(advance, 0b1, 0), 'the first one is earned')
  assert.ok(!isUnlocked(advance, 0b1, 3))
  assert.ok(isUnlocked(advance, 0, 11), 'secondaries are never locked')

  // Outside advance everything is available.
  assert.ok(isUnlocked(NO_MODIFIERS, 0, 5))
  assert.ok(isUnlocked(undefined, 0, 5))
})
