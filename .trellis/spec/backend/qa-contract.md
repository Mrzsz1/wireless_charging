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
7. Provider deltas are internal generation-buffer input only. The production `AnswerStreamEvent` contract has no raw `token` variant: the UI receives progress boundaries, then exactly one `completed` event carrying the persisted Final Answer, or `failed` / `cancelled` without draft text. Zero-evidence and offline paths follow the same final-only boundary.

## 4. Retrieval and History

- Existing sessions expose two repository-scoped channels. Trusted History loads supported `completed` exchanges plus the persisted trusted projection of `mixed` exchanges and groups them by request ID; there is no product-level turn, message, or character cap. Reference History contains only prior user messages whose paired assistant reached `completed | mixed | unverified`; it excludes assistant content entirely. Orphans and `failed | cancelled | in_progress` exchanges are excluded from both channels.
- `qa/context.rs` allocates the 8,192–1,000,000-token model window across research contract, current query, evidence, output reserve, safety margin, exact recent exchanges, deterministic structured session memory, and serialization overhead. The UI exposes no recent-turn limit.
- Evidence is fitted first to a bounded share and unused capacity returns to history. Complete exchanges are selected contiguously newest-first until the token budget is reached; an oversized newest exchange is retained so the final total-input gate fails closed rather than silently substituting older short history. Remaining exchanges become `qa-session-memory-v1` JSON entries with source message IDs, stripped historical citations, compact user questions, and trusted answer summaries. The final serialized prompt is re-estimated before provider execution. `ContextPlan` records exact/compacted IDs, fingerprint, token breakdown, free tokens, and truncation state.
- `unverified` assistant content never enters Trusted History, Reference History, research state, query fact expansion, the generator prompt, or the next turn's trusted facts. Its paired user question may enter Reference History solely for deterministic pronoun/entity/ellipsis resolution. `failed`, `cancelled`, and `in_progress` exchanges enter neither channel.
- A `RetrievalQuery` owns `originalQuestion`, `resolvedQuestion`, history-resolved `entities`, trusted/canonical `usedHistoryMessageIds`, reference-only `usedReferenceHistoryMessageIds`, structured `researchIntent`, `executionMode`, routing reason, resolver/router status and latency, the RetrievalContract version/status, planned/covered facet IDs, requested/attempted kinds, and unresolved source gaps. The run manifest persists only bounded IDs, kind/status values, aggregate latencies, fallback reasons, and fingerprints; it never persists raw round queries, understanding payloads, or planner payloads.
- Retrieval planning uses `qa-retrieval-contract-v2`, not a fixed answer-profile classifier. The contract expresses source scope, explicit sources, concepts, aliases, related problems, open facets, requested/must-attempt kinds, and bounded query/candidate/round budgets. `answerProfile` and `minimumEvidence` are forbidden Provider fields. Final answer formatting is independently selected by `answerFormat`; production uses natural Markdown while the legacy structured renderer remains read-compatible.
- `qa/understanding.rs` owns `ConversationResolver`, `ResolvedQuestion`, `ResearchQuery`, `ResearchIntent`, `IntentRouter`, and `ExecutionMode`. For a contextual reference with trusted history, any provider advertising the understanding capability may make one bounded Provider-native `qa-understanding-v2` call before retrieval; self-contained turns skip that call. Reference History is `serde(skip)` provider input and is consumed only by deterministic resolution, so a reference recovered solely from an unverified exchange cannot expose that prior user text to the Provider. The v2 input includes a compact current-state summary and the closed output may add only an ordered `ResearchStatePatch`, never a replacement final state. Invalid, timed-out, unavailable, budget-rejected, or schema-rejected output falls back to deterministic resolution/routing and a validated deterministic patch with explicit telemetry. Offline evidence mode uses the deterministic path.
- Deterministic rewrite runs only for explicit references or continuations such as 它们/二者/上述/第二个/继续/they/both. Broad markers `其中` and `分别` do not trigger rewrite. A self-contained question naming at least two explicit model/page entities never imports history entities. Rewrite prefers Trusted History, then may add bounded user-only Reference History entities or a selected enumerated item; it never appends assistant text or old `[E#]` values to FTS. `usedHistoryMessageIds` contains only trusted/canonical messages that contributed a resolved entity, while `usedReferenceHistoryMessageIds` independently records contributing user-only references.
- `ResearchIntent` is domain-neutral and classifies direct fact, literature search, comparison, origin/derivation, method improvement, solution search, problem modeling, novelty, follow-up, or exploratory research. `ExecutionMode` is `direct | research | exploratory`; it is an auditable routing decision and does not bypass RetrievalContract limits or evidence requirements.
- Deterministic routing emits `routingConfidence=high|medium|low`. Contextual references, low-confidence self-contained questions, and recognized open-problem modes escalate to a capable understanding provider; medium/high-confidence direct facts stay deterministic. `resolverEscalated` and confidence are persisted separately from provider success/fallback.
- `MethodImprovement`, `SolutionSearch`, `ProblemModeling`, and `ExploratoryResearch` retain distinct answer profiles through generation. Their natural-answer contracts require profile-specific named information elements, and completeness validates those elements without imposing one universal heading layout.
- History resolves references only. Only the current evidence package can support current `[E#]` citations.
- Retrieval is a bounded agent loop of at most three rounds. The Provider-native RetrievalContract supplies bounded bilingual facet queries before execution; the controller releases those queries only when the first-round coverage snapshot exposes a relevant gap. A final round may expand from recalled index titles/identifiers. It stops on all requested surfaces attempted, unresolved explicit source, no novel candidates, query budget exhaustion, cancellation, or the round cap. Diagnostics expose only round count, aggregate channel timing/counts/status, candidate gains, stop reason, and SHA-256 round fingerprints; never expose the question, terms, snippets, paths, or secrets.
- The semantic channel embeds the resolved query plus bounded Wiki, primary-paper-section, and core-book text with the quantized multilingual Paraphrase MiniLM L12 v2 model under local ONNX Runtime. Documents and questions are not sent to an embedding API. Vectors are persisted by hashed repository identity, knowledge snapshot, model, and document identity; snapshot changes invalidate reuse. Model/runtime acquisition, initialization, or inference failure degrades to an empty semantic channel and must not fail the answer request. Initialization failure uses a bounded retry delay and never disables semantic retrieval for the remainder of the process.
- Semantic model storage is a machine-global setting, independent from repository-scoped `LunaSettings`, and remains available before a repository is selected. The default is `%LOCALAPPDATA%/LunaWiki/fastembed`; a validated absolute writable custom directory may override it. Switching directories waits for the current semantic operation, then clears the in-memory model, corpus, and retry deadline before publishing the new path.
- Deployment inspection is strictly offline. It reports `missing | partial | invalid | ready | error` only after checking the ONNX runtime, one complete current-model snapshot, all tokenizer files, current-model `.part` files, and—when static files are complete—a 384-dimensional finite-value probe. Ordinary QA retrieval never downloads a missing model. Only the explicit download/repair command may access the network; invalid resources are quarantined rather than deleted.
- Explicit semantic download/repair emits `SemanticDownloadProgress` through a Tauri Channel. Runtime archive reads and Hugging Face model/tokenizer downloads report real accumulated bytes, remote total bytes when known, percentage, average bytes/second, phase, safe file label, and `downloading | verifying | complete | skipped | failed`; progress must never be synthesized by a timer. Cached files emit `skipped`, and inference initialization plus the 384-dimensional probe emit separate `verifying` events. Events never include URLs, absolute cache-file paths, credentials, or remote response bodies.
- Cache copy is explicit, rejects equal or nested source/target directories, uses temporary destination files plus rename, skips lock files, and preserves the source directory as a rollback copy. No cache-switch, repair, or inspection path automatically deletes an old model directory.
- RetrievalContract has hard bounds: at most twelve concepts/aliases/related problems, at most eight unique facets, four queries per facet, twenty queries total, three rounds, and only `wiki | paper | book` requested/preferred kinds. The backend revalidates these bounds after Provider schema enforcement. Invalid, timed-out, unavailable, or rejected planner output degrades to an open one-round contract that retains the complete Unicode question and extracted explicit source names; raw planner output and chain-of-thought are never persisted.
- The v2 query builder is domain-neutral. It combines the complete current question, Provider concepts/aliases/related problems, resolved source aliases, and bounded facet queries. When caps apply, it preserves both the beginning and the tail of the term stream so a late core concept cannot disappear through prefix-only `.take()` or truncation. Production code must not map domain phrases, complete questions, or fixture-specific source IDs to expansion terms.
- Title/alias, ContentBlock FTS/BM25, metadata-filtered FTS, dense vectors, and graph-mapped ContentBlocks are independent channels. Channel-native scores are never added directly across scales: candidates fuse by stable block identity through reciprocal-rank fusion, then `HybridResearchReranker` applies one cached local FastEmbed `TextRerank` session over at most 80 bounded candidate texts. Batch size is explicit and measurable; the production default avoids multiple Rayon batches that each request all ONNX intra-op threads. Deterministic/base and cross-encoder scores are finite-value normalized before weighted fusion, with a bounded top-score bonus plus existing explicit-source/reference/graph/fallback adjustments. A uniform document-repeat penalty prevents one document's many blocks from erasing source diversity; it is keyed by document identity and kind, never by fixture/query IDs. Graph nodes must map back to an active ContentBlock before they can enter the evidence set. A cross-encoder, cosine, or reranker score is retrieval relevance, not factual confidence.
- Query-time reranking never downloads a model. The cross-encoder loads only predeployed files from `QA_RERANKER_MODEL_DIR` or the configured semantic cache's `reranker-bge-base` directory. Missing/corrupt cross-encoder state falls back to `EmbeddingRescorer`; missing/invalid embeddings then fall back to `DeterministicResearchReranker`. Cancellation never becomes fallback. The actual provider name (`cross-encoder-research-v1`, `embedding-rescorer-v2`, or `deterministic-research-v2`), status, aggregate latency, candidate count, batch size/count, model max length, fallback boolean, and stable fallback reason are recorded in retrieval diagnostics and `QaRunManifest`.
- Cross-Encoder production lifecycle is explicit. Offline inspection reports `missing | partial | invalid | ready | error` for `BAAI/bge-reranker-base`, model version, ONNX Runtime, model/tokenizer files, and a real rerank health probe. The only network-capable path is the user-triggered provision/repair command. It pins revision `2cfc18c9415c912f9d8155881c133215df768a70` plus the exact size and SHA-256 of the ONNX and four tokenizer/config artifacts; each artifact streams to a same-directory `.part`, reports real accumulated/total/session-speed progress, verifies size and SHA-256, flushes/syncs, then atomically renames. Range-capable servers resume partial files; a full response safely restarts the part. Cancellation is request-scoped, checked at stream-chunk boundaries, returns `RERANKER_DEPLOYMENT_CANCELLED`, and leaves a resumable partial without touching a valid ready artifact. Concurrent repair is rejected as busy, while repeated repair of a valid artifact performs offline integrity/health checks without network transfer. Both direct artifact directories and Hugging Face snapshot layouts resolve to the same user-defined runtime loader. A regression fixture scorer or fallback RAG pass never satisfies the production model gate.
- Post-ranking retention is monotonic: adding a required channel, method, or Wiki/paper pair must not evict the last already-satisfied required channel or the last retained method. Pair repair protects both sides of earlier selected pairs and never inserts an orphan Wiki page after its paper was displaced.
- `EvidenceManager` owns final candidate deduplication, tie-break authority weighting, MMR-style document/type diversity, a maximum of two evidence items per primary-paper document, evidence token estimation for the authoritative `ContextPlan` budget, and parent-section expansion only from the selected semantic block's exact indexed `parent_block_id`. Parent context must be active and belong to the same document; missing, inactive, or cross-document parents fail closed without heuristic sibling substitution. `QaRunManifest` records its version and aggregate input/dedup/selection/document/expansion/token counts.
- Claim verification is a separate post-generation stage. A legal evidence appendix or Evidence ID proves provenance syntax only. `atomic-claim-extractor-v1` first applies Markdown/citation-aware sentence segmentation and then guarded clause splitting so suggestion+reason, fact+suggestion, causal, parallel, and contrast constructions cannot smuggle a factual proposition under a suggestion type. Each `AtomicClaim` has one primary proposition, its own local Evidence IDs, independent `ClaimType`, initial `verificationStatus=unverified`, and optional confidence. Citation suffixes stay with the final local clause; an earlier clause receives support only from a citation inside that clause. `VerificationProvider` then assigns supported/partial/contradicted/not-verifiable status, and `AnswerRepair` removes contradicted or unverifiable factual prose and lowers certainty for partial support. Provider failure records `verificationStatus=unavailable`; it never sets `entailmentChecked=true` or fabricates a verified result. `QaRunManifest` persists extractor/verifier versions, fallback, status, status counts, repair count, and per-claim audit data.
- `semantic-claim-verifier-v2` uses one bounded structured-output call for all eligible mapped claims, not one LLM call per claim. Codex and Compatible API share the same closed schema through provider capability `semantic_verification`; offline/unknown providers do not advertise it. The call runs after generation and before the repository write lock, reserves and settles stage `semantic_verifier` through the request's `LlmBudgetGuard`, and returns only `entailed | contradicted | unknown` plus bounded confidence/reason. Its decision order is strict: first require support for every material claim part; if that fails, use `contradicted` only for an explicit opposite, mutually exclusive fact, or genuine exclusion; otherwise use `unknown`. Lack of support, bounded-to-universal scope expansion, correlation-to-causation, extrapolation, and unsupported guarantees are not contradictions unless the evidence explicitly rules them out. Missing, unknown, or graph-only evidence is rejected before Provider execution. Timeout, invalid JSON, rate/provider failure, or budget rejection records `semanticVerificationStatus=unavailable`, keeps `semanticVerificationChecked=false`, and falls back per claim to the deterministic verifier; cancellation aborts the request rather than becoming fallback. Semantic contradiction wins, semantic entailment cannot override a deterministic negation contradiction, and semantic unknown maps only to `NotVerifiable` or `PartiallySupported`, never `Contradicted`. `entailmentChecked=true` is legal only when the complete validated semantic result set covers every eligible claim.
- Exploratory and research queries pass through `ProblemParser` and `MethodMatcher`. The representation preserves domain, objectives, constraints, assumptions, decision variables, and related problem types. Rule-matched methods are explicit hypotheses (`source=hypothesis`, `corroborated=false`) and never seed first-round retrieval terms. Neutral retrieval uses problem classes and objectives; then recalled `page_type=method` evidence supplies discovered methods for applicability analysis. `QaRunManifest` keeps hypotheses, discovered methods, independently corroborated hypotheses, and method-evidence provenance in separate fields.
- `ResearchSessionState` v2 is reconstructed deterministically from trusted user turns by applying ordered per-object patches. It tracks active objectives, constraints, assumptions and methods separately from `excludedMethods`, typed bounded parameters, paper aliases, hypotheses, open questions, source message IDs, last patch ID, and a monotonic revision. `add/remove/keep/replace/set/set_all/clear` are field-scoped; a low-confidence destructive operation or replacement with a missing source fails closed. An unnamed numeric parameter follow-up inherits the existing key only when the clause is a strict value-only reference (for example `改成 2`) and exactly one parameter candidate exists. A new unrecognized noun phrase (for example `充电功率改成 50W`) and a multi-parameter ambiguity leave existing parameters unchanged; a validated Provider patch may still add it as `custom:<normalized-key>`. The current-turn patch is applied before `ResearchQueryContext` is built, and both deterministic fallback planning and Provider planning consume that post-patch projection. Its serialized size is included in the context budget; `QaRunManifest` records state/patch/query-context and parameter-resolution counts without raw planner payloads.
- `AdaptiveRoutingPolicy` v2 maps `DirectQA`, `ResearchQA`, and `ExploratoryResearch` to bounded retrieval rounds, queries, candidates, LLM-call budgets, and token ceilings. The total call budgets are respectively 3, 4, and 5; all three policies reserve exactly one call for Semantic Verification. Token ceilings remain respectively 8,000, 18,000, and 32,000. Direct executes at most one retrieval round and never invokes the query planner, even when a callback is available. Coverage can stop any path early, while exploratory mode is capped at three rounds.
- A request-scoped `LlmBudgetGuard` gates understanding/resolver, query planner, generator, and semantic verifier before provider execution. `SEMANTIC_VERIFIER_STAGE` is the single canonical stage identifier. For a non-Semantic candidate, admission requires `nextCalls + (semanticVerifierCallReserve - semanticVerifierCallsUsed) <= llmCallBudget`; therefore an optional stage or retry cannot consume the final verification slot. A Semantic candidate consumes the reserve and is rejected when the configured reserve has already been used. Rejected attempts mutate neither counter.
- Token admission remains exactly `tokenCostUsed + tokenCostInFlight + newReservation <= tokenCostCeiling`; the Semantic reserve never bypasses it, and historical cumulative reservations never participate. `tokenCostUsed` is actual cost committed by completed reservations, while `tokenCostInFlight` is the maximum cost held by active reservations. Total calls and successful Semantic reservations are non-refundable after admission: settlement, explicit release, Drop, Provider failure, and cancellation cleanup release only in-flight token capacity. Rejection stays stable as `LLM_BUDGET_EXCEEDED: <stage>:<call_budget|token_budget>` and remains fail-soft only where the stage already has a deterministic fallback.
- `LlmBudgetGuard::reserve(stage, ceiling)` returns a unique non-clone `LlmReservation`. Consuming `settle(actual)` removes that exact active ID, releases its in-flight ceiling, and adds actual cost to used; consuming `release()` removes it without adding used. An unclosed handle releases in `Drop`, so pre-provider errors, early `?`, cancellation unwind, and task panic cannot leak in-flight capacity. A closed ID rejects a second internal close without changing counters. `reconfigure()` replaces only the policy and preserves calls, Semantic calls used, token accounting, active reservations, historical reserved totals, stages, and rejections.
- `qa-run-v23` records `routingPolicyVersion=adaptive-routing-v2`, `routingLlmCallBudget`, `routingTokenCostCeiling`, `routingTokenCostUsed`, `routingTokenCostInFlight`, historical compatibility field `routingTokenCostReserved`, explicit alias `routingTokenCostReservedTotal`, `routingLlmCallsUsed`, stages, rejection reasons, Evidence Availability aggregates, and `zeroEvidenceAudit`. No manifest field is added for the internal Semantic reserve counter. The two reserved-token fields are audit-only historical cumulative maxima and remain equal; old manifests default newer telemetry fields to zero. The QA sidebar exposes used/in-flight/ceiling/history separately.
- The broad Research QA corpus is `evals/research_questions_v1.json`: 360 unique questions, sealed by canonical SHA-256 and split before evaluation into 160 development, 120 regression, and 80 heldout questions across 12 domains and 10 intents. Heldout entries contain no expected answers/evidence and must not drive rules. Production accuracy claims remain gated by the stricter independently curated and double-reviewed `heldout_questions.json` workflow.
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

- Research and Exploratory Providers return ordinary Markdown with an internal audit marker on every evidence-backed factual sentence: explicit current-turn `[E#]` tokens identify the evidence selected for that sentence. A natural-v2 Direct request with non-empty evidence is the narrow exception: it uses internal `qa-direct-grounded-answer-v1` JSON with one to three atomic `{ text, evidenceIds }` claims (or `claims=[]` plus `insufficientEvidence=true`). The schema is bound to current Evidence IDs, and the backend independently rejects unknown/empty IDs, Graph-only mappings, non-atomic text, and more than three claims before deterministically rendering citation-bearing natural text. The user never sees this JSON. Research/Exploratory must not use this Direct schema.
- Claim verification runs on the citation-preserving canonical answer before presentation rendering. Only after the per-claim audit and repair does the backend strip `[E#]` tokens for natural-answer display, remove any Provider-authored `## 参考证据`, block unsafe Markdown targets, and redact visible absolute Windows/UNC paths.
- The backend deterministically appends one `## 参考证据` section from selected, non-Graph evidence that owns a valid `SourceLocator`. Link labels are compact source kind + canonical title + useful heading. The link target is the opaque current `evidence:E#` identity; arbitrary Provider paths never become navigation targets.
- `appendixIntegrity` is true only when the appendix is backed by persisted current evidence locators; `appendixEvidenceIds` must be the emitted ordered IDs. Appendix presence never marks a claim supported. Deterministic lexical checking sets `heuristicVerificationChecked=true` while semantic/model entailment remains `entailmentChecked=false`.
- `ClaimType` (`knowledge_fact | general_knowledge | reasoned_inference | research_suggestion`) and `VerificationStatus` (`supported | partially_supported | contradicted | not_verifiable | not_applicable`) are independent fields. Cue words can classify a claim but can never create a supported verification status.
- A factual, general-knowledge, or inferred claim without explicit current evidence IDs is `not_verifiable`; it is never implicitly bound to the whole evidence bundle. Unknown IDs and Graph-only mappings also fail closed. Research suggestions are `not_applicable`, not “supported”.
- Locator resolution is repository-boundary checked and degrades in order: exact block, heading path, saved line range, document. Every fallback returns `matchedBy` and a user-visible `degradedReason`; it never guesses another document.
- Zero evidence produces the backend-owned unverified notice, no appendix, and a paired `unverified` exchange. `unverified`, `failed`, and `cancelled` content never enters trusted history. Trusted history is built only from ordered `finalGroundingAudit.claims` whose status is `supported`; citation tokens are projected away, duplicate visible facts are removed without reordering, and research suggestions, non-supported claims, system notices, model supplements, and the backend evidence appendix never enter the next prompt. An invalid audit or zero Final Supported claims produces empty trusted context.
- Production defaults to v2. `LUNAWIKI_RAG_ANSWER_V2=false` (also `RAG_ANSWER_V2`/`rag_answer_v2`, values `0|false|off|no`) restores legacy generation without deleting v2 messages or the structured parser.
- Direct structured parsing emits `qa_direct_answer_parse_started`, then exactly one completed/failed stage event containing only request/mode/provider/model and aggregate evidence/claim counts or a sanitized error code; raw Provider JSON and claim text are never logged.

### 4. Validation & Error Matrix

| Condition | Result |
|---|---|
| Evidence exists, every factual claim has a valid explicit mapping and passes the heuristic gate, and at least one selected non-Graph item has a valid locator | backend appendix, paired `completed` exchange; `appendixIntegrity=true`; claim audit persisted |
| Appendix exists but a factual claim has no explicit ID, an unknown ID, Graph-only support, contradiction, or insufficient alignment | fail closed with `groundingStatus=invalid`; appendix presence does not override the claim result |
| Research/Exploratory Provider emits `[E99]`, its own appendix, `file:`/app-protocol target, or visible absolute path | sanitize/redact it; never register it as source navigation |
| Direct structured output has malformed/unknown fields, unknown or empty `evidenceIds`, Graph-only support, a non-atomic claim, or more than three claims | reject before semantic verification and persistence with `DIRECT_GROUNDED_ANSWER_INVALID`; never expose the raw JSON |
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
- Bad: require headings such as `结论`/`模型与方法`, apply the Direct claim-count/schema contract to Research or Exploratory, trust Direct Provider `evidenceIds` without backend validation, or parse a local path from answer prose to open a file.

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
Provider Markdown + internal [E#] mappings -> per-claim heuristic audit/repair
                  -> presentation sanitization -> backend selected-evidence appendix
                  -> persisted locator + claim audit -> repository-bound resolver -> Markdown focus
```

## 5.1 Prompt and Run Manifest

- Codex and compatible API share one provider-neutral `PromptEnvelope` with six ordered layers: `research_contract`, `session_memory`, `recent_exchanges`, `current_query`, `evidence_bundle`, and `answer_contract`.
- History, current query, and evidence are JSON data. `<`, `>`, and `&` are escaped so embedded content cannot close an envelope layer. Provider-specific code may wrap the envelope but must not define a divergent factual contract.
- Natural v2 Research/Exploratory answers use question-appropriate prose and headings chosen by the Provider. Their answer contract asks for a direct answer, explicit current-turn `[E#]` mapping on each library-backed factual sentence, evidence-bounded model/method discussion, and explicit uncertainty where needed, but has no canonical section array or minimum claim count. Evidence-backed Direct uses the closed internal Direct schema and is converted to the same natural visible format before audit/rendering. The backend, not the Provider, adds user-facing source links.
- Natural v2 grounding drafting is atomic and evidence-minimal: a short sentence expresses one verifiable fact, its adjacent `[E#]` must support the complete meaning, direct factual questions prefer one to three minimally sufficient facts, and wording retains the evidence snippet's subject, predicate, terminology, qualifiers, and—when needed—short original-language phrases. The Provider may not expand local to global, correlation to causation, average to worst-case guarantee, simulation to real-world guarantee, parameter-specific to universal, one method to unique/optimal, or paper proposal to industrial validation. Every numeric value must occur in the cited evidence. Unsupported boundaries use the fixed non-factual sentence `当前证据不足以核验该陈述`; general knowledge belongs only in the uncited model-supplement region, and research suggestions must be visibly phrased as suggestions.
- Every assistant message stores schema/prompt/retriever/context versions, provider, requested/resolved model, temperature where applicable, output/context limits, prompt SHA-256, index snapshot SHA-256, recent/compacted/coreference-resolved history IDs, resolver/router/planner/reranker status and aggregate latency, structured research intent and execution mode, fallback reasons, evidence checksums, context budget, repair record, completeness result, aggregate verification counts, and serialized per-claim `claimVerifications` in `run_manifest`.
- Evidence-backed Direct runs record `structuredOutputMode=direct-grounded-json`; Research/Exploratory natural runs record `structuredOutputMode=natural-markdown`. Both retain `answerFormat=natural-markdown-v2` because the persisted/user-visible result is natural text. Planner calls may still use Provider-native JSON Schema because retrieval planning and final-answer rendering are independent contracts.
- Answers rejected by citation or completeness gates persist the rejected answer, evidence, validation result, and run manifest on the failed assistant message; pre-context retrieval failures retain an empty legacy manifest because no prompt/evidence snapshot exists.
- QA column migrations are idempotent column-existence checks. This module must not overwrite SQLite's global `PRAGMA user_version`, because compile-center and other subsystems share the same repository database.
- Requested and resolved model are distinct. If Codex omits its actual default model, the resolved value is `provider-default-unreported`; it must not be invented.
- Old messages hydrate with `runManifest=None`. The manifest excludes endpoint, API key, token, cookie, question/answer text, raw provider payload, and chain-of-thought.

Claim splitting and same-claim citation attachment are part of the `natural-markdown-v2` fail-closed audit path. They run before display tokens are removed; presentation rendering must never destroy the mapping before verification.

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
- Provider planning is capability-based rather than provider-name-based. `ProviderDescriptor` exposes `understanding`, `queryPlanning`, `structuredOutput`, and `naturalGeneration`; Codex subscription and Compatible API advertise the same planning capabilities, while offline evidence mode advertises none.
- Both capable providers use the same `PlanningProvider::complete_structured` boundary for two possible short-lived calls: a pre-retrieval understanding call receives the bounded current question, compact current-state summary and trusted recent history, while the RetrievalContract call after baseline retrieval receives the resolved question, bounded post-patch `researchContext`, and bounded candidate summaries. Codex uses `codex exec --output-schema`; Compatible API uses a non-streaming `response_format=json_schema` request against the configured endpoint. Both emit no UI answer tokens, validate the same closed schemas, consume the request budget, and fail soft to deterministic behavior.
- Compatible API planning reads its credential only from the configured environment variable at the request boundary. Missing credentials, unsupported structured output, malformed JSON, timeouts, and provider errors produce stable secret-free fallback telemetry. The final natural-answer call remains independently streamed.
- When the answer-v2 rollback flag is disabled, legacy evidence-backed Codex runs may still generate the old intent/evidence schema inside the isolated temporary workspace. That file is removed with the workspace. Compatible API final answers use the same natural Markdown contract in v2.
- Settings-page status refresh bypasses the TTL and refreshes the cache.
- Offline and compatible API requests never probe Codex.
- Codex token/cookie and API key values are never returned, persisted, or added to ordinary logs/errors.
- Compatible API SSE accepts `[DONE]` or `finish_reason=stop` as a complete stream. If the terminal `stop` frame also contains a final content delta, that token is appended to the internal answer buffer before termination. The production UI adapter never forwards the callback as an answer event. `length`, other non-empty finish reasons, malformed JSON, and EOF before a legal terminator are failed exchanges; partial text is neither displayed nor persisted as `completed`.

## 9. Answer Rendering and Source Navigation Contract

- Persisted natural v2 content is Markdown source text plus the backend-owned appendix. The desktop renders Markdown, GFM tables, fenced code, and KaTeX through a lazy-loaded renderer.
- Raw HTML is not enabled. Remote images are replaced by text placeholders. Only `http(s)` links become external anchors; `evidence:` links are resolved against the message's persisted evidence collection.
- Appendix labels show short user-facing source descriptions rather than full filenames or absolute paths. Detailed snippets and retrieval reasons remain in the evidence panel and audit bundle.
- Clicking a current evidence link passes its persisted `SourceLocator` to `read_source_locator`. The returned Markdown is shown in an internal read-only source view and focused using `headingPath`/`lineStart`; the UI shows fallback/degradation state rather than silently presenting a wrong location.
- Legacy inline `[E#]` projection remains Markdown-aware: it skips fenced/inline code, math, escapes, and labels already inside links. Unknown IDs remain visibly invalid.
- Natural supported messages summarize per-claim explicit mapping and heuristic verification separately from appendix integrity. The UI must not describe retrieval relevance or appendix presence as factual correctness, and must state that model/semantic entailment was not executed.
- The evidence sidebar shows context budget, snapshot, prompt/answer versions, retrieval rounds/stop reason, and current evidence. Assistant actions can copy an audit bundle containing question, answer, evidence, and manifest.
- `offline-evidence` is presented as “证据浏览模式”; it is evidence navigation, not a generated research answer.
- Legacy structured parsing errors keep `STRUCTURED_ANSWER_VALIDATION_FAILED`. Natural body/appendix validation uses `ANSWER_VALIDATION_FAILED`; new natural runs must never fail because a canonical section title, role, or claim count is absent.

## 10. Tests Required

- Request registration covers early cancel and duplicate active IDs.
- Follow-up retrieval covers CCSP/GAIN reference resolution and excludes old citations.
- Intent regression covers all three canonical literals and asserts that `solve`/`novelty` evidence retains a recalled method candidate.
- History is repository-scoped, completed-only, ordered, and bounded.
- Natural-answer tests cover explicit current-ID mapping, no-ID fail-closed behavior, unknown Provider IDs, Graph-only mappings, type/status separation, heuristic-versus-entailment telemetry, per-claim manifest round-trip, unsafe links/paths, appendix integrity, locator-less evidence, and zero-evidence unverified answers.
- A natural answer may use the exact optional `## 模型补充（可能不准确）` section with the fixed notice. Such answers are `mixed` and remain fully visible for display/audit, but trusted context is projected only from Final Supported claims; the supplement, research suggestions, system notices, and backend appendix are never candidates for trusted history. Model supplementation and evidence-link prose therefore never become later retrieval entities or trusted prompt facts. Citation entailment remains outside the contract and `entailmentChecked` stays false.
- Codex subscription execution treats the repository timeout as an idle deadline refreshed only by valid JSONL stdout events, plus a separate bounded hard deadline. It returns stable `CODEX_IDLE_TIMEOUT` and `CODEX_TOTAL_TIMEOUT` errors; cancellation still terminates the process tree and partial output is never persisted as completed. The compatible API keeps its HTTP timeout semantics.
- Codex Provider tests assert `--output-schema` only for planner/legacy calls, schema placement/cleanup, natural final-answer execution without a schema, and a distinct `CODEX_OUTPUT_SCHEMA_REJECTED` error when a schema-enabled call is explicitly rejected.
- Codex model/effort metadata is projected from the local top-level config and list-visible model cache. The composer sends an explicit per-request snapshot; backend resolution validates the effort against that model's reported capabilities before invoking Codex. Metadata DTOs must never expose authentication material.
- Compatible API parser tests cover token deltas, `[DONE]`, `stop`, `length`, abnormal finish reasons, malformed JSON, and EOF before termination without contacting a provider.
- The `stop` regression must include a terminal frame that carries both content and `finish_reason=stop` and assert that the final token is not lost.
- Budget-ledger regressions cover reusable unused reservation (8k ceiling, 4k reserved/1k actual, then 6k generator), true used+in-flight overage, concurrent anti-oversell, cumulative calls after settle/release, Direct→Research reconfigure preservation, provider-error/Drop release, duplicate-close no-op protection, fixed 8k/18k/32k ceilings, and a synthetic Exploratory understanding→planner→generator→verifier sequence. These tests use no held-out content or model calls.
- First-turn and existing-session failure tests assert paired messages and exact retry questions.
- Graph tests cover node-, relation-, and neighbor-only hits, source filtering, page ID resolution, and cache invalidation.
- Semantic retrieval tests cover cosine ordering, similarity-floor filtering, exact vector serialization, and optional-table degradation without downloading a model during the ordinary unit-test suite.
- Semantic deployment tests cover missing/partial classification without network access, complete snapshot component checks, cache-switch state reset, non-destructive cache copy, and global settings round-trip without repository SQLite.
- Semantic download progress tests cover exact accumulated-byte percentages, completion, cached-file detection without network access, Channel wiring, and frontend rendering of file/bytes/percentage/speed beside the repair action.
- Understanding tests cover the closed Provider schema and patch schema, unknown history IDs, deterministic provider fallback, 50 frozen follow-up cases, and ordinal references. State tests cover ordered mixed operations, self-correction, parameter overwrite, strict value-only parameter inheritance, unknown-name and multi-candidate rejection, custom-key normalization, set-all/clear, missing-source replacement, and ambiguous destructive fail-closed behavior. Planner integration asserts mutation → context → planner order. `conversation_state_v2_cases.json` freezes 14 core cases, 5 parameter-safety cases, and 20/50/100-turn cases and gates full state/field exact match, unexpected state, destructive errors, parameter corruption count, and query-context recall. RetrievalContract tests cover closed Provider schema objects, open facet IDs, unknown-field rejection, value/array bounds, fail-soft planning, source scope, and facet-only expansion after a baseline miss.
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
- `qa-rag-evaluation-cases-v1` is a closed fixture schema. Unknown fields, duplicate case IDs, invalid kinds/scopes, malformed conversations, and contradictory zero-evidence expectations fail before retrieval. `expectedDocuments=[]` is legal only when `zeroEvidenceExpected=true`; an unlabeled empty expected set is invalid rather than an implicit perfect ranking score.
- Evaluation builds an isolated in-memory SQLite index from the real Markdown repository. It does not call an answer Provider, download a model, mutate the repository database, or require network access.
- Each case may define explicit sources and trusted conversation turns. Explicit-source misses must stop as `unresolved_explicit_source`; they must never reopen scope and return an unrelated document.

### 12.2 Metrics and release gates

- `qa-rag-evaluation-report-v4` fixes production relevance to Canonical Research Work. The single identity helper maps `wiki:sources/<id>` and `paper:sources/<id>` to `source:<id>` and leaves every other ID unchanged. One deduplicated work relevance view supplies Work Recall@5/10/20, Work MRR, and Work binary nDCG@10; a separate exact-source view supplies diagnostic surface-source metrics, and passage MRR is diagnostic-only.
- Zero-evidence cases emit `null` per-case ranking metrics and are excluded from every ranking aggregate denominator. Reports expose `rankingEligibleCaseCount`, `zeroEvidenceCaseCount`, zero-evidence TP/FP/FN/TN plus precision/recall/specificity (undefined ratios are `null`), normalized-case-array SHA-256, and case count. Markdown is rendered from the JSON report object and prints `N/A` for null metrics.
- For v4 compatibility only, `documentRecallAt5/10/20` alias the matching Work Recall fields, `documentMrr` and `mrr` alias Work MRR, and `ndcgAt10` aliases Work nDCG@10. Release gates read the explicit Work fields; their reviewed numeric thresholds remain unchanged.
- A passing report requires every case gate to pass, all expected documents to appear by Recall@20, all requested kinds to be attempted, all selected locators to resolve inside the repository, and zero false positive/negative zero-evidence decisions.
- Reports and fixtures must contain no credentials, absolute repository paths, raw planner payloads, provider responses, or chain-of-thought. A small regression suite is a release gate, not a population-level factual-accuracy claim.
- The reviewed baseline is tracked in `evals/rag-evaluation-baseline.md`. Thresholds may be tightened after review; they must not be silently lowered to make a regression pass.

### 12.3 Retrieval invariants found by evaluation

- Source-constrained content queries remove the already-resolved source title and history-only entity suffixes before section ranking; title tokens must not drown the requested body concept.
- Section and semantic hits for the same document, heading, and source span share one stable identity and cannot consume multiple evidence slots.
- Reranker failure is fail-soft: `HybridResearchReranker` records whether cross-encoder, embedding, or deterministic ranking actually ran, and diagnostics record a degraded `reranker` attempt rather than silently reporting cross-encoder success.
- Reranking may use contract concepts, aliases, related problems, and facet queries for bilingual relevance. Production code must not contain fixture-question or domain-keyword patches.
- Adding an extra Rust binary requires `default-run = "app"` so `cargo run`/Tauri packaging cannot accidentally select the evaluation CLI as the desktop executable.

### 12.4 Required verification

- Unit tests cover strict fixture parsing, unresolved-source fail-closed behavior, duplicate identity, reranker degradation, real-repository evaluation, migration preservation, and report redaction.
- Zero-evidence ranking regressions must exercise the production `EvaluationAggregate` path rather than only a parallel test metric helper: averaging `Some(0.0)` with an ineligible `None` must remain `Some(0.0)`, all-ineligible input must remain `None`, and the real `qa-rag-evaluation-report-v4` serialization must emit JSON `null` for every ineligible work/exact Recall, MRR, and nDCG field.
- Release verification includes Rust formatting/tests/clippy, Python tests and Wiki evaluation, QA frontend tests, frontend production build, P3 verification, `npm run eval:rag`, `cargo build --release`, `npm run tauri build`, strict GUI smoke, and strict installer install/launch/uninstall smoke.
- GUI research-trail smoke waits for both library FTS completion and the cold-start retrieval result/error. Its cold-start budget must cover first semantic initialization; an explicit backend error still fails immediately.

## 13. Production Evaluation and Release Gate

- Every production evaluation JSON contains one `qa-eval-metadata-v1` envelope: full Git SHA, UTC timestamp, canonical dataset hash, hash-only runtime configuration, provider/model identities, platform, CPU count/type, and memory. It must not contain secrets, endpoints, absolute paths, raw questions/answers, prompts, evidence snippets, or provider responses.
- `evals/heldout_contract.json` is the single canonical frozen-case type contract. New datasets use exactly `direct_factual | literature_search | comparison | origin_derivation | method_improvement | solution_search | problem_modeling | related_problem | counterfactual | novelty`; the public template, Rust runner, Python evaluator and curator workflow must match it without alias remapping.
- `npm run eval:heldout:run -- --dataset <external> --output-dir <external> --repository <root>` is the Phase-1 runner. It accepts only an independently curated frozen dataset with at least 30 unique cases and a matching canonical cases SHA-256, then requires a clean Git worktree and records the exact commit. Dataset SHA + commit + requested runtime config form a non-overwritable run identity; an existing complete or `.part` run fails closed.
- Each held-out case uses empty history and a distinct UUID session marker, then composes the current production understanding/planning/retrieval/generation/semantic-audit functions. The public CLI never installs the synthetic test executor. It saves the final repaired answer, complete `EvidenceItem[]`, unmodified `QaRunManifest`, and claim projection from that manifest only; no second LLM summarizes claims.
- `qa-heldout-run-v3` changes `answerClaims` authority from Draft `claimVerifications` to `finalGroundingAudit.claims`. A current `qa-run-v22` bundle is valid only when Final Audit succeeded, is `supported`, has zero unsupported/unknown evidence, `citationCoverage=1.0`, and `supportedCount == factualClaimCount == answerCompleteness.claimCount == answerClaims.len`. Only Final `supported` claims are exported; every visible span must occur verbatim in the final answer, every structured cited ID must exist in the complete evidence set with at least one non-Graph item, and the complete visible claim sequence must match Final Audit so renderer-added facts fail closed. Draft claims remain in the manifest for audit only. Rust and Python validate the Final Audit/count/provenance/checksum contracts independently; the frozen dataset contract is unchanged.
- `heldout_questions.json` remains pending until an independent curator freezes at least 30 cases selected from the sealed 80 candidates in `research_questions_v1.json#split=heldout`. Freeze validates the candidate-pool seal/count and exact ID, question, and `ResearchIntent`/type equality before sealing the canonical cases array. Every run must contain complete answer claims, EvidenceItems, stable source IDs, and checksums; every claim, expected method family, and expected critical constraint requires two distinct blinded independent reviews, and every disagreement in any channel requires a third distinct adjudicator. Missing, duplicate, mismatched, non-independent, or tampered data fails closed.
- Held-out claim verdicts are `supported | partially_supported | unsupported | contradicted | not_applicable | not_verifiable`; method coverage is `covered | not_covered`, and constraint coverage is `preserved | not_preserved`. `answerClaims.dimension` may classify a claim as `factual | reference | method | constraint` and defaults to `factual` for runner compatibility, but it is diagnostic metadata only. Relevant-method recall uses frozen `acceptableMethodFamilies` as its sole denominator and independent final method coverage as its numerator; critical-constraint preservation analogously uses frozen `criticalConstraints` and independent final constraint coverage. System-emitted claim count or dimension can never shrink either denominator.
- Frozen machine-readable thresholds live in `evals/qa_release_thresholds.json`. `tools/check_qa_release_gate.py` reads only enveloped artifacts, rejects missing/non-finite/invalid values, and emits every gate's actual value, requirement, result, and reason. Thresholds are never inferred from the current run or automatically lowered.
- Core reliability, Grounding, real Cross-Encoder/semantic-provider evidence, and independent held-out evidence can only produce `PASS` or `FAIL`; they never receive a conditional waiver. `CONDITIONAL PASS` is disabled by the frozen policy.
- `tools/collect_qa_release_artifacts.py` writes metrics through same-directory `.part` files and atomic rename into a new run directory. It rejects raw prompt/content/question/answer/credential fields and non-finite values before collection.
- PR CI runs deterministic metadata, held-out, release-gate, Rust, and frontend contracts without downloading a model. The manual/scheduled self-hosted RC workflow requires model/eval paths outside the system drive, runs the real reranker benchmark, and applies the same frozen production gate.
- `QA_PRODUCTION_RELEASE_REPORT.md` must identify commit/build, dataset and runtime hashes, models/providers, machine facts, every metric/gate, fallbacks, limitations, and final decision. Missing independent held-out, real semantic-provider measurement, or a frozen measured performance profile yields an explicit `FAIL`; regression fixtures never substitute for those external production inputs.
- Production retrieval MRR is Work MRR. Duplicate passages collapse before rank calculation, and a Wiki source plus its paired `wiki:sources/<source-id>` / `paper:sources/<source-id>` primary paper collapse to the same research-work identity. Work Recall and Work nDCG use that exact same identity and deduplication view. Passage MRR and exact-source metrics remain diagnostics only. `mrr-diagnostics-latest.json` records first relevant passage/work ranks, stable and canonical IDs, channel membership, RRF/base/cross/final scores, and document-repeat penalties without paths or source text.
- `evals/semantic_verification_real_cases.json` is a sealed 100-case claim/evidence benchmark. `semantic-verifier-eval` must reuse the production closed structured transport and may set `realProviderMeasured=true` only when every case completed through a real Provider. The report records provider/model/config, accuracy, contradiction recall, unknown precision, timeout/invalid/fallback rates, and invalid verified-state count; Provider failure never creates a verified result.
- `evals/semantic_verification_v2_cases.json` is a development-visible 60-case suite with an exact 20/20/20 label distribution and a canonical cases SHA-256. It is never an Independent Production Held-out. `qa-semantic-verifier-report-v2` adds per-class precision/recall, macro F1, a complete 3×3 confusion matrix, category metrics, failed-case IDs, and per-case status/provider/fallback/latency. Because one Provider call verifies a batch, per-case latency is an explicitly labeled deterministic allocation whose sum equals the measured batch totals; it is not an independent-call measurement.
- `tools/qa_production_eval.py` is the single production collector. It executes or imports the real RAG, canonical 50-case conversation, semantic, performance, and reliability reports, binds them to one full Git SHA, and writes `evals/releases/<git-sha>/` (or an explicit non-system-drive output root) atomically. Reusing `*-latest.json` never changes the measured fields or manufactures missing external artifacts.
- `tools/qa_heldout_workflow.py` owns the independent production workflow: 50 empty curator slots, canonical method/constraint IDs, canonical cases SHA-256, freeze seal, blind reviewer export, two distinct independent primary reviews, and third-party adjudication of exactly the disagreements. One sealed dataset/run pair derives `heldout.json`, `grounding.json`, and `open_research.json`; all three must have the same `sourceRun`.
- `evals/qa_target_machine.json` is sealed before performance measurement and names the exact model revision, machine class, warmup/measured counts, minimum per-mode samples, cold-load limit, and Direct/Research/Exploratory P95 SLOs. `reranker-performance-eval` reports cold model load plus warm nearest-rank P50/P95/P99 per mode. Request telemetry separately records `modelLoadMs`, `inputPrepareMs`, `inferenceMs`, candidate/batch/max-length counts, and average input tokens. Warm session reuse must produce zero additional model-load time; stable dedup precedes the ExecutionMode candidate cap and batched rerank, while parent expansion follows rerank.

## 14. Real Answer Generator Development E2E

### 1. Scope / Trigger

- Use this contract when proving that the desktop production QA path reaches a real final-answer Provider, semantic verification, audit, and persistence gate. It is a small Development/Regression/Synthetic diagnostic and never substitutes for Independent Held-out evaluation.

### 2. Signatures

- Shared production core: `prepare_production_qa(connection, root, request, request_id, cancelled) -> PreparedProductionQa` followed by `run_production_qa_generation(context, settings, budget_guard, codex_ready, model, effort, cancelled, on_token) -> ProductionQaGenerated`.
- CLI bridge: `run_real_qa_e2e_files(repository, cases, output, model, effort) -> Result<bool, String>`.
- Repository command: `npm run eval:qa-real-e2e`; the binary exits `0` for a passing complete suite, `2` for a measured suite with failed assertions, and `1` for setup/provider/runner failure.

### 3. Contracts

- UI `ask_luna` and the runner call the same two core functions. Only the UI adapter owns Tauri channels, active-request registration, repository identity checks, and the formal App database; the runner owns a UUID-named temporary SQLite workspace and production persistence for its isolated multi-turn history.
- The generator call remains `budget_guard.reserve("generator") -> codex_subscription::stream_answer -> settle(actual)`. After every generation attempt the wrapper records final budget usage into `QuestionContext` and rebuilds `AnswerAudit`, so the returned audit contains the settled generator/verifier stages and rejections rather than a pre-settlement snapshot.
- Grounding is a three-boundary contract: `verify_and_repair_with_semantic(...) -> (repaired_answer, draft_report)` records Draft Audit and `qa-repair-projection-v1`; `audit_repaired_answer(...) -> FinalGroundingAudit` maps final claims through `FinalClaimKey { normalized visible text, sorted unique evidence IDs }` with a FIFO queue so duplicate text and citation order are deterministic; then `audit_rendered_visible_answer(...)` reuses the natural visible projection and fails closed unless audited/visible body hashes, ordered visible claims, and Final-to-Draft provenance all match. `final-grounding-audit-v2` adds `claimSources`, `visibleProjectionValid`, `auditedBodySha256`, and `visibleBodySha256`; each supported final claim records `sourceDraftClaimId`, canonical evidence IDs, text hash, verification method, alignment, and confidence. Persistence, trusted history, heldout export, and `CitationValidation.supported` read only this finalized audit; Draft counts remain diagnostics.
- Claim repair never mutates the working answer with global `String::replacen`. Every ordered Draft Claim is located once against the immutable source from a monotonic cursor, validated as a non-empty/non-overlapping UTF-8 byte span whose source slice equals the exact claim text, and then all spans are rebuilt in one pass. Duplicate text consumes successive occurrences; a later substring cannot rewrite an earlier supported occurrence. Missing, invalid, overlapping, or out-of-order spans fail closed with `claim_span_not_found` or `claim_span_invalid`; there is no string-replacement fallback.
- `RepairProjectionAudit` stores only schema/status/error code, source/repaired hashes, operation count, and per-operation Claim ID, byte span, source-text hash, and replacement kind. After reconstruction, every factual fragment must consume one exact Supported Draft key; an introduced fact or lost supported fact fails before Final Audit with `introduced_factual_claim` or `supported_claim_lost`. Failed invariant audits retain the safe attempted operation metadata and replacement count while the user-facing result is replaced by the fixed no-supported notice.
- A final factual claim passes only when its whitespace/repair-boundary-punctuation-normalized text and exact evidence-ID list map to a Draft `Supported` claim, every ID belongs to the current evidence set, and at least one cited item is non-Graphify. New or unmatched final facts fail closed. Fixed repair notices are deterministic control text, not factual claims or citation obligations. Atomic extraction splits facts around those exact notices before classification, so an inline boundary such as `supported-clause，<repair notice>` neither merges the notice into the supported fact nor changes its Draft mapping. When no supported factual claim remains, the visible result is the fixed insufficiency sentence, final status is `insufficient_supported_claims`, persistence is rejected, and trusted history is empty.
- `qa-prompt-v17` makes `executionMode=direct` override the full research profile: Direct asks for 1–3 minimal supported facts plus an evidence boundary. Research/Exploratory receives deterministic evidence-coverage labels and must omit or use a fixed system notice for uncovered profile elements. With no support-eligible evidence, the contract forbids inferring an unknown named object's existence or mechanism and permits continuation only as explicitly unverified general knowledge or a hypothetical research setting. Natural-answer completeness requires the grounded claim minimum but does not force ungrounded profile labels; structured output continues to require its full role contract.
- `evals/qa_real_generator_e2e_cases.json` is the authoritative development suite and currently declares six Direct, Research, Exploratory, Multi-turn, Zero-evidence, and custom-vocabulary cases through its own `caseCount`, plus a canonical cases SHA-256. It contains no expected answer and no held-out material; validators derive the expected count from the frozen suite metadata rather than a duplicated historical five-case constant.
- `qa-run-v23` embeds `repairProjectionAudit`, `finalGroundingAudit`, Evidence Availability telemetry, and `zeroEvidenceAudit` beside the existing Draft verification fields and is the persisted schema. `qa-real-generator-e2e-report-v6` is metadata-only and additionally projects repair status/error/operation metadata, Planner attempted/used/status/fallback/reason/latency, Planner-stage/budget state, QueryPlan version, facet/query/kind counts, retrieval stop reason, the zero-evidence audit, persisted message status, and persisted trusted-context byte count. Each case has `persisted`, `prePersist`, optional `final`, and stable `errors`. `prePersist` diagnoses generator/repair output and never contributes validation errors. When persistence succeeds, only the persisted `final` observation supplies provider/model/generator/budget/citation/semantic/manifest/mode validation; when persistence fails, `final=null`, `persisted=false`, and the persistence code is the final error. This prevents a pre-persist failure from remaining beside overwritten final fields.
- Both observations explicitly separate `draft*` counts/claims from `finalFactualClaimCount`, `finalSupportedClaimCount`, `finalUnsupportedClaimCount`, `finalCitedClaimCount`, `finalUnknownCitationCount`, `finalCitationCoverage`, and `finalClaims`. Committed per-claim diagnostics contain only ID, type, status, evidence-ID count, stable reason code, alignment score, and SHA-256 of claim text; raw claim text is forbidden.
- Optional `QA_REAL_E2E_GROUNDING_DIAGNOSTIC_DIR` enables a temporary raw alignment diagnostic containing claim text, cited evidence title/snippet, status, reason, and alignment. The directory must be absolute and outside the repository; text uses the natural visible-text path/link redaction. The artifact is never committed and is deleted after diagnosis.
- QA lifecycle logging uses `qa-trace-v1` typed events for prepare/generate/semantic/audit/persist/E2E boundaries, `qa_repair_projection_started/completed/failed`, and Planner lifecycle events at their actual orchestration boundaries. Repair events contain only request hash, execution mode, claim/replacement counts, status, and a stable `repair_projection_invalid_*` error code. Planner events contain only request hash, execution mode, Provider/model when available, candidate/facet/query/kind counts, duration, and a stable error code. Desktop files use Tauri AppLogDir in debug and release; development E2E uses Git-ignored `apps/desktop/logs/`. Logs exclude questions, answers, prompts, claims, snippets, paths, credentials, tokens, provider payloads, and reasoning.
- `stable_planner_failure_kind` is the single payload-free Planner failure taxonomy: call/token budget, output-schema rejection, idle/total timeout, Provider rate-limit/exit/protocol/unavailable, RetrievalContract JSON/schema/scope/kind/facet/budget/general validation, cancellation, or `unknown`. Manifest, report, and ordinary logs receive only this enum, never the raw Provider/parser error.
- Research and Exploratory Report-v6 gates require a real Planner attempt and accepted plan: `plannerUsed=true`, `plannerStatus=succeeded`, no fallback/reason, observed non-rejected Planner stage, `qa-retrieval-contract-v2`, and at least one facet and planned search query. `failed_fallback` adds `planner_failed_fallback`; `succeeded` without an accepted plan adds `planner_success_without_plan`. Direct continues to accept `policy_disabled` and is never forced through Planner.
- A measured product gate failure is a valid E2E result and must be written to the report before exit `2`; it must not be converted to runner success. `QA_REAL_E2E_CASE_ID` may select one public case for diagnosis. Report v6 separates `scope`, `executedScopePassed`, `fullSuiteEvaluated`, and `releaseEligible`: a passing selected case sets `passed=true` / exit 0 while `releaseEligible=false`; only an executed and passing complete suite is release eligible. Expected executed-scope failure exits 2, while environment/provider/file errors exit 1.
- `EvidenceAvailability` deterministically classifies `paper | book | wiki` as support eligible and graph-only navigation as ineligible. `zero_evidence::has_support_eligible_evidence` is the sole production boolean predicate for Prompt selection, Direct grounded-mode selection, Codex output schema, Compatible API `response_format`, and Semantic-Verifier admission; no caller may substitute `!evidence.is_empty()`. `RetrievalQuery.plannedRequiredFacetIds` is the sorted/deduplicated authority for both `plannedRequiredFacetCount` and Partial Coverage. `coveredRequiredFacetCount` is the distinct set intersection of those required IDs with `coveredFacetIds`; optional covered facets never compensate for a missing required facet. Zero usable evidence means no selected evidence or an all-Graph set; a support-eligible set with a required-facet gap is partial coverage, not zero evidence. No retrieval score, top-K, grounding threshold, call budget, or token ceiling changes to manufacture zero evidence.
- The shared production core owns zero-evidence projection for Codex, Compatible API, and Offline generation. It strips all `[E#]`, provider reference appendices and `evidence:` links through the canonical natural visible-text transformation, inserts the exact notice once at the start, preserves a safe body under `## 一般知识参考（未经本库核验）`, and replaces empty or explicit false-KB-attribution bodies with deterministic next steps. General knowledge remains visible only with `epistemicStatus=unverified_general_knowledge`; unknown named entities are never promoted to known mechanisms.
- Zero-evidence completeness is independently applicable with minimum factual claim count `0`. PASS requires one notice, a non-empty general body or deterministic next step, the epistemic boundary, no citation token/unknown ID/appendix/evidence link/false KB attribution, and empty trusted context. `CitationValidation` remains `groundingStatus=unverified`, `zeroEvidence=true`, and never `supported=true`.
- With zero support-eligible evidence, Semantic Verification is deterministically `not_requested` with an empty fallback reason and performs no reservation or Provider call. Persistence accepts only a completed zero audit, stores the assistant message as `unverified`, and stores zero trusted-context bytes. Such content never enters trusted conversation history or later query rewriting.
- A `zeroEvidenceExpected` final Report-v6 observation derives availability/audit values from the production manifest and persisted status/trusted byte length from SQLite. It requires retrieval rounds greater than zero, zero support-eligible evidence, raw evidence empty or all Graph, the complete zero audit, `semanticStatus=not_requested`, `answerComplete=true`, `persistedMessageStatus=unverified`, and `persistedTrustedContextBytes=0`.

### 4. Validation & Error Matrix

| Condition | Required result |
|---|---|
| Codex subscription is not ready or a real model cannot be resolved | Stable `QA_REAL_E2E_PROVIDER_BLOCKED`; no fabricated report PASS |
| Provider/model is not real, generator stage is absent, or generator reservation is rejected | Case FAIL with a bounded metadata error code |
| Research/Exploratory Planner is not attempted, falls back, reports success without a usable plan, lacks a Planner stage, or produces no facet/query | Case FAIL with stable Planner gate errors; fallback answer quality cannot manufacture Planner success |
| Direct routing records `plannerStatus=policy_disabled` | Legal; Direct has no Planner-success requirement |
| A structured cited/appendix evidence ID is unknown | Case FAIL even when answer text is non-empty |
| A Draft Claim span is missing/invalid or repair introduces/loses a factual claim | Repair Projection FAIL, safe operation metadata retained, fixed fail-closed output, no persistence or trusted history |
| Final factual count is zero, final supported differs from factual, final unsupported/unknown is nonzero, or final citation coverage is not `1.0` | Case FAIL; never repair by zeroing Draft counters |
| Repair leaves only system notices | `insufficient_supported_claims`, no persistence, no trusted history |
| Semantic verification is unavailable for an evidence-backed turn | Accept only with explicit non-empty fallback reason and preserve the unavailable count |
| A zero-evidence turn has no semantic batch | Accept empty/`not_requested` semantic status; completeness and zero-evidence answer gates still apply independently |
| Production persistence rejects citation/completeness | Preserve `prePersist`, set `persisted=false` and `final=null`, stop that case, and report only the stable persistence error without saving answer text |
| Temporary database scope ends | Remove SQLite, WAL, SHM, journal, and the UUID workspace; never touch the App database |

### 5. Good / Base / Bad Cases

- Good: every case declared by the current public development suite calls the shared production core with real Codex and reranker metadata, produces a metadata-only report, and either passes or exposes exact product gate failures.
- Base: semantic verification returns an explicit budget/unavailable reason; the case remains auditable and fails only the independent production assertions that actually failed.
- Bad: invoke `codex exec` from the runner, use a mock answer, copy QA logic, read Independent Held-out data, persist into the App database, serialize full answers, or mark a partial/single-case run as passing.

### 6. Tests Required

- Deterministic tests assert global-replacement RED, duplicate/substring/same-prefix/Markdown occurrence binding, ordered UTF-8 spans, missing/overlapping fail-closed behavior, Draft `4 -> Final 1/1/0`, introduced-fact rejection, all atomic-connector notice boundaries in both supported-neighbor directions, system-notice classification, Final Audit idempotence, partial-claim removal, no-supported insufficiency, trusted-history gating, visible projection, unknown-citation rejection, Direct contract priority, the exact category/source/hash fixture contract, shared UI/runner core entrypoints, temporary workspace cleanup, Report-v6 metadata redaction, pre/final verdict separation, persistence-failure `final=null`, generator-budget rejection, zero-evidence projection/audit/completeness/persistence/contamination handling, Research failed-fallback rejection, usable Research success, Direct policy-disabled legality, every stable Planner failure category, and a valid Exploratory stub plan.
- Final quality requires Rust format, clippy for library/binaries, the task-directed QA subset, frozen state regression, and frontend production build/script wiring. A P1-5 close gate runs only `real-zero-evidence` once after deterministic gates; a later release task owns the complete real suite. Do not repeatedly call the real model after a conclusive report.
- Inspect the emitted JSON independently for forbidden content/path keys and assert that `caseCount` equals the current suite's declared case count, `generatorInvocationCount` covers every executed generator turn, provider/model identities are real, and `realProviderMeasured=true`.

### 7. Wrong vs Correct

#### Wrong

```text
runner -> separate codex command/mock -> custom answer checks -> PASS
```

#### Correct

```text
UI adapter -----+
                +-> shared prepare -> Draft Audit -> repair -> deterministic Final Audit -> persistence
temp E2E adapter+
                +-> v6 draft/final metadata-only report -> explicit PASS or measured FAIL
```

## 15. Codex JSONL Terminal Failures and Planner Provider Probes

### 1. Scope / Trigger

- Apply this contract whenever `codex_subscription::stream_answer` parses `codex exec --json`, classifies a Codex subprocess failure, or a Development probe isolates Query Planner Provider failures.
- The shared adapter owns these semantics for Planner, Generator, Semantic, and every other Codex caller. A Planner module must never reimplement subprocess or JSONL parsing.

### 2. Signatures

- `parse_codex_jsonl_line(line) -> Option<CodexJsonlObservation>` distinguishes activity, model metadata, agent delta/completion, `turn.completed`, fatal `turn.failed`, fatal top-level `error`, and non-fatal item warning.
- `classify_codex_terminal_message(message) -> &'static str` returns only a fixed safe category.
- `run_planner_probe_files(repository, probe_id, output, model, effort) -> Result<bool, String>` runs exactly one `a | b | c` Development probe and writes one non-overwritable atomic safe report.
- Optional raw diagnostic environment key: `QA_CODEX_EXEC_DIAGNOSTIC_DIR`.

### 3. Contracts

- `CodexTerminalFailure` stores only event type, fixed category, and SHA-256 of the message. Ordinary errors/logs/reports never retain the raw message.
- Fatal-event precedence is cancellation, local idle timeout, local total timeout, stdout `turn.failed`/top-level `error`, classified stderr, then generic exit code. A fatal stdout event terminates and joins the child process immediately and never refreshes idle timeout.
- `item.completed` with `item.type=error` is a warning only. It cannot override a later non-empty agent message, `turn.completed`, and exit code 0.
- A successful JSONL run requires exit code 0, no fatal observation, and a non-empty final agent message. `turn.completed` and item-warning counts remain safe telemetry.
- Stable categories cover schema rejection, bad request, context too large, authentication, usage/rate limits, overload, transport/connection/protocol, unavailable/unsupported model, cancellation, and unknown. Request-timeout/stream-disconnect failures classify as `transport`; Planner fallback projects this as `provider_transport`.
- `QA_CODEX_EXEC_DIAGNOSTIC_DIR` defaults off, must be absolute and outside the repository, and is consumed only by the Development probe path. It may temporarily contain stdout JSONL and stderr plus prompt/schema hashes; it is deleted after inspection and never committed.
- Probe A uses a tiny Boolean schema. Probe B is eligible only after A passes and uses the current RetrievalContract schema with minimal input. Probe C is eligible only after B passes and uses the unchanged public Research Planner input. A failed prerequisite stops downstream probes and forbids speculative Schema/input/timeout/budget changes.
- Probe lifecycle events are `qa_planner_probe_started`, `qa_planner_probe_completed`, and `qa_planner_probe_failed` with one hashed operation ID and safe counts/enums only.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| `turn.failed` carries a schema-class message | Return `CODEX_JSONL_TURN_FAILED: schema_rejected`; never fall through to `CODEX_EXIT_ERROR` |
| Top-level `error` carries rate/auth/context/transport information | Preserve its fixed category and message hash, terminate promptly, and omit raw text |
| Item error then agent message + completed turn + exit 0 | Succeed; record an item-warning count only |
| Non-zero exit without fatal JSONL or classifiable stderr | Keep generic `CODEX_EXIT_ERROR` / `provider_exit` |
| Diagnostic path is relative or inside the repository | `QA_CODEX_EXEC_DIAGNOSTIC_DIR_INVALID`; do not call the Provider |
| Probe output path or `.part` already exists | Fail closed; never overwrite a previous real probe identity |
| Probe A fails with auth/usage/rate/overload/transport/connection | Stop B/C, record external Provider block, and do not modify Planner behavior |

### 5. Good / Base / Bad Cases

- **Good**: a fatal JSONL event becomes a fixed category and hash within milliseconds; the safe report identifies the event type and failed stage without payload text.
- **Base**: an item warning occurs, then a valid final agent message and completed turn succeed normally.
- **Bad**: refresh idle timeout for a fatal line, ignore its message until process exit, log raw stdout/stderr, rerun probes until an accidental success, or change RetrievalContract/timeout/budget after Probe A failed.

### 6. Tests Required

- J1–J7 cover schema `turn.failed`, top-level rate limit, auth, context too large, generic exit, non-fatal item warning plus successful completion, and bounded prompt termination.
- The fixed-category matrix includes request timeout/stream disconnect plus every stable class and proves raw suffixes do not propagate.
- Probe tests cover absolute outside-repository diagnostics, safe report redaction, bounded A/B definitions, atomic non-overwrite behavior, and start/completion/failure event identity.
- Before real probes, run Codex Subscription/Provider Capabilities tests, fmt, and library/binary Clippy. After classification, run the taskbook's focused Rust, Python, frontend QA, and build gates without Independent Heldout execution.

### 7. Wrong vs Correct

#### Wrong

```text
valid JSON line -> refresh idle timeout -> ignore event -> non-zero exit -> provider_exit
```

#### Correct

```text
JSONL line -> typed observation -> fixed fatal category + hash -> terminate/join -> safe report
item warning -> continue -> final agent message + exit 0 -> success
```

## 16. RetrievalContract Provider Schema and Codex Child Proxy

### 1. Scope / Trigger

- Apply this contract whenever the full Query Planner domain schema is projected into a Codex `--output-schema`, or when any Codex subscription child process is spawned for version/login/exec work.
- The domain schema and Provider schema are distinct contracts. A caller that crosses the Provider boundary must never pass the full schema by convenience or naming ambiguity.

### 2. Signatures

- `retrieval_contract_schema() -> serde_json::Value` returns the complete domain schema.
- `retrieval_contract_provider_schema() -> serde_json::Value` clones the complete schema and applies the evidence-driven Provider compatibility transform.
- `query_plan_schema() -> serde_json::Value` exposes the complete domain schema; `query_plan_provider_schema() -> serde_json::Value` is the only legal schema for Planner Provider calls and Probe B/C.
- `WIRELESS_CODEX_PROXY_URL` is the optional child-process override. `off`, `direct`, and `none` are case-insensitive direct-mode values.
- The default Codex child proxy is `http://127.0.0.1:7890`.

### 3. Contracts

- The first compatibility transform recursively removes only `uniqueItems`. The full domain schema retains it at `requestedKinds`, `mustAttemptKinds`, and `facets[].preferredKinds`.
- Compatibility changes are evidence-driven: a new removed keyword requires a real `schema_rejected` observation plus one deterministic RED fixture. Never bulk-strip constraints merely because Structured Outputs supports only a subset.
- Rust remains the final acceptance boundary. `deny_unknown_fields` and `RetrievalContract::normalize()` continue to enforce kind legality/deduplication, subset relations, facet ID uniqueness/format, facet/query counts, budget ranges, schema version, and scope.
- Planner Provider wiring in production, heldout runtime consistency code, and Probe B/C uses only `query_plan_provider_schema()`. This wiring change does not authorize reading or running heldout data.
- Proxy resolution priority is `WIRELESS_CODEX_PROXY_URL` > any existing non-empty uppercase/lowercase HTTP/HTTPS/ALL proxy variable > the localhost:7890 default.
- Explicit proxy URLs are copied to the Codex child environment. Existing standard proxy variables are inherited unchanged. Direct mode removes child proxy variables and sets child-only `NO_PROXY=*`. The implementation never calls `std::env::set_var` and never changes the parent process.
- Proxy URLs/credentials are never logged. Existing Codex/Planner/Probe lifecycle events remain authoritative and record only safe mode-independent stage/status/category metadata.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Full domain schema is requested | Return all domain constraints, including all three `uniqueItems` occurrences |
| Provider schema is requested | Return the same tree with only `uniqueItems` absent |
| Duplicate kind values are returned | Deterministically deduplicate while preserving first occurrence order |
| Duplicate facet ID, invalid facet ID, invalid budget, or unknown field is returned | Fail locally with `RETRIEVAL_CONTRACT_INVALID` |
| Probe B still returns `schema_rejected` | Stop Probe C/Research; capture one external diagnostic and change only the next proven keyword |
| `WIRELESS_CODEX_PROXY_URL` contains a URL | Override inherited standard proxy values for the Codex child only |
| Override is `off`, `direct`, or `none` | Remove child proxy variables and force child `NO_PROXY=*` |
| Override is absent and a standard proxy exists | Preserve inherited proxy configuration without overwrite |
| No override or standard proxy exists | Inject `http://127.0.0.1:7890` into the Codex child only |
| Probes/Planner succeed but final Research fails citation/grounding | Classify as the downstream QA gate; do not blame proxy/schema, rerun live traffic, or modify forbidden layers |

### 5. Good / Base / Bad Cases

- **Good**: the full schema is cloned, one proven incompatible keyword is removed recursively, Probe B/C pass, and Rust rejects an invalid facet or budget after generation.
- **Base**: a user already has `HTTPS_PROXY`; the Codex child inherits it and the application adds no default override.
- **Bad**: delete every validation keyword after one schema rejection; pass `query_plan_schema()` to production Provider calls; mutate global environment; log a proxy URL; rerun a failed real Research until it happens to pass.

### 6. Tests Required

- S1–S3 recursively prove Provider absence, full-schema retention, and structural equality except `uniqueItems`.
- S4–S8 prove kind deduplication, duplicate facet rejection, budget rejection, facet-pattern rejection, and unknown-field rejection.
- Proxy unit tests cover explicit override precedence, inherited standard proxy, localhost:7890 default, and all three direct tokens without parent environment mutation.
- Codex adapter regression, fmt, and focused Clippy must pass. Live verification is ordered and one-shot: temporary-proxy B -> C -> selected Research, then after default integration and cleared Shell proxy A -> B -> C -> selected Research.
- A live report must distinguish Planner/Provider status from downstream Semantic/Grounding/citation/persistence status; no Independent Heldout run is permitted for this contract.

### 7. Wrong vs Correct

#### Wrong

```text
full domain schema -> Codex --output-schema
schema_rejected -> remove every unfamiliar keyword -> retry until green
global set_var(HTTP_PROXY, localhost:7890)
```

#### Correct

```text
full domain schema
  -> clone
  -> remove only proven uniqueItems incompatibility
  -> Provider schema
  -> Codex child with override > inherited > localhost:7890 policy
  -> local strict parse/normalize
  -> one-shot staged verification with independent downstream gates
```
