# QA Cross-Layer Contract

## 1. Scope / Trigger

Use this contract when changing desktop QA request identity, retrieval, provider execution, chat persistence, citations, cancellation, or repository switching.

## 2. Signatures

- Tauri command: `ask_luna(request: AskRequest, on_event: Channel<AnswerStreamEvent>) -> AskResult`.
- `AskRequest` requires client-generated UUID `requestId`, `question`, and normalized `repositoryId`; `sessionId` and `evidenceLimit` are optional.
- `AskResult` returns persisted user/assistant messages, evidence, aggregate `RetrievalDiagnostics`, `ContextBudget`, `QaRunManifest`, waterline, offline state, and `CitationValidation`.
- Paginated history commands are `list_chat_sessions_page(cursor, query, limit) -> ChatSessionPage` and `get_chat_session_page(sessionId, before, limit) -> ChatMessagePage`. Legacy full-list/detail commands remain compatibility entry points.
- `CitationValidation` includes citation syntax fields plus `groundingStatus: supported | unverified | invalid`, `zeroEvidence`, `claimCount`, `citedClaimCount`, `citationCoverage`, `unsupportedClaims`, `graphOnlyClaims`, `syntaxValid`, `coverageValid`, and `entailmentChecked`.
- Failed events may include a persisted `{ sessionId, userMessage, assistantMessage }` exchange.

## 3. Request Lifecycle

1. The frontend creates `requestId` before invoking Tauri and immediately enables Stop.
2. The backend rejects blank/non-UUID and duplicate active IDs, registers cancellation, then emits `started` and `retrieval_started` before history, retrieval, or Codex probing.
3. An early cancel is retained as a short-lived tombstone; registration consumes the already-cancelled flag.
4. SQLite/FTS and Graphify parsing run in `spawn_blocking` with an independent SQLite connection. The repository mutex only snapshots root/database path and owns the final write transaction.
5. Cancellation and repository identity are checked between retrieval channels, around Codex probing/provider generation, and before persistence.
6. Every terminal path removes its active cancellation entry. Cancelled or repository-changed requests are never persisted.

## 4. Retrieval and History

- Existing sessions are read through a 40-message/64,000-character hard cap, then grouped by request ID into complete user/assistant exchanges. Orphans are excluded.
- `qa/context.rs` allocates the model window across research contract, deterministic session memory, recent complete exchanges, current query, evidence, output reserve, safety margin, and serialization overhead. The configured window is 8,192–1,000,000 tokens; recent complete exchanges are configurable from 1–8 (default 3).
- The newest configured number of complete exchanges remain verbatim and are selected strictly newest-first. If the newest exchange exceeds the recent-history slice, it is retained instead of being skipped in favor of older shorter exchanges; the total-input gate then fails closed when it cannot fit the model window. Older exchanges become deterministic extractive memory of prior user questions/constraints; stale `[E#]` values are removed from that memory. `ContextPlan` records recent/compacted message IDs, fingerprint, estimated token breakdown, free tokens, and truncation state.
- `failed`, `cancelled`, and `unverified` messages never enter conversation history, query rewriting, or the next prompt.
- A `RetrievalQuery` owns `originalQuestion`, `resolvedQuestion`, `entities`, `intent`, and `usedHistoryMessageIds`.
- Canonical intent literals are exactly `solve | novelty | relationship | literature`. Every downstream weighting, diversity, and evidence-retention branch must match these exact values; `solution` is not an alias. When a `solve` or `novelty` query recalls a `method` candidate, final evidence selection keeps at least one method; `literature` requires a primary-paper candidate before early sufficiency.
- Deterministic rewrite runs only for explicit references or continuations such as 它们/二者/上述/第二个/继续/they/both. Broad markers `其中` and `分别` do not trigger rewrite. A self-contained question naming at least two explicit model/page entities never imports history entities. Rewrite adds bounded recent entities only; it never appends full assistant history or old `[E#]` values to FTS. `usedHistoryMessageIds` contains only messages that actually contributed a resolved entity.
- History resolves references only. Only the current evidence package can support current `[E#]` citations.
- Retrieval is a bounded agent loop of at most three passes. It starts from compositional domain concepts, may expand from recalled index titles/identifiers, and stops on evidence sufficiency, no novel terms, low unique-candidate gain, or the pass cap. Diagnostics expose only pass counts, aggregate channel timing/counts, candidate gains, and the stop enum; never expose the question, terms, snippets, paths, or secrets.
- Domain rewriting may combine independent concept signals (for example interference + concurrent, request + directional, or pricing + mobility) but must not map a complete user question or a fixture-specific paper ID to a result.
- Known domain expressions use the curated bilingual expansion table. Previously unseen Chinese wording may add bounded 3–4 character fragments only when no curated expansion fired, so generic n-grams cannot displace proven domain terms.
- Each retrieval channel is normalized independently and receives a reciprocal-rank component before intent weighting. Final selection applies deterministic diversity penalties, caps paper/book/graph occupancy, keeps at most two paper sections per source, and applies channel minimum scores before type retention.
- Post-ranking retention is monotonic: adding a required channel, method, or Wiki/paper pair must not evict the last already-satisfied required channel or the last retained method. Pair repair protects both sides of earlier selected pairs and never inserts an orphan Wiki page after its paper was displaced.
- Every selected primary-paper section is paired with its indexed Wiki source when the evidence budget permits, regardless of whether the section originated from direct paper FTS or Wiki down-drill. Wiki down-drill first executes the current query inside that source's paper sections; Abstract/Problem/Model/Introduction is an explicit `wiki_source_to_primary_fallback` navigation candidate and does not satisfy the query-matched primary-section contract.
- Retrieval diagnostics contain only aggregate `totalMs`, per-channel `name/durationMs/candidateCount`, `selectedCount`, and `cancelCheckCount`. They must never contain the question, query terms, snippets, paths, credentials, tokens, or provider payloads.

## 4.1 Session History Pagination

- Session ordering and cursor keys are `(updated_at DESC, id DESC)`; message ordering and cursor keys are `(created_at DESC, rowid DESC)`. Cursors contain only these stable keys and no message content.
- Session search is executed by SQLite across both session title and every message body. The frontend must not filter only the already-loaded page.
- A message page is fetched newest-first and reversed before presentation so each visible page remains chronological. Older pages prepend by message ID without duplicates and preserve the current scroll anchor.
- Evidence for a message page or legacy full detail is fetched in one parameterized `IN (...)` query and grouped by `message_id`; per-message evidence queries are prohibited.
- Repository switches, a new session, and a newer open-session request invalidate stale pagination responses.

## 5. Grounding and Persistence Matrix

| Condition | Result |
|---|---|
| Evidence exists; every detected factual claim contains a current non-Graphify citation in the same claim | paired `completed` exchange; `groundingStatus=supported` |
| Evidence exists; a factual claim is uncited, has an unknown ID, or is supported only by Graphify | paired `failed` exchange; `CITATION_VALIDATION_FAILED` |
| Remote evidence answer omits a required heading, intent-specific element, or minimum information claims | paired `failed` exchange; `ANSWER_COMPLETENESS_FAILED` |
| No evidence; Codex/API selected | server-owned no-source notice + model general-knowledge answer; paired `unverified` exchange |
| No evidence; offline selected | deterministic no-source notice; paired `unverified` exchange |
| Remote provider fails | paired `failed` exchange; never converted to offline completed |
| Cancelled or repository changed | no persistence |

Completed, unverified, and failed exchanges save both the exact user question and assistant result/error with the same request ID. First-turn failures create a recoverable session inside the same transaction.

Claim coverage is deterministic structural validation, not semantic entailment. Current completed answers report `entailmentChecked=false`; UI copy must say that citation coverage was checked but citation semantics were not automatically verified. Do not rename this state to factual correctness.

Before validation, one backend-owned parse pipeline protects Markdown literal/link regions, canonicalizes only provably equivalent current-evidence citation spellings, splits claims, validates completeness/grounding, and persists the normalized Markdown consumed by the frontend. `[E1；E5]`, `[E1, E5]`, and a single known ID with source-location prose may become independent `[E#]` tokens with location outside the brackets. Unknown IDs, ranges, ambiguous prose, code, math, escapes, and Markdown links are never canonicalized. A restricted repair may then delete an unknown `[E#]` token only when the same claim already contains a known non-Graphify citation. Repair never adds a citation, rewrites a fact, repairs an uncited claim, guesses evidence, or treats Graphify as factual support. Normalized group count and removed IDs are recorded in the run manifest.

## 5.1 Prompt and Run Manifest

- Codex and compatible API share one provider-neutral `PromptEnvelope` with six ordered layers: `research_contract`, `session_memory`, `recent_exchanges`, `current_query`, `evidence_bundle`, and `answer_contract`.
- History, current query, and evidence are JSON data. `<`, `>`, and `&` are escaped so embedded content cannot close an envelope layer. Provider-specific code may wrap the envelope but must not define a divergent factual contract.
- Solve, relationship, and novelty answers use six fixed Markdown headings: 结论、模型与适用前提、证据综合、方法或比较、边界/冲突/未覆盖项、库水位与复现信息. Literature lookup uses four headings: 结论、库内相关论文、主题/模型/方法、边界与复现信息, requires title/relevance/method/boundary/location, and keeps at least two factual claims. All multi-source citations are independent tokens such as `[E1] [E5]`; source locations stay outside citation brackets.
- Every assistant message stores schema/prompt/retriever/context versions, provider, requested/resolved model, temperature where applicable, output/context limits, prompt SHA-256, index snapshot SHA-256, recent/compacted/coreference-resolved history IDs, evidence checksums, context budget, repair record, and completeness result in `run_manifest`.
- Answers rejected by citation or completeness gates persist the rejected answer, evidence, validation result, and run manifest on the failed assistant message; pre-context retrieval failures retain an empty legacy manifest because no prompt/evidence snapshot exists.
- QA column migrations are idempotent column-existence checks. This module must not overwrite SQLite's global `PRAGMA user_version`, because compile-center and other subsystems share the same repository database.
- Requested and resolved model are distinct. If Codex omits its actual default model, the resolved value is `provider-default-unreported`; it must not be invented.
- Old messages hydrate with `runManifest=None`. The manifest excludes endpoint, API key, token, cookie, question/answer text, raw provider payload, and chain-of-thought.

Claim splitting treats a period followed by whitespace/end as a sentence boundary even when the preceding character is a digit (`There are 2. Next ...`). Decimal and URL dots are naturally retained because they are followed by a digit/domain character. For GFM tables, the header and separator are structural; each data row is an independent factual claim and requires its own current non-Graphify citation.
Citation tokens immediately following sentence punctuation on the same line remain attached to that claim (`事实。 [E1] [E2]` and `事实。（[E1]）`). Attachment never crosses a newline, so a later reference paragraph cannot retroactively support earlier prose.

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
- Deterministic offline answers omit Graphify-only bullets. Graph hints remain visible in the evidence panel but never pass the claim gate as factual support.
- Parsed graph indexes are cached by graph path, length, and modification time. A cache miss parses in the blocking worker; malformed/missing graph degrades to an empty channel.
- The parsed cache precomputes normalized node/relation/neighbor haystacks plus a token-to-node index. Queries take the indexed candidate union with a controlled full-scan fallback when no index key matches.
- Canonical `pages` metadata is loaded once per Graphify query; the node scoring loop must not execute SQL. Cancellation is checked inside that loop at least every 64 candidate nodes in addition to cross-channel checks.

## 8. Provider and Credential Boundaries

- Codex readiness is probed only for `codex-subscription`; ask-time results use a short 30-second cache. The secret-free status projection reads only top-level `model`/`model_reasoning_effort` and list-visible model metadata from Codex home. Empty QA overrides follow that projected selection; explicit model/reasoning values are passed to `codex exec` while `--ignore-user-config`, `--ignore-rules`, and read-only sandbox isolation remain enabled.
- Settings-page status refresh bypasses the TTL and refreshes the cache.
- Offline and compatible API requests never probe Codex.
- Codex token/cookie and API key values are never returned, persisted, or added to ordinary logs/errors.
- Compatible API SSE accepts `[DONE]` or `finish_reason=stop` as a complete stream. If the terminal `stop` frame also contains a final content delta, that token is emitted and appended before termination. `length`, other non-empty finish reasons, malformed JSON, and EOF before a legal terminator are failed exchanges; partial text is never persisted as `completed`.

## 9. Answer Rendering Contract

- Evidence-backed Codex/API generations use `qa-structured-answer-v1` JSON rather than model-authored Markdown. Each claim carries explicit `evidenceIds`; backend validation checks existence and at least one non-Graphify source directly from this array, then deterministically renders Markdown. Punctuation, prose line breaks, headings, labels, and source locations are therefore never rediscovered as claim boundaries.
- Every structured section carries a stable machine ID plus its canonical display title. The backend owns the ordered `id/title` contract and emits it to the model as a JSON array; never serialize required headings by joining them with a natural-language delimiter because a title may contain that delimiter. New output must include IDs. Title-only v1 output remains readable when it maps exactly, and the legacy adjacent literature sections `主题` + `模型与方法` are merged without changing claims or `evidenceIds`.
- The structured contract contains ordered sections, optional short group labels, claims, and a separate supplement array. The supplement cannot carry evidence tokens or repository locations and the rendered mixed-answer projection retains the existing trusted-context isolation boundary.
- The backend appends one generated `参考证据` section. It renders only `[E#]`, source kind, and compact location; full title/path metadata remains in the evidence payload and audit bundle rather than visible answer prose.
- Persisted content remains Markdown source text. The desktop renders Markdown, GFM tables, fenced code, and KaTeX through a lazy-loaded renderer.
- Raw HTML is not enabled. Remote images are replaced by text placeholders rather than fetched. Only `http(s)` links are emitted as external anchors.
- Before rendering, current `[E#]` tokens are projected to internal `evidence:` links. Known IDs open the evidence detail; unknown IDs remain visibly invalid.
- Citation projection skips fenced/inline code, math spans/blocks, escaped tokens, and labels that already own a Markdown link; these regions must remain byte-for-byte display content rather than becoming nested evidence links.
- Supported messages show deterministic claim coverage and the text `语义未自动核验`. Zero-evidence completed views show `本轮未检索到参考来源`; only an active retrieval may show `正在检索`.
- The evidence sidebar shows the context token breakdown, compacted count, snapshot ID, prompt/answer schema versions, and completeness state. Assistant actions can copy an audit bundle containing question, answer, evidence, and manifest.
- `offline-evidence` is presented as “证据浏览模式”; it is evidence navigation, not a generated research answer.
- Structured parsing/contract failures return `STRUCTURED_ANSWER_VALIDATION_FAILED` with the actual reason and a zero-claim invalid audit projection. Unknown IDs, missing evidence, and Graphify-only support remain `CITATION_VALIDATION_FAILED`; structural errors must never be converted into a synthetic `1 / 1` missing-citation result.

## 10. Tests Required

- Request registration covers early cancel and duplicate active IDs.
- Follow-up retrieval covers CCSP/GAIN reference resolution and excludes old citations.
- Intent regression covers all three canonical literals and asserts that `solve`/`novelty` evidence retains a recalled method candidate.
- History is repository-scoped, completed-only, ordered, and bounded.
- Grounding tests cover fully cited claims, partially uncited claims, unknown IDs, Graphify-only claims, offline graph omission, and zero-evidence unverified answers.
- Grounding tests also cover numeric sentence endings and GFM table header/data-row behavior.
- A partially grounded answer may use exactly one optional `## 模型补充（可能不准确）` section with the fixed notice. Claims before it retain the ordinary same-claim citation gate; claims inside it must have no evidence token or source-location marker. Such answers are `mixed`, keep the full content for display/audit, and persist a citation-free `trusted_context` projection containing only the verified prefix. `mixed` history reads that projection, so model supplementation never becomes a later retrieval entity or trusted prompt fact. Citation entailment remains outside this structural contract and `entailmentChecked` stays false.
- Codex subscription execution treats the repository timeout as an idle deadline refreshed only by valid JSONL stdout events, plus a separate bounded hard deadline. It returns stable `CODEX_IDLE_TIMEOUT` and `CODEX_TOTAL_TIMEOUT` errors; cancellation still terminates the process tree and partial output is never persisted as completed. The compatible API keeps its HTTP timeout semantics.
- Codex model/effort metadata is projected from the local top-level config and list-visible model cache. The composer sends an explicit per-request snapshot; backend resolution validates the effort against that model's reported capabilities before invoking Codex. Metadata DTOs must never expose authentication material.
- Compatible API parser tests cover token deltas, `[DONE]`, `stop`, `length`, abnormal finish reasons, malformed JSON, and EOF before termination without contacting a provider.
- The `stop` regression must include a terminal frame that carries both content and `finish_reason=stop` and assert that the final token is not lost.
- First-turn and existing-session failure tests assert paired messages and exact retry questions.
- Graph tests cover node-, relation-, and neighbor-only hits, source filtering, page ID resolution, and cache invalidation.
- Frontend tests assert immediate client request identity, completion idempotency, failed/unverified retry, exchange merging, optimistic rollback, citation-boundary copy, and completed zero-evidence empty state.
- Frontend citation tests assert that ordinary `[E#]` becomes an evidence link while code, math, escaped tokens, and existing Markdown links remain unchanged.
- Session regressions cover cursor stability, backend title/message search, batched evidence hydration, duplicate-free page merge, stale request invalidation, and older-message scroll preservation.
- Gold retrieval reports and thresholds Recall@5/10/20, MRR, binary nDCG@10, required-kind coverage, and Wiki-primary pair coverage. Generic paper fallback is excluded from ranked paper hits and pair coverage. Thresholds pin the reviewed current baseline; changes must not silently lower them.
- `gold_questions.json` is explicitly development/regression and cannot support a production accuracy claim. `heldout_questions.json` is the independently curated/frozen production entry; `tools/qa_accuracy_eval.py` reports claim precision and Wilson intervals only after exact claim coverage, canonical evidence checksum verification, two independent blinded reviews, and third-reviewer adjudication of every disagreement.
- Production fixture tests build the shared prompt, enforce the answer schema, persist the answer, and reload an identical manifest through both full and paginated history paths.
- Structured-answer regressions cover canonical section IDs, title-only v1 compatibility, the legacy literature split merge, explicit expected/actual contract arrays, dedicated structural error propagation, and preservation of genuine citation failures.
- Diagnostics tests serialize the DTO and assert that only aggregate timing/count metadata crosses the Rust/TypeScript boundary.

## 11. Wrong vs Correct Edge Cases

### Wrong

- Suppress every period after a digit; this merges `There are 2. Next claim [E1].` and lets one citation cover two claims.
- Return `Complete` before consuming content from a terminal `finish_reason=stop` frame.
- Repair source diversity with `selected.pop()`; later repairs can silently evict an earlier required channel or the only method.
- Apply a global citation regex to Markdown source; this mutates code/math and creates nested links.
- Join canonical section titles with `、` inside a prompt; `主题、模型与方法` becomes indistinguishable from two separate sections.

### Correct

- Split on period plus whitespace/end, treat table data rows as claims, and ignore only structural table rows.
- Consume a terminal frame's content before marking the SSE state complete.
- Remove only the lowest-scored unprotected candidate and skip a repair when the evidence budget has no safe slot.
- Project citation links with a Markdown-aware scanner that preserves literal regions.
- Serialize the ordered section contract as JSON objects such as `{"id":"topic_methods","title":"主题、模型与方法"}` and validate IDs before rendering backend-owned titles.
