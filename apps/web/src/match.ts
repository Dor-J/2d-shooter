/**
 * Reading the authoritative match state for the HUD.
 *
 * Nothing here decides anything: the server owns the phase, the clock, and the score. These are
 * the small conversions the HUD needs to put them on screen.
 */

/** The phases the server reports, matching `game_core::MatchPhase`. */
export type MatchPhase = 'Lobby' | 'Countdown' | 'Active' | 'Overtime' | 'RoundEnd' | 'MapTransition'

export type Outcome = { Player: number } | { Team: number } | 'Draw'

export type WireMatchState = {
  phase: MatchPhase
  phase_ticks: number
  elapsed: number
  outcome: Outcome | null
  final_screenshot?: boolean
  minimum_players?: number
}

export type WireLimits = { kills: number; points: number; captures: number; time_ticks: number }
export type WireModifierSet = { realistic: boolean; survival: boolean; advance: boolean }
export type WireRules = {
  kind: string
  limits: WireLimits
  friendly_fire: boolean
  modifiers?: WireModifierSet
}

const TICK_RATE = 60

/** Seconds as `m:ss`, which is how a match clock reads. */
export function formatClock(seconds: number): string {
  const whole = Math.max(0, Math.floor(seconds))
  const minutes = Math.floor(whole / 60)
  return `${minutes}:${String(whole % 60).padStart(2, '0')}`
}

/** What the clock should show: time left when the match is timed, time played when it is not. */
export function clockText(state: WireMatchState | undefined, rules: WireRules | undefined): string {
  if (!state) return ''
  const limit = rules?.limits?.time_ticks ?? 0
  const ticks = limit > 0 ? Math.max(0, limit - state.elapsed) : state.elapsed
  return formatClock(ticks / TICK_RATE)
}

/** Whether the match is being played, so the HUD can dim itself when it is not. */
export function isPlayable(state: WireMatchState | undefined): boolean {
  return state?.phase === 'Active' || state?.phase === 'Overtime'
}

/** The banner across the middle of the screen, or an empty string during normal play. */
export function phaseBanner(state: WireMatchState | undefined): string {
  if (!state) return ''
  switch (state.phase) {
    case 'Lobby':
      return 'Waiting for players'
    case 'Countdown': {
      const seconds = Math.ceil(state.phase_ticks / TICK_RATE)
      return seconds > 0 ? `Starting in ${seconds}` : 'Go'
    }
    case 'Overtime':
      return 'Overtime'
    case 'RoundEnd':
      return outcomeText(state.outcome)
    case 'MapTransition':
      return 'Loading the next map'
    default:
      return ''
  }
}

/** How the match ended, in words. */
export function outcomeText(outcome: Outcome | null | undefined): string {
  if (!outcome) return 'Round over'
  if (outcome === 'Draw') return 'Draw'
  if (typeof outcome === 'object' && 'Team' in outcome) {
    return outcome.Team === 1 ? 'Alpha wins' : 'Bravo wins'
  }
  if (typeof outcome === 'object' && 'Player' in outcome) return 'Round over'
  return 'Round over'
}

/** The winner's name, when the server named a player and we know who that is. */
export function winnerName(
  outcome: Outcome | null | undefined,
  names: Record<string, string>,
): string | null {
  if (!outcome || outcome === 'Draw' || !(typeof outcome === 'object' && 'Player' in outcome)) {
    return null
  }
  return names[String(outcome.Player)] ?? null
}
