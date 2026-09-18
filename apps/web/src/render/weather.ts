// Rain, snow, and the wind that blows them sideways.
//
// Weather is decoration and never touches the simulation, so it runs off the frame clock rather
// than the tick and is free to be switched off entirely on a slow machine.

export type WeatherKind = 'none' | 'rain' | 'snow'

export const WEATHER_KINDS: readonly WeatherKind[] = ['none', 'rain', 'snow']

export type WeatherDrop = {
  x: number
  y: number
  vx: number
  vy: number
  length: number
  alpha: number
}

export type WeatherSettings = {
  kind: WeatherKind
  /// 0 to 1; how much of the full drop count to run.
  intensity: number
  /// World units per second sideways. Negative blows left.
  wind: number
}

export function defaultWeather(): WeatherSettings {
  return { kind: 'none', intensity: 0.6, wind: 0 }
}

const FULL_DROPS = 320
const RAIN_SPEED = 900
const SNOW_SPEED = 90

export function dropCount(settings: WeatherSettings, particleBudget = 1): number {
  if (settings.kind === 'none') return 0
  const intensity = Math.min(1, Math.max(0, settings.intensity))
  return Math.round(FULL_DROPS * intensity * Math.min(1, Math.max(0, particleBudget)))
}

function seeded(seed: number): () => number {
  let state = (Math.trunc(seed) || 1) >>> 0
  return () => {
    state = (state + 0x6d2b79f5) >>> 0
    let t = state
    t = Math.imul(t ^ (t >>> 15), t | 1)
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61)
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296
  }
}

/// A field of drops spread over the view.
///
/// Seeded, so a screenshot taken twice of the same frame is the same picture — which is what makes
/// a visual regression test possible at all.
export function makeWeather(
  settings: WeatherSettings,
  view: { width: number; height: number },
  seed = 1,
  particleBudget = 1,
): WeatherDrop[] {
  const count = dropCount(settings, particleBudget)
  const random = seeded(seed)
  const drops: WeatherDrop[] = []
  const rain = settings.kind === 'rain'
  for (let i = 0; i < count; i += 1) {
    const speed = (rain ? RAIN_SPEED : SNOW_SPEED) * (0.7 + random() * 0.6)
    drops.push({
      x: random() * view.width,
      y: random() * view.height,
      vx: settings.wind * (rain ? 1 : 0.6 + random() * 0.8),
      vy: speed,
      length: rain ? 8 + random() * 10 : 2 + random() * 2,
      alpha: rain ? 0.25 + random() * 0.3 : 0.5 + random() * 0.4,
    })
  }
  return drops
}

/// Moves the drops on and wraps whatever left the view back into it, so the field never runs out.
///
/// Snow drifts on its own as well as with the wind, which is what tells the two apart at a glance.
export function stepWeather(
  drops: WeatherDrop[],
  dt: number,
  view: { width: number; height: number },
  settings: WeatherSettings,
  elapsed = 0,
) {
  if (!Number.isFinite(dt) || dt <= 0) return
  const drifting = settings.kind === 'snow'
  for (const drop of drops) {
    const drift = drifting ? Math.sin(elapsed * 1.7 + drop.y * 0.05) * 18 : 0
    drop.x += (drop.vx + drift) * dt
    drop.y += drop.vy * dt
    if (drop.y > view.height) {
      drop.y -= view.height
      drop.x = ((drop.x % view.width) + view.width) % view.width
    }
    if (drop.x > view.width) drop.x -= view.width
    if (drop.x < 0) drop.x += view.width
  }
}

/// How much the wind leans a drop, as an angle for drawing it.
export function windAngle(settings: WeatherSettings): number {
  const fall = settings.kind === 'snow' ? SNOW_SPEED : RAIN_SPEED
  return Math.atan2(settings.wind, fall)
}

/// Wind also pushes loose particles — smoke, casings, leaves — which is what makes a gale look like
/// one rather than like rain on a still day.
export function windForce(settings: WeatherSettings): number {
  return settings.kind === 'none' ? 0 : settings.wind
}
