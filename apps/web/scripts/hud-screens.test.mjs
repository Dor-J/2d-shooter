import test from 'node:test'
import assert from 'node:assert/strict'
import {
  endScreen,
  killRatio,
  podiumLabel,
  rowLabel,
  scoreboardRows,
  scoreboardWindow,
  showIds,
  spectatorHud,
  teamSections,
  weaponScreen,
} from '../src/hud/screens.ts'

const players = [
  { id: 1, name: 'Ada', kills: 12, deaths: 4, team: 1, ping: 40 },
  { id: 2, name: 'Bo', kills: 7, deaths: 7, team: 2, ping: 120 },
  { id: 3, name: 'Cy', kills: 7, deaths: 2, team: 1, ping: 20, spectating: true },
]

test('a kill ratio reads as a number even for a player who has never died', () => {
  assert.equal(killRatio(9, 3), '3.00')
  assert.equal(killRatio(5, 0), '5.00')
  assert.equal(killRatio(0, 0), '0.00')
})

test('the scoreboard is ranked, coloured by team, and knows which row is yours', () => {
  const rows = scoreboardRows(players, 2)
  assert.deepEqual(
    rows.map(row => row.id),
    [1, 3, 2],
  )
  assert.equal(rows[0].rank, 1)
  assert.equal(rows[2].own, true)
  assert.equal(rows[1].spectating, true)
  assert.equal(rows[0].ratio, '3.00')
})

test('a scoreboard groups by side and adds each side up', () => {
  const sections = teamSections(scoreboardRows(players, 1), 2)
  assert.deepEqual(
    sections.map(section => section.name),
    ['Alpha', 'Bravo'],
  )
  assert.equal(sections[0].score, 19)
  assert.equal(sections[1].score, 7)
})

test('players with no side get a section of their own rather than vanishing', () => {
  const sections = teamSections(scoreboardRows([{ id: 9, name: 'Free', kills: 1, deaths: 0 }], 9), 2)
  assert.equal(sections.length, 3)
  assert.equal(sections[2].name, 'No team')
  assert.equal(sections[2].rows.length, 1)
})

test('a long scoreboard scrolls a window rather than overflowing the screen', () => {
  const rows = Array.from({ length: 20 }, (unused, index) => index)
  const page = scoreboardWindow(rows, 5, 8)
  assert.deepEqual(page.rows, [5, 6, 7, 8, 9, 10, 11, 12])
  assert.equal(page.more, 7)
  assert.equal(page.scrollable, true)
})

test('scrolling past the end lands on the end rather than on nothing', () => {
  const rows = Array.from({ length: 10 }, (unused, index) => index)
  const page = scoreboardWindow(rows, 999, 4)
  assert.deepEqual(page.rows, [6, 7, 8, 9])
  assert.equal(page.start, 6)
  assert.equal(page.more, 0)
})

test('a scoreboard that fits does not pretend to scroll', () => {
  const page = scoreboardWindow([1, 2, 3], 0, 8)
  assert.equal(page.scrollable, false)
  assert.equal(page.more, 0)
  assert.deepEqual(scoreboardWindow([], 0, 8).rows, [])
})

test('player IDs are shown to somebody who can actually type a command with one', () => {
  assert.equal(showIds(true), true)
  assert.equal(showIds(false), false)
  const [row] = scoreboardRows([players[0]], 1)
  assert.equal(rowLabel(row, true), '#1 Ada (you)')
  assert.equal(rowLabel(row, false), 'Ada (you)')
})

test('the end screen says who won and by how much', () => {
  const rows = scoreboardRows(players, 1)
  const screen = endScreen({ outcome: 'winner', winner: { name: 'Ada' }, nextMapIn: 300 }, rows)
  assert.equal(screen.title, 'Ada wins')
  assert.equal(screen.subtitle, 'by 5 points')
  assert.equal(screen.nextIn, 5)
  assert.equal(screen.podium.length, 3)
})

test('a one-point win is said in the singular and a dead heat is called a draw', () => {
  const close = scoreboardRows(
    [
      { id: 1, name: 'A', kills: 5, deaths: 0 },
      { id: 2, name: 'B', kills: 4, deaths: 0 },
    ],
    1,
  )
  assert.equal(endScreen({ outcome: 'winner', winner: { name: 'A' } }, close).subtitle, 'by 1 point')
  const drawn = scoreboardRows(
    [
      { id: 1, name: 'A', kills: 5, deaths: 1 },
      { id: 2, name: 'B', kills: 5, deaths: 2 },
    ],
    1,
  )
  const screen = endScreen({ outcome: 'draw' }, drawn)
  assert.equal(screen.title, 'Draw')
  assert.equal(screen.subtitle, 'Nobody could be separated')
  assert.equal(screen.nextIn, null, 'a server with no rotation does not count down to nothing')
})

test('a team win names the team', () => {
  const screen = endScreen({ outcome: 'team', winner: { name: 'Alpha', team: 1 } }, [])
  assert.equal(screen.title, 'Alpha wins')
  assert.deepEqual(screen.podium, [])
})

test('the podium line reads as a sentence', () => {
  const [row] = scoreboardRows([players[0]], 0)
  assert.equal(podiumLabel(row), '1st · Ada · 12')
})

test('the weapon screen is ordered by what actually worked', () => {
  const rows = weaponScreen([
    { weapon: 'Ak74', kills: 3, shots: 100, hits: 40 },
    { weapon: 'Barrett', kills: 9, shots: 20, hits: 12 },
    { weapon: 'Knife', kills: 0, shots: 0, hits: 0 },
  ])
  assert.deepEqual(
    rows.map(row => row.weapon),
    ['Barrett', 'Ak74', 'Knife'],
  )
  assert.equal(rows[0].accuracy, 60)
  assert.equal(rows[0].rank, 1)
  assert.equal(rows[2].accuracy, 0, 'a weapon never fired is nought per cent, not a divide by zero')
})

test('no statistics yet is an empty screen rather than a crash', () => {
  assert.deepEqual(weaponScreen(undefined), [])
})

test('a spectator following somebody is told who, and mirrors their HUD', () => {
  const hud = spectatorHud({ following: 2, delay_ticks: 180 }, players)
  assert.equal(hud.mode, 'follow')
  assert.equal(hud.following.name, 'Bo')
  assert.equal(hud.delay, 3)
  assert.equal(hud.mirrorPlayerHud, true)
  assert.match(hud.hint, /Following Bo/)
})

test('a free camera mirrors nobody, because there is no player whose HUD it would be', () => {
  const hud = spectatorHud({ free: true }, players)
  assert.equal(hud.mode, 'free')
  assert.equal(hud.following, null)
  assert.equal(hud.mirrorPlayerHud, false)
  assert.match(hud.hint, /Free camera/)
})

test('a player who is not spectating has no spectator HUD at all', () => {
  assert.equal(spectatorHud(undefined, players), null)
})

test('following somebody who has left falls back to a free camera rather than a blank name', () => {
  const hud = spectatorHud({ following: 404 }, players)
  assert.equal(hud.following, null)
  assert.equal(hud.mirrorPlayerHud, false)
})
