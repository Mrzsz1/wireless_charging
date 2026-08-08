import assert from 'node:assert/strict'
import { spawn } from 'node:child_process'
import test from 'node:test'
import { assertProcessStaysAlive, processExited, terminateProcessTree } from '../scripts/process-lifecycle.mjs'

test('installer launch probes terminate the complete fixture process', async () => {
  const child = spawn(process.execPath, ['-e', 'setInterval(() => {}, 1000)'], { stdio: 'ignore', windowsHide: true })
  try {
    await assertProcessStaysAlive(child, 100)
    assert.equal(processExited(child), false)
  } finally {
    await terminateProcessTree(child)
  }
  assert.equal(processExited(child), true)
})

test('an application that exits during the launch probe is rejected', async () => {
  const child = spawn(process.execPath, ['-e', 'process.exit(7)'], { stdio: 'ignore', windowsHide: true })
  await assert.rejects(() => assertProcessStaysAlive(child, 100), /exited/)
  await terminateProcessTree(child)
})
