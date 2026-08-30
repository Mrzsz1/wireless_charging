# Direct Root-Cause Diagnosis

## Run control

- Case: public regression `real-direct-rose`.
- Real Provider diagnostic executions: exactly 1.
- Provider/model: `codex-subscription` / `gpt-5.6-luna`.
- Actual run selected 2 evidence items, used the real Cross-Encoder, had no generator fallback and no generator budget rejection.
- The repository-external raw diagnostic was inspected once and deleted; only this aggregate conclusion is retained.

## Four-part conclusion

| Question | Result | Evidence |
|---|---:|---|
| `evidenceAnswersQuestion` | `true` | The current retrieval path selected the exact ROSE primary paper and its problem/challenge section; the section identifies the probabilistic EMR-safety constrained ROSE problem and its non-convex/infinite-constraint difficulty. |
| `rawGeneratorContainsEvidenceId` | `false` | The one Draft claim had `evidenceIdCount=0` and reason `missing_explicit_evidence_mapping`. |
| `parserExtractedEvidenceId` | `false` | Draft diagnostics recorded no cited evidence for the claim. |
| `finalProjectionPreservedEvidenceBinding` | `false` | There was no binding to preserve: Final factual/support counts were both zero and persistence failed at citation validation. |

## Decision

Select **Branch B — Direct generator structured evidence binding**.

The evidence layer can answer the question, while the raw Direct answer omitted every `[E#]`. The parser and final projection did not discard an existing binding; they received none. The next change therefore modifies only Direct generation/output parsing and deterministic rendering. Retrieval, planner, semantic-verifier policy, budgets, reranker, embedding, and Research/Exploratory generation remain unchanged.
