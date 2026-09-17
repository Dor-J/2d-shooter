import type { Action } from './actions.ts'
import { type BindingSet, parseBindings, resetBindings, serializeBindings } from './bindings.ts'

export const PROFILE_VERSION = 1
export const PROFILES_KEY = 'arena-input-profiles'

export type ProfileSettings = {
  sensitivity: number
  volume: number
  musicVolume: number
  /** Hold controls the player prefers to latch instead of hold. */
  toggleActions: Action[]
}

export type ControlProfile = {
  version: number
  id: string
  name: string
  bindings: BindingSet
  settings: ProfileSettings
}

export const DEFAULT_PROFILE_SETTINGS: ProfileSettings = {
  sensitivity: 1,
  volume: 0.8,
  musicVolume: 0.5,
  toggleActions: [],
}

/** Controls that need two keys at once, each paired with the single control that replaces it. */
export const ACCESSIBILITY_COMBOS = [
  { keys: ['jump', 'crouch'], alternative: 'backflip', label: 'Backflip' },
  { keys: ['jump', 'crouch'], alternative: 'flagThrow', label: 'Throw flag' },
]

let counter = 0

function newId(): string {
  counter += 1
  return `profile-${Date.now().toString(36)}-${counter}-${Math.floor(Math.random() * 1e6).toString(36)}`
}

function readSettings(value: unknown): ProfileSettings {
  if (!value || typeof value !== 'object') return { ...DEFAULT_PROFILE_SETTINGS }
  const raw = value as Partial<ProfileSettings>
  const number = (input: unknown, fallback: number, min: number, max: number) =>
    typeof input === 'number' && Number.isFinite(input) ? Math.min(max, Math.max(min, input)) : fallback
  return {
    sensitivity: number(raw.sensitivity, DEFAULT_PROFILE_SETTINGS.sensitivity, 0.1, 5),
    volume: number(raw.volume, DEFAULT_PROFILE_SETTINGS.volume, 0, 1),
    musicVolume: number(raw.musicVolume, DEFAULT_PROFILE_SETTINGS.musicVolume, 0, 1),
    toggleActions: Array.isArray(raw.toggleActions) ? raw.toggleActions.filter(item => typeof item === 'string') : [],
  }
}

export function createProfile(name: string, bindings: BindingSet = resetBindings()): ControlProfile {
  return {
    version: PROFILE_VERSION,
    id: newId(),
    name: name.trim().slice(0, 24) || 'Default',
    bindings,
    settings: { ...DEFAULT_PROFILE_SETTINGS },
  }
}

/** Brings any stored profile shape, including pre-versioned ones, up to the current format. */
export function migrateProfile(raw: unknown): ControlProfile {
  if (!raw || typeof raw !== 'object') return createProfile('Default')
  const value = raw as Partial<ControlProfile> & { bindings?: unknown }
  const bindings = value.bindings
    ? parseBindings(serializeBindings(value.bindings as BindingSet))
    : resetBindings()
  return {
    version: PROFILE_VERSION,
    id: typeof value.id === 'string' && value.id ? value.id : newId(),
    name: typeof value.name === 'string' && value.name.trim() ? value.name.trim().slice(0, 24) : 'Default',
    bindings,
    settings: readSettings(value.settings),
  }
}

export function exportProfile(profile: ControlProfile): string {
  return JSON.stringify({ version: PROFILE_VERSION, profile }, null, 2)
}

/** Reads an exported profile, giving it a fresh id so importing never overwrites an existing one. */
export function importProfile(text: string): ControlProfile | null {
  let payload: unknown
  try {
    payload = JSON.parse(text)
  } catch {
    return null
  }
  if (!payload || typeof payload !== 'object') return null
  const candidate = (payload as { profile?: unknown }).profile ?? payload
  if (!candidate || typeof candidate !== 'object') return null
  if (typeof (candidate as { name?: unknown }).name !== 'string') return null
  return { ...migrateProfile(candidate), id: newId() }
}

type StorageLike = {
  getItem(key: string): string | null
  setItem(key: string, value: string): void
  removeItem(key: string): void
}

export class ProfileStore {
  #storage: StorageLike
  #profiles: ControlProfile[] = []
  #activeId = ''

  constructor(storage: StorageLike) {
    this.#storage = storage
    this.#load()
  }

  #load() {
    let payload: unknown
    try {
      payload = JSON.parse(this.#storage.getItem(PROFILES_KEY) ?? 'null')
    } catch {
      payload = null
    }
    const stored = payload && typeof payload === 'object' ? (payload as { profiles?: unknown; activeId?: unknown }) : {}
    const profiles = Array.isArray(stored.profiles) ? stored.profiles.map(migrateProfile) : []
    this.#profiles = profiles.length > 0 ? profiles : [createProfile('Default')]
    const active = typeof stored.activeId === 'string' ? stored.activeId : ''
    this.#activeId = this.#profiles.some(profile => profile.id === active) ? active : this.#profiles[0].id
    if (profiles.length === 0) this.#persist()
  }

  #persist() {
    this.#storage.setItem(
      PROFILES_KEY,
      JSON.stringify({ version: PROFILE_VERSION, activeId: this.#activeId, profiles: this.#profiles }),
    )
  }

  list(): ControlProfile[] {
    return this.#profiles.map(profile => ({ ...profile }))
  }

  get(id: string): ControlProfile {
    return this.#profiles.find(profile => profile.id === id) ?? this.active()
  }

  active(): ControlProfile {
    return this.get(this.#activeId)
  }

  activeId(): string {
    return this.#activeId
  }

  select(id: string): ControlProfile {
    if (this.#profiles.some(profile => profile.id === id)) {
      this.#activeId = id
      this.#persist()
    }
    return this.active()
  }

  create(name: string, bindings?: BindingSet): ControlProfile {
    const profile = createProfile(name, bindings ?? resetBindings())
    this.#profiles.push(profile)
    this.#persist()
    return profile
  }

  add(profile: ControlProfile): ControlProfile {
    this.#profiles.push(profile)
    this.#persist()
    return profile
  }

  update(id: string, change: (profile: ControlProfile) => ControlProfile): ControlProfile {
    const index = this.#profiles.findIndex(profile => profile.id === id)
    if (index < 0) return this.active()
    const next = { ...change(this.#profiles[index]), id, version: PROFILE_VERSION }
    this.#profiles[index] = next
    this.#persist()
    return next
  }

  remove(id: string): ControlProfile {
    this.#profiles = this.#profiles.filter(profile => profile.id !== id)
    if (this.#profiles.length === 0) this.#profiles = [createProfile('Default')]
    if (!this.#profiles.some(profile => profile.id === this.#activeId)) this.#activeId = this.#profiles[0].id
    this.#persist()
    return this.active()
  }
}
