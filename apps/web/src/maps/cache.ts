import type { MapManifest } from './types'

export interface MapStorage {
  get(key: string): Promise<Uint8Array | undefined>
  set(key: string, value: Uint8Array): Promise<void>
}

export class MemoryMapStorage implements MapStorage {
  private values = new Map<string, Uint8Array>()
  async get(key: string) { return this.values.get(key) }
  async set(key: string, value: Uint8Array) { this.values.set(key, value) }
}

export function mapCacheKey(manifest: MapManifest): string {
  const hashes = manifest.assets.map(asset => hex(asset.sha256)).sort().join('.')
  return `${manifest.map_hash}:${hashes}`
}

export class MapCache {
  private storage: MapStorage
  constructor(storage: MapStorage = new MemoryMapStorage()) { this.storage = storage }
  async has(manifest: MapManifest) { return (await this.storage.get(mapCacheKey(manifest))) !== undefined }
  async put(manifest: MapManifest, packageBytes: Uint8Array) { await this.storage.set(mapCacheKey(manifest), packageBytes) }
  async get(manifest: MapManifest) { return this.storage.get(mapCacheKey(manifest)) }
}

function hex(bytes: number[]) { return bytes.map(byte => byte.toString(16).padStart(2, '0')).join('') }
