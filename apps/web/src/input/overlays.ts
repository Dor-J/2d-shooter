import type { ActionState } from './state.ts'

export type InterfaceEvent =
  | { type: 'toggle'; name: string; value: boolean }
  | { type: 'chat'; scope: 'all' | 'team' }
  | { type: 'screenshot' }
  | { type: 'pause' }
  | { type: 'minimize' }
  | { type: 'demo'; recording: boolean }
  | { type: 'weaponMenu'; open: boolean }
  | { type: 'volume'; volume: number }
  | { type: 'music'; enabled: boolean; track: number }
  | { type: 'sensitivity'; delta: number }

const VOLUME_STEP = 0.1
export const MUSIC_TRACKS = 4

const clamp01 = (value: number) => Math.min(1, Math.max(0, Math.round(value * 100) / 100))

/**
 * Client-side consequences of interface controls. Everything here is presentation state the server
 * never owns, so it lives beside the input layer rather than inside the simulation.
 */
export class InterfaceState {
  scoreboardMode: 'hold' | 'toggle' = 'hold'
  scoreboardVisible = false
  scoreboardOffset = 0
  scoreboardRows = 0
  scoreboardWindow = 8
  minimap = true
  sniperLine = true
  performanceStats = true
  weaponStats = false
  consoleOpen = false
  weaponMenuOpen = false
  paused = false
  backgrounded = false
  recordingDemo = false
  fastForward = false
  musicEnabled = true
  musicTrack = 0
  volume = 0.8
  musicVolume = 0.5

  setScoreboardExtent(rows: number, window: number) {
    this.scoreboardRows = Math.max(0, Math.trunc(rows))
    this.scoreboardWindow = Math.max(1, Math.trunc(window))
    this.scoreboardOffset = Math.min(this.scoreboardOffset, this.maxScoreboardOffset())
  }

  maxScoreboardOffset(): number {
    return Math.max(0, this.scoreboardRows - this.scoreboardWindow)
  }

  #toggle(events: InterfaceEvent[], name: 'minimap' | 'sniperLine' | 'performanceStats' | 'weaponStats') {
    this[name] = !this[name]
    events.push({ type: 'toggle', name, value: this[name] })
  }

  /** Reads one tick of action state and returns the intents the rest of the client must act on. */
  apply(state: ActionState): InterfaceEvent[] {
    const events: InterfaceEvent[] = []

    if (this.scoreboardMode === 'toggle') {
      if (state.pressed('scoreboard')) this.scoreboardVisible = !this.scoreboardVisible
    } else {
      this.scoreboardVisible = state.held('scoreboard')
    }
    if (this.scoreboardVisible) {
      if (state.pressed('scoreboardScrollUp')) this.scoreboardOffset = Math.max(0, this.scoreboardOffset - 1)
      if (state.pressed('scoreboardScrollDown')) {
        this.scoreboardOffset = Math.min(this.maxScoreboardOffset(), this.scoreboardOffset + 1)
      }
    } else {
      this.scoreboardOffset = 0
    }

    if (state.pressed('minimap')) this.#toggle(events, 'minimap')
    if (state.pressed('sniperLine')) this.#toggle(events, 'sniperLine')
    if (state.pressed('performanceStats')) this.#toggle(events, 'performanceStats')
    if (state.pressed('weaponStats')) this.#toggle(events, 'weaponStats')

    if (state.pressed('chat')) events.push({ type: 'chat', scope: 'all' })
    if (state.pressed('teamChat')) events.push({ type: 'chat', scope: 'team' })
    if (state.pressed('console')) {
      this.consoleOpen = !this.consoleOpen
      events.push({ type: 'toggle', name: 'console', value: this.consoleOpen })
    }
    if (state.pressed('weaponMenu')) {
      this.weaponMenuOpen = !this.weaponMenuOpen
      events.push({ type: 'weaponMenu', open: this.weaponMenuOpen })
    }

    if (state.pressed('screenshot')) events.push({ type: 'screenshot' })
    if (state.pressed('pause')) {
      this.paused = !this.paused
      events.push({ type: 'pause' })
    }
    // A browser tab cannot minimise itself; the closest honest equivalent is leaving the match in
    // the background with input released, which is what the client does here.
    if (state.pressed('minimize')) {
      this.backgrounded = true
      events.push({ type: 'minimize' })
    }
    if (state.pressed('demoRecord')) {
      this.recordingDemo = !this.recordingDemo
      events.push({ type: 'demo', recording: this.recordingDemo })
    }
    this.fastForward = state.held('demoFastForward')

    if (state.pressed('volumeUp') || state.pressed('volumeDown')) {
      const delta = state.pressed('volumeUp') ? VOLUME_STEP : -VOLUME_STEP
      this.volume = clamp01(this.volume + delta)
      events.push({ type: 'volume', volume: this.volume })
    }
    if (state.pressed('musicToggle')) {
      this.musicEnabled = !this.musicEnabled
      events.push({ type: 'music', enabled: this.musicEnabled, track: this.musicTrack })
    }
    if (state.pressed('musicNext') || state.pressed('musicPrevious')) {
      const step = state.pressed('musicNext') ? 1 : -1
      this.musicTrack = (this.musicTrack + step + MUSIC_TRACKS) % MUSIC_TRACKS
      events.push({ type: 'music', enabled: this.musicEnabled, track: this.musicTrack })
    }
    if (state.pressed('sensitivityUp')) events.push({ type: 'sensitivity', delta: 1 })
    if (state.pressed('sensitivityDown')) events.push({ type: 'sensitivity', delta: -1 })

    return events
  }
}
