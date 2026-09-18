import test from 'node:test'
import assert from 'node:assert/strict'
import {
  BOT_DIFFICULTIES,
  MAX_BOTS,
  clampCount,
  createRoomBots,
  describeBots,
  readDifficulty,
  setBotsMessage,
} from '../src/bots.ts'

test('a count typed into the form is read as a number the server will take', () => {
  assert.equal(clampCount('4'), 4)
  assert.equal(clampCount(4), 4)
  assert.equal(clampCount('4.7'), 4)
})

test('a count outside the range is pulled back into it rather than refused', () => {
  assert.equal(clampCount(-3), 0)
  assert.equal(clampCount(99), MAX_BOTS)
  assert.equal(clampCount(''), 0, 'an empty field means no bots')
  assert.equal(clampCount('nonsense'), 0)
  assert.equal(clampCount(undefined), 0)
})

test('every difficulty the menu offers is one the server knows', () => {
  for (const difficulty of BOT_DIFFICULTIES) {
    assert.equal(readDifficulty(difficulty), difficulty)
  }
  assert.equal(BOT_DIFFICULTIES.length, 4)
})

test('an unrecognised difficulty falls back the way the server does', () => {
  assert.equal(readDifficulty('IMPOSSIBLE'), 'normal')
  assert.equal(readDifficulty(''), 'normal')
  assert.equal(readDifficulty(null), 'normal')
  assert.equal(readDifficulty(' Elite '), 'elite', 'stray spacing and case are forgiven')
})

test('a room opened with no bots says nothing about them', () => {
  assert.deepEqual(createRoomBots({ count: 0, difficulty: 'elite' }), {})
})

test('a room opened with bots carries the count and the difficulty', () => {
  assert.deepEqual(createRoomBots({ count: 5, difficulty: 'veteran' }), {
    bots: 5,
    bot_difficulty: 'veteran',
  })
})

test('the create message never asks for more bots than the room takes', () => {
  assert.deepEqual(createRoomBots({ count: 40, difficulty: 'rookie' }), {
    bots: MAX_BOTS,
    bot_difficulty: 'rookie',
  })
})

test('changing the bot count of an open room sends a zero rather than nothing', () => {
  assert.deepEqual(setBotsMessage({ count: 0, difficulty: 'normal' }), {
    type: 'bots',
    count: 0,
    difficulty: 'normal',
  })
})

test('the roster line reads as a sentence', () => {
  assert.equal(describeBots({ count: 0, difficulty: 'elite' }), 'No bots')
  assert.equal(describeBots({ count: 1, difficulty: 'rookie' }), '1 bot · Rookie')
  assert.equal(describeBots({ count: 6, difficulty: 'veteran' }), '6 bots · Veteran')
})
