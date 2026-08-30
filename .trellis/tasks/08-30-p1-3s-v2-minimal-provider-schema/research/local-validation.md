# S4–S8 Local Retrieval Validation

Command: `cargo test qa::retrieval_contract::tests --lib`

Result: 8 passed, 0 failed.

- S4: requested/must-attempt/preferred kind arrays deterministically remove duplicates while preserving first occurrence order.
- S5: duplicate facet IDs return `RETRIEVAL_CONTRACT_INVALID`.
- S6: `maxRounds=99` returns `RETRIEVAL_CONTRACT_INVALID`.
- S7: facet ID `A bad id!` returns `RETRIEVAL_CONTRACT_INVALID`.
- S8: an unknown top-level field is rejected through the Serde `deny_unknown_fields` contract.
- The full-domain/provider-schema S1–S3 test also remained green in the same run.
