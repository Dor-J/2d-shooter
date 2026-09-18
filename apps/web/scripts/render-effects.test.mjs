import test from 'node:test'
import assert from 'node:assert/strict'
import {
  WEATHER_KINDS,
  defaultWeather,
  dropCount,
  makeWeather,
  stepWeather,
  windAngle,
  windForce,
} from '../src/render/weather.ts'
import {
  BULLET_TIME_SCALE,
  PREDATOR_ALPHA,
  ScreenShake,
  blastShake,
  bonusOverlay,
  bulletTimeScale,
  bulletTimeWash,
  damageFeedback,
  explosionFlash,
} from '../src/render/effects.ts'
import {
  MIN_RESOLUTION_SCALE,
  adaptQuality,
  clampResolutionScale,
  defaultQuality,
  drawingBufferSize,
  lowQuality,
  needsCompatibility,
  particleBudget,
  particleLimit,
  textureParameters,
} from '../src/render/quality.ts'
import {
  INTERFACE_SLOTS,
  applyMod,
  assetPath,
  defaultManifest,
  drawSize,
  importedAssets,
  readInterfaceSkin,
  slotSource,
} from '../src/render/manifest.ts'

const view = { width: 800, height: 600 }

test('clear weather draws nothing at all', () => {
  assert.equal(dropCount({ kind: 'none', intensity: 1, wind: 0 }), 0)
  assert.deepEqual(makeWeather({ kind: 'none', intensity: 1, wind: 0 }, view), [])
  assert.deepEqual(WEATHER_KINDS, ['none', 'rain', 'snow'])
  assert.equal(defaultWeather().kind, 'none')
})

test('rain and snow fill the view, and intensity thins them out', () => {
  const heavy = makeWeather({ kind: 'rain', intensity: 1, wind: 0 }, view)
  const light = makeWeather({ kind: 'rain', intensity: 0.25, wind: 0 }, view)
  assert.ok(heavy.length > light.length)
  for (const drop of heavy) {
    assert.ok(drop.x >= 0 && drop.x <= view.width)
    assert.ok(drop.y >= 0 && drop.y <= view.height)
  }
})

test('rain falls faster than snow', () => {
  const rain = makeWeather({ kind: 'rain', intensity: 1, wind: 0 }, view, 7)
  const snow = makeWeather({ kind: 'snow', intensity: 1, wind: 0 }, view, 7)
  const average = drops => drops.reduce((sum, drop) => sum + drop.vy, 0) / drops.length
  assert.ok(average(rain) > average(snow) * 5)
})

test('the same seed makes the same weather, so a screenshot can be compared', () => {
  const settings = { kind: 'snow', intensity: 0.8, wind: 20 }
  assert.deepEqual(makeWeather(settings, view, 42), makeWeather(settings, view, 42))
  assert.notDeepEqual(makeWeather(settings, view, 42), makeWeather(settings, view, 43))
})

test('weather wraps back into the view rather than running out', () => {
  const settings = { kind: 'rain', intensity: 1, wind: 0 }
  const drops = makeWeather(settings, view, 3)
  for (let i = 0; i < 120; i += 1) stepWeather(drops, 1 / 60, view, settings, i / 60)
  for (const drop of drops) {
    assert.ok(drop.x >= 0 && drop.x <= view.width, `x stayed in view: ${drop.x}`)
    assert.ok(drop.y >= 0 && drop.y <= view.height, `y stayed in view: ${drop.y}`)
  }
})

test('wind blows the weather sideways and snow drifts on its own', () => {
  const blown = makeWeather({ kind: 'rain', intensity: 1, wind: 200 }, view, 5)
  assert.ok(blown.every(drop => drop.vx > 0))
  assert.ok(windAngle({ kind: 'rain', intensity: 1, wind: 200 }) > 0)
  assert.equal(windAngle({ kind: 'rain', intensity: 1, wind: 0 }), 0)
  assert.ok(
    Math.abs(windAngle({ kind: 'snow', intensity: 1, wind: 60 })) >
      Math.abs(windAngle({ kind: 'rain', intensity: 1, wind: 60 })),
    'the same wind leans slow snow further than fast rain',
  )
})

test('wind pushes loose particles too, but only when there is weather', () => {
  assert.equal(windForce({ kind: 'rain', intensity: 1, wind: 90 }), 90)
  assert.equal(windForce({ kind: 'none', intensity: 1, wind: 90 }), 0)
})

test('a step of no time at all changes nothing', () => {
  const settings = { kind: 'rain', intensity: 1, wind: 0 }
  const drops = makeWeather(settings, view, 1)
  const before = structuredClone(drops)
  stepWeather(drops, 0, view, settings)
  stepWeather(drops, Number.NaN, view, settings)
  assert.deepEqual(drops, before)
})

test('bullet time slows the picture and never the simulation', () => {
  assert.equal(bulletTimeScale(true), BULLET_TIME_SCALE)
  assert.equal(bulletTimeScale(false), 1)
  assert.ok(BULLET_TIME_SCALE < 1 && BULLET_TIME_SCALE > 0)
})

test('the bullet-time wash comes in over a moment rather than snapping on', () => {
  assert.equal(bulletTimeWash(false, 1), 0)
  assert.equal(bulletTimeWash(true, 0), 0)
  assert.equal(bulletTimeWash(true, 0.25), 1)
  assert.equal(bulletTimeWash(true, 10), 1)
})

test('predator makes a player faint but never invisible', () => {
  const enemy = bonusOverlay('Predator', false)
  assert.equal(enemy.playerAlpha, PREDATOR_ALPHA)
  assert.ok(enemy.playerAlpha > 0, 'there is always something to shoot at')
  assert.ok(bonusOverlay('Predator', true).playerAlpha > enemy.playerAlpha)
})

test('berserker and flame god tint your own screen and nobody else’s', () => {
  for (const effect of ['Berserker', 'FlameGod']) {
    assert.ok(bonusOverlay(effect, true).tint[3] > 0, `${effect} tints your screen`)
    assert.equal(bonusOverlay(effect, false).tint[3], 0, `${effect} does not tint theirs`)
  }
  assert.equal(bonusOverlay('Berserker', true).name, 'Berserker')
  assert.equal(bonusOverlay('FlameGod', true).name, 'Flame God')
})

test('no bonus is no overlay at all', () => {
  const none = bonusOverlay(null, true)
  assert.equal(none.playerAlpha, 1)
  assert.equal(none.tint[3], 0)
  assert.equal(bonusOverlay(undefined, true).name, '')
})

test('the strongest knock wins rather than the sum, so a cluster does not throw the screen away', () => {
  const shake = new ScreenShake()
  for (let i = 0; i < 20; i += 1) shake.add(10, i)
  assert.ok(shake.magnitude <= 24)
  shake.add(20, 1)
  assert.equal(shake.magnitude, 20)
})

test('a shake decays on its own, so nothing has to remember to stop it', () => {
  const shake = new ScreenShake()
  shake.add(12)
  assert.notDeepEqual(shake.offset(), { x: 0, y: 0 })
  for (let i = 0; i < 60; i += 1) shake.step(1 / 60)
  assert.equal(shake.magnitude, 0)
  assert.deepEqual(shake.offset(), { x: 0, y: 0 })
})

test('a nonsense knock is ignored rather than freezing the camera', () => {
  const shake = new ScreenShake()
  shake.add(Number.NaN)
  shake.add(-5)
  assert.equal(shake.magnitude, 0)
  shake.step(0)
  assert.equal(shake.magnitude, 0)
})

test('an explosion shakes hardest at its centre and not at all outside its radius', () => {
  assert.ok(blastShake(0, 100) > blastShake(50, 100))
  assert.equal(blastShake(100, 100), 0)
  assert.equal(blastShake(200, 100), 0)
  assert.equal(blastShake(10, 0), 0)
})

test('damage feedback points at whoever shot you and fades', () => {
  const hit = damageFeedback(0.7, { x: -1, y: 0 }, 0)
  assert.ok(Math.abs(hit.arrow - Math.PI) < 1e-9)
  assert.equal(hit.alpha, 1)
  assert.equal(damageFeedback(0.7, { x: -1, y: 0 }, 2).arrow, null, 'and stops pointing once it fades')
})

test('damage from nowhere in particular has no arrow', () => {
  assert.equal(damageFeedback(0.5, null, 0).arrow, null)
  assert.equal(damageFeedback(0.5, { x: 0, y: 0 }, 0).arrow, null)
  assert.equal(damageFeedback(5, null, 0).vignette, 1, 'and the vignette stays in range')
})

test('an explosion flash fades over a few frames', () => {
  assert.equal(explosionFlash(0), 1)
  assert.ok(explosionFlash(0.09) > 0 && explosionFlash(0.09) < 1)
  assert.equal(explosionFlash(0.18), 0)
  assert.equal(explosionFlash(-1), 0)
})

test('the drawing buffer follows the canvas, the device, and the player’s own scale', () => {
  const quality = defaultQuality()
  assert.deepEqual(drawingBufferSize(800, 600, 1, quality), { width: 800, height: 600 })
  assert.deepEqual(drawingBufferSize(800, 600, 2, quality), { width: 1600, height: 1200 })
  assert.deepEqual(
    drawingBufferSize(800, 600, 4, quality),
    { width: 1600, height: 1200 },
    'a phone reporting four times the pixels does not have four times the fill rate',
  )
  assert.deepEqual(drawingBufferSize(800, 600, 1, { ...quality, resolutionScale: 0.5 }), {
    width: 400,
    height: 300,
  })
})

test('a canvas of no size still has a buffer of at least one pixel', () => {
  assert.deepEqual(drawingBufferSize(0, 0, 1, defaultQuality()), { width: 1, height: 1 })
})

test('the resolution scale is clamped in both directions', () => {
  assert.equal(clampResolutionScale(0.01), MIN_RESOLUTION_SCALE)
  assert.equal(clampResolutionScale(99), 2)
  assert.equal(clampResolutionScale(Number.NaN), 1)
})

test('a lower particle level runs fewer particles but never none of them', () => {
  assert.equal(particleBudget('off'), 0)
  assert.ok(particleBudget('low') > 0)
  assert.ok(particleBudget('high') > particleBudget('normal'))
  assert.equal(particleLimit('normal', 400), 400)
  assert.equal(particleLimit('low', 400), 100)
  assert.ok(lowQuality().particles !== 'off', 'a player who cannot see blood cannot see that they hit')
})

test('mipmaps are asked for only when a texture is actually minified', () => {
  const quality = defaultQuality()
  assert.equal(textureParameters(quality, true).generateMipmap, true)
  assert.equal(textureParameters(quality, true).minFilter, 'LINEAR_MIPMAP_LINEAR')
  assert.equal(textureParameters(quality, false).generateMipmap, false)
  assert.equal(textureParameters(quality, false).minFilter, 'LINEAR')
})

test('nearest filtering and the compatibility path turn mipmapping off', () => {
  assert.equal(textureParameters({ ...defaultQuality(), filtering: 'nearest' }, true).magFilter, 'NEAREST')
  assert.equal(
    textureParameters({ ...defaultQuality(), compatibility: true }, true).generateMipmap,
    false,
  )
})

test('a machine that cannot manage the full path is given the simple one', () => {
  assert.equal(needsCompatibility({ webgl2: true, maxTextureSize: 8192, floatTextures: true }), false)
  assert.equal(needsCompatibility({ webgl2: false }), true)
  assert.equal(needsCompatibility({ maxTextureSize: 1024 }), true)
  assert.equal(needsCompatibility({ floatTextures: false }), true)
  assert.equal(needsCompatibility({}), false, 'an unknown machine is given the benefit of the doubt')
})

test('quality steps down when frames are being missed, and never back up on its own', () => {
  const start = defaultQuality()
  assert.deepEqual(adaptQuality(start, 60), start, 'a machine keeping up is left alone')
  const noWeather = adaptQuality(start, 40)
  assert.equal(noWeather.weather, false)
  const fewer = adaptQuality(noWeather, 20)
  assert.equal(fewer.particles, 'low')
  const smaller = adaptQuality(fewer, 20)
  assert.ok(smaller.resolutionScale < fewer.resolutionScale)
  assert.deepEqual(adaptQuality(smaller, 60), smaller, 'and it does not climb back on its own')
})

test('quality stops stepping down once there is nothing left to give up', () => {
  const floor = { ...lowQuality(), resolutionScale: MIN_RESOLUTION_SCALE, particles: 'low' }
  assert.deepEqual(adaptQuality(floor, 5), floor)
})

test('the manifest is the only place a filename lives', () => {
  const manifest = defaultManifest()
  assert.equal(assetPath(manifest, 'soldier'), 'art/soldier.png')
  assert.equal(assetPath(manifest, 'nothing'), null)
  assert.deepEqual(importedAssets(manifest), [], 'every stock asset was generated for this project')
})

test('a sprite is drawn at the size the manifest says', () => {
  const manifest = defaultManifest()
  assert.deepEqual(drawSize(manifest, 'soldier', { width: 52, height: 48 }), { width: 52, height: 48 })
  const scaled = applyMod(manifest, { name: 'big', defaultScale: 2 })
  assert.deepEqual(drawSize(scaled.manifest, 'soldier', { width: 52, height: 48 }), {
    width: 104,
    height: 96,
  })
})

test('a mod may re-point and re-scale an asset that exists', () => {
  const result = applyMod(defaultManifest(), {
    name: 'winter',
    scales: { soldier: 1.2 },
    replace: { soldier: 'gostek.png' },
  })
  assert.ok('manifest' in result)
  assert.equal(result.manifest.entries.soldier.path, 'mods/winter/gostek.png')
  assert.equal(result.manifest.entries.soldier.scale, 1.2)
  assert.deepEqual(importedAssets(result.manifest), ['soldier'], 'and the replacement needs provenance')
})

test('a mod may not invent an asset, scale one out of all proportion, or escape its directory', () => {
  const manifest = defaultManifest()
  assert.ok('errors' in applyMod(manifest, { name: 'm', scales: { nothing: 1 } }))
  assert.ok('errors' in applyMod(manifest, { name: 'm', scales: { soldier: 99 } }))
  assert.ok('errors' in applyMod(manifest, { name: 'm', defaultScale: 0 }))
  assert.ok('errors' in applyMod(manifest, { name: 'm', replace: { soldier: '../../etc/passwd' } }))
  assert.ok('errors' in applyMod(manifest, { name: 'm', replace: { soldier: '/absolute.png' } }))
  assert.ok('errors' in applyMod(manifest, { name: 'm', replace: { soldier: 'https://elsewhere/x.png' } }))
})

test('a custom interface provides the slots it names and the built-in shapes cover the rest', () => {
  const result = readInterfaceSkin({ name: 'neon', provides: ['hud-health', 'hud-crosshair'] })
  assert.ok('skin' in result)
  assert.equal(slotSource(result.skin, 'hud-health'), 'skin')
  assert.equal(slotSource(result.skin, 'hud-ammo'), 'builtin')
  assert.equal(slotSource(null, 'hud-health'), 'builtin')
})

test('a custom interface naming a slot this game does not draw is refused', () => {
  assert.ok('errors' in readInterfaceSkin({ provides: ['hud-nonsense'] }))
  assert.ok('errors' in readInterfaceSkin({ provides: 'everything' }))
  assert.ok('errors' in readInterfaceSkin(null))
  assert.equal(INTERFACE_SLOTS.length, 6)
})
