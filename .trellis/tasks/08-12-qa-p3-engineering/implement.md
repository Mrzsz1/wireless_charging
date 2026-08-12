# 实施计划

## 1. Graphify 性能与取消

- [x] 抽取 `qa/graph.rs`。
- [x] 预计算 haystack/token index。
- [x] 批量加载 pages 映射，移除逐节点 SQL。
- [x] 增加通道内部取消与缓存/等价回归。

## 2. 会话分页与搜索

- [x] 抽取 `qa/session.rs`。
- [x] 实现 session cursor/query page。
- [x] 实现 message cursor page 和批量 evidence。
- [x] 保留旧接口兼容适配器。
- [x] 注册新 Tauri commands 和 TypeScript service/DTO。
- [x] AskView 接入后端搜索、更多会话和更早消息。

## 3. 诊断与排序评测

- [x] 抽取 `qa/metrics.rs` 并接入各检索通道计时/计数。
- [x] 将 diagnostics 传到 stream/AskResult/UI。
- [x] 为 Gold 增加 Recall/MRR/NDCG/kind/pair 指标和阈值。

## 4. 引用模块拆分

- [x] 抽取 `qa/grounding.rs`，保持全部 claim/citation 回归。
- [x] 确认 `entailmentChecked=false` 和 UI 文案不变。

## 5. 验证与规范

- [x] Rust fmt / Clippy / full tests。
- [x] Node QA tests / TypeScript / Vite build。
- [x] Wiki Gold eval / Trellis validate / diff check。
- [x] 更新 QA code-spec、任务验收和 P3 审查状态。

## 6. 准确率与检索相关性整改

- [x] 将现有 Gold 标记为 development/regression，并增加生产契约 fixture/held-out 入口。
- [x] linked-paper 在目标论文内部按当前 query 选 section，fallback 显式降级。
- [x] Gold/回归测试区分 query-matched section 与 generic fallback。

## 7. Token-aware 上下文

- [x] 新增 `qa/context.rs` 与 `ContextPlan/ContextBudget`。
- [x] 历史按完整 exchange 保留，旧轮生成无旧 `[E#]` 的确定性 session memory。
- [x] 修窄指代触发，增加 self-contained contamination 回归。
- [x] 设置可配置 context window、近期轮次数和输出预留。

## 8. Prompt、Manifest 与回答门禁

- [x] Codex/API 共用六层 `PromptEnvelope` 与意图化 answer contract。
- [x] 增加 index snapshot、prompt/evidence SHA-256 和 `QaRunManifest`。
- [x] 升级 SQLite schema 并贯通 Rust DTO、Tauri、TypeScript 和历史分页。
- [x] 增加 answer completeness 与不新增事实的受限引用修复。

## 9. 前端科研审计

- [x] 显示 context token 分解、压缩轮次、snapshot、schema 状态。
- [x] 支持复制问题/答案/证据/manifest 审计包。
- [x] 更新离线模式为“证据浏览模式”一致文案。

## 10. 最终验证

- [x] Rust fmt / Clippy / full tests。
- [x] Node tests / TypeScript / Vite build。
- [x] Wiki eval / production fixture eval / Trellis validate / diff check。
- [x] 更新 QA contract、完成报告和最终审查结论。
