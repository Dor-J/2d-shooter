const endpoint = process.env.ARENA_WS_URL || 'ws://127.0.0.1:3000/ws'

function connect(name) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(endpoint)
    const timeout = setTimeout(() => reject(new Error(`Timed out connecting ${name}`)), 5000)
    ws.addEventListener('open', () => ws.send(JSON.stringify({ type: 'hello', version: 3, name, resume: null })))
    ws.addEventListener('message', event => {
      const message = JSON.parse(event.data)
      if (message.type === 'welcome') { clearTimeout(timeout); resolve({ ws, player: message.player }) }
      if (message.type === 'error') reject(new Error(message.code))
    })
    ws.addEventListener('error', reject)
  })
}

function waitFor(ws, type, predicate = () => true) {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => { ws.removeEventListener('message', onMessage); reject(new Error(`Timed out waiting for ${type}`)) }, 5000)
    function onMessage(event) {
      const message = JSON.parse(event.data)
      if (message.type === type && predicate(message)) { clearTimeout(timeout); ws.removeEventListener('message', onMessage); resolve(message) }
    }
    ws.addEventListener('message', onMessage)
  })
}

const a = await connect('Smoke A')
const b = await connect('Smoke B')
try {
  const first = waitFor(a.ws, 'joined')
  a.ws.send(JSON.stringify({ type: 'create_room', name: 'Private Smoke', mode: 'deathmatch', public: false }))
  const created = await first
  const roomsReady = waitFor(b.ws, 'rooms')
  b.ws.send(JSON.stringify({ type: 'rooms' }))
  const roomList = await roomsReady
  if (roomList.rooms.some(room => room.id === created.room)) throw new Error('Private room leaked into public list')
  const snapshotReady = waitFor(a.ws, 'snapshot', m => Object.keys(m.world.players).length === 2)
  const second = waitFor(b.ws, 'joined')
  b.ws.send(JSON.stringify({ type: 'join_by_code', code: created.code.toLowerCase() }))
  const joined = await second
  if (joined.room !== created.room) throw new Error('Invite code joined the wrong room')
  const snapshot = await snapshotReady
  a.ws.send(JSON.stringify({ type: 'input', input: { seq: 1, left: false, right: true, jump: false, jet: false, fire: false, throw_grenade: false, aim: { x: 800, y: 400 }, weapon: 0 } }))
  const moved = await waitFor(a.ws, 'snapshot', m => m.world.players[a.player].pos.x > snapshot.world.players[a.player].pos.x)
  console.log(`PASS: private room ${created.code} stayed hidden; friend joined by code; input applied at tick ${moved.world.tick}`)
} finally { a.ws.close(); b.ws.close() }
