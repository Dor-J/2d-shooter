// Acceptance evidence for docs/parity/coverage.json:
//   web:bonuses:hud       — the active-bonus panel, its countdown, and its overlay
//   web:bonuses:predator  — a Predator is drawn nearly invisible until they bleed
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  bonusOverlay,
  bonusText,
  effectName,
  hasBonus,
  kitName,
  playerOpacity,
  secondsLeft,
} from '../src/bonuses.ts'

const effect = (active, ticks_left) => ({ active, ticks_left })
const none = { active: null, ticks_left: 0 }

test('nothing running shows no panel at all', () => {
  assert.equal(hasBonus(none), false)
  assert.equal(hasBonus(undefined), false)
  assert.equal(bonusText(none), '')
  assert.equal(bonusOverlay(none), '')
  assert.equal(secondsLeft(none), 0)
})

test('a running bonus is named and counted down in whole seconds', () => {
  assert.ok(hasBonus(effect('Berserker', 900)))
  assert.equal(secondsLeft(effect('Berserker', 900)), 15)
  assert.equal(secondsLeft(effect('Predator', 61)), 2, 'rounded up, never a false zero')
  assert.equal(secondsLeft(effect('Predator', 1)), 1)
  assert.equal(bonusText(effect('FlameGod', 600)), 'Flame God · 10s')
})

test('each effect has its own overlay', () => {
  assert.equal(bonusOverlay(effect('FlameGod', 10)), 'flame-god')
  assert.equal(bonusOverlay(effect('Berserker', 10)), 'berserker')
  assert.equal(bonusOverlay(effect('Predator', 10)), 'predator')
})

test('the kits and effects read the way players talk about them', () => {
  assert.equal(effectName('FlameGod'), 'Flame God')
  assert.equal(effectName('Berserker'), 'Berserker')
  assert.equal(kitName('ClusterGrenades'), 'Cluster Grenades')
  assert.equal(kitName('Medic'), 'Medical Kit')
  assert.equal(kitName('Vest'), 'Bulletproof Vest')
})

test('a healthy predator is nearly invisible and a wounded one is not', () => {
  const predator = effect('Predator', 1500)
  const healthy = playerOpacity(predator, 100)
  const scratched = playerOpacity(predator, 60)
  const hurt = playerOpacity(predator, 30)
  const dying = playerOpacity(predator, 5)

  assert.ok(healthy < 0.05, `nearly invisible: ${healthy}`)
  assert.ok(healthy > 0, 'but never perfectly so')
  assert.equal(scratched, healthy, 'a scratch does not show')
  assert.ok(hurt > healthy, 'a wound does')
  assert.ok(dying > hurt, 'and the worse it is the more it shows')
  assert.ok(dying <= 1)
})

test('everybody else is drawn solid', () => {
  assert.equal(playerOpacity(none, 10), 1)
  assert.equal(playerOpacity(undefined, 10), 1)
  assert.equal(playerOpacity(effect('Berserker', 10), 10), 1)
})
