import type { Action } from './actions.ts'
import { type BindingSet, actionsForCode } from './bindings.ts'
import type { ActionState } from './state.ts'

export type GamepadLike = {
  id?: string
  connected?: boolean
  buttons: readonly { pressed: boolean }[]
  axes: readonly number[]
}

export const GAMEPAD_DEADZONE = 0.35
export const GAMEPAD_BUTTONS = 16
export const GAMEPAD_AXES = 4

/** Every binding code a gamepad can produce, for the rebinding UI. */
export function gamepadCodes(): string[] {
  const buttons = Array.from({ length: GAMEPAD_BUTTONS }, (_, index) => `Button${index}`)
  const axes = Array.from({ length: GAMEPAD_AXES }, (_, index) => [`Axis${index}-`, `Axis${index}+`]).flat()
  return [...buttons, ...axes]
}

export class GamepadDevice {
  bindings: BindingSet
  #state: ActionState
  #suspended = false
  #active = new Set<string>()

  constructor(state: ActionState, bindings: BindingSet) {
    this.#state = state
    this.bindings = bindings
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
    for (const code of this.#active) {
      for (const action of actionsForCode(this.bindings, `gamepad:${code}`)) this.#state.set(action, false)
    }
    this.#active.clear()
  }

  /** Right stick aim direction, or null while it rests inside the dead zone. */
  aimStick(pad: GamepadLike | null | undefined): { x: number; y: number } | null {
    if (!pad) return null
    const x = pad.axes[2] ?? 0
    const y = pad.axes[3] ?? 0
    if (Math.hypot(x, y) < GAMEPAD_DEADZONE) return null
    return { x, y }
  }

  /** Reads one snapshot of the pad and mirrors it onto the shared action state. */
  poll(pad: GamepadLike | null | undefined): Action[] {
    if (this.#suspended || !pad || pad.connected === false) {
      this.releaseAll()
      return []
    }
    const down = new Set<string>()
    pad.buttons.forEach((button, index) => {
      if (button?.pressed) down.add(`Button${index}`)
    })
    pad.axes.forEach((value, index) => {
      if (value <= -GAMEPAD_DEADZONE) down.add(`Axis${index}-`)
      if (value >= GAMEPAD_DEADZONE) down.add(`Axis${index}+`)
    })

    const changed: Action[] = []
    for (const code of this.#active) {
      if (down.has(code)) continue
      for (const action of actionsForCode(this.bindings, `gamepad:${code}`)) {
        this.#state.set(action, false)
        changed.push(action)
      }
    }
    for (const code of down) {
      for (const action of actionsForCode(this.bindings, `gamepad:${code}`)) {
        this.#state.set(action, true)
        if (!this.#active.has(code)) changed.push(action)
      }
    }
    this.#active = down
    return changed
  }
}
