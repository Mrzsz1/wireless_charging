# Implementation Plan

1. Harden contracts and freeze provenance
   - Add reusable sealed candidate-pool validation.
   - Verify frozen draft ID/question/type against the exact 80 heldout candidates.
   - Record candidate-pool provenance in frozen output.
   - Add synthetic freeze success and drift/failure tests.

2. Refactor reviewer coverage and metrics
   - Add independent method/constraint coverage validators.
   - Generalize two-reviewer agreement and third-person adjudication across all three verdict channels.
   - Change `review_totals` callers to pass the frozen case.
   - Remove method/constraint metric dependence on `answerClaims.dimension`.

3. Update public schemas, export bundle and regression tests
   - Publish expected coverage lists and allowed verdicts in blind bundles.
   - Update `heldout_questions.json` and QA contract.
   - Add anti-cheating regressions for 1/4 method recall and 1/3 constraint preservation.

4. Verification and Git save
   - Run focused Python unit tests only.
   - Run heldout pending CLI smoke and JSON validation.
   - Commit each completed implementation phase locally, excluding user-owned untracked files.
