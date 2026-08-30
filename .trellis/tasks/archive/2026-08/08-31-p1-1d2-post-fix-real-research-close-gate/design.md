# P1-1D2 Design

## Verification Boundary

This task changes no production behavior before the live close-gate run. It validates the exact committed P1-1D implementation through deterministic suites, then invokes the shared production runner once. The runner must continue using the same prepare, generation, Semantic verification, repair, Final Audit, persistence, and Report-v5 paths as the desktop UI.

## Decision Table

| Result | Action |
|---|---|
| All deterministic checks pass and real run passes | Record exact metrics, mark PASS, archive, journal, push |
| Deterministic check fails | Fix only the confirmed P1-1D regression, rerun deterministic checks; no live invocation until green |
| Real run fails | Preserve the report and stable trace classification, stop live traffic, mark PARTIAL-BLOCKED or FAIL without weakening gates |

## Invariants

- One live invocation maximum.
- No raw answer/claim/evidence text enters committed reports or logs.
- No forbidden subsystem changes.
- No Independent Heldout access.
