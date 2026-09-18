// The map itself: textured polygons, the edges around them, and the scenery in front of and behind
// the fight.

import type { Rgba } from './gostek.ts'

export type Vec2 = { x: number; y: number }

export type MapPolygon = {
  vertices: Vec2[]
  kind: string
  /// The texture this polygon is painted with, when the map names one.
  texture?: string
}

/// Every polygon kind a map can carry, and the colour it falls back to when its texture is missing.
export const POLYGON_COLORS: Record<string, Rgba> = {
  Normal: [0.3, 0.34, 0.35, 1],
  Ice: [0.38, 0.72, 0.86, 1],
  Bouncy: [0.82, 0.54, 0.28, 1],
  Deadly: [0.78, 0.18, 0.16, 1],
  OneWay: [0.55, 0.57, 0.5, 1],
  Hurts: [0.7, 0.3, 0.24, 1],
  Regenerates: [0.32, 0.66, 0.42, 1],
  Background: [0.2, 0.22, 0.26, 1],
}

export function polygonColor(kind: string): Rgba {
  return POLYGON_COLORS[kind] ?? POLYGON_COLORS.Normal
}

/// Texture coordinates for one polygon, in texture space.
///
/// The texture is projected from world coordinates rather than stretched to each triangle, so two
/// polygons that meet show one continuous surface instead of a seam.
export function textureCoords(polygon: MapPolygon, scale = 128): { u: number; v: number }[] {
  return polygon.vertices.map(vertex => ({ u: vertex.x / scale, v: vertex.y / scale }))
}

export type Edge = { from: Vec2; to: Vec2; color: Rgba; width: number }

/// The lit edges of a polygon.
///
/// Only the edges that face the light are drawn, so a platform reads as solid ground with a top
/// rather than as an outlined shape.
export function edgesOf(polygon: MapPolygon, light: Vec2 = { x: 0, y: -1 }): Edge[] {
  const edges: Edge[] = []
  const vertices = polygon.vertices
  for (let i = 0; i < vertices.length; i += 1) {
    const from = vertices[i]
    const to = vertices[(i + 1) % vertices.length]
    const dx = to.x - from.x
    const dy = to.y - from.y
    const length = Math.hypot(dx, dy)
    if (length < 1e-6) continue
    // The outward normal of an edge, for a polygon wound clockwise in screen space.
    const nx = dy / length
    const ny = -dx / length
    const lit = nx * light.x + ny * light.y
    if (lit <= 0) continue
    const base = polygonColor(polygon.kind)
    edges.push({
      from,
      to,
      color: [
        Math.min(1, base[0] + 0.18 * lit),
        Math.min(1, base[1] + 0.18 * lit),
        Math.min(1, base[2] + 0.18 * lit),
        1,
      ],
      width: 2,
    })
  }
  return edges
}

export type Scenery = {
  sprite: string
  x: number
  y: number
  width: number
  height: number
  angle: number
  /// Less than 1 moves slower than the camera and reads as further away; more than 1 is foreground.
  parallax: number
  /// Foreground scenery is drawn over the soldiers, background behind them.
  foreground: boolean
  alpha: number
}

/// Where a piece of scenery lands on screen for a given camera.
///
/// Parallax is applied to the camera rather than to the object, so a distant tree stays in the same
/// place on the map and merely moves less — which is what keeps two players' screens agreeing.
export function parallaxPosition(scenery: Scenery, camera: Vec2): Vec2 {
  return {
    x: scenery.x - camera.x * scenery.parallax,
    y: scenery.y - camera.y * scenery.parallax,
  }
}

/// Scenery split into what is drawn before the soldiers and what is drawn after.
export function sceneryLayers(scenery: Scenery[]) {
  const background = scenery.filter(piece => !piece.foreground).sort((a, b) => a.parallax - b.parallax)
  const foreground = scenery.filter(piece => piece.foreground).sort((a, b) => a.parallax - b.parallax)
  return { background, foreground }
}

/// Whether a piece of scenery is worth drawing at all for this view.
export function sceneryVisible(
  scenery: Scenery,
  camera: Vec2,
  view: { width: number; height: number },
): boolean {
  const at = parallaxPosition(scenery, camera)
  const margin = Math.max(scenery.width, scenery.height)
  return (
    at.x + scenery.width / 2 >= -margin &&
    at.x - scenery.width / 2 <= view.width + margin &&
    at.y + scenery.height / 2 >= -margin &&
    at.y - scenery.height / 2 <= view.height + margin
  )
}
