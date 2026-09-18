export type ParticleKind =
  | 'blood'
  | 'gib'
  | 'flash'
  | 'casing'
  | 'spark'
  | 'smoke'
  | 'fire'
  | 'explosion'
  | 'trail'

export type Particle = {
  kind: ParticleKind
  x: number
  y: number
  vx: number
  vy: number
  life: number
  maxLife: number
  size: number
}

export type ParticleFieldOptions = { limit?: number; gravity?: number; budget?: number }

/**
 * Cosmetic particles derived from authoritative damage events. A seeded generator keeps the spray
 * reproducible, so the same match replays to the same picture; nothing here is ever read back into
 * the simulation.
 */
function seededRandom(seed: number): () => number {
  let state = (Math.trunc(seed) || 1) >>> 0
  return () => {
    state = (state + 0x6d2b79f5) >>> 0
    let t = state
    t = Math.imul(t ^ (t >>> 15), t | 1)
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61)
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296
  }
}

const BLOOD_PER_DAMAGE = 0.35
const MIN_BLOOD = 2
const MAX_BLOOD = 16
const GIB_COUNT = 9
const SPARK_COUNT = 7
const SMOKE_COUNT = 5
const FIRE_COUNT = 4
const EXPLOSION_SPARKS = 14

export class ParticleField {
  particles: Particle[] = []
  limit: number
  gravity: number

  budget: number

  constructor({ limit = 400, gravity = 620, budget = 1 }: ParticleFieldOptions = {}) {
    this.limit = limit
    this.gravity = gravity
    this.budget = budget
  }

  /// How many of a full spray to actually emit at this quality.
  ///
  /// At least one, whenever the effect runs at all: a hit that produces no particle at all is a hit
  /// the player cannot see, and that is a gameplay change rather than a graphics setting.
  #count(full: number): number {
    if (this.budget <= 0) return 0
    return Math.max(1, Math.round(full * this.budget))
  }

  clear() {
    this.particles = []
  }

  #add(particle: Particle) {
    this.particles.push(particle)
    if (this.particles.length > this.limit) {
      this.particles.splice(0, this.particles.length - this.limit)
    }
  }

  /** A spray of blood along the hit direction; heavier hits bleed more. */
  emitBlood(
    position: { x: number; y: number },
    direction: { x: number; y: number },
    amount: number,
    seed: number,
  ) {
    const random = seededRandom(seed)
    const count = this.#count(
      Math.min(MAX_BLOOD, Math.max(MIN_BLOOD, Math.round(amount * BLOOD_PER_DAMAGE))),
    )
    const length = Math.hypot(direction.x, direction.y) || 1
    const nx = direction.x / length
    const ny = direction.y / length
    for (let i = 0; i < count; i += 1) {
      const spread = (random() - 0.5) * 1.6
      const speed = 40 + random() * 150
      this.#add({
        kind: 'blood',
        x: position.x,
        y: position.y,
        vx: (nx * Math.cos(spread) - ny * Math.sin(spread)) * speed,
        vy: (nx * Math.sin(spread) + ny * Math.cos(spread)) * speed - random() * 60,
        life: 0.5 + random() * 0.7,
        maxLife: 1.2,
        size: 1.5 + random() * 2.5,
      })
    }
  }

  /** Gore from a body torn apart, thrown outward on top of the body's own velocity. */
  emitGibs(position: { x: number; y: number }, velocity: { x: number; y: number }, seed: number) {
    const random = seededRandom(seed)
    const gibs = this.#count(GIB_COUNT)
    for (let i = 0; i < gibs; i += 1) {
      const angle = (i / Math.max(1, gibs)) * Math.PI * 2 + random() * 0.4
      const speed = 90 + random() * 180
      this.#add({
        kind: 'gib',
        x: position.x,
        y: position.y,
        vx: velocity.x * 0.4 + Math.cos(angle) * speed,
        vy: velocity.y * 0.4 + Math.sin(angle) * speed - 120,
        life: 1.2 + random() * 0.9,
        maxLife: 2.1,
        size: 2.5 + random() * 3.5,
      })
    }
  }

  emitFlash(position: { x: number; y: number }) {
    this.#add({
      kind: 'flash',
      x: position.x,
      y: position.y,
      vx: 0,
      vy: 0,
      life: 0.08,
      maxLife: 0.08,
      size: 10,
    })
  }

  emitCasing(position: { x: number; y: number }) {
    this.#add({
      kind: 'casing',
      x: position.x,
      y: position.y,
      vx: -40,
      vy: -80,
      life: 0.45,
      maxLife: 0.45,
      size: 3,
    })
  }


  /** Sparks where a round struck something that is not a person. */
  emitSparks(position: { x: number; y: number }, normal: { x: number; y: number }, seed: number) {
    const random = seededRandom(seed)
    const count = this.#count(SPARK_COUNT)
    const length = Math.hypot(normal.x, normal.y) || 1
    const nx = normal.x / length
    const ny = normal.y / length
    for (let i = 0; i < count; i += 1) {
      const spread = (random() - 0.5) * 1.9
      const speed = 70 + random() * 180
      this.#add({
        kind: 'spark',
        x: position.x,
        y: position.y,
        vx: (nx * Math.cos(spread) - ny * Math.sin(spread)) * speed,
        vy: (nx * Math.sin(spread) + ny * Math.cos(spread)) * speed,
        life: 0.12 + random() * 0.22,
        maxLife: 0.34,
        size: 1 + random() * 1.5,
      })
    }
  }

  /** Smoke, which rises rather than falls and is pushed by whatever wind is blowing. */
  emitSmoke(position: { x: number; y: number }, seed: number, wind = 0) {
    const random = seededRandom(seed)
    const count = this.#count(SMOKE_COUNT)
    for (let i = 0; i < count; i += 1) {
      this.#add({
        kind: 'smoke',
        x: position.x + (random() - 0.5) * 10,
        y: position.y + (random() - 0.5) * 10,
        vx: (random() - 0.5) * 22 + wind * 0.4,
        vy: -18 - random() * 26,
        life: 0.9 + random() * 1.4,
        maxLife: 2.3,
        size: 5 + random() * 9,
      })
    }
  }

  /** Fire, which climbs and dies quickly; a burning soldier emits it every few ticks. */
  emitFire(position: { x: number; y: number }, seed: number) {
    const random = seededRandom(seed)
    const count = this.#count(FIRE_COUNT)
    for (let i = 0; i < count; i += 1) {
      this.#add({
        kind: 'fire',
        x: position.x + (random() - 0.5) * 9,
        y: position.y + (random() - 0.5) * 12,
        vx: (random() - 0.5) * 26,
        vy: -40 - random() * 50,
        life: 0.18 + random() * 0.3,
        maxLife: 0.48,
        size: 3 + random() * 4,
      })
    }
  }

  /** An explosion: one bright core, a ring of fire, and the smoke it leaves behind. */
  emitExplosion(position: { x: number; y: number }, radius: number, seed: number) {
    const random = seededRandom(seed)
    this.#add({
      kind: 'explosion',
      x: position.x,
      y: position.y,
      vx: 0,
      vy: 0,
      life: 0.3,
      maxLife: 0.3,
      size: Math.max(8, radius),
    })
    const count = this.#count(EXPLOSION_SPARKS)
    for (let i = 0; i < count; i += 1) {
      const angle = (i / Math.max(1, count)) * Math.PI * 2
      const speed = radius * (1.4 + random() * 1.4)
      this.#add({
        kind: 'fire',
        x: position.x,
        y: position.y,
        vx: Math.cos(angle) * speed,
        vy: Math.sin(angle) * speed,
        life: 0.2 + random() * 0.35,
        maxLife: 0.55,
        size: 3 + random() * 5,
      })
    }
    this.emitSmoke(position, seed + 1)
  }

  /** The streak a round leaves behind it, which lives a couple of frames and no longer. */
  emitTrail(position: { x: number; y: number }, velocity: { x: number; y: number }) {
    if (this.budget <= 0) return
    this.#add({
      kind: 'trail',
      x: position.x,
      y: position.y,
      vx: velocity.x * 0.08,
      vy: velocity.y * 0.08,
      life: 0.06,
      maxLife: 0.06,
      size: 2,
    })
  }

  step(dt: number) {
    if (!Number.isFinite(dt) || dt <= 0) return
    for (const particle of this.particles) {
      // Smoke and fire are hot: they climb instead of falling, and a streak is gone before gravity
      // would have moved it at all.
      const pull =
        particle.kind === 'smoke' || particle.kind === 'fire'
          ? -this.gravity * 0.06
          : particle.kind === 'trail' || particle.kind === 'explosion'
            ? 0
            : this.gravity
      particle.vy += pull * dt
      particle.x += particle.vx * dt
      particle.y += particle.vy * dt
      particle.life -= dt
    }
    this.particles = this.particles.filter(particle => particle.life > 0)
  }
}
