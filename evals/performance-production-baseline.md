# Reranker 目标机器性能基线

- 检查日期：2026-08-26
- Profile：`2026-08-26-windows-cpu-v1`（测量前冻结）
- 机器类别：Windows 11 x86_64、Intel mobile CPU class、内存至少 32 GiB、CPU ONNX
- 模型：`BAAI/bge-reranker-base@2cfc18c`
- 工作负载：Direct / Research / Exploratory 各 3 个真实 RAG case；1 次 warmup、每题 2 次测量，各模式 6 个 warm samples
- Cold model load：2917 ms，冻结上限 60000 ms，`PASS`

## Warm latency

| ExecutionMode | Candidate cap | P50 | P95 | P99 | Frozen P95 SLO | Result |
|---|---:|---:|---:|---:|---:|---:|
| Direct | 30 | 6805 ms | 7645 ms | 7645 ms | 20000 ms | PASS |
| Research | 50 | 12459 ms | 21576 ms | 21576 ms | 30000 ms | PASS |
| Exploratory | 60 | 21821 ms | 23862 ms | 23862 ms | 45000 ms | PASS |

- 全部模式 SLO：`PASS`
- 全局 warm P95 / 上限：23862 / 45000 ms
- 平均 input prepare / inference：2.6 / 14110.4 ms
- 平均输入：175.4 tokens

性能门禁仅适用于该 sealed target profile；运行结果没有用于放宽阈值。模型 session 在进程内复用，warm samples 的 model-load 时间为零；候选在稳定去重后按模式 cap 截断，再批量 rerank，parent expansion 位于 rerank 之后。
