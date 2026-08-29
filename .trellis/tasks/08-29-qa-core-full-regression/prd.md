# QA 核心功能总回归

## Goal

在 baseline commit `4ef7b8f151732f41d61d4edebf244cdc91b4fbfa` 上，对桌面端核心研究问答能力执行可审计的全量回归，完成“复现 → 回归测试 → 最小修复 → 相关套件重跑 → 最终报告 → 编译安装”的闭环，证明现有 QA 能力在开发/回归场景下仍然正确、稳定、可用。

## Requirements

### R1 — 隔离与不可变约束

- 不读取、运行、分析或修改 Independent Held-out 数据、答案、curator 标签或 reviewer bundle。
- 不降低任何冻结质量阈值；Work Recall@20/Recall@10/MRR/nDCG@10 分别保持 0.95/0.90/0.85/0.85。
- Direct/Research/Exploratory token ceiling 保持 8,000/18,000/32,000，准入保持 `used + inFlight + new <= ceiling`。
- 不做性能优化、UI 重设计、架构重写、Release Artifact Traceability 或无关功能。
- 不删除、跳过、弱化失败测试，不通过增加 timeout/token budget 或减少产品能力制造 PASS。

### R2 — 回归层级

- 运行 QA 纯函数单元测试、模块集成测试、生产 pipeline synthetic 测试、公开 Development/Regression repository 测试。
- 在 provider 可用时运行真实 configured-provider E2E；不可用时明确记录 `BLOCKED_BY_ENVIRONMENT`，不把 fallback 当作完整 E2E PASS。
- 覆盖 Rust、Python、TypeScript/frontend QA 检查及最终安装构建。

### R3 — 功能矩阵

- A 普通知识库 QA：A01–A08。
- B 多轮上下文：B01–B06。
- C Research State 与状态操作：C01–C10。
- D Parameter State：D01–D10，`parameterStateCorruptionCount = 0`。
- E Research/Exploratory：E01–E10，并保持 method hypothesis 与 evidence 分离。
- F Retrieval/Evidence：F01–F10，覆盖 zero-evidence、唯一 Evidence ID、稳定来源与 checksum。
- G Citation/Claim：G01–G08，拒绝 unknown citation，保护 visible claim projection。
- H Semantic Verifier v2：Entailed/Contradicted/Unknown 及 scope/causality/temporal 等扩张语义。
- I Failure/Fallback：I01–I10，provider failure/cancellation 不污染 session/state。
- J Token Budget：J01–J11，覆盖 reservation reuse、并发、settle/release/drop/error/unwind、reconfigure 与真实超限拒绝。

### R4 — 缺陷闭环

- 只修复由稳定失败或静态契约证据确认的缺口。
- 每个生产 Bug 先添加最小失败回归测试，再做最小 production 修复并重跑相关 suite。
- 需要大规模设计的缺口记录为 Blocking Follow-up，不在本任务中重写系统。
- Bug 按 P0/P1/P2/P3 分类并记录症状、复现、根因、修复、测试和影响文件。

### R5 — Git、报告与安装

- 保持用户工作区安全；禁止 hard reset、clean、rebase、force push 和未知 stash 操作。
- 每个实际修改阶段用独立 Git commit 保存；本任务不自动推送远程。
- 生成根目录 `QA_CORE_REGRESSION_REPORT.md`，包含真实运行命令、数量、矩阵、缺陷、阻塞、环境、baseline/final commit 和最终结论。
- 编译可安装桌面应用并安装到本机；安装失败时保留构建证据并准确分类。

## Acceptance Criteria

- [x] AC1：环境、branch、baseline commit、toolchain、provider 和本地模型状态已记录。
- [x] AC2：QaRunManifest v21、Conversation/Research/Parameter、Semantic v2、Natural Answer v2、Evaluation v4 与 Rust/TS 字段静态契约无 drift。
- [x] AC3：A–J 十组矩阵均有来自实际测试的 PASS/FAIL/BLOCKED 数量和证据。
- [x] AC4：所有可执行 deterministic QA suites 通过，或存在可复现且明确记录的未修复阻塞。
- [x] AC5：公开 Development/Regression 生产路径通过，或真实环境/provider 阻塞被单独列出。
- [x] AC6：`parameterStateCorruptionCount = 0`，zero-evidence、unknown citation、Semantic Unknown/Contradicted、provider failure session safety、budget ledger 与真实超限契约均已验证。
- [x] AC7：`cargo fmt --check`、`cargo clippy --lib -- -D warnings`、QA Rust tests、Python regression、frontend QA tests、TypeScript build/type check 均有实际结果。
- [x] AC8：未使用 Independent Held-out，未修改 Frozen Threshold，未做性能优化。
- [x] AC9：最终报告回答任务书要求的 25 个问题，最终状态仅为 PASS、PARTIAL-BLOCKED 或 FAIL。
- [x] AC10：桌面应用完成 release 编译并执行本机安装验证。

## Out of Scope

- Independent Held-out 及任何 partial blind run。
- Release traceability、性能基准/优化、UI/CSS 重构、新功能、大规模架构改造。
- GitHub push。
