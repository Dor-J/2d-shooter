const endpoint = process.env.ARENA_WS_URL || 'ws://127.0.0.1:3000/ws'

function connect(name) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(endpoint)
    const timeout = setTimeout(() => reject(new Error(`Timed out connecting ${name}`)), 5000)
    ws.addEventListener('open', () => ws.send(JSON.stringify({ type: 'hello', version: 1, name, resume: null })))
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
  a.ws.send(JSON.stringify({ type: 'join_room', room: 1 }))
  await first
  const second = waitFor(b.ws, 'joined')
  b.ws.send(JSON.stringify({ type: 'join_room', room: 1 }))
  await second
  const snapshot = await waitFor(a.ws, 'snapshot', m => Object.keys(m.world.players).length === 2)
  a.ws.send(JSON.stringify({ type: 'input', input: { seq: 1, left: false, right: true, jump: false, jet: false, fire: false, aim: { x: 800, y: 400 }, weapon: 0 } }))
  const moved = await waitFor(a.ws, 'snapshot', m => m.world.players[a.player].pos.x > snapshot.world.players[a.player].pos.x)
  console.log(`PASS: two guests joined; server advanced to tick ${moved.world.tick} and applied input`)
} finally { a.ws.close(); b.ws.close() }
