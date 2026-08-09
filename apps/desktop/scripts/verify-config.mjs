import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
const root = resolve(import.meta.dirname, '..')
const config = JSON.parse(readFileSync(resolve(root,'src-tauri/tauri.conf.json'),'utf8'))
if (config.version !== '0.8.0') throw new Error(`expected tauri version 0.8.0, got ${config.version}`)
if (!config.identifier) throw new Error('missing identifier')
if (config.bundle?.active !== true) throw new Error('bundle must be active')
console.log('PASS tauri config 0.8.x')
console.log(`updater endpoint: ${process.env.TAURI_UPDATER_ENDPOINT ? 'configured' : 'not configured (offline mode)'}`)
console.log(`updater key: ${process.env.TAURI_UPDATER_PUBKEY ? 'configured' : 'not configured (offline mode)'}`)
