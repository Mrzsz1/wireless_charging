import { createReadStream, existsSync } from 'node:fs'
import { createServer } from 'node:http'
import { fileURLToPath } from 'node:url'

export function manifestFor(mode, origin, artifactPath = '/fixture-update.zip', signature = 'LOCAL_FIXTURE_SIGNATURE') {
  const version = mode === 'no-update' ? '0.12.2' : '0.12.3'
  return {
    version,
    notes: `local updater fixture: ${mode}`,
    pub_date: '2026-08-02T00:00:00Z',
    platforms: {
      'windows-x86_64': {
        signature: mode === 'tampered' ? `TAMPERED_${signature}` : signature,
        url: `${origin}${artifactPath}`,
      },
    },
  }
}

export function startFixtureServer({ port = 4179, mode = 'no-update', artifact = '' } = {}) {
  const server = createServer((request, response) => {
    const origin = `http://127.0.0.1:${port}`
    if (request.url === '/health') {
      response.writeHead(200, { 'content-type': 'text/plain' }).end('ok')
      return
    }
    if (request.url === '/latest.json') {
      response.writeHead(200, { 'content-type': 'application/json', 'cache-control': 'no-store' })
      response.end(JSON.stringify(manifestFor(mode, origin)))
      return
    }
    if (request.url === '/fixture-update.zip' && artifact && existsSync(artifact)) {
      response.writeHead(200, { 'content-type': 'application/octet-stream' })
      createReadStream(artifact).pipe(response)
      return
    }
    response.writeHead(404, { 'content-type': 'text/plain' }).end('not found')
  })
  server.listen(port, '127.0.0.1', () => {
    console.log(`UPDATER_FIXTURE_READY http://127.0.0.1:${port}/latest.json mode=${mode}`)
  })
  return server
}

const isMain = process.argv[1] && fileURLToPath(import.meta.url) === fileURLToPath(new URL(`file:///${process.argv[1].replace(/\\/g, '/')}`))
if (isMain) {
  const value = (name, fallback) => process.argv.find((item) => item.startsWith(`--${name}=`))?.split('=').slice(1).join('=') || fallback
  const mode = value('mode', 'no-update')
  if (!['no-update', 'update', 'tampered'].includes(mode)) throw new Error(`unsupported fixture mode: ${mode}`)
  startFixtureServer({ port: Number(value('port', '4179')), mode, artifact: value('artifact', '') })
}
