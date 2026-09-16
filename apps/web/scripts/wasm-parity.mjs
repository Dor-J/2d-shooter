import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { resolve } from 'node:path'

const require = createRequire(import.meta.url)
const wasm = require(resolve('target/wasm-node/game_core.js'))
const fixtureText = await readFile(resolve('tests/fixtures/parity/empty-world-one-tick.json'), 'utf8')
const fixture = JSON.parse(fixtureText)
const actual = JSON.parse(wasm.replay_fixture(fixtureText))

assert.deepEqual(actual, fixture.expected)
process.stdout.write(`Wasm parity PASS: ${actual.length} digest matched native fixture\n`)
