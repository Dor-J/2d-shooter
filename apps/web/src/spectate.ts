/**
 * The spectator HUD: who is being watched and how to change it.
 *
 * The server decides who may be followed and what a spectator is allowed to see. This only reads
 * the state it is sent and turns it into something to put on screen.
 */

export type WireSpectator = {
  view: 'Following' | 'FreeCamera'
  target: number | null
  camera: { x: number; y: number }
}

export type SpectateCommand =
  | { type: 'next' }
  | { type: 'previous' }
  | { type: 'follow'; player: number }
  | { type: 'free_camera' }

/** Whether this client is watching rather than playing. */
export function isSpectating(spectator: WireSpectator | undefined): boolean {
  return spectator !== undefined
}

/** What the spectator bar says it is showing. */
export function spectatorLabel(
  spectator: WireSpectator | undefined,
  names: Record<string, string>,
): string {
  if (!spectator) return ''
  if (spectator.view === 'FreeCamera') return 'Free camera'
  if (spectator.target === null) return 'Nobody left to watch'
  return `Watching ${names[String(spectator.target)] ?? 'a player'}`
}

/** The command a key press asks for, or nothing when the key means something else. */
export function commandForKey(key: string): SpectateCommand | null {
  if (key === 'ArrowRight' || key === 'd' || key === 'D') return { type: 'next' }
  if (key === 'ArrowLeft' || key === 'a' || key === 'A') return { type: 'previous' }
  if (key === 'f' || key === 'F') return { type: 'free_camera' }
  return null
}

/** Whether the delay bar should be shown, and what it should say. */
export function delayLabel(delayTicks: number | undefined): string {
  const ticks = delayTicks ?? 0
  if (ticks <= 0) return ''
  const seconds = Math.round(ticks / 60)
  return `Broadcast delayed ${seconds}s`
}
