// The soldier, in layers.
//
// A character is not one sprite but a rig: legs, torso, head, arms, the weapon in them, and the
// jets behind. Each part is placed relative to the soldier's own position and angle, so a pose is
// a description rather than a drawing, and a test can assert on the arm without a canvas.

export type Vec2 = { x: number; y: number }

export type Rgba = [number, number, number, number]

/// Every layer a soldier is made of, in the order they are drawn.
export const BODY_PARTS = [
  'jets',
  'back-leg',
  'back-arm',
  'torso',
  'front-leg',
  'head',
  'hair',
  'headgear',
  'chain',
  'front-arm',
  'weapon',
] as const

export type BodyPart = (typeof BODY_PARTS)[number]

/// What a player chose to look like.
export type Appearance = {
  shirt: Rgba
  pants: Rgba
  skin: Rgba
  hair: Rgba
  shoes: Rgba
  jet: Rgba
  hairStyle: HairStyle
  headgear: Headgear
  chain: Chain
}

export const HAIR_STYLES = ['dreadlocks', 'punk', 'mohawk', 'normal', 'flat', 'bald'] as const
export type HairStyle = (typeof HAIR_STYLES)[number]

export const HEADGEAR = ['none', 'helmet', 'hat'] as const
export type Headgear = (typeof HEADGEAR)[number]

export const CHAINS = ['none', 'gold', 'silver', 'dog-tags'] as const
export type Chain = (typeof CHAINS)[number]

export function defaultAppearance(): Appearance {
  return {
    shirt: [0.36, 0.42, 0.5, 1],
    pants: [0.24, 0.26, 0.3, 1],
    skin: [0.84, 0.68, 0.56, 1],
    hair: [0.24, 0.18, 0.14, 1],
    shoes: [0.16, 0.16, 0.18, 1],
    jet: [0.55, 0.58, 0.62, 1],
    hairStyle: 'normal',
    headgear: 'helmet',
    chain: 'none',
  }
}

/// The pose a soldier's body is in, which the simulation decides and the rig only draws.
export const POSES = ['stand', 'walk', 'run', 'crouch', 'prone', 'roll', 'jet', 'fall', 'dead', 'emote'] as const
export type Pose = (typeof POSES)[number]

export function readPose(state: string | undefined, grounded: boolean, hp: number): Pose {
  if (hp <= 0) return 'dead'
  const named = (state ?? '').toLowerCase()
  if (named.includes('emote') || named.includes('victory') || named.includes('mercy') || named.includes('cigar') || named.includes('taunt')) {
    return 'emote'
  }
  if (named.includes('prone')) return 'prone'
  if (named.includes('roll')) return 'roll'
  if (named.includes('crouch')) return 'crouch'
  if (!grounded) return 'fall'
  return 'stand'
}

/// How tall a soldier stands in each pose, which is also what the camera and the HUD measure from.
export const POSE_HEIGHT: Record<Pose, number> = {
  stand: 32,
  walk: 32,
  run: 32,
  crouch: 22,
  prone: 12,
  roll: 16,
  jet: 32,
  fall: 30,
  dead: 10,
  emote: 32,
}

export type Placement = {
  part: BodyPart
  /// Centre of the part in world coordinates.
  x: number
  y: number
  width: number
  height: number
  /// Radians, zero pointing right.
  angle: number
  color: Rgba
  /// Mirrored when the soldier faces left.
  flip: boolean
}

export type RigInput = {
  pos: Vec2
  aim: Vec2
  pose: Pose
  appearance: Appearance
  /// Tint from the team, which the shirt and the jets take on.
  team: Rgba
  /// Ticks of animation, so a walk cycle advances.
  tick: number
  /// Whether the jets are actually firing.
  jetting: boolean
  /// Whether this soldier is moving under their own legs, which makes the walk cycle run.
  moving: boolean
}

/// The angle from a soldier to what they are aiming at.
export function aimAngle(pos: Vec2, aim: Vec2): number {
  return Math.atan2(aim.y - pos.y, aim.x - pos.x)
}

/// Which way a soldier faces, from where they are aiming rather than where they are moving — a
/// player walking backwards while shooting keeps their gun pointed at the fight.
export function facing(pos: Vec2, aim: Vec2): 1 | -1 {
  return aim.x < pos.x ? -1 : 1
}

/// The arm's angle, which follows the aim but is limited: a soldier cannot fold an arm through
/// their own chest, so the aim is clamped to the half-circle they are facing.
export function armAngle(pos: Vec2, aim: Vec2): number {
  const angle = aimAngle(pos, aim)
  const side = facing(pos, aim)
  if (side === 1) return Math.max(-Math.PI / 2, Math.min(Math.PI / 2, angle))
  // Mirrored: the arm swings on the other side, so the clamp is around π.
  const mirrored = angle > 0 ? Math.PI - angle : -Math.PI - angle
  const clamped = Math.max(-Math.PI / 2, Math.min(Math.PI / 2, mirrored))
  return clamped > 0 ? Math.PI - clamped : -Math.PI - clamped
}

const WALK_PERIOD = 24

/// How far through a walk cycle a soldier is, from 0 to 1.
export function walkPhase(tick: number, moving: boolean): number {
  if (!moving) return 0
  return ((tick % WALK_PERIOD) + WALK_PERIOD) % WALK_PERIOD / WALK_PERIOD
}

function tinted(base: Rgba, team: Rgba, weight: number): Rgba {
  return [
    base[0] * (1 - weight) + team[0] * weight,
    base[1] * (1 - weight) + team[1] * weight,
    base[2] * (1 - weight) + team[2] * weight,
    base[3],
  ]
}

const HAIR_HEIGHT: Record<HairStyle, number> = {
  dreadlocks: 9,
  punk: 8,
  mohawk: 10,
  normal: 5,
  flat: 3,
  bald: 0,
}

const HEADGEAR_SIZE: Record<Headgear, { width: number; height: number }> = {
  none: { width: 0, height: 0 },
  helmet: { width: 13, height: 7 },
  hat: { width: 15, height: 5 },
}

const CHAIN_COLORS: Record<Chain, Rgba | null> = {
  none: null,
  gold: [0.86, 0.72, 0.26, 1],
  silver: [0.78, 0.8, 0.84, 1],
  'dog-tags': [0.7, 0.72, 0.7, 1],
}

/// Builds the whole soldier, part by part, in draw order.
///
/// Everything is derived here and nothing is stored, so two clients given the same snapshot draw
/// the same soldier — which is what makes a screenshot comparison worth running.
export function buildRig(input: RigInput): Placement[] {
  const { pos, appearance, team, pose } = input
  const side = facing(pos, input.aim)
  const flip = side === -1
  const height = POSE_HEIGHT[pose]
  const phase = walkPhase(input.tick, input.moving && pose !== 'prone' && pose !== 'dead')
  const swing = Math.sin(phase * Math.PI * 2)
  const crouched = pose === 'crouch' || pose === 'prone' || pose === 'roll'
  const prone = pose === 'prone' || pose === 'dead'
  const top = pos.y - height
  const shirt = tinted(appearance.shirt, team, 0.45)
  const jetColor = tinted(appearance.jet, team, 0.25)
  const parts: Placement[] = []

  const add = (
    part: BodyPart,
    x: number,
    y: number,
    width: number,
    height: number,
    color: Rgba,
    angle = 0,
  ) => {
    parts.push({ part, x, y, width, height, angle, color, flip })
  }

  if (input.jetting) {
    add('jets', pos.x - side * 9, pos.y - height * 0.45, 7, 16, [jetColor[0], jetColor[1], jetColor[2], 0.9])
  }
  const legY = pos.y - height * 0.16
  const legHeight = prone ? 4 : crouched ? 8 : 12
  add('back-leg', pos.x - side * 3 - swing * 3, legY, 6, legHeight, appearance.pants)
  add('front-leg', pos.x + side * 3 + swing * 3, legY, 6, legHeight, appearance.pants)
  add('torso', pos.x, top + height * 0.42, 15, prone ? 9 : height * 0.42, shirt)

  const shoulder = { x: pos.x + side * 2, y: top + height * 0.36 }
  const angle = armAngle(pos, input.aim)
  add('back-arm', shoulder.x - side * 3, shoulder.y + 2, 11, 5, appearance.skin, angle)

  const headY = prone ? pos.y - 6 : top + 6
  add('head', pos.x + side * 1.5, headY, 11, 11, appearance.skin)
  const hairHeight = HAIR_HEIGHT[appearance.hairStyle]
  if (hairHeight > 0) {
    add('hair', pos.x + side * 1.5, headY - 5, 12, hairHeight, appearance.hair)
  }
  const gear = HEADGEAR_SIZE[appearance.headgear]
  if (gear.height > 0) {
    add('headgear', pos.x + side * 1.5, headY - 6, gear.width, gear.height, tinted(shirt, team, 0.3))
  }
  const chain = CHAIN_COLORS[appearance.chain]
  if (chain) {
    add('chain', pos.x + side * 1, top + height * 0.34, 7, 3, chain)
  }
  const salute = pose === 'emote' ? -Math.PI / 2 : angle
  add('front-arm', shoulder.x + side * 4, shoulder.y + 2, 13, 5, appearance.skin, salute)
  add('weapon', shoulder.x + side * 13, shoulder.y + 2, 24, 5, [0.2, 0.19, 0.17, 1], angle)
  if (pose === 'emote') {
    add('headgear', pos.x + side * 8, headY + 4, 4, 8, [0.55, 0.36, 0.18, 1])
  }

  return parts.sort((a, b) => BODY_PARTS.indexOf(a.part) - BODY_PARTS.indexOf(b.part))
}

/// Where a weapon's muzzle ends up once the arm has been posed, which is where a flash belongs.
export function muzzlePoint(rig: Placement[]): Vec2 | null {
  const weapon = rig.find(part => part.part === 'weapon')
  if (!weapon) return null
  const reach = weapon.width / 2
  return {
    x: weapon.x + Math.cos(weapon.angle) * reach,
    y: weapon.y + Math.sin(weapon.angle) * reach,
  }
}

/// How a corpse is drawn while it fades.
export function corpseFade(ticksLeft: number, fadeTicks = 60): number {
  if (ticksLeft <= 0) return 0
  return Math.min(1, ticksLeft / fadeTicks)
}

/// Blood that stays on the soldier. Particles spray and vanish; this is the stain that remains.
///
/// Amount is 0–1. Shirt and pants take it; skin and hair do not, so a stained soldier is still
/// recognisable as the same player.
export function bloodStain(base: Rgba, amount: number): Rgba {
  if (!Number.isFinite(amount) || amount <= 0) return base
  const stain = Math.min(1, amount)
  return [
    base[0] * (1 - stain * 0.35) + 0.42 * stain,
    base[1] * (1 - stain * 0.65),
    base[2] * (1 - stain * 0.65),
    base[3],
  ]
}

/// A soldier that is on fire is drawn hotter as they burn, which is also how a player knows to find
/// water rather than a fight.
export function burningTint(base: Rgba, burnTicks: number): Rgba {
  if (burnTicks <= 0) return base
  const heat = Math.min(1, burnTicks / 180)
  return [
    Math.min(1, base[0] + heat * 0.5),
    base[1] * (1 - heat * 0.35),
    base[2] * (1 - heat * 0.55),
    base[3],
  ]
}
