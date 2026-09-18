// Client-side prediction and remote interpolation.
//
// The server is still the authority. This only walks the local player forward from the last
// acknowledged snapshot and slides everyone else between the two snapshots we already have.

export type Vec2 = { x: number; y: number }

export type PredictedPlayer = {
  id: number
  pos: Vec2
  vel: Vec2
  last_seq: number
}

export type InputFrame = {
  seq: number
  left: boolean
  right: boolean
  jump: boolean
  jet: boolean
  aim: Vec2
}

const SPEED = 5.4

/// Replays unacknowledged inputs onto the last server copy of the local player.
export function reconcile<T extends PredictedPlayer>(
  acknowledged: T,
  inputs: InputFrame[],
  dt = 1 / 60,
): T {
  const player = { ...acknowledged, pos: { ...acknowledged.pos }, vel: { ...acknowledged.vel } }
  for (const input of inputs) {
    if (input.seq <= acknowledged.last_seq) continue
    const dir = (input.right ? 1 : 0) - (input.left ? 1 : 0)
    player.vel = { x: dir * SPEED * 60, y: player.vel.y + (input.jet ? -18 : 18) }
    player.pos = { x: player.pos.x + player.vel.x * dt, y: player.pos.y + player.vel.y * dt }
    player.last_seq = input.seq
  }
  return player
}

/// Slides a remote body between two snapshots. Extrapolation is capped so a stalled packet does
/// not send somebody through a wall.
export function interpolate(from: Vec2, to: Vec2, t: number, max = 1): Vec2 {
  const blend = Math.min(max, Math.max(0, t))
  return {
    x: from.x + (to.x - from.x) * blend,
    y: from.y + (to.y - from.y) * blend,
  }
}

export function extrapolate(pos: Vec2, vel: Vec2, dt: number, maxDt = 0.12): Vec2 {
  const time = Math.min(maxDt, Math.max(0, dt))
  return { x: pos.x + vel.x * time, y: pos.y + vel.y * time }
}

export type Clock = {
  offset: number
  samples: number
}

/// A one-way estimate of how far the server clock is ahead, from a ping/pong pair.
export function syncClock(clock: Clock, clientSent: number, serverAt: number, clientRecv: number): Clock {
  const rtt = clientRecv - clientSent
  if (!Number.isFinite(rtt) || rtt < 0 || rtt > 2000) return clock
  const offset = serverAt + rtt / 2 - clientRecv
  const samples = clock.samples + 1
  return {
    offset: clock.offset + (offset - clock.offset) / samples,
    samples,
  }
}

export class InputRing {
  frames: InputFrame[] = []
  limit = 120

  push(frame: InputFrame) {
    this.frames.push(frame)
    if (this.frames.length > this.limit) this.frames.splice(0, this.frames.length - this.limit)
  }

  dropThrough(seq: number) {
    this.frames = this.frames.filter(frame => frame.seq > seq)
  }
}

export type DisconnectReason =
  | 'version_mismatch'
  | 'kicked'
  | 'banned'
  | 'room_full'
  | 'timeout'
  | 'duplicate'
  | 'server'

export function disconnectText(reason: DisconnectReason): string {
  return {
    version_mismatch: 'This client is on the wrong version.',
    kicked: 'You were removed from the room.',
    banned: 'You are not allowed on this server.',
    room_full: 'That room is full.',
    timeout: 'The connection timed out.',
    duplicate: 'This name is already in the match.',
    server: 'The server closed the connection.',
  }[reason]
}
