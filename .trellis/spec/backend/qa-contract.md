# QA Cross-Layer Contract

## 1. Scope / Trigger

Use this contract when changing desktop QA request identity, retrieval, provider execution, chat persistence, citations, cancellation, or repository switching.

## 2. Signatures

- Tauri command: `ask_luna(request: AskRequest, on_event: Channel<AnswerStreamEvent>) -> AskResult`.
- `AskRequest` requires client-generated UUID `requestId`, `question`, and normalized `repositoryId`; `sessionId` and `evidenceLimit` are optional.
- `AskResult` returns persisted user/assistant messages, evidence, waterline, offline state, and `CitationValidation`.
- `CitationValidation` includes citation syntax fields plus `groundingStatus: supported | unverified | invalid` and `zeroEvidence`.
- Failed events may include a persisted `{ sessionId, userMessage, assistantMessage }` exchange.

## 3. Request Lifecycle

1. The frontend creates `requestId` before invoking Tauri and immediately enables Stop.
2. The backend rejects blank/non-UUID and duplicate active IDs, registers cancellation, then emits `started` and `retrieval_started` before history, retrieval, or Codex probing.
3. An early cancel is retained as a short-lived tombstone; registration consumes the already-cancelled flag.
4. SQLite/FTS and Graphify parsing run in `spawn_blocking` with an independent SQLite connection. The repository mutex only snapshots root/database path and owns the final write transaction.
5. Cancellation and repository identity are checked between retrieval channels, around Codex probing/provider generation, and before persistence.
6. Every terminal path removes its active cancellation entry. Cancelled or repository-changed requests are never persisted.

## 4. Retrieval and History

- Existing sessions contribute at most 8 `completed` user/assistant messages and 12,000 characters.
- `failed`, `cancelled`, and `unverified` messages never enter conversation history, query rewriting, or the next prompt.
- A `RetrievalQuery` owns `originalQuestion`, `resolvedQuestion`, `entities`, `intent`, and `usedHistoryMessageIds`.
- Deterministic rewrite runs only for explicit references such as 它们/二者/上述/they/both. It adds bounded recent model/page entities; it never appends full assistant history or old `[E#]` values to FTS.
- History resolves references only. Only the current evidence package can support current `[E#]` citations.

## 5. Grounding and Persistence Matrix

| Condition | Result |
|---|---|
| Evidence exists; all cited IDs are current and at least one is cited | paired `completed` exchange; `groundingStatus=supported` |
| Evidence exists; missing or unknown citation | paired `failed` exchange; `CITATION_VALIDATION_FAILED` |
| No evidence; Codex/API selected | server-owned no-source notice + model general-knowledge answer; paired `unverified` exchange |
| No evidence; offline selected | deterministic no-source notice; paired `unverified` exchange |
| Remote provider fails | paired `failed` exchange; never converted to offline completed |
| Cancelled or repository changed | no persistence |

Completed, unverified, and failed exchanges save both the exact user question and assistant result/error with the same request ID. First-turn failures create a recoverable session inside the same transaction.

## 6. Zero-Evidence Contract

- The fixed notice is owned by the backend and cannot be omitted by a model: `当前知识库没有检索到参考来源。以下内容来自模型的一般知识，未经本库证据核验。`
- Zero-evidence prompts prohibit claims of repository support, `[E#]`, wikilinks, paper locations, and book pages.
- Model-created `[E数字]` tokens are normalized to a visible no-source marker before persistence.
- `unverified` is visible and retryable in history but is not a supported answer and never contributes to subsequent context.

## 7. Graphify Contract

- Search documents include node label/description, source file/location, mapped Wiki title, community/name, edge relations, and one-hop neighbor labels.
- Node, relation, and neighbor hits are scored and reported independently; relation-only and neighbor-only queries can create candidates.
- A candidate must resolve its center node to both an existing `wiki/**/*.md` file and an indexed page ID.
- Graph evidence remains `graph_hint`; it cannot independently support factual claims.
- Parsed graph indexes are cached by graph path, length, and modification time. A cache miss parses in the blocking worker; malformed/missing graph degrades to an empty channel.

## 8. Provider and Credential Boundaries

- Codex readiness is probed only for `codex-subscription`; ask-time results use a short 30-second cache.
- Settings-page status refresh bypasses the TTL and refreshes the cache.
- Offline and compatible API requests never probe Codex.
- Codex token/cookie and API key values are never returned, persisted, or added to ordinary logs/errors.

## 9. Tests Required

- Request registration covers early cancel and duplicate active IDs.
- Follow-up retrieval covers CCSP/GAIN reference resolution and excludes old citations.
- History is repository-scoped, completed-only, ordered, and bounded.
- Grounding tests cover supported, missing, unknown, and zero-evidence unverified answers.
- First-turn and existing-session failure tests assert paired messages and exact retry questions.
- Graph tests cover node-, relation-, and neighbor-only hits, source filtering, page ID resolution, and cache invalidation.
- Frontend tests assert immediate client request identity, completion idempotency, failed/unverified retry, exchange merging, and optimistic rollback.
