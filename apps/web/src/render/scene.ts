// One frame, described rather than drawn.
//
// The renderer turns this list into draw calls and a test turns the same list into pixels, which
// is what makes a screenshot comparison mean anything: both are looking at the same frame.

import { gaugeFillRect, healthGauge, jetGauge, ammoGauge, teamColor, type Rgba } from '../hud/gauges.ts'
import { placeLayout, type HudLayout, type Viewport } from '../hud/layout.ts'
import { buildRig, type Appearance, type Pose } from './gostek.ts'
import { bonusOverlay } from './effects.ts'

export type Draw = {
  /// What produced this rectangle, so a failing comparison can name it.
  id: string
  x: number
  y: number
  width: number
  height: number
  color: Rgba
  /// Centred rectangles are the world's; corner-anchored ones are the HUD's.
  centred: boolean
}

export type SceneSoldier = {
  id: number
  pos: { x: number; y: number }
  aim: { x: number; y: number }
  team: number
  hp: number
  pose: Pose
  appearance: Appearance
  jetting: boolean
  moving: boolean
  bonus?: string | null
}

export type SceneInput = {
  layout: HudLayout
  viewport: Viewport
  soldiers: SceneSoldier[]
  local: number
  health: number
  fuel: number
  fuelCapacity: number
  ammo: { ammo: number; magazine: number; reloadTimer: number; reloadTicks: number }
  tick: number
}

/// Everything drawn this frame, world first and HUD on top.
export function buildScene(input: SceneInput): Draw[] {
  const draws: Draw[] = []

  for (const soldier of input.soldiers) {
    if (soldier.hp <= 0) continue
    const overlay = bonusOverlay(soldier.bonus, soldier.id === input.local)
    const rig = buildRig({
      pos: soldier.pos,
      aim: soldier.aim,
      pose: soldier.pose,
      appearance: soldier.appearance,
      team: teamColor(soldier.team),
      tick: input.tick,
      jetting: soldier.jetting,
      moving: soldier.moving,
    })
    for (const part of rig) {
      draws.push({
        id: `soldier-${soldier.id}-${part.part}`,
        x: part.x,
        y: part.y,
        width: part.width,
        height: part.height,
        color: [part.color[0], part.color[1], part.color[2], part.color[3] * overlay.playerAlpha],
        centred: true,
      })
    }
  }

  const placed = placeLayout(input.layout, input.viewport)
  const gauges = {
    health: healthGauge(input.health),
    jet: jetGauge(input.fuel, input.fuelCapacity),
    ammo: ammoGauge(input.ammo),
  }
  for (const element of placed) {
    draws.push({
      id: `hud-${element.id}-track`,
      x: element.x,
      y: element.y,
      width: element.width,
      height: element.height,
      color: [0.04, 0.07, 0.09, 0.85],
      centred: false,
    })
    const gauge = gauges[element.id as keyof typeof gauges]
    if (!gauge) continue
    const fill = gaugeFillRect(gauge, element)
    draws.push({
      id: `hud-${element.id}-fill`,
      x: fill.x,
      y: fill.y,
      width: fill.width,
      height: fill.height,
      color: gauge.color,
      centred: false,
    })
  }

  return draws
}
