# Technical Design — QA GitHub Review Remediation

## 1. Architecture Boundaries

### 1.1 Answer/grounding pipeline

```text
Provider raw answer with inline [E#]
  -> Markdown-safe citation canonicalization
  -> ClaimExtractor (claim text + explicit IDs)
  -> ClaimClassifier (ClaimType only)
  -> VerificationProvider / heuristic fallback (VerificationStatus only)
  -> AnswerRepair
  -> DisplayRenderer strips inline IDs and appends backend evidence links
  -> Persist display answer + claim audit + manifest
```

`NaturalAnswerResult` distinguishes internal verification source from display Markdown. Display sanitization is downstream of verification.

### 1.2 Verification DTO

```text
VerifiedClaim {
  id
  text
  claim_type
  evidence_ids
  verification_status
  score
  verification_method
  reason
}
```

`CitationValidation.supported` describes the final repaired answer, not mere appendix integrity. Appendix integrity remains separate. Heuristic and semantic checks use separate booleans.

### 1.3 Reranking

```text
RRF candidates (Top 80)
  -> policy protections / eligibility
  -> CrossEncoderRerankProvider when explicitly deployed
  -> EmbeddingRescorer fallback
  -> Deterministic rank fallback
  -> rank fusion, never raw-score addition
```

The provider returns candidate identity, rank and score; final scoring is rank-based and stable. Availability checks never download at query time.

### 1.4 Provider capabilities and budget

```text
ProviderDescriptor
  capabilities: { answer, streaming, structured_output, understanding, planning }

LlmBudgetGuard
  reserve(stage, estimated_input, output_limit)
  complete(stage, actual/estimated usage)
  reject(stage, reason)
```

Codex and Compatible API adapters implement the same planning contract. One request-scoped guard spans planning and answer execution. Resolver escalation checks confidence, route policy, capability and budget.

### 1.5 Problem/method flow

```text
ProblemRepresentation
  -> neutral search terms (domain/objective/constraint/problem class)
  -> evidence retrieval discovers method pages/papers
  -> MethodMatcher evaluates discovered methods
  -> optional hypothesis-expansion channel
  -> corroboration required before evidence-backed recommendation
```

### 1.6 Parent expansion

Production Evidence Manager receives an exact parent resolver backed by:

```sql
child.id -> child.parent_block_id
         -> parent.id AND parent.document_id = child.document_id AND active=1
```

The candidate-only longest-block heuristic is deleted.

## 2. Cross-Layer Contract Changes

- Rust: claim DTOs, citation validation fields, run manifest versions, capability/budget telemetry, reranker status, method provenance.
- TypeScript: mirror optional fields for old persisted messages; UI says heuristic checked separately from semantic entailment.
- Persistence: JSON columns remain backward compatible through fail-closed serde defaults.
- Prompt: natural answer emits inline Evidence IDs for internal verification; renderer removes them only after audit.
- Evaluation: reports distinguish real cross-encoder deployed from embedding/deterministic fallback.

## 3. Compatibility and Migration

- No destructive SQLite migration; manifests are versioned and old rows hydrate with defaults.
- Missing `groundingStatus` becomes `unverified`.
- Old natural messages remain displayable but are not retrospectively promoted to verified.
- Existing deterministic retrieval remains fallback.
- Existing embedding model remains retrieval/EmbeddingRescorer fallback; cross-encoder has separate explicit deployment state.

## 4. Failure Semantics

- Missing claim mapping -> factual claim not_verifiable.
- Verification provider unavailable -> answer remains auditable; no semantic entailment flag.
- Cross-encoder unavailable -> exact fallback identity recorded.
- Budget exhausted -> stage is not called; stable fallback/stop reason recorded.
- Compatible API structured planning malformed -> deterministic fallback.
- Parent lookup ambiguous/missing -> no expansion.

## 5. Rollout and Rollback

- Each implementation phase is one local commit.
- Version/rollback flags remain at major contract boundaries until gates pass.
- A regressing phase can be reverted without reverting earlier reviewed phases.
- No GitHub push until the user explicitly requests it.

## 6. Key Trade-offs

- Inline IDs are required internally but hidden from final display to retain UX.
- Cross-encoder is optional at runtime but is the only path named cross-encoder reranker; fallback naming remains honest.
- Provider token usage is labeled actual or estimated.
- Production accuracy remains unclaimed until independent held-out review.
