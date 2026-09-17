import { type Action, actionIds, isAction } from './actions.ts'

export type DeviceKind = 'keyboard' | 'mouse' | 'gamepad' | 'touch'
export type Binding = { device: DeviceKind; code: string }
export type BindingSet = Record<Action, Binding[]>

export const BINDINGS_VERSION = 1

const key = (code: string): Binding => ({ device: 'keyboard', code })
const mouse = (code: string): Binding => ({ device: 'mouse', code })
const pad = (code: string): Binding => ({ device: 'gamepad', code })

const weaponSlotDefaults = Object.fromEntries(
  Array.from({ length: 10 }, (_, index) => [
    `selectWeapon${index + 1}`,
    [key(index === 9 ? 'Digit0' : `Digit${index + 1}`)],
  ]),
)

const defaults: BindingSet = {
  moveLeft: [key('KeyA'), key('ArrowLeft'), pad('Axis0-')],
  moveRight: [key('KeyD'), key('ArrowRight'), pad('Axis0+')],
  jump: [key('Space'), key('KeyW'), key('ArrowUp'), pad('Button0')],
  jet: [key('ShiftLeft'), key('ShiftRight'), pad('Button6')],
  crouch: [key('KeyS'), key('ArrowDown'), pad('Button1')],
  prone: [key('KeyX'), pad('Button2')],
  roll: [key('KeyC')],
  backflip: [key('KeyZ')],
  fire: [mouse('Mouse0'), pad('Button7')],
  throwGrenade: [key('KeyE'), mouse('Mouse2'), pad('Button5')],
  reload: [key('KeyR'), pad('Button3')],
  switchWeapon: [key('KeyQ'), pad('Button4')],
  dropWeapon: [key('KeyG')],
  throwWeapon: [key('KeyT')],
  throwKnife: [key('KeyV')],
  pickup: [key('KeyF')],
  flagThrow: [key('KeyB')],
  weaponMenu: [key('KeyM')],
  selectPrimary: [key('KeyO')],
  selectSecondary: [key('KeyP')],
  ...weaponSlotDefaults,
  nextWeapon: [key('Period')],
  previousWeapon: [key('Comma')],
  chat: [key('Enter')],
  teamChat: [key('KeyY')],
  console: [key('Backquote')],
  emoteTaunt: [key('KeyH')],
  emoteVictory: [key('KeyJ')],
  emoteMercy: [key('KeyK')],
  emoteCigar: [key('KeyU')],
  scoreboard: [key('Tab'), pad('Button8')],
  scoreboardScrollUp: [key('PageUp'), mouse('WheelUp')],
  scoreboardScrollDown: [key('PageDown'), mouse('WheelDown')],
  weaponStats: [key('F4')],
  minimap: [key('KeyN')],
  sniperLine: [key('KeyL')],
  performanceStats: [key('F3')],
  screenshot: [key('F2')],
  musicToggle: [key('F5')],
  musicPrevious: [key('F6')],
  musicNext: [key('F7')],
  demoRecord: [key('F8')],
  demoFastForward: [key('F9')],
  volumeUp: [key('Equal')],
  volumeDown: [key('Minus')],
  sensitivityUp: [key('BracketRight')],
  sensitivityDown: [key('BracketLeft')],
  pause: [key('Pause')],
  minimize: [key('F10')],
}

export const DEFAULT_BINDINGS: BindingSet = Object.freeze(
  Object.fromEntries(actionIds().map(action => [action, Object.freeze(defaults[action] ?? [])])),
) as BindingSet

export function bindingCode(binding: Binding): string {
  return `${binding.device}:${binding.code}`
}

export function cloneBindings(set: BindingSet): BindingSet {
  return Object.fromEntries(
    Object.entries(set).map(([action, bindings]) => [action, bindings.map(binding => ({ ...binding }))]),
  ) as BindingSet
}

export function actionsForCode(set: BindingSet, code: string): Action[] {
  return Object.entries(set)
    .filter(([, bindings]) => bindings.some(binding => bindingCode(binding) === code))
    .map(([action]) => action)
}

export function conflictsFor(set: BindingSet, binding: Binding, action: Action): Action[] {
  return actionsForCode(set, bindingCode(binding)).filter(owner => owner !== action)
}

/** Assigns `binding` to `action` at `index`, taking the code away from whichever control held it. */
export function rebind(set: BindingSet, action: Action, index: number, binding: Binding): BindingSet {
  if (!isAction(action)) return cloneBindings(set)
  const code = bindingCode(binding)
  const next = cloneBindings(set)
  for (const owner of Object.keys(next)) {
    next[owner] = next[owner].filter(existing => bindingCode(existing) !== code)
  }
  const slots = next[action] ?? []
  if (index >= 0 && index < slots.length) slots[index] = { ...binding }
  else slots.push({ ...binding })
  next[action] = slots
  return next
}

export function clearBinding(set: BindingSet, action: Action, index: number): BindingSet {
  const next = cloneBindings(set)
  if (next[action]) next[action] = next[action].filter((_, slot) => slot !== index)
  return next
}

export function resetBindings(): BindingSet {
  return cloneBindings(DEFAULT_BINDINGS)
}

export function serializeBindings(set: BindingSet): string {
  return JSON.stringify({ version: BINDINGS_VERSION, bindings: set })
}

const DEVICES: DeviceKind[] = ['keyboard', 'mouse', 'gamepad', 'touch']

function readBindingList(value: unknown): Binding[] | null {
  if (!Array.isArray(value)) return null
  const bindings: Binding[] = []
  for (const entry of value) {
    if (!entry || typeof entry !== 'object') return null
    const { device, code } = entry as Partial<Binding>
    if (typeof code !== 'string' || code.length === 0 || code.length > 32) return null
    if (!DEVICES.includes(device as DeviceKind)) return null
    bindings.push({ device: device as DeviceKind, code })
  }
  return bindings
}

/** Reads stored bindings, falling back to the defaults for anything missing, unknown, or malformed. */
export function parseBindings(raw: string | null | undefined): BindingSet {
  const merged = resetBindings()
  if (!raw) return merged
  let payload: unknown
  try {
    payload = JSON.parse(raw)
  } catch {
    return merged
  }
  if (!payload || typeof payload !== 'object') return merged
  const { version, bindings } = payload as { version?: unknown; bindings?: unknown }
  if (version !== BINDINGS_VERSION) return merged
  if (!bindings || typeof bindings !== 'object') return merged
  for (const [action, value] of Object.entries(bindings as Record<string, unknown>)) {
    if (!isAction(action)) continue
    const parsed = readBindingList(value)
    if (parsed) merged[action] = parsed
  }
  return merged
}
