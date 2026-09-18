// The screens that cover the match rather than sit beside it: the big scoreboard, the end-of-round
// summary, the weapon breakdown, and what a spectator sees instead of their own HUD.

import { teamColor, teamName, type Rgba } from './gauges.ts'
import { ordinal, rankedRows, type WireStanding } from './status.ts'

export type ScoreboardRow = {
  id: number
  name: string
  kills: number
  deaths: number
  points: number
  team: number
  color: Rgba
  rank: number
  /// Whether this is the local player's own row.
  own: boolean
  /// Whether this player is currently watching rather than playing.
  spectating: boolean
  ping: number
  ratio: string
}

export type ScoreboardPlayer = WireStanding & {
  team?: number
  ping?: number
  spectating?: boolean
}

/// Kills over deaths, as a scoreboard writes it: a player who has never died shows their kills
/// rather than an infinity.
export function killRatio(kills: number, deaths: number): string {
  if (deaths <= 0) return kills.toFixed(2)
  return (kills / deaths).toFixed(2)
}

/// Every row of the big scoreboard, ranked, with the local player marked.
export function scoreboardRows(players: ScoreboardPlayer[], self: number): ScoreboardRow[] {
  return rankedRows(players).map((player, index) => ({
    id: player.id,
    name: player.name,
    kills: player.kills,
    deaths: player.deaths,
    points: player.points ?? player.kills,
    team: player.team ?? 0,
    color: teamColor(player.team),
    rank: index + 1,
    own: player.id === self,
    spectating: player.spectating === true,
    ping: player.ping ?? 0,
    ratio: killRatio(player.kills, player.deaths),
  }))
}

/// The rows of one team, for a scoreboard that groups by side.
export function teamSections(rows: ScoreboardRow[], teams: number) {
  const sections = []
  for (let team = 1; team <= teams; team += 1) {
    const members = rows.filter(row => row.team === team)
    sections.push({
      team,
      name: teamName(team),
      color: teamColor(team),
      rows: members,
      score: members.reduce((sum, row) => sum + row.points, 0),
    })
  }
  const unassigned = rows.filter(row => row.team === 0 || row.team > teams)
  if (unassigned.length > 0) {
    sections.push({
      team: 0,
      name: teamName(0),
      color: teamColor(0),
      rows: unassigned,
      score: 0,
    })
  }
  return sections
}

/// The window of a scoreboard too long to fit.
///
/// The offset is clamped against the real length rather than trusted, so a player who holds the
/// scroll key down while people leave does not end up looking at an empty list.
export function scoreboardWindow<T>(rows: T[], offset: number, size: number) {
  const visible = Math.max(1, Math.trunc(size))
  const maxOffset = Math.max(0, rows.length - visible)
  const start = Math.min(maxOffset, Math.max(0, Math.trunc(offset)))
  return {
    rows: rows.slice(start, start + visible),
    start,
    more: Math.max(0, rows.length - start - visible),
    scrollable: rows.length > visible,
  }
}

/// Whether player IDs are shown beside names.
///
/// They are, when the player can actually use them — an admin typing a command needs the number,
/// and everybody else only needs the name.
export function showIds(canIssueCommands: boolean): boolean {
  return canIssueCommands
}

export function rowLabel(row: ScoreboardRow, withIds: boolean): string {
  const name = row.own ? `${row.name} (you)` : row.name
  return withIds ? `#${row.id} ${name}` : name
}

export type EndScreen = {
  title: string
  subtitle: string
  rows: ScoreboardRow[]
  /// The three best players, for the podium across the top.
  podium: ScoreboardRow[]
  nextIn: number | null
}

export type EndOfRound = {
  outcome: 'winner' | 'draw' | 'team' | 'ended'
  winner?: { name: string; team?: number }
  nextMapIn?: number
}

/// The end-of-round screen: who won, by how much, and how long until the next map.
export function endScreen(result: EndOfRound, rows: ScoreboardRow[]): EndScreen {
  const title =
    result.outcome === 'draw'
      ? 'Draw'
      : result.outcome === 'team'
        ? `${result.winner?.name ?? teamName(result.winner?.team)} wins`
        : result.outcome === 'winner'
          ? `${result.winner?.name ?? 'Nobody'} wins`
          : 'Round over'
  const leader = rows[0]
  const runnerUp = rows[1]
  const margin = leader && runnerUp ? leader.points - runnerUp.points : 0
  const subtitle =
    result.outcome === 'draw'
      ? 'Nobody could be separated'
      : margin > 0
        ? `by ${margin} ${margin === 1 ? 'point' : 'points'}`
        : 'on the tiebreak'
  return {
    title,
    subtitle,
    rows,
    podium: rows.slice(0, 3),
    nextIn: result.nextMapIn === undefined ? null : Math.max(0, Math.ceil(result.nextMapIn / 60)),
  }
}

export type WeaponRow = {
  weapon: string
  kills: number
  shots: number
  hits: number
  accuracy: number
  /// Where this weapon ranks among the player's own, best first.
  rank: number
}

type WireWeaponStat = { weapon: string; kills?: number; shots?: number; hits?: number }

/// The weapon-statistics screen, ordered by what actually worked.
export function weaponScreen(stats: WireWeaponStat[] | undefined): WeaponRow[] {
  const rows = (stats ?? []).map(stat => {
    const shots = stat.shots ?? 0
    const hits = stat.hits ?? 0
    return {
      weapon: stat.weapon,
      kills: stat.kills ?? 0,
      shots,
      hits,
      accuracy: shots > 0 ? Math.round((hits / shots) * 100) : 0,
      rank: 0,
    }
  })
  rows.sort((a, b) => b.kills - a.kills || b.accuracy - a.accuracy || a.weapon.localeCompare(b.weapon))
  return rows.map((row, index) => ({ ...row, rank: index + 1 }))
}

export type SpectatorHud = {
  /// Who the camera is on, or null for a free camera.
  following: { id: number; name: string; team: number } | null
  mode: 'follow' | 'free'
  /// What the bottom of the screen tells a spectator they can press.
  hint: string
  /// How far behind live this view is, in seconds, for a broadcast delay.
  delay: number
  /// Whether the followed player's own HUD — their health, their ammo — is mirrored.
  mirrorPlayerHud: boolean
}

type WireSpectatorState = {
  following?: number | null
  free?: boolean
  delay_ticks?: number
}

/// What a spectator sees instead of their own gauges.
///
/// A free camera never mirrors a player's HUD, because there is no player whose HUD it would be;
/// following one mirrors exactly what that player can see and nothing more.
export function spectatorHud(
  state: WireSpectatorState | undefined,
  players: { id: number; name: string; team?: number }[],
): SpectatorHud | null {
  if (!state) return null
  const free = state.free === true || state.following === null || state.following === undefined
  const target = free ? null : players.find(player => player.id === state.following)
  return {
    following: target ? { id: target.id, name: target.name, team: target.team ?? 0 } : null,
    mode: free ? 'free' : 'follow',
    hint: free
      ? 'Free camera · arrows to move · F to follow a player'
      : `Following ${target?.name ?? 'nobody'} · ← → to change · F for a free camera`,
    delay: Math.max(0, Math.round((state.delay_ticks ?? 0) / 60)),
    mirrorPlayerHud: !free && target !== undefined,
  }
}

/// The ordinal line under a podium place.
export function podiumLabel(row: ScoreboardRow): string {
  return `${ordinal(row.rank)} · ${row.name} · ${row.points}`
}
