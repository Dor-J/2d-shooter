// Acceptance evidence: scripts:record-image-digests
// Writes docs/ops/release.md from compose images + `docker image inspect` RepoDigests.
import { execSync } from 'node:child_process'
import { writeFileSync } from 'node:fs'
import { resolve } from 'node:path'

const out = resolve('docs/ops/release.md')
const compose = 'docker compose -f infrastructure/compose.yaml'

function sh(cmd) {
  try {
    return execSync(cmd, { encoding: 'utf8' }).trim()
  } catch {
    return ''
  }
}

function rows() {
  const raw = sh(`${compose} images --format json`)
  if (!raw) return []
  let parsed
  try {
    parsed = JSON.parse(raw)
  } catch {
    parsed = raw.split(/\r?\n/).filter(Boolean).map(line => {
      try {
        return JSON.parse(line)
      } catch {
        return null
      }
    })
  }
  const list = (Array.isArray(parsed) ? parsed : [parsed]).filter(Boolean)
  return list.map(row => {
      const repository = row.Repository ?? row.Name ?? ''
      const tag = row.Tag ?? 'latest'
      const ref = repository ? `${repository}:${tag}` : ''
      const digest = ref ? sh(`docker image inspect --format "{{json .RepoDigests}}" ${ref}`) : ''
      return {
        service: row.Service ?? row.Name ?? '?',
        image: ref || String(row.ID ?? ''),
        digest,
      }
    })
}

const recorded = rows()
const table = recorded.length
  ? [
      '| Service | Image | Digest |',
      '| --- | --- | --- |',
      ...recorded.map(row => `| ${row.service} | ${row.image} | ${row.digest} |`),
    ]
  : [
      '_No compose images yet. After `docker compose -f infrastructure/compose.yaml build`, re-run this script._',
      '',
      '| Service | Image | Digest |',
      '| --- | --- | --- |',
      '| api | (unrecorded) | |',
      '| web | (unrecorded) | |',
    ]

const lines = [
  '# Release digests',
  '',
  `Recorded ${new Date().toISOString().slice(0, 10)}. Keep the previous table when you deploy so rollback has a digest to start.`,
  '',
  '## Current',
  '',
  ...table,
  '',
  '## Rollback',
  '',
  '1. `/DRAIN` on the live server (or stop accepting joins).',
  '2. Wait until `GET /ready` is 503.',
  '3. `docker compose -f infrastructure/compose.yaml down`',
  '4. Start the **previous** image digests with the same `arena-data` volume.',
  '5. `/UNDRAIN` if the restored process came up drained.',
  '',
  'Matches do not survive. Bans and config do.',
  '',
]

writeFileSync(out, `${lines.join('\n')}\n`)
console.log(`wrote ${out}`)
