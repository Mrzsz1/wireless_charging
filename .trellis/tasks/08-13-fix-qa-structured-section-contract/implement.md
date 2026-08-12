# Implementation Plan

1. Add canonical section descriptors and emit an unambiguous JSON section contract in the QA prompt.
2. Extend structured section decoding with optional stable IDs.
3. Normalize canonical-ID, legacy-title, and literature split-title payloads before strict validation and deterministic rendering.
4. Carry structured parser errors separately through `AnswerAudit` and return the dedicated error code before citation formatting.
5. Add focused regression tests for canonical IDs, legacy title compatibility, split-section normalization, and dedicated error reporting.
6. Run Rust formatting and focused QA tests/checks; review the diff for unrelated changes.
