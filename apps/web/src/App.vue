<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import { GameClient, type Input, type Room, type World } from './game'
import { acceptWeaponTable, DEFAULT_WEAPON_NAMES, nearestPickup, ownedSlots } from './weapons'
import { clockText, phaseBanner, winnerName } from './match'
import { flagStatusLines, isCarrying } from './objectives'
import { COMMUNITY_MODES, deathMessage, modifierBadges, modifierSummary } from './modifiers'
import { bonusOverlay, bonusText, hasBonus, kitName } from './bonuses'
import { commandForKey, isSpectating, spectatorLabel } from './spectate'
import { accuracyPercent, behindText, limitText, weaponRows } from './stats'
import { BOT_DIFFICULTIES, MAX_BOTS, createRoomBots, describeBots, setBotsMessage, type BotDifficulty } from './bots'
import { defaultFilter, filterRooms, parseHostPort, PingAll, roomFlags } from './lobby.ts'
import { disconnectText } from './network/prediction.ts'
import { PlayerStore, toggleFavorite } from './profiles/player.ts'
import MobileControls from './MobileControls.vue'
import ControlsSettings from './ControlsSettings.vue'
import VideoSettings from './VideoSettings.vue'
import { networkPanel } from './hud/net.ts'
import { endScreen, rowLabel, scoreboardRows, scoreboardWindow, showIds, weaponScreen } from './hud/screens.ts'
import { flagIndicators, limitLine, MessageLog, standingFor, teamScores, weaponPanel } from './hud/status.ts'
import { type HudLayout } from './hud/layout.ts'
import { defaultQuality, type QualitySettings } from './render/quality.ts'
import { defaultWeather, type WeatherSettings } from './render/weather.ts'
import type { HudPreset } from './hud/layout.ts'
import { InputSystem } from './input'
import { ProfileStore } from './input/profiles'
import { KillFeed, respawnCountdown, damageArrowAngle, type KillFeedEntry, type FeedLine } from './hud/feed'
import type { InterfaceEvent } from './input'
import type { Action } from './input/actions'

const playerProfiles = new PlayerStore(localStorage)
const name = ref(playerProfiles.active().name || localStorage.getItem('arena-name') || '')
const status = ref('Disconnected')
const error = ref('')
const rooms = ref<Room[]>([])
const roomSearch = ref('')
const hideFullRooms = ref(false)
const hideEmptyRooms = ref(false)
const joinPassword = ref('')
const roomPassword = ref('')
const joinAsSpectator = ref(false)
const directHost = ref('')
const roomPings = new PingAll()
let roomsRequestedAt = 0
const favorites = ref(playerProfiles.active().favorites)
const roomId = ref<number | null>(null)
const playerId = ref(0)
const world = ref<World | null>(null)
const chat = ref<{ name: string; text: string; tone?: string }[]>([])
const chatDraft = ref('')
const chatScope = ref<'all' | 'team'>('all')
const newRoom = ref('New Arena')
const mode = ref('deathmatch')
const maps = ref<{ name: string; mode: string; preview: string }[]>([])
const selectedMap = ref('Aero')
const visibility = ref<'private' | 'public'>('private')
const inviteCode = ref('')
const activeRoom = ref<{ name: string; code: string; public: boolean; ruleset?: string | null } | null>(null)
const canvas = ref<HTMLCanvasElement | null>(null)
const chatField = ref<HTMLInputElement | null>(null)
const mobile = navigator.maxTouchPoints > 0 || matchMedia('(pointer: coarse)').matches
const showInfo = ref(false)
const showControls = ref(false)
const selectedWeapon = ref(0)
const weaponNames = ref<string[]>([...DEFAULT_WEAPON_NAMES])
const weaponMod = ref('')
const realistic = ref(false)
const survival = ref(false)
const advance = ref(false)
const ruleset = ref('')
const botCount = ref(0)
const botDifficulty = ref<BotDifficulty>('normal')
const showVideo = ref(false)
const hudPreset = ref<HudPreset>(mobile ? 'mobile' : 'default')
const hudScale = ref(1)
const quality = ref<QualitySettings>(defaultQuality())
const weather = ref<WeatherSettings>(defaultWeather())
/** The network corner, refreshed on the same timer as the room list rather than every frame. */
const network = ref({ ping: { text: '— ms', grade: 'good', color: [0,0,0,1], radius: 3 }, jitter: 0, fps: 0, worstFrameMs: 0, down: '0 B/s', up: '0 B/s' })
const listedRooms = computed(() =>
  filterRooms(
    roomPings.apply(rooms.value as never),
    {
      ...defaultFilter(),
      search: roomSearch.value,
      hideFull: hideFullRooms.value,
      hideEmpty: hideEmptyRooms.value,
    },
    favorites.value,
  ),
)
const botChoice = computed(() => ({ count: botCount.value, difficulty: botDifficulty.value }))
const botSummary = computed(() => describeBots(botChoice.value))
const scoreboardVisible = ref(false)
const scoreboardOffset = ref(0)
const showMinimap = ref(true)
const showSniperLine = ref(true)
const showPerformance = ref(true)
const showWeaponStats = ref(false)
const customHud = ref<HudLayout | null>(null)
const chatLog = new MessageLog()
const chatOverlay = ref<{ text: string; tone: string }[]>([])
const SCOREBOARD_WINDOW = 8
const feedLines = ref<FeedLine[]>([])
const killFeed = new KillFeed()
let feedTimer = 0

const inputSystem = new InputSystem()
const profileStore = new ProfileStore(localStorage)
inputSystem.applyProfile(profileStore.active())

declare global {
  interface Window {
    __arenaAgent?: {
      set: (action: Action, down: boolean) => void
      aim: (x: number, y: number) => void
      weapon: (index: number) => void
      observe: () => ReturnType<GameClient['observe']> | null
    }
  }
}

function attachArenaAgent() {
  window.__arenaAgent = {
    set: (action, down) => inputSystem.setAction(action, down),
    aim: (x, y) => { if (game) game.aim = { x, y } },
    weapon: index => { inputSystem.weapon = index },
    observe: () => game?.observe() ?? null,
  }
}

function clearArenaAgent() {
  delete window.__arenaAgent
}

let socket: WebSocket | null = null
let game: GameClient | null = null
let reconnectTimer = 0
let shouldReconnect = false
const me = computed(() => world.value?.players[playerId.value])
const dead = computed(() => (me.value?.hp ?? 100) <= 0)
const fuelPct = computed(() => {
  const cap = world.value?.movement?.fuel_capacity || 1
  return Math.round(((me.value?.fuel ?? cap) / cap) * 100)
})
const carried = computed(() => ownedSlots(me.value?.inventory))
const pickupHint = computed(() => {
  const player = me.value
  if (!player || player.hp <= 0) return ''
  const nearby = nearestPickup(player.pos, world.value?.objects)
  if (!nearby || nearby.weapon_slot == null) return ''
  return weaponNames.value[nearby.weapon_slot] || 'Weapon'
})
const respawnIn = computed(() => respawnCountdown(me.value?.respawn ?? 0))
const damageAngle = computed(() => damageArrowAngle(me.value?.last_damage_direction ?? { x: 0, y: 0 }))
function playerName(id: number) {
  return world.value?.players[id]?.name ?? `Player ${id}`
}
function refreshFeed() {
  feedLines.value = [...killFeed.visible(performance.now())]
}
/** Shows a short in-match note, such as why a shot was refused. */
function showNotice(text: string) {
  error.value = text
  clearTimeout(noticeTimer)
  noticeTimer = window.setTimeout(() => { if (error.value === text) error.value = '' }, 2500)
}
let noticeTimer = 0
function recordKills(entries: KillFeedEntry[]) {
  killFeed.push(entries, playerName, performance.now())
  refreshFeed()
}
const matchClock = computed(() => clockText(world.value?.match_state, world.value?.rules))
const matchBanner = computed(() => {
  const banner = phaseBanner(world.value?.match_state)
  if (!banner) return ''
  const names = Object.fromEntries(Object.entries(world.value?.players ?? {}).map(([id, player]) => [id, player.name]))
  const winner = winnerName(world.value?.match_state?.outcome, names)
  return winner ? `${winner} wins` : banner
})
const roomModifiers = computed(() => ({
  modifiers: world.value?.rules?.modifiers,
  ruleset: activeRoom.value?.ruleset ?? null,
}))
const rulesSummary = computed(() => modifierSummary(roomModifiers.value))
function badgesFor(room: Room) {
  return modifierBadges({ modifiers: room.modifiers, ruleset: room.ruleset })
}
const MODE_LABELS: Record<string, string> = {
  deathmatch: 'Deathmatch',
  team: 'Team deathmatch',
  pointmatch: 'Pointmatch',
  ctf: 'Capture the Flag',
  htf: 'Hold the Flag',
  infiltration: 'Infiltration',
  rambomatch: 'Rambomatch',
}
function modeLabel(id: string) {
  return MODE_LABELS[id] ?? 'Deathmatch'
}
/** Only maps built for the selected mode; a flag mode cannot run on a deathmatch layout. */
const MODE_MAP_KINDS: Record<string, string[]> = {
  deathmatch: ['deathmatch', 'teammatch'],
  team: ['deathmatch', 'teammatch'],
  pointmatch: ['deathmatch', 'pointmatch'],
  ctf: ['capturetheflag'],
  htf: ['holdtheflag'],
  infiltration: ['infiltration'],
}
const mapsForMode = computed(() => {
  const wanted = MODE_MAP_KINDS[mode.value] ?? ['deathmatch']
  return maps.value.filter(map => wanted.includes(map.mode.toLowerCase().replace(/[^a-z]/g, '')))
})
const outMessage = computed(() => deathMessage(world.value?.rules?.modifiers, respawnIn.value))
const playerNames = computed(() =>
  Object.fromEntries(Object.entries(world.value?.players ?? {}).map(([id, player]) => [id, player.name])),
)
const flagStatus = computed(() => flagStatusLines(world.value?.objectives, playerNames.value))
const carryingFlag = computed(() => isCarrying(world.value?.objectives, playerId.value ?? -1))
const myBonus = computed(() => me.value?.bonus)
const bonusActive = computed(() => hasBonus(myBonus.value))
const bonusLabel = computed(() => bonusText(myBonus.value))
const bonusClass = computed(() => bonusOverlay(myBonus.value))
/** Kits on the map near enough to be worth pointing out. */
const kitHint = computed(() => {
  const player = me.value
  if (!player || player.hp <= 0) return ''
  const items = world.value?.pickups?.items ?? []
  for (const item of items) {
    const dx = item.body.pos.x - player.pos.x
    const dy = item.body.pos.y - player.pos.y
    if (Math.hypot(dx, dy) <= 26) return kitName(item.kind as never)
  }
  return ''
})
const mySpectator = computed(() => world.value?.spectators?.[String(playerId.value)])
const spectating = computed(() => isSpectating(mySpectator.value))
const spectatorText = computed(() => spectatorLabel(mySpectator.value, playerNames.value))
const matchLimit = computed(() => limitText(world.value?.rules?.limits))
const myStats = computed(() => (world.value?.stats as Record<string, never> | undefined)?.[String(playerId.value)])
const myWeaponRows = computed(() => weaponRows(myStats.value))
const myAccuracy = computed(() => accuracyPercent(myStats.value))
function spectate(command: object) { send({ type: 'spectate', command }) }
function joinSpectators() { send({ type: 'set_team', team: 'spectator' }) }
function stopSpectating() { send({ type: 'set_team', team: 'auto' }) }
/** Previous, next, and free camera while spectating; every other key means what it always did. */
function spectatorKey(event: KeyboardEvent) {
  if (!spectating.value) return
  const command = commandForKey(event.key)
  if (!command) return
  event.preventDefault()
  if (command.type === 'next') spectate({ type: 'next' })
  else if (command.type === 'previous') spectate({ type: 'previous' })
  else if (command.type === 'free_camera') spectate({ type: 'free_camera' })
}
const scoreboard = computed(() => Object.values(world.value?.players || {}).sort((a, b) => b.kills - a.kills))
/** Whether a row is leading, or how far behind the leader it is. */
function standingText(player: { kills: number }, index: number) {
  const leader = scoreboard.value[0]?.kills ?? 0
  return behindText({
    player: 0,
    team: 0,
    rank: index + 1,
    behind_leader: leader - player.kills,
    score: { points: 0, kills: 0, deaths: 0, teamkills: 0, suicides: 0, objectives: 0 },
    stats: { shots: 0, hits: 0, headshots: 0, captures: 0, returns: 0, holds: 0 },
  })
}
/** The big scoreboard, ranked and coloured, with the window a long list scrolls through. */
const bigScoreboard = computed(() =>
  scoreboardRows(
    Object.values(world.value?.players || {}).map(player => ({
      id: player.id,
      name: player.name,
      kills: player.kills,
      deaths: player.deaths,
      team: player.team,
      spectating: Boolean(world.value?.spectators?.[String(player.id)]),
    })),
    playerId.value ?? -1,
  ),
)
const scoreboardView = computed(() => scoreboardWindow(bigScoreboard.value, scoreboardOffset.value, SCOREBOARD_WINDOW))
/**
 * IDs are shown to whoever can actually type a command with one. There is no command console yet —
 * that is its own task — so for now this is a switch on the scoreboard rather than a privilege.
 */
const showPlayerIds = ref(false)
const withIds = computed(() => showIds(showPlayerIds.value))
function labelFor(row: { id: number; name: string; own: boolean }) { return rowLabel(row as never, withIds.value) }
/** Where the local player stands, and how far off the lead. */
const myStanding = computed(() => standingFor(bigScoreboard.value, playerId.value ?? -1))
/** The team counters, for however many teams this mode runs. */
const TEAM_MODES = new Set(['Teammatch', 'CaptureTheFlag', 'HoldTheFlag', 'Infiltration'])
const teamRows = computed(() => {
  if (!TEAM_MODES.has(String(world.value?.rules?.kind ?? ''))) return []
  return teamScores(world.value?.scores as number[] | undefined, 2, me.value?.team)
})
/** What each flag is doing, said plainly rather than left to a small icon. */
const flagRows = computed(() => flagIndicators((world.value?.objectives?.flags ?? []) as never[], me.value?.team))
/** What is in your hands, on your back, and in your pocket. */
const weaponCorner = computed(() => {
  const other = carried.value.find(slot => slot !== selectedWeapon.value) ?? null
  return weaponPanel(weaponNames.value, selectedWeapon.value, other, me.value?.grenades ?? 0, (me.value as { cluster_grenades?: number } | undefined)?.cluster_grenades ?? 0)
})
const matchLimitLine = computed(() => limitLine(world.value?.rules?.limits as never))
/** The end-of-round screen, shown while the match is over and the next map is loading. */
const finished = computed(() => {
  const state = world.value?.match_state
  const phase = String((state as { phase?: unknown } | undefined)?.phase ?? '')
  return phase === 'RoundEnd' || phase === 'MapTransition'
})
const endOfRound = computed(() => {
  if (!finished.value) return null
  const outcome = world.value?.match_state?.outcome as { Team?: number; Player?: number } | string | null | undefined
  const names = playerNames.value
  if (outcome && typeof outcome === 'object' && 'Team' in outcome) {
    return endScreen({ outcome: 'team', winner: { name: teamRows.value.find(row => row.team === outcome.Team)?.name ?? 'A team', team: outcome.Team } }, bigScoreboard.value)
  }
  if (outcome && typeof outcome === 'object' && 'Player' in outcome) {
    return endScreen({ outcome: 'winner', winner: { name: names[String(outcome.Player)] ?? 'Somebody' } }, bigScoreboard.value)
  }
  if (outcome === 'Draw') return endScreen({ outcome: 'draw' }, bigScoreboard.value)
  return endScreen({ outcome: 'ended' }, bigScoreboard.value)
})
/** The weapon-statistics screen, ordered by what actually worked. */
const weaponScreenRows = computed(() => weaponScreen(myWeaponRows.value.map(row => ({ weapon: row.weapon, kills: row.kills, shots: row.shots, hits: row.hits }))))
function toggleVideo() {
  showVideo.value = !showVideo.value
  if (showVideo.value) suspendGameplayInput()
  else resumeGameplayInput()
}
function applyQuality(next: QualitySettings) { quality.value = next; game?.setQuality(next) }
function toggleFullscreen() {
  if (document.fullscreenElement) void document.exitFullscreen()
  else void document.documentElement.requestFullscreen()
}
function applyBackground(url: string) {
  if (game) game.backgroundUrl = url.trim()
}
function applyWeather(next: WeatherSettings) { weather.value = next; game?.setWeather(next) }
function applyPreset(next: HudPreset) { hudPreset.value = next; game?.setHudLayout(next) }
function applyHudScale(next: number) { hudScale.value = next; if (game) game.hudScale = next }
function applyCustomHud(layout: HudLayout | null) {
  customHud.value = layout
  if (layout) game?.setHudLayout(layout)
  else game?.setHudLayout(hudPreset.value)
}
const endpoint = (import.meta.env.VITE_WS_URL as string | undefined) || `${location.protocol === 'https:' ? 'wss:' : 'ws:'}//${location.host}/ws`

function send(message: object) { if (socket?.readyState === WebSocket.OPEN) socket.send(JSON.stringify(message)) }
function connect() {
  if (!name.value.trim()) { error.value = 'Enter a guest name'; return }
  localStorage.setItem('arena-name', name.value.trim())
  playerProfiles.update(playerProfiles.active().id, profile => ({ ...profile, name: name.value.trim() }))
  shouldReconnect = true; status.value = 'Connecting'; error.value = ''
  const parsed = parseHostPort(directHost.value)
  const url = parsed ? `${location.protocol === 'https:' ? 'wss:' : 'ws:'}//${parsed.host}:${parsed.port}/ws` : endpoint
  socket = new WebSocket(url)
  socket.onopen = () => send({ type: 'hello', version: 15, name: name.value.trim(), resume: sessionStorage.getItem('arena-token') })
  socket.onmessage = async event => {
    let message: any
    // Counted before it is parsed, because what the link carried is what the link carried.
    if (game && typeof event.data === 'string') game.bandwidth.received(event.data.length, performance.now())
    try { message = JSON.parse(event.data) } catch { return }
    switch (message.type) {
      case 'welcome': playerId.value = message.player; sessionStorage.setItem('arena-token', message.token); status.value = 'Connected'; break
      case 'rooms':
        if (roomsRequestedAt) {
          const ms = performance.now() - roomsRequestedAt
          for (const room of message.rooms) roomPings.record(room.id, ms)
        }
        rooms.value = message.rooms
        break
      case 'map_catalog': maps.value = message.maps; if (!maps.value.some(map => map.name === selectedMap.value)) selectedMap.value = maps.value[0]?.name || 'Aero'; break
      case 'joined': roomId.value = message.room; activeRoom.value = { name: message.name, code: message.code, public: message.public }; await nextTick(); mountGame(); break
      case 'snapshot':
        if (roomId.value === message.room) {
          const listed = rooms.value.find(room => room.id === message.room)
          const accepted = acceptWeaponTable(message.world.weapons, listed?.weapon_hash)
          if (!accepted.ok) {
            error.value = 'This room uses a weapon mod this client does not recognize.'
            leave()
            break
          }
          weaponNames.value = accepted.names
          world.value = message.world
          game?.setSnapshot(message.world)
          inputSystem.ui.setScoreboardExtent(Object.keys(message.world.players).length, SCOREBOARD_WINDOW)
        }
        break
      case 'chat': {
        const tone = message.scope === 'team' ? 'team' : message.scope === 'server' ? 'server' : 'all'
        chat.value.push({ name: message.name, text: message.text, tone })
        chat.value = chat.value.slice(-30)
        chatLog.push(message.scope === 'server' ? message.text : `${message.name}: ${message.text}`, tone, performance.now())
        chatOverlay.value = chatLog.visible(performance.now())
        break
      }
      case 'kicked':
        error.value = disconnectText(message.reason === 'banned' ? 'banned' : 'kicked')
        shouldReconnect = false
        disconnect()
        break
      case 'error': error.value = message.message; if (message.code === 'version_mismatch') { shouldReconnect = false; status.value = 'Update required'; socket?.close() } break
    }
  }
  socket.onclose = () => { status.value = 'Disconnected'; roomId.value = null; world.value = null; game?.destroy(); game = null; clearArenaAgent(); if (shouldReconnect) reconnectTimer = window.setTimeout(connect, 2000) }
  socket.onerror = () => { error.value = 'Connection failed. Is the server running?' }
}
function disconnect() { shouldReconnect = false; clearTimeout(reconnectTimer); clearInterval(feedTimer); socket?.close(); socket = null; game?.destroy(); game = null; clearArenaAgent(); roomId.value = null; world.value = null; status.value = 'Disconnected'; killFeed.clear(); feedLines.value = [] }

function handleInterfaceEvent(event: InterfaceEvent) {
  if (event.type === 'chat') { chatScope.value = event.scope; showInfo.value = true; openChat() }
  if (event.type === 'minimize') { showInfo.value = true; error.value = 'Match sent to the background. Tap the arena to play again.' }
  if (event.type === 'toggle' && event.name === 'console') showControls.value = event.value
  if (event.type === 'volume' && game) game.audio.setMaster(event.volume)
  if (event.type === 'music' && game) {
    game.audio.setMusic(event.enabled ? inputSystem.ui.musicVolume : 0)
  }
  if (event.type === 'pause') send({ type: 'command', line: inputSystem.ui.paused ? '/PAUSE' : '/UNPAUSE' })
}

function mountGame() {
  if (!canvas.value || game) return
  try {
    game = new GameClient(canvas.value, (input: Input) => send({ type: 'input', input }), inputSystem)
    game.localId = playerId.value
    game.onInterfaceEvent = handleInterfaceEvent
    game.onUiChange = ui => {
      selectedWeapon.value = ui.weapon
      scoreboardVisible.value = ui.scoreboardVisible
      scoreboardOffset.value = ui.scoreboardOffset
      showMinimap.value = ui.minimap
      showSniperLine.value = ui.sniperLine
      showPerformance.value = ui.performanceStats
      showWeaponStats.value = ui.weaponStats
    }
    game.onKills = recordKills
    game.onNotice = showNotice
    clearInterval(feedTimer)
    feedTimer = window.setInterval(() => {
      refreshFeed()
      chatOverlay.value = chatLog.visible(performance.now())
      if (game) network.value = networkPanel(game.ping, game.frames, game.bandwidth, performance.now()) as never
    }, 500)
    game.setQuality(quality.value)
    game.setWeather(weather.value)
    game.setHudLayout(hudPreset.value)
    game.hudScale = hudScale.value
    void game.loadWasm()
    attachArenaAgent()
  } catch (e) { error.value = e instanceof Error ? e.message : 'Game cannot start' }
}
function join(id: number) {
  const room = rooms.value.find(candidate => candidate.id === id)
  if (room?.required_mod && room.required_mod !== localStorage.getItem('arena-mod-hash')) {
    error.value = 'This room needs a matching interface pack'
    return
  }
  send({ type: 'join_room', room: id, password: joinPassword.value || undefined, spectator: joinAsSpectator.value, mod_hash: localStorage.getItem('arena-mod-hash') || undefined })
}
function refreshRooms() {
  roomsRequestedAt = performance.now()
  send({ type: 'rooms' })
}
function starRoom(key: string) {
  favorites.value = toggleFavorite(favorites.value, key)
  playerProfiles.update(playerProfiles.active().id, profile => ({ ...profile, favorites: favorites.value }))
}
function createRoom() {
  send({
    type: 'create_room',
    name: newRoom.value.trim(),
    mode: mode.value,
    public: visibility.value === 'public',
    map: selectedMap.value,
    weapon_mod: weaponMod.value || undefined,
    // A community ruleset brings its own base mode and modifiers, so the checkboxes are only sent
    // when no ruleset was picked.
    ruleset: ruleset.value || undefined,
    modifiers: ruleset.value
      ? undefined
      : { realistic: realistic.value, survival: survival.value, advance: advance.value },
    ...createRoomBots(botChoice.value),
    password: roomPassword.value || undefined,
    required_mod: localStorage.getItem('arena-mod-hash') || undefined,
  })
}
function changeBots(delta: number) {
  botCount.value = Math.min(MAX_BOTS, Math.max(0, botCount.value + delta))
  send(setBotsMessage(botChoice.value))
}
function joinByCode() {
  const code = inviteCode.value.trim().toUpperCase()
  if (code) send({ type: 'join_by_code', code, password: joinPassword.value || undefined, spectator: joinAsSpectator.value, mod_hash: localStorage.getItem('arena-mod-hash') || undefined })
}
async function copyInviteCode() { if (!activeRoom.value) return; try { await navigator.clipboard.writeText(activeRoom.value.code) } catch { error.value = `Invite code: ${activeRoom.value.code}` } }
function leave() { send({ type: 'leave_room' }); roomId.value = null; activeRoom.value = null; world.value = null; game?.destroy(); game = null; clearArenaAgent(); send({ type: 'rooms' }) }

async function openChat() {
  await nextTick()
  chatField.value?.focus()
}
function sendChat() {
  const text = chatDraft.value.trim()
  if (!text) { chatField.value?.blur(); return }
  if (text.startsWith('/')) send({ type: 'command', line: text })
  else send({ type: 'chat', text, scope: chatScope.value })
  chatDraft.value = ''
  chatField.value?.blur()
}
// Text fields and overlays own the keyboard while they are focused; gameplay must not read it.
function suspendGameplayInput() { inputSystem.suspend() }
function resumeGameplayInput() { if (!showControls.value) inputSystem.resume() }

function moveTouch(state: { left: boolean; right: boolean; jump: boolean }) { game?.touchMove(state) }
function aimTouch(state: { dx: number; dy: number; fire: boolean }) { game?.touchAim(state) }
function actionTouch(state: { pointerId: number; action: Action; down: boolean }) { game?.touchAction(state.pointerId, state.action, state.down) }
function cancelTouch() { game?.touchCancel() }

function changeWeapon(delta: number) {
  const slots = !dead.value && carried.value.length ? carried.value : weaponNames.value.map((_, index) => index)
  const index = slots.indexOf(selectedWeapon.value)
  selectedWeapon.value = slots[(index + delta + slots.length) % slots.length] ?? slots[0] ?? 0
  inputSystem.weapon = selectedWeapon.value
}
function pickWeapon(index: number) {
  selectedWeapon.value = index
  inputSystem.weapon = index
}
function toggleInfo() {
  showInfo.value = !showInfo.value
  game?.touchCancel()
}
function toggleControls() {
  showControls.value = !showControls.value
  if (showControls.value) suspendGameplayInput()
  else inputSystem.resume()
}
onMounted(() => { inputSystem.ui.setScoreboardExtent(0, SCOREBOARD_WINDOW) })
onMounted(() => window.addEventListener('keydown', spectatorKey))
onBeforeUnmount(() => window.removeEventListener('keydown', spectatorKey))
onBeforeUnmount(() => { clearInterval(feedTimer); disconnect() })
</script>

<template>
  <div class="shell" :class="{ 'in-match': roomId !== null }">
    <header><div class="brand"><span class="brand-mark" aria-hidden="true"></span><div><strong>ARENA</strong><small>2D multiplayer combat</small></div></div><div class="connection"><button class="outline controls-open" @click="toggleControls">Controls</button><button class="outline controls-open" @click="toggleVideo">Video</button><span :class="['dot', status === 'Connected' ? 'online' : '']"></span>{{ status }}</div></header>
    <main v-if="status === 'Disconnected' || status === 'Connecting' || status === 'Update required'" class="landing">
      <div class="hero">
        <div class="hero-rule"><span></span> BROWSER MULTIPLAYER / 01</div>
        <h1>THE ARENA<br><em>IS LIVE.</em></h1>
        <p>Take the high ground. Burn your jetpack. Win the duel.</p>
        <div class="connect-form">
          <label for="guest-name">YOUR CALLSIGN</label>
          <div class="join-row"><input id="guest-name" v-model="name" maxlength="20" placeholder="Enter a guest name" @focus="suspendGameplayInput" @blur="resumeGameplayInput" @keyup.enter="connect"/><button :disabled="status === 'Connecting'" @click="connect">{{ status === 'Connecting' ? 'CONNECTING…' : 'JOIN MATCH →' }}</button></div>
          <input v-model="directHost" placeholder="host:port (optional)" aria-label="Direct host and port" @focus="suspendGameplayInput" @blur="resumeGameplayInput"/>
          <span class="hint">No account. No download. Straight into the fight.</span>
        </div>
      </div>
      <div class="landing-footer"><span>DEATHMATCH / TEAM DEATHMATCH</span><span>DESKTOP + TOUCH</span><span>RUST / WEBGL2</span></div>
    </main>
    <main v-else-if="roomId === null" class="lobby">
      <div class="section-head"><div><p class="eyebrow">MULTIPLAYER</p><h1>Choose an arena</h1></div><button class="outline" @click="refreshRooms">Refresh rooms</button></div>
      <div class="join-code"><input v-model="roomSearch" placeholder="Search rooms" aria-label="Search rooms" @focus="suspendGameplayInput" @blur="resumeGameplayInput"/><input v-model="joinPassword" type="password" placeholder="Room password" aria-label="Room password" @focus="suspendGameplayInput" @blur="resumeGameplayInput"/><label class="switch"><input v-model="hideFullRooms" type="checkbox"/> Hide full</label><label class="switch"><input v-model="hideEmptyRooms" type="checkbox"/> Hide empty</label><label class="switch"><input v-model="joinAsSpectator" type="checkbox"/> Spectate</label></div>
      <div class="join-code">
        <div><h2>Join a friend</h2><p>Enter the six-character invite code.</p></div>
        <form @submit.prevent="joinByCode"><input v-model="inviteCode" maxlength="6" autocomplete="off" autocapitalize="characters" placeholder="ABC123" aria-label="Invite code" @focus="suspendGameplayInput" @blur="resumeGameplayInput"/><button>Join by code</button></form>
      </div>
      <div class="rooms"><div v-for="room in listedRooms" :key="room.id" class="room"><div><strong>{{ room.name }}</strong><span>{{ room.map }} · {{ room.weapon_mod || 'Default mod' }} · {{ modeLabel(room.mode) }} · {{ room.players }}/{{ room.capacity }} players<span v-if="room.ping != null"> · {{ room.ping }} ms</span></span><p v-if="badgesFor(room as never).length || roomFlags(room as never).length" class="room-badges"><span v-for="badge in [...badgesFor(room as never), ...roomFlags(room as never)]" :key="badge">{{ badge }}</span></p></div><button class="outline" @click="starRoom(room.name)">{{ favorites.includes(room.name) ? '★' : '☆' }}</button><button :disabled="room.players >= room.capacity && !joinAsSpectator" @click="join(room.id)">Join</button></div></div>
      <div class="create"><h2>Create a room</h2><p>Private rooms are hidden from the browser. Share their invite code with friends.</p><input v-model="newRoom" maxlength="24" aria-label="Room name" @focus="suspendGameplayInput" @blur="resumeGameplayInput"/><select v-model="mode" :disabled="!!ruleset" aria-label="Game mode"><option value="deathmatch">Deathmatch</option><option value="team">Team deathmatch</option><option value="pointmatch">Pointmatch</option><option value="ctf">Capture the Flag</option><option value="htf">Hold the Flag</option><option value="infiltration">Infiltration</option></select><select v-model="ruleset" aria-label="Community mode"><option value="">Official rules</option><option v-for="community in COMMUNITY_MODES" :key="community" :value="community">{{ community }}</option></select><fieldset class="modifiers" :disabled="!!ruleset"><legend>Modifiers</legend><label><input v-model="realistic" type="checkbox"/> Realistic</label><label><input v-model="survival" type="checkbox"/> Survival</label><label><input v-model="advance" type="checkbox"/> Advance</label></fieldset><select v-model="selectedMap" aria-label="Map selection"><option v-for="map in mapsForMode" :key="map.name" :value="map.name">{{ map.name }}</option></select><div class="map-preview" role="img" :aria-label="`Preview of ${selectedMap}`"><strong>{{ selectedMap }}</strong><span>Original compatible layout</span></div><select v-model="weaponMod" aria-label="Weapon mod"><option value="">Default mod</option><option value="realistic">Realistic mod</option></select><div class="bot-setup"><label>Bots<input v-model.number="botCount" type="number" min="0" :max="MAX_BOTS" aria-label="Number of bots"/></label><select v-model="botDifficulty" aria-label="Bot difficulty"><option v-for="level in BOT_DIFFICULTIES" :key="level" :value="level">{{ level.charAt(0).toUpperCase() + level.slice(1) }}</option></select></div><select v-model="visibility" aria-label="Room visibility"><option value="private">Private · code only</option><option value="public">Public · listed</option></select><input v-model="roomPassword" type="password" placeholder="Optional room password" aria-label="New room password" @focus="suspendGameplayInput" @blur="resumeGameplayInput"/><button @click="createRoom">Create & join</button></div>
    </main>
    <main v-else class="match"><div class="match-head"><div><strong>{{ activeRoom?.name || rooms.find(r => r.id === roomId)?.name || 'Arena' }}</strong><span>{{ activeRoom?.public ? 'Public room' : 'Private room' }} · {{ world?.weapons?.name || 'Default mod' }} · {{ modeLabel(world?.mode ?? 'deathmatch') }} · {{ rulesSummary }}</span></div><button class="invite-chip" :aria-label="`Copy invite code ${activeRoom?.code}`" @click="copyInviteCode"><small>INVITE CODE</small><b>{{ activeRoom?.code }}</b><span>COPY</span></button><div class="match-actions"><button class="outline" @click="toggleInfo">{{ showInfo ? "Close panel" : "Score / chat" }}</button><button class="outline" @click="leave">Leave match</button></div></div><div class="game-wrap"><canvas ref="canvas" aria-label="Game arena"></canvas><div class="hud"><div class="stat"><small>HEALTH</small><strong>{{ me?.hp ?? 100 }}</strong></div><div class="stat"><small>JET FUEL</small><strong>{{ fuelPct }}%</strong></div><div class="stat"><small>AMMO / GRENADES</small><strong>{{ me?.ammo ?? 0 }} / {{ me?.grenades ?? 0 }}{{ me?.reload_timer ? " · RELOADING" : "" }}</strong></div><div class="stat"><small>KILLS / DEATHS</small><strong>{{ me?.kills ?? 0 }} / {{ me?.deaths ?? 0 }}</strong></div><div v-if="(me?.armor ?? 0) > 0" class="stat"><small>ARMOR</small><strong>{{ me?.armor }}</strong></div><div class="stat"><small>TIME</small><strong>{{ matchClock }}</strong></div><div v-if="world?.mode === 'team'" class="stat"><small>ALPHA / BRAVO</small><strong>{{ world?.scores?.[0] ?? 0 }} / {{ world?.scores?.[1] ?? 0 }}</strong></div></div>
      <div v-if="matchBanner" class="match-banner" role="status">{{ matchBanner }}</div>
      <div v-if="flagStatus.length" class="flag-status" role="status" aria-label="Flag status"><p v-for="line in flagStatus" :key="line.kind" :class="[line.kind.toLowerCase(), { missing: line.missing }]">{{ line.text }}</p></div>
      <div v-if="carryingFlag" class="carrying-flag" role="status">You have the flag — take it home</div>
      <div v-if="spectating" class="spectator-bar" role="status"><span>{{ spectatorText }}</span><button class="outline" @click="spectate({ type: 'previous' })">◀ Previous</button><button class="outline" @click="spectate({ type: 'next' })">Next ▶</button><button class="outline" @click="spectate({ type: 'free_camera' })">Free camera</button></div>
      <div v-if="feedLines.length" class="kill-feed" role="log" aria-label="Kill feed"><p v-for="line in feedLines" :key="line.id" :class="line.tone">{{ line.text }}</p></div>
      <div v-if="pickupHint && !dead" class="pickup-hint">Pick up {{ pickupHint }} · F</div>
      <div v-if="kitHint && !dead" class="kit-hint" role="status">{{ kitHint }}</div>
      <div v-if="bonusActive" class="bonus-panel" :class="bonusClass" role="status">{{ bonusLabel }}</div>
      <div v-if="bonusActive" class="bonus-overlay" :class="bonusClass" aria-hidden="true"></div>
      <div v-if="damageAngle !== null && !dead" class="damage-arrow" aria-hidden="true" :style="{ transform: `rotate(${damageAngle}deg)` }"></div>
      <div v-if="dead" class="respawn-panel" role="status">
        <p class="respawn-count">{{ outMessage }}</p>
        <p class="respawn-hint">Choose the weapon you come back with</p>
        <div class="respawn-weapons"><button v-for="(weapon, index) in weaponNames" :key="weapon" :class="{ selected: selectedWeapon === index }" @click="pickWeapon(index)">{{ index + 1 }} · {{ weapon }}</button></div>
      </div>
<div class="hud-top"><div v-if="myStanding" class="standing" role="status"><b>{{ myStanding.text }}</b><small v-if="matchLimitLine">{{ matchLimitLine }}</small></div><div v-if="teamRows.length" class="team-scores"><span v-for="row in teamRows" :key="row.team" :class="{ own: row.own, leading: row.leading }" :style="{ color: `rgb(${row.color[0]*255},${row.color[1]*255},${row.color[2]*255})` }">{{ row.name }} {{ row.score }}</span></div><div v-if="flagRows.length" class="flag-row"><span v-for="row in flagRows" :key="row.flag" :class="row.state">{{ row.text }}</span></div><div v-if="showPerformance" class="network" role="status" aria-label="Connection"><span class="dot" :style="{ width: `${network.ping.radius*2}px`, height: `${network.ping.radius*2}px`, background: `rgb(${network.ping.color[0]*255},${network.ping.color[1]*255},${network.ping.color[2]*255})` }"></span><span>{{ network.ping.text }}</span><span>{{ network.fps }} fps</span><span>↓{{ network.down }} ↑{{ network.up }}</span></div></div>
      <div v-if="chatOverlay.length" class="chat-overlay" role="log" aria-label="Chat"><p v-for="(line, index) in chatOverlay" :key="index" :class="line.tone">{{ line.text }}</p></div>
      <div v-if="showWeaponStats && weaponScreenRows.length" class="weapon-stats-overlay" role="status" aria-label="Weapon statistics"><h2>Weapons</h2><div v-for="row in weaponScreenRows" :key="row.weapon"><span>{{ row.weapon }}</span><b>{{ row.kills }} · {{ row.accuracy }}%</b></div></div>
      <div v-if="scoreboardVisible" class="scoreboard-overlay" role="status" aria-label="Scoreboard"><h2>Scoreboard</h2><div v-for="row in scoreboardView.rows" :key="row.id" class="row" :class="{ own: row.own, spectating: row.spectating }" :style="{ borderColor: `rgb(${row.color[0]*255},${row.color[1]*255},${row.color[2]*255})` }"><span>{{ labelFor(row) }}<small v-if="row.spectating"> · watching</small></span><b>{{ row.kills }} / {{ row.deaths }} · {{ row.ratio }}</b></div><small v-if="scoreboardView.scrollable">Showing {{ scoreboardView.start + 1 }}–{{ scoreboardView.start + scoreboardView.rows.length }} of {{ bigScoreboard.length }} · scroll with the wheel or Page Up/Down</small><button class="outline" @click="showPlayerIds = !showPlayerIds">{{ showPlayerIds ? 'Hide' : 'Show' }} player IDs</button></div>
<div v-if="endOfRound" class="end-screen" role="status" aria-label="Round over"><h2>{{ endOfRound.title }}</h2><p>{{ endOfRound.subtitle }}</p><ol class="podium"><li v-for="row in endOfRound.podium" :key="row.id">{{ row.rank }} · {{ row.name }} · {{ row.points }}</li></ol><section v-if="weaponScreenRows.length" class="weapon-screen"><h3>Your weapons</h3><div v-for="row in weaponScreenRows" :key="row.weapon"><span>{{ row.weapon }}</span><b>{{ row.kills }} kills · {{ row.accuracy }}%</b></div></section><p v-if="endOfRound.nextIn !== null" class="next-map">Next map in {{ endOfRound.nextIn }}</p></div>
      <MobileControls v-if="mobile && !showInfo" @move="moveTouch" @aim="aimTouch" @action="actionTouch" @cancel="cancelTouch" /></div><div v-if="mobile" class="mobile-weapon"><button aria-label="Previous weapon" @click="changeWeapon(-1)">−</button><span>{{ weaponNames[selectedWeapon] }}</span><button aria-label="Next weapon" @click="changeWeapon(1)">+</button></div><div class="weapon-corner" role="status" aria-label="Weapon"><b>{{ weaponCorner.primary }}</b><span v-if="weaponCorner.secondary">on your back: {{ weaponCorner.secondary }}</span><span>{{ weaponCorner.grenadeKind }} ×{{ weaponCorner.grenades }}</span></div><div class="weapons"><button v-for="(weapon, index) in weaponNames" :key="weapon" :class="{ selected: selectedWeapon === index, owned: dead || carried.includes(index) }" @click="pickWeapon(index)">{{ index + 1 }} · {{ weapon }}</button></div><div class="match-bottom" :class="{ open: showInfo }"><section class="scoreboard"><h2>Scoreboard</h2><p v-if="matchLimit" class="limit">{{ matchLimit }}</p><div v-for="(p, index) in scoreboard" :key="p.id"><span>{{ p.name }}{{ p.id === playerId ? ' (you)' : '' }}<small>{{ standingText(p, index) }}</small></span><b>{{ p.kills }} / {{ p.deaths }}</b></div><div class="bot-controls"><span>{{ botSummary }}</span><button class="outline" aria-label="Remove a bot" @click="changeBots(-1)">−</button><button class="outline" aria-label="Add a bot" @click="changeBots(1)">+</button></div><div class="spectate-controls"><button v-if="!spectating" class="outline" @click="joinSpectators">Spectate</button><button v-else class="outline" @click="stopSpectating">Join the match</button></div></section><section v-if="myWeaponRows.length" class="weapon-stats"><h2>Your weapons</h2><p class="limit">{{ myAccuracy }}% accuracy overall</p><div v-for="entry in myWeaponRows" :key="entry.weapon"><span>{{ entry.weapon }}</span><b>{{ entry.kills }} kills · {{ entry.accuracy }}%</b></div></section><section class="chat"><h2>Match chat</h2><div class="chat-log"><p v-for="(line,index) in chat" :key="index"><b>{{ line.name }}:</b> {{ line.text }}</p></div><form @submit.prevent="sendChat"><input ref="chatField" v-model="chatDraft" maxlength="120" :placeholder="chatScope === 'team' ? 'Team chat…' : 'Say something…'" aria-label="Chat message" @focus="suspendGameplayInput" @blur="resumeGameplayInput"/><button>Send</button><button v-if="chatScope === 'team'" type="button" class="outline" @click="chatScope = 'all'">Switch to all</button></form></section></div><p class="controls">Move A/D · Jump Space · Crouch S · Prone X · Roll C · Jet Shift · Reload R · Switch Q · Drop G · Throw T · Knife V · Pickup F · Grenade E or right mouse · Scoreboard Tab · Weapons 1–0 · Rebind everything under Controls</p></main>
    <div v-if="showVideo" class="controls-overlay" @click.self="toggleVideo">
      <VideoSettings
        :preset="hudPreset"
        :hud-scale="hudScale"
        :quality="quality"
        :weather="weather"
        @update:preset="applyPreset"
        @update:hud-scale="applyHudScale"
        @update:quality="applyQuality"
        @update:weather="applyWeather"
        @custom-hud="applyCustomHud"
        @fullscreen="toggleFullscreen"
        @background="applyBackground"
        @close="toggleVideo"
      />
    </div>
    <div v-if="showControls" class="controls-overlay" @click.self="toggleControls">
      <ControlsSettings :system="inputSystem" :store="profileStore" @close="toggleControls" @profile="selectedWeapon = inputSystem.weapon" @scoreboard-mode="mode => (inputSystem.ui.scoreboardMode = mode)" />
    </div>
    <div v-if="error" class="toast" role="alert" @click="error = ''">{{ error }} ×</div>
  </div>
</template>
