// Everything on the map that is not a soldier: the things they carry, drop, throw, and stand on.
//
// Each object is described as a sprite id, a size, and a tint, so the renderer looks up one atlas
// entry rather than carrying a branch per object kind.

import type { Rgba } from './gostek.ts'

export type Vec2 = { x: number; y: number }

export type ObjectSprite = {
  sprite: string
  x: number
  y: number
  width: number
  height: number
  angle: number
  color: Rgba
  /// Drawn low to high, so a flag on the ground never hides the soldier taking it.
  layer: number
}

const KIT_COLORS: Record<string, Rgba> = {
  Medical: [0.9, 0.24, 0.26, 1],
  Grenade: [0.42, 0.48, 0.3, 1],
  Flamer: [0.95, 0.5, 0.16, 1],
  Predator: [0.4, 0.78, 0.66, 1],
  Vest: [0.62, 0.72, 0.86, 1],
  Berserker: [0.86, 0.22, 0.5, 1],
  Cluster: [0.78, 0.66, 0.3, 1],
}

export const KIT_KINDS = Object.keys(KIT_COLORS)

/// A bonus kit lying on the ground.
export function kitSprite(kind: string, pos: Vec2, bobTick = 0): ObjectSprite {
  const color = KIT_COLORS[kind] ?? [0.7, 0.7, 0.7, 1]
  // A gentle bob is the difference between a kit and a piece of scenery.
  const bob = Math.sin((bobTick % 120) / 120 * Math.PI * 2) * 1.5
  return {
    sprite: `kit-${kind.toLowerCase()}`,
    x: pos.x,
    y: pos.y + bob,
    width: 16,
    height: 14,
    angle: 0,
    color,
    layer: 1,
  }
}

export const FLAG_COLORS: Record<string, Rgba> = {
  Alpha: [0.64, 0.79, 1, 1],
  Bravo: [1, 0.66, 0.62, 1],
  Pointmatch: [0.95, 0.85, 0.35, 1],
}

/// A flag, on its base, in somebody's hands, or on the ground.
///
/// A carried flag is drawn behind the carrier's shoulder rather than on top of them, so a firefight
/// is still readable when somebody runs through it with the objective.
export function flagSprite(
  kind: string,
  pos: Vec2,
  state: 'base' | 'carried' | 'dropped',
  tick = 0,
): ObjectSprite {
  const wave = Math.sin(tick / 9) * 2
  return {
    sprite: `flag-${kind.toLowerCase()}`,
    x: pos.x + (state === 'carried' ? -4 : 0),
    y: pos.y - (state === 'base' ? 12 : 4) + (state === 'base' ? wave * 0.4 : 0),
    width: 18,
    height: 12,
    angle: state === 'dropped' ? Math.PI / 12 : 0,
    color: FLAG_COLORS[kind] ?? [0.8, 0.8, 0.8, 1],
    layer: state === 'carried' ? 0 : 1,
  }
}

/// A weapon lying where somebody dropped it.
export function droppedWeaponSprite(name: string, pos: Vec2): ObjectSprite {
  return {
    sprite: `weapon-${name.toLowerCase().replace(/[^a-z0-9]+/g, '-')}`,
    x: pos.x,
    y: pos.y,
    width: 22,
    height: 6,
    angle: Math.PI / 16,
    color: [0.72, 0.55, 0.22, 1],
    layer: 1,
  }
}

/// A grenade or an arrow in the air. Both spin, which is how a player judges where one will land.
export function projectileSprite(
  kind: 'grenade' | 'cluster' | 'arrow' | 'bullet' | 'rocket',
  pos: Vec2,
  velocity: Vec2,
  tick = 0,
): ObjectSprite {
  const heading = Math.atan2(velocity.y, velocity.x)
  const spinning = kind === 'grenade' || kind === 'cluster'
  const sizes: Record<string, { width: number; height: number; color: Rgba }> = {
    grenade: { width: 8, height: 8, color: [0.34, 0.4, 0.28, 1] },
    cluster: { width: 7, height: 7, color: [0.55, 0.45, 0.24, 1] },
    arrow: { width: 18, height: 3, color: [0.76, 0.66, 0.44, 1] },
    bullet: { width: 6, height: 2, color: [1, 0.9, 0.55, 1] },
    rocket: { width: 14, height: 5, color: [0.85, 0.4, 0.2, 1] },
  }
  const size = sizes[kind]
  return {
    sprite: `projectile-${kind}`,
    x: pos.x,
    y: pos.y,
    width: size.width,
    height: size.height,
    angle: spinning ? (tick / 4) % (Math.PI * 2) : heading,
    color: size.color,
    layer: 2,
  }
}

/// The stationary gun bolted to a map.
export function stationaryGunSprite(pos: Vec2, angle: number, manned: boolean): ObjectSprite {
  return {
    sprite: 'stationary-gun',
    x: pos.x,
    y: pos.y,
    width: 30,
    height: 10,
    angle,
    color: manned ? [0.68, 0.62, 0.5, 1] : [0.46, 0.46, 0.44, 1],
    layer: 1,
  }
}

/// A bullet's trail, which is drawn as a line behind where it is now.
///
/// The trail is derived from the round's own velocity rather than remembered between frames, so a
/// snapshot that skips does not leave a streak hanging in the air.
export function bulletTrail(pos: Vec2, velocity: Vec2, lengthTicks = 2) {
  return {
    from: { x: pos.x - velocity.x * lengthTicks * (1 / 60), y: pos.y - velocity.y * lengthTicks * (1 / 60) },
    to: { x: pos.x, y: pos.y },
    color: [1, 0.86, 0.5, 0.45] as Rgba,
    width: 1.5,
  }
}
