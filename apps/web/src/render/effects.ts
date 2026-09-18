// What the whole screen does when something big happens: bullet time, the bonus overlays, the
// shake of an explosion, and the red edges of being shot.

import type { Rgba } from './gostek.ts'

export type Vec2 = { x: number; y: number }

/// Bullet time slows the picture, not the simulation.
///
/// The server still runs at sixty ticks a second; only the rate at which the client walks towards
/// the next snapshot changes, so a slowed client is never a client that has fallen behind.
export const BULLET_TIME_SCALE = 0.35

export function bulletTimeScale(active: boolean): number {
  return active ? BULLET_TIME_SCALE : 1
}

/// The desaturation bullet time washes the picture with, from 0 to 1.
export function bulletTimeWash(active: boolean, sinceSeconds: number): number {
  if (!active) return 0
  return Math.min(1, Math.max(0, sinceSeconds / 0.25))
}

export type Overlay = {
  /// A full-screen tint.
  tint: Rgba
  /// How transparent the player themselves is drawn, from 0 (invisible) to 1 (solid).
  playerAlpha: number
  /// A name for the effect, for a HUD line and for a test.
  name: string
}

/// Predator's transparency, matching the upstream alpha rather than a guess.
export const PREDATOR_ALPHA = 0.32

/// What a bonus does to the picture.
export function bonusOverlay(effect: string | null | undefined, own: boolean): Overlay {
  switch (effect) {
    case 'Predator':
      return {
        tint: [0.4, 0.78, 0.66, own ? 0.06 : 0],
        // Your own predator is drawn faintly so you can still see yourself; an enemy's is fainter
        // still, but never invisible — a player must have something to shoot at.
        playerAlpha: own ? 0.55 : PREDATOR_ALPHA,
        name: 'Predator',
      }
    case 'Berserker':
      return { tint: [0.86, 0.22, 0.5, own ? 0.14 : 0], playerAlpha: 1, name: 'Berserker' }
    case 'FlameGod':
      return { tint: [0.95, 0.5, 0.16, own ? 0.16 : 0], playerAlpha: 1, name: 'Flame God' }
    default:
      return { tint: [0, 0, 0, 0], playerAlpha: 1, name: '' }
  }
}

/// A shake, which decays on its own so nothing has to remember to stop it.
export class ScreenShake {
  magnitude = 0
  elapsed = 0
  seed = 1

  /// Adds a knock. The strongest one wins rather than the sum, so a cluster of grenades rattles the
  /// screen instead of throwing it off the monitor.
  add(magnitude: number, seed = 1) {
    if (!Number.isFinite(magnitude) || magnitude <= 0) return
    if (magnitude > this.magnitude) {
      this.magnitude = Math.min(24, magnitude)
      this.elapsed = 0
      this.seed = seed
    }
  }

  step(dt: number) {
    if (!Number.isFinite(dt) || dt <= 0) return
    this.elapsed += dt
    this.magnitude = Math.max(0, this.magnitude - dt * 36)
  }

  /// The camera offset this instant.
  offset(): Vec2 {
    if (this.magnitude <= 0) return { x: 0, y: 0 }
    const t = this.elapsed * 47 + this.seed
    return {
      x: Math.sin(t) * this.magnitude,
      y: Math.cos(t * 1.37) * this.magnitude * 0.6,
    }
  }
}

/// How hard an explosion shakes the screen from where the player is standing.
export function blastShake(distance: number, radius: number, power = 14): number {
  if (!Number.isFinite(distance) || !Number.isFinite(radius) || radius <= 0) return 0
  if (distance >= radius) return 0
  return power * (1 - distance / radius)
}

export type DamageFeedback = {
  /// How strongly the edges bleed, 0 to 1.
  vignette: number
  /// Where the hit came from, in radians, or null when it came from nowhere in particular.
  arrow: number | null
  alpha: number
}

/// The red edges and the direction arrow after a hit.
export function damageFeedback(
  vignette: number,
  direction: Vec2 | null | undefined,
  sinceSeconds: number,
): DamageFeedback {
  const alpha = Math.max(0, 1 - Math.max(0, sinceSeconds) / 1.2)
  const hasDirection =
    direction !== null &&
    direction !== undefined &&
    (Math.abs(direction.x) > 1e-6 || Math.abs(direction.y) > 1e-6)
  return {
    vignette: Math.min(1, Math.max(0, vignette)),
    arrow: hasDirection && alpha > 0 ? Math.atan2(direction.y, direction.x) : null,
    alpha,
  }
}

/// How much of the picture an explosion lights up, as a flash that fades over a few frames.
export function explosionFlash(ageSeconds: number, duration = 0.18): number {
  if (ageSeconds < 0 || ageSeconds >= duration) return 0
  return 1 - ageSeconds / duration
}
