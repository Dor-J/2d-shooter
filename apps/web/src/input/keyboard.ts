import { type Action } from './actions.ts'
import { type BindingSet, actionsForCode } from './bindings.ts'
import type { ActionState } from './state.ts'

export type KeyLikeEvent = {
  code: string
  ctrlKey?: boolean
  altKey?: boolean
  shiftKey?: boolean
  target?: unknown
  preventDefault?: () => void
  /** Tests set this. The live device uses the attached canvas instead. */
  canvasFocused?: boolean
}

/** Modifier order is Control, then Alt, then Shift, then the physical code. */
export function chordCode(event: {
  code: string
  ctrlKey?: boolean
  altKey?: boolean
  shiftKey?: boolean
}): string {
  const parts: string[] = []
  if (event.ctrlKey) parts.push('Control')
  if (event.altKey) parts.push('Alt')
  if (event.shiftKey) parts.push('Shift')
  parts.push(event.code)
  return parts.join('+')
}

/** Soldat defaults the browser would steal. Ctrl+W is left alone. */
export function browserWouldSteal(event: KeyLikeEvent): boolean {
  if (event.ctrlKey && event.code === 'KeyW') return false
  if (event.code === 'Tab') return true
  if (/^F(?:[1-9]|10)$/.test(event.code)) return true
  return Boolean(
    event.ctrlKey &&
    !event.altKey &&
    !event.shiftKey &&
    /^Digit[1-4]$/.test(event.code),
  )
}

export const KEYBOARD_LOCK_CODES = [
  'Tab',
  'F1',
  'F2',
  'F3',
  'F4',
  'F5',
  'F6',
  'F7',
  'F8',
  'F9',
  'F10',
  'Digit1',
  'Digit2',
  'Digit3',
  'Digit4',
]

type LockableKeyboard = { lock?: (codes: string[]) => Promise<unknown> }

/** Asks the browser to hold capturable keys in fullscreen. A missing API or a rejection is ignored. */
export function lockCapturableKeys(keyboard?: LockableKeyboard | null): void {
  const target =
    keyboard === undefined
      ? ((globalThis.navigator as Navigator & { keyboard?: LockableKeyboard })
          ?.keyboard ?? null)
      : keyboard
  if (!target?.lock) return
  void Promise.resolve(target.lock(KEYBOARD_LOCK_CODES)).catch(() => {})
}

const TEXT_ENTRY_TAGS = new Set(['INPUT', 'TEXTAREA', 'SELECT'])

/** True when a text field or editable region owns the event, so gameplay must not consume the key. */
export function isTextEntryTarget(target: unknown): boolean {
  if (!target || typeof target !== 'object') return false
  const element = target as { tagName?: unknown; isContentEditable?: unknown }
  if (element.isContentEditable === true) return true
  return (
    typeof element.tagName === 'string' && TEXT_ENTRY_TAGS.has(element.tagName)
  )
}

/** Codes the browser would otherwise act on while a match is in focus. */
const SWALLOWED = new Set([
  'Space',
  'Tab',
  'ArrowUp',
  'ArrowDown',
  'ArrowLeft',
  'ArrowRight',
])

export class KeyboardDevice {
  bindings: BindingSet
  #state: ActionState
  #suspended = false
  #canvas: HTMLElement | null = null
  /** Physical code to the chord that went down, so keyup still matches if a modifier is released first. */
  #down = new Map<string, string>()

  attachCanvas(canvas: HTMLElement) {
    this.#canvas = canvas
  }

  #canvasFocused(event: KeyLikeEvent): boolean {
    if (event.canvasFocused !== undefined) return event.canvasFocused
    if (!this.#canvas || typeof document === 'undefined') return false
    return document.activeElement === this.#canvas
  }

  constructor(state: ActionState, bindings: BindingSet) {
    this.#state = state
    this.bindings = bindings
  }

  get suspended(): boolean {
    return this.#suspended
  }

  /** Stops routing keys to gameplay and lets go of everything currently held. */
  suspend() {
    if (this.#suspended) return
    this.#suspended = true
    this.releaseAll()
  }

  resume() {
    this.#suspended = false
  }

  releaseAll() {
    for (const chord of this.#down.values()) {
      for (const action of actionsForCode(this.bindings, `keyboard:${chord}`))
        this.#state.set(action, false)
    }
    this.#down.clear()
  }

  actionsFor(code: string): Action[] {
    return actionsForCode(this.bindings, `keyboard:${code}`)
  }

  keydown(event: KeyLikeEvent): Action[] {
    if (this.#suspended || isTextEntryTarget(event.target)) return []
    if (this.#canvasFocused(event) && browserWouldSteal(event))
      event.preventDefault?.()
    const chord = chordCode(event)
    const actions = this.actionsFor(chord)
    if (actions.length === 0) return []
    if (SWALLOWED.has(event.code)) event.preventDefault?.()
    this.#down.set(event.code, chord)
    for (const action of actions) this.#state.set(action, true)
    return actions
  }

  keyup(event: KeyLikeEvent): Action[] {
    // A key that went down before a suspension must still be released on the way up.
    const chord = this.#down.get(event.code) ?? chordCode(event)
    const actions = this.actionsFor(chord)
    this.#down.delete(event.code)
    for (const action of actions) this.#state.set(action, false)
    return actions
  }
}
