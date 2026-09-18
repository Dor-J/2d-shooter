// Which screen the player is looking at, and what they may do from it.
//
// A menu is a named place, not a pile of flags, so a test can say "first-run goes to the profile
// picker" without mounting Vue.

export const SCREENS = [
  'first-run',
  'onboarding',
  'profiles',
  'main',
  'join',
  'create',
  'options',
  'controls',
  'customize',
  'team',
  'credits',
  'help',
  'loading',
  'download',
  'match',
  'pause',
  'results',
  'disconnected',
] as const

export type Screen = (typeof SCREENS)[number]

export type MenuState = {
  screen: Screen
  previous: Screen | null
  firstRun: boolean
}

export function initialMenu(hasProfile: boolean): MenuState {
  return {
    screen: hasProfile ? 'main' : 'first-run',
    previous: null,
    firstRun: !hasProfile,
  }
}

export function openScreen(state: MenuState, screen: Screen): MenuState {
  if (state.screen === screen) return state
  return { ...state, previous: state.screen, screen, firstRun: state.firstRun && screen !== 'main' }
}

export function back(state: MenuState): MenuState {
  return { ...state, screen: state.previous ?? 'main', previous: null }
}

export function afterConnect(state: MenuState): MenuState {
  return openScreen(state, 'join')
}

export function afterJoin(state: MenuState): MenuState {
  return { ...state, screen: 'match', previous: 'join', firstRun: false }
}

export function afterLeave(state: MenuState): MenuState {
  return { ...state, screen: 'join', previous: 'match' }
}

export function afterDisconnect(state: MenuState): MenuState {
  return { ...state, screen: 'disconnected', previous: state.screen }
}

export function confirmDisconnect(state: MenuState): MenuState {
  return { ...state, screen: 'main', previous: null }
}

export function afterFirstRun(state: MenuState, mobile: boolean): MenuState {
  return openScreen(state, mobile ? 'onboarding' : 'profiles')
}

export const SCREEN_COPY: Record<Screen, { title: string; body: string }> = {
  'first-run': { title: 'Welcome', body: 'Pick a name, then try A/D to move, Space to jump, and Shift to jet.' },
  onboarding: { title: 'Touch controls', body: 'Left pad moves. Right pad aims and fires. The top buttons jump, jet, and reload.' },
  profiles: { title: 'Profiles', body: 'Choose who you play as.' },
  main: { title: 'Arena', body: 'Join a match or change your settings.' },
  join: { title: 'Join game', body: 'Pick a room, or enter a host and port.' },
  create: { title: 'Start game', body: 'Name the room and pick a mode.' },
  options: { title: 'Options', body: 'Audio, video, and network.' },
  controls: { title: 'Controls', body: 'Rebind every action.' },
  customize: { title: 'Customize', body: 'Hair, kit, and colours.' },
  team: { title: 'Choose a side', body: 'Alpha, Bravo, or spectate.' },
  credits: { title: 'Credits', body: 'Built for this project. Historical Soldat interfaces are named only until licensed.' },
  help: { title: 'Help', body: 'Move A/D · Jump Space · Crouch S · Jet Shift · Chat Enter · Scoreboard Tab.' },
  loading: { title: 'Loading map', body: 'Preparing the arena.' },
  download: { title: 'Downloading', body: 'Fetching the map package.' },
  match: { title: 'Match', body: 'Play.' },
  pause: { title: 'Paused', body: 'Resume, options, or leave.' },
  results: { title: 'Results', body: 'Who won, and with which weapons.' },
  disconnected: { title: 'Connection lost', body: 'Return to the menu or try again.' },
}
