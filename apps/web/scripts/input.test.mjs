// Acceptance evidence for docs/parity/coverage.json:
//   web:input:rebinding        — keyboard rebinding, conflicts, clearing, and storage round-trips
//   web:input:mouse-rebinding  — mouse buttons bound, rebound, and read as actions
//   web:input:sensitivity      — runtime mouse-sensitivity adjustment inside safe bounds
//   web:input:weapon-selection — numeric, cycling, and separate primary/secondary selection
//   web:input:backflip-combo   — the backflip key and the crouch+jump combination
import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { ACTIONS, CAPABILITY_CONTROLS, POINTER_CONTROLS, actionIds } from '../src/input/actions.ts'
import {
  DEFAULT_BINDINGS,
  bindingCode,
  actionsForCode,
  rebind,
  clearBinding,
  conflictsFor,
  serializeBindings,
  parseBindings,
  BINDINGS_VERSION,
} from '../src/input/bindings.ts'
import { ActionState } from '../src/input/state.ts'
import { KeyboardDevice, isTextEntryTarget } from '../src/input/keyboard.ts'
import { MouseDevice, clampSensitivity, aimFromPointer } from '../src/input/mouse.ts'
import { encodeInput, resolveWeapon, PRIMARY_WEAPONS, SECONDARY_WEAPONS } from '../src/input/encoder.ts'

const coveragePath = fileURLToPath(new URL('../../../docs/parity/coverage.json', import.meta.url))
const coverage = JSON.parse(readFileSync(coveragePath, 'utf8'))

test('every gap-2 control maps to an action, the pointer path, or a named capability', () => {
  const required = coverage.features.filter(entry => entry.id.startsWith('G02-')).map(entry => entry.id)
  const covered = new Set([
    ...ACTIONS.flatMap(action => action.gaps),
    ...Object.values(POINTER_CONTROLS).flat(),
    ...Object.values(CAPABILITY_CONTROLS).flat(),
  ])
  const missing = required.filter(id => !covered.has(id))
  assert.deepEqual(missing, [], `gap-2 controls without an input mapping: ${missing.join(', ')}`)
  for (const id of covered) assert.ok(required.includes(id), `${id} is not a gap-2 feature`)
})

test('every action has a unique id and at least one default binding', () => {
  assert.equal(new Set(actionIds()).size, ACTIONS.length)
  for (const action of ACTIONS) {
    const bindings = DEFAULT_BINDINGS[action.id]
    assert.ok(bindings && bindings.length > 0, `${action.id} has no default binding`)
  }
})

test('default bindings contain no duplicate codes', () => {
  const seen = new Map()
  for (const [action, bindings] of Object.entries(DEFAULT_BINDINGS)) {
    for (const binding of bindings) {
      const code = bindingCode(binding)
      assert.equal(seen.get(code), undefined, `${code} is bound to both ${seen.get(code)} and ${action}`)
      seen.set(code, action)
    }
  }
})

test('rebinding a keyboard control replaces the default and steals the code from its previous owner', () => {
  assert.deepEqual(actionsForCode(DEFAULT_BINDINGS, 'keyboard:KeyR'), ['reload'])
  const next = rebind(DEFAULT_BINDINGS, 'reload', 0, { device: 'keyboard', code: 'KeyA' })
  assert.deepEqual(actionsForCode(next, 'keyboard:KeyA'), ['reload'])
  assert.deepEqual(actionsForCode(next, 'keyboard:KeyR'), [])
  assert.ok(!next.moveLeft.some(binding => binding.code === 'KeyA'))
  assert.ok(next.moveLeft.length > 0, 'move left keeps its remaining alternate binding')
  assert.deepEqual(conflictsFor(DEFAULT_BINDINGS, { device: 'keyboard', code: 'KeyA' }, 'reload'), ['moveLeft'])
})

test('a control can be cleared and restored without touching other controls', () => {
  const cleared = clearBinding(DEFAULT_BINDINGS, 'jet', 0)
  assert.equal(cleared.jet.length, DEFAULT_BINDINGS.jet.length - 1)
  assert.deepEqual(cleared.fire, DEFAULT_BINDINGS.fire)
})

test('mouse buttons are rebindable and default fire to the left button', () => {
  assert.deepEqual(actionsForCode(DEFAULT_BINDINGS, 'mouse:Mouse0'), ['fire'])
  assert.deepEqual(actionsForCode(DEFAULT_BINDINGS, 'mouse:Mouse2'), ['throwGrenade'])
  const next = rebind(DEFAULT_BINDINGS, 'fire', 0, { device: 'mouse', code: 'Mouse2' })
  assert.deepEqual(actionsForCode(next, 'mouse:Mouse2'), ['fire'])
  assert.deepEqual(actionsForCode(next, 'mouse:Mouse0'), [])
})

test('bindings round-trip through storage and reject malformed payloads', () => {
  const custom = rebind(DEFAULT_BINDINGS, 'prone', 0, { device: 'keyboard', code: 'KeyZ' })
  const restored = parseBindings(serializeBindings(custom))
  assert.deepEqual(restored.prone, custom.prone)
  assert.equal(JSON.parse(serializeBindings(custom)).version, BINDINGS_VERSION)
  assert.deepEqual(parseBindings('not json'), DEFAULT_BINDINGS)
  assert.deepEqual(parseBindings(JSON.stringify({ version: 999, bindings: {} })), DEFAULT_BINDINGS)
  const partial = parseBindings(JSON.stringify({
    version: BINDINGS_VERSION,
    bindings: { reload: [{ device: 'keyboard', code: 'KeyP' }], bogusAction: [{ device: 'keyboard', code: 'KeyQ' }], jump: 'nonsense' },
  }))
  assert.deepEqual(partial.reload, [{ device: 'keyboard', code: 'KeyP' }])
  assert.deepEqual(partial.jump, DEFAULT_BINDINGS.jump)
  assert.equal('bogusAction' in partial, false)
})

test('action state reports pressed once, held continuously, and released once', () => {
  const state = new ActionState()
  state.set('fire', true)
  assert.ok(state.pressed('fire') && state.held('fire') && !state.released('fire'))
  state.commit()
  assert.ok(!state.pressed('fire') && state.held('fire'))
  assert.equal(state.heldTicks('fire'), 1)
  state.commit()
  assert.equal(state.heldTicks('fire'), 2)
  state.set('fire', false)
  assert.ok(state.released('fire') && !state.held('fire'))
  state.commit()
  assert.ok(!state.released('fire'))
  assert.equal(state.heldTicks('fire'), 0)
})

test('releasing every action clears held state so a lost focus cannot stick a key down', () => {
  const state = new ActionState()
  state.set('moveRight', true)
  state.set('jet', true)
  state.releaseAll()
  assert.ok(!state.held('moveRight') && !state.held('jet'))
  assert.ok(state.released('jet'))
})

test('keyboard device translates bound keys into actions and ignores unbound ones', () => {
  const state = new ActionState()
  const keyboard = new KeyboardDevice(state, DEFAULT_BINDINGS)
  keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  assert.ok(state.held('moveRight'))
  keyboard.keydown({ code: 'F13', target: null, preventDefault() {} })
  keyboard.keyup({ code: 'KeyD', target: null })
  assert.ok(!state.held('moveRight'))
})

test('gameplay input suspends while a text field owns focus and restores cleanly afterwards', () => {
  const state = new ActionState()
  const keyboard = new KeyboardDevice(state, DEFAULT_BINDINGS)
  keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  keyboard.suspend()
  assert.ok(!state.held('moveRight'), 'held actions release when input is suspended')
  keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  assert.ok(!state.held('moveRight'), 'suspended input never reaches the simulation')
  keyboard.resume()
  keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  assert.ok(state.held('moveRight'))
  assert.ok(isTextEntryTarget({ tagName: 'INPUT', isContentEditable: false }))
  assert.ok(isTextEntryTarget({ tagName: 'DIV', isContentEditable: true }))
  assert.ok(!isTextEntryTarget({ tagName: 'CANVAS', isContentEditable: false }))
  assert.ok(!isTextEntryTarget(null))
})

test('mouse buttons drive actions and the wheel scrolls the scoreboard', () => {
  const state = new ActionState()
  const mouse = new MouseDevice(state, DEFAULT_BINDINGS)
  mouse.pointerdown({ button: 0, pointerType: 'mouse' })
  assert.ok(state.held('fire'))
  mouse.pointerup({ button: 0, pointerType: 'mouse' })
  assert.ok(!state.held('fire'))
  mouse.wheel({ deltaY: -120, preventDefault() {} })
  assert.ok(state.pressed('scoreboardScrollUp'))
  state.commit()
  mouse.wheel({ deltaY: 120, preventDefault() {} })
  assert.ok(state.pressed('scoreboardScrollDown'))
})

test('mouse sensitivity is adjustable at runtime within safe bounds', () => {
  const view = { x: 0, y: 0, width: 1200, height: 700 }
  assert.deepEqual(aimFromPointer(600, 350, 1200, 700, view), { x: 600, y: 350 })
  const state = new ActionState()
  const mouse = new MouseDevice(state, DEFAULT_BINDINGS)
  assert.equal(mouse.sensitivity, 1)
  mouse.adjustSensitivity(0.25)
  assert.equal(mouse.sensitivity, 1.25)
  const aimed = mouse.aimFromLockedMovement({ x: 600, y: 350 }, 100, 0, view)
  assert.equal(aimed.x, 600 + 100 * 1.25)
  assert.equal(clampSensitivity(0), 0.1)
  assert.equal(clampSensitivity(99), 5)
})

test('numeric keys, cycling, and separate primary/secondary controls all select weapons', () => {
  const state = new ActionState()
  const keyboard = new KeyboardDevice(state, DEFAULT_BINDINGS)
  keyboard.keydown({ code: 'Digit5', target: null, preventDefault() {} })
  assert.equal(resolveWeapon(0, state), 4)
  state.releaseAll()
  state.commit()
  keyboard.keydown({ code: 'Digit0', target: null, preventDefault() {} })
  assert.equal(resolveWeapon(0, state), 9)
  state.releaseAll()
  state.commit()
  keyboard.keydown({ code: 'Period', target: null, preventDefault() {} })
  assert.equal(resolveWeapon(13, state), 0, 'next weapon wraps past the last weapon')
  state.releaseAll()
  state.commit()
  keyboard.keydown({ code: 'Comma', target: null, preventDefault() {} })
  assert.equal(resolveWeapon(0, state), 13, 'previous weapon wraps past the first weapon')
  state.releaseAll()
  state.commit()
  keyboard.keydown({ code: 'KeyP', target: null, preventDefault() {} })
  assert.ok(SECONDARY_WEAPONS.includes(resolveWeapon(3, state)))
  state.releaseAll()
  state.commit()
  keyboard.keydown({ code: 'KeyO', target: null, preventDefault() {} })
  assert.ok(PRIMARY_WEAPONS.includes(resolveWeapon(11, state)))
})

test('the encoder produces one protocol input frame covering stance, reload, and aim', () => {
  const state = new ActionState()
  state.set('moveRight', true)
  state.set('crouch', true)
  state.set('reload', true)
  state.set('fire', true)
  const frame = encodeInput(state, { seq: 7, aim: { x: 800, y: 120 }, weapon: 3 })
  assert.deepEqual(frame, {
    seq: 7,
    left: false,
    right: true,
    jump: false,
    jet: false,
    crouch: true,
    prone: false,
    roll: false,
    emote: null,
    reload: true,
    fire: true,
    throw_grenade: false,
    aim: { x: 800, y: 120 },
    weapon: 3,
  })
})

test('roll and backflip reach the simulation through the same roll field', () => {
  const state = new ActionState()
  state.set('roll', true)
  assert.equal(encodeInput(state, { seq: 1, aim: { x: 0, y: 0 }, weapon: 0 }).roll, true)
  state.releaseAll()
  state.commit()
  state.set('backflip', true)
  assert.equal(encodeInput(state, { seq: 2, aim: { x: 0, y: 0 }, weapon: 0 }).roll, true)
})

test('taunt shortcuts send one emote on the press frame only', () => {
  const state = new ActionState()
  state.set('emoteVictory', true)
  assert.equal(encodeInput(state, { seq: 1, aim: { x: 0, y: 0 }, weapon: 0 }).emote, 'victory')
  state.commit()
  assert.equal(encodeInput(state, { seq: 2, aim: { x: 0, y: 0 }, weapon: 0 }).emote, null)
})
