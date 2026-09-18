// Plays a recorded match. The simulation already happened; this only walks the saved ticks.

export type ReplayHeader = {
  format: number
  protocol: number
  map: string
  mode: string
  weapon_hash: number
  seed: number
  source_revision: string
}

export type ReplayChunk = {
  tick: number
  inputs: Record<string, unknown>
  checksum: string
}

export type ReplayFile = {
  header: ReplayHeader
  chunks: ReplayChunk[]
}

export type Camera = { mode: 'follow'; player: number } | { mode: 'free'; x: number; y: number }

export class DemoPlayer {
  readonly file: ReplayFile
  index = 0
  paused = false
  camera: Camera = { mode: 'free', x: 0, y: 0 }

  constructor(file: ReplayFile) {
    this.file = file
  }

  static open(source: string | ReplayFile, protocol: number, weaponHash: number): DemoPlayer | { error: string } {
    const file = typeof source === 'string' ? (JSON.parse(source) as ReplayFile) : source
    if (!file?.header || !Array.isArray(file.chunks) || file.chunks.length === 0) return { error: 'empty' }
    if (file.header.format !== 1) return { error: 'format' }
    if (file.header.protocol !== protocol) return { error: 'protocol' }
    if (file.header.weapon_hash !== weaponHash) return { error: 'weapon' }
    return new DemoPlayer(file)
  }

  get tick(): number {
    return this.file.chunks[this.index]?.tick ?? 0
  }

  get checksum(): string {
    return this.file.chunks[this.index]?.checksum ?? ''
  }

  pause(): void {
    this.paused = true
  }

  resume(): void {
    this.paused = false
  }

  step(count = 1): number {
    if (this.paused) return this.tick
    this.index = Math.min(this.file.chunks.length - 1, this.index + Math.max(1, count))
    return this.tick
  }

  seek(tick: number): number {
    const found = this.file.chunks.findIndex(chunk => chunk.tick >= tick)
    this.index = found < 0 ? this.file.chunks.length - 1 : found
    return this.tick
  }

  fastForward(factor = 4): number {
    return this.step(factor)
  }

  follow(player: number): void {
    this.camera = { mode: 'follow', player }
  }

  freeCamera(x = 0, y = 0): void {
    this.camera = { mode: 'free', x, y }
  }

  export(): string {
    return JSON.stringify(this.file)
  }

  static repair(file: ReplayFile): ReplayFile | { error: string } {
    const chunks = file.chunks.filter(chunk => chunk.checksum)
    if (chunks.length === 0) return { error: 'truncated' }
    return { ...file, chunks }
  }
}

export class DemoRecorder {
  chunks: ReplayChunk[] = []

  record(tick: number, checksum: string) {
    this.chunks.push({ tick, inputs: {}, checksum })
  }

  toFile(header: ReplayHeader): ReplayFile {
    return { header, chunks: this.chunks }
  }
}
