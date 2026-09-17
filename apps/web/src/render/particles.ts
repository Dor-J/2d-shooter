export type ParticleKind = 'blood' | 'gib'

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

export type ParticleFieldOptions = { limit?: number; gravity?: number }

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

export class ParticleField {
  particles: Particle[] = []
  limit: number
  gravity: number

  constructor({ limit = 400, gravity = 620 }: ParticleFieldOptions = {}) {
    this.limit = limit
    this.gravity = gravity
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
    const count = Math.min(MAX_BLOOD, Math.max(MIN_BLOOD, Math.round(amount * BLOOD_PER_DAMAGE)))
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
    for (let i = 0; i < GIB_COUNT; i += 1) {
      const angle = (i / GIB_COUNT) * Math.PI * 2 + random() * 0.4
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

  step(dt: number) {
    if (!Number.isFinite(dt) || dt <= 0) return
    for (const particle of this.particles) {
      particle.vy += this.gravity * dt
      particle.x += particle.vx * dt
      particle.y += particle.vy * dt
      particle.life -= dt
    }
    this.particles = this.particles.filter(particle => particle.life > 0)
  }
}
