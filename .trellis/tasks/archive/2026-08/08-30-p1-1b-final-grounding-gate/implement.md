# P1-1B Final Grounding Gate — Implementation Plan

## Phase A — Baseline and confirmed-gap audit

- 追踪 `audit_generated_answer_with_semantic`、claim repair、citation validation、persistence 与 trusted history 的真实调用路径。
- 记录当前 gate 读取 draft report 的具体位置和现有测试覆盖。
- 只在确认缺口后修改。

## Phase B — Shared notice and final audit core

- 增加共享 system notice 分类。
- 修正 PartiallySupported repair，不保留完整原 claim。
- 实现 deterministic Final Audit、supported draft mapping、unknown citation 校验和 insufficiency 状态。
- 增加核心回归测试并提交。

## Phase C — Gate, persistence, manifest, trusted history

- 让最终 CitationValidation/grounding status/persistence 只读取 Final Audit。
- Draft Audit 继续进入诊断字段；Final Audit 进入运行清单与 E2E 报告。
- 排除 system notice 与 insufficiency answer 的 trusted-history 资格。
- 增加集成回归测试并提交。

## Phase D — Coverage-aware answer contract

- 扩展 answer contract 上下文。
- Direct 采用最小直接回答优先；Research/Exploratory 只要求证据覆盖要素。
- 增加 Direct contract 回归测试并提交。

## Phase E — Focused verification and real rerun

- 运行 fmt、相关 Rust tests、clippy 与必要构建，不做无关的重复全量测试。
- 先跑 Research 真实链路，再跑 Direct 真实链路。
- 使用仓库外临时诊断核对 claim/evidence 对齐，随后删除。
- 更新任务记录与必要 spec，创建阶段提交并汇报剩余阻塞。
