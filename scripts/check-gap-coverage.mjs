import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const statusPattern = /^\*\*(Missing|Partial|Present):\*\*\s*/
const idPattern = /\s+<!--\s+(G\d{2}-[A-Z0-9-]+-\d{3})\s+-->$/

export function readGapFeatures(markdown) {
  const features = []
  let section = null

  for (const rawLine of markdown.split(/\r?\n/)) {
    const heading = rawLine.match(/^## (\d+\.\s+.+)$/)
    if (heading) {
      section = heading[1]
      continue
    }
    if (rawLine.startsWith('## ')) section = null
    if (!section) continue

    const bullet = rawLine.match(/^- (.+)$/)
    if (!bullet) continue
    const status = bullet[1].match(statusPattern)?.[1] ?? 'Missing'
    const id = bullet[1].match(idPattern)?.[1]
    const feature = bullet[1].replace(statusPattern, '').replace(idPattern, '')
    features.push({ id, section, feature, status })
  }

  return features
}

export function validateCoverage(markdown, coverage) {
  const gaps = readGapFeatures(markdown)
  const errors = []
  const keys = new Set()
  const ids = new Set()
  const evidence = new Set(coverage.passingEvidence ?? [])

  for (const entry of coverage.features ?? []) {
    const key = `${entry.section}\u0000${entry.feature}`
    if (ids.has(entry.id)) errors.push(`Duplicate feature ID: ${entry.id}`)
    if (keys.has(key)) errors.push(`Duplicate feature mapping: ${entry.section} / ${entry.feature}`)
    if (!entry.id || !entry.section || !entry.feature || !Array.isArray(entry.acceptance) || entry.acceptance.length === 0) {
      errors.push(`Invalid coverage entry: ${entry.id ?? '(missing ID)'}`)
    }
    ids.add(entry.id)
    keys.add(key)
  }

  let presentCount = 0
  for (const gap of gaps) {
    const entry = (coverage.features ?? []).find(item => item.section === gap.section && item.feature === gap.feature)
    if (!entry) {
      errors.push(`Unmapped feature: ${gap.section} / ${gap.feature}`)
      continue
    }
    if (gap.id && entry.id !== gap.id) errors.push(`Feature ID mismatch: ${gap.section} / ${gap.feature}`)
    if (gap.status === 'Present') {
      presentCount += 1
      if (!entry.acceptance.some(testId => evidence.has(testId))) {
        errors.push(`Present feature lacks passing evidence: ${gap.section} / ${gap.feature}`)
      }
    }
  }

  const gapKeys = new Set(gaps.map(gap => `${gap.section}\u0000${gap.feature}`))
  for (const entry of coverage.features ?? []) {
    if (!gapKeys.has(`${entry.section}\u0000${entry.feature}`)) {
      errors.push(`Stale coverage entry: ${entry.section} / ${entry.feature}`)
    }
  }

  return { errors, featureCount: gaps.length, presentCount }
}

function main(argv) {
  const gapPath = resolve(argv[0] ?? 'docs/gaps/gap-list.md')
  const coveragePath = resolve(argv[1] ?? 'docs/parity/coverage.json')
  const markdown = readFileSync(gapPath, 'utf8')
  const coverage = JSON.parse(readFileSync(coveragePath, 'utf8'))
  const result = validateCoverage(markdown, coverage)
  if (result.errors.length > 0) {
    process.stderr.write(`${result.errors.join('\n')}\n`)
    process.exitCode = 1
    return
  }
  process.stdout.write(`${result.featureCount} features mapped; ${result.presentCount} present feature${result.presentCount === 1 ? '' : 's'} ${result.presentCount === 1 ? 'has' : 'have'} passing evidence\n`)
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main(process.argv.slice(2))
