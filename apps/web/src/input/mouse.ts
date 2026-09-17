import type { Action } from './actions.ts'
import { type BindingSet, actionsForCode } from './bindings.ts'
import type { ActionState } from './state.ts'
import type { View } from '../mobile.ts'

export type Vec2 = { x: number; y: number }
export type PointerLikeEvent = { button?: number; pointerType?: string }
export type WheelLikeEvent = { deltaY: number; preventDefault?: () => void }

export const MIN_SENSITIVITY = 0.1
export const MAX_SENSITIVITY = 5
export const SENSITIVITY_STEP = 0.25

export function clampSensitivity(value: number): number {
  if (!Number.isFinite(value)) return 1
  return Math.min(MAX_SENSITIVITY, Math.max(MIN_SENSITIVITY, Math.round(value * 100) / 100))
}

/** Absolute pointer aiming: the cursor is the crosshair, so sensitivity does not scale it. */
export function aimFromPointer(
  clientX: number,
  clientY: number,
  width: number,
  height: number,
  view: View,
): Vec2 {
  return {
    x: view.x + (clientX / Math.max(1, width)) * view.width,
    y: view.y + (clientY / Math.max(1, height)) * view.height,
  }
}

export class MouseDevice {
  bindings: BindingSet
  sensitivity = 1
  #state: ActionState
  #suspended = false
  #down = new Set<string>()

  constructor(state: ActionState, bindings: BindingSet, sensitivity = 1) {
    this.#state = state
    this.bindings = bindings
    this.sensitivity = clampSensitivity(sensitivity)
  }

  get suspended(): boolean {
    return this.#suspended
  }

  suspend() {
    if (this.#suspended) return
    this.#suspended = true
    this.releaseAll()
  }

  resume() {
    this.#suspended = false
  }

  releaseAll() {
    for (const code of this.#down) {
      for (const action of actionsForCode(this.bindings, `mouse:${code}`)) this.#state.set(action, false)
    }
    this.#down.clear()
  }

  /** Runtime sensitivity change; returns the value actually applied after clamping. */
  adjustSensitivity(delta: number): number {
    this.sensitivity = clampSensitivity(this.sensitivity + delta)
    return this.sensitivity
  }

  setSensitivity(value: number): number {
    this.sensitivity = clampSensitivity(value)
    return this.sensitivity
  }

  /** Relative aiming (pointer lock, trackpad, gamepad-style) is what sensitivity scales. */
  aimFromLockedMovement(aim: Vec2, movementX: number, movementY: number, view: View): Vec2 {
    return {
      x: Math.min(view.x + view.width, Math.max(view.x, aim.x + movementX * this.sensitivity)),
      y: Math.min(view.y + view.height, Math.max(view.y, aim.y + movementY * this.sensitivity)),
    }
  }

  #code(event: PointerLikeEvent): string {
    return `Mouse${event.button ?? 0}`
  }

  pointerdown(event: PointerLikeEvent): Action[] {
    if (this.#suspended || (event.pointerType && event.pointerType !== 'mouse')) return []
    const code = this.#code(event)
    const actions = actionsForCode(this.bindings, `mouse:${code}`)
    if (actions.length === 0) return []
    this.#down.add(code)
    for (const action of actions) this.#state.set(action, true)
    return actions
  }

  pointerup(event: PointerLikeEvent): Action[] {
    const code = this.#code(event)
    const actions = actionsForCode(this.bindings, `mouse:${code}`)
    this.#down.delete(code)
    for (const action of actions) this.#state.set(action, false)
    return actions
  }

  wheel(event: WheelLikeEvent): Action[] {
    if (this.#suspended || event.deltaY === 0) return []
    const code = event.deltaY < 0 ? 'WheelUp' : 'WheelDown'
    const actions = actionsForCode(this.bindings, `mouse:${code}`)
    if (actions.length === 0) return []
    event.preventDefault?.()
    for (const action of actions) this.#state.pulse(action)
    return actions
  }
}
