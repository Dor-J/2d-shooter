<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref } from 'vue'
import { GameClient, weaponNames, type Input, type Room, type World } from './game'
import MobileControls from './MobileControls.vue'

const name = ref(localStorage.getItem('arena-name') || '')
const status = ref('Disconnected')
const error = ref('')
const rooms = ref<Room[]>([])
const roomId = ref<number | null>(null)
const playerId = ref(0)
const world = ref<World | null>(null)
const chat = ref<{ name: string; text: string }[]>([])
const chatDraft = ref('')
const newRoom = ref('New Arena')
const mode = ref('deathmatch')
const maps = ref<{ name: string; mode: string; preview: string }[]>([])
const selectedMap = ref('Aero')
const visibility = ref<'private' | 'public'>('private')
const inviteCode = ref('')
const activeRoom = ref<{ name: string; code: string; public: boolean } | null>(null)
const canvas = ref<HTMLCanvasElement | null>(null)
const mobile = navigator.maxTouchPoints > 0 || matchMedia('(pointer: coarse)').matches
const showInfo = ref(false)
const selectedWeapon = ref(0)
let socket: WebSocket | null = null
let game: GameClient | null = null
let reconnectTimer = 0
let shouldReconnect = false
const me = computed(() => world.value?.players[playerId.value])
const scoreboard = computed(() => Object.values(world.value?.players || {}).sort((a,b) => b.kills - a.kills))
const endpoint = (import.meta.env.VITE_WS_URL as string | undefined) || `${location.protocol === 'https:' ? 'wss:' : 'ws:'}//${location.host}/ws`

function send(message: object) { if (socket?.readyState === WebSocket.OPEN) socket.send(JSON.stringify(message)) }
function connect() {
  if (!name.value.trim()) { error.value = 'Enter a guest name'; return }
  localStorage.setItem('arena-name', name.value.trim())
  shouldReconnect = true; status.value = 'Connecting'; error.value = ''
  socket = new WebSocket(endpoint)
  socket.onopen = () => send({ type: 'hello', version: 5, name: name.value.trim(), resume: sessionStorage.getItem('arena-token') })
  socket.onmessage = async event => {
    let message: any
    try { message = JSON.parse(event.data) } catch { return }
    switch (message.type) {
      case 'welcome': playerId.value = message.player; sessionStorage.setItem('arena-token', message.token); status.value = 'Connected'; break
      case 'rooms': rooms.value = message.rooms; break
      case 'map_catalog': maps.value = message.maps; if (!maps.value.some(map => map.name === selectedMap.value)) selectedMap.value = maps.value[0]?.name || 'Aero'; break
      case 'joined': roomId.value = message.room; activeRoom.value = { name: message.name, code: message.code, public: message.public }; await nextTick(); mountGame(); break
      case 'snapshot': if (roomId.value === message.room) { world.value = message.world; game?.setSnapshot(message.world) } break
      case 'chat': chat.value.push({ name: message.name, text: message.text }); chat.value = chat.value.slice(-30); break
      case 'error': error.value = message.message; if (message.code === 'version_mismatch') { shouldReconnect = false; status.value = 'Update required'; socket?.close() } break
    }
  }
  socket.onclose = () => { status.value = 'Disconnected'; roomId.value = null; world.value = null; game?.destroy(); game = null; if (shouldReconnect) reconnectTimer = window.setTimeout(connect, 2000) }
  socket.onerror = () => { error.value = 'Connection failed. Is the server running?' }
}
function disconnect() { shouldReconnect = false; clearTimeout(reconnectTimer); socket?.close(); socket = null; game?.destroy(); game = null; roomId.value = null; world.value = null; status.value = 'Disconnected' }
function mountGame() { if (!canvas.value || game) return; try { game = new GameClient(canvas.value, (input: Input) => send({ type: 'input', input })); game.localId = playerId.value; void game.loadWasm() } catch (e) { error.value = e instanceof Error ? e.message : 'Game cannot start' } }
function join(id: number) { send({ type: 'join_room', room: id }) }
function createRoom() { send({ type: 'create_room', name: newRoom.value.trim(), mode: mode.value, public: visibility.value === 'public', map: selectedMap.value }) }
function joinByCode() { const code = inviteCode.value.trim().toUpperCase(); if (code) send({ type: 'join_by_code', code }) }
async function copyInviteCode() { if (!activeRoom.value) return; try { await navigator.clipboard.writeText(activeRoom.value.code) } catch { error.value = `Invite code: ${activeRoom.value.code}` } }
function leave() { send({ type: 'leave_room' }); roomId.value = null; activeRoom.value = null; world.value = null; game?.destroy(); game = null; send({ type: 'rooms' }) }
function sendChat() { if (chatDraft.value.trim()) send({ type: 'chat', text: chatDraft.value.trim() }); chatDraft.value = '' }
function moveTouch(state: { left: boolean; right: boolean; jump: boolean }) { if (game) Object.assign(game.touch, state) }
function aimTouch(state: { dx: number; dy: number; fire: boolean }) {
  if (!game) return
  game.touch.fire = state.fire
  if (!state.fire) return
  const player = game.predicted ?? world.value?.players[playerId.value]
  if (player) game.aim = { x: player.pos.x + state.dx * 400, y: player.pos.y + state.dy * 400 }
}
function jetTouch(active: boolean) { if (game) game.touch.jet = active }
function grenadeTouch(active: boolean) { if (game) game.touch.grenade = active }
function changeWeapon(delta: number) {
  selectedWeapon.value = (selectedWeapon.value + delta + weaponNames.length) % weaponNames.length
  if (game) game.weapon = selectedWeapon.value
}
function toggleInfo() {
  showInfo.value = !showInfo.value
  if (game) game.touch = { left: false, right: false, jump: false, jet: false, fire: false, grenade: false }
}
onBeforeUnmount(disconnect)
</script>

<template>
  <div class="shell" :class="{ 'in-match': roomId !== null && mobile }">
    <header><div class="brand"><span class="brand-mark" aria-hidden="true"></span><div><strong>ARENA</strong><small>2D multiplayer combat</small></div></div><div class="connection"><span :class="['dot', status === 'Connected' ? 'online' : '']"></span>{{ status }}</div></header>
    <main v-if="status === 'Disconnected' || status === 'Connecting' || status === 'Update required'" class="landing">
      <div class="hero">
        <div class="hero-rule"><span></span> BROWSER MULTIPLAYER / 01</div>
        <h1>THE ARENA<br><em>IS LIVE.</em></h1>
        <p>Take the high ground. Burn your jetpack. Win the duel.</p>
        <div class="connect-form">
          <label for="guest-name">YOUR CALLSIGN</label>
          <div class="join-row"><input id="guest-name" v-model="name" maxlength="20" placeholder="Enter a guest name" @keyup.enter="connect"/><button :disabled="status === 'Connecting'" @click="connect">{{ status === 'Connecting' ? 'CONNECTING…' : 'JOIN MATCH →' }}</button></div>
          <span class="hint">No account. No download. Straight into the fight.</span>
        </div>
      </div>
      <div class="landing-footer"><span>DEATHMATCH / TEAM DEATHMATCH</span><span>DESKTOP + TOUCH</span><span>RUST / WEBGL2</span></div>
    </main>
    <main v-else-if="roomId === null" class="lobby">
      <div class="section-head"><div><p class="eyebrow">MULTIPLAYER</p><h1>Choose an arena</h1></div><button class="outline" @click="send({type:'rooms'})">Refresh rooms</button></div>
      <div class="join-code">
        <div><h2>Join a friend</h2><p>Enter the six-character invite code.</p></div>
        <form @submit.prevent="joinByCode"><input v-model="inviteCode" maxlength="6" autocomplete="off" autocapitalize="characters" placeholder="ABC123" aria-label="Invite code"/><button>Join by code</button></form>
      </div>
      <div class="rooms"><div v-for="room in rooms" :key="room.id" class="room"><div><strong>{{ room.name }}</strong><span>{{ room.map }} · {{ room.mode === 'team' ? 'Team deathmatch' : 'Deathmatch' }} · {{ room.players }}/{{ room.capacity }} players</span></div><button :disabled="room.players >= room.capacity" @click="join(room.id)">Join</button></div></div>
      <div class="create"><h2>Create a room</h2><p>Private rooms are hidden from the browser. Share their invite code with friends.</p><input v-model="newRoom" maxlength="24" aria-label="Room name"/><select v-model="mode" aria-label="Game mode"><option value="deathmatch">Deathmatch</option><option value="team">Team deathmatch</option></select><select v-model="selectedMap" aria-label="Map selection"><option v-for="map in maps.filter(map => map.mode === 'deathmatch')" :key="map.name" :value="map.name">{{ map.name }}</option></select><div class="map-preview" role="img" :aria-label="`Preview of ${selectedMap}`"><strong>{{ selectedMap }}</strong><span>Original compatible layout</span></div><select v-model="visibility" aria-label="Room visibility"><option value="private">Private · code only</option><option value="public">Public · listed</option></select><button @click="createRoom">Create & join</button></div>
    </main>
    <main v-else class="match"><div class="match-head"><div><strong>{{ activeRoom?.name || rooms.find(r => r.id === roomId)?.name || 'Arena' }}</strong><span>{{ activeRoom?.public ? 'Public room' : 'Private room' }} · {{ world?.mode === 'team' ? 'Team deathmatch' : 'Deathmatch' }}</span></div><button class="invite-chip" :aria-label="`Copy invite code ${activeRoom?.code}`" @click="copyInviteCode"><small>INVITE CODE</small><b>{{ activeRoom?.code }}</b><span>COPY</span></button><div class="match-actions"><button v-if="mobile" class="outline" @click="toggleInfo">{{ showInfo ? "Close panel" : "Score / chat" }}</button><button class="outline" @click="leave">Leave match</button></div></div><div class="game-wrap"><canvas ref="canvas" aria-label="Game arena"></canvas><div class="hud"><div class="stat"><small>HEALTH</small><strong>{{ me?.hp ?? 100 }}</strong></div><div class="stat"><small>JET FUEL</small><strong>{{ Math.round((me?.fuel ?? 1) * 100) }}%</strong></div><div class="stat"><small>AMMO / GRENADES</small><strong>{{ me?.ammo ?? 0 }} / {{ me?.grenades ?? 0 }}{{ me?.reload_timer ? " · RELOADING" : "" }}</strong></div><div class="stat"><small>KILLS / DEATHS</small><strong>{{ me?.kills ?? 0 }} / {{ me?.deaths ?? 0 }}</strong></div></div><MobileControls v-if="mobile && !showInfo" @move="moveTouch" @aim="aimTouch" @jet="jetTouch" @grenade="grenadeTouch" /></div><div v-if="mobile" class="mobile-weapon"><button aria-label="Previous weapon" @click="changeWeapon(-1)">−</button><span>{{ weaponNames[selectedWeapon] }}</span><button aria-label="Next weapon" @click="changeWeapon(1)">+</button></div><div class="weapons"><button v-for="(weapon, index) in weaponNames" :key="weapon" :class="{ selected: selectedWeapon === index }" @click="game && (game.weapon = index); selectedWeapon = index">{{ index + 1 }} · {{ weapon }}</button></div><div class="match-bottom" :class="{ open: showInfo }"><section class="scoreboard"><h2>Scoreboard</h2><div v-for="p in scoreboard" :key="p.id"><span>{{ p.name }}{{ p.id === playerId ? ' (you)' : '' }}</span><b>{{ p.kills }} / {{ p.deaths }}</b></div></section><section class="chat"><h2>Match chat</h2><div class="chat-log"><p v-for="(line,index) in chat" :key="index"><b>{{ line.name }}:</b> {{ line.text }}</p></div><form @submit.prevent="sendChat"><input v-model="chatDraft" maxlength="120" placeholder="Say something…" aria-label="Chat message"/><button>Send</button></form></section></div><p class="controls">Move A/D · Jump Space · Jet Shift · Grenade E · Aim and fire with mouse · Weapons 1–0 or picker</p></main>
    <div v-if="error" class="toast" role="alert" @click="error = ''">{{ error }} ×</div>
  </div>
</template>
