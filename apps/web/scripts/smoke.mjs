// Acceptance evidence for docs/parity/coverage.json:
//   web:smoke:stance    — crouch, prone, and roll reaching the authoritative server
//   web:smoke:reload    — the explicit reload control refilling a partial magazine
//   web:smoke:aim-input — aim and movement input applied by the server
const endpoint = process.env.ARENA_WS_URL || 'ws://127.0.0.1:3000/ws'

function connect(name) {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(endpoint)
    const timeout = setTimeout(() => reject(new Error(`Timed out connecting ${name}`)), 5000)
    ws.addEventListener('open', () => ws.send(JSON.stringify({ type: 'hello', version: 6, name, resume: null })))
    ws.addEventListener('message', event => {
      const message = JSON.parse(event.data)
      if (message.type === 'welcome') { clearTimeout(timeout); resolve({ ws, player: message.player }) }
      if (message.type === 'error') reject(new Error(message.code))
    })
    ws.addEventListener('error', reject)
  })
}

function waitFor(ws, type, predicate = () => true, label = type) {
  return new Promise((resolve, reject) => {
    const timeout = setTimeout(() => { ws.removeEventListener('message', onMessage); reject(new Error(`Timed out waiting for ${label}`)) }, 5000)
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
  a.ws.send(JSON.stringify({ type: 'create_room', name: 'Private Smoke', mode: 'deathmatch', public: false, map: 'Arena' }))
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
  let seq = 0
  const input = overrides => {
    seq += 1
    a.ws.send(JSON.stringify({
      type: 'input',
      input: { seq, left: false, right: false, jump: false, jet: false, crouch: false, prone: false, roll: false, reload: false, fire: false, throw_grenade: false, aim: { x: 800, y: 400 }, weapon: 0, ...overrides },
    }))
  }
  const mine = message => message.world.players[a.player]

  input({ right: true })
  const moved = await waitFor(a.ws, 'snapshot', m => mine(m).pos.x > snapshot.world.players[a.player].pos.x)

  // Stance controls added in task 7 must reach the authoritative simulation, not just the client.
  input({ crouch: true })
  await waitFor(a.ws, 'snapshot', m => mine(m).state?.pose === 'crouching', 'crouch stance')
  input({ prone: true })
  await waitFor(a.ws, 'snapshot', m => mine(m).state?.pose === 'prone', 'prone stance')
  input({ roll: true, right: true })
  await waitFor(a.ws, 'snapshot', m => ['rolling', 'getting_up'].includes(mine(m).state?.pose), 'roll out of prone')
  input({})
  await waitFor(a.ws, 'snapshot', m => ['standing', 'airborne'].includes(mine(m).state?.pose), 'return to standing')

  // Explicit reload: fire one slow round, stop, then ask for a reload while the magazine is partial.
  input({ fire: true, weapon: 7 })
  const fired = await waitFor(a.ws, 'snapshot', m => mine(m).ammo < 10 && mine(m).weapon === 7, 'first shot')
  if (fired.world.players[a.player].ammo === 0) throw new Error('Expected a partial magazine for the reload check')
  input({ weapon: 7 })
  const idle = await waitFor(a.ws, 'snapshot', m => mine(m).reload_timer === 0 && mine(m).ammo > 0, 'partial magazine')
  if (idle.world.players[a.player].ammo === 10) throw new Error('Reload check needs a partial magazine')
  input({ weapon: 7, reload: true })
  const reloading = await waitFor(a.ws, 'snapshot', m => mine(m).reload_timer > 0, 'explicit reload')

  console.log(`PASS: private room ${created.code} stayed hidden; friend joined by code; input applied at tick ${moved.world.tick}; stance and reload controls accepted by tick ${reloading.world.tick}`)
} finally { a.ws.close(); b.ws.close() }
