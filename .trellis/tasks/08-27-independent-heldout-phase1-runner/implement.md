# Implementation Plan — Independent Held-out Phase 1 Runner

## Phase A — Shared freeze contract

- [x] Add canonical heldout contract file and update heldout template types.
- [x] Extract Python dataset validation/hash/type rules into a shared module.
- [x] Update `qa_accuracy_eval.py` and its tests to consume the shared rules.
- [x] Commit Phase A.

## Phase B — Rust runner core

- [ ] Add strict DTOs and frozen dataset validation matching Python.
- [ ] Add Git cleanliness/commit/runtime identity and no-overwrite directory reservation.
- [ ] Add audit claim/evidence/checksum validation and atomic bundle/run completion writes.
- [ ] Add synthetic tests for all fail-closed boundaries and case isolation.
- [ ] Commit Phase B.

## Phase C — Production executor and CLI

- [ ] Add reusable one-shot production QA executor without changing QA algorithms/prompts.
- [ ] Wire `heldout-eval` CLI and npm script with external paths and runtime overrides.
- [ ] Record provider/model/effort, verifier/reranker/embedding, KB snapshot, dataset SHA and Git commit.
- [ ] Prove fixture tests inject only at runner boundary while public CLI always uses production executor.
- [ ] Commit Phase C.

## Phase D — Documentation and limited verification

- [ ] Update eval README and backend QA contract.
- [ ] Run fmt/check/clippy, Python tests, runner synthetic tests and required QA/retrieval/conversation/semantic regression filters.
- [ ] Confirm heldout template remains empty and no heldout candidate content entered tests.
- [ ] Check ACs, commit, archive and journal.

## Explicitly excluded

- Freezing/selecting real heldout questions, running official blind evaluation or writing reviewer verdicts.
- Any retrieval, prompt, state, reranker, embedding, matcher, answer or verifier behavior change.
- Lowering release thresholds or adding gold expectations.
