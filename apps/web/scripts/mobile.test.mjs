import test from 'node:test'
import assert from 'node:assert/strict'
import { viewForCanvas, screenToWorld, movementFromDrag } from '../src/mobile.ts'

test('portrait camera follows player and stays inside arena', () => {
  const middle = viewForCanvas(390, 700, 600)
  assert.equal(middle.width, 390)
  assert.equal(middle.x, 405)
  assert.equal(middle.y, 0)
  assert.equal(viewForCanvas(390, 700, 50).x, 0)
  assert.equal(viewForCanvas(390, 700, 1150).x, 810)
})

test('landscape camera shows the whole arena', () => {
  const view = viewForCanvas(1200, 700, 900)
  assert.deepEqual(view, { x: 0, y: 0, width: 1200, height: 700 })
})

test('wide phone camera crops vertically without distorting world units', () => {
  const view = viewForCanvas(667, 280, 600, 350)
  assert.equal(view.width, 1200)
  assert.ok(view.height < 700)
  assert.equal(view.y, (700 - view.height) / 2)
})

test('pointer position maps through camera without stretching aim', () => {
  const view = viewForCanvas(390, 700, 600)
  assert.deepEqual(screenToWorld(195, 350, 390, 700, view), { x: 600, y: 350 })
})

test('movement pad has a dead zone and an upward jump gesture', () => {
  assert.deepEqual(movementFromDrag(3, -2), { left: false, right: false, jump: false })
  assert.deepEqual(movementFromDrag(-35, -42), { left: true, right: false, jump: true })
  assert.deepEqual(movementFromDrag(35, 3), { left: false, right: true, jump: false })
})
