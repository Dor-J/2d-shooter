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
  assert.match(differences, /Rambo/)
  assert.match(differences, /invite/)
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
