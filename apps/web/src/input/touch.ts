import type { Action } from './actions.ts'
import type { ActionState } from './state.ts'

export type TouchButton = {
  action: Action
  label: string
  hint: string
  kind: 'hold' | 'press'
  group: 'stance' | 'combat' | 'weapons' | 'panel'
}

/** The on-screen equivalents of the desktop controls, in thumb-reach order. */
export const TOUCH_BUTTONS: TouchButton[] = [
  { action: 'jet', label: 'JET', hint: 'Hold to use the jetpack', kind: 'hold', group: 'combat' },
  { action: 'throwGrenade', label: 'NADE', hint: 'Throw a grenade', kind: 'hold', group: 'combat' },
  { action: 'crouch', label: 'CROUCH', hint: 'Hold to crouch', kind: 'hold', group: 'stance' },
  { action: 'prone', label: 'PRONE', hint: 'Go prone or get up', kind: 'hold', group: 'stance' },
  { action: 'roll', label: 'ROLL', hint: 'Roll on the ground or backflip in the air', kind: 'press', group: 'stance' },
  { action: 'reload', label: 'RELOAD', hint: 'Reload the current weapon', kind: 'press', group: 'weapons' },
  { action: 'switchWeapon', label: 'SWITCH', hint: 'Switch between primary and secondary', kind: 'press', group: 'weapons' },
  { action: 'throwWeapon', label: 'THROW', hint: 'Hold to charge a weapon throw', kind: 'hold', group: 'weapons' },
  { action: 'flagThrow', label: 'FLAG', hint: 'Throw the carried flag', kind: 'press', group: 'weapons' },
  { action: 'scoreboard', label: 'SCORE', hint: 'Hold to show the scoreboard', kind: 'hold', group: 'panel' },
  { action: 'teamChat', label: 'TEAM', hint: 'Send a team chat message', kind: 'press', group: 'panel' },
]

export type TouchLayout = {
  orientation: 'portrait' | 'landscape'
  padSize: number
  buttonSize: number
  padding: { top: string; bottom: string; left: string; right: string }
}

const gutter = (minimum: number, inset: string) => `max(${minimum}px, env(${inset}))`

/** Pad and gutter sizes that keep both orientations clear of the device safe areas. */
export function touchLayout(width: number, height: number): TouchLayout {
  const orientation = height >= width ? 'portrait' : 'landscape'
  const padSize = Math.min(160, Math.max(96, Math.round(height * 0.3)))
  const sideGutter = orientation === 'landscape' ? 24 : 18
  return {
    orientation,
    padSize,
    buttonSize: Math.round(padSize * 0.44),
    padding: {
      top: gutter(12, 'safe-area-inset-top'),
      bottom: gutter(orientation === 'portrait' ? 30 : 18, 'safe-area-inset-bottom'),
      left: gutter(sideGutter, 'safe-area-inset-left'),
      right: gutter(sideGutter, 'safe-area-inset-right'),
    },
  }
}

export function movementFromDrag(dx: number, dy: number) {
  return { left: dx < -18, right: dx > 18, jump: dy < -30 }
}

/** How far ahead of the player a fully deflected aim stick points, before sensitivity scaling. */
export const STICK_AIM_RANGE = 400

/** Turns an aim-pad direction into a world point, or null while the stick is centred. */
export function aimFromStick(
  origin: { x: number; y: number },
  dx: number,
  dy: number,
  range: number,
): { x: number; y: number } | null {
  const length = Math.hypot(dx, dy)
  if (length === 0) return null
  return { x: origin.x + (dx / length) * range, y: origin.y + (dy / length) * range }
}

/**
 * Tracks which pointer holds which action so that lifting one finger, or the browser cancelling a
 * touch, never leaves another control stuck down.
 */
export class TouchDevice {
  #state: ActionState
  #held = new Map<number, Action>()

  constructor(state: ActionState) {
    this.#state = state
  }

  press(pointerId: number, action: Action) {
    this.#held.set(pointerId, action)
    this.#state.set(action, true)
  }

  release(pointerId: number) {
    const action = this.#held.get(pointerId)
    if (action === undefined) return
    this.#held.delete(pointerId)
    this.#state.set(action, false)
  }

  cancel() {
    for (const action of this.#held.values()) this.#state.set(action, false)
    this.#held.clear()
  }
}
