// Acceptance evidence for docs/parity/coverage.json:
//   web:input:scoreboard        — hold and toggle scoreboard behaviour
//   web:input:scoreboard-scroll — bounded scoreboard scrolling that resets when it closes
//   web:input:screenshot        — the screenshot control and its timestamped file name
//   web:input:minimize          — the browser equivalent of the minimize shortcut
import test from 'node:test'
import assert from 'node:assert/strict'
import { ActionState } from '../src/input/state.ts'
import { InterfaceState } from '../src/input/overlays.ts'
import { screenshotFilename, captureCanvas } from '../src/input/screenshot.ts'
import { InputSystem } from '../src/input/index.ts'
import { DEFAULT_BINDINGS } from '../src/input/bindings.ts'

function press(state, action) {
  state.set(action, true)
}

test('the scoreboard follows the key while held and latches when the mode is toggle', () => {
  const state = new ActionState()
  const ui = new InterfaceState()
  press(state, 'scoreboard')
  ui.apply(state)
  assert.equal(ui.scoreboardVisible, true)
  state.commit()
  state.set('scoreboard', false)
  ui.apply(state)
  assert.equal(ui.scoreboardVisible, false, 'hold mode hides the board on release')

  ui.scoreboardMode = 'toggle'
  state.commit()
  press(state, 'scoreboard')
  ui.apply(state)
  assert.equal(ui.scoreboardVisible, true)
  state.commit()
  state.set('scoreboard', false)
  ui.apply(state)
  assert.equal(ui.scoreboardVisible, true, 'toggle mode keeps the board open after release')
  state.commit()
  press(state, 'scoreboard')
  ui.apply(state)
  assert.equal(ui.scoreboardVisible, false)
})

test('the scoreboard scrolls within bounds and resets when it closes', () => {
  const state = new ActionState()
  const ui = new InterfaceState()
  ui.setScoreboardExtent(24, 8)
  press(state, 'scoreboard')
  ui.apply(state)
  state.commit()
  state.set('scoreboardScrollUp', false)
  for (let i = 0; i < 3; i += 1) {
    state.pulse('scoreboardScrollDown')
    ui.apply(state)
    state.commit()
  }
  assert.equal(ui.scoreboardOffset, 3)
  for (let i = 0; i < 40; i += 1) {
    state.pulse('scoreboardScrollDown')
    ui.apply(state)
    state.commit()
  }
  assert.equal(ui.scoreboardOffset, 16, 'scrolling stops at the last page')
  for (let i = 0; i < 40; i += 1) {
    state.pulse('scoreboardScrollUp')
    ui.apply(state)
    state.commit()
  }
  assert.equal(ui.scoreboardOffset, 0)
  state.set('scoreboard', false)
  ui.apply(state)
  assert.equal(ui.scoreboardVisible, false)
  assert.equal(ui.scoreboardOffset, 0)
})

test('interface toggles flip independently and report what changed', () => {
  const state = new ActionState()
  const ui = new InterfaceState()
  for (const action of ['minimap', 'sniperLine', 'performanceStats', 'weaponStats']) {
    const before = ui[action]
    state.releaseAll()
    state.commit()
    press(state, action)
    const events = ui.apply(state)
    assert.equal(ui[action], !before, `${action} did not toggle`)
    assert.ok(events.some(event => event.type === 'toggle' && event.name === action))
  }
})

test('runtime volume and music controls stay inside their range', () => {
  const state = new ActionState()
  const ui = new InterfaceState()
  ui.volume = 0.95
  state.pulse('volumeUp')
  ui.apply(state)
  assert.equal(ui.volume, 1)
  state.commit()
  ui.volume = 0.05
  state.pulse('volumeDown')
  ui.apply(state)
  assert.equal(ui.volume, 0)
  state.commit()
  press(state, 'musicToggle')
  ui.apply(state)
  assert.equal(ui.musicEnabled, false)
  state.releaseAll()
  state.commit()
  press(state, 'musicNext')
  ui.apply(state)
  assert.equal(ui.musicTrack, 1)
  state.releaseAll()
  state.commit()
  press(state, 'musicPrevious')
  press(state, 'musicPrevious')
  ui.apply(state)
  assert.equal(ui.musicTrack, 0)
})

test('pause, demo, and background controls raise intents instead of guessing at match rules', () => {
  const state = new ActionState()
  const ui = new InterfaceState()
  press(state, 'pause')
  let events = ui.apply(state)
  assert.ok(events.some(event => event.type === 'pause'))
  state.releaseAll()
  state.commit()
  press(state, 'demoRecord')
  events = ui.apply(state)
  assert.equal(ui.recordingDemo, true)
  assert.ok(events.some(event => event.type === 'demo' && event.recording === true))
  state.releaseAll()
  state.commit()
  press(state, 'demoFastForward')
  ui.apply(state)
  assert.equal(ui.fastForward, true)
  state.releaseAll()
  state.commit()
  press(state, 'minimize')
  events = ui.apply(state)
  assert.equal(ui.backgrounded, true)
  assert.ok(events.some(event => event.type === 'minimize'))
})

test('chat controls raise a scoped compose intent', () => {
  const state = new ActionState()
  const ui = new InterfaceState()
  press(state, 'chat')
  assert.deepEqual(
    ui.apply(state).filter(event => event.type === 'chat'),
    [{ type: 'chat', scope: 'all' }],
  )
  state.releaseAll()
  state.commit()
  press(state, 'teamChat')
  assert.deepEqual(
    ui.apply(state).filter(event => event.type === 'chat'),
    [{ type: 'chat', scope: 'team' }],
  )
  state.releaseAll()
  state.commit()
  press(state, 'console')
  ui.apply(state)
  assert.equal(ui.consoleOpen, true)
})

test('a screenshot is saved with a sortable timestamped name', () => {
  assert.equal(
    screenshotFilename(new Date(Date.UTC(2026, 8, 17, 10, 15, 30))),
    'arena-2026-09-17-101530.png',
  )
  const saved = []
  const canvas = { toBlob: callback => callback({ size: 12 }) }
  const name = captureCanvas(canvas, (blob, filename) => saved.push([blob, filename]))
  assert.equal(saved.length, 1)
  assert.equal(saved[0][1], name)
  assert.equal(captureCanvas(null, () => {}), null)
})

test('the input system produces one frame per tick and suspends every device together', () => {
  const state = new ActionState()
  const system = new InputSystem({ bindings: DEFAULT_BINDINGS, state })
  system.keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  const first = system.tick({ aim: { x: 10, y: 20 } })
  assert.equal(first.frame.seq, 1)
  assert.equal(first.frame.right, true)
  const second = system.tick({ aim: { x: 10, y: 20 } })
  assert.equal(second.frame.seq, 2)

  system.suspend()
  const third = system.tick({ aim: { x: 10, y: 20 } })
  assert.equal(third.frame.right, false, 'suspending drops held gameplay actions')
  system.keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  assert.equal(system.tick({ aim: { x: 10, y: 20 } }).frame.right, false)
  system.resume()
  system.keyboard.keydown({ code: 'KeyD', target: null, preventDefault() {} })
  assert.equal(system.tick({ aim: { x: 10, y: 20 } }).frame.right, true)
})

test('the input system applies a profile to every device at once', () => {
  const system = new InputSystem({ bindings: DEFAULT_BINDINGS })
  system.applyProfile({
    version: 1,
    id: 'p',
    name: 'Custom',
    bindings: { ...DEFAULT_BINDINGS, fire: [{ device: 'keyboard', code: 'KeyB' }] },
    settings: { sensitivity: 2, volume: 0.2, musicVolume: 0.1, toggleActions: ['jet'] },
  })
  assert.equal(system.mouse.sensitivity, 2)
  assert.equal(system.ui.volume, 0.2)
  assert.deepEqual(system.state.toggleActions(), ['jet'])
  system.keyboard.keydown({ code: 'KeyB', target: null, preventDefault() {} })
  assert.equal(system.tick({ aim: { x: 0, y: 0 } }).frame.fire, true)
})
