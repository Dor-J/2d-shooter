import { existsSync } from 'node:fs'
import { chdir } from 'node:process'
import { fileURLToPath } from 'node:url'
import husky from 'husky'

const root = fileURLToPath(new URL('../../../', import.meta.url))
if (existsSync(new URL('../../../.git', import.meta.url))) {
  chdir(root)
  const result = husky()
  if (result) throw new Error(result)
}
