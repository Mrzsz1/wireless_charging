# QA Cross-Layer Contract

## 1. Scope / Trigger

Use this contract when changing the desktop QA request, retrieval, provider execution, chat persistence, citations, or repository switching behavior.

## 2. Signatures

- Tauri command: `ask_luna(request: AskRequest, on_event: Channel<AnswerStreamEvent>) -> AskResult`.
- `AskRequest` requires `question`, `repositoryId`; `sessionId` and `evidenceLimit` are optional.
- `AskResult` returns persisted user/assistant messages, evidence, waterline, offline status, and `CitationValidation`.
- `chat_messages.citation_validation` is a non-null JSON text column added by QA schema version 4.

## 3. Contracts

- `repositoryId` is a normalized lowercase path with `/` separators and no trailing slash.
- Existing sessions contribute at most 8 completed user/assistant messages and 12,000 characters to the prompt.
- Conversation history resolves references only; it is not evidence and old `[E#]` values are invalid for the new turn.
- `CitationValidation` contains `citedIds`, `unknownIds`, `citationPrecision`, `hasCitations`, and `supported`.
- Graph candidates must resolve `source_file` to an existing indexed `wiki/**/*.md` page before becoming evidence.
- Codex readiness is probed only for `codex-subscription`; offline and compatible API requests do not probe Codex.

## 4. Validation & Error Matrix

| Condition | Result |
|---|---|
| Request repository ID differs from current root | `REPOSITORY_CHANGED`; no persistence |
| Repository changes after generation | old result discarded; no new-repository write |
| Answer cites an unknown `[E#]` | `CITATION_VALIDATION_FAILED` |
| Evidence exists but answer has no citation | `CITATION_VALIDATION_FAILED` |
| Remote provider fails | structured `failed`; never converted to offline completed |
| User selected offline provider | deterministic offline completed answer |
| Failed request has an existing session | persist one failed assistant message, never the optimistic user message |

## 5. Good / Base / Bad Cases

- Good: a follow-up question includes bounded completed history, cites only current evidence, and persists after all repository checks.
- Base: a first offline question has no history and persists a deterministic evidence answer.
- Bad: a Codex timeout is displayed and stored as a normal offline completed answer.

## 6. Tests Required

- Migration adds `citation_validation` to an existing chat table.
- History is repository-scoped, completed-only, ordered, and bounded.
- Valid, missing, and unknown citations have explicit assertions.
- Intent bonuses differ for solve, novelty, and relationship candidates.
- Graph tests assert source filtering, one-hop relation text, community, and canonical page ID.
- Frontend tests assert repository identity, adjacent-question retry, and optimistic rollback.

## 7. Wrong vs Correct

```rust
// Wrong: successful-looking fallback hides a provider failure.
let answer = stream_remote().unwrap_or_else(|_| offline_answer(&context));

// Correct: offline completion is only an explicit provider choice.
match settings.answer_provider.as_str() {
    PROVIDER_OFFLINE => Ok(offline_answer(&context)),
    PROVIDER_API => stream_remote(),
    _ => run_selected_provider(),
}
```
