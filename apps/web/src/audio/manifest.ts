// Which clip plays for which simulation event.
//
// The engine never names a file. It names a clip; this table is the only place a path lives, so a
// mod can re-point a sound without touching a line of mixing code.

export type ClipId =
  | 'fire'
  | 'reload'
  | 'empty'
  | 'grenade-pin'
  | 'grenade-throw'
  | 'grenade-bounce'
  | 'explosion'
  | 'impact'
  | 'ricochet'
  | 'chainsaw'
  | 'knife'
  | 'punch'
  | 'flame'
  | 'pain'
  | 'death'
  | 'gore'
  | 'footstep'
  | 'jet'
  | 'flag'
  | 'kit'
  | 'bonus'
  | 'ui'
  | 'chat'
  | 'distant'
  | 'weather'
  | 'whistle'

export type Clip = {
  id: ClipId
  /// Generated tone description used when no file is loaded. Tests assert on this, not on audio.
  kind: 'shot' | 'loop' | 'thud' | 'tone' | 'noise' | 'click' | 'whoosh'
  hz: number
  seconds: number
}

export const CLIPS: Record<ClipId, Clip> = {
  fire: { id: 'fire', kind: 'shot', hz: 220, seconds: 0.08 },
  reload: { id: 'reload', kind: 'thud', hz: 140, seconds: 0.2 },
  empty: { id: 'empty', kind: 'click', hz: 80, seconds: 0.06 },
  'grenade-pin': { id: 'grenade-pin', kind: 'tone', hz: 900, seconds: 0.05 },
  'grenade-throw': { id: 'grenade-throw', kind: 'whoosh', hz: 180, seconds: 0.12 },
  'grenade-bounce': { id: 'grenade-bounce', kind: 'thud', hz: 110, seconds: 0.08 },
  explosion: { id: 'explosion', kind: 'noise', hz: 60, seconds: 0.45 },
  impact: { id: 'impact', kind: 'thud', hz: 200, seconds: 0.06 },
  ricochet: { id: 'ricochet', kind: 'tone', hz: 1400, seconds: 0.1 },
  chainsaw: { id: 'chainsaw', kind: 'loop', hz: 90, seconds: 0.3 },
  knife: { id: 'knife', kind: 'shot', hz: 400, seconds: 0.07 },
  punch: { id: 'punch', kind: 'thud', hz: 90, seconds: 0.08 },
  flame: { id: 'flame', kind: 'loop', hz: 70, seconds: 0.3 },
  pain: { id: 'pain', kind: 'tone', hz: 260, seconds: 0.15 },
  death: { id: 'death', kind: 'noise', hz: 80, seconds: 0.35 },
  gore: { id: 'gore', kind: 'noise', hz: 70, seconds: 0.2 },
  footstep: { id: 'footstep', kind: 'thud', hz: 100, seconds: 0.05 },
  jet: { id: 'jet', kind: 'loop', hz: 160, seconds: 0.2 },
  flag: { id: 'flag', kind: 'tone', hz: 520, seconds: 0.18 },
  kit: { id: 'kit', kind: 'tone', hz: 640, seconds: 0.16 },
  bonus: { id: 'bonus', kind: 'tone', hz: 480, seconds: 0.22 },
  ui: { id: 'ui', kind: 'tone', hz: 720, seconds: 0.05 },
  chat: { id: 'chat', kind: 'tone', hz: 880, seconds: 0.06 },
  distant: { id: 'distant', kind: 'noise', hz: 50, seconds: 0.8 },
  weather: { id: 'weather', kind: 'loop', hz: 40, seconds: 0.4 },
  whistle: { id: 'whistle', kind: 'tone', hz: 2100, seconds: 0.9 },
}

export type SoundEventName =
  | 'Shot'
  | 'Reload'
  | 'Empty'
  | 'Explosion'
  | 'Impact'
  | 'MuzzleFlash'
  | 'Casing'
  | 'Blood'
  | 'Gibs'
  | 'Kill'
  | 'Damage'
  | 'KitTaken'
  | 'BonusExpired'
  | 'KitSpawned'
  | 'Chat'
  | 'Ui'

const EVENT_CLIPS: Record<SoundEventName, ClipId> = {
  Shot: 'fire',
  Reload: 'reload',
  Empty: 'empty',
  Explosion: 'explosion',
  Impact: 'impact',
  MuzzleFlash: 'fire',
  Casing: 'impact',
  Blood: 'pain',
  Gibs: 'gore',
  Kill: 'death',
  Damage: 'pain',
  KitTaken: 'kit',
  BonusExpired: 'bonus',
  KitSpawned: 'kit',
  Chat: 'chat',
  Ui: 'ui',
}

export function clipFor(event: string): Clip | null {
  const id = EVENT_CLIPS[event as SoundEventName]
  return id ? CLIPS[id] : null
}
