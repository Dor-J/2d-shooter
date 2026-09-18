// A versioned content-addressed interface/mod package. Paths stay relative, sizes stay bounded,
// and a hash mismatch is an error rather than a silent load.

import { applyMod, defaultManifest, readInterfaceSkin, type AssetManifest, type ModRules } from '../render/manifest.ts'

export const PACKAGE_VERSION = 1
export const MAX_FILES = 64
export const MAX_FILE_BYTES = 256 * 1024
export const MAX_TOTAL_BYTES = 1024 * 1024
export const MIN_SCALE = 0.25
export const MAX_SCALE = 4

export const HISTORICAL_INTERFACES = [
  'Cabbage',
  'Classic',
  'Lacey V2',
  'Micro1',
  'Military',
  'Predator',
  'Soldat Style',
  'Storm',
  'Tech',
  'Text',
] as const

export type PackageMeta = {
  name: string
  license: string
  provenance: string
  preview?: string
}

export type ModPackage = {
  format_version: number
  meta: PackageMeta
  files: Record<string, string>
  scales: Record<string, number>
  cursor?: string
  hud?: { name: string; provides: string[]; positions?: Record<string, { x: number; y: number }> }
}

export type PackageError =
  | 'empty'
  | 'version'
  | 'path'
  | 'too-many-files'
  | 'file-too-large'
  | 'total-too-large'
  | 'scale'
  | 'license'
  | 'hash'
  | 'preview'

export function isSafePath(value: string): boolean {
  const normalized = value.replace(/\\/g, '/')
  return !normalized.startsWith('/') && !normalized.includes(':') && !normalized.split('/').some(segment => segment === '.' || segment === '..' || segment === '')
}

export function parsePackage(source: unknown): { package: ModPackage } | { error: PackageError } {
  if (typeof source !== 'object' || source === null) return { error: 'empty' }
  const data = source as ModPackage
  if (data.format_version !== PACKAGE_VERSION) return { error: 'version' }
  if (!data.meta?.name?.trim() || !data.meta.license?.trim() || !data.meta.provenance?.trim()) return { error: 'license' }
  if (data.meta.preview && !isSafePath(data.meta.preview)) return { error: 'preview' }
  const files = data.files ?? {}
  const names = Object.keys(files)
  if (names.length > MAX_FILES) return { error: 'too-many-files' }
  let total = 0
  for (const [path, bytes] of Object.entries(files)) {
    if (!isSafePath(path)) return { error: 'path' }
    const size = new TextEncoder().encode(bytes).length
    if (size > MAX_FILE_BYTES) return { error: 'file-too-large' }
    total += size
    if (total > MAX_TOTAL_BYTES) return { error: 'total-too-large' }
  }
  for (const scale of Object.values(data.scales ?? {})) {
    if (!Number.isFinite(scale) || scale < MIN_SCALE || scale > MAX_SCALE) return { error: 'scale' }
  }
  if (data.hud) {
    const skin = readInterfaceSkin(data.hud)
    if ('errors' in skin) return { error: 'empty' }
  }
  return { package: { ...data, files, scales: data.scales ?? {} } }
}

export async function packageHash(pkg: ModPackage): Promise<string> {
  const encoded = new TextEncoder().encode(JSON.stringify(pkg))
  if (globalThis.crypto?.subtle) {
    const digest = await crypto.subtle.digest('SHA-256', encoded)
    return [...new Uint8Array(digest)].map(byte => byte.toString(16).padStart(2, '0')).join('')
  }
  // ponytail: node unit tests have no SubtleCrypto; FNV-1a is enough to catch a mismatched blob.
  let hash = 2166136261
  for (const byte of encoded) hash = Math.imul(hash ^ byte, 16777619)
  return (hash >>> 0).toString(16).padStart(8, '0')
}

export async function requireHash(pkg: ModPackage, expected: string): Promise<true | PackageError> {
  return (await packageHash(pkg)) === expected.trim().toLowerCase() ? true : 'hash'
}

export function parseModIni(text: string): { scales: Record<string, number> } | { error: PackageError } {
  const scales: Record<string, number> = {}
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim()
    if (!line || line.startsWith('#') || line.startsWith('[')) continue
    const split = line.indexOf('=')
    if (split < 0) return { error: 'scale' }
    const name = line.slice(0, split).trim()
    const scale = Number(line.slice(split + 1).trim())
    if (!name || (!isSafePath(name) && name !== 'default')) return { error: 'path' }
    if (!Number.isFinite(scale) || scale < MIN_SCALE || scale > MAX_SCALE) return { error: 'scale' }
    scales[name] = scale
  }
  return { scales }
}

export function historicalInterface(name: string): boolean {
  return HISTORICAL_INTERFACES.some(entry => entry.toLowerCase() === name.trim().toLowerCase())
}

export function applyPackage(pkg: ModPackage, manifest: AssetManifest = defaultManifest()): { manifest: AssetManifest } | { errors: { asset: string; reason: string }[] } {
  const rules: ModRules = {
    name: pkg.meta.name,
    scales: pkg.scales,
    replace: Object.fromEntries(Object.keys(pkg.files).map(path => [path.replace(/\.[^.]+$/, ''), path])),
  }
  return applyMod(manifest, rules)
}

export async function downloadPackage(source: string | ModPackage): Promise<{ package: ModPackage; hash: string } | { error: PackageError }> {
  const parsed = parsePackage(typeof source === 'string' ? JSON.parse(source) : source)
  if ('error' in parsed) return parsed
  return { package: parsed.package, hash: await packageHash(parsed.package) }
}

export function requiredModMatch(roomHash: string | undefined, localHash: string | undefined): boolean {
  if (!roomHash) return true
  return !!localHash && roomHash.toLowerCase() === localHash.toLowerCase()
}

export const INTERFACE_PRESETS = ['default', 'minimal', 'mobile'] as const
