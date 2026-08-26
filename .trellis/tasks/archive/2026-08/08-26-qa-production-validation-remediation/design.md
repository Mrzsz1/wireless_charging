# Design — QA Production Validation Remediation

## Boundaries

不改变 Retriever/Graphify/Intent/Conversation Memory 架构。修复点限定在 reranker 输入与排序、评测 runner、真实 verifier benchmark、held-out review protocol 和 performance telemetry。

## Ranking data flow

```text
channel candidates
  -> stable passage dedup
  -> RRF/base score
  -> execution-mode candidate cap
  -> Cross-Encoder(exact passage, clean resolved query)
  -> finite normalized score fusion
  -> bounded document-aware diversification
  -> parent context expansion
  -> EvidenceManager
```

`mrr_diagnostics.json` 同时输出 passage rank 与 collapsed document rank。生产阈值读取 document rank；诊断禁止输出绝对路径或整段正文。

## Semantic benchmark

冻结 fixture 保存 claim、bounded evidence、gold verdict 与类别；运行器复用生产 `VerificationProvider` structured path。真实 Provider 的原始响应不落盘，只保存解析结果、聚合指标、安全错误种类和 metadata envelope。

## Unified release layout

```text
evals/releases/<git-sha>/
  manifest.json
  retrieval.json
  mrr_diagnostics.json
  conversation.json
  reranker.json
  semantic_verifier.json
  performance.json
  heldout.json
  grounding.json
  open_research.json
  release_gate.json
  QA_PRODUCTION_RELEASE_REPORT.md
```

每次运行先创建新目录，任何工件通过 `.part -> fsync -> atomic rename` 写入。release gate 只读取同一 run manifest 声明的工件。

## Held-out separation

系统先导出不含 system verdict 的 reviewer bundle。人工 review 导入后校验 reviewer identity、blind/independent flags、完整 claim coverage、差异集合和第三人裁决。一个 canonical run 同时派生 heldout/grounding/open-research，避免分母和版本漂移。

## Performance

模型加载 session 仍由进程级缓存拥有。单次请求 telemetry 分离 load/prepare/inference；benchmark 对 cold start 单独计数，warm run 按 ExecutionMode 报 P50/P95/P99。Profile hash 写入 metadata，并在测量开始前验证 sealed=true。

## Rollback

每个阶段单独提交。Ranking、semantic benchmark、artifact harness、held-out tooling、performance 可分别回滚。任何阶段失败时保留现有 deterministic fallback 和当前 FAIL 报告。

