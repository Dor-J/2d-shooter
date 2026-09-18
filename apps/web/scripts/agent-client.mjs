// Same-path headless client: hello/join/snapshot/input over the existing protocol.
// No policy. Run `node scripts/agent-client.mjs` against a local server to jet for ~30 ticks.
import { pathToFileURL } from 'node:url'

const DEFAULT_URL = process.env.ARENA_WS_URL || 'ws://127.0.0.1:3000/ws'

export const IDLE_INPUT = {
  left: false,
  right: false,
  jump: false,
  jet: false,
  crouch: false,
  prone: false,
  roll: false,
  reload: false,
  fire: false,
  throw_grenade: false,
  drop: false,
  throw_weapon: false,
  throw_knife: false,
  pickup: false,
  aim: { x: 800, y: 400 },
  weapon: 0,
}

export function mergeInput(overrides = {}, seq = 1) {
  return { seq, ...IDLE_INPUT, ...overrides }
}

function waitFor(ws, type, predicate = () => true, label = type) {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => {
      ws.removeEventListener('message', onMessage)
      reject(new Error(`Timed out waiting for ${label}`))
    }, 5000)
    function onMessage(event) {
      const message = JSON.parse(event.data)
      if (message.type === type && predicate(message)) {
        clearTimeout(timeout)
        ws.removeEventListener('message', onMessage)
        resolve(message)
      }
    }
    ws.addEventListener('message', onMessage)
  })
}

export function connect({ name = 'Agent', url = DEFAULT_URL } = {}) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(url)
    const timeout = setTimeout(() => reject(new Error(`Timed out connecting ${name}`)), 5000)
    let seq = 0
    const snapshotListeners = []
    let opened = false

    ws.addEventListener('open', () => ws.send(JSON.stringify({ type: 'hello', version: 15, name, resume: null })))
    ws.addEventListener('message', event => {
      const message = JSON.parse(event.data)
      if (message.type === 'welcome') {
        opened = true
        clearTimeout(timeout)
        resolve({
          ws,
          player: message.player,
          createRoom(opts = {}) {
            ws.send(JSON.stringify({
              type: 'create_room',
              name: opts.name ?? 'Agent Room',
              mode: opts.mode ?? 'deathmatch',
              public: opts.public ?? false,
              map: opts.map,
              weapon_mod: opts.weapon_mod,
            }))
            return waitFor(ws, 'joined')
          },
          join(room) {
            ws.send(JSON.stringify({ type: 'join_room', room }))
            return waitFor(ws, 'joined', m => m.room === room)
          },
          joinByCode(code) {
            ws.send(JSON.stringify({ type: 'join_by_code', code }))
            return waitFor(ws, 'joined')
          },
          chat(text) {
            ws.send(JSON.stringify({ type: 'chat', text }))
          },
          close() {
            ws.close()
          },
          onSnapshot(cb) {
            snapshotListeners.push(cb)
          },
          input(overrides = {}) {
            seq += 1
            const input = mergeInput(overrides, seq)
            ws.send(JSON.stringify({ type: 'input', input }))
            return input
          },
        })
      }
      if (message.type === 'error' && !opened) {
        clearTimeout(timeout)
        reject(new Error(message.code))
      }
      if (message.type === 'snapshot') {
        for (const cb of snapshotListeners) cb(message)
      }
    })
    ws.addEventListener('error', reject)
  })
}

async function demo() {
  const agent = await connect({ name: 'Jet Demo' })
  let ticks = 0
  const done = new Promise((resolve, reject) => {
    const timeout = setTimeout(() => reject(new Error('Timed out waiting for jet ticks')), 8000)
    agent.onSnapshot(message => {
      const me = message.world.players[agent.player]
      if (!me) return
      ticks += 1
      agent.input({ jet: true })
      if (ticks >= 30) {
        clearTimeout(timeout)
        console.log(`tick ${message.world.tick} pos.y=${me.pos.y} fuel=${me.fuel}`)
        resolve()
      }
    })
  })
  await agent.createRoom({ name: 'Agent Jet', public: false })
  agent.input({ jet: true })
  await done
  agent.close()
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  demo().catch(err => {
    console.error(err)
    process.exit(1)
  })
}
