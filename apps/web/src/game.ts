import { ARENA_H, ARENA_W, viewForCanvas } from './mobile.ts'
import type { View } from './mobile.ts'
import { polygonVertexBuffer, type MapPolygon } from './map-render.ts'
import { InputSystem } from './input/index.ts'
import type { InputFrame, InterfaceEvent } from './input/index.ts'
import { captureCanvas, downloadBlob } from './input/screenshot.ts'
import { aimFromStick, STICK_AIM_RANGE } from './input/touch.ts'
import { aimFromPointer } from './input/mouse.ts'
import { ParticleField } from './render/particles.ts'
import { buildRig, defaultAppearance, readPose, burningTint, bloodStain, type Appearance } from './render/gostek.ts'
import { ScreenShake, blastShake, bonusOverlay, bulletTimeScale, damageFeedback } from './render/effects.ts'
import { defaultQuality, drawingBufferSize, particleLimit, particleBudget, textureParameters, type QualitySettings } from './render/quality.ts'
import { defaultWeather, makeWeather, stepWeather, windForce, type WeatherDrop, type WeatherSettings } from './render/weather.ts'
import { bulletTrail, flagSprite, kitSprite, projectileSprite } from './render/objects.ts'
import { edgesOf, polygonColor, textureCoords } from './render/map.ts'
import { defaultLayout, placeLayout, presetLayout, type HudLayout, type HudPreset, type PlacedElement } from './hud/layout.ts'
import { ammoGauge, armorGauge, damageVignette, fireIntervalGauge, gaugeFillRect, healthGauge, jetGauge, playerTint } from './hud/gauges.ts'
import { blips, boundsOf, crosshair, projectToMinimap, sniperLine } from './hud/minimap.ts'
import { BandwidthMeter, FrameMeter, PingMeter } from './hud/net.ts'
import type { KillFeedEntry } from './hud/feed.ts'
import type { Action } from './input/actions.ts'
import type { WireMatchState, WireRules } from './match.ts'
import type { WireModifiers } from './modifiers.ts'
import type { WireObjectives } from './objectives.ts'
import type { WireTimedEffect } from './bonuses.ts'
import type { WireSpectator } from './spectate.ts'
import { DEFAULT_WEAPON_NAMES, fireRefusalText, muzzleOrigin, nearestPickup, ownedSlots, syncWeaponFromSnapshot, type WireInventory, type WireWeaponTable } from './weapons.ts'
import { AudioEngine } from './audio/engine.ts'
import { DemoPlayer, DemoRecorder } from './replay/player.ts'
import { InputRing, interpolate, reconcile } from './network/prediction.ts'

export type Vec2 = { x: number; y: number }
export type Input = InputFrame
export type Player = { id: number; name: string; pos: Vec2; vel: Vec2; hp: number; fuel: number; kills: number; deaths: number; team: number; grounded: boolean; cooldown: number; respawn: number; last_seq: number; weapon: number; ammo: number; reload_timer: number; startup: number; inventory?: WireInventory; grenades: number; grenade_cooldown: number; grenade_held: boolean; armor: number; assists: number; spawn_protection: number; bleed: unknown; last_damage_direction: Vec2; accuracy?: number; recoil_aim?: number; state?: { pose?: string }; bonus?: WireTimedEffect; unlocked?: number }
export type RagdollSegment = { region: string; pos: Vec2; vel: Vec2; radius: number; grounded: boolean }
export type Ragdoll = { player: number; team: number; segments: RagdollSegment[]; ticks_left: number; gibbed: boolean }
export type WorldEvent =
  | { Blood: { target: number; position: Vec2; direction: Vec2; amount: number } }
  | { Gibs: { target: number; position: Vec2; velocity: Vec2 } }
  | { KillFeed: KillFeedEntry }
  | Record<string, unknown>
export const weaponNames = [...DEFAULT_WEAPON_NAMES]
/** Weapons that draw a sniper line, which is an aid for the player holding one and nobody else. */
const SCOPED_WEAPONS = new Set(['Ruger 77', 'Barrett M82A1'])
const PROJECTILE_SPRITES: Record<string, 'grenade' | 'cluster' | 'arrow' | 'bullet' | 'rocket'> = {
  FragGrenade: 'grenade',
  ClusterGrenade: 'cluster',
  M79Grenade: 'rocket',
  LawRocket: 'rocket',
  Arrow: 'arrow',
  ThrownKnife: 'arrow',
}
type ParticleTint = (fade: number) => number[]
const PARTICLE_COLORS: Record<string, ParticleTint> = {
  flash: fade => [1, .85, .35, fade],
  casing: fade => [.78, .62, .28, fade],
  gib: fade => [.55, .10, .10, fade],
  spark: fade => [1, .92, .62, fade],
  smoke: fade => [.38, .38, .4, fade * .45],
  fire: fade => [1, .52, .18, fade],
  explosion: fade => [1, .86, .55, fade * .8],
  trail: fade => [1, .86, .5, fade * .5],
  blood: fade => [.78, .12, .14, fade],
}
export type Projectile = { id: number; pos: Vec2; owner: number }
export type GroundObject = { kind: string; pos: Vec2; weapon_slot?: number | null; active?: boolean }
export type World = { tick: number; mode: string; players: Record<string, Player>; projectiles: Projectile[]; objects?: GroundObject[]; map_polygons: MapPolygon[]; ragdolls: Ragdoll[]; scores: number[]; events: WorldEvent[]; weapons?: WireWeaponTable; movement?: { fuel_capacity?: number }; rules?: WireRules; match_state?: WireMatchState; objectives?: WireObjectives; spectators?: Record<string, WireSpectator>; stats?: Record<string, unknown>; pickups?: { items: { kind: string; body: { pos: Vec2 } }[] } }
export type Room = { id: number; name: string; mode: string; players: number; capacity: number; map: string; weapon_mod?: string; weapon_hash?: number; modifiers?: WireModifiers; ruleset?: string | null; password?: boolean; ping?: number; version?: number; required_mod?: string | null; region?: string | null }
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
  /** A procedural surface for the terrain, generated here rather than shipped as a file. */
  terrainTexture: WebGLTexture | null = null
  /** Every GL object this client made, so teardown can release all of them rather than some. */
  shaders: WebGLShader[] = []
  released = false
  world: World | null = null
  previousWorld: World | null = null
  snapshotAt = performance.now()
  localId = 0
  predicted: Player | null = null
  predict: Predict | null = null
  send: (input: Input) => void
  input = new InputSystem()
  particles = new ParticleField()
  audio = new AudioEngine()
  ring = new InputRing()
  /** The whole-screen effects, each of which decays on its own. */
  shake = new ScreenShake()
  quality: QualitySettings = defaultQuality()
  weather: WeatherSettings = defaultWeather()
  weatherDrops: WeatherDrop[] = []
  /** What the local player looks like; other players carry their own once profiles land. */
  appearance: Appearance = defaultAppearance()
  hudLayout: HudLayout = defaultLayout()
  hudScale = 1
  /** The three honest numbers behind the network corner. */
  frames = new FrameMeter()
  bandwidth = new BandwidthMeter()
  ping = new PingMeter()
  /** When each input frame was sent, so ping is measured from an acknowledgement rather than guessed. */
  sentAt = new Map<number, number>()
  /** When the local player was last hit, for the red edges and the direction arrow. */
  lastDamageAt = -Infinity
  lastDamageDirection: Vec2 | null = null
  /** Whether the picture is running slowly for effect; the simulation never is. */
  bulletTime = false
  onInterfaceEvent: (event: InterfaceEvent) => void = () => {}
  onKills: (entries: KillFeedEntry[]) => void = () => {}
  /** A one-line note for the player, such as why a shot did not go off. */
  onNotice: (text: string) => void = () => {}
  onUiChange: (ui: { weapon: number; scoreboardVisible: boolean; scoreboardOffset: number; minimap: boolean; sniperLine: boolean; performanceStats: boolean; weaponStats: boolean }) => void = () => {}
  uiSignature = ''
  /** Blood that stayed on each soldier, 0–1, so a hit is still visible after the spray fades. */
  blood = new Map<number, number>()
  aim: Vec2 = { x: 600, y: 350 }
  last = performance.now()
  accumulator = 0
  frame = 0
  resizeObserver: ResizeObserver
  view: View = { x: 0, y: 0, width: 1200, height: 700 }
  recorder: DemoRecorder | null = null
  demo: DemoPlayer | null = null
  backgroundUrl = ''

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
    this.shaders.push(vertex, fragment)
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
    this.shaders.push(spriteVertex, spriteFragment)
    this.spriteProgram = spriteProgram; this.spriteBuffer = gl.createBuffer()!
    this.spritePosition = gl.getAttribLocation(spriteProgram,'a_position'); this.spriteUv = gl.getAttribLocation(spriteProgram,'a_uv')
    this.spriteResolution = gl.getUniformLocation(spriteProgram,'u_resolution')!; this.spriteCamera = gl.getUniformLocation(spriteProgram,'u_camera')!; this.spriteTint = gl.getUniformLocation(spriteProgram,'u_tint')!
    const soldier = new Image()
    soldier.src = '/art/soldier.png'
    soldier.onload = () => { const texture = gl.createTexture(); if (!texture) return; gl.bindTexture(gl.TEXTURE_2D,texture); gl.texImage2D(gl.TEXTURE_2D,0,gl.RGBA,gl.RGBA,gl.UNSIGNED_BYTE,soldier); gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MIN_FILTER,gl.LINEAR); gl.texParameteri(gl.TEXTURE_2D,gl.TEXTURE_MAG_FILTER,gl.LINEAR); this.soldierTexture = texture }
    this.terrainTexture = this.makeTerrainTexture()
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
  /** Applies a quality setting to everything that reads one. */
  setQuality(quality: QualitySettings) {
    this.quality = quality
    this.particles.limit = particleLimit(quality.particles)
    this.particles.budget = particleBudget(quality.particles)
    if (!quality.weather) this.weatherDrops = []
    if (this.terrainTexture) { this.gl.deleteTexture(this.terrainTexture); this.terrainTexture = this.makeTerrainTexture() }
    this.resize()
  }

  /** Switches the HUD between its presets, or to a custom layout a player loaded. */
  setHudLayout(layout: HudLayout | HudPreset) {
    this.hudLayout = typeof layout === 'string' ? presetLayout(layout) : layout
  }

  setWeather(weather: WeatherSettings) {
    this.weather = weather
    this.weatherDrops = []
  }

  suspendInput() { this.input.suspend() }
  resumeInput() { this.input.resume() }
  resize() {
    const size = drawingBufferSize(this.canvas.clientWidth, this.canvas.clientHeight, window.devicePixelRatio || 1, this.quality)
    this.canvas.width = size.width; this.canvas.height = size.height
    this.gl.viewport(0, 0, this.canvas.width, this.canvas.height)
  }
  updateView() {
    const player = this.predicted ?? this.world?.players[this.localId]
    const rect = this.canvas.getBoundingClientRect()
    this.view = viewForCanvas(rect.width, rect.height, player?.pos.x ?? 600, player?.pos.y ?? 350, this.aim.x, this.aim.y)
    const shake = this.shake.offset()
    this.view.x += shake.x; this.view.y += shake.y
    if (this.backgroundUrl) this.canvas.style.backgroundImage = `url("${this.backgroundUrl}")`
    this.canvas.style.backgroundSize = `${ARENA_W / this.view.width * 100}% ${ARENA_H / this.view.height * 100}%`
    this.canvas.style.backgroundPosition = `${this.view.width >= ARENA_W ? 50 : this.view.x / (ARENA_W - this.view.width) * 100}% ${this.view.height >= ARENA_H ? 50 : this.view.y / (ARENA_H - this.view.height) * 100}%`
  }
  setSnapshot(world: World) {
    this.previousWorld = this.world; this.world = world; this.snapshotAt = performance.now()
    const me = world.players[this.localId]
    if (me) {
      this.ring.dropThrough(me.last_seq)
      const replayed = reconcile({ id: me.id, pos: me.pos, vel: me.vel, last_seq: me.last_seq }, this.ring.frames)
      this.predicted = { ...structuredClone(me), pos: replayed.pos, vel: replayed.vel, last_seq: replayed.last_seq }
    } else {
      this.predicted = null
    }
    if (me) {
      // The server says which of our frames it has seen; the round trip is the honest ping.
      const sent = this.sentAt.get(me.last_seq)
      if (sent !== undefined) {
        this.ping.sample(performance.now() - sent)
        for (const seq of [...this.sentAt.keys()]) if (seq <= me.last_seq) this.sentAt.delete(seq)
      }
    }
    if (me && me.hp > 0) this.input.weapon = syncWeaponFromSnapshot(this.input.weapon, me.weapon, ownedSlots(me.inventory))
    this.consumeEvents(world)
    this.recorder?.record(world.tick, String(world.tick))
  }
  /** Turns authoritative damage events into cosmetics; nothing here is sent back to the server. */
  consumeEvents(world: World) {
    const kills: KillFeedEntry[] = []
    const listener = world.players[this.localId]?.pos ?? { x: 0, y: 0 }
    this.audio.lastListener = listener
    for (const event of world.events ?? []) {
      const blood = (event as { Blood?: { target: number; position: Vec2; direction: Vec2; amount: number } }).Blood
      if (blood) {
        this.particles.emitBlood(blood.position, blood.direction, blood.amount, world.tick * 31 + blood.target)
        const stain = Math.min(1, (this.blood.get(blood.target) ?? 0) + Math.min(1, blood.amount / 40))
        this.blood.set(blood.target, stain)
      }
      const respawn = (event as { Respawn?: { player: number } }).Respawn
      if (respawn) this.blood.delete(respawn.player)
      const gibs = (event as { Gibs?: { target: number; position: Vec2; velocity: Vec2 } }).Gibs
      if (gibs) this.particles.emitGibs(gibs.position, gibs.velocity, world.tick * 17 + gibs.target)
      const feed = (event as { KillFeed?: KillFeedEntry }).KillFeed
      if (feed) kills.push(feed)
      const flash = (event as { MuzzleFlash?: { player: number; pos: Vec2 } }).MuzzleFlash
      if (flash) this.particles.emitFlash(flash.pos)
      const explosion = (event as { Explosion?: { pos: Vec2; radius: number } }).Explosion
      if (explosion) {
        this.particles.emitExplosion(explosion.pos, explosion.radius, world.tick * 13 + 7)
        const me = this.predicted ?? world.players[this.localId]
        if (me) {
          const distance = Math.hypot(me.pos.x - explosion.pos.x, me.pos.y - explosion.pos.y)
          this.shake.add(blastShake(distance, explosion.radius * 4), world.tick)
        }
      }
      const impact = (event as { Impact?: { pos: Vec2; normal: Vec2 } }).Impact
      if (impact) this.particles.emitSparks(impact.pos, impact.normal, world.tick * 7 + 3)
      const damage = (event as { Damage?: { target: number; direction: Vec2 } }).Damage
      if (damage && damage.target === this.localId) {
        this.lastDamageAt = performance.now()
        this.lastDamageDirection = damage.direction
      }
      const casing = (event as { Casing?: { player: number; pos: Vec2 } }).Casing
      if (casing) this.particles.emitCasing(casing.pos)
      // A refused shot is not a dropped packet: say why, so the player knows to crouch.
      const refused = (event as { FireRefused?: { player: number; reason: string } }).FireRefused
      if (refused && refused.player === this.localId) this.onNotice(fireRefusalText(refused.reason))
      const named = Object.keys(event)[0]
      if (named) {
        const body = (event as Record<string, { pos?: Vec2 } | undefined>)[named]
        this.audio.hear(named, { at: body?.pos, listener })
      }
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
    const owned = player && player.hp > 0 ? ownedSlots(player.inventory) : undefined
    const { frame, events } = this.input.tick({ aim: this.aim, airborne: player ? !player.grounded : false, gamepad, owned })
    if (this.input.ui.fastForward && this.demo) this.demo.fastForward()
    for (const event of events) {
      if (event.type === 'screenshot') captureCanvas(this.canvas, (blob, filename) => downloadBlob(blob as Blob, filename))
      if (event.type === 'demo') {
        if (event.recording) this.recorder = new DemoRecorder()
        else this.flushDemo()
      }
      this.onInterfaceEvent(event)
    }
    const ui = {
      weapon: this.input.weapon,
      scoreboardVisible: this.input.ui.scoreboardVisible,
      scoreboardOffset: this.input.ui.scoreboardOffset,
      minimap: this.input.ui.minimap,
      sniperLine: this.input.ui.sniperLine,
      performanceStats: this.input.ui.performanceStats,
      weaponStats: this.input.ui.weaponStats,
    }
    const signature = `${ui.weapon}:${ui.scoreboardVisible}:${ui.scoreboardOffset}:${ui.minimap}:${ui.sniperLine}:${ui.performanceStats}:${ui.weaponStats}`
    if (signature !== this.uiSignature) { this.uiSignature = signature; this.onUiChange(ui) }
    this.sentAt.set(frame.seq, performance.now())
    // Only the last second of frames can still be acknowledged; the rest is dead weight.
    if (this.sentAt.size > 120) { const oldest = this.sentAt.keys().next().value; if (oldest !== undefined) this.sentAt.delete(oldest) }
    this.bandwidth.sent(JSON.stringify(frame).length, performance.now())
    this.send(frame)
    this.ring.push({ seq: frame.seq, left: frame.left, right: frame.right, jump: frame.jump, jet: frame.jet, aim: frame.aim })
    this.audio.step(1 / 60)
    if (this.predict && this.predicted) {
      try {
        const result = this.predict(JSON.stringify(this.predicted), JSON.stringify(frame))
        if (result) this.predicted = JSON.parse(result) as Player
      } catch { this.predict = null }
    } else if (this.predicted && this.world.players[this.localId]) {
      const acked = this.world.players[this.localId]
      const replayed = reconcile({ id: acked.id, pos: acked.pos, vel: acked.vel, last_seq: acked.last_seq }, this.ring.frames)
      this.predicted = { ...this.predicted, pos: replayed.pos, vel: replayed.vel, last_seq: replayed.last_seq }
    }
  }
  /** Draws in screen pixels rather than world units, for the HUD on top of the match. */
  screenRect(x: number, y: number, w: number, h: number, c: number[]) {
    const gl = this.gl
    const rect = this.canvas.getBoundingClientRect()
    gl.uniform2f(this.resolution, rect.width || this.view.width, rect.height || this.view.height)
    gl.uniform2f(this.camera, 0, 0)
    this.rect(x, y, w, h, c)
    gl.uniform2f(this.resolution, this.view.width, this.view.height)
    gl.uniform2f(this.camera, this.view.x, this.view.y)
  }

  /** The soldier, drawn as the layered rig rather than one flat sprite. */
  drawSoldier(player: Player, pos: Vec2, aim: Vec2, jetting: boolean) {
    const overlay = bonusOverlay(player.bonus?.active ?? undefined, player.id === this.localId)
    const stain = this.blood.get(player.id) ?? 0
    const appearance = stain > 0
      ? { ...this.appearance, shirt: bloodStain(this.appearance.shirt, stain), pants: bloodStain(this.appearance.pants, stain) }
      : this.appearance
    const rig = buildRig({
      pos,
      aim,
      pose: readPose(player.state?.pose, player.grounded, player.hp),
      appearance,
      team: playerTint(player.team, player.id === this.localId),
      tick: this.world?.tick ?? 0,
      jetting,
      moving: Math.abs(player.vel.x) > 10,
    })
    const burn = (player as { burning?: number }).burning ?? 0
    for (const part of rig) {
      const color = burningTint(part.color, burn)
      this.rect(part.x - part.width / 2, part.y - part.height / 2, part.width, part.height, [
        color[0], color[1], color[2], color[3] * overlay.playerAlpha,
      ])
    }
  }

  /** The minimap, the blips on it, and nothing the screen is not already showing. */
  drawMinimap(panel: PlacedElement) {
    if (!this.world || !this.input.ui.minimap) return
    const bounds = boundsOf(this.world.map_polygons ?? [])
    this.screenRect(panel.x, panel.y, panel.width, panel.height, [.04,.07,.09,.7])
    for (const polygon of this.world.map_polygons ?? []) {
      const points = (polygon.vertices ?? []).map(vertex => projectToMinimap(vertex, bounds, panel))
      for (const point of points) this.screenRect(point.x - 1, point.y - 1, 2, 2, [.45,.48,.5,.8])
    }
    const marks = blips(Object.values(this.world.players), bounds, panel, this.localId)
    for (const blip of marks) {
      this.screenRect(blip.x - blip.radius, blip.y - blip.radius, blip.radius * 2, blip.radius * 2, blip.color)
    }
  }

  rect(x: number, y: number, w: number, h: number, c: number[]) { const gl = this.gl; gl.uniform4f(this.color, c[0], c[1], c[2], c[3] ?? 1); gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([x,y,x+w,y,x,y+h,x,y+h,x+w,y,x+w,y+h]), gl.STREAM_DRAW); gl.drawArrays(gl.TRIANGLES, 0, 6) }
  /**
   * A grain of rock, built here from a seeded pattern.
   *
   * Generating it means the terrain has a surface without shipping an image for it, and it is the
   * same surface on every machine, which is what lets a screenshot comparison mean anything.
   */
  makeTerrainTexture(size = 64): WebGLTexture | null {
    const gl = this.gl
    const texture = gl.createTexture()
    if (!texture) return null
    const pixels = new Uint8Array(size * size * 4)
    let state = 0x9e3779b9
    for (let i = 0; i < size * size; i += 1) {
      state = (Math.imul(state ^ (state >>> 15), state | 1) + 0x6d2b79f5) >>> 0
      const grain = 200 + ((state >>> 24) % 56)
      pixels[i * 4] = grain; pixels[i * 4 + 1] = grain; pixels[i * 4 + 2] = grain; pixels[i * 4 + 3] = 255
    }
    gl.bindTexture(gl.TEXTURE_2D, texture)
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, size, size, 0, gl.RGBA, gl.UNSIGNED_BYTE, pixels)
    const parameters = textureParameters(this.quality, true)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl[parameters.minFilter])
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl[parameters.magFilter])
    if (parameters.generateMipmap) gl.generateMipmap(gl.TEXTURE_2D)
    return texture
  }

  /**
   * One polygon, textured and lit.
   *
   * The texture is projected from world coordinates rather than stretched to the triangle, so two
   * polygons that meet show one continuous surface instead of a seam down the join.
   */
  polygon(polygon: MapPolygon) {
    const gl = this.gl
    const color = polygonColor(polygon.kind)
    if (this.terrainTexture && !this.quality.compatibility) {
      const uvs = textureCoords(polygon)
      const vertices = polygon.vertices
      const data = new Float32Array(3 * 4)
      for (let i = 0; i < 3; i += 1) {
        data[i * 4] = vertices[i].x; data[i * 4 + 1] = vertices[i].y
        data[i * 4 + 2] = uvs[i].u; data[i * 4 + 3] = uvs[i].v
      }
      gl.useProgram(this.spriteProgram)
      gl.bindBuffer(gl.ARRAY_BUFFER, this.spriteBuffer)
      gl.bufferData(gl.ARRAY_BUFFER, data, gl.STREAM_DRAW)
      gl.enableVertexAttribArray(this.spritePosition); gl.vertexAttribPointer(this.spritePosition,2,gl.FLOAT,false,16,0)
      gl.enableVertexAttribArray(this.spriteUv); gl.vertexAttribPointer(this.spriteUv,2,gl.FLOAT,false,16,8)
      gl.uniform2f(this.spriteResolution,this.view.width,this.view.height); gl.uniform2f(this.spriteCamera,this.view.x,this.view.y)
      gl.uniform4f(this.spriteTint,color[0],color[1],color[2],color[3])
      gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D,this.terrainTexture)
      gl.drawArrays(gl.TRIANGLES,0,3)
      this.useWorldProgram()
    } else {
      gl.uniform4f(this.color,color[0],color[1],color[2],color[3])
      gl.bufferData(gl.ARRAY_BUFFER,polygonVertexBuffer([polygon]),gl.STREAM_DRAW)
      gl.drawArrays(gl.TRIANGLES,0,3)
    }
    // Only the edges facing the light are drawn, so a platform reads as ground with a top rather
    // than as an outlined shape.
    for (const edge of edgesOf(polygon)) {
      const dx = edge.to.x - edge.from.x
      const dy = edge.to.y - edge.from.y
      const length = Math.hypot(dx, dy) || 1
      const steps = Math.min(64, Math.max(2, Math.round(length / 6)))
      for (let i = 0; i < steps; i += 1) {
        const at = i / steps
        this.rect(edge.from.x + dx * at - 1, edge.from.y + dy * at - 1, edge.width, edge.width, edge.color)
      }
    }
  }

  /** Puts the plain-triangle program back, after something borrowed the sprite one. */
  useWorldProgram() {
    const gl = this.gl
    gl.useProgram(this.program); gl.bindBuffer(gl.ARRAY_BUFFER,this.buffer)
    gl.enableVertexAttribArray(this.position); gl.vertexAttribPointer(this.position,2,gl.FLOAT,false,0,0)
    gl.uniform2f(this.resolution,this.view.width,this.view.height); gl.uniform2f(this.camera,this.view.x,this.view.y)
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
    const dt = (1 / 60) * bulletTimeScale(this.bulletTime)
    this.frames.frame(performance.now())
    this.shake.step(dt)
    this.particles.step(dt)
    this.drawWeather(dt)
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
      const color = PARTICLE_COLORS[particle.kind](fade)
      this.rect(particle.x - particle.size / 2, particle.y - particle.size / 2, particle.size, particle.size, color)
    }
    for (const object of this.world.objects ?? []) {
      if (object.active === false) continue
      const sprite = object.kind === 'Flag'
        ? flagSprite(String((object as { team?: string }).team ?? 'Alpha'), object.pos, 'dropped', this.world.tick)
        : kitSprite(String(object.kind ?? 'Medical'), object.pos, this.world.tick)
      this.rect(sprite.x - sprite.width / 2, sprite.y - sprite.height / 2, sprite.width, sprite.height, sprite.color)
    }
    for (const item of this.world.pickups?.items ?? []) {
      const sprite = kitSprite(String(item.kind), item.body.pos, this.world.tick)
      this.rect(sprite.x - sprite.width / 2, sprite.y - sprite.height / 2, sprite.width, sprite.height, sprite.color)
    }
    const blend = Math.min(1,(performance.now()-this.snapshotAt)/50)
    for (const projectile of this.world.projectiles) {
      const velocity = (projectile as { vel?: Vec2 }).vel ?? { x: 0, y: 0 }
      const trail = bulletTrail(projectile.pos, velocity)
      this.rect(Math.min(trail.from.x, trail.to.x), trail.to.y - trail.width / 2, Math.abs(trail.to.x - trail.from.x), trail.width, trail.color)
      const sprite = projectileSprite(PROJECTILE_SPRITES[String((projectile as { kind?: string }).kind)] ?? 'bullet', projectile.pos, velocity, this.world.tick)
      this.rect(sprite.x - sprite.width / 2, sprite.y - sprite.height / 2, sprite.width, sprite.height, sprite.color)
    }
    for (const p of Object.values(this.world.players)) {
      if (p.hp <= 0) continue
      const previous = this.previousWorld?.players[p.id]
      const pos = p.id === this.localId && this.predicted ? this.predicted.pos : previous ? interpolate(previous.pos, p.pos, blend) : p.pos
      const cap = this.world.movement?.fuel_capacity ?? 1
      const jetting = p.id === this.localId ? this.input.state.held('jet') : !p.grounded && p.fuel < cap - 0.001
      if (jetting && p.hp > 0) {
        this.rect(pos.x-13,pos.y+9,8,18,[.32,.69,1,.35]); this.rect(pos.x-11,pos.y+12,4,12,[.68,.91,1,.85])
      }
      const aimAt = p.id === this.localId ? this.aim : { x: pos.x + (p.vel.x < -10 ? -40 : 40), y: pos.y }
      this.drawSoldier(p, pos, aimAt, jetting)
      this.rect(pos.x-20,pos.y-53,40,4,[.04,.07,.09,.85])
      this.rect(pos.x-19,pos.y-52,38*p.hp/100,2,[p.hp<30 ? 1 : .88,p.hp<30 ? .25 : .72,.28,1])
      if (p.id === this.localId) { this.rect(pos.x-2,pos.y-61,4,4,[1,.82,.38,1]); this.rect(pos.x-12,pos.y-59,24,1,[1,.82,.38,.8]) }
    }
    const local = this.predicted ?? this.world.players[this.localId]
    if (local && local.hp > 0) {
      const nearby = nearestPickup(local.pos, this.world.objects)
      if (nearby) this.rect(nearby.pos.x - 10, nearby.pos.y - 10, 20, 20, [1,.82,.38,.28])
    }
    const me = this.world.players[this.localId]
    if (me && me.hp > 0) {
      const cursor = crosshair(me.accuracy ?? 0, (me as { bink?: number }).bink ?? 0, (me.cooldown ?? 0) > 0)
      this.rect(this.aim.x - cursor.spread, this.aim.y + cursor.offsetY - 1, cursor.spread * 2, 2, cursor.color)
      this.rect(this.aim.x - 1, this.aim.y + cursor.offsetY - cursor.spread, 2, cursor.spread * 2, cursor.color)
      const scoped = this.input.ui.sniperLine && SCOPED_WEAPONS.has(weaponNames[this.input.weapon] ?? '')
      const line = sniperLine(muzzleOrigin(me.pos, this.aim, me.state?.pose), this.aim, { enabled: scoped })
      if (line) {
        const length = Math.hypot(line.to.x - line.from.x, line.to.y - line.from.y)
        const steps = Math.min(120, Math.round(length / 12))
        for (let i = 0; i < steps; i += 1) {
          const at = i / steps
          this.rect(line.from.x + (line.to.x - line.from.x) * at, line.from.y + (line.to.y - line.from.y) * at, 2, 1, line.color)
        }
      }
    }
    this.drawHud()
  }

  /** Rain or snow, drawn in screen space because weather is in front of the camera, not in it. */
  drawWeather(dt: number) {
    if (!this.quality.weather || this.weather.kind === 'none') return
    const rect = this.canvas.getBoundingClientRect()
    const view = { width: rect.width || this.view.width, height: rect.height || this.view.height }
    if (this.weatherDrops.length === 0) {
      this.weatherDrops = makeWeather(this.weather, view, 1, particleBudget(this.quality.particles))
    }
    stepWeather(this.weatherDrops, dt, view, this.weather, performance.now() / 1000)
    const wind = windForce(this.weather)
    for (const drop of this.weatherDrops) {
      const lean = (wind / 900) * drop.length
      this.screenRect(drop.x, drop.y, Math.max(1, Math.abs(lean) / 4 + 1), drop.length, [.78,.85,.95,drop.alpha])
    }
  }

  /** The gauges, the minimap, and the red edges: everything measured in screen pixels. */
  drawHud() {
    if (!this.world) return
    const rect = this.canvas.getBoundingClientRect()
    const viewport = { width: rect.width || this.view.width, height: rect.height || this.view.height, scale: this.hudScale }
    const me = this.predicted ?? this.world.players[this.localId]
    const placed = placeLayout(this.hudLayout, viewport)
    const magazine = Math.max(me?.ammo ?? 0, 1)
    const gauges = me
      ? {
          health: healthGauge(me.hp),
          jet: jetGauge(me.fuel, this.world.movement?.fuel_capacity ?? 1),
          ammo: ammoGauge({ ammo: me.ammo, magazine, reloadTimer: me.reload_timer, reloadTicks: 108 }),
          'fire-interval': fireIntervalGauge(me.cooldown ?? 0, 10),
          armor: armorGauge(me.armor ?? 0),
        }
      : null
    for (const element of placed) {
      if (element.id === 'minimap') { this.drawMinimap(element); continue }
      const gauge = gauges?.[element.id as keyof NonNullable<typeof gauges>]
      if (!gauge) continue
      this.screenRect(element.x, element.y, element.width, element.height, gauge.track)
      const fill = gaugeFillRect(gauge, element)
      this.screenRect(fill.x, fill.y, fill.width, fill.height, gauge.color)
    }
    if (!me) return
    const since = (performance.now() - this.lastDamageAt) / 1000
    const feedback = damageFeedback(damageVignette(me.hp, since), this.lastDamageDirection, since)
    if (feedback.vignette > 0) {
      const edge = Math.max(8, viewport.height * 0.06)
      const tint = [.85,.12,.12,feedback.vignette * .5]
      this.screenRect(0, 0, viewport.width, edge, tint)
      this.screenRect(0, viewport.height - edge, viewport.width, edge, tint)
      this.screenRect(0, 0, edge, viewport.height, tint)
      this.screenRect(viewport.width - edge, 0, edge, viewport.height, tint)
    }
    if (feedback.arrow !== null) {
      const radius = Math.min(viewport.width, viewport.height) * 0.18
      const cx = viewport.width / 2 + Math.cos(feedback.arrow) * radius
      const cy = viewport.height / 2 + Math.sin(feedback.arrow) * radius
      this.screenRect(cx - 6, cy - 6, 12, 12, [.95,.25,.22,feedback.alpha])
    }
    const overlay = bonusOverlay(me.bonus?.active ?? undefined, true)
    if (overlay.tint[3] > 0) this.screenRect(0, 0, viewport.width, viewport.height, overlay.tint)
  }
  flushDemo() {
    if (!this.recorder || this.recorder.chunks.length === 0) {
      this.recorder = null
      return
    }
    const file = this.recorder.toFile({
      format: 1,
      protocol: 15,
      map: this.world?.mode ?? 'demo',
      mode: this.world?.mode ?? 'deathmatch',
      weapon_hash: this.world?.weapons?.hash ?? 0,
      seed: 0,
      source_revision: 'client',
    })
    downloadBlob(new Blob([JSON.stringify(file)], { type: 'application/json' }), `demo-${file.header.mode}.json`)
    this.recorder = null
  }

  observe() {
    return {
      frame: this.input.lastFrame,
      world: this.world,
      localId: this.localId,
      pixels: this.canvas.toDataURL('image/png'),
    }
  }
  /**
   * Gives back everything this client took: the frame, the observer, every GL object, and every
   * listener. A player who joins and leaves a hundred matches in one session should end where they
   * started, so teardown releases all of it rather than only the texture that was easy to see.
   */
  destroy() {
    cancelAnimationFrame(this.frame); this.resizeObserver.disconnect()
    if (!this.released) {
      this.released = true
      const gl = this.gl
      for (const shader of this.shaders) gl.deleteShader(shader)
      this.shaders = []
      gl.deleteProgram(this.program); gl.deleteProgram(this.spriteProgram)
      gl.deleteBuffer(this.buffer); gl.deleteBuffer(this.spriteBuffer)
      if (this.soldierTexture) { gl.deleteTexture(this.soldierTexture); this.soldierTexture = null }
      if (this.terrainTexture) { gl.deleteTexture(this.terrainTexture); this.terrainTexture = null }
    }
    window.removeEventListener('keydown',this.keydown); window.removeEventListener('keyup',this.keyup); window.removeEventListener('blur',this.blur); this.canvas.removeEventListener('pointermove',this.pointer); this.canvas.removeEventListener('pointerdown',this.pointerDown); window.removeEventListener('pointerup',this.pointerUp); this.canvas.removeEventListener('wheel',this.wheel)
  }
}
