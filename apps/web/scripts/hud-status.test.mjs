// Acceptance evidence: web:hud:status
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  MessageLog,
  carrierBanner,
  flagIndicators,
  limitLine,
  ordinal,
  rankedRows,
  respawnLine,
  standingFor,
  teamScores,
  weaponPanel,
  weaponSprite,
} from '../src/hud/status.ts'
import { teamColor } from '../src/hud/gauges.ts'

const rows = [
  { id: 1, name: 'A', kills: 9, deaths: 3 },
  { id: 2, name: 'B', kills: 12, deaths: 7 },
  { id: 3, name: 'C', kills: 9, deaths: 1 },
]

test('players are ranked by score, with fewer deaths breaking a tie', () => {
  assert.deepEqual(
    rankedRows(rows).map(row => row.id),
    [2, 3, 1],
  )
})

test('your rank line says where you are and how far off the lead', () => {
  assert.deepEqual(standingFor(rows, 2), {
    rank: 1,
    score: 12,
    deaths: 7,
    behind: null,
    text: '1st · 12',
  })
  assert.deepEqual(standingFor(rows, 1), {
    rank: 3,
    score: 9,
    deaths: 3,
    behind: 3,
    text: '3rd · 9 (−3)',
  })
})

test('a mode that counts points ranks on points rather than kills', () => {
  const pointed = [
    { id: 1, name: 'A', kills: 20, deaths: 0, points: 1 },
    { id: 2, name: 'B', kills: 0, deaths: 9, points: 5 },
  ]
  assert.equal(standingFor(pointed, 2).rank, 1)
  assert.equal(standingFor(pointed, 1).behind, 4)
})

test('a player who is not in the match has no standing rather than a zeroth place', () => {
  assert.equal(standingFor(rows, 99), null)
})

test('ordinals read the way English does, including the teens', () => {
  assert.deepEqual([1, 2, 3, 4, 11, 12, 13, 21, 22, 101].map(ordinal), [
    '1st',
    '2nd',
    '3rd',
    '4th',
    '11th',
    '12th',
    '13th',
    '21st',
    '22nd',
    '101st',
  ])
})

test('the limit line names whatever actually ends this match', () => {
  assert.equal(limitLine({ kills: 30 }), 'First to 30 kills')
  assert.equal(limitLine({ captures: 5, time: 900 }), 'First to 5 captures · 15 min')
  assert.equal(limitLine({}), 'No limit')
  assert.equal(limitLine(undefined), '')
})

test('a two-team mode shows two counters and a four-team mode shows four', () => {
  assert.equal(teamScores([0, 3, 5], 2).length, 2)
  const four = teamScores([0, 1, 2, 3, 4], 4)
  assert.deepEqual(
    four.map(row => row.name),
    ['Alpha', 'Bravo', 'Charlie', 'Delta'],
  )
  assert.deepEqual(
    four.map(row => row.score),
    [1, 2, 3, 4],
  )
})

test('the leading team and your own team are both called out', () => {
  const scores = teamScores([0, 7, 2], 2, 2)
  assert.equal(scores[0].leading, true)
  assert.equal(scores[1].leading, false)
  assert.equal(scores[1].own, true)
  assert.deepEqual(scores[0].color, teamColor(1))
})

test('a missing score array reads as nil-all rather than crashing the HUD', () => {
  assert.deepEqual(
    teamScores(undefined, 2).map(row => row.score),
    [0, 0],
  )
})

test('a flag at home says so and a stolen one is distinguished from a taken one', () => {
  const flags = [
    { kind: 'Alpha', state: { Base: null }, team: 1 },
    { kind: 'Bravo', state: { Carried: { by: 4 } }, team: 2 },
  ]
  const indicators = flagIndicators(flags, 1)
  assert.equal(indicators[0].state, 'home')
  assert.equal(indicators[1].state, 'carried')
  assert.equal(indicators[1].text, 'Bravo flag taken')

  const fromBravo = flagIndicators(flags, 2)
  assert.equal(fromBravo[1].state, 'taken', 'the same flag is stolen when it is yours')
  assert.equal(fromBravo[1].text, 'Bravo flag stolen')
})

test('a dropped flag counts down to its own return', () => {
  const [indicator] = flagIndicators(
    [{ kind: 'Alpha', state: { Dropped: { ticks_left: 300 } }, team: 1 }],
    2,
  )
  assert.equal(indicator.state, 'dropped')
  assert.equal(indicator.returnIn, 5)
  assert.equal(indicator.text, 'Alpha flag dropped · 5s')
})

test('carrying the objective is said in the middle of the screen, not left to an icon', () => {
  const indicators = flagIndicators([{ kind: 'Bravo', state: { Carried: { by: 1 } }, team: 2 }], 1)
  assert.equal(carrierBanner(indicators, true), 'You have the Bravo flag — take it home')
  assert.equal(carrierBanner(indicators, false), null)
  assert.equal(carrierBanner([], true), 'You are Rambo — only your kills count')
})

test('the respawn line counts down and then becomes the protection line', () => {
  assert.equal(respawnLine(180), 'Respawning in 3')
  assert.equal(respawnLine(1), 'Respawning in 1')
  assert.equal(respawnLine(0, 120), 'Protected for 2')
  assert.equal(respawnLine(0, 0), null)
})

test('the weapon corner names what is in your hands and what is on your back', () => {
  const names = ['Knife', 'Ak74', 'Barrett M82A1']
  const panel = weaponPanel(names, 1, 0, 3)
  assert.equal(panel.primary, 'Ak74')
  assert.equal(panel.secondary, 'Knife')
  assert.equal(panel.grenades, 3)
  assert.equal(panel.grenadeKind, 'Frag')
  assert.equal(panel.sprite, 'weapon-ak74')
})

test('a cluster grenade replaces the frag count rather than being added to it', () => {
  const panel = weaponPanel(['Knife'], 0, null, 2, 4)
  assert.equal(panel.grenades, 4)
  assert.equal(panel.grenadeKind, 'Cluster')
  assert.equal(panel.secondary, null)
})

test('a weapon sprite id survives spaces and punctuation in the name', () => {
  assert.equal(weaponSprite('Barrett M82A1'), 'weapon-barrett-m82a1')
  assert.equal(weaponSprite('USSOCOM'), 'weapon-ussocom')
  assert.equal(weaponSprite('M79'), 'weapon-m79')
})

test('server messages and team chat are kept apart by tone, not by a typed prefix', () => {
  const log = new MessageLog({ limit: 3, ttl: 1000 })
  log.push('Map changed', 'server', 0)
  log.push('regrouping', 'team', 0)
  log.push('(team) not really', 'all', 0)
  assert.deepEqual(
    log.visible(0).map(message => message.tone),
    ['server', 'team', 'all'],
  )
  assert.equal(log.visible(0)[2].tone, 'all', 'typing the word team does not make it team chat')
})

test('the message log forgets old lines and never grows past its limit', () => {
  const log = new MessageLog({ limit: 2, ttl: 1000 })
  log.push('one', 'server', 0)
  log.push('two', 'server', 0)
  log.push('three', 'server', 0)
  assert.equal(log.messages.length, 2)
  assert.deepEqual(
    log.visible(0).map(message => message.text),
    ['two', 'three'],
  )
  assert.deepEqual(log.visible(5000), [])
  log.clear()
  assert.deepEqual(log.messages, [])
})

test('an empty message is not logged at all', () => {
  const log = new MessageLog()
  log.push('', 'server', 0)
  assert.deepEqual(log.messages, [])
})
