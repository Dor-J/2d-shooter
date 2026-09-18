/**
 * Reading the authoritative scoreboard and weapon statistics for the HUD.
 *
 * Every number here came from the server's one ledger; nothing is counted client-side, so the
 * scoreboard and the match that produced it can never disagree.
 */

export type WireWeaponStats = {
  shots: number
  hits: number
  kills: number
  deaths: number
  headshots: number
}

export type WirePlayerStats = {
  shots: number
  hits: number
  headshots: number
  captures: number
  returns: number
  holds: number
  weapons?: Record<string, WireWeaponStats>
  deaths_by_cause?: Record<string, number>
}

export type WirePlayerScore = {
  points: number
  kills: number
  deaths: number
  teamkills: number
  suicides: number
  objectives: number
}

export type WireScoreboardRow = {
  player: number
  team: number
  rank: number
  score: WirePlayerScore
  stats: WirePlayerStats
  behind_leader: number
}

/** Hits as a whole-number percentage. No shots is no accuracy, not a flattering hundred. */
export function accuracyPercent(stats: { shots: number; hits: number } | undefined): number {
  if (!stats || stats.shots <= 0) return 0
  return Math.floor((stats.hits * 100) / stats.shots)
}

/** The weapon a player has killed most with, for the weapon-statistics panel. */
export function favouriteWeapon(stats: WirePlayerStats | undefined): string | null {
  const weapons = Object.entries(stats?.weapons ?? {})
  let best: [string, WireWeaponStats] | null = null
  for (const entry of weapons) {
    if (entry[1].kills <= 0) continue
    if (!best || entry[1].kills > best[1].kills) best = entry
  }
  return best ? best[0] : null
}

/** One row of the weapon-statistics panel, most kills first. */
export function weaponRows(stats: WirePlayerStats | undefined) {
  return Object.entries(stats?.weapons ?? {})
    .map(([weapon, entry]) => ({
      weapon,
      shots: entry.shots,
      hits: entry.hits,
      kills: entry.kills,
      deaths: entry.deaths,
      accuracy: accuracyPercent(entry),
    }))
    .sort((left, right) => right.kills - left.kills || left.weapon.localeCompare(right.weapon))
}

/** How far behind the leader a row is, in words. */
export function behindText(row: WireScoreboardRow): string {
  if (row.rank === 1) return 'Leading'
  const behind = row.behind_leader
  return behind === 1 ? '1 point behind' : `${behind} points behind`
}

/** The limit line at the top of the scoreboard: what it takes to win. */
export function limitText(limits: {
  kills: number
  points: number
  captures: number
  time_ticks: number
} | undefined): string {
  if (!limits) return ''
  const parts: string[] = []
  if (limits.kills > 0) parts.push(`${limits.kills} kills`)
  if (limits.points > 0) parts.push(`${limits.points} points`)
  if (limits.captures > 0) parts.push(`${limits.captures} captures`)
  if (limits.time_ticks > 0) parts.push(`${Math.round(limits.time_ticks / 60 / 60)} minutes`)
  return parts.length > 0 ? `First to ${parts.join(' or ')}` : 'No limit'
}

/** The end-of-round summary line for one player. */
export function summaryLine(row: WireScoreboardRow, name: string): string {
  const accuracy = accuracyPercent(row.stats)
  return `${row.rank}. ${name} — ${row.score.points} pts, ${row.score.kills}/${row.score.deaths}, ${accuracy}% accuracy`
}
