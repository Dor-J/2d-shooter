export type View = { x: number; y: number; width: number; height: number }

export const ARENA_W = 1200
export const ARENA_H = 700
/** Classic Soldat default window. */
export const VIEW_W = 800
export const VIEW_H = 600
/** How far aim pulls the camera, Soldat-style. */
export const LOOK = 0.35

export function viewForCanvas(
  pixelWidth: number,
  pixelHeight: number,
  playerX: number,
  playerY = 350,
  aimX?: number,
  aimY?: number,
): View {
  const aspect = Math.max(1, pixelWidth) / Math.max(1, pixelHeight)
  let width = VIEW_W
  let height = width / aspect
  if (height > VIEW_H) {
    height = VIEW_H
    width = height * aspect
  }
  width = Math.min(ARENA_W, width)
  height = Math.min(ARENA_H, height)
  const focusX = aimX == null ? playerX : playerX + (aimX - playerX) * LOOK
  const focusY = aimY == null ? playerY : playerY + (aimY - playerY) * LOOK
  const x = Math.max(0, Math.min(ARENA_W - width, focusX - width / 2))
  const y = Math.max(0, Math.min(ARENA_H - height, focusY - height / 2))
  return { x, y, width, height }
}
