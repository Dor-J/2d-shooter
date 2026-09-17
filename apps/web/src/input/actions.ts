// One action per control listed in `docs/gaps/gap-list.md` section 2. Devices resolve to actions;
// only the encoder turns actions into a protocol input frame, so a control is defined exactly once.

export type ActionCategory = 'movement' | 'combat' | 'weapons' | 'communication' | 'interface' | 'media' | 'system'
export type ActionKind = 'hold' | 'press'
export type ActionTarget = 'simulation' | 'client'

export type ActionDescriptor = {
  id: string
  label: string
  category: ActionCategory
  kind: ActionKind
  target: ActionTarget
  gaps: string[]
}

const describe = (
  id: string,
  label: string,
  category: ActionCategory,
  kind: ActionKind,
  target: ActionTarget,
  gaps: string[],
): ActionDescriptor => ({ id, label, category, kind, target, gaps })

const weaponSlots = Array.from({ length: 10 }, (_, index) =>
  describe(`selectWeapon${index + 1}`, `Select weapon ${index + 1}`, 'weapons', 'press', 'simulation', ['G02-INPUT-AND-007']),
)

export const ACTIONS: ActionDescriptor[] = [
  describe('moveLeft', 'Move left', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-001']),
  describe('moveRight', 'Move right', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-001']),
  describe('jump', 'Jump', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-002']),
  describe('jet', 'Jet', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-003']),
  describe('crouch', 'Crouch', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-010']),
  describe('prone', 'Prone', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-011']),
  describe('roll', 'Roll', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-012']),
  describe('backflip', 'Backflip', 'movement', 'hold', 'simulation', ['G02-INPUT-AND-013']),
  describe('fire', 'Fire', 'combat', 'hold', 'simulation', ['G02-INPUT-AND-005']),
  describe('throwGrenade', 'Throw grenade', 'combat', 'hold', 'simulation', ['G02-INPUT-AND-006']),
  describe('reload', 'Reload', 'combat', 'hold', 'simulation', ['G02-INPUT-AND-014']),
  describe('switchWeapon', 'Switch primary/secondary', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-015']),
  describe('dropWeapon', 'Drop weapon', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-016']),
  describe('throwWeapon', 'Throw weapon (hold to charge)', 'weapons', 'hold', 'simulation', ['G02-INPUT-AND-017']),
  describe('throwKnife', 'Throw combat knife', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-018']),
  describe('pickup', 'Pick up weapons, flags, and kits', 'weapons', 'hold', 'simulation', ['G02-INPUT-AND-019', 'G02-INPUT-AND-020']),
  describe('flagThrow', 'Throw flag', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-021', 'G02-INPUT-AND-022']),
  describe('weaponMenu', 'Respawn weapon menu', 'weapons', 'press', 'client', ['G02-INPUT-AND-023']),
  describe('selectPrimary', 'Select primary weapon', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-024']),
  describe('selectSecondary', 'Select secondary weapon', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-024']),
  ...weaponSlots,
  describe('nextWeapon', 'Next weapon', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-007']),
  describe('previousWeapon', 'Previous weapon', 'weapons', 'press', 'simulation', ['G02-INPUT-AND-007']),
  describe('chat', 'Chat', 'communication', 'press', 'client', ['G02-INPUT-AND-008']),
  describe('teamChat', 'Team chat', 'communication', 'press', 'client', ['G02-INPUT-AND-025']),
  describe('console', 'Command console', 'communication', 'press', 'client', ['G02-INPUT-AND-026']),
  describe('emoteTaunt', 'Taunt', 'communication', 'press', 'simulation', ['G02-INPUT-AND-039']),
  describe('emoteVictory', 'Victory taunt', 'communication', 'press', 'simulation', ['G02-INPUT-AND-039']),
  describe('emoteMercy', 'Mercy taunt', 'communication', 'press', 'simulation', ['G02-INPUT-AND-039']),
  describe('emoteCigar', 'Cigar taunt', 'communication', 'press', 'simulation', ['G02-INPUT-AND-039']),
  describe('scoreboard', 'Scoreboard', 'interface', 'hold', 'client', ['G02-INPUT-AND-027']),
  describe('scoreboardScrollUp', 'Scroll scoreboard up', 'interface', 'press', 'client', ['G02-INPUT-AND-042']),
  describe('scoreboardScrollDown', 'Scroll scoreboard down', 'interface', 'press', 'client', ['G02-INPUT-AND-042']),
  describe('weaponStats', 'Weapon statistics', 'interface', 'press', 'client', ['G02-INPUT-AND-028']),
  describe('minimap', 'Toggle minimap', 'interface', 'press', 'client', ['G02-INPUT-AND-029']),
  describe('sniperLine', 'Toggle sniper line', 'interface', 'press', 'client', ['G02-INPUT-AND-030']),
  describe('performanceStats', 'Toggle performance overlay', 'interface', 'press', 'client', ['G02-INPUT-AND-031']),
  describe('screenshot', 'Save screenshot', 'interface', 'press', 'client', ['G02-INPUT-AND-032']),
  describe('musicToggle', 'Toggle music', 'media', 'press', 'client', ['G02-INPUT-AND-033']),
  describe('musicPrevious', 'Previous track', 'media', 'press', 'client', ['G02-INPUT-AND-034']),
  describe('musicNext', 'Next track', 'media', 'press', 'client', ['G02-INPUT-AND-034']),
  describe('demoRecord', 'Record demo', 'media', 'press', 'client', ['G02-INPUT-AND-035']),
  describe('demoFastForward', 'Fast-forward demo', 'media', 'hold', 'client', ['G02-INPUT-AND-036']),
  describe('volumeUp', 'Volume up', 'media', 'press', 'client', ['G02-INPUT-AND-041']),
  describe('volumeDown', 'Volume down', 'media', 'press', 'client', ['G02-INPUT-AND-041']),
  describe('sensitivityUp', 'Mouse sensitivity up', 'system', 'press', 'client', ['G02-INPUT-AND-040']),
  describe('sensitivityDown', 'Mouse sensitivity down', 'system', 'press', 'client', ['G02-INPUT-AND-040']),
  describe('pause', 'Pause', 'system', 'press', 'client', ['G02-INPUT-AND-037']),
  describe('minimize', 'Send the match to the background', 'system', 'press', 'client', ['G02-INPUT-AND-038']),
]

export type Action = string

// Mouse aiming is analogue, so it has no digital action; the pointer path owns it.
export const POINTER_CONTROLS: Record<string, string[]> = {
  aim: ['G02-INPUT-AND-004'],
}

// Capabilities of the input layer itself rather than individual controls.
export const CAPABILITY_CONTROLS: Record<string, string[]> = {
  touch: ['G02-INPUT-AND-009'],
  keyboardRebinding: ['G02-INPUT-AND-043'],
  mouseRebinding: ['G02-INPUT-AND-044'],
  gamepad: ['G02-INPUT-AND-045'],
  profiles: ['G02-INPUT-AND-046'],
  accessibility: ['G02-INPUT-AND-047'],
  mobileEquivalents: ['G02-INPUT-AND-048'],
}

const byId = new Map(ACTIONS.map(action => [action.id, action]))

export function actionIds(): Action[] {
  return ACTIONS.map(action => action.id)
}

export function isAction(id: string): boolean {
  return byId.has(id)
}
