# 真实回答生成 E2E Runner 实施计划

## Phase 1 — Shared Core

- [x] 用 CodeGraph 固化 `ask_luna` prepare/generator/verifier/audit/persist 调用路径。
- [x] 先添加 deterministic wiring regression，证明 Runner 与 UI 调用同一 generator core。
- [x] 最小抽取 Tauri-independent production QA core；UI adapter 保持 Channel/AppState/persistence 行为。
- [x] 运行 targeted Rust tests 与 `qa::` suite，Git 提交。

## Phase 2 — Runner and Cases

- [x] 新增 5-case development/regression/synthetic schema，禁止 answer gold 与 heldout 来源。
- [x] 新增临时 SQLite/index/session 生命周期和 multi-turn production persistence。
- [x] 新增 metadata-only validator/report writer，使用 `.part → atomic rename`。
- [x] 新增 `qa-real-e2e` binary 与 `npm run eval:qa-real-e2e`。
- [x] 添加 schema、脱敏、unknown citation、budget、temporary DB cleanup 和 exit-code regression tests，Git 提交。

## Phase 3 — Real Execution

- [x] 检查 Codex subscription 与 E 盘 semantic/reranker 部署。
- [x] 运行 `npm run eval:qa-real-e2e`，对每个不明失败最多一次有依据重试。
- [x] 检查报告不含 answer/prompt/path/secret，并记录真实 provider/model/stage/budget/citation/semantic 结果。
- [x] 若真实 Provider 不可用，报告环境阻塞，不修改生产 fallback（本次 Provider 可用，本项按非触发分支核验）。

## Phase 4 — Final Quality

- [x] `cargo fmt --check`。
- [x] `cargo clippy --lib --bins -- -D warnings`。
- [x] Rust QA/runner tests。
- [x] `npm run build` 与 script wiring 检查。
- [x] 更新任务 AC、提交报告、归档 Trellis 并记录 journal。

## Guardrails

- 不运行任何 heldout 命令或读取 heldout 数据。
- 不增加 timeout/token ceilings，不降低 assertion/threshold，不保存完整模型回答。
- 不在正式 App DB 创建、删除或迁移测试 session。
