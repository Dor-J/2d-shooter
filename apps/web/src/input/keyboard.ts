import { type Action } from './actions.ts'
import { type BindingSet, actionsForCode } from './bindings.ts'
import type { ActionState } from './state.ts'

export type KeyLikeEvent = { code: string; target?: unknown; preventDefault?: () => void }

const TEXT_ENTRY_TAGS = new Set(['INPUT', 'TEXTAREA', 'SELECT'])

/** True when a text field or editable region owns the event, so gameplay must not consume the key. */
export function isTextEntryTarget(target: unknown): boolean {
  if (!target || typeof target !== 'object') return false
  const element = target as { tagName?: unknown; isContentEditable?: unknown }
  if (element.isContentEditable === true) return true
  return typeof element.tagName === 'string' && TEXT_ENTRY_TAGS.has(element.tagName)
}

/** Codes the browser would otherwise act on while a match is in focus. */
const SWALLOWED = new Set(['Space', 'Tab', 'ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'F2', 'F3', 'F4', 'F5'])

export class KeyboardDevice {
  bindings: BindingSet
  #state: ActionState
  #suspended = false
  #down = new Set<string>()

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
    for (const code of this.#down) {
      for (const action of actionsForCode(this.bindings, `keyboard:${code}`)) this.#state.set(action, false)
    }
    this.#down.clear()
  }

  actionsFor(code: string): Action[] {
    return actionsForCode(this.bindings, `keyboard:${code}`)
  }

  keydown(event: KeyLikeEvent): Action[] {
    if (this.#suspended || isTextEntryTarget(event.target)) return []
    const actions = this.actionsFor(event.code)
    if (actions.length === 0) return []
    if (SWALLOWED.has(event.code)) event.preventDefault?.()
    this.#down.add(event.code)
    for (const action of actions) this.#state.set(action, true)
    return actions
  }

  keyup(event: KeyLikeEvent): Action[] {
    // A key that went down before a suspension must still be released on the way up.
    const actions = this.actionsFor(event.code)
    this.#down.delete(event.code)
    for (const action of actions) this.#state.set(action, false)
    return actions
  }
}
