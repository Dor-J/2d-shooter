export type View = { x: number; y: number; width: number; height: number }

export function viewForCanvas(pixelWidth: number, pixelHeight: number, playerX: number, playerY = 350): View {
  const aspect = Math.max(1, pixelWidth) / Math.max(1, pixelHeight)
  const width = Math.min(1200, 700 * aspect)
  const height = Math.min(700, 1200 / aspect)
  const x = Math.max(0, Math.min(1200 - width, playerX - width / 2))
  const y = Math.max(0, Math.min(700 - height, playerY - height / 2))
  return { x, y, width, height }
}

export function screenToWorld(clientX: number, clientY: number, width: number, height: number, view: View) {
  return { x: view.x + clientX / width * view.width, y: view.y + clientY / height * view.height }
}

export function movementFromDrag(dx: number, dy: number) {
  return { left: dx < -18, right: dx > 18, jump: dy < -30 }
}
