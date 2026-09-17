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

export class BrowserMapStorage implements MapStorage {
  private database: Promise<IDBDatabase>
  constructor() {
    this.database = new Promise((resolve, reject) => {
      const request = indexedDB.open('arena-map-cache', 1)
      request.onupgradeneeded = () => request.result.createObjectStore('packages')
      request.onsuccess = () => resolve(request.result)
      request.onerror = () => reject(request.error)
    })
  }
  async get(key: string) { return this.request<Uint8Array | undefined>('readonly', store => store.get(key)) }
  async set(key: string, value: Uint8Array) { await this.request('readwrite', store => store.put(value, key)) }
  private async request<T>(mode: IDBTransactionMode, operation: (store: IDBObjectStore) => IDBRequest<T>): Promise<T> {
    const database = await this.database
    return new Promise((resolve, reject) => {
      const request = operation(database.transaction('packages', mode).objectStore('packages'))
      request.onsuccess = () => resolve(request.result)
      request.onerror = () => reject(request.error)
    })
  }
}

export function mapCacheKey(manifest: MapManifest): string {
  const hashes = manifest.assets.map(asset => hex(asset.sha256)).sort().join('.')
  return `${manifest.map_hash}:${hashes}`
}

export class MapCache {
  private storage: MapStorage
  constructor(storage: MapStorage = typeof indexedDB === 'undefined' ? new MemoryMapStorage() : new BrowserMapStorage()) { this.storage = storage }
  async has(manifest: MapManifest) { return (await this.storage.get(mapCacheKey(manifest))) !== undefined }
  async put(manifest: MapManifest, packageBytes: Uint8Array) { await this.storage.set(mapCacheKey(manifest), packageBytes) }
  async get(manifest: MapManifest) { return this.storage.get(mapCacheKey(manifest)) }
}

function hex(bytes: number[]) { return bytes.map(byte => byte.toString(16).padStart(2, '0')).join('') }
