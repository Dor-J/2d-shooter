// Acceptance evidence: web:render:gostek
import test from 'node:test'
import assert from 'node:assert/strict'
import {
  BODY_PARTS,
  CHAINS,
  HAIR_STYLES,
  HEADGEAR,
  POSES,
  POSE_HEIGHT,
  aimAngle,
  armAngle,
  buildRig,
  bloodStain,
  burningTint,
  corpseFade,
  defaultAppearance,
  facing,
  muzzlePoint,
  readPose,
  walkPhase,
} from '../src/render/gostek.ts'

const base = {
  pos: { x: 100, y: 200 },
  aim: { x: 200, y: 200 },
  pose: 'stand',
  appearance: defaultAppearance(),
  team: [0.64, 0.79, 1, 1],
  tick: 0,
  jetting: false,
  moving: false,
}

test('a soldier is built from separate parts rather than one sprite', () => {
  const rig = buildRig(base)
  const parts = rig.map(placement => placement.part)
  for (const required of ['back-leg', 'front-leg', 'torso', 'head', 'front-arm', 'weapon']) {
    assert.ok(parts.includes(required), `the rig has a ${required}`)
  }
})

test('parts come out in draw order, so an arm is never hidden behind its own torso', () => {
  const rig = buildRig({ ...base, jetting: true })
  const order = rig.map(placement => BODY_PARTS.indexOf(placement.part))
  assert.deepEqual(order, [...order].sort((a, b) => a - b))
  assert.equal(rig[0].part, 'jets', 'the jets are behind everything')
  assert.equal(rig[rig.length - 1].part, 'weapon', 'and the weapon is in front')
})

test('the jets are drawn only when they are firing', () => {
  assert.ok(!buildRig(base).some(placement => placement.part === 'jets'))
  assert.ok(buildRig({ ...base, jetting: true }).some(placement => placement.part === 'jets'))
})

test('a soldier faces where they are aiming, not where they are walking', () => {
  assert.equal(facing({ x: 100, y: 0 }, { x: 300, y: 0 }), 1)
  assert.equal(facing({ x: 100, y: 0 }, { x: 10, y: 0 }), -1)
  const left = buildRig({ ...base, aim: { x: 0, y: 200 } })
  assert.ok(left.every(placement => placement.flip))
})

test('the aim angle is measured from the soldier to the cursor', () => {
  assert.equal(aimAngle({ x: 0, y: 0 }, { x: 10, y: 0 }), 0)
  assert.ok(Math.abs(aimAngle({ x: 0, y: 0 }, { x: 0, y: 10 }) - Math.PI / 2) < 1e-9)
})

test('an arm cannot be folded through its own chest', () => {
  // Straight up, straight down, and behind: none of these may leave the half-circle it faces.
  for (const aim of [
    { x: 100, y: 100 },
    { x: 100, y: 300 },
    { x: 140, y: 260 },
  ]) {
    const angle = armAngle({ x: 100, y: 200 }, aim)
    assert.ok(Math.abs(angle) <= Math.PI / 2 + 1e-9, `facing right: ${angle}`)
  }
  for (const aim of [
    { x: 60, y: 100 },
    { x: 60, y: 300 },
  ]) {
    const angle = armAngle({ x: 100, y: 200 }, aim)
    assert.ok(Math.abs(angle) >= Math.PI / 2 - 1e-9, `facing left: ${angle}`)
  }
})

test('the arm points where the cursor is when the cursor is in front', () => {
  const angle = armAngle({ x: 0, y: 0 }, { x: 100, y: 100 })
  assert.ok(Math.abs(angle - Math.PI / 4) < 1e-9)
})

test('the walk cycle advances while moving and stands still otherwise', () => {
  assert.equal(walkPhase(0, true), 0)
  assert.equal(walkPhase(12, true), 0.5)
  assert.equal(walkPhase(24, true), 0)
  assert.equal(walkPhase(12, false), 0, 'a soldier standing still does not stride')
  assert.equal(walkPhase(-12, true), 0.5, 'and a negative tick does not go backwards through zero')
})

test('a walking soldier moves their legs and a standing one does not', () => {
  const standing = buildRig(base)
  const walking = buildRig({ ...base, moving: true, tick: 6 })
  const legOf = rig => rig.find(placement => placement.part === 'front-leg').x
  assert.notEqual(legOf(walking), legOf(standing))
})

test('every pose has a height, and crouching really is shorter than standing', () => {
  for (const pose of POSES) assert.ok(POSE_HEIGHT[pose] > 0, `${pose} has a height`)
  assert.ok(POSE_HEIGHT.crouch < POSE_HEIGHT.stand)
  assert.ok(POSE_HEIGHT.prone < POSE_HEIGHT.crouch)
  assert.ok(POSE_HEIGHT.dead < POSE_HEIGHT.prone)
})

test('the pose is read from what the simulation said, with death winning over everything', () => {
  assert.equal(readPose('Prone', true, 100), 'prone')
  assert.equal(readPose('Crouching', true, 100), 'crouch')
  assert.equal(readPose('Rolling', true, 100), 'roll')
  assert.equal(readPose('Standing', false, 100), 'fall')
  assert.equal(readPose('Standing', true, 100), 'stand')
  assert.equal(readPose('Prone', true, 0), 'dead')
  assert.equal(readPose(undefined, true, 100), 'stand')
  assert.equal(readPose('Emote victory', true, 100), 'emote')
  assert.equal(readPose('Mercy', true, 100), 'emote')
})

test('a prone soldier is drawn lower than a standing one', () => {
  const headOf = pose => buildRig({ ...base, pose }).find(part => part.part === 'head').y
  assert.ok(headOf('prone') > headOf('stand'), 'a prone head is nearer the ground')
})

test('every hair style, headgear, and chain draws something a player can tell apart', () => {
  for (const hairStyle of HAIR_STYLES) {
    const rig = buildRig({ ...base, appearance: { ...defaultAppearance(), hairStyle } })
    const hair = rig.find(part => part.part === 'hair')
    if (hairStyle === 'bald') assert.equal(hair, undefined, 'a bald soldier has no hair layer')
    else assert.ok(hair.height > 0, `${hairStyle} has height`)
  }
  for (const headgear of HEADGEAR) {
    const rig = buildRig({ ...base, appearance: { ...defaultAppearance(), headgear } })
    const gear = rig.find(part => part.part === 'headgear')
    if (headgear === 'none') assert.equal(gear, undefined)
    else assert.ok(gear.width > 0, `${headgear} is drawn`)
  }
  for (const chain of CHAINS) {
    const rig = buildRig({ ...base, appearance: { ...defaultAppearance(), chain } })
    const worn = rig.find(part => part.part === 'chain')
    if (chain === 'none') assert.equal(worn, undefined)
    else assert.ok(worn, `${chain} is worn`)
  }
})

test('the shirt takes the team colour while the skin stays the colour a player chose', () => {
  const appearance = defaultAppearance()
  const alpha = buildRig({ ...base, team: [0, 0, 1, 1] })
  const bravo = buildRig({ ...base, team: [1, 0, 0, 1] })
  const shirtOf = rig => rig.find(part => part.part === 'torso').color
  assert.notDeepEqual(shirtOf(alpha), shirtOf(bravo))
  const headOf = rig => rig.find(part => part.part === 'head').color
  assert.deepEqual(headOf(alpha), appearance.skin)
  assert.deepEqual(headOf(alpha), headOf(bravo))
})

test('the muzzle is where the posed weapon ends, so a flash is never left behind the soldier', () => {
  const right = muzzlePoint(buildRig(base))
  assert.ok(right.x > base.pos.x, 'ahead of a soldier facing right')
  const left = muzzlePoint(buildRig({ ...base, aim: { x: 0, y: 200 } }))
  assert.ok(left.x < base.pos.x, 'and behind one facing left')
  const up = muzzlePoint(buildRig({ ...base, aim: { x: 140, y: 120 } }))
  assert.ok(up.y < right.y, 'and higher when aiming up')
})

test('the muzzle of a soldier with no weapon is nothing rather than a guess', () => {
  assert.equal(muzzlePoint([]), null)
})

test('a corpse fades out over its last second rather than vanishing', () => {
  assert.equal(corpseFade(60, 60), 1)
  assert.equal(corpseFade(30, 60), 0.5)
  assert.equal(corpseFade(0), 0)
  assert.equal(corpseFade(600, 60), 1, 'a fresh corpse is not more than solid')
})

test('blood stays on the shirt and never washes the colour out of range', () => {
  const shirt = defaultAppearance().shirt
  assert.deepEqual(bloodStain(shirt, 0), shirt)
  const stained = bloodStain(shirt, 1)
  assert.ok(stained[1] < shirt[1], 'the stain takes the green out')
  assert.ok(stained[0] > 0.3)
  for (const channel of stained) assert.ok(channel >= 0 && channel <= 1)
  assert.deepEqual(bloodStain(shirt, -4), shirt, 'a negative stain is nothing')
})

test('a burning soldier is drawn hotter the longer they burn', () => {
  const skin = defaultAppearance().skin
  assert.deepEqual(burningTint(skin, 0), skin)
  const alight = burningTint(skin, 180)
  assert.ok(alight[0] > skin[0])
  assert.ok(alight[2] < skin[2])
  for (const channel of alight) assert.ok(channel >= 0 && channel <= 1)
})
