# Phase 1 Deterministic RED Baseline

## Control

- Production policy and `LlmBudgetGuard` logic were unchanged from baseline `adaptive-routing-v1` when this run occurred.
- Command: `cargo test --lib adaptive_routing -- --nocapture`.
- Result: **FAILED as required**, 12 passed / 6 failed.

## Reproduced failures

| Test | Baseline failure |
|---|---|
| B1 Direct legal chain | Third `semantic_verifier` reservation rejected with `call_budget` after Understanding + Generator. |
| B2 Research worst-case legal chain | Fourth `semantic_verifier` reservation rejected with `call_budget` after Understanding + Planner + Generator. |
| B3 reserve protection | Extra non-Semantic call was rejected only because the total was exhausted; the following Semantic call was also rejected, proving no protected slot existed. |
| B4 one-shot Semantic | A second Semantic reservation was accepted, proving no per-request Semantic reserve accounting existed. |
| B6 reconfigure | Direct Understanding followed by Research Planner + Generator exhausted the Research total before Semantic. |
| B7 failed Provider accounting | Releasing the first Semantic reservation allowed a second Semantic reservation, proving no non-refundable Semantic-use counter existed. |

B5 passed on the baseline and confirms the existing token equation already fails closed. The implementation phase must preserve that behavior.
