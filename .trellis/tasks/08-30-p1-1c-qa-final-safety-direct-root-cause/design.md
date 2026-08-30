# Design — P1-1C QA Final Safety and Direct Root Cause

## Boundaries

The authoritative chain is:

```text
Draft claim extraction/verification
  -> deterministic repair
  -> FinalGroundingAudit
  -> natural renderer
  -> post-render visible integrity
  -> persistence
  -> Completed(final answer)
```

Only data proven at or after `FinalGroundingAudit` may flow into heldout export, trusted history, persistence success, or user-visible answer delivery.

## Component changes

### Heldout bundle and Python evaluator

- Rust runner reads final audit claims for v22 and emits a versioned bundle contract with Final-only provenance.
- Compatibility handling is version-gated; a v22 formal run with no valid final audit is invalid rather than downgraded.
- Python validates the same invariants independently against the full current evidence set and checks audit aggregate consistency.

### Trusted context

- Introduce a pure projection accepting final audit claims, filtering exact `Supported` status, preserving order, removing citation tokens through the shared visible projection, excluding special sections/notices, and deduplicating without reordering.
- Do not derive trust from `CitationValidation`, the complete rendered answer, or Draft status counts.

### UI safety boundary

- `run_production_qa_generation` may still use its internal callback for non-UI diagnostics/tests, but `ask_luna` passes a non-emitting production adapter.
- Remove zero-evidence and offline simulated Token emission. Terminal final content remains on `Completed` only.
- Frontend treats `token` as compatibility-ignored input and never presents it as answer content.

### Real E2E scope semantics

- Report derives `executedScopePassed` from executed results, `fullSuiteEvaluated` from selection scope, and `releaseEligible` from both.
- Process exit maps expected case failure to 2 and infrastructure failure to 1 without conflating release eligibility.

### Final provenance and post-render integrity

- Key Draft claims with `(normalized_text, sorted_unique_evidence_ids)` and store a FIFO queue per key.
- Add source mapping records to `FinalGroundingAudit`; additive serde defaults preserve older manifests.
- Reuse the canonical natural-answer visible-text transformation. Validate supported-claim spans in the final visible body and extract visible factual claims to reject unaudited additions.
- Persist only after this validation succeeds.

### Direct diagnosis decision

- One external diagnostic artifact outside the repository records raw local details, is inspected once, summarized into four booleans, and deleted.
- Branch A modifies only evidence selection, B only Direct structured generation/binding, or C only citation/parser/render/projection preservation.
- No second Provider call occurs until the selected deterministic regression and fix pass.

## Structured logging

Existing `qa::trace::emit` remains the sink. New/changed events use the request hash as operation identity and record stage/status/counts/error code only. Required event families cover heldout export validation, final visible audit, trusted-context projection, UI finalization boundary, E2E scope evaluation, and Direct diagnosis outcome without content payloads.

## Compatibility and schema

- Prefer additive defaults on `FinalGroundingAudit` and `QaRunManifest` where old field meaning is unchanged.
- Upgrade final-audit schema to v2 for source mapping and visible integrity fields.
- Upgrade heldout bundle/report schemas only where semantics change; current formal v22 runs must not masquerade as legacy.

## Rollback

Each deterministic phase is a separate commit. A failed phase is reverted by its own commit without touching earlier green phases. Local diagnostic artifacts remain outside Git and are deleted after the four-boolean conclusion is recorded.

