# QA 核心功能总回归技术设计

## Boundaries

本任务以现有测试和公开 development/regression fixtures 为主，新增内容仅用于填补已确认的 deterministic contract coverage。不得接触任何 heldout 数据路径。生产代码只有在失败回归证明缺陷后才修改。

## Regression Architecture

1. **Baseline gate**：记录 Git/toolchain/provider/local-model 状态，确认工作区无用户修改。
2. **Static contract gate**：比对 Rust manifest schema、TypeScript types、frontend consumers 和固定阈值/预算常量。
3. **Deterministic gate**：运行 Rust QA unit/integration、Python synthetic regression、frontend QA tests 和 TypeScript build。
4. **Matrix mapping**：将实际 test names 映射到 A–J；只在现有测试不足且能用 synthetic deterministic case 验证时新增测试。
5. **Production-path gate**：运行公开 development/regression repository pipeline；任何可能触发 heldout 的脚本均先检查入口并排除。
6. **Provider gate**：探测 configured provider；可用时运行真实 E2E，不可用时记录环境阻塞，不修改 production fallback。
7. **Build/install gate**：release 编译桌面安装包，使用生成的安装器进行本机安装并记录结果。
8. **Reporting gate**：从命令输出和测试清单生成 `QA_CORE_REGRESSION_REPORT.md`，不预填结果。

## Contract Preservation

- Budget：`tokenCostUsed + tokenCostInFlight + newReservation <= tokenCostCeiling`；8k/18k/32k 不变。
- Retrieval thresholds：0.95/0.90/0.85/0.85 不变。
- State：当前轮 patch 先于 `ResearchQueryContext` 和 retrieval；destructive low-confidence 操作 fail closed。
- Evidence：answer citation 必须属于当前完整 evidence set；zero-evidence 不伪造来源。
- Semantic：unsupported expansion 默认 Unknown，只有明确反对才 Contradicted。
- Rendering：Natural Markdown v2 用户可见行为不因 evaluation infrastructure 改变。
- Failure atomicity：provider failure/cancellation 不提交半成品 message pair、state 或 parameter mutation。

## Bug Fix Strategy

每个缺陷单独形成最小测试和最小修复。测试先在旧实现上失败；修复后运行 targeted suite，再回到总回归。若不能在当前边界内安全修复，则停止相关路径并记录 Blocking Follow-up。

## Compatibility and Rollback

- 不迁移 frozen data，不改已有公开 schema，除非失败证明 schema drift；任何兼容修改必须同步 Rust/TS/frontend tests。
- 每个修复逻辑独立 commit，可通过普通 `git revert <commit>` 回滚。
- 安装前保留构建产物路径；不删除旧安装器和用户数据。

