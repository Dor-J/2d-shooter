// What the connection is doing: round-trip time, frame rate, and how much the link is carrying.
//
// These are the numbers a player blames when a shot does not land, so they are measured honestly:
// ping from acknowledged frames rather than a guess, frame rate from a rolling window rather than
// the last gap, and bandwidth from bytes actually seen.

import type { Rgba } from './gauges.ts'

/// How a connection is graded, worst-case first in the player's mind.
export type LinkGrade = 'good' | 'fair' | 'poor' | 'bad'

export const GRADE_COLORS: Record<LinkGrade, Rgba> = {
  good: [0.24, 0.78, 0.36, 1],
  fair: [0.86, 0.79, 0.28, 1],
  poor: [0.92, 0.55, 0.2, 1],
  bad: [0.92, 0.22, 0.2, 1],
}

/// The dot grows as well as reddens, because colour alone is not readable for every player.
export const GRADE_RADII: Record<LinkGrade, number> = { good: 3, fair: 4, poor: 5, bad: 6 }

export function gradeForPing(ms: number): LinkGrade {
  if (!Number.isFinite(ms) || ms < 0) return 'bad'
  if (ms < 60) return 'good'
  if (ms < 120) return 'fair'
  if (ms < 220) return 'poor'
  return 'bad'
}

export type PingDot = { grade: LinkGrade; color: Rgba; radius: number; text: string }

export function pingDot(ms: number): PingDot {
  const grade = gradeForPing(ms)
  const rounded = Number.isFinite(ms) && ms >= 0 ? Math.round(ms) : null
  return {
    grade,
    color: GRADE_COLORS[grade],
    radius: GRADE_RADII[grade],
    text: rounded === null ? '— ms' : `${rounded} ms`,
  }
}

/// A rolling measurement of round-trip time.
///
/// A single sample is noisy enough to make a stable connection look broken, so the reported figure
/// is a smoothed average while the worst recent sample is kept alongside it — a link that spikes
/// every second is not a good link, however good its average looks.
export class PingMeter {
  samples: number[] = []
  window: number

  constructor(window = 20) {
    this.window = Math.max(1, window)
  }

  /// One round trip, in milliseconds.
  sample(ms: number) {
    if (!Number.isFinite(ms) || ms < 0) return
    this.samples.push(ms)
    if (this.samples.length > this.window) this.samples.splice(0, this.samples.length - this.window)
  }

  get average(): number {
    if (this.samples.length === 0) return 0
    return this.samples.reduce((sum, value) => sum + value, 0) / this.samples.length
  }

  get worst(): number {
    return this.samples.length === 0 ? 0 : Math.max(...this.samples)
  }

  /// How much the link jitters, as the spread of recent samples.
  get jitter(): number {
    return this.samples.length < 2 ? 0 : this.worst - Math.min(...this.samples)
  }

  clear() {
    this.samples = []
  }
}

/// Frames per second over a rolling second, which is what a player means by "my FPS".
export class FrameMeter {
  times: number[] = []
  window: number

  constructor(windowMs = 1000) {
    this.window = windowMs
  }

  frame(now: number) {
    this.times.push(now)
    const cutoff = now - this.window
    while (this.times.length > 0 && this.times[0] < cutoff) this.times.shift()
  }

  get fps(): number {
    if (this.times.length < 2) return 0
    const span = this.times[this.times.length - 1] - this.times[0]
    if (span <= 0) return 0
    return Math.round(((this.times.length - 1) * 1000) / span)
  }

  /// The longest gap between frames in the window, which is where a stutter shows up.
  get worstFrameMs(): number {
    let worst = 0
    for (let i = 1; i < this.times.length; i += 1) worst = Math.max(worst, this.times[i] - this.times[i - 1])
    return Math.round(worst)
  }
}

/// Bytes in and out over a rolling second.
export class BandwidthMeter {
  entries: { at: number; down: number; up: number }[] = []
  window: number

  constructor(windowMs = 1000) {
    this.window = windowMs
  }

  received(bytes: number, at: number) {
    this.#add(bytes, 0, at)
  }

  sent(bytes: number, at: number) {
    this.#add(0, bytes, at)
  }

  #add(down: number, up: number, at: number) {
    if (!Number.isFinite(down) || !Number.isFinite(up)) return
    this.entries.push({ at, down, up })
    const cutoff = at - this.window
    while (this.entries.length > 0 && this.entries[0].at < cutoff) this.entries.shift()
  }

  rate(now: number): { down: number; up: number } {
    const cutoff = now - this.window
    let down = 0
    let up = 0
    for (const entry of this.entries) {
      if (entry.at < cutoff) continue
      down += entry.down
      up += entry.up
    }
    const seconds = this.window / 1000
    return { down: down / seconds, up: up / seconds }
  }
}

/// Bytes per second as a player would read them.
export function formatRate(bytesPerSecond: number): string {
  if (!Number.isFinite(bytesPerSecond) || bytesPerSecond <= 0) return '0 B/s'
  if (bytesPerSecond < 1024) return `${Math.round(bytesPerSecond)} B/s`
  if (bytesPerSecond < 1024 * 1024) return `${(bytesPerSecond / 1024).toFixed(1)} KB/s`
  return `${(bytesPerSecond / (1024 * 1024)).toFixed(2)} MB/s`
}

export type NetworkPanel = {
  ping: PingDot
  jitter: number
  fps: number
  worstFrameMs: number
  down: string
  up: string
}

/// The whole corner panel, in one call, so the HUD does not assemble it itself.
export function networkPanel(
  ping: PingMeter,
  frames: FrameMeter,
  bandwidth: BandwidthMeter,
  now: number,
): NetworkPanel {
  const rate = bandwidth.rate(now)
  return {
    ping: pingDot(ping.average),
    jitter: Math.round(ping.jitter),
    fps: frames.fps,
    worstFrameMs: frames.worstFrameMs,
    down: formatRate(rate.down),
    up: formatRate(rate.up),
  }
}
