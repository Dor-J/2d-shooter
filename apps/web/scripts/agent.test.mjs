import test from 'node:test'
import assert from 'node:assert/strict'
import { DEFAULT_BINDINGS } from '../src/input/bindings.ts'
import { InputSystem } from '../src/input/index.ts'
import { IDLE_INPUT, mergeInput } from './agent-client.mjs'

test('setAction writes the same path as devices and lands in lastFrame', () => {
  const input = new InputSystem({ bindings: DEFAULT_BINDINGS })
  assert.equal(input.lastFrame, null)
  input.setAction('jet', true)
  const { frame } = input.tick({ aim: { x: 0, y: 0 } })
  assert.equal(frame.jet, true)
  assert.equal(input.lastFrame?.jet, true)
  assert.equal(input.lastFrame, frame)
})

test('idle input merge keeps weapon and drop defaults', () => {
  const idle = mergeInput()
  assert.equal(idle.weapon, 0)
  assert.equal(idle.drop, false)
  assert.equal(idle.throw_weapon, false)
  assert.equal(idle.throw_knife, false)
  assert.equal(idle.pickup, false)
  assert.equal(idle.seq, 1)
  assert.deepEqual(idle.aim, IDLE_INPUT.aim)
  const jet = mergeInput({ jet: true }, 4)
  assert.equal(jet.jet, true)
  assert.equal(jet.weapon, IDLE_INPUT.weapon)
  assert.equal(jet.drop, false)
  assert.equal(jet.seq, 4)
})
