// Acceptance evidence for docs/parity/coverage.json:
//   web:input:rebinding        — keyboard rebinding, conflicts, clearing, and storage round-trips
//   web:input:mouse-rebinding  — mouse buttons bound, rebound, and read as actions
//   web:input:sensitivity      — runtime mouse-sensitivity adjustment inside safe bounds
//   web:input:weapon-selection — numeric, cycling, and separate primary/secondary selection
//   web:input:backflip-combo   — the backflip key and the crouch+jump combination
//   web:input:flag-throw       — the dedicated key and the jump+crouch combo
import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import {
  ACTIONS,
  CAPABILITY_CONTROLS,
  POINTER_CONTROLS,
  actionIds,
} from '../src/input/actions.ts'
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
import {
  KeyboardDevice,
  isTextEntryTarget,
  browserWouldSteal,
  lockCapturableKeys,
} from '../src/input/keyboard.ts'
import {
  MouseDevice,
  clampSensitivity,
  aimFromPointer,
} from '../src/input/mouse.ts'
import {
  encodeInput,
  resolveWeapon,
  PRIMARY_WEAPONS,
  SECONDARY_WEAPONS,
} from '../src/input/encoder.ts'

const coveragePath = fileURLToPath(
  new URL('../../../docs/parity/coverage.json', import.meta.url),
)
const coverage = JSON.parse(readFileSync(coveragePath, 'utf8'))

test('every gap-2 control maps to an action, the pointer path, or a named capability', () => {
  const required = coverage.features
    .filter((entry) => entry.id.startsWith('G02-'))
    .map((entry) => entry.id)
  const covered = new Set([
    ...ACTIONS.flatMap((action) => action.gaps),
    ...Object.values(POINTER_CONTROLS).flat(),
    ...Object.values(CAPABILITY_CONTROLS).flat(),
  ])
  const missing = required.filter((id) => !covered.has(id))
  assert.deepEqual(
    missing,
    [],
    `gap-2 controls without an input mapping: ${missing.join(', ')}`,
  )
  for (const id of covered)
    assert.ok(required.includes(id), `${id} is not a gap-2 feature`)
})

function bindingCodes(action) {
  return DEFAULT_BINDINGS[action].map((binding) => bindingCode(binding))
}

test('soldat defaults bind the soldat.pl table and leave roll, backflip, and pickup empty', () => {
  assert.equal(new Set(actionIds()).size, ACTIONS.length)
  for (const action of [
    'roll',
    'backflip',
    'pickup',
    'throwKnife',
    'demoFastForward',
  ]) {
    assert.deepEqual(
      DEFAULT_BINDINGS[action],
      [],
      `${action} has no soldat default`,
    )
  }
  assert.deepEqual(bindingCodes('jump'), [
    'keyboard:KeyW',
    'keyboard:Space',
    'keyboard:ArrowUp',
    'gamepad:Button0',
  ])
  assert.deepEqual(bindingCodes('jet'), ['mouse:Mouse2', 'gamepad:Button6'])
  assert.equal(bindingCodes('crouch')[0], 'keyboard:KeyS')
  assert.deepEqual(bindingCodes('throwGrenade'), [
    'keyboard:KeyE',
    'gamepad:Button5',
  ])
  assert.deepEqual(bindingCodes('throwWeapon'), ['keyboard:KeyF'])
  assert.deepEqual(bindingCodes('chat'), ['keyboard:KeyT'])
  assert.deepEqual(bindingCodes('teamChat'), ['keyboard:KeyY'])
  assert.deepEqual(bindingCodes('console'), ['keyboard:Slash'])
  assert.deepEqual(bindingCodes('flagThrow'), ['keyboard:KeyB'])
  assert.deepEqual(bindingCodes('selectWeapon1'), ['keyboard:Digit1'])
  assert.deepEqual(bindingCodes('selectWeapon10'), ['keyboard:Digit0'])
  assert.deepEqual(bindingCodes('selectWeapon11'), ['keyboard:Control+Digit1'])
  assert.deepEqual(bindingCodes('selectWeapon12'), ['keyboard:Control+Digit2'])
  assert.deepEqual(bindingCodes('selectWeapon13'), ['keyboard:Control+Digit3'])
  assert.deepEqual(bindingCodes('selectWeapon14'), ['keyboard:Control+Digit4'])
  assert.deepEqual(bindingCodes('scoreboard'), [
    'keyboard:Tab',
    'keyboard:F1',
    'gamepad:Button8',
  ])
  assert.deepEqual(bindingCodes('weaponStats'), ['keyboard:F2'])
  assert.deepEqual(bindingCodes('sniperLine'), ['keyboard:Alt+F2'])
  assert.deepEqual(bindingCodes('minimap'), ['keyboard:F3'])
  assert.deepEqual(bindingCodes('performanceStats'), ['keyboard:Alt+F3'])
  assert.deepEqual(bindingCodes('screenshot'), ['keyboard:F4'])
  assert.deepEqual(bindingCodes('musicToggle'), ['keyboard:F5'])
  assert.deepEqual(bindingCodes('musicPrevious'), ['keyboard:F6'])
  assert.deepEqual(bindingCodes('musicNext'), ['keyboard:F7'])
  assert.deepEqual(bindingCodes('demoRecord'), ['keyboard:F8'])
  assert.deepEqual(bindingCodes('minimize'), ['keyboard:F9'])
  assert.deepEqual(bindingCodes('pause'), ['keyboard:F10'])
  assert.deepEqual(bindingCodes('volumeUp'), ['keyboard:Shift+Equal'])
  assert.deepEqual(bindingCodes('volumeDown'), ['keyboard:Shift+Minus'])
  assert.deepEqual(bindingCodes('sensitivityUp'), [
    'keyboard:Control+Alt+Equal',
  ])
  assert.deepEqual(bindingCodes('sensitivityDown'), [
    'keyboard:Control+Alt+Minus',
  ])
  assert.deepEqual(bindingCodes('emoteTaunt'), ['keyboard:Alt+KeyA'])
  assert.deepEqual(bindingCodes('emoteVictory'), ['keyboard:Alt+KeyB'])
  assert.deepEqual(bindingCodes('emoteMercy'), ['keyboard:Alt+KeyC'])
  assert.deepEqual(bindingCodes('emoteCigar'), ['keyboard:Alt+Digit1'])
  for (const action of ACTIONS) {
    if (
      ['roll', 'backflip', 'pickup', 'throwKnife', 'demoFastForward'].includes(
        action.id,
      )
    )
      continue
    assert.ok(
      DEFAULT_BINDINGS[action.id]?.length > 0,
      `${action.id} has no default binding`,
    )
  }
})

test('version 1 profiles keep a custom binding and reset only untouched soldat defaults', () => {
  const parsed = parseBindings(
    JSON.stringify({
      version: 1,
      bindings: {
        jet: [
          { device: 'keyboard', code: 'ShiftLeft' },
          { device: 'keyboard', code: 'ShiftRight' },
          { device: 'gamepad', code: 'Button6' },
        ],
        reload: [{ device: 'keyboard', code: 'KeyL' }],
      },
    }),
  )
  assert.deepEqual(parsed.jet, [
    { device: 'mouse', code: 'Mouse2' },
    { device: 'gamepad', code: 'Button6' },
  ])
  assert.deepEqual(parsed.reload, [{ device: 'keyboard', code: 'KeyL' }])
  assert.deepEqual(parsed.jump, DEFAULT_BINDINGS.jump)
})

test('default bindings contain no duplicate codes', () => {
  const seen = new Map()
  for (const [action, bindings] of Object.entries(DEFAULT_BINDINGS)) {
    for (const binding of bindings) {
      const code = bindingCode(binding)
      assert.equal(
        seen.get(code),
        undefined,
        `${code} is bound to both ${seen.get(code)} and ${action}`,
      )
      seen.set(code, action)
    }
  }
})

test('rebinding a keyboard control replaces the default and steals the code from its previous owner', () => {
  assert.deepEqual(actionsForCode(DEFAULT_BINDINGS, 'keyboard:KeyR'), [
    'reload',
  ])
  const next = rebind(DEFAULT_BINDINGS, 'reload', 0, {
    device: 'keyboard',
    code: 'KeyA',
  })
  assert.deepEqual(actionsForCode(next, 'keyboard:KeyA'), ['reload'])
  assert.deepEqual(actionsForCode(next, 'keyboard:KeyR'), [])
  assert.ok(!next.moveLeft.some((binding) => binding.code === 'KeyA'))
  assert.ok(
    next.moveLeft.length > 0,
    'move left keeps its remaining alternate binding',
  )
  assert.deepEqual(
    conflictsFor(
      DEFAULT_BINDINGS,
      { device: 'keyboard', code: 'KeyA' },
      'reload',
    ),
    ['moveLeft'],
  )
})

test('a control can be cleared and restored without touching other controls', () => {
  const cleared = clearBinding(DEFAULT_BINDINGS, 'jet', 0)
  assert.equal(cleared.jet.length, DEFAULT_BINDINGS.jet.length - 1)
  assert.deepEqual(cleared.fire, DEFAULT_BINDINGS.fire)
})

test('mouse buttons are rebindable and default fire to the left button', () => {
  assert.deepEqual(actionsForCode(DEFAULT_BINDINGS, 'mouse:Mouse0'), ['fire'])
  assert.deepEqual(actionsForCode(DEFAULT_BINDINGS, 'mouse:Mouse2'), ['jet'])
  const next = rebind(DEFAULT_BINDINGS, 'fire', 0, {
    device: 'mouse',
    code: 'Mouse2',
  })
  assert.deepEqual(actionsForCode(next, 'mouse:Mouse2'), ['fire'])
  assert.deepEqual(actionsForCode(next, 'mouse:Mouse0'), [])
})

test('bindings round-trip through storage and reject malformed payloads', () => {
  const custom = rebind(DEFAULT_BINDINGS, 'prone', 0, {
    device: 'keyboard',
    code: 'KeyZ',
  })
  const restored = parseBindings(serializeBindings(custom))
  assert.deepEqual(restored.prone, custom.prone)
  assert.equal(JSON.parse(serializeBindings(custom)).version, BINDINGS_VERSION)
  assert.deepEqual(parseBindings('not json'), DEFAULT_BINDINGS)
  assert.deepEqual(
    parseBindings(JSON.stringify({ version: 999, bindings: {} })),
    DEFAULT_BINDINGS,
  )
  const partial = parseBindings(
    JSON.stringify({
      version: BINDINGS_VERSION,
      bindings: {
        reload: [{ device: 'keyboard', code: 'KeyP' }],
        bogusAction: [{ device: 'keyboard', code: 'KeyQ' }],
        jump: 'nonsense',
      },
    }),
  )
  assert.deepEqual(partial.reload, [{ device: 'keyboard', code: 'KeyP' }])
  assert.deepEqual(partial.jump, DEFAULT_BINDINGS.jump)
  assert.equal('bogusAction' in partial, false)
})

test('action state reports pressed once, held continuously, and released once', () => {
  const state = new ActionState()
  state.set('fire', true)
  assert.ok(
    state.pressed('fire') && state.held('fire') && !state.released('fire'),
  )
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
  assert.ok(
    !state.held('moveRight'),
    'held actions release when input is suspended',
  )
  keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  assert.ok(
    !state.held('moveRight'),
    'suspended input never reaches the simulation',
  )
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
  assert.deepEqual(aimFromPointer(600, 350, 1200, 700, view), {
    x: 600,
    y: 350,
  })
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
  assert.equal(
    resolveWeapon(13, state),
    0,
    'next weapon wraps past the last weapon',
  )
  state.releaseAll()
  state.commit()
  keyboard.keydown({ code: 'Comma', target: null, preventDefault() {} })
  assert.equal(
    resolveWeapon(0, state),
    13,
    'previous weapon wraps past the first weapon',
  )
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
  const frame = encodeInput(state, {
    seq: 7,
    aim: { x: 800, y: 120 },
    weapon: 3,
  })
  assert.deepEqual(frame, {
    seq: 7,
    left: false,
    right: true,
    jump: false,
    jet: false,
    crouch: true,
    prone: false,
    roll: true,
    emote: null,
    reload: true,
    fire: true,
    throw_grenade: false,
    aim: { x: 800, y: 120 },
    weapon: 3,
    drop: false,
    throw_weapon: false,
    throw_knife: false,
    pickup: false,
  })
})

test('drop, throw, knife, and pickup reach the protocol frame', () => {
  const state = new ActionState()
  state.set('dropWeapon', true)
  state.set('throwWeapon', true)
  state.set('throwKnife', true)
  state.set('pickup', true)
  const frame = encodeInput(state, { seq: 1, aim: { x: 0, y: 0 }, weapon: 0 })
  assert.equal(frame.drop, true)
  assert.equal(frame.throw_weapon, true)
  assert.equal(frame.throw_knife, true)
  assert.equal(frame.pickup, true)
})

test('the dedicated flag-throw key and the jump-plus-crouch combo set throw_weapon', () => {
  const state = new ActionState()
  state.set('flagThrow', true)
  assert.equal(
    encodeInput(state, {
      seq: 1,
      aim: { x: 0, y: 0 },
      weapon: 0,
      airborne: true,
    }).throw_weapon,
    true,
  )
  state.releaseAll()
  state.commit()
  state.set('jump', true)
  state.set('crouch', true)
  assert.equal(
    encodeInput(state, {
      seq: 2,
      aim: { x: 0, y: 0 },
      weapon: 0,
      airborne: false,
    }).throw_weapon,
    true,
  )
})

test('switchWeapon toggles between the two carried slots', () => {
  const state = new ActionState()
  state.set('switchWeapon', true)
  assert.equal(
    resolveWeapon(0, state, { primary: 0, secondary: 10 }, [0, 10]),
    10,
  )
})

test('roll and backflip reach the simulation through the same roll field', () => {
  const state = new ActionState()
  state.set('roll', true)
  assert.equal(
    encodeInput(state, { seq: 1, aim: { x: 0, y: 0 }, weapon: 0 }).roll,
    true,
  )
  state.releaseAll()
  state.commit()
  state.set('backflip', true)
  assert.equal(
    encodeInput(state, { seq: 2, aim: { x: 0, y: 0 }, weapon: 0 }).roll,
    true,
  )
})

test('grounded crouch plus a direction rolls, and jet during a jump backflips', () => {
  const state = new ActionState()
  state.set('crouch', true)
  state.set('moveLeft', true)
  assert.equal(
    encodeInput(state, {
      seq: 1,
      aim: { x: 0, y: 0 },
      weapon: 0,
      airborne: false,
    }).roll,
    true,
  )
  state.releaseAll()
  state.commit()
  state.set('jump', true)
  state.set('crouch', true)
  assert.equal(
    encodeInput(state, {
      seq: 2,
      aim: { x: 0, y: 0 },
      weapon: 0,
      airborne: true,
    }).roll,
    false,
    'airborne jump plus crouch is not a backflip',
  )
  state.releaseAll()
  state.commit()
  state.set('jump', true)
  state.set('jet', true)
  state.set('moveRight', true)
  assert.equal(
    encodeInput(state, {
      seq: 3,
      aim: { x: 0, y: 0 },
      weapon: 0,
      airborne: true,
    }).roll,
    true,
  )
  state.releaseAll()
  state.commit()
  state.set('jump', true)
  state.set('crouch', true)
  const grounded = encodeInput(state, {
    seq: 4,
    aim: { x: 0, y: 0 },
    weapon: 0,
    airborne: false,
  })
  assert.equal(grounded.throw_weapon, true)
  assert.equal(grounded.roll, false)
})

test('ctrl number keys select secondary slots without firing the primary', () => {
  const state = new ActionState()
  const keyboard = new KeyboardDevice(state, DEFAULT_BINDINGS)
  keyboard.keydown({
    code: 'Digit1',
    ctrlKey: true,
    target: null,
    canvasFocused: true,
    preventDefault() {},
  })
  assert.equal(resolveWeapon(0, state), 10)
  state.releaseAll()
  state.commit()
  keyboard.keyup({ code: 'Digit1', ctrlKey: true, target: null })
  keyboard.keydown({
    code: 'Digit1',
    target: null,
    preventDefault() {},
  })
  assert.equal(resolveWeapon(0, state), 0)
})

test('a focused canvas swallows stolen soldat keys and never Ctrl+W', () => {
  assert.equal(browserWouldSteal({ code: 'F5' }), true)
  assert.equal(browserWouldSteal({ code: 'F10' }), true)
  assert.equal(browserWouldSteal({ code: 'Tab' }), true)
  assert.equal(browserWouldSteal({ code: 'Digit4', ctrlKey: true }), true)
  assert.equal(browserWouldSteal({ code: 'Digit5', ctrlKey: true }), false)
  assert.equal(browserWouldSteal({ code: 'KeyW', ctrlKey: true }), false)
  const prevented = []
  const state = new ActionState()
  const keyboard = new KeyboardDevice(state, DEFAULT_BINDINGS)
  keyboard.keydown({
    code: 'F1',
    canvasFocused: true,
    target: null,
    preventDefault() {
      prevented.push('F1')
    },
  })
  keyboard.keydown({
    code: 'Digit2',
    ctrlKey: true,
    canvasFocused: true,
    target: null,
    preventDefault() {
      prevented.push('Digit2')
    },
  })
  keyboard.keydown({
    code: 'KeyW',
    ctrlKey: true,
    canvasFocused: true,
    target: null,
    preventDefault() {
      prevented.push('KeyW')
    },
  })
  keyboard.keydown({
    code: 'F5',
    canvasFocused: false,
    target: null,
    preventDefault() {
      prevented.push('unfocused')
    },
  })
  assert.deepEqual(prevented, ['F1', 'Digit2'])
})

test('keyboard lock requests capturable codes and ignores rejection', async () => {
  const seen = []
  lockCapturableKeys({
    lock(codes) {
      seen.push(codes)
      return Promise.reject(new Error('denied'))
    },
  })
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.ok(seen[0].includes('F1') && seen[0].includes('F10'))
  assert.ok(seen[0].includes('Tab'))
  assert.ok(seen[0].includes('Digit1') && seen[0].includes('Digit4'))
  assert.equal(seen[0].includes('KeyW'), false)
  assert.equal(lockCapturableKeys(null), undefined)
})

test('the controls screen notes keys the browser can still steal', () => {
  const source = readFileSync(
    fileURLToPath(new URL('../src/ControlsSettings.vue', import.meta.url)),
    'utf8',
  )
  assert.match(source, /Ctrl\+W/)
  assert.match(source, /Keyboard Lock/)
})

test('taunt shortcuts send one emote on the press frame only', () => {
  const state = new ActionState()
  state.set('emoteVictory', true)
  assert.equal(
    encodeInput(state, { seq: 1, aim: { x: 0, y: 0 }, weapon: 0 }).emote,
    'victory',
  )
  state.commit()
  assert.equal(
    encodeInput(state, { seq: 2, aim: { x: 0, y: 0 }, weapon: 0 }).emote,
    null,
  )
})
