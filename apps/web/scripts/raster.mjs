// A tiny software rasteriser, for tests only.
//
// It draws the same rectangles the WebGL renderer draws, in the same order, with the same blending.
// That is enough to catch a HUD element that moved, a colour that changed, or a layer that started
// covering another — the things a screenshot comparison is actually for — without a browser, a GPU,
// or a megabyte of baseline images.

export function createSurface(width, height, background = [0.06, 0.07, 0.09, 1]) {
  const pixels = new Float32Array(width * height * 4)
  for (let i = 0; i < width * height; i += 1) {
    pixels[i * 4] = background[0]
    pixels[i * 4 + 1] = background[1]
    pixels[i * 4 + 2] = background[2]
    pixels[i * 4 + 3] = background[3] ?? 1
  }
  return { width, height, pixels }
}

/** Source-over blending, the same as the renderer's own blend function. */
export function fillRect(surface, x, y, width, height, color) {
  const alpha = color[3] ?? 1
  if (alpha <= 0) return
  const left = Math.max(0, Math.round(x))
  const top = Math.max(0, Math.round(y))
  const right = Math.min(surface.width, Math.round(x + width))
  const bottom = Math.min(surface.height, Math.round(y + height))
  for (let py = top; py < bottom; py += 1) {
    for (let px = left; px < right; px += 1) {
      const i = (py * surface.width + px) * 4
      surface.pixels[i] = surface.pixels[i] * (1 - alpha) + color[0] * alpha
      surface.pixels[i + 1] = surface.pixels[i + 1] * (1 - alpha) + color[1] * alpha
      surface.pixels[i + 2] = surface.pixels[i + 2] * (1 - alpha) + color[2] * alpha
      surface.pixels[i + 3] = Math.min(1, surface.pixels[i + 3] + alpha)
    }
  }
}

/** A rectangle centred on a point, which is how the rig and the world objects are described. */
export function fillCentred(surface, x, y, width, height, color) {
  fillRect(surface, x - width / 2, y - height / 2, width, height, color)
}

export function pixelAt(surface, x, y) {
  const i = (Math.round(y) * surface.width + Math.round(x)) * 4
  return [
    surface.pixels[i],
    surface.pixels[i + 1],
    surface.pixels[i + 2],
    surface.pixels[i + 3],
  ].map(channel => Math.round(channel * 255))
}

/** A stable digest of the whole surface, quantised to 8 bits so float noise cannot move it. */
export function digest(surface) {
  let hash = 5381
  for (let i = 0; i < surface.pixels.length; i += 1) {
    const value = Math.round(Math.min(1, Math.max(0, surface.pixels[i])) * 255)
    hash = (Math.imul(hash, 33) ^ value) >>> 0
  }
  return hash.toString(16).padStart(8, '0')
}

/** How many pixels differ between two surfaces, for a failure message worth reading. */
export function differingPixels(a, b) {
  if (a.width !== b.width || a.height !== b.height) return Infinity
  let count = 0
  for (let i = 0; i < a.pixels.length; i += 4) {
    for (let channel = 0; channel < 4; channel += 1) {
      if (Math.abs(a.pixels[i + channel] - b.pixels[i + channel]) > 1 / 255) {
        count += 1
        break
      }
    }
  }
  return count
}
