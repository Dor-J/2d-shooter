// Acceptance evidence: web:maps:download
import type { MapManifest } from './types.ts'

export class DownloadError extends Error {}

export class MapDownload {
  private files = new Map<string, number[]>()
  private received = 0
  private cancelled = false
  readonly manifest: MapManifest
  readonly totalBytes: number

  constructor(manifest: MapManifest, totalBytes: number) { this.manifest = manifest; this.totalBytes = totalBytes }

  get progress() { return this.totalBytes === 0 ? 1 : Math.min(1, this.received / this.totalBytes) }

  append(path: string, offset: number, total: number, bytes: number[]) {
    if (this.cancelled) throw new DownloadError('download_cancelled')
    const current = this.files.get(path) ?? []
    if (offset !== current.length) throw new DownloadError('invalid_chunk_offset')
    if (offset + bytes.length > total) throw new DownloadError('chunk_exceeds_file')
    current.push(...bytes)
    this.files.set(path, current)
    this.received += bytes.length
  }

  cancel() { this.cancelled = true; this.files.clear() }

  async finish(): Promise<Map<string, Uint8Array>> {
    if (this.cancelled) throw new DownloadError('download_cancelled')
    const expected = [
      { path: `maps/${this.manifest.name}.pms`, sha256: this.manifest.pms_sha256 },
      ...this.manifest.assets,
    ]
    const verified = new Map<string, Uint8Array>()
    for (const file of expected) {
      const bytes = this.files.get(file.path)
      if (!bytes) throw new DownloadError(`missing_asset:${file.path}`)
      const value = new Uint8Array(bytes)
      const digest = new Uint8Array(await crypto.subtle.digest('SHA-256', value))
      if (!digest.every((byte, index) => byte === file.sha256[index])) throw new DownloadError(`checksum_mismatch:${file.path}`)
      verified.set(file.path, value)
    }
    return verified
  }
}
