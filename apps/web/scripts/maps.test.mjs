import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import test from 'node:test'
import { MapCache, mapCacheKey } from '../src/maps/cache.ts'
import { DownloadError, MapDownload } from '../src/maps/download.ts'

const hash = bytes => [...createHash('sha256').update(bytes).digest()]
const pms = new Uint8Array([1, 2, 3])
const texture = new Uint8Array([4, 5])
const manifest = {
  format_version: 1, name: 'Original', map_hash: 42, pms_sha256: hash(pms),
  assets: [{ path: 'textures/original.png', sha256: hash(texture), bytes: texture.length }],
  modes: ['deathmatch'], preview: null,
}

test('download reports progress and verifies every file', async () => {
  const download = new MapDownload(manifest, 5)
  download.append('maps/Original.pms', 0, 3, [1, 2])
  assert.equal(download.progress, .4)
  download.append('maps/Original.pms', 2, 3, [3])
  download.append('textures/original.png', 0, 2, [4, 5])
  assert.equal(download.progress, 1)
  assert.equal((await download.finish()).size, 2)
})

test('download rejects bad offsets, checksums, missing assets, and cancellation', async () => {
  const offset = new MapDownload(manifest, 5)
  assert.throws(() => offset.append('maps/Original.pms', 1, 3, [1]), /invalid_chunk_offset/)
  const bad = new MapDownload(manifest, 5)
  bad.append('maps/Original.pms', 0, 3, [9, 9, 9])
  await assert.rejects(bad.finish(), /checksum_mismatch/)
  const missing = new MapDownload(manifest, 5)
  missing.append('maps/Original.pms', 0, 3, [1, 2, 3])
  await assert.rejects(missing.finish(), /missing_asset/)
  missing.cancel()
  assert.throws(() => missing.append('x', 0, 1, [1]), DownloadError)
})

test('cache keys include the map and all asset hashes', async () => {
  const cache = new MapCache()
  assert.equal(await cache.has(manifest), false)
  await cache.put(manifest, new Uint8Array([7]))
  assert.equal(await cache.has(manifest), true)
  assert.deepEqual(await cache.get(manifest), new Uint8Array([7]))
  assert.match(mapCacheKey(manifest), /^42:/)
})
