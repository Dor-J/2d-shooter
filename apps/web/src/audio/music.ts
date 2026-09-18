// The playlist. Four tracks, looping, with previous/next and a mute that is not a volume of zero
// so the player's saved volume comes back when they unmute.

export const TRACKS = ['Track 1', 'Track 2', 'Track 3', 'Track 4'] as const

export type MusicState = {
  enabled: boolean
  track: number
  volume: number
  playing: boolean
}

export function defaultMusic(): MusicState {
  return { enabled: true, track: 0, volume: 0.5, playing: false }
}

export function trackName(track: number): string {
  const index = ((Math.trunc(track) % TRACKS.length) + TRACKS.length) % TRACKS.length
  return TRACKS[index]
}

export function stepTrack(track: number, delta: number): number {
  return ((Math.trunc(track) + Math.trunc(delta)) % TRACKS.length + TRACKS.length) % TRACKS.length
}

export function applyMusicCommand(
  state: MusicState,
  command: { enabled?: boolean; track?: number; volume?: number },
): MusicState {
  const enabled = command.enabled ?? state.enabled
  const track = command.track === undefined ? state.track : stepTrack(command.track, 0)
  const volume = command.volume === undefined ? state.volume : Math.min(1, Math.max(0, command.volume))
  return {
    enabled,
    track,
    volume,
    playing: enabled && volume > 0,
  }
}
