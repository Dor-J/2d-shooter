// What to draw on the machine the player actually has.
//
// Every setting here trades picture for frame rate, and each one is a number the renderer reads
// rather than a branch it takes, so a low-end path is the same code with smaller numbers.

export const FILTERING = ['nearest', 'linear'] as const
export type Filtering = (typeof FILTERING)[number]

export const PARTICLE_LEVELS = ['off', 'low', 'normal', 'high'] as const
export type ParticleLevel = (typeof PARTICLE_LEVELS)[number]

export type QualitySettings = {
  /// A share of the device's own pixel ratio, so half on a retina screen is still a sharp picture.
  resolutionScale: number
  filtering: Filtering
  mipmaps: boolean
  particles: ParticleLevel
  weather: boolean
  /// The compatibility path: no texture atlas, no instancing, plain triangles.
  compatibility: boolean
}

export function defaultQuality(): QualitySettings {
  return {
    resolutionScale: 1,
    filtering: 'linear',
    mipmaps: true,
    particles: 'normal',
    weather: true,
    compatibility: false,
  }
}

/// The quality a slow machine gets: everything cheap, nothing missing.
///
/// Particles are reduced rather than switched off, because a player who cannot see blood cannot
/// see that they hit.
export function lowQuality(): QualitySettings {
  return {
    resolutionScale: 0.6,
    filtering: 'nearest',
    mipmaps: false,
    particles: 'low',
    weather: false,
    compatibility: true,
  }
}

export const MIN_RESOLUTION_SCALE = 0.4
export const MAX_RESOLUTION_SCALE = 2

export function clampResolutionScale(scale: number): number {
  if (!Number.isFinite(scale)) return 1
  return Math.min(MAX_RESOLUTION_SCALE, Math.max(MIN_RESOLUTION_SCALE, scale))
}

/// The backing-store size for a canvas of this CSS size.
///
/// The device pixel ratio is capped before the player's own scale is applied, because a phone that
/// reports four times the pixels does not have four times the fill rate.
export function drawingBufferSize(
  cssWidth: number,
  cssHeight: number,
  devicePixelRatio: number,
  quality: QualitySettings,
): { width: number; height: number } {
  const dpr = Math.min(Math.max(devicePixelRatio || 1, 1), 2)
  const scale = dpr * clampResolutionScale(quality.resolutionScale)
  return {
    width: Math.max(1, Math.round(Math.max(0, cssWidth) * scale)),
    height: Math.max(1, Math.round(Math.max(0, cssHeight) * scale)),
  }
}

/// The share of the full particle count this level runs.
export function particleBudget(level: ParticleLevel): number {
  return { off: 0, low: 0.25, normal: 1, high: 1.5 }[level] ?? 1
}

/// The hard cap on live particles, which is what keeps a firefight from becoming a slideshow.
export function particleLimit(level: ParticleLevel, base = 400): number {
  return Math.round(base * particleBudget(level))
}

export type GlTextureParameters = {
  minFilter: 'NEAREST' | 'LINEAR' | 'LINEAR_MIPMAP_LINEAR' | 'NEAREST_MIPMAP_NEAREST'
  magFilter: 'NEAREST' | 'LINEAR'
  generateMipmap: boolean
}

/// The filtering a texture is uploaded with.
///
/// Mipmaps are only asked for when the texture is actually being minified; a HUD atlas drawn at its
/// own size gains nothing from them and loses sharpness.
export function textureParameters(quality: QualitySettings, minified: boolean): GlTextureParameters {
  const linear = quality.filtering === 'linear'
  const mipmap = quality.mipmaps && minified && !quality.compatibility
  return {
    minFilter: mipmap
      ? linear
        ? 'LINEAR_MIPMAP_LINEAR'
        : 'NEAREST_MIPMAP_NEAREST'
      : linear
        ? 'LINEAR'
        : 'NEAREST',
    magFilter: linear ? 'LINEAR' : 'NEAREST',
    generateMipmap: mipmap,
  }
}

/// Whether the compatibility path should be taken for a given context.
///
/// A machine without WebGL2, or one that reports no float textures, gets the simple path rather
/// than a black screen.
export function needsCompatibility(capabilities: {
  webgl2?: boolean
  maxTextureSize?: number
  floatTextures?: boolean
}): boolean {
  if (capabilities.webgl2 === false) return true
  if ((capabilities.maxTextureSize ?? 4096) < 2048) return true
  if (capabilities.floatTextures === false) return true
  return false
}

/// What a machine can manage, from what it just managed.
///
/// Quality is only ever stepped down automatically, never up: a renderer that oscillates between
/// two settings is worse than one that stays on the lower.
export function adaptQuality(quality: QualitySettings, fps: number): QualitySettings {
  if (!Number.isFinite(fps) || fps >= 50) return quality
  if (fps >= 35) {
    return quality.weather ? { ...quality, weather: false } : quality
  }
  if (quality.particles !== 'low' && quality.particles !== 'off') {
    return { ...quality, weather: false, particles: 'low' }
  }
  if (quality.resolutionScale > MIN_RESOLUTION_SCALE) {
    return {
      ...quality,
      weather: false,
      resolutionScale: clampResolutionScale(quality.resolutionScale - 0.2),
    }
  }
  return quality
}
