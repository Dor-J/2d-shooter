import { viewForCanvas } from './mobile'
import type { View } from './mobile'
import { polygonVertexBuffer, type MapPolygon } from './map-render'
import { InputSystem } from './input'
import type { InputFrame, InterfaceEvent } from './input'
import { captureCanvas, downloadBlob } from './input/screenshot'
import { aimFromStick, STICK_AIM_RANGE } from './input/touch'
import { aimFromPointer } from './input/mouse'
import { ParticleField } from './render/particles'
import type { KillFeedEntry } from './hud/feed'
import type { Action } from './input/actions'

export type Vec2 = { x: number; y: number }
export type Input = InputFrame
export type Player = { id: number; name: string; pos: Vec2; vel: Vec2; hp: number; fuel: number; kills: number; deaths: number; team: number; grounded: boolean; cooldown: number; respawn: number; last_seq: number; weapon: number; ammo: number; reload_timer: number; startup: number; magazines: number[]; grenades: number; grenade_cooldown: number; grenade_held: boolean; armor: number; assists: number; spawn_protection: number; bleed: unknown; last_damage_direction: Vec2 }
export type RagdollSegment = { region: string; pos: Vec2; vel: Vec2; radius: number; grounded: boolean }
export type Ragdoll = { player: number; team: number; segments: RagdollSegment[]; ticks_left: number; gibbed: boolean }
export type WorldEvent =
  | { Blood: { target: number; position: Vec2; direction: Vec2; amount: number } }
  | { Gibs: { target: number; position: Vec2; velocity: Vec2 } }
  | { KillFeed: KillFeedEntry }
  | Record<string, unknown>
export const weaponNames = ['Desert Eagles','HK MP5','AK-74','Steyr AUG','SPAS-12','Ruger 77','M79','Barrett M82A1','FN Minimi','XM214 Minigun','USSOCOM','Combat Knife','Chainsaw','M72 LAW']
export type Projectile = { id: number; pos: Vec2; owner: number }
export type World = { tick: number; mode: string; players: Record<string, Player>; projectiles: Projectile[]; map_polygons: MapPolygon[]; ragdolls: Ragdoll[]; scores: number[]; events: WorldEvent[] }
export type Room = { id: number; name: string; mode: string; players: number; capacity: number; map: string }
type Predict = (player: string, input: string) => string
declare global { interface Window { __arenaWasmReady?: Promise<Predict | null> } }
export class GameClient {
  canvas: HTMLCanvasElement
  gl: WebGL2RenderingContext
  program: WebGLProgram
  buffer: WebGLBuffer
  position: number
  resolution: WebGLUniformLocation
  camera: WebGLUniformLocation
  color: WebGLUniformLocation
  spriteProgram: WebGLProgram
  spriteBuffer: WebGLBuffer
  spritePosition: number
  spriteUv: number
  spriteResolution: WebGLUniformLocation
  spriteCamera: WebGLUniformLocation
  spriteTint: WebGLUniformLocation
  soldierTexture: WebGLTexture | null = null
  world: World | null = null
  previousWorld: World | null = null
  snapshotAt = performance.now()
  localId = 0
  predicted: Player | null = null
  predict: Predict | null = null
  send: (input: Input) => void
  input = new InputSystem()
  particles = new ParticleField()
  onInterfaceEvent: (event: InterfaceEvent) => void = () => {}
  onKills: (entries: KillFeedEntry[]) => void = () => {}
  onUiChange: (ui: { weapon: number; scoreboardVisible: boolean; scoreboardOffset: number }) => void = () => {}
  uiSignature = ''
  aim: Vec2 = { x: 600, y: 350 }
  last = performance.now()
  accumulator = 0
  frame = 0
  resizeObserver: ResizeObserver
  view: View = { x: 0, y: 0, width: 1200, height: 700 }

  constructor(canvas: HTMLCanvasElement, send: (input: Input) => void, input?: InputSystem) {
    this.canvas = canvas; this.send = send; if (input) this.input = input
    const gl = canvas.getContext('webgl2', { antialias: false })
    if (!gl) throw new Error('WebGL2 is required on this device')
    this.gl = gl
    const vertex = gl.createShader(gl.VERTEX_SHADER)!
    gl.shaderSource(vertex, `#version 300 es\nin vec2 a_position; uniform vec2 u_resolution; uniform vec2 u_camera; void main(){ vec2 p = (a_position - u_camera) / u_resolution * 2.0 - 1.0; gl_Position = vec4(p.x, -p.y, 0, 1); }`)
    gl.compileShader(vertex)
    const fragment = gl.createShader(gl.FRAGMENT_SHADER)!
    gl.shaderSource(fragment, `#version 300 es\nprecision mediump float; uniform vec4 u_color; out vec4 color; void main(){color=u_color;}`)
    gl.compileShader(fragment)
    const program = gl.createProgram()!
    gl.attachShader(program, vertex); gl.attachShader(program, fragment); gl.linkProgram(program)
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) throw new Error('WebGL shader setup failed')
    this.program = program; this.buffer = gl.createBuffer()!; this.position = gl.getAttribLocation(program, 'a_position'); this.resolution = gl.getUniformLocation(program, 'u_resolution')!; this.camera = gl.getUniformLocation(program, 'u_camera')!; this.color = gl.getUniformLocation(program, 'u_color')!
    const spriteVertex = gl.createShader(gl.VERTEX_SHADER)!
    gl.shaderSource(spriteVertex, `#version 300 es\nin vec2 a_position; in vec2 a_uv; uniform vec2 u_resolution; uniform vec2 u_camera; out vec2 v_uv; void main(){ vec2 p=(a_position-u_camera)/u_resolution*2.0-1.0; gl_Position=vec4(p.x,-p.y,0,1); v_uv=a_uv; }`)
    gl.compileShader(spriteVertex)
    const spriteFragment = gl.createShader(gl.FRAGMENT_SHADER)!
    gl.shaderSource(spriteFragment, `#version 300 es\nprecision mediump float; in vec2 v_uv; uniform sampler2D u_texture; uniform vec4 u_tint; out vec4 color; void main(){ color=texture(u_texture,v_uv)*u_tint; }`)
    gl.compileShader(spriteFragment)
    const spriteProgram = gl.createProgram()!
    gl.attachShader(spriteProgram,spriteVertex); gl.attachShader(spriteProgram,spriteFragment); gl.linkProgram(spriteProgram)
    if (!gl.getProgramParameter(spriteProgram, gl.LINK_STATUS)) throw new Error('Sprite shader setup failed')
    this.spriteProgram = spriteProgram; this.spriteBuffer = gl.createBuffer()!
    this.spritePosition = gl.getAttribLocation(spriteProgram,'a_position'); this.spriteUv = gl.getAttribLocation(spriteProgram,'a_uv')
    this.spriteResolution = gl.getUniformLocation(spriteProgram,'u_resolution')!; this.spriteCamera = gl.getUniformLocation(spriteProgram,'u_camera')!; this.spriteTint = gl.getUniformLocation(spriteProgram,'u_tint')!
    const soldier = new Image()
    soldier.src = '/art/soldier.png'
    soldier.onload = () => { const texture = gl.createTexture(); if (!texture) return; gl.bindTexture(gl.TEXTURE_2D,texture); gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,gl.RGBA,gl.UNSIGNED_BYTE,soldier); gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MIN_FILTER,gl.LINEAR); gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MAG_FILTER,gl.LINEAR); this.soldierTexture = texture }
    gl.enable(gl.BLEND); gl.blendFunc(gl.SRC_ALPHA,gl.ONE_MINUS_SRC_ALPHA)
    this.resizeObserver = new ResizeObserver(() => this.resize()); this.resizeObserver.observe(canvas)
    window.addEventListener('keydown', this.keydown); window.addEventListener('keyup', this.keyup); window.addEventListener('blur', this.blur)
    canvas.addEventListener('pointermove', this.pointer); canvas.addEventListener('pointerdown', this.pointerDown); window.addEventListener('pointerup', this.pointerUp)
    canvas.addEventListener('wheel', this.wheel, { passive: false })
    this.resize(); this.frame = requestAnimationFrame(this.loop)
  }
  async loadWasm() {
    this.predict = await (window.__arenaWasmReady ?? Promise.resolve(null))
  }
  keydown = (e: KeyboardEvent) => { this.input.keyboard.keydown(e) }
  keyup = (e: KeyboardEvent) => { this.input.keyboard.keyup(e) }
  blur = () => this.input.releaseAll()
  wheel = (e: WheelEvent) => { this.input.mouse.wheel(e) }
  pointer = (e: PointerEvent) => { const r = this.canvas.getBoundingClientRect(); this.aim = aimFromPointer(e.clientX - r.left, e.clientY - r.top, r.width, r.height, this.view) }
  pointerDown = (e: PointerEvent) => { this.pointer(e); this.input.mouse.pointerdown(e) }
  pointerUp = (e: PointerEvent) => { this.input.mouse.pointerup(e) }
  get weapon() { return this.input.weapon }
  set weapon(index: number) { this.input.weapon = index }
  /** Touch movement pad: one drag drives the three movement actions at once. */
  touchMove(state: { left: boolean; right: boolean; jump: boolean }) {
    this.input.state.set('moveLeft', state.left); this.input.state.set('moveRight', state.right); this.input.state.set('jump', state.jump)
  }
  /** Touch aim pad: the drag direction becomes a world-space aim point and holds the fire action. */
  touchAim(state: { dx: number; dy: number; fire: boolean }) {
    this.input.state.set('fire', state.fire)
    const player = this.predicted ?? (this.world ? this.world.players[this.localId] : null)
    if (!state.fire || !player) return
    const aimed = aimFromStick(player.pos, state.dx, state.dy, STICK_AIM_RANGE)
    if (aimed) this.aim = aimed
  }
  touchAction(pointerId: number, action: Action, down: boolean) { if (down) this.input.touch.press(pointerId, action); else this.input.touch.release(pointerId) }
  touchCancel() { this.input.touch.cancel(); this.touchMove({ left: false, right: false, jump: false }); this.touchAim({ dx: 0, dy: 0, fire: false }) }
  suspendInput() { this.input.suspend() }
  resumeInput() { this.input.resume() }
  resize() { const dpr = Math.min(window.devicePixelRatio || 1, 2); this.canvas.width = Math.max(1, Math.round(this.canvas.clientWidth * dpr)); this.canvas.height = Math.max(1, Math.round(this.canvas.clientHeight * dpr)); this.gl.viewport(0, 0, this.canvas.width, this.canvas.height) }
  updateView() { const player = this.predicted ?? this.world?.players[this.localId]; const rect = this.canvas.getBoundingClientRect(); this.view = viewForCanvas(rect.width, rect.height, player?.pos.x ?? 600, player?.pos.y ?? 350); this.canvas.style.backgroundSize = `${1200 / this.view.width * 100}% ${700 / this.view.height * 100}%`; this.canvas.style.backgroundPosition = `${this.view.width >= 1200 ? 50 : this.view.x / (1200 - this.view.width) * 100}% ${this.view.height >= 700 ? 50 : this.view.y / (700 - this.view.height) * 100}%` }
  setSnapshot(world: World) {
    this.previousWorld = this.world; this.world = world; this.snapshotAt = performance.now()
    this.predicted = world.players[this.localId] ? structuredClone(world.players[this.localId]) : null
    this.consumeEvents(world)
  }
  /** Turns authoritative damage events into cosmetics; nothing here is sent back to the server. */
  consumeEvents(world: World) {
    const kills: KillFeedEntry[] = []
    for (const event of world.events ?? []) {
      const blood = (event as { Blood?: { target: number; position: Vec2; direction: Vec2; amount: number } }).Blood
      if (blood) this.particles.emitBlood(blood.position, blood.direction, blood.amount, world.tick * 31 + blood.target)
      const gibs = (event as { Gibs?: { target: number; position: Vec2; velocity: Vec2 } }).Gibs
      if (gibs) this.particles.emitGibs(gibs.position, gibs.velocity, world.tick * 17 + gibs.target)
      const feed = (event as { KillFeed?: KillFeedEntry }).KillFeed
      if (feed) kills.push(feed)
    }
    if (kills.length > 0) this.onKills(kills)
  }
  loop = (now: number) => { this.accumulator += Math.min(now - this.last, 100); this.last = now; while (this.accumulator >= 1000 / 60) { this.tick(); this.accumulator -= 1000 / 60 } this.render(); this.frame = requestAnimationFrame(this.loop) }
  activeGamepad() {
    const pads = typeof navigator !== 'undefined' && navigator.getGamepads ? navigator.getGamepads() : []
    for (const pad of pads ?? []) if (pad && pad.connected) return pad
    return null
  }
  tick() {
    if (!this.world || !this.localId) return
    const player = this.predicted ?? this.world.players[this.localId]
    const gamepad = this.activeGamepad()
    const stick = this.input.gamepad.aimStick(gamepad)
    if (stick && player) {
      const aimed = aimFromStick(player.pos, stick.x, stick.y, STICK_AIM_RANGE * this.input.mouse.sensitivity)
      if (aimed) this.aim = aimed
    }
    const { frame, events } = this.input.tick({ aim: this.aim, airborne: player ? !player.grounded : false, gamepad })
    for (const event of events) {
      if (event.type === 'screenshot') captureCanvas(this.canvas, (blob, filename) => downloadBlob(blob as Blob, filename))
      this.onInterfaceEvent(event)
    }
    const ui = { weapon: this.input.weapon, scoreboardVisible: this.input.ui.scoreboardVisible, scoreboardOffset: this.input.ui.scoreboardOffset }
    const signature = `${ui.weapon}:${ui.scoreboardVisible}:${ui.scoreboardOffset}`
    if (signature !== this.uiSignature) { this.uiSignature = signature; this.onUiChange(ui) }
    this.send(frame)
    if (this.predict && this.predicted) {
      try {
        const result = this.predict(JSON.stringify(this.predicted), JSON.stringify(frame))
        if (result) this.predicted = JSON.parse(result) as Player
      } catch { this.predict = null }
    }
  }
  rect(x: number, y: number, w: number, h: number, c: number[]) { const gl = this.gl; gl.uniform4f(this.color, c[0], c[1], c[2], c[3] ?? 1); gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([x,y,x+w,y,x,y+h,x,y+h,x+w,y,x+w,y+h]), gl.STREAM_DRAW); gl.drawArrays(gl.TRIANGLES, 0, 6) }
  polygon(polygon: MapPolygon) {
    const colors: Record<string, number[]> = { Ice: [.38,.72,.86,1], Bouncy: [.82,.54,.28,1], Deadly: [.78,.18,.16,1], OneWay: [.55,.57,.50,1] }
    const color = colors[polygon.kind] ?? [.30,.34,.35,1]
    this.gl.uniform4f(this.color,color[0],color[1],color[2],color[3])
    this.gl.bufferData(this.gl.ARRAY_BUFFER,polygonVertexBuffer([polygon]),this.gl.STREAM_DRAW)
    this.gl.drawArrays(this.gl.TRIANGLES,0,3)
  }
  sprite(x: number, y: number, w: number, h: number, tint: number[], flip: boolean) {
    const gl = this.gl
    if (!this.soldierTexture) return
    const left = flip ? 1 : 0, right = flip ? 0 : 1
    gl.useProgram(this.spriteProgram)
    gl.bindBuffer(gl.ARRAY_BUFFER,this.spriteBuffer)
    gl.bufferData(gl.ARRAY_BUFFER,new Float32Array([x,y,left,0,x+w,y,right,0,x,y+h,left,1,x,y+h,left,1,x+w,y,right,0,x+w,y+h,right,1]),gl.STREAM_DRAW)
    gl.enableVertexAttribArray(this.spritePosition); gl.vertexAttribPointer(this.spritePosition,2,gl.FLOAT,false,16,0)
    gl.enableVertexAttribArray(this.spriteUv); gl.vertexAttribPointer(this.spriteUv,2,gl.FLOAT,false,16,8)
    gl.uniform2f(this.spriteResolution,this.view.width,this.view.height); gl.uniform2f(this.spriteCamera,this.view.x,this.view.y)
    gl.uniform4f(this.spriteTint,tint[0],tint[1],tint[2],1)
    gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D,this.soldierTexture)
    gl.drawArrays(gl.TRIANGLES,0,6)
  }
  render() {
    const gl = this.gl
    this.updateView()
    gl.clearColor(0,0,0,0); gl.clear(gl.COLOR_BUFFER_BIT)
    gl.useProgram(this.program); gl.bindBuffer(gl.ARRAY_BUFFER,this.buffer)
    gl.enableVertexAttribArray(this.position); gl.vertexAttribPointer(this.position,2,gl.FLOAT,false,0,0)
    gl.uniform2f(this.resolution,this.view.width,this.view.height); gl.uniform2f(this.camera,this.view.x,this.view.y)
    if (!this.world) return
    for (const polygon of this.world.map_polygons ?? []) this.polygon(polygon)
    this.particles.step(1 / 60)
    for (const ragdoll of this.world.ragdolls ?? []) {
      const fade = Math.min(1, ragdoll.ticks_left / 60)
      const tint = ragdoll.team === 1 ? [.42,.53,.72] : ragdoll.team === 2 ? [.72,.44,.42] : [.62,.58,.50]
      for (const segment of ragdoll.segments) {
        const size = segment.radius * 2
        this.rect(segment.pos.x - segment.radius, segment.pos.y - segment.radius, size, size, [tint[0],tint[1],tint[2],.85 * fade])
      }
    }
    for (const particle of this.particles.particles) {
      const fade = Math.max(0, particle.life / particle.maxLife)
      const color = particle.kind === 'gib' ? [.55,.10,.10,fade] : [.78,.12,.14,fade]
      this.rect(particle.x - particle.size / 2, particle.y - particle.size / 2, particle.size, particle.size, color)
    }
    const blend = Math.min(1,(performance.now()-this.snapshotAt)/50)
    for (const projectile of this.world.projectiles) {
      this.rect(projectile.pos.x-7,projectile.pos.y-2,14,4,[1,.50,.14,.32])
      this.rect(projectile.pos.x-3,projectile.pos.y-1,6,2,[1,.90,.55,1])
    }
    for (const p of Object.values(this.world.players)) {
      if (p.hp <= 0) continue
      const previous = this.previousWorld?.players[p.id]
      const pos = p.id === this.localId && this.predicted ? this.predicted.pos : previous ? {x:previous.pos.x+(p.pos.x-previous.pos.x)*blend,y:previous.pos.y+(p.pos.y-previous.pos.y)*blend} : p.pos
      const color = p.team === 1 ? [.64,.79,1] : p.team === 2 ? [1,.66,.62] : [1,.95,.83]
      if (!p.grounded && p.fuel < .99) {
        this.rect(pos.x-13,pos.y+9,8,18,[.32,.69,1,.35]); this.rect(pos.x-11,pos.y+12,4,12,[.68,.91,1,.85])
      }
      if (!this.soldierTexture) this.rect(pos.x-12,pos.y-16,24,32,[color[0],color[1],color[2],1])
      else this.sprite(pos.x-22,pos.y-32,52,48,color,p.vel.x < -10)
      gl.useProgram(this.program); gl.bindBuffer(gl.ARRAY_BUFFER,this.buffer)
      gl.enableVertexAttribArray(this.position); gl.vertexAttribPointer(this.position,2,gl.FLOAT,false,0,0); gl.uniform2f(this.resolution,this.view.width,this.view.height); gl.uniform2f(this.camera,this.view.x,this.view.y)
      this.rect(pos.x-20,pos.y-53,40,4,[.04,.07,.09,.85])
      this.rect(pos.x-19,pos.y-52,38*p.hp/100,2,[p.hp<30 ? 1 : .88,p.hp<30 ? .25 : .72,.28,1])
      if (p.id === this.localId) { this.rect(pos.x-2,pos.y-61,4,4,[1,.82,.38,1]); this.rect(pos.x-12,pos.y-59,24,1,[1,.82,.38,.8]) }
    }
  }
  destroy() { cancelAnimationFrame(this.frame); this.resizeObserver.disconnect(); if (this.soldierTexture) this.gl.deleteTexture(this.soldierTexture); window.removeEventListener('keydown',this.keydown); window.removeEventListener('keyup',this.keyup); window.removeEventListener('blur',this.blur); this.canvas.removeEventListener('pointermove',this.pointer); this.canvas.removeEventListener('pointerdown',this.pointerDown); window.removeEventListener('pointerup',this.pointerUp); this.canvas.removeEventListener('wheel',this.wheel) }
}
