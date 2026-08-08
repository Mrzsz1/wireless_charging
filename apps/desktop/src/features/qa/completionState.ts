import type { AskResult, ChatMessage } from '../../types'

export type CompletionLedger = {
  repositoryPath: string
  requestIds: Set<string>
}

export function createCompletionLedger(repositoryPath = ''): CompletionLedger {
  return { repositoryPath, requestIds: new Set<string>() }
}

/**
 * Claims a completion event before applying any UI or persistence side effect.
 * The stream channel and the invoke promise can both deliver the same result;
 * request IDs make that fan-in idempotent.
 */
export function claimCompletion(ledger: CompletionLedger, repositoryPath: string, requestId: string): boolean {
  if (ledger.repositoryPath !== repositoryPath) {
    ledger.repositoryPath = repositoryPath
    ledger.requestIds.clear()
  }
  const key = requestId.trim()
  if (!key || ledger.requestIds.has(key)) return false
  ledger.requestIds.add(key)
  return true
}

export function mergeCompletedMessages(messages: ChatMessage[], result: AskResult): ChatMessage[] {
  const completedIds = new Set([result.userMessage.id, result.assistantMessage.id])
  const retained = messages.filter((message) => !message.id.startsWith('local-') && !completedIds.has(message.id))
  return [...retained, result.userMessage, result.assistantMessage]
}
