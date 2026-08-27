# Implementation Plan

1. Shared projection and Rust runner
   - Extract canonical visible-text projection from the natural renderer.
   - Use it in `build_bundle`, bump run schema to v2, retain structured evidence IDs.
   - Preserve final-answer containment, duplicate/empty claim and checksum gates.
   - Add the requested Rust regressions, including renderer output invariance.

2. Python independent validation and contracts
   - Validate `citedEvidenceIds` against complete run evidence IDs.
   - Remove the obsolete final-text `[E#]` requirement only.
   - Add unknown-ID and visible-text regressions.
   - Update heldout public schema, README and backend QA contract.

3. Verify and save
   - Run focused heldout/natural-answer Rust tests and focused Python tests.
   - Run formatting, compilation and pending-entry smoke checks without broad unrelated suites.
   - Commit locally by phase.

4. Preserve invalid partial and rerun
   - Discover the old `.part` path without opening answer files.
   - Rename it to an `invalid_harness_run` record without overwrite/delete.
   - On a clean fixed commit, rerun the exact same frozen 40-case dataset from case 1 through case 40.
   - Verify the completed run schema/count/seal using metadata only.
