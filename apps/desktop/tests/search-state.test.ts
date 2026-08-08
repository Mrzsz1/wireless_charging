import test from 'node:test'
import assert from 'node:assert/strict'
import { createLatestRequestGuard } from '../src/lib/latestRequest.ts'

test('only the newest request can publish after responses arrive out of order', () => {
  const guard = createLatestRequestGuard()
  const first = guard.next()
  const second = guard.next()
  assert.equal(guard.isCurrent(first), false)
  assert.equal(guard.isCurrent(second), true)
})

test('clearing the query invalidates an in-flight request', () => {
  const guard = createLatestRequestGuard()
  const token = guard.next()
  guard.invalidate()
  assert.equal(guard.isCurrent(token), false)
})

test('a stale failure is ignored just like a stale success', () => {
  const guard = createLatestRequestGuard()
  const oldToken = guard.next()
  const currentToken = guard.next()
  assert.equal(guard.isCurrent(oldToken), false)
  assert.equal(guard.isCurrent(currentToken), true)
})
