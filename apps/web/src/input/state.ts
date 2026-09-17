import type { Action } from './actions.ts'

/**
 * Holds the current action set for one simulation tick. Devices write into it; the encoder and the
 * client overlays read pressed/held/released from it, so every device shares one set of semantics.
 */
export class ActionState {
  #held = new Set<Action>()
  #previous = new Set<Action>()
  #ticks = new Map<Action, number>()
  #toggles = new Set<Action>()
  #latched = new Set<Action>()
  #pulses = new Set<Action>()

  /** Actions in this set flip on a press instead of following the key, for one-handed play. */
  setToggleActions(actions: Iterable<Action>) {
    this.#toggles = new Set(actions)
    for (const action of [...this.#latched]) {
      if (!this.#toggles.has(action)) {
        this.#latched.delete(action)
        this.#held.delete(action)
      }
    }
  }

  toggleActions(): Action[] {
    return [...this.#toggles]
  }

  set(action: Action, down: boolean) {
    if (this.#toggles.has(action)) {
      if (!down) return
      if (this.#latched.delete(action)) this.#held.delete(action)
      else {
        this.#latched.add(action)
        this.#held.add(action)
      }
      return
    }
    if (down) this.#held.add(action)
    else this.#held.delete(action)
  }

  /** Fires an action for exactly one tick, for wheel notches and other momentary devices. */
  pulse(action: Action) {
    this.#previous.delete(action)
    this.#held.add(action)
    this.#pulses.add(action)
  }

  held(action: Action): boolean {
    return this.#held.has(action)
  }

  pressed(action: Action): boolean {
    return this.#held.has(action) && !this.#previous.has(action)
  }

  released(action: Action): boolean {
    return !this.#held.has(action) && this.#previous.has(action)
  }

  /** Whole ticks this action has been held, which is what a charged throw measures. */
  heldTicks(action: Action): number {
    return this.#ticks.get(action) ?? 0
  }

  /** Drops every held action while leaving the release edge visible for one tick. */
  releaseAll() {
    for (const action of this.#held) this.#previous.add(action)
    this.#held.clear()
    this.#latched.clear()
    this.#pulses.clear()
    this.#ticks.clear()
  }

  /** Ends the tick: today's held set becomes the comparison baseline for the next one. */
  commit() {
    for (const action of this.#held) this.#ticks.set(action, (this.#ticks.get(action) ?? 0) + 1)
    for (const action of [...this.#ticks.keys()]) if (!this.#held.has(action)) this.#ticks.delete(action)
    this.#previous = new Set(this.#held)
    for (const action of this.#pulses) {
      this.#held.delete(action)
      this.#ticks.delete(action)
    }
    this.#pulses.clear()
  }
}
