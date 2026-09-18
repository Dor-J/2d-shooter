// The bars along the bottom of the screen, as numbers rather than pixels.
//
// Each gauge answers two questions — how full is it, and what colour is it — so the same
// description drives the WebGL HUD, the DOM overlay, and a test that never opens a window.

import type { PlacedElement } from './layout.ts'

export type Rgba = [number, number, number, number]

/// A bar ready to be drawn: its track, the filled part of it, and the colour of the fill.
export type Gauge = {
  id: string
  /// 0 to 1.
  fill: number
  color: Rgba
  track: Rgba
  /// What a screen reader, or a player who turned numbers on, should be told.
  label: string
}

const TRACK: Rgba = [0.04, 0.07, 0.09, 0.85]

/// Health is green until it is worth worrying about and red when it is.
export const HEALTH_FULL: Rgba = [0.24, 0.78, 0.36, 1]
export const HEALTH_HURT: Rgba = [0.92, 0.72, 0.24, 1]
export const HEALTH_CRITICAL: Rgba = [0.92, 0.22, 0.2, 1]
/// Below this share of full health the bar goes red and the edges of the screen bleed.
export const CRITICAL_HEALTH = 0.3
const HURT_HEALTH = 0.6

export const AMMO_COLOR: Rgba = [0.95, 0.83, 0.3, 1]
export const RELOAD_COLOR: Rgba = [0.95, 0.55, 0.18, 1]
export const FIRE_INTERVAL_COLOR: Rgba = [0.78, 0.78, 0.82, 1]
export const JET_COLOR: Rgba = [0.32, 0.69, 1, 1]
export const JET_EMPTY_COLOR: Rgba = [0.4, 0.45, 0.5, 1]
export const ARMOR_COLOR: Rgba = [0.62, 0.72, 0.86, 1]

function ratio(value: number, of: number): number {
  if (!Number.isFinite(value) || !Number.isFinite(of) || of <= 0) return 0
  return Math.min(1, Math.max(0, value / of))
}

/// The red health bar.
export function healthGauge(hp: number, max = 100): Gauge {
  const fill = ratio(hp, max)
  const color = fill <= CRITICAL_HEALTH ? HEALTH_CRITICAL : fill <= HURT_HEALTH ? HEALTH_HURT : HEALTH_FULL
  return { id: 'health', fill, color, track: TRACK, label: `${Math.max(0, Math.round(hp))} health` }
}

/// The blue jet-fuel bar. An empty tank is greyed rather than hidden, so a player can see why they
/// are not going up.
export function jetGauge(fuel: number, capacity: number): Gauge {
  const fill = ratio(fuel, capacity)
  return {
    id: 'jet',
    fill,
    color: fill <= 0.001 ? JET_EMPTY_COLOR : JET_COLOR,
    track: TRACK,
    label: `${Math.round(fill * 100)}% jet fuel`,
  }
}

export type AmmoState = {
  ammo: number
  magazine: number
  /// Ticks left of a reload in progress, and how long that reload takes.
  reloadTimer: number
  reloadTicks: number
}

/// The yellow ammunition bar, which becomes the reload bar while reloading.
///
/// Reloading shows progress rather than the word RELOADING, because a player deciding whether to
/// break cover needs to know how much longer, not merely that it is happening.
export function ammoGauge(state: AmmoState): Gauge {
  if (state.reloadTimer > 0 && state.reloadTicks > 0) {
    const done = 1 - ratio(state.reloadTimer, state.reloadTicks)
    return {
      id: 'ammo',
      fill: done,
      color: RELOAD_COLOR,
      track: TRACK,
      label: `reloading ${Math.round(done * 100)}%`,
    }
  }
  const fill = ratio(state.ammo, state.magazine)
  return {
    id: 'ammo',
    fill,
    color: AMMO_COLOR,
    track: TRACK,
    label: `${Math.max(0, Math.trunc(state.ammo))} of ${Math.max(0, Math.trunc(state.magazine))} rounds`,
  }
}

/// The bullet count that sits with the ammo bar. An empty magazine is worth saying out loud.
export function bulletCountText(state: AmmoState): string {
  if (state.reloadTimer > 0) return '— / —'
  const ammo = Math.max(0, Math.trunc(state.ammo))
  return ammo === 0 ? 'EMPTY' : `${ammo} / ${Math.max(0, Math.trunc(state.magazine))}`
}

/// The thin bar that fills between shots, so a player can time a burst.
export function fireIntervalGauge(cooldown: number, interval: number): Gauge {
  const remaining = ratio(cooldown, interval)
  return {
    id: 'fire-interval',
    fill: 1 - remaining,
    color: FIRE_INTERVAL_COLOR,
    track: TRACK,
    label: remaining > 0 ? 'weapon cycling' : 'ready',
  }
}

/// Armor is shown only when a player has some, because an empty vest slot is not information.
export function armorGauge(armor: number, max = 100): Gauge | null {
  if (!Number.isFinite(armor) || armor <= 0) return null
  return {
    id: 'armor',
    fill: ratio(armor, max),
    color: ARMOR_COLOR,
    track: TRACK,
    label: `${Math.round(armor)} armor`,
  }
}

/// The filled rectangle of a gauge inside its placed element, in screen pixels.
export function gaugeFillRect(gauge: Gauge, placed: PlacedElement) {
  const inset = Math.min(2, placed.height / 6)
  return {
    x: placed.x + inset,
    y: placed.y + inset,
    width: Math.max(0, (placed.width - inset * 2) * gauge.fill),
    height: Math.max(0, placed.height - inset * 2),
  }
}

/// The four teams, in the colours the original used.
export const TEAM_COLORS: Record<number, Rgba> = {
  0: [1, 0.95, 0.83, 1],
  1: [0.64, 0.79, 1, 1],
  2: [1, 0.66, 0.62, 1],
  3: [0.72, 0.92, 0.66, 1],
  4: [0.98, 0.88, 0.58, 1],
}

export const TEAM_NAMES: Record<number, string> = {
  0: 'No team',
  1: 'Alpha',
  2: 'Bravo',
  3: 'Charlie',
  4: 'Delta',
}

export function teamColor(team: number | undefined): Rgba {
  return TEAM_COLORS[team ?? 0] ?? TEAM_COLORS[0]
}

export function teamName(team: number | undefined): string {
  return TEAM_NAMES[team ?? 0] ?? `Team ${team}`
}

/// How a player is tinted: their own team's colour, brightened when it is you.
export function playerTint(team: number | undefined, isSelf: boolean): Rgba {
  const color = teamColor(team)
  if (!isSelf) return color
  return [
    Math.min(1, color[0] * 1.12),
    Math.min(1, color[1] * 1.12),
    Math.min(1, color[2] * 1.12),
    color[3],
  ]
}

/// How strongly the screen edges bleed red, from 0 to 1.
///
/// Damage feedback fades with the hit and then holds at a floor while health is critical, so a
/// player who is one shot from dead never stops being told.
export function damageVignette(hp: number, sinceHitSeconds: number, max = 100): number {
  const health = ratio(hp, max)
  const flash = Math.max(0, 1 - Math.max(0, sinceHitSeconds) / 0.6)
  const critical = health <= CRITICAL_HEALTH && hp > 0 ? (CRITICAL_HEALTH - health) / CRITICAL_HEALTH : 0
  return Math.min(1, Math.max(flash * 0.7, critical * 0.55))
}
