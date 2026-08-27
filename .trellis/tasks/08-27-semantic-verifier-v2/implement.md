# Implementation Plan — Semantic Verifier v2

## Phase A — Dataset and contract

- [x] 审核并修正 v1 十个 `missing_condition` Gold，更新 version/hash。
- [x] 从 Markdown 转换 60-case v2 JSON，校验 ID、字段、hash 和 20/20/20 分布。
- [x] loader 支持 v1/v2 且按 schema fail closed。

## Phase B — Semantic boundary and metrics

- [x] 升级 Semantic Verifier version 与严格三步 Prompt。
- [x] 保持 hard checks/merge/repair 三态边界一致。
- [x] 扩展 report v2：六项 precision/recall、macro F1、confusion matrix、category、failed cases、逐题 latency/provider/fallback。
- [x] 增加少量相关 unit tests，不增加全量回归。

## Phase C — Limited verification

- [x] `cargo fmt --check` 与 `cargo check`。
- [x] 只运行 Semantic Verifier/benchmark 相关 Rust tests。
- [x] 使用真实 Provider 对 v2 运行一次，不重复刷题。
- [x] 生成 `semantic-verifier-v2-report.json` 与 `semantic-verifier-v2-baseline.md`。
- [x] 更新 QA spec 与任务 AC，本地 Git 提交。

## Explicitly skipped

- 全量 Rust/frontend/GUI/installer/RAG/Production Gate。
- 第二轮真实 benchmark 或按结果继续调 Prompt。
- 其他 QA 架构、provider/model/temperature/batch 修改。
