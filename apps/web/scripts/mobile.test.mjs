import test from 'node:test'
import assert from 'node:assert/strict'
import { ARENA_H, ARENA_W, VIEW_H, VIEW_W, viewForCanvas } from '../src/mobile.ts'
import { movementFromDrag } from '../src/input/touch.ts'
import { aimFromPointer } from '../src/input/mouse.ts'

test('soldat camera crops the arena and follows the player', () => {
  const view = viewForCanvas(1200, 700, 600)
  assert.equal(view.width, VIEW_W)
  assert.ok(view.height < ARENA_H)
  assert.equal(view.x, 600 - VIEW_W / 2)
  assert.ok(view.x > 0)
})

test('portrait camera follows player and stays inside arena', () => {
  const middle = viewForCanvas(390, 700, 600)
  assert.ok(middle.width < ARENA_W)
  assert.ok(middle.height <= VIEW_H)
  assert.equal(viewForCanvas(390, 700, 50).x, 0)
  const right = viewForCanvas(390, 700, 1150)
  assert.ok(right.x > 0)
  assert.ok(right.x + right.width <= ARENA_W + 1e-6)
})

test('wide canvas keeps square world units and crops', () => {
  const view = viewForCanvas(667, 280, 600, 350)
  assert.ok(Math.abs(view.width / view.height - 667 / 280) < 1e-6)
  assert.ok(view.width <= VIEW_W)
  assert.ok(view.height < ARENA_H)
})

test('look-ahead shifts the camera toward aim', () => {
  const centered = viewForCanvas(1200, 700, 600, 350)
  const looking = viewForCanvas(1200, 700, 600, 350, 1000, 350)
  assert.ok(looking.x > centered.x)
})

test('pointer position maps through camera without stretching aim', () => {
  const view = viewForCanvas(390, 700, 600)
  assert.deepEqual(aimFromPointer(195, 350, 390, 700, view), { x: 600, y: 350 })
})

test('movement pad has a dead zone and an upward jump gesture', () => {
  assert.deepEqual(movementFromDrag(3, -2), { left: false, right: false, jump: false })
  assert.deepEqual(movementFromDrag(-35, -42), { left: true, right: false, jump: true })
  assert.deepEqual(movementFromDrag(35, 3), { left: false, right: true, jump: false })
})
