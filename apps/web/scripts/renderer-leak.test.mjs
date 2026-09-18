import test from 'node:test'
import assert from 'node:assert/strict'
import { createCanvasStub, createGlStub, installBrowserGlobals } from './gl-stub.mjs'

/// The renderer reaches for browser globals in its constructor, so they go in before it is loaded.
const browser = installBrowserGlobals()
const { GameClient } = await import('../src/game.ts')

function start() {
  const stub = createGlStub()
  const canvas = createCanvasStub(stub.gl)
  const client = new GameClient(canvas, () => {})
  return { stub, canvas, client }
}

test('a renderer that has been destroyed leaves no GL resource behind', () => {
  const { stub, client } = start()
  // A texture only exists once the art has loaded, which is the case a leak usually hides in.
  client.soldierTexture = stub.gl.createTexture()
  assert.ok(stub.created.program.length > 0, 'it really did make something to leak')

  client.destroy()
  assert.deepEqual(stub.leaked(), {}, 'every program, shader, buffer, and texture was released')
})

test('a renderer that never loaded its art still tears down cleanly', () => {
  const { stub, client } = start()
  client.destroy()
  assert.deepEqual(stub.leaked(), {})
})

test('destroying twice is not an error and does not double-delete', () => {
  const { stub, client } = start()
  client.destroy()
  const afterFirst = stub.deleted.program.length
  client.destroy()
  assert.equal(stub.deleted.program.length, afterFirst, 'the second teardown released nothing again')
})

test('every listener the renderer added is taken off again', () => {
  const before = browser.windowListeners.length
  const { canvas, client } = start()
  assert.ok(browser.windowListeners.length > before, 'it really did attach something')
  assert.ok(canvas.listeners.length > 0)

  client.destroy()
  assert.equal(browser.windowListeners.length, before, 'the window is left as it was found')
  assert.deepEqual(canvas.listeners, [], 'and so is the canvas')
})

test('the animation frame and the resize observer are both stopped', () => {
  const { client } = start()
  const observer = browser.observers[browser.observers.length - 1]
  assert.equal(observer.connected, true)
  assert.ok(browser.pendingFrames() > 0)

  client.destroy()
  assert.equal(observer.connected, false)
  assert.equal(browser.pendingFrames(), 0, 'nothing is still being asked to draw')
})

test('opening and closing a hundred matches leaks nothing', () => {
  const windowBefore = browser.windowListeners.length
  for (let i = 0; i < 100; i += 1) {
    const { stub, canvas, client } = start()
    client.soldierTexture = stub.gl.createTexture()
    client.destroy()
    assert.deepEqual(stub.leaked(), {}, `round ${i}`)
    assert.deepEqual(canvas.listeners, [])
  }
  assert.equal(browser.windowListeners.length, windowBefore)
  assert.equal(browser.pendingFrames(), 0)
})

test.after(() => browser.restore())
