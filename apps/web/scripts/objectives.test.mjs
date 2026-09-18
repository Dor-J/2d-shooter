// Acceptance evidence for docs/parity/coverage.json:
//   web:objectives:status    — flag status lines, carrier, and missing-flag indicators
//   web:objectives:countdown — the return countdown on a dropped flag
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  carrierOf,
  flagName,
  flagStatusLines,
  isCarrying,
  isMissing,
  returnCountdown,
} from '../src/objectives.ts'

const flag = (kind, state) => ({ kind, base: { x: 0, y: 0 }, state, body: { pos: { x: 0, y: 0 } } })

test('each flag is named the way players talk about it', () => {
  assert.equal(flagName('Alpha'), 'Red flag')
  assert.equal(flagName('Bravo'), 'Blue flag')
  assert.equal(flagName('Yellow'), 'Yellow flag')
})

test('a flag at base is not missing and has no carrier', () => {
  const home = flag('Alpha', 'AtBase')
  assert.equal(carrierOf(home), null)
  assert.equal(isMissing(home), false)
  assert.equal(returnCountdown(home), null)
})

test('a carried flag names its carrier and counts as missing', () => {
  const held = flag('Bravo', { Carried: { by: 7 } })
  assert.equal(carrierOf(held), 7)
  assert.equal(isMissing(held), true)
  assert.equal(returnCountdown(held), null, 'it is not counting down while held')
})

test('a dropped flag counts down in whole seconds', () => {
  assert.equal(returnCountdown(flag('Yellow', { Dropped: { ticks_left: 1500 } })), 25)
  assert.equal(returnCountdown(flag('Yellow', { Dropped: { ticks_left: 61 } })), 2)
  assert.equal(returnCountdown(flag('Yellow', { Dropped: { ticks_left: 1 } })), 1)
})

test('the status lines say where every flag is', () => {
  const lines = flagStatusLines(
    {
      flags: [
        flag('Alpha', 'AtBase'),
        flag('Bravo', { Carried: { by: 7 } }),
        flag('Yellow', { Dropped: { ticks_left: 600 } }),
      ],
    },
    { 7: 'Ada' },
  )
  assert.equal(lines.length, 3)
  assert.match(lines[0].text, /at base/)
  assert.equal(lines[0].missing, false)
  assert.match(lines[1].text, /taken by Ada/)
  assert.equal(lines[1].missing, true)
  assert.match(lines[2].text, /back in 10s/)
})

test('an unknown carrier still reads sensibly rather than showing undefined', () => {
  const lines = flagStatusLines({ flags: [flag('Bravo', { Carried: { by: 99 } })] }, {})
  assert.doesNotMatch(lines[0].text, /undefined/)
  assert.match(lines[0].text, /someone/)
})

test('a mode with no flags produces no status lines', () => {
  assert.deepEqual(flagStatusLines({ flags: [] }, {}), [])
  assert.deepEqual(flagStatusLines(undefined, {}), [])
  assert.equal(isCarrying(undefined, 1), false)
})

test('the carrier indicator knows who is holding something', () => {
  const objectives = { flags: [flag('Bravo', { Carried: { by: 7 } })] }
  assert.ok(isCarrying(objectives, 7))
  assert.ok(!isCarrying(objectives, 8))
})
