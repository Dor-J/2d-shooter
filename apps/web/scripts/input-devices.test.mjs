// Acceptance evidence for docs/parity/coverage.json:
//   web:input:gamepad       — gamepad buttons, sticks, dead zone, rebinding, and disconnects
//   web:input:profiles      — saved control profiles, persistence, import/export, and migration
//   web:input:accessibility — single-control alternatives for combos and hold-to-toggle latching
import test from 'node:test'
import assert from 'node:assert/strict'
import { DEFAULT_BINDINGS, rebind } from '../src/input/bindings.ts'
import { ActionState } from '../src/input/state.ts'
import { GamepadDevice, GAMEPAD_DEADZONE, gamepadCodes } from '../src/input/gamepad.ts'
import {
  TOUCH_BUTTONS,
  TouchDevice,
  movementFromDrag,
  touchLayout,
  aimFromStick,
} from '../src/input/touch.ts'
import {
  ACCESSIBILITY_COMBOS,
  DEFAULT_PROFILE_SETTINGS,
  PROFILE_VERSION,
  ProfileStore,
  createProfile,
  exportProfile,
  importProfile,
  migrateProfile,
} from '../src/input/profiles.ts'
import { backflipRequested, flagThrowRequested, encodeInput } from '../src/input/encoder.ts'

function memoryStorage() {
  const map = new Map()
  return {
    getItem: key => (map.has(key) ? map.get(key) : null),
    setItem: (key, value) => map.set(key, String(value)),
    removeItem: key => map.delete(key),
  }
}

function fakePad(buttons = [], axes = [0, 0, 0, 0]) {
  return { id: 'Test Pad', connected: true, buttons: buttons.map(pressed => ({ pressed })), axes }
}

test('gamepad buttons and sticks drive the same actions as the keyboard', () => {
  const state = new ActionState()
  const pad = new GamepadDevice(state, DEFAULT_BINDINGS)
  pad.poll(fakePad([true], [0, 0, 0, 0]))
  assert.ok(state.held('jump'), 'button 0 jumps by default')
  state.commit()
  pad.poll(fakePad([false], [-1, 0, 0, 0]))
  assert.ok(state.held('moveLeft') && !state.held('moveRight'))
  assert.ok(!state.held('jump'), 'releasing the pad button releases the action')
  pad.poll(fakePad([false], [1, 0, 0, 0]))
  assert.ok(state.held('moveRight') && !state.held('moveLeft'))
})

test('gamepad sticks ignore drift inside the dead zone', () => {
  const state = new ActionState()
  const pad = new GamepadDevice(state, DEFAULT_BINDINGS)
  pad.poll(fakePad([], [GAMEPAD_DEADZONE - 0.01, 0, 0, 0]))
  assert.ok(!state.held('moveRight') && !state.held('moveLeft'))
  assert.deepEqual(pad.aimStick(fakePad([], [0, 0, 0.02, -0.02])), null)
  assert.deepEqual(pad.aimStick(fakePad([], [0, 0, 1, 0])), { x: 1, y: 0 })
})

test('gamepad bindings are rebindable and disconnecting the pad releases its actions', () => {
  const state = new ActionState()
  const bindings = rebind(DEFAULT_BINDINGS, 'reload', 0, { device: 'gamepad', code: 'Button0' })
  const pad = new GamepadDevice(state, bindings)
  pad.poll(fakePad([true]))
  assert.ok(state.held('reload') && !state.held('jump'))
  pad.releaseAll()
  assert.ok(!state.held('reload'))
  assert.ok(gamepadCodes().includes('Button0') && gamepadCodes().includes('Axis0+'))
})

test('touch buttons cover every stance, combat, and panel control the gap list asks for', () => {
  const required = ['crouch', 'prone', 'roll', 'reload', 'switchWeapon', 'throwWeapon', 'flagThrow', 'scoreboard', 'teamChat', 'jet', 'throwGrenade']
  const provided = TOUCH_BUTTONS.map(button => button.action)
  for (const action of required) assert.ok(provided.includes(action), `touch layout is missing ${action}`)
  for (const button of TOUCH_BUTTONS) assert.ok(button.label.length > 0 && button.hint.length > 0)
})

test('a cancelled touch releases every action that touch is holding', () => {
  const state = new ActionState()
  const touch = new TouchDevice(state)
  touch.press(1, 'crouch')
  touch.press(2, 'jet')
  assert.ok(state.held('crouch') && state.held('jet'))
  touch.cancel()
  assert.ok(!state.held('crouch') && !state.held('jet'))
  touch.press(3, 'prone')
  touch.release(3)
  assert.ok(!state.held('prone'))
})

test('touch pointers are tracked individually so one finger cannot release another', () => {
  const state = new ActionState()
  const touch = new TouchDevice(state)
  touch.press(1, 'fire')
  touch.press(2, 'jet')
  touch.release(1)
  assert.ok(!state.held('fire') && state.held('jet'))
})

test('touch layout keeps both orientations inside the device safe areas', () => {
  const portrait = touchLayout(390, 780)
  const landscape = touchLayout(844, 390)
  assert.equal(portrait.orientation, 'portrait')
  assert.equal(landscape.orientation, 'landscape')
  for (const layout of [portrait, landscape]) {
    assert.ok(layout.padding.bottom.includes('safe-area-inset-bottom'))
    assert.ok(layout.padding.left.includes('safe-area-inset-left'))
    assert.ok(layout.padding.right.includes('safe-area-inset-right'))
    assert.ok(layout.padSize >= 96 && layout.padSize <= 160)
  }
  assert.ok(landscape.padSize <= portrait.padSize, 'short screens use smaller pads')
})

test('the aim stick keeps the drag direction and the movement pad keeps its dead zone', () => {
  assert.deepEqual(movementFromDrag(3, -2), { left: false, right: false, jump: false })
  assert.deepEqual(movementFromDrag(-35, -42), { left: true, right: false, jump: true })
  assert.deepEqual(aimFromStick({ x: 100, y: 100 }, 1, 0, 400), { x: 500, y: 100 })
  assert.equal(aimFromStick({ x: 100, y: 100 }, 0, 0, 400), null)
})

test('profiles persist bindings and per-profile settings across sessions', () => {
  const storage = memoryStorage()
  const store = new ProfileStore(storage)
  assert.equal(store.list().length, 1, 'a default profile exists on first run')
  const custom = store.create('Southpaw')
  store.select(custom.id)
  store.update(custom.id, profile => ({
    ...profile,
    bindings: rebind(profile.bindings, 'fire', 0, { device: 'mouse', code: 'Mouse2' }),
    settings: { ...profile.settings, sensitivity: 2.5, volume: 0.3 },
  }))
  const reopened = new ProfileStore(storage)
  assert.equal(reopened.active().name, 'Southpaw')
  assert.equal(reopened.active().settings.sensitivity, 2.5)
  assert.deepEqual(reopened.active().bindings.fire, [
    { device: 'mouse', code: 'Mouse2' },
    { device: 'gamepad', code: 'Button7' },
  ])
  assert.equal(reopened.list().length, 2)
})

test('profiles can be renamed, deleted, exported, and imported', () => {
  const storage = memoryStorage()
  const store = new ProfileStore(storage)
  const created = store.create('Tourney')
  store.update(created.id, profile => ({ ...profile, name: 'Tournament' }))
  const text = exportProfile(store.get(created.id))
  const imported = importProfile(text)
  assert.equal(imported?.name, 'Tournament')
  assert.notEqual(imported?.id, created.id, 'an imported profile gets its own id')
  store.select(created.id)
  store.remove(created.id)
  assert.equal(store.list().some(profile => profile.id === created.id), false)
  assert.ok(store.active(), 'removing the active profile falls back to another one')
  assert.equal(importProfile('{"nope":true}'), null)
  assert.equal(importProfile('not json'), null)
})

test('stored profiles from an older version are migrated instead of discarded', () => {
  const migrated = migrateProfile({ version: 0, name: 'Legacy', bindings: { reload: [{ device: 'keyboard', code: 'KeyP' }] } })
  assert.equal(migrated.version, PROFILE_VERSION)
  assert.deepEqual(migrated.bindings.reload, [{ device: 'keyboard', code: 'KeyP' }])
  assert.deepEqual(migrated.settings, DEFAULT_PROFILE_SETTINGS)
  assert.equal(migrateProfile(null).name, createProfile('Default').name)
})

test('every combined input has a single-control alternative', () => {
  for (const combo of ACCESSIBILITY_COMBOS) {
    assert.ok(DEFAULT_BINDINGS[combo.alternative]?.length > 0, `${combo.alternative} needs a default binding`)
    assert.ok(combo.keys.length > 1)
  }
  const state = new ActionState()
  state.set('jump', true)
  state.set('crouch', true)
  assert.equal(backflipRequested(state, true), true, 'the combo still works in the air')
  assert.equal(flagThrowRequested(state, false), true, 'the combo still works on the ground')
  state.releaseAll()
  state.commit()
  state.set('backflip', true)
  assert.equal(backflipRequested(state, false), true, 'the dedicated key needs no second control')
  state.releaseAll()
  state.commit()
  state.set('flagThrow', true)
  assert.equal(flagThrowRequested(state, true), true)
})

test('hold controls can be switched to toggles for one-handed play', () => {
  const state = new ActionState()
  state.setToggleActions(['jet', 'prone'])
  state.set('jet', true)
  state.set('jet', false)
  assert.ok(state.held('jet'), 'a toggled control stays on after the key is released')
  assert.equal(encodeInput(state, { seq: 1, aim: { x: 0, y: 0 }, weapon: 0 }).jet, true)
  state.commit()
  state.set('jet', true)
  assert.ok(!state.held('jet'), 'pressing again turns it off')
  state.setToggleActions([])
  state.set('prone', true)
  state.set('prone', false)
  assert.ok(!state.held('prone'), 'clearing the option restores hold behaviour')
})
