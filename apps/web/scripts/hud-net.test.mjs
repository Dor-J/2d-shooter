// Acceptance evidence: web:hud:net
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  BandwidthMeter,
  FrameMeter,
  GRADE_RADII,
  PingMeter,
  formatRate,
  gradeForPing,
  networkPanel,
  pingDot,
} from '../src/hud/net.ts'
import {
  blips,
  boundsOf,
  crosshair,
  projectToMinimap,
  sniperLine,
  withinBounds,
} from '../src/hud/minimap.ts'
import { teamColor } from '../src/hud/gauges.ts'

test('a connection is graded from its round-trip time', () => {
  assert.equal(gradeForPing(20), 'good')
  assert.equal(gradeForPing(90), 'fair')
  assert.equal(gradeForPing(180), 'poor')
  assert.equal(gradeForPing(400), 'bad')
  assert.equal(gradeForPing(Number.NaN), 'bad', 'an unknown ping is not a good one')
})

test('the ping dot grows as well as reddens, for a player who cannot rely on colour', () => {
  assert.ok(GRADE_RADII.bad > GRADE_RADII.poor)
  assert.ok(GRADE_RADII.poor > GRADE_RADII.fair)
  assert.ok(GRADE_RADII.fair > GRADE_RADII.good)
  assert.notDeepEqual(pingDot(20).color, pingDot(400).color)
})

test('the ping dot says its number, and says so honestly when it has none', () => {
  assert.equal(pingDot(87).text, '87 ms')
  assert.equal(pingDot(Number.NaN).text, '— ms')
})

test('ping is averaged over a window rather than taken from the last packet', () => {
  const meter = new PingMeter(4)
  for (const sample of [40, 44, 42, 46]) meter.sample(sample)
  assert.equal(meter.average, 43)
  meter.sample(400)
  assert.equal(meter.samples.length, 4, 'the window slides rather than growing')
  assert.equal(meter.worst, 400)
  assert.ok(meter.jitter > 350, 'a link that spikes is not a steady link, whatever its average')
})

test('a nonsense ping sample is ignored rather than poisoning the average', () => {
  const meter = new PingMeter()
  meter.sample(50)
  meter.sample(-1)
  meter.sample(Number.NaN)
  assert.equal(meter.average, 50)
  meter.clear()
  assert.equal(meter.average, 0)
  assert.equal(meter.jitter, 0)
})

test('frame rate is measured over a rolling second', () => {
  const meter = new FrameMeter(1000)
  for (let i = 0; i <= 60; i += 1) meter.frame(i * (1000 / 60))
  assert.equal(meter.fps, 60)
  assert.ok(meter.worstFrameMs <= 17)
})

test('a single frame is not yet a frame rate', () => {
  const meter = new FrameMeter()
  meter.frame(0)
  assert.equal(meter.fps, 0)
})

test('a stutter shows up as the worst frame even when the average is fine', () => {
  const meter = new FrameMeter(2000)
  meter.frame(0)
  meter.frame(16)
  meter.frame(300)
  assert.equal(meter.worstFrameMs, 284)
})

test('bandwidth counts the bytes actually seen, each way', () => {
  const meter = new BandwidthMeter(1000)
  meter.received(2048, 0)
  meter.sent(512, 0)
  const rate = meter.rate(0)
  assert.equal(rate.down, 2048)
  assert.equal(rate.up, 512)
})

test('bandwidth forgets bytes older than its window', () => {
  const meter = new BandwidthMeter(1000)
  meter.received(4096, 0)
  meter.received(1024, 1500)
  assert.equal(meter.rate(1500).down, 1024)
})

test('rates read the way a player reads them', () => {
  assert.equal(formatRate(0), '0 B/s')
  assert.equal(formatRate(900), '900 B/s')
  assert.equal(formatRate(2048), '2.0 KB/s')
  assert.equal(formatRate(3 * 1024 * 1024), '3.00 MB/s')
})

test('the whole network corner comes out of one call', () => {
  const ping = new PingMeter()
  ping.sample(50)
  const frames = new FrameMeter()
  frames.frame(0)
  frames.frame(1000 / 60)
  const bandwidth = new BandwidthMeter()
  bandwidth.received(1024, 0)
  const panel = networkPanel(ping, frames, bandwidth, 0)
  assert.equal(panel.ping.text, '50 ms')
  assert.equal(panel.fps, 60)
  assert.equal(panel.down, '1.0 KB/s')
  assert.equal(panel.up, '0 B/s')
})

const bounds = { minX: 0, minY: 0, maxX: 400, maxY: 200 }
const panel = { id: 'minimap', x: 100, y: 50, width: 200, height: 200, layer: 0 }

test('map bounds come from the polygons rather than being declared twice', () => {
  const polygons = [
    { vertices: [{ x: -10, y: 5 }, { x: 40, y: 5 }, { x: 40, y: 30 }] },
    { vertices: [{ x: 90, y: -20 }, { x: 120, y: 0 }, { x: 120, y: 60 }] },
  ]
  assert.deepEqual(boundsOf(polygons), { minX: -10, minY: -20, maxX: 120, maxY: 60 })
})

test('a map with no polygons yet does not produce an infinite rectangle', () => {
  assert.deepEqual(boundsOf([]), { minX: 0, minY: 0, maxX: 1, maxY: 1 })
})

test('the minimap letterboxes the map rather than stretching it', () => {
  const topLeft = projectToMinimap({ x: 0, y: 0 }, bounds, panel)
  const bottomRight = projectToMinimap({ x: 400, y: 200 }, bounds, panel)
  assert.equal(bottomRight.x - topLeft.x, 200, 'the wide side fills the panel')
  assert.equal(bottomRight.y - topLeft.y, 100, 'and the other keeps its proportion')
  assert.equal(topLeft.y, 100, 'with the difference split as a margin')
})

test('a point in the middle of the map lands in the middle of the panel', () => {
  const middle = projectToMinimap({ x: 200, y: 100 }, bounds, panel)
  assert.equal(middle.x, panel.x + panel.width / 2)
  assert.equal(middle.y, panel.y + panel.height / 2)
})

test('blips are team-coloured and your own is the bigger one', () => {
  const players = [
    { id: 1, pos: { x: 0, y: 0 }, team: 1, hp: 100 },
    { id: 2, pos: { x: 400, y: 200 }, team: 2, hp: 100 },
  ]
  const [mine, theirs] = blips(players, bounds, panel, 1)
  assert.equal(mine.self, true)
  assert.ok(mine.radius > theirs.radius)
  assert.deepEqual(theirs.color, teamColor(2))
})

test('the minimap never shows somebody the screen is hiding', () => {
  const players = [
    { id: 1, pos: { x: 0, y: 0 }, team: 1, hp: 100 },
    { id: 2, pos: { x: 10, y: 10 }, team: 2, hp: 100, visible: false },
    { id: 3, pos: { x: 20, y: 20 }, team: 2, hp: 0 },
  ]
  assert.deepEqual(
    blips(players, bounds, panel, 1).map(blip => blip.id),
    [1],
  )
})

test('a point outside the map is known to be outside rather than pinned to the edge', () => {
  assert.equal(withinBounds({ x: 10, y: 10 }, bounds), true)
  assert.equal(withinBounds({ x: -5, y: 10 }, bounds), false)
  assert.equal(withinBounds({ x: 10, y: 900 }, bounds), false)
})

test('the sniper line is drawn only for a weapon that has one', () => {
  assert.equal(sniperLine({ x: 0, y: 0 }, { x: 100, y: 0 }, { enabled: false }), null)
  const line = sniperLine({ x: 0, y: 0 }, { x: 100, y: 0 }, { enabled: true, range: 500 })
  assert.deepEqual(line.to, { x: 500, y: 0 })
  assert.equal(line.from.x, 0)
})

test('the sniper line has no direction when the cursor is on the muzzle', () => {
  assert.equal(sniperLine({ x: 5, y: 5 }, { x: 5, y: 5 }, { enabled: true }), null)
})

test('the crosshair opens with the same inaccuracy the server shot with', () => {
  const tight = crosshair(0, 0, false)
  const wide = crosshair(0.05, 0, false)
  assert.ok(wide.spread > tight.spread)
  assert.equal(tight.ready, true)
})

test('being shot at opens the crosshair and lifts it', () => {
  const binked = crosshair(0, 40, false)
  assert.ok(binked.spread > crosshair(0, 0, false).spread)
  assert.ok(binked.offsetY < 0, 'recoil lifts the aim point')
})

test('a weapon still cycling dims its crosshair rather than hiding it', () => {
  const cooling = crosshair(0, 0, true)
  assert.equal(cooling.ready, false)
  assert.ok(cooling.color[3] < crosshair(0, 0, false).color[3])
})
