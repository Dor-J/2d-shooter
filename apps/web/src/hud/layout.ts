// Where every HUD element sits, at any size of screen.
//
// The layout is data rather than CSS so that a custom interface can replace it, a test can assert
// on it without a browser, and the same description can drive the desktop page and the phone.

/// The corner or edge an element is measured from.
export type Anchor =
  | 'top-left'
  | 'top'
  | 'top-right'
  | 'left'
  | 'center'
  | 'right'
  | 'bottom-left'
  | 'bottom'
  | 'bottom-right'

/// One HUD element, in design units. A layout is written for `DESIGN_WIDTH` × `DESIGN_HEIGHT` and
/// scaled to whatever the screen turns out to be.
export type HudElement = {
  id: string
  anchor: Anchor
  /// Offset from the anchor, inward-positive: `x` moves right of a left anchor and left of a right
  /// one, so the same numbers mirror correctly without a sign per corner.
  x: number
  y: number
  width: number
  height: number
  /// Elements are drawn low to high; ties keep their declared order.
  layer?: number
  visible?: boolean
}

export type HudLayout = {
  name: string
  elements: HudElement[]
}

/// The size every layout is written against.
export const DESIGN_WIDTH = 1280
export const DESIGN_HEIGHT = 720

/// The insets a display reserves — a phone's notch, a TV's overscan. Nothing is placed inside them.
export type SafeArea = { top: number; right: number; bottom: number; left: number }

export const NO_INSETS: SafeArea = { top: 0, right: 0, bottom: 0, left: 0 }

export type Viewport = {
  width: number
  height: number
  safeArea?: SafeArea
  /// A player's own scaling preference, on top of whatever the screen size implies.
  scale?: number
}

/// A placed element, in screen pixels, ready to be drawn or asserted on.
export type PlacedElement = {
  id: string
  x: number
  y: number
  width: number
  height: number
  layer: number
}

/// Scaling is clamped, because a HUD that shrinks without limit is unreadable and one that grows
/// without limit swallows the match.
export const MIN_SCALE = 0.5
export const MAX_SCALE = 2

export function clampScale(scale: number): number {
  if (!Number.isFinite(scale)) return 1
  return Math.min(MAX_SCALE, Math.max(MIN_SCALE, scale))
}

/// How much to scale a design-sized layout to fit this viewport.
///
/// The smaller of the two ratios wins, so nothing is ever placed off the edge of a narrow screen.
export function scaleFor(viewport: Viewport): number {
  const safe = viewport.safeArea ?? NO_INSETS
  const width = Math.max(1, viewport.width - safe.left - safe.right)
  const height = Math.max(1, viewport.height - safe.top - safe.bottom)
  const fit = Math.min(width / DESIGN_WIDTH, height / DESIGN_HEIGHT)
  return clampScale(fit * clampScale(viewport.scale ?? 1))
}

/// Places one element on a real screen.
export function place(element: HudElement, viewport: Viewport): PlacedElement {
  const safe = viewport.safeArea ?? NO_INSETS
  const scale = scaleFor(viewport)
  const width = element.width * scale
  const height = element.height * scale
  const dx = element.x * scale
  const dy = element.y * scale
  const left = safe.left
  const right = viewport.width - safe.right
  const top = safe.top
  const bottom = viewport.height - safe.bottom

  let x: number
  if (element.anchor.includes('left')) x = left + dx
  else if (element.anchor.includes('right')) x = right - dx - width
  else x = (left + right) / 2 - width / 2 + dx

  let y: number
  if (element.anchor.includes('top')) y = top + dy
  else if (element.anchor.includes('bottom')) y = bottom - dy - height
  else y = (top + bottom) / 2 - height / 2 + dy

  return { id: element.id, x, y, width, height, layer: element.layer ?? 0 }
}

/// Places a whole layout, in draw order, leaving out anything switched off.
export function placeLayout(layout: HudLayout, viewport: Viewport): PlacedElement[] {
  return layout.elements
    .filter(element => element.visible !== false)
    .map((element, index) => ({ placed: place(element, viewport), index }))
    .sort((a, b) => a.placed.layer - b.placed.layer || a.index - b.index)
    .map(entry => entry.placed)
}

/// Whether a placed element sits entirely inside the safe area.
export function withinSafeArea(placed: PlacedElement, viewport: Viewport): boolean {
  const safe = viewport.safeArea ?? NO_INSETS
  const slack = 0.01
  return (
    placed.x >= safe.left - slack &&
    placed.y >= safe.top - slack &&
    placed.x + placed.width <= viewport.width - safe.right + slack &&
    placed.y + placed.height <= viewport.height - safe.bottom + slack
  )
}

/// The stock layout: everything the player needs, in the places a Soldat player expects it.
export function defaultLayout(): HudLayout {
  return {
    name: 'default',
    elements: [
      { id: 'health', anchor: 'bottom-left', x: 24, y: 24, width: 240, height: 18 },
      { id: 'jet', anchor: 'bottom-left', x: 24, y: 48, width: 240, height: 10 },
      { id: 'ammo', anchor: 'bottom-right', x: 24, y: 24, width: 220, height: 18 },
      { id: 'fire-interval', anchor: 'bottom-right', x: 24, y: 48, width: 220, height: 6 },
      { id: 'weapon', anchor: 'bottom-right', x: 24, y: 62, width: 220, height: 40 },
      { id: 'grenades', anchor: 'bottom-right', x: 252, y: 24, width: 64, height: 40 },
      { id: 'bonus', anchor: 'bottom', x: 0, y: 96, width: 260, height: 22 },
      { id: 'score', anchor: 'top-left', x: 24, y: 20, width: 280, height: 56 },
      { id: 'team-scores', anchor: 'top', x: 0, y: 16, width: 320, height: 48 },
      { id: 'flags', anchor: 'top', x: 0, y: 68, width: 320, height: 24 },
      { id: 'kill-feed', anchor: 'top-right', x: 24, y: 20, width: 380, height: 140 },
      { id: 'network', anchor: 'top-right', x: 24, y: 168, width: 160, height: 44 },
      { id: 'minimap', anchor: 'bottom-left', x: 24, y: 72, width: 220, height: 132 },
      { id: 'chat', anchor: 'bottom-left', x: 24, y: 216, width: 460, height: 120 },
      { id: 'notice', anchor: 'center', x: 0, y: -120, width: 520, height: 40 },
      { id: 'respawn', anchor: 'center', x: 0, y: 0, width: 360, height: 64, layer: 2 },
    ],
  }
}

/// The phone layout: fewer elements, further from the edges, out of the thumbs' way.
export function mobileLayout(): HudLayout {
  const hidden = new Set(['network', 'chat', 'minimap', 'fire-interval'])
  return {
    name: 'mobile',
    elements: defaultLayout().elements.map(element =>
      hidden.has(element.id) ? { ...element, visible: false } : element,
    ),
  }
}

/// The presets a player picks between in the settings screen.
export const HUD_PRESETS = ['default', 'mobile', 'minimal'] as const

export type HudPreset = (typeof HUD_PRESETS)[number]

/// The barest HUD a competitive player asks for: health, ammo, and nothing in the way.
export function minimalLayout(): HudLayout {
  const kept = new Set(['health', 'jet', 'ammo', 'weapon', 'respawn', 'notice'])
  return {
    name: 'minimal',
    elements: defaultLayout().elements.map(element =>
      kept.has(element.id) ? element : { ...element, visible: false },
    ),
  }
}

export function presetLayout(preset: HudPreset): HudLayout {
  if (preset === 'mobile') return mobileLayout()
  if (preset === 'minimal') return minimalLayout()
  return defaultLayout()
}

/// Why a custom layout was refused.
export type LayoutError = { element: string; reason: string }

const ANCHORS: readonly Anchor[] = [
  'top-left',
  'top',
  'top-right',
  'left',
  'center',
  'right',
  'bottom-left',
  'bottom',
  'bottom-right',
]

/// Reads a custom HUD supplied by a player or a mod.
///
/// Anything unrecognised is refused with a reason rather than silently dropped, because a HUD that
/// quietly loses the health bar is worse than one that fails to load.
export function readLayout(source: unknown): { layout: HudLayout } | { errors: LayoutError[] } {
  const errors: LayoutError[] = []
  const data = source as { name?: unknown; elements?: unknown }
  if (typeof data !== 'object' || data === null || !Array.isArray(data.elements)) {
    return { errors: [{ element: '(document)', reason: 'expected an object with an elements list' }] }
  }
  const known = new Set(defaultLayout().elements.map(element => element.id))
  const seen = new Set<string>()
  const elements: HudElement[] = []
  for (const raw of data.elements as unknown[]) {
    const entry = raw as Partial<HudElement>
    const id = typeof entry.id === 'string' ? entry.id : ''
    if (!id) {
      errors.push({ element: '(unnamed)', reason: 'every element needs an id' })
      continue
    }
    if (!known.has(id)) {
      errors.push({ element: id, reason: 'not a HUD element this game draws' })
      continue
    }
    if (seen.has(id)) {
      errors.push({ element: id, reason: 'listed twice' })
      continue
    }
    seen.add(id)
    if (entry.anchor !== undefined && !ANCHORS.includes(entry.anchor as Anchor)) {
      errors.push({ element: id, reason: `unknown anchor ${String(entry.anchor)}` })
      continue
    }
    const numbers = ['x', 'y', 'width', 'height'] as const
    const bad = numbers.find(key => entry[key] !== undefined && !Number.isFinite(entry[key]))
    if (bad) {
      errors.push({ element: id, reason: `${bad} must be a number` })
      continue
    }
    const fallback = defaultLayout().elements.find(element => element.id === id)!
    elements.push({
      ...fallback,
      ...(entry.anchor ? { anchor: entry.anchor as Anchor } : {}),
      ...Object.fromEntries(numbers.filter(key => entry[key] !== undefined).map(key => [key, entry[key]])),
      ...(entry.visible !== undefined ? { visible: Boolean(entry.visible) } : {}),
      ...(entry.layer !== undefined ? { layer: Number(entry.layer) } : {}),
      id,
    })
  }
  if (errors.length > 0) return { errors }
  // Anything the custom file left out keeps its stock place, so a partial HUD is a valid HUD.
  const merged = defaultLayout().elements.map(
    element => elements.find(custom => custom.id === element.id) ?? element,
  )
  return {
    layout: { name: typeof data.name === 'string' && data.name ? data.name : 'custom', elements: merged },
  }
}
