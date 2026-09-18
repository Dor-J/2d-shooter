import test from 'node:test'
import assert from 'node:assert/strict'
import {
  DESIGN_HEIGHT,
  DESIGN_WIDTH,
  HUD_PRESETS,
  MAX_SCALE,
  MIN_SCALE,
  clampScale,
  defaultLayout,
  minimalLayout,
  mobileLayout,
  place,
  placeLayout,
  presetLayout,
  readLayout,
  scaleFor,
  withinSafeArea,
} from '../src/hud/layout.ts'

const desktop = { width: DESIGN_WIDTH, height: DESIGN_HEIGHT }

test('a design-sized screen needs no scaling at all', () => {
  assert.equal(scaleFor(desktop), 1)
})

test('a smaller screen scales down by whichever side is tighter', () => {
  assert.equal(scaleFor({ width: DESIGN_WIDTH / 2, height: DESIGN_HEIGHT }), 0.5)
  assert.equal(scaleFor({ width: DESIGN_WIDTH, height: DESIGN_HEIGHT / 2 }), 0.5)
})

test('scaling is clamped so the HUD stays readable and never swallows the match', () => {
  assert.equal(clampScale(0.01), MIN_SCALE)
  assert.equal(clampScale(99), MAX_SCALE)
  assert.equal(clampScale(Number.NaN), 1)
  assert.equal(scaleFor({ width: 320, height: 180 }), MIN_SCALE)
  assert.equal(scaleFor({ width: 7680, height: 4320 }), MAX_SCALE)
})

test("a player's own scaling preference multiplies the fit", () => {
  assert.equal(scaleFor({ ...desktop, scale: 1.5 }), 1.5)
  assert.equal(scaleFor({ width: DESIGN_WIDTH / 2, height: DESIGN_HEIGHT / 2, scale: 1.5 }), 0.75)
})

test('an element is measured inward from its own corner', () => {
  const element = { id: 'x', anchor: 'top-left', x: 10, y: 20, width: 100, height: 40 }
  assert.deepEqual(place(element, desktop), {
    id: 'x',
    x: 10,
    y: 20,
    width: 100,
    height: 40,
    layer: 0,
  })
  const mirrored = place({ ...element, anchor: 'bottom-right' }, desktop)
  assert.equal(mirrored.x, DESIGN_WIDTH - 10 - 100)
  assert.equal(mirrored.y, DESIGN_HEIGHT - 20 - 40)
})

test('a centred element is centred, with its offset applied on top', () => {
  const placed = place(
    { id: 'c', anchor: 'center', x: 0, y: -100, width: 200, height: 50 },
    desktop,
  )
  assert.equal(placed.x, DESIGN_WIDTH / 2 - 100)
  assert.equal(placed.y, DESIGN_HEIGHT / 2 - 25 - 100)
})

test('a notch pushes the HUD inward rather than under it', () => {
  const viewport = { ...desktop, safeArea: { top: 44, right: 12, bottom: 34, left: 12 } }
  const top = place({ id: 't', anchor: 'top-left', x: 0, y: 0, width: 10, height: 10 }, viewport)
  assert.equal(top.x, 12)
  assert.equal(top.y, 44)
  for (const placed of placeLayout(defaultLayout(), viewport)) {
    assert.ok(withinSafeArea(placed, viewport), `${placed.id} stayed out of the insets`)
  }
})

test('every stock element stays on screen at desktop and phone sizes', () => {
  const sizes = [
    { width: 1920, height: 1080 },
    { width: DESIGN_WIDTH, height: DESIGN_HEIGHT },
    { width: 1024, height: 768 },
    { width: 844, height: 390, safeArea: { top: 0, right: 47, bottom: 21, left: 47 } },
    { width: 390, height: 844, safeArea: { top: 47, right: 0, bottom: 34, left: 0 } },
  ]
  for (const viewport of sizes) {
    const layout = viewport.width < viewport.height ? mobileLayout() : defaultLayout()
    for (const placed of placeLayout(layout, viewport)) {
      assert.ok(
        withinSafeArea(placed, viewport),
        `${placed.id} fits ${viewport.width}x${viewport.height}`,
      )
    }
  }
})

test('elements are drawn low layer first and keep their order within a layer', () => {
  const layout = {
    name: 'ordering',
    elements: [
      { id: 'respawn', anchor: 'center', x: 0, y: 0, width: 1, height: 1, layer: 2 },
      { id: 'health', anchor: 'top-left', x: 0, y: 0, width: 1, height: 1 },
      { id: 'ammo', anchor: 'top-left', x: 0, y: 0, width: 1, height: 1 },
    ],
  }
  assert.deepEqual(
    placeLayout(layout, desktop).map(placed => placed.id),
    ['health', 'ammo', 'respawn'],
  )
})

test('an element switched off is not placed at all', () => {
  const layout = {
    name: 'off',
    elements: [{ id: 'health', anchor: 'top-left', x: 0, y: 0, width: 1, height: 1, visible: false }],
  }
  assert.deepEqual(placeLayout(layout, desktop), [])
})

test('the phone layout hides what a thumb would cover and the minimal one keeps only the essentials', () => {
  const mobileIds = placeLayout(mobileLayout(), desktop).map(placed => placed.id)
  assert.ok(!mobileIds.includes('chat'))
  assert.ok(!mobileIds.includes('minimap'))
  assert.ok(mobileIds.includes('health'))

  const minimalIds = placeLayout(minimalLayout(), desktop).map(placed => placed.id)
  assert.deepEqual(minimalIds.sort(), ['ammo', 'health', 'jet', 'notice', 'respawn', 'weapon'])
})

test('every preset names a layout that exists', () => {
  for (const preset of HUD_PRESETS) {
    assert.equal(presetLayout(preset).name, preset)
  }
})

test('a custom HUD moves what it names and leaves the rest where it was', () => {
  const result = readLayout({
    name: 'mine',
    elements: [{ id: 'health', anchor: 'top-right', x: 8, y: 8 }],
  })
  assert.ok('layout' in result)
  assert.equal(result.layout.name, 'mine')
  const health = result.layout.elements.find(element => element.id === 'health')
  assert.equal(health.anchor, 'top-right')
  assert.equal(health.width, 240, 'the size it did not mention is the stock one')
  assert.equal(result.layout.elements.length, defaultLayout().elements.length)
})

test('a custom HUD that names something this game does not draw is refused with a reason', () => {
  const result = readLayout({ elements: [{ id: 'nonsense', anchor: 'top' }] })
  assert.ok('errors' in result)
  assert.deepEqual(result.errors, [
    { element: 'nonsense', reason: 'not a HUD element this game draws' },
  ])
})

test('a custom HUD is refused for a bad anchor, a repeat, a missing id, or a bad number', () => {
  assert.ok('errors' in readLayout({ elements: [{ id: 'health', anchor: 'middle' }] }))
  assert.ok(
    'errors' in readLayout({ elements: [{ id: 'health' }, { id: 'health' }] }),
    'listing one element twice is an error, not a last-one-wins',
  )
  assert.ok('errors' in readLayout({ elements: [{ anchor: 'top' }] }))
  assert.ok('errors' in readLayout({ elements: [{ id: 'health', x: 'far left' }] }))
  assert.ok('errors' in readLayout({ elements: 'not a list' }))
  assert.ok('errors' in readLayout(null))
})
