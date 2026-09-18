// What the room browser shows, and how a player narrows it.
//
// Ping is measured here from a round-trip the client started. The listing itself is just columns.

export type LobbyRoom = {
  id: number
  name: string
  mode: string
  players: number
  capacity: number
  map: string
  weapon_mod?: string
  modifiers?: { realistic?: boolean; survival?: boolean; advance?: boolean }
  ruleset?: string | null
  password?: boolean
  ping?: number
  required_mod?: string | null
  region?: string | null
}

export type LobbyFilter = {
  search: string
  hideFull: boolean
  hideEmpty: boolean
  favoritesOnly: boolean
  sort: 'name' | 'players' | 'ping' | 'map'
}

export function defaultFilter(): LobbyFilter {
  return { search: '', hideFull: false, hideEmpty: false, favoritesOnly: false, sort: 'players' }
}

export function filterRooms(rooms: LobbyRoom[], filter: LobbyFilter, favorites: string[]): LobbyRoom[] {
  const needle = filter.search.trim().toLowerCase()
  const rows = rooms.filter(room => {
    if (filter.hideFull && room.players >= room.capacity) return false
    if (filter.hideEmpty && room.players === 0) return false
    if (filter.favoritesOnly && !favorites.includes(room.name) && !favorites.includes(String(room.id))) return false
    if (!needle) return true
    return [room.name, room.map, room.mode, room.region ?? ''].some(field => field.toLowerCase().includes(needle))
  })
  return sortRooms(rows, filter.sort)
}

export function sortRooms(rooms: LobbyRoom[], key: LobbyFilter['sort']): LobbyRoom[] {
  return [...rooms].sort((a, b) => {
    if (key === 'players') return b.players - a.players || a.name.localeCompare(b.name)
    if (key === 'ping') return (a.ping ?? 9999) - (b.ping ?? 9999)
    if (key === 'map') return a.map.localeCompare(b.map)
    return a.name.localeCompare(b.name)
  })
}

export function roomFlags(room: LobbyRoom): string[] {
  const flags: string[] = []
  if (room.password) flags.push('password')
  if (room.modifiers?.realistic) flags.push('realistic')
  if (room.modifiers?.survival) flags.push('survival')
  if (room.modifiers?.advance) flags.push('advance')
  if (room.weapon_mod) flags.push('mod')
  if (room.required_mod) flags.push('required-mod')
  if (room.region) flags.push(room.region)
  if (room.players >= room.capacity) flags.push('full')
  return flags
}

export function parseHostPort(value: string): { host: string; port: number } | null {
  const trimmed = value.trim()
  if (!trimmed) return null
  const [host, portText] = trimmed.includes(']:')
    ? trimmed.replace(/^\[/, '').split(']:')
    : trimmed.split(':')
  if (!host) return null
  const port = portText ? Number(portText) : 3000
  if (!Number.isInteger(port) || port < 1 || port > 65535) return null
  return { host, port }
}

export class PingAll {
  samples = new Map<number, number>()

  record(id: number, ms: number) {
    if (!Number.isFinite(ms) || ms < 0) return
    this.samples.set(id, Math.round(ms))
  }

  apply(rooms: LobbyRoom[]): LobbyRoom[] {
    return rooms.map(room => ({ ...room, ping: this.samples.get(room.id) ?? room.ping }))
  }
}
