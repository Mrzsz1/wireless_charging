# Design — Held-out Visible Claim Projection

## Boundary

The production QA pipeline remains authoritative and unchanged. This task changes only how the held-out harness projects already-verified pre-render claims into the already-rendered answer and how exported bundles are independently validated.

## Shared canonical transformation

Before natural rendering, the existing AnswerRepair may replace a claim according to its already-decided verification status. The harness first uses the same pure `project_claim_after_repair` mapping used by production repair: contradicted/not-verifiable claims become the existing fixed replacement sentence, partially supported claims receive the existing prefix, and other statuses keep the original text. This is a pure extraction of current behavior, not a verifier-policy change.

`qa/natural_answer.rs` owns one pure projection pipeline:

1. cut any existing backend appendix at `## 参考证据`;
2. trim the input boundary;
3. remove well-formed `[E<digits>]` tokens while collecting their IDs;
4. sanitize unsafe Markdown link targets to `#blocked-link`;
5. redact Windows drive/UNC absolute paths as `[本地路径已隐藏]`;
6. trim the resulting visible text.

The renderer consumes this pipeline for the answer body. The held-out runner consumes the same pipeline for each verified claim. Therefore later renderer sanitation changes cannot silently diverge from export projection.

## Bundle contract v2

- `answerClaims[].text` is the canonical visible projection and must be non-empty and a byte-for-byte substring of final `answer`.
- `answerClaims[].citedEvidenceIds` is structured provenance copied from the verified claim. It need not occur as `[E#]` in visible text.
- IDs must be unique per claim and must be members of the bundle's complete `evidence[].id` set.
- Existing evidence checksum cardinality and digest validation remains mandatory.

## Python independent validation

`qa_accuracy_eval._validate_answer_claims` receives the complete known evidence-ID set returned by `_validate_evidence_and_manifest`. It rejects any citation outside that set and no longer requires inline `[E#]` tokens in final visible claim text.

## Failed partial run handling

The harness keeps non-overwrite semantics. A failed `.part` directory is never promoted. Operationally, the known old partial is renamed in place to an immutable sibling whose name contains `invalid_harness_run`; no answer file is opened, deleted or reused. Because run identity includes Git SHA, the fixed commit creates a fresh identity and reruns all 40 cases.

## Compatibility and non-goals

- Run schema changes from v1 to v2.
- No frozen dataset, QA algorithm, verifier decision, prompt or final rendered answer changes.
- No migration reads old partial answers.
