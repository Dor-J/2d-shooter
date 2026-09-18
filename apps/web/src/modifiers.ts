/**
 * The three modifiers and the community rulesets, as the browser and the HUD see them.
 *
 * The server owns which ones a room runs; these are the conversions the UI needs to say so.
 */

export type WireModifiers = { realistic: boolean; survival: boolean; advance: boolean }

export type RoomModifiers = {
  modifiers?: WireModifiers
  ruleset?: string | null
}

/** The eleven community rulesets a room can be opened with, in the order the wiki lists them. */
export const COMMUNITY_MODES = [
  'Climb',
  'Dodgeball',
  'Domination',
  'Hide and Seek',
  'Knife Only',
  'OneShots',
  'Pirates vs Ninjas',
  'RS/CS',
  'Trench Wars',
  'Tactical Trench Wars',
  'Zombie',
] as const

export const NO_MODIFIERS: WireModifiers = { realistic: false, survival: false, advance: false }

/** Whether any modifier is on, which is what decides whether a badge is shown at all. */
export function anyModifier(modifiers: WireModifiers | undefined): boolean {
  return Boolean(modifiers && (modifiers.realistic || modifiers.survival || modifiers.advance))
}

/** Short badges for the room browser, in a fixed order so the list does not jump about. */
export function modifierBadges(room: RoomModifiers): string[] {
  const badges: string[] = []
  if (room.ruleset) badges.push(room.ruleset)
  if (room.modifiers?.realistic) badges.push('Realistic')
  if (room.modifiers?.survival) badges.push('Survival')
  if (room.modifiers?.advance) badges.push('Advance')
  return badges
}

/** One line describing a room's rules, for the match header. */
export function modifierSummary(room: RoomModifiers): string {
  const badges = modifierBadges(room)
  return badges.length > 0 ? badges.join(' · ') : 'Standard rules'
}

/** Whether the HUD should drop back to its Realistic form. */
export function hudIsMinimal(modifiers: WireModifiers | undefined): boolean {
  return Boolean(modifiers?.realistic)
}

/**
 * What a dead player is told.
 *
 * In Survival there is nothing to count down to, so saying "respawn in 0" would be a lie; they are
 * out until the round ends.
 */
export function deathMessage(
  modifiers: WireModifiers | undefined,
  respawnIn: string | number,
): string {
  if (modifiers?.survival) return 'You are out for this round'
  const seconds = String(respawnIn ?? '')
  return seconds && seconds !== '0' ? `Respawn in ${seconds}` : 'Respawning'
}

/** How far along the Advance ladder a player is, for the weapon menu. */
export function advanceProgress(unlocked: number | undefined, primaries = 10): string {
  const bits = unlocked ?? 0
  let earned = 0
  for (let index = 0; index < primaries; index += 1) {
    if (bits & (1 << index)) earned += 1
  }
  return `${earned}/${primaries} weapons unlocked`
}

/** Whether a weapon slot is available to this player under Advance. */
export function isUnlocked(
  modifiers: WireModifiers | undefined,
  unlocked: number | undefined,
  slot: number,
  primaries = 10,
): boolean {
  if (!modifiers?.advance) return true
  if (slot >= primaries) return true
  return Boolean((unlocked ?? 0) & (1 << slot))
}
