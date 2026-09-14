<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref } from 'vue'
import { GameClient, type Input, type Room, type World } from './game'

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
const canvas = ref<HTMLCanvasElement | null>(null)
const mobile = /Android|iPhone|iPad|iPod/i.test(navigator.userAgent)
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
  socket.onopen = () => send({ type: 'hello', version: 1, name: name.value.trim(), resume: sessionStorage.getItem('arena-token') })
  socket.onmessage = async event => {
    let message: any
    try { message = JSON.parse(event.data) } catch { return }
    switch (message.type) {
      case 'welcome': playerId.value = message.player; sessionStorage.setItem('arena-token', message.token); status.value = 'Connected'; break
      case 'rooms': rooms.value = message.rooms; break
      case 'joined': roomId.value = message.room; await nextTick(); mountGame(); break
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
function createRoom() { send({ type: 'create_room', name: newRoom.value.trim(), mode: mode.value }); send({ type: 'rooms' }) }
function leave() { send({ type: 'leave_room' }); roomId.value = null; world.value = null; game?.destroy(); game = null; send({ type: 'rooms' }) }
function sendChat() { if (chatDraft.value.trim()) send({ type: 'chat', text: chatDraft.value.trim() }); chatDraft.value = '' }
function touch(key: 'left'|'right'|'jump'|'jet'|'fire', state: boolean) { if (game) game.touch[key] = state }
onBeforeUnmount(disconnect)
</script>

<template>
  <div class="shell">
    <header><div class="brand"><span class="brand-mark">✦</span><div><strong>ARENA</strong><small>2D multiplayer combat</small></div></div><div class="connection"><span :class="['dot', status === 'Connected' ? 'online' : '']"></span>{{ status }}</div></header>
    <main v-if="status === 'Disconnected' || status === 'Connecting' || status === 'Update required'" class="landing">
      <div class="hero"><p class="eyebrow">FAST. PRECISE. CHAOTIC.</p><h1>Enter the arena.</h1><p>Jet across platforms, line up your shot, and fight for the top of the board.</p><div class="connect-form"><input v-model="name" maxlength="20" placeholder="Your guest name" aria-label="Guest name" @keyup.enter="connect"/><button :disabled="status === 'Connecting'" @click="connect">{{ status === 'Connecting' ? 'Connecting…' : 'Play now' }}</button></div><p class="hint">Guest play · No account required · Desktop and touch controls</p></div>
      <div class="preview" aria-hidden="true"><div class="sun"></div><div class="preview-platform one"></div><div class="preview-platform two"></div><div class="preview-player a"></div><div class="preview-player b"></div><div class="preview-shot"></div></div>
    </main>
    <main v-else-if="roomId === null" class="lobby"><div class="section-head"><div><p class="eyebrow">MULTIPLAYER</p><h1>Choose an arena</h1></div><button class="outline" @click="send({type:'rooms'})">Refresh rooms</button></div><div class="rooms"><div v-for="room in rooms" :key="room.id" class="room"><div><strong>{{ room.name }}</strong><span>{{ room.mode === 'team' ? 'Team deathmatch' : 'Deathmatch' }} · {{ room.players }}/{{ room.capacity }} players</span></div><button :disabled="room.players >= room.capacity" @click="join(room.id)">Join</button></div></div><div class="create"><h2>Create a room</h2><input v-model="newRoom" maxlength="24" aria-label="Room name"/><select v-model="mode" aria-label="Game mode"><option value="deathmatch">Deathmatch</option><option value="team">Team deathmatch</option></select><button @click="createRoom">Create & join</button></div></main>
    <main v-else class="match"><div class="match-head"><div><strong>{{ rooms.find(r => r.id === roomId)?.name || 'Arena' }}</strong><span>Room #{{ roomId }} · {{ world?.mode === 'team' ? 'Team deathmatch' : 'Deathmatch' }}</span></div><button class="outline" @click="leave">Leave match</button></div><div class="game-wrap"><canvas ref="canvas" aria-label="Game arena"></canvas><div class="hud"><div class="stat"><small>HEALTH</small><strong>{{ me?.hp ?? 100 }}</strong></div><div class="stat"><small>JET FUEL</small><strong>{{ Math.round((me?.fuel ?? 1) * 100) }}%</strong></div><div class="stat"><small>KILLS / DEATHS</small><strong>{{ me?.kills ?? 0 }} / {{ me?.deaths ?? 0 }}</strong></div></div><div v-if="mobile" class="touch-layer"><div class="touch-move"><button @pointerdown.prevent="touch('left',true)" @pointerup="touch('left',false)" @pointercancel="touch('left',false)">◀</button><button @pointerdown.prevent="touch('right',true)" @pointerup="touch('right',false)" @pointercancel="touch('right',false)">▶</button></div><div class="touch-actions"><button @pointerdown.prevent="touch('jump',true)" @pointerup="touch('jump',false)" @pointercancel="touch('jump',false)">JUMP</button><button @pointerdown.prevent="touch('jet',true)" @pointerup="touch('jet',false)" @pointercancel="touch('jet',false)">JET</button><button class="fire" @pointerdown.prevent="touch('fire',true)" @pointerup="touch('fire',false)" @pointercancel="touch('fire',false)">FIRE</button></div></div></div><div v-if="mobile" class="weapons"><button v-for="w in 3" :key="w" @click="game && (game.weapon = w - 1)">Weapon {{ w }}</button></div><div class="match-bottom"><section class="scoreboard"><h2>Scoreboard</h2><div v-for="p in scoreboard" :key="p.id"><span>{{ p.name }}{{ p.id === playerId ? ' (you)' : '' }}</span><b>{{ p.kills }} / {{ p.deaths }}</b></div></section><section class="chat"><h2>Match chat</h2><div class="chat-log"><p v-for="(line,index) in chat" :key="index"><b>{{ line.name }}:</b> {{ line.text }}</p></div><form @submit.prevent="sendChat"><input v-model="chatDraft" maxlength="120" placeholder="Say something…" aria-label="Chat message"/><button>Send</button></form></section></div><p class="controls">Move A/D · Jump Space · Jet Shift · Aim and fire with mouse · Weapons 1–3</p></main>
    <div v-if="error" class="toast" role="alert" @click="error = ''">{{ error }} ×</div>
  </div>
</template>
