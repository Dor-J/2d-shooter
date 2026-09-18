// Acceptance evidence for docs/parity/coverage.json:
//   web:stats:scoreboard — rank, difference from the leader, and the limit line
//   web:stats:weapons    — per-weapon shots, hits, accuracy, and kills
//   web:stats:summary    — the end-of-round summary line
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  accuracyPercent,
  behindText,
  favouriteWeapon,
  limitText,
  summaryLine,
  weaponRows,
} from '../src/stats.ts'

const row = (over = {}) => ({
  player: 1,
  team: 0,
  rank: 1,
  behind_leader: 0,
  score: { points: 5, kills: 5, deaths: 2, teamkills: 0, suicides: 0, objectives: 0 },
  stats: { shots: 10, hits: 4, headshots: 1, captures: 0, returns: 0, holds: 0 },
  ...over,
})

test('accuracy is a whole percentage and never divides by zero', () => {
  assert.equal(accuracyPercent({ shots: 10, hits: 4 }), 40)
  assert.equal(accuracyPercent({ shots: 3, hits: 1 }), 33, 'rounded down')
  assert.equal(accuracyPercent({ shots: 0, hits: 0 }), 0)
  assert.equal(accuracyPercent(undefined), 0)
})

test('the leader leads and everybody else is behind by a counted amount', () => {
  assert.equal(behindText(row()), 'Leading')
  assert.equal(behindText(row({ rank: 2, behind_leader: 1 })), '1 point behind')
  assert.equal(behindText(row({ rank: 3, behind_leader: 7 })), '7 points behind')
})

test('the limit line says what it takes to win', () => {
  assert.equal(
    limitText({ kills: 30, points: 0, captures: 0, time_ticks: 36000 }),
    'First to 30 kills or 10 minutes',
  )
  assert.equal(
    limitText({ kills: 0, points: 0, captures: 10, time_ticks: 0 }),
    'First to 10 captures',
  )
  assert.equal(limitText({ kills: 0, points: 0, captures: 0, time_ticks: 0 }), 'No limit')
  assert.equal(limitText(undefined), '')
})

test('weapon rows are sorted by kills and carry their own accuracy', () => {
  const stats = {
    shots: 20,
    hits: 9,
    headshots: 2,
    captures: 0,
    returns: 0,
    holds: 0,
    weapons: {
      Ak74: { shots: 10, hits: 4, kills: 1, deaths: 0, headshots: 0 },
      Spas12: { shots: 10, hits: 5, kills: 4, deaths: 1, headshots: 2 },
    },
  }
  const rows = weaponRows(stats)
  assert.equal(rows[0].weapon, 'Spas12', 'most kills first')
  assert.equal(rows[0].accuracy, 50)
  assert.equal(rows[1].weapon, 'Ak74')
  assert.equal(rows[1].accuracy, 40)
  assert.equal(favouriteWeapon(stats), 'Spas12')
})

test('a player who has killed with nothing has no favourite weapon', () => {
  assert.equal(favouriteWeapon(undefined), null)
  assert.equal(
    favouriteWeapon({
      shots: 4,
      hits: 0,
      headshots: 0,
      captures: 0,
      returns: 0,
      holds: 0,
      weapons: { Ak74: { shots: 4, hits: 0, kills: 0, deaths: 2, headshots: 0 } },
    }),
    null,
  )
  assert.deepEqual(weaponRows(undefined), [])
})

test('the summary line reads as a sentence', () => {
  const line = summaryLine(row({ rank: 2 }), 'Ada')
  assert.match(line, /^2\. Ada/)
  assert.match(line, /5 pts/)
  assert.match(line, /5\/2/)
  assert.match(line, /40% accuracy/)
})
