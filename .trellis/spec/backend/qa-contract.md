# QA Cross-Layer Contract

## 1. Scope / Trigger

Use this contract when changing desktop QA request identity, retrieval, provider execution, chat persistence, citations, cancellation, or repository switching.

## 2. Signatures

- Tauri command: `ask_luna(request: AskRequest, on_event: Channel<AnswerStreamEvent>) -> AskResult`.
- `AskRequest` requires client-generated UUID `requestId`, `question`, and normalized `repositoryId`; `sessionId` and `evidenceLimit` are optional.
- `AskResult` returns persisted user/assistant messages, evidence, aggregate `RetrievalDiagnostics`, `ContextBudget`, `QaRunManifest`, waterline, offline state, and `CitationValidation`.
- Paginated history commands are `list_chat_sessions_page(cursor, query, limit) -> ChatSessionPage` and `get_chat_session_page(sessionId, before, limit) -> ChatMessagePage`. Legacy full-list/detail commands remain compatibility entry points.
- `CitationValidation` keeps legacy claim/citation fields for history compatibility and adds `appendixIntegrity` plus `appendixEvidenceIds`. For `answerFormat=natural-markdown-v2`, the latter fields are authoritative; legacy claim coverage is not a success gate.
- Failed events may include a persisted `{ sessionId, userMessage, assistantMessage }` exchange.

## 3. Request Lifecycle

1. The frontend creates `requestId` before invoking Tauri and immediately enables Stop.
2. The backend rejects blank/non-UUID and duplicate active IDs, registers cancellation, then emits `started` and `retrieval_started` before history, retrieval, or Codex probing.
3. An early cancel is retained as a short-lived tombstone; registration consumes the already-cancelled flag.
4. SQLite/FTS and Graphify parsing run in `spawn_blocking` with an independent SQLite connection. The repository mutex only snapshots root/database path and owns the final write transaction.
5. Cancellation and repository identity are checked between retrieval channels, around Codex probing/provider generation, and before persistence.
6. Every terminal path removes its active cancellation entry. Cancelled or repository-changed requests are never persisted.

## 4. Retrieval and History

- Existing sessions load every repository-scoped trusted user/assistant message and then group them by request ID into complete exchanges; there is no product-level turn, message, or character cap. Orphans are excluded.
- `qa/context.rs` allocates the 8,192–1,000,000-token model window across research contract, current query, evidence, output reserve, safety margin, exact recent exchanges, deterministic structured session memory, and serialization overhead. The UI exposes no recent-turn limit.
- Evidence is fitted first to a bounded share and unused capacity returns to history. Complete exchanges are selected contiguously newest-first until the token budget is reached; an oversized newest exchange is retained so the final total-input gate fails closed rather than silently substituting older short history. Remaining exchanges become `qa-session-memory-v1` JSON entries with source message IDs, stripped historical citations, compact user questions, and trusted answer summaries. The final serialized prompt is re-estimated before provider execution. `ContextPlan` records exact/compacted IDs, fingerprint, token breakdown, free tokens, and truncation state.
- `failed`, `cancelled`, and `unverified` messages never enter conversation history, query rewriting, or the next prompt.
- A `RetrievalQuery` owns `originalQuestion`, `resolvedQuestion`, history-resolved `entities`, `usedHistoryMessageIds`, structured `researchIntent`, `executionMode`, routing reason, resolver/router status and latency, the RetrievalContract version/status, planned/covered facet IDs, requested/attempted kinds, and unresolved source gaps. The run manifest persists only bounded IDs, kind/status values, aggregate latencies, fallback reasons, and fingerprints; it never persists raw round queries, understanding payloads, or planner payloads.
- Retrieval planning uses `qa-retrieval-contract-v2`, not a fixed answer-profile classifier. The contract expresses source scope, explicit sources, concepts, aliases, related problems, open facets, requested/must-attempt kinds, and bounded query/candidate/round budgets. `answerProfile` and `minimumEvidence` are forbidden Provider fields. Final answer formatting is independently selected by `answerFormat`; production uses natural Markdown while the legacy structured renderer remains read-compatible.
- `qa/understanding.rs` owns `ConversationResolver`, `ResolvedQuestion`, `ResearchQuery`, `ResearchIntent`, `IntentRouter`, and `ExecutionMode`. For a contextual reference with trusted history, Codex may make one bounded Provider-native `qa-understanding-v1` call before retrieval; self-contained turns skip that call. Invalid, timed-out, unavailable, or rejected output falls back to deterministic resolution and routing with explicit telemetry. Compatible API and offline evidence mode use the deterministic path.
- Deterministic rewrite runs only for explicit references or continuations such as 它们/二者/上述/第二个/继续/they/both. Broad markers `其中` and `分别` do not trigger rewrite. A self-contained question naming at least two explicit model/page entities never imports history entities. Rewrite adds bounded recent entities or a selected enumerated item only; it never appends full assistant history or old `[E#]` values to FTS. `usedHistoryMessageIds` contains only messages that actually contributed a resolved entity.
- `ResearchIntent` is domain-neutral and classifies direct fact, literature search, comparison, origin/derivation, method improvement, solution search, problem modeling, novelty, follow-up, or exploratory research. `ExecutionMode` is `direct | research | exploratory`; it is an auditable routing decision and does not bypass RetrievalContract limits or evidence requirements.
- History resolves references only. Only the current evidence package can support current `[E#]` citations.
- Retrieval is a bounded agent loop of at most three rounds. The Provider-native RetrievalContract supplies bounded bilingual facet queries before execution; the controller releases those queries only when the first-round coverage snapshot exposes a relevant gap. A final round may expand from recalled index titles/identifiers. It stops on all requested surfaces attempted, unresolved explicit source, no novel candidates, query budget exhaustion, cancellation, or the round cap. Diagnostics expose only round count, aggregate channel timing/counts/status, candidate gains, stop reason, and SHA-256 round fingerprints; never expose the question, terms, snippets, paths, or secrets.
- The semantic channel embeds the resolved query plus bounded Wiki, primary-paper-section, and core-book text with the quantized multilingual Paraphrase MiniLM L12 v2 model under local ONNX Runtime. Documents and questions are not sent to an embedding API. Vectors are persisted by hashed repository identity, knowledge snapshot, model, and document identity; snapshot changes invalidate reuse. Model/runtime acquisition, initialization, or inference failure degrades to an empty semantic channel and must not fail the answer request. Initialization failure uses a bounded retry delay and never disables semantic retrieval for the remainder of the process.
- Semantic model storage is a machine-global setting, independent from repository-scoped `LunaSettings`, and remains available before a repository is selected. The default is `%LOCALAPPDATA%/LunaWiki/fastembed`; a validated absolute writable custom directory may override it. Switching directories waits for the current semantic operation, then clears the in-memory model, corpus, and retry deadline before publishing the new path.
- Deployment inspection is strictly offline. It reports `missing | partial | invalid | ready | error` only after checking the ONNX runtime, one complete current-model snapshot, all tokenizer files, current-model `.part` files, and—when static files are complete—a 384-dimensional finite-value probe. Ordinary QA retrieval never downloads a missing model. Only the explicit download/repair command may access the network; invalid resources are quarantined rather than deleted.
- Explicit semantic download/repair emits `SemanticDownloadProgress` through a Tauri Channel. Runtime archive reads and Hugging Face model/tokenizer downloads report real accumulated bytes, remote total bytes when known, percentage, average bytes/second, phase, safe file label, and `downloading | verifying | complete | skipped | failed`; progress must never be synthesized by a timer. Cached files emit `skipped`, and inference initialization plus the 384-dimensional probe emit separate `verifying` events. Events never include URLs, absolute cache-file paths, credentials, or remote response bodies.
- Cache copy is explicit, rejects equal or nested source/target directories, uses temporary destination files plus rename, skips lock files, and preserves the source directory as a rollback copy. No cache-switch, repair, or inspection path automatically deletes an old model directory.
- RetrievalContract has hard bounds: at most twelve concepts/aliases/related problems, at most eight unique facets, four queries per facet, twenty queries total, three rounds, and only `wiki | paper | book` requested/preferred kinds. The backend revalidates these bounds after Provider schema enforcement. Invalid, timed-out, unavailable, or rejected planner output degrades to an open one-round contract that retains the complete Unicode question and extracted explicit source names; raw planner output and chain-of-thought are never persisted.
- The v2 query builder is domain-neutral. It combines the complete current question, Provider concepts/aliases/related problems, resolved source aliases, and bounded facet queries. When caps apply, it preserves both the beginning and the tail of the term stream so a late core concept cannot disappear through prefix-only `.take()` or truncation. Production code must not map domain phrases, complete questions, or fixture-specific source IDs to expansion terms.
- Title/alias, ContentBlock FTS/BM25, metadata-filtered FTS, dense vectors, and graph-mapped ContentBlocks are independent channels. Channel-native scores are never added directly across scales: candidates fuse by stable block identity through reciprocal-rank fusion, then `HybridResearchReranker` first applies deterministic explicit-source protection and reference/graph/fallback penalties and semantically reranks at most 80 bounded candidate texts with the already-deployed local embedding model. Graph nodes must map back to an active ContentBlock before they can enter the evidence set. A semantic cosine or reranker score is retrieval relevance, not factual confidence.
- Semantic reranking never downloads a model. Missing/invalid model state, inference failure, malformed embeddings, or availability errors fall back to `DeterministicResearchReranker`; the result remains usable and records reranker version, status, aggregate latency, fallback boolean, and stable fallback reason in retrieval diagnostics and `QaRunManifest`.
- Post-ranking retention is monotonic: adding a required channel, method, or Wiki/paper pair must not evict the last already-satisfied required channel or the last retained method. Pair repair protects both sides of earlier selected pairs and never inserts an orphan Wiki page after its paper was displaced.
- `EvidenceManager` owns final candidate deduplication, tie-break authority weighting, MMR-style document/type diversity, a maximum of two evidence items per primary-paper document, evidence token estimation for the authoritative `ContextPlan` budget, and parent-section expansion when a selected semantic block has an indexed broader section candidate. `QaRunManifest` records its version and aggregate input/dedup/selection/document/expansion/token counts.
- Claim verification is a separate post-generation stage. A legal evidence appendix or Evidence ID proves provenance syntax only: `ClaimExtractor` aligns each factual claim with evidence text, `VerificationProvider` assigns supported/partial/contradicted/not-verifiable/general-knowledge/inference/suggestion status, and `AnswerRepair` removes contradicted or unverifiable factual prose and lowers certainty for partial support. Provider failure records `verificationStatus=unavailable`; it never sets `entailmentChecked=true` or fabricates a verified result. `QaRunManifest` persists verifier version, fallback, status, status counts, and repair count.
- Every selected primary-paper section is paired with its indexed Wiki source when the evidence budget permits, regardless of whether the section originated from direct paper FTS or Wiki down-drill. Wiki down-drill first executes the current query inside that source's paper sections; Abstract/Problem/Model/Introduction is an explicit `wiki_source_to_primary_fallback` navigation candidate and does not satisfy the query-matched primary-section contract.
- Retrieval diagnostics contain only aggregate `totalMs`, per-channel `name/durationMs/candidateCount/round/status/errorKind/roundFingerprint`, `selectedCount`, `cancelCheckCount`, round gains, and stop reason. They must never contain the question, query terms, snippets, paths, credentials, tokens, or provider payloads.

## 4.1 Session History Pagination

- Session ordering and cursor keys are `(updated_at DESC, id DESC)`; message ordering and cursor keys are `(created_at DESC, rowid DESC)`. Cursors contain only these stable keys and no message content.
- Session search is executed by SQLite across both session title and every message body. The frontend must not filter only the already-loaded page.
- A message page is fetched newest-first and reversed before presentation so each visible page remains chronological. Older pages prepend by message ID without duplicates and preserve the current scroll anchor.
- Evidence for a message page or legacy full detail is fetched in one parameterized `IN (...)` query and grouped by `message_id`; per-message evidence queries are prohibited.
- Repository switches, a new session, and a newer open-session request invalidate stale pagination responses.

## 4.2 Markdown Corpus v2

- Markdown is the required queryable body for Wiki pages, canonical papers, and core books. PDF paths and physical/printed pages are legacy-compatible optional metadata; their absence must not block indexing, retrieval, or source navigation.
- Knowledge index schema version `3` adds `documents_v2`, `document_aliases_v2`, `content_blocks_v2`, and `content_blocks_fts_v2` without removing legacy knowledge tables or chat/session tables.
- A `DocumentRecord` uses repository-relative POSIX `markdownPath`, a canonical title, auditable aliases, authors/tags, provenance, content hash, snapshot, and `wiki | paper | book` kind. Raw/canonical bodies are read-only.
- Content is indexed at `document | section | semantic` granularity. Blocks retain the full heading path, structural role, line range, content hash, embedding text, parent, and a serialized `SourceLocator`.
- Stable source identity is `documentId + headingPath + blockId`; line ranges are a fallback, never the primary identity. `blockId` must not depend on absolute paths or line numbers.
- Source resolution is fail-closed at the repository boundary and falls back in order: exact block, current heading path, saved line range, then document. A degraded match is reported explicitly and never silently opens another document.
- Reindexing discovers the complete Markdown corpus before replacing changed documents. Unchanged `(documentId, contentHash)` records and blocks are reused; removed documents become inactive; a parse failure must leave the previous usable snapshot intact.
- Book title aliases come from auditable metadata such as source frontmatter or Wiki link aliases. Query code must not add fixture- or title-specific translations.

## 4.3 Multi-granularity Embeddings and VectorStore v2

### 1. Scope / Trigger

- Use this contract whenever ContentBlock embeddings, semantic-vector persistence, remote pgvector configuration, vector synchronization, or dense-retrieval fallback changes.
- Corpus synchronization is an explicit background operation. Ordinary QA may embed only the current query; it must not download a model or rebuild the corpus.

### 2. Signatures

- Storage interface: `VectorStore::{health, stats, upsert_batch, query, delete_snapshot, close}`. Local SQLite and remote PostgreSQL + pgvector return the same `VectorHit { blockId, score, store, modelId }` projection.
- Commands: `get_semantic_vector_status() -> SemanticVectorStatus`, `sync_semantic_vectors(onEvent: Channel<VectorSyncProgress>) -> SemanticVectorStatus`, `cancel_semantic_vector_sync()`, `save_semantic_vector_settings({ enabled, endpoint, apiKey })`, and `delete_semantic_vector_key()`.
- Local table `embedding_records_v2` is keyed by `(repository_id, model_id, block_id)` and stores snapshot, document/kind/granularity/role, dimension, content hash, vector blob, active flag, and remote-sync status.
- Remote migration `src-tauri/migrations/pgvector_rag.sql` owns `rag_embeddings`, `match_rag_embeddings`, and `rag_embedding_stats`; callers access it through the PostgREST/Supabase endpoint rather than embedding provider-specific SQL in retrieval code.

### 3. Contracts

- Each active `document | section | semantic` ContentBlock produces one `VectorRecord`. Its embedding input is the corpus-owned `embedding_text`, which includes canonical title, aliases, kind, heading path, role, and the granularity-appropriate body.
- Reuse requires exact `(blockId, contentHash, modelId)` equality. A new corpus snapshot updates unchanged records to the active snapshot; changed blocks are recomputed; removed blocks become inactive.
- Model ID is `Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q` and vectors must contain exactly 384 finite `f32` values.
- Remote endpoint metadata may be stored in the machine-global semantic settings file. The API key is stored only in the OS credential manager and is never returned by a command, written to SQLite, or included in diagnostics.
- Dense query routing is `configured healthy remote -> local SQLite -> legacy LUNAVEC1 -> lexical-only`. Remote/status queries have a short bounded timeout; synchronization may use a longer bounded timeout. Every non-cancellation dense failure is fail-soft.
- `VectorSyncProgress` reports phase, status, total/completed/computed/reused/remote-synced counts, percentage, and a safe message. Cancellation never writes the `semantic_vector_last_sync_at` completion marker.

### 4. Validation & Error Matrix

| Condition | Result |
|---|---|
| Vector dimension differs from 384, byte length is invalid, or a value is non-finite | `corrupt`; reject the record/query |
| Remote endpoint is neither HTTPS nor localhost HTTP | `configuration`; settings are not enabled |
| Remote key is absent/invalid | `authentication`; local vectors remain queryable |
| Remote timeout, 429, sleeping free instance, or network outage | sanitized `timeout | rate_limit | unavailable`; query falls back locally and sync records degraded status |
| Corpus snapshot is missing | vector sync reports `VECTOR_INDEX_MISSING`; lexical retrieval remains available |
| User cancels synchronization | `VECTOR_SYNC_CANCELLED`; partial local rows may remain reusable but the snapshot is not marked complete |
| Query embedding/model initialization fails | dense channel returns no candidates; the answer request continues through legacy semantic or lexical channels |

### 5. Good / Base / Bad Cases

- Good: a changed semantic block is embedded once, upserted locally, synchronized remotely, and recalled through filtered pgvector search with the same stable block ID used by `SourceLocator`.
- Base: remote storage is disabled; all three granularities are stored and queried locally, with the legacy LUNAVEC1 corpus retained as a read-only fallback during rollout.
- Bad: a remote request times out or the model directory is unavailable. The UI reports degraded vector status, while QA continues and never concludes “the knowledge base has no source” merely because the dense channel failed.

### 6. Tests Required

- Local round trip must assert cosine ordering, kind/granularity filters, snapshot deletion, and 384-dimension rejection.
- Incremental planning must assert zero changed blocks for identical hashes and exactly the changed block for one hash mutation across document/section/semantic inputs.
- Fake HTTP integration must assert health, stats, idempotent upsert, filter payloads, query decoding, snapshot cleanup, and authorization header handling without real credentials.
- Cancellation must assert no completion metadata. Secret-redaction tests must assert rendered errors and returned DTOs contain no API key.
- Cross-layer checks require Rust tests, `test:qa-settings`, TypeScript/Vite build, and release Cargo build.

### 7. Wrong vs Correct

#### Wrong

```text
QA request -> rebuild whole repository vector file -> remote failure -> fail answer
```

#### Correct

```text
explicit sync -> hash-level local upsert -> optional remote sync
QA request -> embed query -> remote (bounded) -> local v2 -> legacy semantic -> lexical
```

The correct flow prevents per-conversation downloads, preserves offline research use, and keeps retrieval independent of the selected vector-store provider.

## 4.4 Hybrid RetrievalContract v2 and Bounded Agentic Rounds

### 1. Scope / Trigger

- Use this contract whenever query planning, explicit-source resolution, Markdown ContentBlock retrieval, channel fusion, reranking, coverage control, or retrieval diagnostics change.
- The contract governs evidence candidate discovery only. It does not prescribe final answer headings, claim counts, citation entailment, or a factual “sufficient/insufficient” verdict.

### 2. Signatures

- Planner contract: `RetrievalContract { scope, concepts, aliases, relatedProblems, facets, requestedKinds, mustAttemptKinds, budget }` with `schemaVersion = "qa-retrieval-contract-v2"`.
- Source resolver: `resolve_sources(connection, question, contract) -> SourceResolution`.
- Retrieval engine: `run_retrieval(connection, root, question, contract, cancelled) -> RetrievalOutcome`.
- Channel status values are `not_requested | attempted_zero_hit | succeeded_with_hits | degraded | failed`; current fail-soft adapters use `degraded` for recoverable channel errors.
- Stop reasons include `all_requested_surfaces_attempted`, `unresolved_explicit_source`, `no_novel_candidates`, `query_budget_exhausted`, `max_rounds`, and cancellation through the request lifecycle.

### 3. Contracts

- Codex planning uses Provider-native JSON Schema and a complete valid example. Unknown fields fail closed. `answerProfile` and `minimumEvidence` must not appear in the schema or accepted payload.
- Source-constrained questions resolve titles and auditable aliases first, then push canonical document IDs into lexical and dense filters. An unresolved explicit source is a recorded gap; unrelated documents cannot satisfy it.
- Open questions execute every requested kind independently. Zero hits are an attempted state, not proof that the source kind is absent from the repository.
- Provider-generated facet queries are bounded and queued for a later round. The coverage controller decides whether to release them from observed gaps; it never decides whether the evidence is factually true or “enough”. The engine performs at most the initial round plus two follow-up rounds.
- All blocking retrieval remains inside the existing QA worker and checks the shared cancellation flag between channels and rounds. Individual non-cancellation channel failures are fail-soft and do not erase successful channels.
- Production defaults to v2. `LUNAWIKI_RAG_RETRIEVER_V2=false` (also `0`, `off`, or `no`) is the emergency rollback. During the evaluation rollout, open scope treats legacy and v2 as independent ranked lists and fuses them by stable-ID RRF; resolved source-constrained scope uses v2 exclusively.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Planner returns unknown/unbounded fields or is unavailable | Use the deterministic open fallback with the complete current question; record `failed_fallback` |
| Explicit title/alias resolves | Restrict FTS and dense retrieval to those document IDs; keep an exact-title candidate protected |
| Explicit source does not resolve | Record the source gap and stop with `unresolved_explicit_source`; never substitute an unrelated source |
| Requested paper/book channel returns no row | Record `attempted_zero_hit`; do not convert it to `not_requested` or “repository has none” |
| Dense, Graph, or reranker implementation fails | Record a sanitized degraded channel/model state and continue with the remaining candidates |
| Graph returns a relation without an active ContentBlock | Exclude it from factual evidence |
| Query/round budget ends or cancellation is set | Stop at the boundary, preserve the reason, and never start an unbounded retry loop |

### 5. Good / Base / Bad Cases

- Good: `《近似算法》中有没有移动路径规划` resolves the book alias, searches only that Markdown document, and retrieves its TSP/Euclidean TSP block through lexical and/or dense matching.
- Base: `有没有文献或者哪本书涉及移动路径规划` attempts paper and book independently; one may have zero hits while the other returns ranked ContentBlocks, and both states remain auditable.
- Bad: four weak Wiki hits satisfy a numeric threshold before paper/book were attempted, or a high raw Graph score outranks an exact in-book body block.

### 6. Tests Required

- Schema tests reject `answerProfile`, `minimumEvidence`, unknown fields, invalid kinds, and budgets above three rounds or twenty queries.
- Query-builder tests preserve a core concept at the tail of a long Unicode question.
- Source tests cover alias resolution, unresolved gaps, document-ID filtering, and no cross-document leakage.
- Retrieval tests cover open paper/book zero-hit attempts, required-facet second-round expansion, RRF, reference/graph penalties, channel degradation, cancellation, and the three-round cap.
- A real-repository regression covers both the source-constrained Approximation Algorithms question and the open literature-or-book mobile-path-planning question.
- Cross-layer checks require Rust fmt/check/tests, frontend contract tests and production build, plus the Rust release build.

### 7. Wrong vs Correct

#### Wrong

```text
question -> fixed intent/profile -> take first N terms -> add raw channel scores
         -> stop when candidate_count >= minimumEvidence
```

#### Correct

```text
question -> RetrievalContract -> source resolution -> independent filtered channels
         -> stable-ID RRF -> reranker/diversity -> soft coverage snapshot
         -> bounded Provider expansion or index expansion -> stop with audit reason
```

## 5. Natural Markdown v2, Evidence Appendix, and Persistence

### 1. Scope / Trigger

- Use this contract whenever final-answer prompting, answer validation, evidence-link creation, source navigation, message persistence, or trusted-history projection changes.
- Production format is `natural-markdown-v2`. `qa-structured-answer-v1` remains a rollback and old-history compatibility format, not the default generation target.

### 2. Signatures

- `natural_answer::render(answer, evidence) -> NaturalAnswerResult { markdown, validation, repair }`.
- `EvidenceItem.locator: Option<SourceLocator>`; old rows may omit it and must still deserialize.
- `read_source_locator(locator: SourceLocator) -> ResolvedSourceDocument { title, body, location }`.
- `QaRunManifest.answerFormat` is `natural-markdown-v2 | structured-v1 | legacy-markdown`; old manifests hydrate an empty/default value.

### 3. Contracts

- The Provider returns ordinary Markdown. It does not emit evidence IDs, repository paths, a reference appendix, fixed headings, fixed claim counts, or structured-answer JSON.
- The backend strips any Provider-authored `## 参考证据`, removes citation-like `[E#]` tokens, blocks unsafe Markdown targets, redacts visible absolute Windows/UNC paths, and enforces only non-empty/length/stream-completion safety.
- The backend deterministically appends one `## 参考证据` section from selected, non-Graph evidence that owns a valid `SourceLocator`. Link labels are compact source kind + canonical title + useful heading. The link target is the opaque current `evidence:E#` identity; arbitrary Provider paths never become navigation targets.
- `appendixIntegrity` is true only when the appendix is backed by persisted current evidence locators; `appendixEvidenceIds` must be the emitted ordered IDs. Semantic entailment is out of scope and remains `entailmentChecked=false`.
- Locator resolution is repository-boundary checked and degrades in order: exact block, heading path, saved line range, document. Every fallback returns `matchedBy` and a user-visible `degradedReason`; it never guesses another document.
- Zero evidence produces the backend-owned unverified notice, no appendix, and a paired `unverified` exchange. `unverified`, `failed`, and `cancelled` content never enters trusted history. Before trusted-history projection, both model-supplement and backend evidence-appendix regions are removed.
- Production defaults to v2. `LUNAWIKI_RAG_ANSWER_V2=false` (also `RAG_ANSWER_V2`/`rag_answer_v2`, values `0|false|off|no`) restores legacy generation without deleting v2 messages or the structured parser.

### 4. Validation & Error Matrix

| Condition | Result |
|---|---|
| Evidence exists and at least one selected non-Graph item has a valid locator | backend appendix, paired `completed` exchange; `appendixIntegrity=true` |
| Provider emits `[E99]`, its own appendix, `file:`/app-protocol target, or visible absolute path | sanitize/redact it; never register it as source navigation |
| Evidence rows exist but none has a usable locator | no evidence appendix; `groundingStatus=unverified`; never invent a path |
| Answer body is empty/oversized or Provider stream is incomplete | paired `failed` exchange; `ANSWER_VALIDATION_FAILED` or Provider terminal error |
| No evidence; Codex/API selected | server-owned no-source notice + model general-knowledge answer; paired `unverified` exchange |
| No evidence; offline selected | deterministic no-source notice; paired `unverified` exchange |
| Remote provider fails | paired `failed` exchange; never converted to offline completed |
| Cancelled or repository changed | no persistence |
| Locator block/hash drifted but heading or line still resolves | open same Markdown with explicit degradation metadata |
| Locator path escapes repository or document identity mismatches | fail closed; do not open a file |

Completed, unverified, and failed exchanges save both the exact user question and assistant result/error with the same request ID. First-turn failures create a recoverable session inside the same transaction.

### 5. Good / Base / Bad Cases

- Good: natural prose with question-appropriate headings is persisted unchanged except for a backend-created short appendix; clicking a link opens the exact Markdown block.
- Base: an old structured JSON/Markdown message reloads through the legacy renderer and its existing `[E#]` links still open.
- Bad: require headings such as `结论`/`模型与方法`, count claims, trust Provider `evidenceIds`, or parse a local path from answer prose to open a file.

### 6. Tests Required

- Unit tests cover ordinary Markdown without fixed headings, empty/oversized output, Provider-authored appendix removal, unknown `[E99]`, unsafe links, absolute-path redaction, and zero evidence.
- Locator tests cover exact block, heading fallback, line fallback, document fallback, Markdown-only book/paper rows with empty PDF fields, and traversal rejection.
- Persistence tests assert `answerFormat`, evidence locator snapshots, appendix IDs, old structured history, and trusted-context appendix exclusion.
- Cross-layer tests assert short appendix labels, locator service/command wiring, natural citation-summary wording, frontend production build, full Rust tests, and release build.

### 7. Wrong vs Correct

#### Wrong

```text
Provider JSON -> fixed section/role/claim validation -> model-owned E# -> parse path from prose
```

#### Correct

```text
Provider Markdown -> minimal safety normalization -> backend selected-evidence appendix
                  -> persisted locator snapshot -> repository-bound resolver -> Markdown focus
```

## 5.1 Prompt and Run Manifest

- Codex and compatible API share one provider-neutral `PromptEnvelope` with six ordered layers: `research_contract`, `session_memory`, `recent_exchanges`, `current_query`, `evidence_bundle`, and `answer_contract`.
- History, current query, and evidence are JSON data. `<`, `>`, and `&` are escaped so embedded content cannot close an envelope layer. Provider-specific code may wrap the envelope but must not define a divergent factual contract.
- Natural v2 answers use question-appropriate prose and headings chosen by the Provider. The answer contract asks for a direct answer, evidence-bounded model/method discussion, and explicit uncertainty where needed, but has no canonical section array or minimum claim count. The backend, not the Provider, adds source links.
- Every assistant message stores schema/prompt/retriever/context versions, provider, requested/resolved model, temperature where applicable, output/context limits, prompt SHA-256, index snapshot SHA-256, recent/compacted/coreference-resolved history IDs, resolver/router/planner/reranker status and aggregate latency, structured research intent and execution mode, fallback reasons, evidence checksums, context budget, repair record, and completeness result in `run_manifest`.
- Evidence-backed natural runs record `structuredOutputMode=natural-markdown` and `answerFormat=natural-markdown-v2`. Planner calls may still use Provider-native JSON Schema because retrieval planning and final-answer rendering are independent contracts.
- Answers rejected by citation or completeness gates persist the rejected answer, evidence, validation result, and run manifest on the failed assistant message; pre-context retrieval failures retain an empty legacy manifest because no prompt/evidence snapshot exists.
- QA column migrations are idempotent column-existence checks. This module must not overwrite SQLite's global `PRAGMA user_version`, because compile-center and other subsystems share the same repository database.
- Requested and resolved model are distinct. If Codex omits its actual default model, the resolved value is `provider-default-unreported`; it must not be invented.
- Old messages hydrate with `runManifest=None`. The manifest excludes endpoint, API key, token, cookie, question/answer text, raw provider payload, and chain-of-thought.

Claim splitting and same-claim citation attachment are legacy structured/grounding compatibility behavior only. They must not gate a `natural-markdown-v2` completion.

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
- Codex questions may use two separate short-lived `codex exec --output-schema` planning calls: a pre-retrieval understanding call receives the bounded current question and trusted recent history, while the RetrievalContract call after baseline retrieval receives only the resolved question and bounded candidate summaries. Both emit no UI answer tokens, use the same requested model/effort snapshot, validate closed schemas, and fail soft to deterministic behavior. The final natural-answer call is separately isolated and does not receive an answer JSON Schema.
- When the answer-v2 rollback flag is disabled, legacy evidence-backed Codex runs may still generate the old intent/evidence schema inside the isolated temporary workspace. That file is removed with the workspace. Compatible API final answers use the same natural Markdown contract in v2.
- Settings-page status refresh bypasses the TTL and refreshes the cache.
- Offline and compatible API requests never probe Codex.
- Codex token/cookie and API key values are never returned, persisted, or added to ordinary logs/errors.
- Compatible API SSE accepts `[DONE]` or `finish_reason=stop` as a complete stream. If the terminal `stop` frame also contains a final content delta, that token is emitted and appended before termination. `length`, other non-empty finish reasons, malformed JSON, and EOF before a legal terminator are failed exchanges; partial text is never persisted as `completed`.

## 9. Answer Rendering and Source Navigation Contract

- Persisted natural v2 content is Markdown source text plus the backend-owned appendix. The desktop renders Markdown, GFM tables, fenced code, and KaTeX through a lazy-loaded renderer.
- Raw HTML is not enabled. Remote images are replaced by text placeholders. Only `http(s)` links become external anchors; `evidence:` links are resolved against the message's persisted evidence collection.
- Appendix labels show short user-facing source descriptions rather than full filenames or absolute paths. Detailed snippets and retrieval reasons remain in the evidence panel and audit bundle.
- Clicking a current evidence link passes its persisted `SourceLocator` to `read_source_locator`. The returned Markdown is shown in an internal read-only source view and focused using `headingPath`/`lineStart`; the UI shows fallback/degradation state rather than silently presenting a wrong location.
- Legacy inline `[E#]` projection remains Markdown-aware: it skips fenced/inline code, math, escapes, and labels already inside links. Unknown IDs remain visibly invalid.
- Natural supported messages summarize appendix integrity, not claim coverage. The UI must not describe retrieval relevance as factual correctness, and must continue to state that semantic entailment is not automatically checked.
- The evidence sidebar shows context budget, snapshot, prompt/answer versions, retrieval rounds/stop reason, and current evidence. Assistant actions can copy an audit bundle containing question, answer, evidence, and manifest.
- `offline-evidence` is presented as “证据浏览模式”; it is evidence navigation, not a generated research answer.
- Legacy structured parsing errors keep `STRUCTURED_ANSWER_VALIDATION_FAILED`. Natural body/appendix validation uses `ANSWER_VALIDATION_FAILED`; new natural runs must never fail because a canonical section title, role, or claim count is absent.

## 10. Tests Required

- Request registration covers early cancel and duplicate active IDs.
- Follow-up retrieval covers CCSP/GAIN reference resolution and excludes old citations.
- Intent regression covers all three canonical literals and asserts that `solve`/`novelty` evidence retains a recalled method candidate.
- History is repository-scoped, completed-only, ordered, and bounded.
- Natural-answer tests cover ordinary Markdown, unknown Provider IDs, unsafe links/paths, appendix integrity, locator-less evidence, and zero-evidence unverified answers. Legacy grounding tests continue to cover claim splitting and structured history compatibility but do not define the v2 success gate.
- A natural answer may use the exact optional `## 模型补充（可能不准确）` section with the fixed notice. Such answers are `mixed`, remain fully visible for display/audit, and persist trusted context with both that supplement and the backend appendix removed. Model supplementation and evidence-link prose therefore never become later retrieval entities or trusted prompt facts. Citation entailment remains outside the contract and `entailmentChecked` stays false.
- Codex subscription execution treats the repository timeout as an idle deadline refreshed only by valid JSONL stdout events, plus a separate bounded hard deadline. It returns stable `CODEX_IDLE_TIMEOUT` and `CODEX_TOTAL_TIMEOUT` errors; cancellation still terminates the process tree and partial output is never persisted as completed. The compatible API keeps its HTTP timeout semantics.
- Codex Provider tests assert `--output-schema` only for planner/legacy calls, schema placement/cleanup, natural final-answer execution without a schema, and a distinct `CODEX_OUTPUT_SCHEMA_REJECTED` error when a schema-enabled call is explicitly rejected.
- Codex model/effort metadata is projected from the local top-level config and list-visible model cache. The composer sends an explicit per-request snapshot; backend resolution validates the effort against that model's reported capabilities before invoking Codex. Metadata DTOs must never expose authentication material.
- Compatible API parser tests cover token deltas, `[DONE]`, `stop`, `length`, abnormal finish reasons, malformed JSON, and EOF before termination without contacting a provider.
- The `stop` regression must include a terminal frame that carries both content and `finish_reason=stop` and assert that the final token is not lost.
- First-turn and existing-session failure tests assert paired messages and exact retry questions.
- Graph tests cover node-, relation-, and neighbor-only hits, source filtering, page ID resolution, and cache invalidation.
- Semantic retrieval tests cover cosine ordering, similarity-floor filtering, exact vector serialization, and optional-table degradation without downloading a model during the ordinary unit-test suite.
- Semantic deployment tests cover missing/partial classification without network access, complete snapshot component checks, cache-switch state reset, non-destructive cache copy, and global settings round-trip without repository SQLite.
- Semantic download progress tests cover exact accumulated-byte percentages, completion, cached-file detection without network access, Channel wiring, and frontend rendering of file/bytes/percentage/speed beside the repair action.
- Understanding tests cover the closed Provider schema, unknown history IDs, deterministic provider fallback, 50 frozen follow-up cases, and ordinal references. RetrievalContract tests cover closed Provider schema objects, open facet IDs, unknown-field rejection, value/array bounds, fail-soft planning, source scope, and facet-only expansion after a baseline miss.
- Frontend tests assert immediate client request identity, completion idempotency, failed/unverified retry, exchange merging, optimistic rollback, citation-boundary copy, and completed zero-evidence empty state.
- Frontend citation tests assert that ordinary `[E#]` becomes an evidence link while code, math, escaped tokens, and existing Markdown links remain unchanged.
- Session regressions cover cursor stability, backend title/message search, batched evidence hydration, duplicate-free page merge, stale request invalidation, and older-message scroll preservation.
- Gold retrieval reports and thresholds Recall@5/10/20, MRR, binary nDCG@10, required-kind coverage, and Wiki-primary pair coverage. Generic paper fallback is excluded from ranked paper hits and pair coverage. Thresholds pin the reviewed current baseline; changes must not silently lower them.
- `gold_questions.json` is explicitly development/regression and cannot support a production accuracy claim. `heldout_questions.json` is the independently curated/frozen production entry; `tools/qa_accuracy_eval.py` reports claim precision and Wilson intervals only after exact claim coverage, canonical evidence checksum verification, two independent blinded reviews, and third-reviewer adjudication of every disagreement.
- Production fixture tests build the shared natural prompt, accept ordinary Markdown without fixed headings, generate the backend appendix, persist the answer, and reload an identical manifest through both full and paginated history paths.
- Structured-answer regressions cover canonical section IDs, title-only v1 compatibility, the legacy literature split merge, explicit expected/actual contract arrays, dedicated structural error propagation, and preservation of genuine citation failures.
- Role regressions cover natural display labels with explicit roles, missing required roles, unknown roles, bounded legacy label aliases, and proof that required Chinese phrases need not occur in rendered Markdown.
- Diagnostics tests serialize the DTO and assert that only aggregate timing/count metadata crosses the Rust/TypeScript boundary.

## 11. Wrong vs Correct Edge Cases

### Wrong

- Suppress every period after a digit; this merges `There are 2. Next claim [E1].` and lets one citation cover two claims.
- Return `Complete` before consuming content from a terminal `finish_reason=stop` frame.
- Repair source diversity with `selected.pop()`; later repairs can silently evict an earlier required channel or the only method.
- Apply a global citation regex to Markdown source; this mutates code/math and creates nested links.
- Join canonical section titles with `、` inside a prompt; `主题、模型与方法` becomes indistinguishable from two separate sections.
- Search rendered Markdown for `模型或方法` or `证据边界`; equivalent display wording produces false completeness failures.

### Correct

- Split on period plus whitespace/end, treat table data rows as claims, and ignore only structural table rows.
- Consume a terminal frame's content before marking the SSE state complete.
- Remove only the lowest-scored unprotected candidate and skip a repair when the evidence budget has no safe slot.
- Project citation links with a Markdown-aware scanner that preserves literal regions.
- Serialize the ordered section contract as JSON objects such as `{"id":"topic_methods","title":"主题、模型与方法"}` and validate IDs before rendering backend-owned titles.
- Validate explicit claim roles such as `model_or_method` and `evidence_boundary`; allow labels such as `求解方法` or `模型边界` without changing completeness semantics.

## 12. Scientific RAG Evaluation and Rollout

### 12.1 Executable contract

- The repository-scoped entry point is `npm run eval:rag`; it runs the Rust `rag-eval` binary against `evals/rag_retrieval_cases.json` and writes ignored `evals/reports/rag-evaluation-latest.{json,md}` artifacts.
- `qa-rag-evaluation-cases-v1` is a closed fixture schema. Unknown fields, duplicate case IDs, invalid kinds/scopes, malformed conversations, and contradictory zero-evidence expectations fail before retrieval.
- Evaluation builds an isolated in-memory SQLite index from the real Markdown repository. It does not call an answer Provider, download a model, mutate the repository database, or require network access.
- Each case may define explicit sources and trusted conversation turns. Explicit-source misses must stop as `unresolved_explicit_source`; they must never reopen scope and return an unrelated document.

### 12.2 Metrics and release gates

- Reports include source-resolution accuracy, requested-channel attempt rate, document Recall@5/10/20, heading Recall@20, MRR, unique-document binary nDCG@10, locator validity, zero-evidence false negatives/positives, latency, round count, reranker version/status/latency/fallback, per-case stop reason, and legacy/v2 improvements or regressions.
- A passing report requires every case gate to pass, all expected documents to appear by Recall@20, all requested kinds to be attempted, all selected locators to resolve inside the repository, and zero false positive/negative zero-evidence decisions.
- Reports and fixtures must contain no credentials, absolute repository paths, raw planner payloads, provider responses, or chain-of-thought. A small regression suite is a release gate, not a population-level factual-accuracy claim.
- The reviewed baseline is tracked in `evals/rag-evaluation-baseline.md`. Thresholds may be tightened after review; they must not be silently lowered to make a regression pass.

### 12.3 Retrieval invariants found by evaluation

- Source-constrained content queries remove the already-resolved source title and history-only entity suffixes before section ranking; title tokens must not drown the requested body concept.
- Section and semantic hits for the same document, heading, and source span share one stable identity and cannot consume multiple evidence slots.
- Reranker failure is fail-soft: `HybridResearchReranker` returns deterministic ranking, and diagnostics record a degraded `reranker` attempt rather than silently reporting semantic success.
- Reranking may use contract concepts, aliases, related problems, and facet queries for bilingual relevance. Production code must not contain fixture-question or domain-keyword patches.
- Adding an extra Rust binary requires `default-run = "app"` so `cargo run`/Tauri packaging cannot accidentally select the evaluation CLI as the desktop executable.

### 12.4 Required verification

- Unit tests cover strict fixture parsing, unresolved-source fail-closed behavior, duplicate identity, reranker degradation, real-repository evaluation, migration preservation, and report redaction.
- Release verification includes Rust formatting/tests/clippy, Python tests and Wiki evaluation, QA frontend tests, frontend production build, P3 verification, `npm run eval:rag`, `cargo build --release`, `npm run tauri build`, strict GUI smoke, and strict installer install/launch/uninstall smoke.
- GUI research-trail smoke waits for both library FTS completion and the cold-start retrieval result/error. Its cold-start budget must cover first semantic initialization; an explicit backend error still fails immediately.
