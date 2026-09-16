export type AssetEntry = { path: string, sha256: number[], bytes: number }
export type MapManifest = {
  format_version: number
  name: string
  map_hash: number
  pms_sha256: number[]
  assets: AssetEntry[]
  modes: string[]
  preview: string | null
}
export type SignedMapManifest = { manifest: MapManifest, public_key: number[], signature: number[] }
