// Acceptance evidence: web:a11y:labels
import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { resolve, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const app = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), '../src/App.vue'), 'utf8')

test('interactive lobby and match controls keep accessible names', () => {
  for (const label of [
    'YOUR CALLSIGN',
    'Search rooms',
    'Room password',
    'Invite code',
    'Game arena',
    'Chat message',
    'Connection',
    'Scoreboard',
  ]) {
    assert.ok(app.includes(label), label)
  }
  assert.match(app, /aria-label="Game arena"/)
  assert.match(app, /role="alert"/)
  assert.match(app, /role="status"/)
})
