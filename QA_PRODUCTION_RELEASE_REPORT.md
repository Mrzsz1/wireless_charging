# QA Production Release Report

- **Decision:** `FAIL`
- **Git commit:** `0b28fd0eeccf89535821b3d2c40244c5c8c6e41c`
- **Generated:** `2026-08-26T06:23:26.794885+00:00`
- **Build:** `git:0b28fd0eeccf89535821b3d2c40244c5c8c6e41c`
- **Dataset version/hash:** `qa-production-eval-v1` / `0a906ab55680815fb9c773267e6b29ae6646d31cd7386f0e20241ac22cea4bb2`
- **Runtime config hash:** `4678e568e508546dc31adcbf97edfea60fca89e64bf0298b8cf9a26a595219ad`
- **Platform / CPU / memory:** `Windows 11` / `Intel64 Family 6 Model 154 Stepping 3, GenuineIntel` / `34049417216`
- **Providers:** `{"answer": {"model": "deterministic", "provider": "offline-evidence"}, "embedding": {"model": "Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q", "provider": "fastembed-local"}, "reranker": {"model": "BAAI/bge-reranker-base", "provider": "cross-encoder-research-v1"}, "verification": {"model": "gpt-5.6-luna", "provider": "codex-subscription"}}`
- **Models:** `{"answer": "deterministic", "embedding": "Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q", "reranker": "BAAI/bge-reranker-base", "verification": "gpt-5.6-luna"}`

## Gate Results

| Gate | Result | Actual | Requirement | Reason |
|---|---:|---:|---:|---|
| `retrieval.recall20` | PASS | `1.0` | `>= 0.95` | threshold satisfied |
| `retrieval.recall10` | PASS | `0.9615384615384616` | `>= 0.9` | threshold satisfied |
| `retrieval.mrr` | PASS | `0.9615384615384616` | `>= 0.85` | threshold satisfied |
| `retrieval.ndcg10` | PASS | `0.8514179310404342` | `>= 0.85` | threshold satisfied |
| `conversation.reference` | PASS | `1.0` | `>= 0.95` | threshold satisfied |
| `conversation.constraint` | PASS | `1.0` | `>= 0.97` | threshold satisfied |
| `conversation.objective` | PASS | `1.0` | `>= 0.97` | threshold satisfied |
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
| `semantic.realProvider` | PASS | `True` | `== True` | threshold satisfied |
| `semantic.invalidVerified` | PASS | `0` | `== 0` | threshold satisfied |
| `heldout.independent` | FAIL | `None` | `== True` | missing or invalid artifact heldout.json |
| `heldout.cases` | FAIL | `None` | `>= 30` | missing or invalid artifact heldout.json |
| `heldout.precision` | FAIL | `None` | `>= 0.97` | missing or invalid artifact heldout.json |
| `heldout.unsupported` | FAIL | `None` | `<= 0.02` | missing or invalid artifact heldout.json |
| `heldout.citations` | FAIL | `None` | `>= 0.98` | missing or invalid artifact heldout.json |
| `heldout.completeness` | FAIL | `None` | `>= 0.98` | missing or invalid artifact heldout.json |
| `performance.profile` | FAIL | `False` | `== True` | actual False does not satisfy == True |
| `performance.measured` | FAIL | `False` | `== True` | actual False does not satisfy == True |
| `performance.latency` | FAIL | `None` | `<= metric maxP95LatencyMs` | missing metric p95LatencyMs |

## Fallbacks and Limitations

- `grounding.precision`: missing or invalid artifact grounding.json
- `grounding.unsupported`: missing or invalid artifact grounding.json
- `grounding.contradicted`: missing or invalid artifact grounding.json
- `grounding.citations`: missing or invalid artifact grounding.json
- `research.methods`: missing or invalid artifact open_research.json
- `research.constraints`: missing or invalid artifact open_research.json
- `heldout.independent`: missing or invalid artifact heldout.json
- `heldout.cases`: missing or invalid artifact heldout.json
- `heldout.precision`: missing or invalid artifact heldout.json
- `heldout.unsupported`: missing or invalid artifact heldout.json
- `heldout.citations`: missing or invalid artifact heldout.json
- `heldout.completeness`: missing or invalid artifact heldout.json
- `performance.profile`: actual False does not satisfy == True
- `performance.measured`: actual False does not satisfy == True
- `performance.latency`: missing metric p95LatencyMs

## Final Decision

`FAIL` — 15/30 frozen gates passed.
