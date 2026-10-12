// Acceptance evidence: infra:caddy:encode docs:ops:policies ci:audit docs:ops:security-review
import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const caddy = readFileSync(resolve('infrastructure/Caddyfile'), 'utf8')
const policies = readFileSync(resolve('docs/ops/policies.md'), 'utf8')
const differences = readFileSync(resolve('docs/parity/intentional-differences.md'), 'utf8')
const review = readFileSync(resolve('docs/ops/security-review.md'), 'utf8')
const compose = readFileSync(resolve('infrastructure/compose.yaml'), 'utf8')
const ci = readFileSync(resolve('.github/workflows/ci.yml'), 'utf8')
const release = readFileSync(resolve('docs/ops/release.md'), 'utf8')

test('the edge compresses static files and proxies readiness', () => {
  assert.match(caddy, /encode zstd gzip/)
  assert.match(caddy, /\/ready/)
  assert.match(caddy, /@api path \/ws \/health \/ready/)
  assert.doesNotMatch(caddy, /@api path[^\n]*\/metrics/)
})

test('rollback and moderation policies are written down', () => {
  assert.match(policies, /Rollback/)
  assert.match(policies, /Moderation/)
  assert.match(policies, /\/DRAIN/)
  assert.match(differences, /Historical interface packs/)
  assert.match(differences, /invite/)
})

test('every gap the differences doc calls deferred really is still open', () => {
  // The doc cites a deferred gap in backticks and names an already-finished one in plain prose, so
  // the backticked ones are the claims worth checking. A row that has since been implemented has to
  // leave this list, or the doc tells players something is missing that they can already play —
  // which is how the Rambo line came to be asserted here long after Rambomatch worked.
  const gaps = readFileSync(resolve('docs/gaps/gap-list.md'), 'utf8').split(/\r?\n/)
  const cited = [...new Set((differences.match(/`G\d{2}-\d{3}`/g) ?? []).map(id => id.replaceAll('`', '')))]
  assert.ok(cited.length > 0, 'the doc cites the gaps it is talking about')
  for (const id of cited) {
    const [section, number] = id.split('-')
    const row = gaps.find(line => new RegExp(`<!-- ${section}-[A-Z0-9-]+-${number} -->`).test(line))
    assert.ok(row, `${id} names a row that exists in the gap list`)
    assert.ok(
      !row.includes('**Present:**'),
      `${id} is still open, so calling it an intentional difference is honest`,
    )
  }
})

test('invite edge locks origin, trusted proxies, and metrics', () => {
  assert.match(compose, /TRUSTED_PROXIES/)
  assert.match(compose, /PUBLIC_ORIGIN/)
  assert.match(review, /TRUSTED_PROXIES/)
  assert.match(review, /\/metrics/)
})

test('release notes name a digest rollback', () => {
  assert.match(release, /Rollback/)
  assert.match(release, /arena-data/)
})

test('CI runs lint, ops-policy, live smoke, and advisory scans', () => {
  assert.match(ci, /bun run lint/)
  assert.match(ci, /ops-policy\.test\.mjs/)
  assert.match(ci, /smoke\.mjs/)
  assert.match(ci, /cargo audit/)
  assert.match(ci, /npm audit --omit=dev/)
})
