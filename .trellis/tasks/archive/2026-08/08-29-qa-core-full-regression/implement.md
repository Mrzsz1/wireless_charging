# QA 核心功能总回归实施计划

## Phase 0 — Baseline

- [x] 记录 branch、HEAD、git status、Rust/Node/npm/Python/Codex 版本。
- [x] 探测 provider 与 semantic/reranker 本地模型状态，不输出凭据。
- [x] 审查 package scripts，标记并排除所有 heldout 入口。

## Phase 1 — Static Contracts

- [x] 用 CodeGraph 定位 QaRunManifest v21、budget、conversation/state/parameter、semantic、natural renderer、evaluation、Rust/TS/frontend consumers。
- [x] 验证固定 token ceilings、冻结 thresholds、schema 字段与调用顺序。
- [x] 建立 A–J 现有测试映射，确认真实缺口后再新增 synthetic tests。
- [x] Git 提交规划产物。

## Phase 2 — Deterministic Tests

- [x] 运行 `cargo fmt --check`。
- [x] 运行 `cargo clippy --lib -- -D warnings`。
- [x] 运行全部 Rust QA `qa::` tests，并记录 passed/failed/ignored。
- [x] 运行不读取 heldout 的 Python QA regression tests。
- [x] 运行 frontend QA tests 与 TypeScript build/type check。

## Phase 3 — Functional Matrix and Fix Loop

- [x] 逐项核对 A–J 与 actual tests；缺少关键 deterministic invariant 时新增最小 synthetic test。
- [x] 每个确认 Bug：先失败 test，再最小 production 修复，再 targeted + related suite。
- [x] 每个独立修复或覆盖补强用 Git commit 保存。

## Phase 4 — Development/Regression Production Paths

- [x] 运行公开 development/regression retrieval、conversation、state、semantic 等脚本；禁止 heldout。
- [x] 可用时运行 configured-provider E2E；不可用时记录 `BLOCKED_BY_ENVIRONMENT`。
- [x] 对不明失败最多一次有依据的重试并分类。

## Phase 5 — Final Gate, Build and Install

- [x] 重跑所有可执行总回归和质量检查。
- [x] 执行桌面 release/installer 编译，定位最终安装器。
- [x] 执行本机安装和启动级验证；记录安装路径、版本和失败分类。
- [x] 生成 `QA_CORE_REGRESSION_REPORT.md`，回答 25 个问题。
- [x] 运行 Trellis quality check，提交报告/修复，更新 journal 并归档任务。

## Validation Safety

- 执行任何 eval 脚本前先检查其 package command 和数据入口；名称或代码触及 heldout 时不运行。
- 不通过增加 timeout、重试次数或 token ceiling 掩盖失败。
- 不删除构建产物、未知 `.part` run、用户 stash 或未提交文件。
