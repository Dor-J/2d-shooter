// HUD and hash checks read the snapshot table. Known default checksums come from
// tests/fixtures/reference/weapons.json; there is no second stats sheet here.

export const DEFAULT_WEAPON_NAMES = [
  'Desert Eagles',
  'HK MP5',
  'Ak-74',
  'Steyr AUG',
  'Spas-12',
  'Ruger 77',
  'M79',
  'Barrett M82A1',
  'FN Minimi',
  'XM214 Minigun',
  'USSOCOM',
  'Combat Knife',
  'Chainsaw',
  'LAW',
] as const

export const KNOWN_WEAPON_HASHES: Record<number, string> = {
  911431262: 'Default mod',
  2707142329: 'Realistic mod',
}

export type WireWeaponTable = {
  name?: string
  version?: string
  hash?: number
  names?: string[]
  defs?: unknown[]
}

export type AcceptedWeaponTable = {
  ok: true
  name: string
  hash: number
  names: string[]
}

export type RejectedWeaponTable = {
  ok: false
  reason: 'missing' | 'hash_mismatch'
}

export function acceptWeaponTable(
  wire: WireWeaponTable | null | undefined,
  listedHash?: number,
): AcceptedWeaponTable | RejectedWeaponTable {
  if (!wire || typeof wire.hash !== 'number' || !Number.isFinite(wire.hash)) {
    return { ok: false, reason: 'missing' }
  }
  if (listedHash !== undefined && listedHash !== wire.hash) {
    return { ok: false, reason: 'hash_mismatch' }
  }
  const names =
    Array.isArray(wire.names) && wire.names.length === DEFAULT_WEAPON_NAMES.length
      ? wire.names
      : [...DEFAULT_WEAPON_NAMES]
  const known = KNOWN_WEAPON_HASHES[wire.hash]
  if (known) {
    return { ok: true, name: wire.name || known, hash: wire.hash, names }
  }
  if (Array.isArray(wire.defs) && wire.defs.length > 0) {
    return { ok: true, name: wire.name || 'Custom', hash: wire.hash, names }
  }
  return { ok: false, reason: 'hash_mismatch' }
}

/** Crosshair size and recoil lift from the authoritative accuracy/recoil fields. */
export const WEAPON_KIND_SLOT: Record<string, number> = {
  DesertEagles: 0,
  Mp5: 1,
  Ak74: 2,
  SteyrAug: 3,
  Spas12: 4,
  Ruger77: 5,
  M79: 6,
  Barrett: 7,
  Minimi: 8,
  Minigun: 9,
  Ussocom: 10,
  CombatKnife: 11,
  Chainsaw: 12,
  Law: 13,
}

export type WireWeaponSlot = { kind?: string; ammo?: number }
export type WireInventory = { slots?: Array<WireWeaponSlot | null>; active?: number }
export type GroundWeapon = { kind?: string; pos: { x: number; y: number }; weapon_slot?: number | null; active?: boolean }

/** Keep a requested switch; only snap to the server gun after a drop or an illegal pick. */
export function syncWeaponFromSnapshot(selected: number, serverWeapon: number, owned: number[]): number {
  if (owned.includes(selected)) return selected
  if (owned.includes(serverWeapon)) return serverWeapon
  return selected
}

export function ownedSlots(inventory?: WireInventory | null): number[] {
  const slots: number[] = []
  for (const slot of inventory?.slots ?? []) {
    if (!slot?.kind) continue
    const index = WEAPON_KIND_SLOT[slot.kind]
    if (index !== undefined) slots.push(index)
  }
  return slots
}

export function nearestPickup(
  pos: { x: number; y: number },
  objects: GroundWeapon[] | undefined,
  radius = 28,
) {
  let best: { object: GroundWeapon; dist: number } | null = null
  for (const object of objects ?? []) {
    if (object.active === false || object.weapon_slot == null) continue
    const dist = Math.hypot(object.pos.x - pos.x, object.pos.y - pos.y)
    if (dist <= radius && (!best || dist < best.dist)) best = { object, dist }
  }
  return best?.object ?? null
}

export function muzzleOrigin(
  pos: { x: number; y: number },
  aim: { x: number; y: number },
  pose = 'standing',
) {
  const height = pose === 'crouching' ? 6 : pose === 'prone' || pose === 'going_prone' || pose === 'getting_up' ? 2 : pose === 'rolling' || pose === 'backflip' ? 4 : 10
  const dx = aim.x - pos.x
  const dy = aim.y - pos.y
  const len = Math.hypot(dx, dy)
  const dirX = len > 1 ? dx / len : 1
  const dirY = len > 1 ? dy / len : 0
  return { x: pos.x + dirX * 14, y: pos.y - height + dirY * 6 }
}

export function cursorAppearance(accuracy = 0, recoil = 0) {
  const spread = Math.min(0.5, Math.max(0, accuracy))
  return {
    radius: 6 + spread * 48 + Math.abs(recoil) * 12,
    offsetY: -recoil * 40,
  }
}

/** What to tell the player when the server refuses their shot, so it never looks like lag. */
export function fireRefusalText(reason: string): string {
  if (reason === 'NeedsBracing') return 'The LAW has to be braced — crouch or go prone to fire it.'
  return 'That shot could not be taken from here.'
}
