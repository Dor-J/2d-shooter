export type Vec2 = { x: number; y: number }
export type Input = { seq: number; left: boolean; right: boolean; jump: boolean; jet: boolean; fire: boolean; aim: Vec2; weapon: number }
export type Player = { id: number; name: string; pos: Vec2; vel: Vec2; hp: number; fuel: number; kills: number; deaths: number; team: number; grounded: boolean; cooldown: number; respawn: number; last_seq: number; weapon: number }
export type Projectile = { id: number; pos: Vec2; owner: number }
export type World = { tick: number; mode: string; players: Record<string, Player>; projectiles: Projectile[]; scores: number[]; events: unknown[] }
export type Room = { id: number; name: string; mode: string; players: number; capacity: number }
type Predict = (player: string, input: string) => string
const platforms = [ [0,650,1200,50], [120,490,280,20], [800,490,280,20], [480,365,240,20], [510,555,180,20] ]

export class GameClient {
  canvas: HTMLCanvasElement
  gl: WebGL2RenderingContext
  program: WebGLProgram
  buffer: WebGLBuffer
  position: number
  resolution: WebGLUniformLocation
  color: WebGLUniformLocation
  world: World | null = null
  previousWorld: World | null = null
  snapshotAt = performance.now()
  localId = 0
  predicted: Player | null = null
  predict: Predict | null = null
  send: (input: Input) => void
  keys = new Set<string>()
  touch = { left: false, right: false, jump: false, jet: false, fire: false }
  aim: Vec2 = { x: 600, y: 350 }
  seq = 0
  weapon = 0
  last = performance.now()
  accumulator = 0
  frame = 0
  resizeObserver: ResizeObserver

  constructor(canvas: HTMLCanvasElement, send: (input: Input) => void) {
    this.canvas = canvas; this.send = send
    const gl = canvas.getContext('webgl2', { antialias: false })
    if (!gl) throw new Error('WebGL2 is required on this device')
    this.gl = gl
    const vertex = gl.createShader(gl.VERTEX_SHADER)!
    gl.shaderSource(vertex, `#version 300 es\nin vec2 a_position; uniform vec2 u_resolution; void main(){ vec2 p = a_position / u_resolution * 2.0 - 1.0; gl_Position = vec4(p.x, -p.y, 0, 1); }`)
    gl.compileShader(vertex)
    const fragment = gl.createShader(gl.FRAGMENT_SHADER)!
    gl.shaderSource(fragment, `#version 300 es\nprecision mediump float; uniform vec4 u_color; out vec4 color; void main(){color=u_color;}`)
    gl.compileShader(fragment)
    const program = gl.createProgram()!
    gl.attachShader(program, vertex); gl.attachShader(program, fragment); gl.linkProgram(program)
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) throw new Error('WebGL shader setup failed')
    this.program = program; this.buffer = gl.createBuffer()!; this.position = gl.getAttribLocation(program, 'a_position'); this.resolution = gl.getUniformLocation(program, 'u_resolution')!; this.color = gl.getUniformLocation(program, 'u_color')!
    this.resizeObserver = new ResizeObserver(() => this.resize()); this.resizeObserver.observe(canvas)
    window.addEventListener('keydown', this.keydown); window.addEventListener('keyup', this.keyup); window.addEventListener('blur', this.blur)
    canvas.addEventListener('pointermove', this.pointer); canvas.addEventListener('pointerdown', this.pointerDown); window.addEventListener('pointerup', this.pointerUp)
    this.resize(); this.frame = requestAnimationFrame(this.loop)
  }
  async loadWasm() {
    try {
      const wasmPath = '/wasm/game_core.js'
      const module = await import(/* @vite-ignore */ wasmPath) as { default: () => Promise<unknown>; predict_player: Predict }
      await module.default(); this.predict = module.predict_player
    } catch { this.predict = null }
  }
  keydown = (e: KeyboardEvent) => { if (['Space','ArrowUp','ArrowDown','ArrowLeft','ArrowRight'].includes(e.code)) e.preventDefault(); this.keys.add(e.code); if (e.code === 'Digit1') this.weapon = 0; if (e.code === 'Digit2') this.weapon = 1; if (e.code === 'Digit3') this.weapon = 2 }
  keyup = (e: KeyboardEvent) => this.keys.delete(e.code)
  blur = () => { this.keys.clear(); this.touch = { left: false, right: false, jump: false, jet: false, fire: false } }
  pointer = (e: PointerEvent) => { const r = this.canvas.getBoundingClientRect(); this.aim = { x: (e.clientX - r.left) / r.width * 1200, y: (e.clientY - r.top) / r.height * 700 } }
  pointerDown = (e: PointerEvent) => { this.pointer(e); this.touch.fire = true }
  pointerUp = () => { this.touch.fire = false }
  resize() { const dpr = Math.min(window.devicePixelRatio || 1, 2); this.canvas.width = Math.max(1, Math.round(this.canvas.clientWidth * dpr)); this.canvas.height = Math.max(1, Math.round(this.canvas.clientHeight * dpr)); this.gl.viewport(0, 0, this.canvas.width, this.canvas.height) }
  setSnapshot(world: World) { this.previousWorld = this.world; this.world = world; this.snapshotAt = performance.now(); this.predicted = world.players[this.localId] ? structuredClone(world.players[this.localId]) : null }
  loop = (now: number) => { this.accumulator += Math.min(now - this.last, 100); this.last = now; while (this.accumulator >= 1000 / 60) { this.tick(); this.accumulator -= 1000 / 60 } this.render(); this.frame = requestAnimationFrame(this.loop) }
  tick() { if (!this.world || !this.localId) return; const input: Input = { seq: ++this.seq, left: this.keys.has('KeyA') || this.keys.has('ArrowLeft') || this.touch.left, right: this.keys.has('KeyD') || this.keys.has('ArrowRight') || this.touch.right, jump: this.keys.has('Space') || this.keys.has('KeyW') || this.touch.jump, jet: this.keys.has('ShiftLeft') || this.keys.has('ShiftRight') || this.touch.jet, fire: this.touch.fire, aim: this.aim, weapon: this.weapon }; this.send(input); if (this.predict && this.predicted) { try { const result = this.predict(JSON.stringify(this.predicted), JSON.stringify(input)); if (result) this.predicted = JSON.parse(result) as Player } catch { this.predict = null } } }
  rect(x: number, y: number, w: number, h: number, c: number[]) { const gl = this.gl; gl.uniform4f(this.color, c[0], c[1], c[2], c[3] ?? 1); gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([x,y,x+w,y,x,y+h,x,y+h,x+w,y,x+w,y+h]), gl.STREAM_DRAW); gl.drawArrays(gl.TRIANGLES, 0, 6) }
  render() { const gl = this.gl; gl.clearColor(.07,.11,.15,1); gl.clear(gl.COLOR_BUFFER_BIT); gl.useProgram(this.program); gl.bindBuffer(gl.ARRAY_BUFFER,this.buffer); gl.enableVertexAttribArray(this.position); gl.vertexAttribPointer(this.position,2,gl.FLOAT,false,0,0); gl.uniform2f(this.resolution,1200,700); for (const [x,y,w,h] of platforms) { this.rect(x,y,w,h,[.30,.37,.38,1]); this.rect(x,y,w,3,[.58,.70,.65,1]) } if (!this.world) return; const blend = Math.min(1, (performance.now() - this.snapshotAt) / 50); for (const projectile of this.world.projectiles) this.rect(projectile.pos.x-3,projectile.pos.y-3,6,6,[1,.77,.29,1]); for (const p of Object.values(this.world.players)) { if (p.hp <= 0) continue; const previous = this.previousWorld?.players[p.id]; const pos = p.id === this.localId && this.predicted ? this.predicted.pos : previous ? { x: previous.pos.x + (p.pos.x - previous.pos.x) * blend, y: previous.pos.y + (p.pos.y - previous.pos.y) * blend } : p.pos; const color = p.team === 1 ? [.35,.70,1,1] : p.team === 2 ? [1,.38,.38,1] : [.87,.86,.72,1]; this.rect(pos.x-11,pos.y-15,22,30,color); this.rect(pos.x-9,pos.y-23,18,8,[.74,.65,.54,1]); this.rect(pos.x-13,pos.y-30,26 * p.hp / 100,3,[.35,.95,.49,1]) } }
  destroy() { cancelAnimationFrame(this.frame); this.resizeObserver.disconnect(); window.removeEventListener('keydown',this.keydown); window.removeEventListener('keyup',this.keyup); window.removeEventListener('blur',this.blur); this.canvas.removeEventListener('pointermove',this.pointer); this.canvas.removeEventListener('pointerdown',this.pointerDown); window.removeEventListener('pointerup',this.pointerUp) }
}
