import { spawnSync } from 'node:child_process'

const delay = (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds))

export function processExited(child) {
  return child.exitCode !== null || child.signalCode !== null
}

export async function assertProcessStaysAlive(child, observationMs = 1200) {
  if (processExited(child)) throw new Error(`application exited before launch probe (${child.exitCode ?? child.signalCode})`)
  await delay(observationMs)
  if (processExited(child)) throw new Error(`application exited during launch probe (${child.exitCode ?? child.signalCode})`)
}

async function waitForExit(child, timeoutMs) {
  const deadline = Date.now() + timeoutMs
  while (!processExited(child) && Date.now() < deadline) await delay(50)
  return processExited(child)
}

export async function terminateProcessTree(child, { platform = process.platform, timeoutMs = 8000 } = {}) {
  if (processExited(child)) return
  if (!child.pid) throw new Error('application process has no PID')

  if (platform === 'win32') {
    spawnSync('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore', windowsHide: true })
  } else {
    child.kill('SIGTERM')
  }

  if (await waitForExit(child, timeoutMs)) return

  if (platform === 'win32') {
    spawnSync('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore', windowsHide: true })
  } else {
    child.kill('SIGKILL')
  }
  if (!await waitForExit(child, 3000)) throw new Error(`application process ${child.pid} remained alive after termination`)
}
