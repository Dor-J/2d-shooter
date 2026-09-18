// Acceptance evidence: web:screens:flow
import test from 'node:test'
import assert from 'node:assert/strict'
import { afterConnect, afterDisconnect, afterFirstRun, afterJoin, afterLeave, back, confirmDisconnect, initialMenu, openScreen, SCREEN_COPY, SCREENS } from '../src/screens/flow.ts'

test('first-run lands on the profile picker and a returning player lands on the main menu', () => {
  assert.equal(initialMenu(false).screen, 'first-run')
  assert.equal(initialMenu(true).screen, 'main')
})

test('every named screen is a real place', () => {
  for (const screen of ['credits', 'help', 'pause', 'team', 'onboarding', 'loading', 'download', 'disconnected']) {
    assert.ok(SCREENS.includes(screen))
    assert.ok(SCREEN_COPY[screen].title)
  }
})

test('first-run on a phone goes to touch onboarding', () => {
  assert.equal(afterFirstRun(initialMenu(false), true).screen, 'onboarding')
  assert.equal(afterFirstRun(initialMenu(false), false).screen, 'profiles')
})

test('opening a screen remembers where they came from so back works', () => {
  const next = openScreen(initialMenu(true), 'options')
  assert.equal(next.screen, 'options')
  assert.equal(back(next).screen, 'main')
})

test('the join and leave path is a loop, not a stack that grows forever', () => {
  let state = afterConnect(initialMenu(true))
  assert.equal(state.screen, 'join')
  state = afterJoin(state)
  assert.equal(state.screen, 'match')
  state = afterLeave(state)
  assert.equal(state.screen, 'join')
})

test('a dropped connection asks for confirmation before it throws them at the menu', () => {
  const lost = afterDisconnect(afterJoin(initialMenu(true)))
  assert.equal(lost.screen, 'disconnected')
  assert.equal(confirmDisconnect(lost).screen, 'main')
})
