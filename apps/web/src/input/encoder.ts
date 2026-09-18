import type { ActionState } from './state.ts'

export type Vec2 = { x: number; y: number }
export type Emote = 'cigar' | 'victory' | 'mercy' | 'taunt'

/** The wire shape of `game_core::Input`; this module is its only producer. */
export type InputFrame = {
  seq: number
  left: boolean
  right: boolean
  jump: boolean
  jet: boolean
  crouch: boolean
  prone: boolean
  roll: boolean
  emote: Emote | null
  reload: boolean
  fire: boolean
  throw_grenade: boolean
  aim: Vec2
  weapon: number
  drop: boolean
  throw_weapon: boolean
  throw_knife: boolean
  pickup: boolean
}

export const WEAPON_COUNT = 14
export const PRIMARY_WEAPONS = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
export const SECONDARY_WEAPONS = [10, 11, 12, 13]

export type WeaponMemory = { primary: number; secondary: number }

export type EncoderContext = {
  seq: number
  aim: Vec2
  weapon: number
  /** Jump+crouch means backflip in the air and a flag throw on the ground. */
  airborne?: boolean
  /** Carried slot numbers. When set, next/prev/switch stay inside the inventory. */
  owned?: number[]
}

const EMOTES: [string, Emote][] = [
  ['emoteCigar', 'cigar'],
  ['emoteVictory', 'victory'],
  ['emoteMercy', 'mercy'],
  ['emoteTaunt', 'taunt'],
]

/** True when the player asked for a backflip, either with the key or with the jump+crouch combo. */
export function backflipRequested(state: ActionState, airborne: boolean): boolean {
  if (state.held('backflip')) return true
  return airborne && state.held('jump') && state.held('crouch')
}

/** The flag-throw combo, kept here so the dedicated key and the combo cannot drift apart. */
export function flagThrowRequested(state: ActionState, airborne: boolean): boolean {
  if (state.pressed('flagThrow')) return true
  return !airborne && state.pressed('jump') && state.held('crouch')
}

export function resolveWeapon(
  current: number,
  state: ActionState,
  memory: WeaponMemory = { primary: 0, secondary: SECONDARY_WEAPONS[0] },
  owned?: number[],
): number {
  for (let slot = 0; slot < 10; slot += 1) {
    if (state.pressed(`selectWeapon${slot + 1}`)) return slot
  }
  const cycle = owned && owned.length > 0 ? owned : null
  if (state.pressed('switchWeapon')) {
    if (cycle) {
      const index = cycle.indexOf(current)
      return cycle[(index + 1) % cycle.length] ?? cycle[0]
    }
    return PRIMARY_WEAPONS.includes(current) ? memory.secondary : memory.primary
  }
  if (state.pressed('nextWeapon')) {
    if (cycle) {
      const index = cycle.indexOf(current)
      return cycle[(index + 1) % cycle.length] ?? cycle[0]
    }
    return (current + 1) % WEAPON_COUNT
  }
  if (state.pressed('previousWeapon')) {
    if (cycle) {
      const index = cycle.indexOf(current)
      return cycle[(index - 1 + cycle.length) % cycle.length] ?? cycle[0]
    }
    return (current - 1 + WEAPON_COUNT) % WEAPON_COUNT
  }
  if (state.pressed('selectPrimary')) {
    return PRIMARY_WEAPONS.includes(current) ? current : memory.primary
  }
  if (state.pressed('selectSecondary')) {
    return SECONDARY_WEAPONS.includes(current) ? current : memory.secondary
  }
  return current
}

export function rememberWeapon(memory: WeaponMemory, weapon: number): WeaponMemory {
  if (SECONDARY_WEAPONS.includes(weapon)) return { primary: memory.primary, secondary: weapon }
  return { primary: weapon, secondary: memory.secondary }
}

/** Turns the shared action state into exactly one protocol input frame. */
export function encodeInput(state: ActionState, context: EncoderContext): InputFrame {
  const airborne = context.airborne ?? false
  const emote = EMOTES.find(([action]) => state.pressed(action))?.[1] ?? null
  return {
    seq: context.seq,
    left: state.held('moveLeft'),
    right: state.held('moveRight'),
    jump: state.held('jump'),
    jet: state.held('jet'),
    crouch: state.held('crouch'),
    prone: state.held('prone'),
    roll: state.held('roll') || backflipRequested(state, airborne),
    emote,
    reload: state.held('reload'),
    fire: state.held('fire'),
    throw_grenade: state.held('throwGrenade'),
    aim: { x: context.aim.x, y: context.aim.y },
    weapon: Math.min(WEAPON_COUNT - 1, Math.max(0, Math.trunc(context.weapon))),
    drop: state.held('dropWeapon'),
    throw_weapon: state.held('throwWeapon'),
    throw_knife: state.held('throwKnife'),
    pickup: state.held('pickup'),
  }
}
