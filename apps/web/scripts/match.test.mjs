// Acceptance evidence for docs/parity/coverage.json:
//   web:match:clock  — the match clock counts down a timed match and up an untimed one
//   web:match:banner — countdown, overtime, round end, and map transition are announced
import test from 'node:test'
import assert from 'node:assert/strict'
import { clockText, formatClock, isPlayable, outcomeText, phaseBanner, winnerName } from '../src/match.ts'

const timed = { kind: 'Deathmatch', limits: { kills: 30, points: 0, captures: 0, time_ticks: 60 * 60 * 10 }, friendly_fire: false }
const untimed = { kind: 'Deathmatch', limits: { kills: 30, points: 0, captures: 0, time_ticks: 0 }, friendly_fire: false }
const state = (phase, extra = {}) => ({ phase, phase_ticks: 0, elapsed: 0, outcome: null, ...extra })

test('a timed match counts down and an untimed one counts up', () => {
  assert.equal(clockText(state('Active', { elapsed: 0 }), timed), '10:00')
  assert.equal(clockText(state('Active', { elapsed: 60 * 60 }), timed), '9:00')
  assert.equal(clockText(state('Active', { elapsed: 60 * 90 }), untimed), '1:30')
})

test('a clock never runs past zero or shows a negative time', () => {
  assert.equal(clockText(state('Active', { elapsed: 60 * 60 * 20 }), timed), '0:00')
  assert.equal(formatClock(-5), '0:00')
  assert.equal(formatClock(61), '1:01')
})

test('the banner announces every phase but ordinary play', () => {
  assert.equal(phaseBanner(state('Active')), '')
  assert.equal(phaseBanner(state('Lobby')), 'Waiting for players')
  assert.equal(phaseBanner(state('Countdown', { phase_ticks: 180 })), 'Starting in 3')
  assert.equal(phaseBanner(state('Countdown', { phase_ticks: 1 })), 'Starting in 1')
  assert.equal(phaseBanner(state('Overtime')), 'Overtime')
  assert.equal(phaseBanner(state('MapTransition')), 'Loading the next map')
})

test('the end of a round says who won', () => {
  assert.equal(phaseBanner(state('RoundEnd', { outcome: 'Draw' })), 'Draw')
  assert.equal(phaseBanner(state('RoundEnd', { outcome: { Team: 1 } })), 'Alpha wins')
  assert.equal(phaseBanner(state('RoundEnd', { outcome: { Team: 2 } })), 'Bravo wins')
  assert.equal(winnerName({ Player: 7 }, { 7: 'Ada' }), 'Ada')
  assert.equal(winnerName('Draw', {}), null)
  assert.equal(outcomeText(null), 'Round over')
})

test('only an active or overtime match is playable', () => {
  assert.ok(isPlayable(state('Active')))
  assert.ok(isPlayable(state('Overtime')))
  for (const phase of ['Lobby', 'Countdown', 'RoundEnd', 'MapTransition']) {
    assert.ok(!isPlayable(state(phase)), phase)
  }
  assert.ok(!isPlayable(undefined))
})
