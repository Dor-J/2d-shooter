// Acceptance evidence for docs/parity/coverage.json:
//   web:firing:refusal — a refused shot is explained rather than silently dropped
import test from 'node:test'
import assert from 'node:assert/strict'
import { fireRefusalText } from '../src/weapons.ts'

test('a refused LAW shot tells the player what to do about it', () => {
  const text = fireRefusalText('NeedsBracing')
  assert.match(text, /LAW/)
  assert.match(text, /crouch|prone/i, 'it names the stance that would work')
})

test('an unrecognised refusal still says something rather than nothing', () => {
  const text = fireRefusalText('SomethingNew')
  assert.ok(text.length > 0)
  assert.doesNotMatch(text, /undefined/)
})
