# Design — Independent Held-out Phase 1 Runner

## Boundaries

- New evaluation modules live outside `qa/**` so production ranking, prompts and verification behavior remain untouched.
- Existing production QA functions are composed by a reusable one-shot executor; tests inject a fixture executor only at the runner boundary.
- Dataset validation and audit validation are deterministic and perform no model calls.

## Modules

1. `evals/heldout_contract.json`
   - Canonical allowed ResearchIntent names and schema version.
   - `heldout_questions.json`, Rust and Python tests must match it exactly.
2. `tools/heldout_contract.py`
   - Shared Python JSON loader, canonical case serialization and frozen-dataset validation.
   - `qa_accuracy_eval.py` imports it instead of maintaining an independent type list/hash implementation.
3. `apps/desktop/src-tauri/src/heldout_runner.rs`
   - Rust contract/dataset DTOs, strict validation, Git/runtime identity, case isolation, audit projection/validation and atomic output.
   - Generic `CaseExecutor` boundary supports synthetic tests; the public runner always installs the production executor.
4. `apps/desktop/src-tauri/src/bin/heldout-eval.rs`
   - Strict CLI parsing and non-zero exits on validation/run failures.
5. `lib.rs` production one-shot entry
   - Builds an isolated current repository index and executes the same planning, retrieval, provider generation, semantic verification and `audit_generated_answer_with_semantic` functions used by `ask_luna`.
   - No scoring or reviewer verdicts.

## Dataset contract

Canonical SHA-256 is computed over UTF-8 JSON of `cases` with recursively sorted object keys, compact separators and Unicode preserved, matching `qa_accuracy_eval.py`.

Validation order:

1. top-level role/split/status/minimum;
2. curation independence/identity/time/hash;
3. minimum count and unique IDs;
4. canonical type and non-empty question;
5. Git clean/commit;
6. run identity reservation.

## Run identity and output

`sha256(datasetSha + gitCommit + canonical runtime config)` identifies a run. The final directory is `<output-dir>/<dataset-sha>/<git-commit>/<runtime-id>/`. Any existing final directory fails before case execution. A temporary sibling directory is created exclusively and renamed only after every case bundle and run manifest validate.

Each case gets a fresh UUID session marker and empty history. The runner does not reuse desktop chat sessions or case state.

## Audit bundle

Each `<case-id>.json` contains:

- `question`, final repaired/rendered `answer`;
- `answerClaims` projected from current run manifest claim verifications;
- complete `EvidenceItem[]` and unmodified `QaRunManifest`;
- run identity metadata: dataset SHA, Git commit, provider/model/effort, semantic/reranker/embedding identifiers, knowledge snapshot, isolated session ID.

The writer rejects bundles when claims are absent/misaligned, text is not a substring of the final answer, claim count differs, cited IDs are missing, evidence/checksum sets differ, or the production manifest is absent.

## Compatibility and rollback

- `qa_accuracy_eval.py` may keep verdict compatibility, but frozen dataset types are canonical-only.
- Existing QA commands remain unchanged.
- Removing the new binary/module/script restores the prior product; no database migration is introduced.
