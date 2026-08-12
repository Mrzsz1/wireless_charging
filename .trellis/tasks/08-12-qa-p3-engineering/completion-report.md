# P3 工程优化完成报告

## 完成范围

1. **Graphify 检索**：抽取 `qa/graph.rs`，缓存预计算 haystack、token 倒排索引和 source 映射；每问一次性批量加载 pages，节点循环不再查询 SQLite；每 64 个候选检查取消。若任一 query term 无精确索引键，受控全扫保持原 substring 召回语义。
2. **历史分页**：抽取 `qa/session.rs`，新增稳定 session/message cursor、后端标题/消息正文搜索、最近消息首页和更早消息分页；历史证据按页面消息 ID 批量组装，旧完整接口继续可用。
3. **前端分页**：AskView 使用分页 commands，支持后端搜索、加载更多会话、加载更早消息、去重、过期请求隔离和前插后的滚动锚点保持。
4. **检索诊断**：抽取 `qa/metrics.rs`，`QuestionContext`、`retrieval_completed`、`AskResult` 和 UI 串通总耗时、通道耗时/候选数、选中数和取消检查点；序列化回归禁止正文、query、路径和凭据字段。
5. **引用边界**：抽取 `qa/grounding.rs`，原有 claim/citation 回归保持通过；`entailmentChecked` 仍唯一写为 `false`。
6. **排序评测**：Gold test 新增 Recall@5/10/20、MRR、binary nDCG@10、required-kind coverage 和 Wiki-primary pair coverage。

## 当前 Gold 排序基线

| 指标 | 当前值 | 门槛 |
|---|---:|---:|
| Recall@5 | 0.365 | 0.35 |
| Recall@10 | 0.555 | 0.54 |
| Recall@20 | 0.555 | 0.54 |
| MRR | 0.692 | 0.68 |
| nDCG@10 | 0.498 | 0.48 |
| required-kind coverage | 1.000 | 1.00 |
| Wiki-primary pair coverage | 0.800 | 0.80 |

门槛固定当前经复核基线，不把 10/10 的“至少一个预期 Wiki + 可定位 paper 命中”误报为全部标注目标的满召回。

## 质量门禁

- Rust `cargo fmt --all`：通过。
- Rust Clippy `--all-targets --all-features -- -D warnings`：通过。
- Rust full tests：74/74 通过。
- TypeScript `tsc --noEmit`：通过。
- Node tests：52/52 通过。
- Vite production build：通过。
- Wiki Gold answers：10/10 通过。
- Core-book retrieval：295 queries；两书 Recall@5 分别 1.000 / 0.986667，通过 95% 门槛。
- Trellis task validate：通过。
- `git diff --check`：通过（仅 Git 的 LF→CRLF 工作区提示）。

## 后续准确率与上下文整改

随后根据严谨科研审查继续完成 token-aware context、统一 PromptEnvelope、query-relevant paper section、`QaRunManifest`、answer completeness、审计 UI 与 held-out 评测入口。最新实现和质量结果见 `implementation-report.md`；其中覆盖并替代本报告内“聊天数据库 schema 不变”等早期边界。

## 明确未实施

- claim—evidence 语义蕴含、contradiction 或事实真实性模型。
- 在线 embedding、cross-encoder、付费 reranker。
- Raw/Wiki/Graphify 正文修改。
- commit / 发布。
