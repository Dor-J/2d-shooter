// The player, as they persist between matches: name, look, settings, and the servers they like.
//
// Gameplay-affecting values are validated here so a hand-edited save cannot smuggle a name the
// scoreboard cannot show or a sensitivity the aim code cannot use.

import { defaultAppearance, type Appearance } from '../render/gostek.ts'
import { defaultQuality, type QualitySettings } from '../render/quality.ts'
import { defaultAudioSettings, type AudioSettings } from '../audio/engine.ts'
import { type HudPreset } from '../hud/layout.ts'

export const PLAYER_PROFILE_VERSION = 2
export const PLAYER_PROFILES_KEY = 'arena-player-profiles'

export const NAME_MIN = 1
export const NAME_MAX = 20

export type TauntMap = {
  a: string
  b: string
  c: string
  '1': string
  '2': string
  '3': string
}

export const DEFAULT_TAUNTS: TauntMap = {
  a: 'Nice shot',
  b: 'Good game',
  c: 'Thanks',
  '1': 'Yes',
  '2': 'No',
  '3': 'Sorry',
}

export type PlayerProfile = {
  version: number
  id: string
  name: string
  appearance: Appearance
  secondary: number
  taunts: TauntMap
  interface: HudPreset
  sensitivity: number
  audio: AudioSettings
  graphics: QualitySettings
  favorites: string[]
}

export type NameError = 'empty' | 'too-long' | 'controls'

export function constrainName(raw: unknown): { name: string } | { error: NameError } {
  if (typeof raw !== 'string') return { error: 'empty' }
  let name = ''
  for (const ch of raw) {
    const code = ch.charCodeAt(0)
    if (code >= 32 && code !== 127) name += ch
  }
  name = name.trim()
  if (!name) return { error: 'empty' }
  if (name.length > NAME_MAX) return { error: 'too-long' }
  if (name !== raw.trim()) return { error: 'controls' }
  return { name }
}

function newId(): string {
  return `player-${Date.now().toString(36)}-${Math.floor(Math.random() * 1e6).toString(36)}`
}

export function createPlayer(name = 'Guest'): PlayerProfile {
  const checked = constrainName(name)
  return {
    version: PLAYER_PROFILE_VERSION,
    id: newId(),
    name: 'name' in checked ? checked.name : 'Guest',
    appearance: defaultAppearance(),
    secondary: 10,
    taunts: { ...DEFAULT_TAUNTS },
    interface: 'default',
    sensitivity: 1,
    audio: defaultAudioSettings(),
    graphics: defaultQuality(),
    favorites: [],
  }
}

export function migratePlayer(raw: unknown): PlayerProfile {
  const fallback = createPlayer()
  if (!raw || typeof raw !== 'object') return fallback
  const value = raw as Partial<PlayerProfile> & { settings?: { sensitivity?: number } }
  const name = constrainName(value.name)
  return {
    ...fallback,
    id: typeof value.id === 'string' && value.id ? value.id : fallback.id,
    name: 'name' in name ? name.name : fallback.name,
    appearance: { ...fallback.appearance, ...(value.appearance ?? {}) },
    secondary: Number.isFinite(value.secondary) ? Math.max(0, Math.trunc(value.secondary as number)) : 10,
    taunts: { ...DEFAULT_TAUNTS, ...(value.taunts ?? {}) },
    interface: value.interface === 'mobile' || value.interface === 'minimal' ? value.interface : 'default',
    sensitivity: Number.isFinite(value.sensitivity)
      ? Math.min(5, Math.max(0.1, value.sensitivity as number))
      : Number.isFinite(value.settings?.sensitivity)
        ? Math.min(5, Math.max(0.1, value.settings!.sensitivity!))
        : 1,
    audio: { ...fallback.audio, ...(value.audio ?? {}) },
    graphics: { ...fallback.graphics, ...(value.graphics ?? {}) },
    favorites: Array.isArray(value.favorites)
      ? value.favorites.filter((item): item is string => typeof item === 'string' && item.length > 0)
      : [],
  }
}

export function exportPlayer(profile: PlayerProfile): string {
  return JSON.stringify({ version: PLAYER_PROFILE_VERSION, profile }, null, 2)
}

export function importPlayer(text: string): PlayerProfile | null {
  try {
    const payload = JSON.parse(text)
    const candidate = payload?.profile ?? payload
    if (!candidate || typeof candidate !== 'object' || typeof candidate.name !== 'string') return null
    return { ...migratePlayer(candidate), id: newId() }
  } catch {
    return null
  }
}

export function toggleFavorite(favorites: string[], key: string): string[] {
  const id = key.trim()
  if (!id) return favorites
  return favorites.includes(id) ? favorites.filter(item => item !== id) : [...favorites, id]
}

type StorageLike = {
  getItem(key: string): string | null
  setItem(key: string, value: string): void
}

export class PlayerStore {
  #storage: StorageLike
  #profiles: PlayerProfile[] = []
  #activeId = ''

  constructor(storage: StorageLike) {
    this.#storage = storage
    this.#load()
  }

  #load() {
    let payload: unknown
    try {
      payload = JSON.parse(this.#storage.getItem(PLAYER_PROFILES_KEY) ?? 'null')
    } catch {
      payload = null
    }
    const stored = payload && typeof payload === 'object' ? (payload as { profiles?: unknown; activeId?: unknown }) : {}
    const profiles = Array.isArray(stored.profiles) ? stored.profiles.map(migratePlayer) : []
    this.#profiles = profiles.length > 0 ? profiles : [createPlayer()]
    const active = typeof stored.activeId === 'string' ? stored.activeId : ''
    this.#activeId = this.#profiles.some(profile => profile.id === active) ? active : this.#profiles[0].id
    if (profiles.length === 0) this.#persist()
  }

  #persist() {
    this.#storage.setItem(
      PLAYER_PROFILES_KEY,
      JSON.stringify({ version: PLAYER_PROFILE_VERSION, activeId: this.#activeId, profiles: this.#profiles }),
    )
  }

  list(): PlayerProfile[] {
    return this.#profiles.map(profile => ({ ...profile }))
  }

  active(): PlayerProfile {
    return this.#profiles.find(profile => profile.id === this.#activeId) ?? this.#profiles[0]
  }

  select(id: string): PlayerProfile {
    if (this.#profiles.some(profile => profile.id === id)) {
      this.#activeId = id
      this.#persist()
    }
    return this.active()
  }

  create(name: string): PlayerProfile {
    const profile = createPlayer(name)
    this.#profiles.push(profile)
    this.#persist()
    return profile
  }

  update(id: string, change: (profile: PlayerProfile) => PlayerProfile): PlayerProfile {
    const index = this.#profiles.findIndex(profile => profile.id === id)
    if (index < 0) return this.active()
    const next = { ...change(this.#profiles[index]), id, version: PLAYER_PROFILE_VERSION }
    const name = constrainName(next.name)
    if ('name' in name) next.name = name.name
    this.#profiles[index] = next
    this.#persist()
    return next
  }

  remove(id: string): PlayerProfile {
    this.#profiles = this.#profiles.filter(profile => profile.id !== id)
    if (this.#profiles.length === 0) this.#profiles = [createPlayer()]
    if (!this.#profiles.some(profile => profile.id === this.#activeId)) this.#activeId = this.#profiles[0].id
    this.#persist()
    return this.active()
  }
}
