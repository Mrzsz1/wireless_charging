# QA Production Release Report

- **Decision:** `FAIL`
- **Git commit:** `66c5bba3f087a2851553fdfb0d24c2747ae62bd0`
- **Generated:** `2026-08-26T05:08:15.666841+00:00`
- **Build:** `git:66c5bba3f087a2851553fdfb0d24c2747ae62bd0`
- **Dataset version/hash:** `2026-08-12-heldout-v1` / `492726564512368352f7981b98f78bde56ae3873f90428fb1455c56603841270`
- **Runtime config hash:** `fd8eed02bae8b933821188ae177ba5df987e646cb8b26dc6d45bb0dc28c8e15a`
- **Platform / CPU / memory:** `Windows 11` / `Intel64 Family 6 Model 154 Stepping 3, GenuineIntel` / `34049417216`
- **Providers:** `{"answer": {"model": "deterministic", "provider": "offline-evidence"}, "embedding": {"model": "Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q", "provider": "fastembed-local"}, "reranker": {"model": "BAAI/bge-reranker-base", "provider": "cross-encoder-research-v1"}, "verification": {"model": "not_configured", "provider": "deterministic-fallback"}}`
- **Models:** `{"answer": "deterministic", "embedding": "Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q", "reranker": "BAAI/bge-reranker-base", "verification": "not_configured"}`

## Gate Results

| Gate | Result | Actual | Requirement | Reason |
|---|---:|---:|---:|---|
| `retrieval.recall20` | PASS | `1.0` | `>= 0.95` | threshold satisfied |
| `retrieval.recall10` | PASS | `0.962` | `>= 0.9` | threshold satisfied |
| `retrieval.mrr` | FAIL | `0.821` | `>= 0.85` | actual 0.821 does not satisfy >= 0.85 |
| `retrieval.ndcg10` | PASS | `0.851` | `>= 0.85` | threshold satisfied |
| `conversation.reference` | FAIL | `None` | `>= 0.95` | missing or invalid artifact conversation.json |
| `conversation.constraint` | FAIL | `None` | `>= 0.97` | missing or invalid artifact conversation.json |
| `conversation.objective` | FAIL | `None` | `>= 0.97` | missing or invalid artifact conversation.json |
| `grounding.precision` | FAIL | `None` | `>= 0.97` | missing or invalid artifact grounding.json |
| `grounding.unsupported` | FAIL | `None` | `<= 0.02` | missing or invalid artifact grounding.json |
| `grounding.contradicted` | FAIL | `None` | `<= 0.01` | missing or invalid artifact grounding.json |
| `grounding.citations` | FAIL | `None` | `>= 0.98` | missing or invalid artifact grounding.json |
| `research.methods` | FAIL | `None` | `>= 0.9` | missing or invalid artifact open_research.json |
| `research.constraints` | FAIL | `None` | `>= 0.97` | missing or invalid artifact open_research.json |
| `reliability.crashes` | PASS | `0` | `== 0` | threshold satisfied |
| `reliability.providerFailures` | PASS | `1.0` | `>= 1.0` | threshold satisfied |
| `reliability.fallback` | PASS | `1.0` | `>= 0.99` | threshold satisfied |
| `reliability.invalidVerified` | PASS | `0` | `== 0` | threshold satisfied |
| `reranker.realModel` | PASS | `True` | `== True` | threshold satisfied |
| `reranker.fallback` | PASS | `0.0` | `<= 0.05` | threshold satisfied |
| `semantic.realProvider` | FAIL | `False` | `== True` | actual False does not satisfy == True |
| `semantic.invalidVerified` | PASS | `0` | `== 0` | threshold satisfied |
| `heldout.independent` | FAIL | `False` | `== True` | actual False does not satisfy == True |
| `heldout.cases` | FAIL | `0` | `>= 30` | actual 0 does not satisfy >= 30 |
| `heldout.precision` | FAIL | `0.0` | `>= 0.97` | actual 0.0 does not satisfy >= 0.97 |
| `heldout.unsupported` | FAIL | `1.0` | `<= 0.02` | actual 1.0 does not satisfy <= 0.02 |
| `heldout.citations` | FAIL | `0.0` | `>= 0.98` | actual 0.0 does not satisfy >= 0.98 |
| `heldout.completeness` | FAIL | `0.0` | `>= 0.98` | actual 0.0 does not satisfy >= 0.98 |
| `performance.profile` | FAIL | `False` | `== True` | actual False does not satisfy == True |
| `performance.measured` | FAIL | `False` | `== True` | actual False does not satisfy == True |
| `performance.latency` | FAIL | `None` | `<= metric maxP95LatencyMs` | missing metric p95LatencyMs |

## Fallbacks and Limitations

- `retrieval.mrr`: actual 0.821 does not satisfy >= 0.85
- `conversation.reference`: missing or invalid artifact conversation.json
- `conversation.constraint`: missing or invalid artifact conversation.json
- `conversation.objective`: missing or invalid artifact conversation.json
- `grounding.precision`: missing or invalid artifact grounding.json
- `grounding.unsupported`: missing or invalid artifact grounding.json
- `grounding.contradicted`: missing or invalid artifact grounding.json
- `grounding.citations`: missing or invalid artifact grounding.json
- `research.methods`: missing or invalid artifact open_research.json
- `research.constraints`: missing or invalid artifact open_research.json
- `semantic.realProvider`: actual False does not satisfy == True
- `heldout.independent`: actual False does not satisfy == True
- `heldout.cases`: actual 0 does not satisfy >= 30
- `heldout.precision`: actual 0.0 does not satisfy >= 0.97
- `heldout.unsupported`: actual 1.0 does not satisfy <= 0.02
- `heldout.citations`: actual 0.0 does not satisfy >= 0.98
- `heldout.completeness`: actual 0.0 does not satisfy >= 0.98
- `performance.profile`: actual False does not satisfy == True
- `performance.measured`: actual False does not satisfy == True
- `performance.latency`: missing metric p95LatencyMs

## Final Decision

`FAIL` — 10/30 frozen gates passed.
