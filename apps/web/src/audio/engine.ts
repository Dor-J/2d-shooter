// One mixer for every sound the match makes.
//
// The simulation names an event. This engine picks a clip, places it, and respects the volumes
// the player saved. Nothing here is sent back to the server.

import { clipFor, CLIPS, type Clip, type ClipId } from './manifest.ts'

export type Vec2 = { x: number; y: number }

export type SoundQuality = 'low' | 'high'

export type PlayedSound = {
  clip: ClipId
  volume: number
  pan: number
  loop: boolean
  hz: number
}

export type AudioSettings = {
  master: number
  music: number
  quality: SoundQuality
  device: string | null
  distantBattle: boolean
  explosionEffect: boolean
}

export function defaultAudioSettings(): AudioSettings {
  return {
    master: 0.8,
    music: 0.5,
    quality: 'high',
    device: null,
    distantBattle: true,
    explosionEffect: true,
  }
}

export function clampVolume(value: number): number {
  if (!Number.isFinite(value)) return 0
  return Math.min(1, Math.max(0, value))
}

/// How loud a sound is from here, 0 to 1. Beyond `range` it is silence.
export function attenuation(from: Vec2, listener: Vec2, range = 900): number {
  const distance = Math.hypot(from.x - listener.x, from.y - listener.y)
  if (distance >= range) return 0
  return 1 - distance / range
}

/// Left/right placement, -1 to 1, so a shot off-screen still has a side.
export function panOf(from: Vec2, listener: Vec2, width = 600): number {
  const dx = from.x - listener.x
  return Math.min(1, Math.max(-1, dx / width))
}

export class AudioEngine {
  settings: AudioSettings
  /// What was asked to play, for tests that never open a speaker.
  played: PlayedSound[] = []
  loops = new Map<ClipId, PlayedSound>()
  /// Seconds of deafness left, from a nearby blast.
  deafness = 0
  lastListener: Vec2 = { x: 0, y: 0 }

  constructor(settings: AudioSettings = defaultAudioSettings()) {
    this.settings = { ...settings }
  }

  setMaster(volume: number) {
    this.settings.master = clampVolume(volume)
  }

  setMusic(volume: number) {
    this.settings.music = clampVolume(volume)
  }

  setQuality(quality: SoundQuality) {
    this.settings.quality = quality
  }

  setDevice(device: string | null) {
    this.settings.device = device
  }

  /// A nearby explosion takes hearing away and leaves a whistle as it comes back.
  /** Acceptance evidence: web:audio:engine */
  deafen(seconds: number) {
    if (!this.settings.explosionEffect) return
    if (!Number.isFinite(seconds) || seconds <= 0) return
    this.deafness = Math.max(this.deafness, seconds)
  }

  step(dt: number) {
    if (!Number.isFinite(dt) || dt <= 0) return
    this.deafness = Math.max(0, this.deafness - dt)
  }

  /// Plays the clip for a named simulation event, or nothing if the name is unknown.
  hear(
    event: string,
    options: { at?: Vec2; listener?: Vec2; loop?: boolean; weapon?: string } = {},
  ): PlayedSound | null {
    const clip = this.clipForEvent(event, options.weapon)
    if (!clip) return null
    return this.play(clip.id, options)
  }

  play(id: ClipId, options: { at?: Vec2; listener?: Vec2; loop?: boolean } = {}): PlayedSound | null {
    const clip = CLIPS[id]
    if (!clip) return null
    const listener = options.listener ?? this.lastListener
    if (options.listener) this.lastListener = options.listener
    const spatial = options.at ? attenuation(options.at, listener) : 1
    if (spatial <= 0) return null
    const deaf = this.deafness > 0 ? 0.08 : 1
    const quality = this.settings.quality === 'low' ? 0.7 : 1
    const volume = clampVolume(this.settings.master * spatial * deaf * quality)
    if (volume <= 0) return null
    const sound: PlayedSound = {
      clip: id,
      volume,
      pan: options.at ? panOf(options.at, listener) : 0,
      loop: options.loop === true || clip.kind === 'loop',
      hz: clip.hz,
    }
    this.played.push(sound)
    if (sound.loop) this.loops.set(id, sound)
    if (id === 'explosion' && options.at && spatial > 0.55) this.deafen(1.6 * spatial)
    return sound
  }

  stop(id: ClipId) {
    this.loops.delete(id)
  }

  clear() {
    this.played = []
    this.loops.clear()
    this.deafness = 0
  }

  clipForEvent(event: string, weapon?: string): Clip | null {
    if (weapon === 'Chainsaw' && event === 'Shot') return CLIPS.chainsaw
    if (weapon === 'Combat Knife' && event === 'Shot') return CLIPS.knife
    if ((weapon === 'Punch' || weapon === 'Fist') && event === 'Shot') return CLIPS.punch
    if ((weapon === 'Flamethrower' || weapon === 'Flame God') && event === 'Shot') return CLIPS.flame
    return clipFor(event)
  }
}
