import { existsSync, readFileSync } from 'node:fs'
import { resolve } from 'node:path'
const root = resolve(import.meta.dirname, '..')
const required = ['dist/index.html','dist/data/library.json','src-tauri/tauri.conf.json']
for (const file of required) {
  if (!existsSync(resolve(root,file))) throw new Error(`missing ${file}`)
}
const html = readFileSync(resolve(root,'dist/index.html'),'utf8')
if (!html.includes('id="root"') || !/assets\/index-.*\.js/.test(html)) throw new Error('built shell incomplete')
console.log('PASS offline E2E shell smoke')
