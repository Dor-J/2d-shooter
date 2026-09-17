import { type BindingSet, DEFAULT_BINDINGS, cloneBindings } from './bindings.ts'
import { type InputFrame, type WeaponMemory, encodeInput, rememberWeapon, resolveWeapon, SECONDARY_WEAPONS } from './encoder.ts'
import { GamepadDevice, type GamepadLike } from './gamepad.ts'
import { KeyboardDevice } from './keyboard.ts'
import { MouseDevice, clampSensitivity } from './mouse.ts'
import { InterfaceState, type InterfaceEvent } from './overlays.ts'
import type { ControlProfile } from './profiles.ts'
import { ActionState } from './state.ts'
import { TouchDevice } from './touch.ts'

export type InputSystemOptions = {
  bindings?: BindingSet
  state?: ActionState
  sensitivity?: number
}

export type TickContext = {
  aim: { x: number; y: number }
  airborne?: boolean
  gamepad?: GamepadLike | null
}

export type TickResult = {
  frame: InputFrame
  events: InterfaceEvent[]
}

/**
 * Owns every input device, the shared action state, and the single encoder. The rest of the client
 * asks it for one frame per simulation tick and never talks to a device directly.
 */
export class InputSystem {
  bindings: BindingSet
  state: ActionState
  keyboard: KeyboardDevice
  mouse: MouseDevice
  gamepad: GamepadDevice
  touch: TouchDevice
  ui = new InterfaceState()
  weapon = 0
  seq = 0
  #memory: WeaponMemory = { primary: 0, secondary: SECONDARY_WEAPONS[0] }
  #suspended = false

  constructor(options: InputSystemOptions = {}) {
    this.bindings = cloneBindings(options.bindings ?? DEFAULT_BINDINGS)
    this.state = options.state ?? new ActionState()
    this.keyboard = new KeyboardDevice(this.state, this.bindings)
    this.mouse = new MouseDevice(this.state, this.bindings, options.sensitivity ?? 1)
    this.gamepad = new GamepadDevice(this.state, this.bindings)
    this.touch = new TouchDevice(this.state)
  }

  get suspended(): boolean {
    return this.#suspended
  }

  setBindings(bindings: BindingSet) {
    this.bindings = cloneBindings(bindings)
    this.keyboard.bindings = this.bindings
    this.mouse.bindings = this.bindings
    this.gamepad.bindings = this.bindings
  }

  applyProfile(profile: ControlProfile) {
    this.setBindings(profile.bindings)
    this.mouse.setSensitivity(profile.settings.sensitivity)
    this.ui.volume = profile.settings.volume
    this.ui.musicVolume = profile.settings.musicVolume
    this.state.setToggleActions(profile.settings.toggleActions)
  }

  /** Menus and text fields own the keyboard; gameplay keeps nothing held while they do. */
  suspend() {
    if (this.#suspended) return
    this.#suspended = true
    this.keyboard.suspend()
    this.mouse.suspend()
    this.gamepad.suspend()
    this.touch.cancel()
    this.state.releaseAll()
  }

  resume() {
    if (!this.#suspended) return
    this.#suspended = false
    this.keyboard.resume()
    this.mouse.resume()
    this.gamepad.resume()
  }

  releaseAll() {
    this.keyboard.releaseAll()
    this.mouse.releaseAll()
    this.gamepad.releaseAll()
    this.touch.cancel()
    this.state.releaseAll()
  }

  /** One simulation tick: poll, resolve, encode, dispatch interface intents, then commit edges. */
  tick(context: TickContext): TickResult {
    if (context.gamepad !== undefined) this.gamepad.poll(context.gamepad)
    this.weapon = resolveWeapon(this.weapon, this.state, this.#memory)
    this.#memory = rememberWeapon(this.#memory, this.weapon)
    const events = this.ui.apply(this.state)
    for (const event of events) {
      if (event.type === 'sensitivity') this.mouse.adjustSensitivity(event.delta * 0.25)
      if (event.type === 'minimize') this.suspend()
    }
    this.seq += 1
    const frame = encodeInput(this.state, {
      seq: this.seq,
      aim: context.aim,
      weapon: this.weapon,
      airborne: context.airborne,
    })
    this.state.commit()
    return { frame, events }
  }
}

export { ActionState, InterfaceState, KeyboardDevice, MouseDevice, GamepadDevice, TouchDevice, clampSensitivity }
export type { InputFrame, InterfaceEvent, BindingSet, ControlProfile }
