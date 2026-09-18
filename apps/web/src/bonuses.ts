/**
 * Reading the authoritative bonus state for the HUD.
 *
 * The server owns every effect: which one is running, how long is left, and how visible a Predator
 * is. These are the conversions the HUD needs to draw it.
 */

export type BonusEffectName = 'FlameGod' | 'Berserker' | 'Predator'
export type KitName =
  | 'Medic'
  | 'Grenades'
  | 'ClusterGrenades'
  | 'Vest'
  | 'FlameGod'
  | 'Berserker'
  | 'Predator'

export type WireTimedEffect = { active: BonusEffectName | null; ticks_left: number }

const TICK_RATE = 60

/** What players call each effect. */
export function effectName(effect: BonusEffectName): string {
  if (effect === 'FlameGod') return 'Flame God'
  return effect
}

/** What players call each kit. */
export function kitName(kit: KitName): string {
  if (kit === 'ClusterGrenades') return 'Cluster Grenades'
  if (kit === 'FlameGod') return 'Flame God'
  if (kit === 'Medic') return 'Medical Kit'
  if (kit === 'Grenades') return 'Grenades Kit'
  if (kit === 'Vest') return 'Bulletproof Vest'
  return kit
}

/** Whether anything is running, which is what decides if the bonus panel is shown at all. */
export function hasBonus(effect: WireTimedEffect | undefined): boolean {
  return Boolean(effect?.active)
}

/** Seconds left, rounded up, so the countdown never shows zero while it is still running. */
export function secondsLeft(effect: WireTimedEffect | undefined): number {
  if (!effect?.active) return 0
  return Math.ceil(Math.max(0, effect.ticks_left) / TICK_RATE)
}

/** The line the bonus panel shows. */
export function bonusText(effect: WireTimedEffect | undefined): string {
  if (!effect?.active) return ''
  return `${effectName(effect.active)} · ${secondsLeft(effect)}s`
}

/** The screen overlay tint for the running effect, or none. */
export function bonusOverlay(effect: WireTimedEffect | undefined): string {
  if (!effect?.active) return ''
  if (effect.active === 'FlameGod') return 'flame-god'
  if (effect.active === 'Berserker') return 'berserker'
  return 'predator'
}

/**
 * How visible a player should be drawn, 0 to 1.
 *
 * A Predator is nearly transparent, and blood gives a wounded one away — which is the counterplay
 * the whole kit is built around.
 */
export function playerOpacity(
  effect: WireTimedEffect | undefined,
  hp: number,
  maxHp = 100,
): number {
  if (effect?.active !== 'Predator') return 1
  const base = 5 / 255
  if (hp > maxHp / 2) return base
  const missing = Math.max(0, maxHp - hp) / Math.max(1, maxHp)
  return Math.min(1, base + missing * (90 / 255))
}
