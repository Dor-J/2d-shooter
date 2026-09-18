// What a player chooses about bots when they open a room.
//
// The server is the authority on how many bots a room may hold and how hard they are; this is only
// the form's side of that conversation, kept out of the component so it can be tested without a
// browser.

/// The difficulties a server understands, in the order a menu should offer them.
export const BOT_DIFFICULTIES = ['rookie', 'normal', 'veteran', 'elite'] as const

export type BotDifficulty = (typeof BOT_DIFFICULTIES)[number]

/// The most bots a room will take. The server enforces the same cap; this only keeps the form from
/// asking for something it will be told off for.
export const MAX_BOTS = 12

export type BotChoice = {
  count: number
  difficulty: BotDifficulty
}

export const NO_BOTS: BotChoice = { count: 0, difficulty: 'normal' }

/// Reads whatever a number input produced — a string, a blank, a nonsense value — into a count the
/// server will accept.
export function clampCount(value: unknown): number {
  const asNumber = typeof value === 'number' ? value : Number.parseInt(String(value ?? ''), 10)
  if (!Number.isFinite(asNumber)) return 0
  return Math.min(MAX_BOTS, Math.max(0, Math.trunc(asNumber)))
}

/// The difficulty a stored or typed name refers to, falling back the way the server does.
export function readDifficulty(value: unknown): BotDifficulty {
  const name = String(value ?? '')
    .trim()
    .toLowerCase()
  return (BOT_DIFFICULTIES as readonly string[]).includes(name) ? (name as BotDifficulty) : 'normal'
}

/// The bot half of a `create_room` message.
///
/// A room with no bots sends nothing at all, so a server that is told to fill rooms itself keeps
/// its own default rather than being overridden with a zero nobody asked for.
export function createRoomBots(choice: BotChoice): { bots?: number; bot_difficulty?: string } {
  const count = clampCount(choice.count)
  if (count === 0) return {}
  return { bots: count, bot_difficulty: readDifficulty(choice.difficulty) }
}

/// The message that changes the bot count of a room already open.
export function setBotsMessage(choice: BotChoice): {
  type: 'bots'
  count: number
  difficulty: string
} {
  return {
    type: 'bots',
    count: clampCount(choice.count),
    difficulty: readDifficulty(choice.difficulty),
  }
}

/// How the roster should describe a room's bots.
export function describeBots(choice: BotChoice): string {
  const count = clampCount(choice.count)
  if (count === 0) return 'No bots'
  const difficulty = readDifficulty(choice.difficulty)
  const name = difficulty.charAt(0).toUpperCase() + difficulty.slice(1)
  return `${count} ${count === 1 ? 'bot' : 'bots'} · ${name}`
}
