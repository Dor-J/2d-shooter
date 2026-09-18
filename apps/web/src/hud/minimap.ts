// The minimap, the sniper line, and the crosshair — the three things that tell a player where
// something is rather than what it is.

import { teamColor, type Rgba } from './gauges.ts'
import type { PlacedElement } from './layout.ts'

export type Point = { x: number; y: number }

export type Bounds = { minX: number; minY: number; maxX: number; maxY: number }

/// The world rectangle a map occupies, from its polygons.
///
/// Derived rather than declared, because a map that grows a platform should not need its bounds
/// edited in a second place.
export function boundsOf(polygons: { vertices: Point[] }[]): Bounds {
  let minX = Infinity
  let minY = Infinity
  let maxX = -Infinity
  let maxY = -Infinity
  for (const polygon of polygons) {
    for (const vertex of polygon.vertices ?? []) {
      minX = Math.min(minX, vertex.x)
      minY = Math.min(minY, vertex.y)
      maxX = Math.max(maxX, vertex.x)
      maxY = Math.max(maxY, vertex.y)
    }
  }
  if (!Number.isFinite(minX)) return { minX: 0, minY: 0, maxX: 1, maxY: 1 }
  return { minX, minY, maxX, maxY }
}

/// Maps a world point into a placed minimap, keeping the map's own proportions.
///
/// The map is letterboxed inside the panel rather than stretched, because a stretched minimap lies
/// about which way is further.
export function projectToMinimap(point: Point, bounds: Bounds, panel: PlacedElement): Point {
  const worldWidth = Math.max(1e-6, bounds.maxX - bounds.minX)
  const worldHeight = Math.max(1e-6, bounds.maxY - bounds.minY)
  const scale = Math.min(panel.width / worldWidth, panel.height / worldHeight)
  const drawnWidth = worldWidth * scale
  const drawnHeight = worldHeight * scale
  const originX = panel.x + (panel.width - drawnWidth) / 2
  const originY = panel.y + (panel.height - drawnHeight) / 2
  return {
    x: originX + (point.x - bounds.minX) * scale,
    y: originY + (point.y - bounds.minY) * scale,
  }
}

export type Blip = { id: number; x: number; y: number; color: Rgba; radius: number; self: boolean }

export type MinimapPlayer = {
  id: number
  pos: Point
  team?: number
  hp: number
  /// Whether the local player is allowed to see this one at all.
  visible?: boolean
}

/// The blips on the minimap.
///
/// A player the mode has hidden — Realistic's limited sight, a spectator delay — is left out here
/// as well, because a minimap that shows what the screen does not is a wallhack with a nicer name.
export function blips(players: MinimapPlayer[], bounds: Bounds, panel: PlacedElement, self: number): Blip[] {
  const result: Blip[] = []
  for (const player of players) {
    if (player.hp <= 0) continue
    if (player.visible === false && player.id !== self) continue
    const point = projectToMinimap(player.pos, bounds, panel)
    const own = player.id === self
    result.push({
      id: player.id,
      x: point.x,
      y: point.y,
      color: teamColor(player.team),
      radius: own ? 3 : 2,
      self: own,
    })
  }
  return result
}

/// Whether a world point is inside the drawn map at all, so a blip outside it is not clamped onto
/// the edge and read as a player standing there.
export function withinBounds(point: Point, bounds: Bounds): boolean {
  return (
    point.x >= bounds.minX && point.x <= bounds.maxX && point.y >= bounds.minY && point.y <= bounds.maxY
  )
}

export type SniperLine = { from: Point; to: Point; color: Rgba; width: number }

/// The line a scoped rifle draws from the muzzle to where it is pointed.
///
/// It is drawn only for weapons that have it, and only for the player holding one — it is an aid,
/// not a spectator effect, and a second player's line would be information they did not earn.
export function sniperLine(
  from: Point,
  aim: Point,
  { enabled, range = 1200 }: { enabled: boolean; range?: number },
): SniperLine | null {
  if (!enabled) return null
  const dx = aim.x - from.x
  const dy = aim.y - from.y
  const length = Math.hypot(dx, dy)
  if (length < 1e-6) return null
  // The line runs from the muzzle through the cursor and out to the weapon’s reach, so it shows
  // where the round goes rather than stopping short at the cursor.
  const reach = range
  return {
    from,
    to: { x: from.x + (dx / length) * reach, y: from.y + (dy / length) * reach },
    color: [0.95, 0.3, 0.28, 0.35],
    width: 1,
  }
}

export type Crosshair = {
  /// How far the four marks sit from the centre, in world units.
  spread: number
  /// How far the whole cursor is lifted by recoil.
  offsetY: number
  color: Rgba
  /// Whether the weapon can fire this instant.
  ready: boolean
}

const MIN_SPREAD = 3
const SPREAD_PER_UNIT = 220

/// The crosshair, opened up by the weapon's own inaccuracy and by bink from being shot.
///
/// The number it draws is the same accuracy the server used for the shot, so a wide crosshair is a
/// promise the game keeps rather than a decoration.
export function crosshair(accuracy: number, bink: number, cooling: boolean): Crosshair {
  const spread = MIN_SPREAD + Math.max(0, accuracy) * SPREAD_PER_UNIT + Math.max(0, bink) * 0.6
  return {
    spread,
    offsetY: -Math.max(0, bink) * 0.25,
    color: cooling ? [1, 0.82, 0.38, 0.45] : [1, 0.82, 0.38, 0.9],
    ready: !cooling,
  }
}
