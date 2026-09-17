export type DamageCause =
  | 'bullet'
  | 'pellet'
  | 'explosion'
  | 'melee'
  | 'fall'
  | 'bleeding'
  | 'deadly'

/** Mirrors `game_core::KillFeedEntry`; the client renders it and never recomputes it. */
export type KillFeedEntry = {
  killer: number | null
  target: number
  cause: DamageCause
  region: string
  headshot: boolean
  teamkill: boolean
  suicide: boolean
  multi: number
  assists: number[]
}

export type FeedLine = {
  id: number
  text: string
  tone: 'kill' | 'teamkill' | 'suicide'
  at: number
}

const KILL_VERBS: Record<DamageCause, string> = {
  bullet: 'shot',
  pellet: 'shot',
  explosion: 'blew up',
  melee: 'cut down',
  fall: 'dropped',
  bleeding: 'bled out',
  deadly: 'finished off',
}

const SELF_VERBS: Record<DamageCause, string> = {
  bullet: 'killed themselves',
  pellet: 'killed themselves',
  explosion: 'blew themselves up',
  melee: 'killed themselves',
  fall: 'fell to their death',
  bleeding: 'bled out',
  deadly: 'found the deadly ground',
}

const MULTI_LABELS = ['', '', 'double kill', 'triple kill', 'multi kill']

export function multiKillLabel(multi: number): string {
  if (multi < 2) return ''
  return MULTI_LABELS[multi] ?? 'rampage'
}

export function describeKill(entry: KillFeedEntry, name: (id: number) => string): string {
  const target = name(entry.target)
  if (entry.killer === null) {
    return `${target} ${SELF_VERBS[entry.cause] ?? 'died'}`
  }
  const killer = name(entry.killer)
  let line = entry.teamkill
    ? `${killer} team-killed ${target}`
    : entry.headshot
      ? `${killer} headshot ${target}`
      : `${killer} ${KILL_VERBS[entry.cause] ?? 'killed'} ${target}`
  const notes: string[] = []
  const multi = multiKillLabel(entry.multi)
  if (multi) notes.push(multi)
  if (entry.assists.length > 0) {
    notes.push(`assist: ${entry.assists.map(name).join(', ')}`)
  }
  if (notes.length > 0) line += ` (${notes.join(', ')})`
  return line
}

export type KillFeedOptions = { lifetimeMs?: number; limit?: number }

/** Holds the recent kill lines, dropping them by age and by count. */
export class KillFeed {
  lifetimeMs: number
  limit: number
  #lines: FeedLine[] = []
  #next = 1

  constructor({ lifetimeMs = 7000, limit = 6 }: KillFeedOptions = {}) {
    this.lifetimeMs = lifetimeMs
    this.limit = limit
  }

  push(entries: KillFeedEntry[], name: (id: number) => string, now: number) {
    for (const entry of entries) {
      this.#lines.push({
        id: this.#next++,
        text: describeKill(entry, name),
        tone: entry.teamkill ? 'teamkill' : entry.killer === null ? 'suicide' : 'kill',
        at: now,
      })
    }
    if (this.#lines.length > this.limit) this.#lines = this.#lines.slice(-this.limit)
  }

  visible(now: number): FeedLine[] {
    this.#lines = this.#lines.filter(line => now - line.at < this.lifetimeMs)
    return this.#lines
  }

  clear() {
    this.#lines = []
  }
}

/** Whole seconds left before respawn, or an empty string once the player is alive. */
export function respawnCountdown(ticks: number): string {
  if (!Number.isFinite(ticks) || ticks <= 0) return ''
  return String(Math.ceil(ticks / 60))
}

/**
 * Screen angle in degrees for the damage indicator. The stored direction is the way the hit pushed
 * the player, so the arrow points back the other way, at where the shot came from.
 */
export function damageArrowAngle(direction: { x: number; y: number }): number | null {
  if (!direction || (direction.x === 0 && direction.y === 0)) return null
  const degrees = (Math.atan2(-direction.y, -direction.x) * 180) / Math.PI
  return Math.round(((degrees % 360) + 360) % 360)
}
