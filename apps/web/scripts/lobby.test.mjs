// Acceptance evidence: web:lobby:filter
import test from 'node:test'
import assert from 'node:assert/strict'
import { defaultFilter, filterRooms, parseHostPort, PingAll, roomFlags } from '../src/lobby.ts'

const rooms = [
  { id: 1, name: 'Night', mode: 'deathmatch', players: 16, capacity: 16, map: 'Aero', password: true },
  { id: 2, name: 'Friends', mode: 'ctf', players: 0, capacity: 16, map: 'ctf_Ash' },
  { id: 3, name: 'Arena', mode: 'deathmatch', players: 4, capacity: 16, map: 'Aero', modifiers: { realistic: true } },
]

test('search and the full/empty filters leave the one room that matches', () => {
  const found = filterRooms(rooms, { ...defaultFilter(), search: 'ar', hideFull: true, hideEmpty: true }, [])
  assert.equal(found.length, 1)
  assert.equal(found[0].name, 'Arena')
})

test('favorites-only hides everything that was not starred', () => {
  const found = filterRooms(rooms, { ...defaultFilter(), favoritesOnly: true }, ['Night'])
  assert.equal(found.length, 1)
  assert.equal(found[0].name, 'Night')
})

test('flags name the things a player cares about before they join', () => {
  assert.deepEqual(roomFlags(rooms[0]), ['password', 'full'])
  assert.ok(roomFlags(rooms[2]).includes('realistic'))
  assert.ok(roomFlags({ ...rooms[2], region: 'invite' }).includes('invite'))
})

test('search finds a room by region', () => {
  const found = filterRooms([{ ...rooms[2], region: 'eu' }], { ...defaultFilter(), search: 'eu' }, [])
  assert.equal(found[0].name, 'Arena')
})

test('a host:port is read, and a bad port is refused', () => {
  assert.deepEqual(parseHostPort('play.example:3001'), { host: 'play.example', port: 3001 })
  assert.equal(parseHostPort('play.example:99999'), null)
  assert.deepEqual(parseHostPort('play.example'), { host: 'play.example', port: 3000 })
})

test('ping-all writes the number onto the listing', () => {
  const pings = new PingAll()
  pings.record(3, 42.6)
  assert.equal(pings.apply(rooms).find(room => room.id === 3).ping, 43)
})
