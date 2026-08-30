# P1-1D Design

## Boundary

The change is limited to Draft Claim location, deterministic repair projection, projection audit metadata, orchestration logging, final deterministic audit integration, and tests. Retrieval, planning, providers, proxy configuration, generation, and Semantic verification policy remain untouched.

## Data Contracts

```rust
struct LocatedDraftClaim {
    claim_index: usize,
    start: usize,
    end: usize,
}

struct RepairProjectionOperation {
    claim_id: String,
    source_start: usize,
    source_end: usize,
    original_text_sha256: String,
    replacement_kind: String,
}

struct RepairProjectionAudit {
    schema_version: String,
    status: String,
    error_code: String,
    source_body_sha256: String,
    repaired_body_sha256: String,
    operation_count: usize,
    operations: Vec<RepairProjectionOperation>,
}
```

Only safe hashes, IDs, spans, enum-like kinds, counts, and status/error codes enter reports or logs.

## Projection Flow

```text
immutable source answer
  -> extract and verify ordered Draft Claims
  -> locate every claim from cursor in source answer
  -> validate UTF-8 boundaries, equality, monotonicity, and non-overlap
  -> rebuild once: untouched gap + original supported/not-applicable span
                   or existing notice for non-supported span
  -> if supported_count == 0, preserve NO_SUPPORTED_CLAIMS_NOTICE
  -> re-extract and exact-map every factual projected claim to Supported Draft
  -> RepairProjectionAudit PASS/FAIL
  -> existing Final Grounding Audit
  -> existing Natural Markdown render
  -> existing Visible Projection Audit
```

## Error Contract

- `claim_span_not_found`: an ordered claim text cannot be found after the previous end.
- `claim_span_invalid`: empty span, invalid UTF-8 boundary, source mismatch, or out-of-order/overlapping span.
- `introduced_factual_claim`: repaired output contains a factual claim without an exact Supported Draft mapping.

The internal projection helper returns `Result`. The orchestration-facing verifier remains compatible, records a failed audit, marks verification failed, and substitutes the existing no-supported notice so unsafe text is not exposed. Downstream grounding gates remain fail-closed.

## Collision Safety

- Duplicate occurrences are consumed in source order by a monotonic cursor.
- A later short claim cannot replace its first textual occurrence inside an earlier supported claim because reconstruction uses the already-bound later span.
- Replacements never mutate the string used for subsequent locations.
- Source gaps preserve Markdown/list/connector formatting byte-for-byte.

## Logging

At the QA orchestration boundary emit:

```text
qa_repair_projection_started
qa_repair_projection_completed
qa_repair_projection_failed(errorCode=repair_projection_invalid_*)
```

Events use the existing request-ID hash and QA trace sink. They contain aggregate claim/repaired counts only.

## Compatibility and Rollback

- Existing repair notice text and Semantic status merge are unchanged.
- Existing final audit schemas stay readable; new repair-audit fields use serde defaults.
- Rollback is the isolated source-span repair commit; no data migration is required.
