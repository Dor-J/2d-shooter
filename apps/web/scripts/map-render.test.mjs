import assert from 'node:assert/strict'
import test from 'node:test'
import { polygonVertexBuffer } from '../src/map-render.ts'

test('polygon render buffer preserves simulation coordinates', () => {
  const buffer = polygonVertexBuffer([{ vertices: [{ x: 1, y: 2 }, { x: 5, y: 3 }, { x: 4, y: 8 }], kind: 'Ice' }])
  assert.deepEqual([...buffer], [1, 2, 5, 3, 4, 8])
})
