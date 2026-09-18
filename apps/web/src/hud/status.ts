// The counters across the top of the screen: where you stand, where the teams stand, and what is
// happening to the objectives.
//
// Everything here is derived from a snapshot the server sent. Nothing is tallied on the client, so
// the HUD can never disagree with the scoreboard.

import { teamColor, teamName, type Rgba } from './gauges.ts'

export type WireStanding = {
  id: number
  name: string
  kills: number
  deaths: number
  points?: number
  team?: number
  /// How far behind the leader this player is; zero means they are the leader.
  behind?: number
}

export type MatchLimits = {
  kills?: number | null
  points?: number | null
  captures?: number | null
  time?: number | null
}

/// Where a player stands in the match.
export type PlayerStanding = {
  /// 1 for first place.
  rank: number
  score: number
  deaths: number
  /// How many points behind the leader, or null when this player is the leader.
  behind: number | null
  text: string
}

/// The rank a player holds and how far off the lead they are.
///
/// Ranking is by the score the mode counts, with deaths breaking ties, so a player who is level on
/// kills but has died less is ahead — the same order the scoreboard uses.
export function standingFor(rows: WireStanding[], id: number): PlayerStanding | null {
  const ordered = rankedRows(rows)
  const index = ordered.findIndex(row => row.id === id)
  if (index < 0) return null
  const row = ordered[index]
  const score = scoreOf(row)
  const lead = scoreOf(ordered[0])
  const behind = index === 0 ? null : lead - score
  return {
    rank: index + 1,
    score,
    deaths: row.deaths,
    behind,
    text: behind === null ? `1st · ${score}` : `${ordinal(index + 1)} · ${score} (−${behind})`,
  }
}

function scoreOf(row: WireStanding | undefined): number {
  if (!row) return 0
  return row.points ?? row.kills ?? 0
}

export function rankedRows<T extends WireStanding>(rows: T[]): T[] {
  return [...rows].sort((a, b) => scoreOf(b) - scoreOf(a) || a.deaths - b.deaths || a.id - b.id)
}

export function ordinal(rank: number): string {
  const tens = rank % 100
  if (tens >= 11 && tens <= 13) return `${rank}th`
  const suffix = { 1: 'st', 2: 'nd', 3: 'rd' }[rank % 10] ?? 'th'
  return `${rank}${suffix}`
}

/// The one line that says what ends this match.
export function limitLine(limits: MatchLimits | undefined): string {
  if (!limits) return ''
  const parts: string[] = []
  if (limits.captures) parts.push(`${limits.captures} captures`)
  if (limits.points) parts.push(`${limits.points} points`)
  if (limits.kills) parts.push(`${limits.kills} kills`)
  if (limits.time) parts.push(`${Math.round(limits.time / 60)} min`)
  return parts.length > 0 ? `First to ${parts.join(' · ')}` : 'No limit'
}

export type TeamScore = {
  team: number
  name: string
  score: number
  color: Rgba
  /// Whether this team is the one the local player is on.
  own: boolean
  leading: boolean
}

/// The team counters, in team order, for however many teams this mode runs.
///
/// `scores` is the server's array with index 0 unused, exactly as it arrives, so a mode with two
/// teams shows two and one with four shows four without a branch per mode.
export function teamScores(scores: number[] | undefined, teams: number, own?: number): TeamScore[] {
  const counted = Math.min(4, Math.max(0, Math.trunc(teams)))
  const rows: TeamScore[] = []
  let best = -Infinity
  for (let team = 1; team <= counted; team += 1) {
    best = Math.max(best, scores?.[team] ?? 0)
  }
  for (let team = 1; team <= counted; team += 1) {
    const score = scores?.[team] ?? 0
    rows.push({
      team,
      name: teamName(team),
      score,
      color: teamColor(team),
      own: own === team,
      leading: score === best && counted > 0,
    })
  }
  return rows
}

export type FlagIndicator = {
  flag: string
  /// 'home', 'taken' (an enemy has it), 'carried' (your side has it), or 'dropped'.
  state: 'home' | 'taken' | 'carried' | 'dropped'
  /// Seconds until a dropped flag returns itself, when one is running.
  returnIn: number | null
  color: Rgba
  text: string
}

type FlagLike = {
  kind: string
  state: { Base?: unknown; Carried?: { by: number } | number; Dropped?: { ticks_left?: number } } | string
  team?: number
}

/// What the flag row at the top of the screen says.
///
/// A flag that is not at home is the single most important thing on a CTF screen, so it is stated
/// plainly rather than left to a small icon.
export function flagIndicators(flags: FlagLike[], ownTeam: number | undefined): FlagIndicator[] {
  return flags.map(flag => {
    const team = flag.team ?? (flag.kind === 'Alpha' ? 1 : flag.kind === 'Bravo' ? 2 : 0)
    const mine = ownTeam !== undefined && team === ownTeam
    const raw = flag.state
    const carried = typeof raw === 'object' && raw !== null ? raw.Carried : undefined
    const dropped = typeof raw === 'object' && raw !== null ? raw.Dropped : undefined
    let state: FlagIndicator['state'] = 'home'
    if (carried !== undefined) state = mine ? 'taken' : 'carried'
    else if (dropped !== undefined) state = 'dropped'
    const ticks = dropped && typeof dropped.ticks_left === 'number' ? dropped.ticks_left : null
    const returnIn = ticks === null ? null : Math.max(0, Math.ceil(ticks / 60))
    const text =
      state === 'home'
        ? `${flag.kind} flag home`
        : state === 'taken'
          ? `${flag.kind} flag stolen`
          : state === 'carried'
            ? `${flag.kind} flag taken`
            : `${flag.kind} flag dropped${returnIn === null ? '' : ` · ${returnIn}s`}`
    return { flag: flag.kind, state, returnIn, color: teamColor(team), text }
  })
}

/// Whether the local player is the one carrying something, which the HUD calls out in the middle of
/// the screen rather than in a corner.
export function carrierBanner(indicators: FlagIndicator[], carrying: boolean): string | null {
  if (!carrying) return null
  const stolen = indicators.find(indicator => indicator.state === 'carried')
  if (stolen) return `You have the ${stolen.flag} flag — take it home`
  return carrying ? 'You are Rambo — only your kills count' : 'You have the objective'
}

/// The respawn clock, in whole seconds, as a line rather than a bare number.
export function respawnLine(ticks: number, protectedTicks = 0): string | null {
  if (ticks > 0) return `Respawning in ${Math.ceil(ticks / 60)}`
  if (protectedTicks > 0) return `Protected for ${Math.ceil(protectedTicks / 60)}`
  return null
}

export type WeaponPanel = {
  primary: string
  secondary: string | null
  grenades: number
  grenadeKind: string
  /// The name of the sprite the weapon panel draws, which is also the sprite the soldier holds.
  sprite: string
}

const GRENADE_NAMES: Record<string, string> = {
  frag: 'Frag',
  cluster: 'Cluster',
}

/// The weapon corner: what is in your hands, what is on your back, and what you are throwing.
export function weaponPanel(
  names: string[],
  primary: number,
  secondary: number | null,
  grenades: number,
  clusters = 0,
): WeaponPanel {
  const name = names[primary] ?? 'Unarmed'
  return {
    primary: name,
    secondary: secondary === null || secondary === undefined ? null : (names[secondary] ?? null),
    grenades: clusters > 0 ? clusters : Math.max(0, Math.trunc(grenades)),
    grenadeKind: clusters > 0 ? GRENADE_NAMES.cluster : GRENADE_NAMES.frag,
    sprite: weaponSprite(name),
  }
}

/// The sprite id for a weapon name, which is the lower-case name with the spaces taken out.
export function weaponSprite(name: string): string {
  return `weapon-${name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '')}`
}

export type ServerMessage = {
  text: string
  tone: 'server' | 'team' | 'all'
  at: number
}

/// A short queue of server announcements and chat, oldest first.
///
/// Team chat is kept apart from open chat by tone rather than by a prefix somebody could type, so
/// it can never be faked by a player writing "(team)" in their own message.
export class MessageLog {
  messages: ServerMessage[] = []
  limit: number
  ttl: number

  constructor({ limit = 6, ttl = 12000 }: { limit?: number; ttl?: number } = {}) {
    this.limit = limit
    this.ttl = ttl
  }

  push(text: string, tone: ServerMessage['tone'], at: number) {
    if (!text) return
    this.messages.push({ text, tone, at })
    if (this.messages.length > this.limit) this.messages.splice(0, this.messages.length - this.limit)
  }

  /// The messages still worth showing at this moment.
  visible(now: number): ServerMessage[] {
    return this.messages.filter(message => now - message.at < this.ttl)
  }

  clear() {
    this.messages = []
  }
}
