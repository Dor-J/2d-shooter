/**
 * Reading the authoritative flag state for the HUD.
 *
 * The server owns every flag: where it is, who has it, and how long a dropped one has left. These
 * are the conversions the HUD needs to say so on screen.
 */

export type WireFlagState =
  | 'AtBase'
  | { Carried: { by: number } }
  | { Dropped: { ticks_left: number } }

export type WireFlagKind = 'Alpha' | 'Bravo' | 'Yellow'

export type WireFlag = {
  kind: WireFlagKind
  base: { x: number; y: number }
  state: WireFlagState
  body: { pos: { x: number; y: number } }
}

export type WireObjectives = { flags: WireFlag[] }

const TICK_RATE = 60

/** What a flag is called on screen. */
export function flagName(kind: WireFlagKind): string {
  if (kind === 'Alpha') return 'Red flag'
  if (kind === 'Bravo') return 'Blue flag'
  return 'Yellow flag'
}

/** Who is carrying this flag, if anybody. */
export function carrierOf(flag: WireFlag): number | null {
  const state = flag.state
  if (typeof state === 'object' && 'Carried' in state) return state.Carried.by
  return null
}

/** Whether the flag is away from its base, which is what the missing-flag indicator shows. */
export function isMissing(flag: WireFlag): boolean {
  return flag.state !== 'AtBase'
}

/** Seconds before a dropped flag takes itself home, or null when it is not counting down. */
export function returnCountdown(flag: WireFlag): number | null {
  const state = flag.state
  if (typeof state === 'object' && 'Dropped' in state) {
    return Math.ceil(state.Dropped.ticks_left / TICK_RATE)
  }
  return null
}

/** One line per flag for the HUD: what it is and where it is. */
export function flagStatusLines(
  objectives: WireObjectives | undefined,
  names: Record<string, string>,
): { kind: WireFlagKind; text: string; missing: boolean }[] {
  return (objectives?.flags ?? []).map(flag => {
    const carrier = carrierOf(flag)
    if (carrier !== null) {
      const who = names[String(carrier)] ?? 'someone'
      return { kind: flag.kind, text: `${flagName(flag.kind)}: taken by ${who}`, missing: true }
    }
    const countdown = returnCountdown(flag)
    if (countdown !== null) {
      return { kind: flag.kind, text: `${flagName(flag.kind)}: dropped, back in ${countdown}s`, missing: true }
    }
    return { kind: flag.kind, text: `${flagName(flag.kind)}: at base`, missing: false }
  })
}

/** Whether this player is carrying something, for the carrier indicator over their head. */
export function isCarrying(objectives: WireObjectives | undefined, player: number): boolean {
  return (objectives?.flags ?? []).some(flag => carrierOf(flag) === player)
}
