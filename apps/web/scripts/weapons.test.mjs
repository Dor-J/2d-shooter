// Acceptance evidence for docs/parity/coverage.json:
//   web:weapons:table   — HUD names and hash accept/reject from the snapshot table
//   web:weapons:cursor  — cursor expansion and recoil offset
//   web:weapons:inventory — owned-slot HUD, pickup indicator, pose muzzle origin
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  acceptWeaponTable,
  cursorAppearance,
  DEFAULT_WEAPON_NAMES,
  KNOWN_WEAPON_HASHES,
  muzzleOrigin,
  nearestPickup,
  ownedSlots,
  syncWeaponFromSnapshot,
} from '../src/weapons.ts'

test('known default hashes are accepted and unknown hashes are rejected', () => {
  const normal = acceptWeaponTable({
    name: 'Default mod',
    hash: 911431262,
    names: [...DEFAULT_WEAPON_NAMES],
  })
  assert.equal(normal.ok, true)
  if (normal.ok) {
    assert.equal(normal.name, 'Default mod')
    assert.equal(normal.names[13], 'LAW')
  }

  const realistic = acceptWeaponTable({ hash: 2707142329 })
  assert.equal(realistic.ok, true)
  if (realistic.ok) assert.equal(realistic.name, 'Realistic mod')

  assert.equal(acceptWeaponTable({ hash: 1 }).ok, false)
  assert.equal(acceptWeaponTable({ hash: 911431262, defs: [] }, 99).ok, false)
  assert.equal(acceptWeaponTable(null).ok, false)
})

test('a custom table is accepted only when definitions travel with it', () => {
  const custom = acceptWeaponTable({ name: 'Tweaked', hash: 42, defs: [{ kind: 'DesertEagles' }] })
  assert.equal(custom.ok, true)
  if (custom.ok) assert.equal(custom.name, 'Tweaked')
  assert.equal(acceptWeaponTable({ name: 'Tweaked', hash: 42 }).ok, false)
})

test('the HUD does not keep a second copy of the default names', () => {
  assert.equal(DEFAULT_WEAPON_NAMES.length, 14)
  assert.equal(KNOWN_WEAPON_HASHES[911431262], 'Default mod')
  assert.equal(KNOWN_WEAPON_HASHES[2707142329], 'Realistic mod')
})

test('cursor radius grows with accuracy and recoil lifts the aim point', () => {
  const still = cursorAppearance(0, 0)
  const spoiled = cursorAppearance(0.5, 0.2)
  assert.ok(spoiled.radius > still.radius)
  assert.ok(spoiled.offsetY < 0)
})

test('a snapshot does not cancel a switch onto a carried weapon', () => {
  assert.equal(syncWeaponFromSnapshot(10, 0, [0, 10]), 10)
  assert.equal(syncWeaponFromSnapshot(0, 10, [10]), 10)
  assert.equal(syncWeaponFromSnapshot(7, 0, [0, 10]), 0)
})

test('owned slots and nearby pickups drive the HUD highlights', () => {
  assert.deepEqual(
    ownedSlots({ slots: [{ kind: 'DesertEagles', ammo: 7 }, { kind: 'Ussocom', ammo: 12 }], active: 0 }),
    [0, 10],
  )
  const nearby = nearestPickup({ x: 10, y: 10 }, [
    { kind: 'DroppedWeapon', pos: { x: 12, y: 10 }, weapon_slot: 1, active: true },
    { kind: 'DroppedWeapon', pos: { x: 80, y: 10 }, weapon_slot: 4, active: true },
  ])
  assert.equal(nearby?.weapon_slot, 1)
  const standing = muzzleOrigin({ x: 0, y: 0 }, { x: 40, y: 0 }, 'standing')
  const prone = muzzleOrigin({ x: 0, y: 0 }, { x: 40, y: 0 }, 'prone')
  assert.ok(standing.y < prone.y)
})
