# 自然回答、证据附录与深链实施计划

## Phase 0 — 前置

- [ ] 确认 ContentBlock/SourceLocator 和 v2 retriever 已冻结。
- [ ] Git 检查点；保存当前 structured answer/citation error fixtures。
- [ ] 明确旧会话兼容测试，禁止直接删除历史 parser。

## Phase 1 — Answer prompt 与 provider

- [ ] 新建 natural Markdown prompt/version。
- [ ] Codex/compatible API 不再为最终回答提交 answer JSON Schema；planner 仍使用原生 schema。
- [ ] 流式 parser 只收集 answer text，保持 timeout/cancel/child cleanup。
- [ ] Prompt 明确系统追加证据、模型不得伪造路径/ID。

## Phase 2 — 最低校验

- [ ] 新建 v2 AnswerValidation。
- [ ] 从成功门移除 fixed sections、intent elements、claim count、逐 claim citation coverage。
- [ ] 清理/转义未知 `[E#]`、绝对路径和不受信协议。
- [ ] 保持 zero evidence、failed、cancelled、unverified/trusted context 语义。

## Phase 3 — EvidenceAppendixBuilder

- [ ] 从 selected ContentBlock 生成短标签和 EvidenceLink。
- [ ] 确定性追加末尾 `参考证据`，处理已有同名标题避免重复。
- [ ] 持久化 answerFormat、links 和 locator snapshot。
- [ ] 旧 EvidenceItem 反序列化补默认 locator。

## Phase 4 — Tauri/TypeScript 深链

- [ ] 新增 resolve/open locator command 和 TS types/service。
- [ ] MarkdownReader 支持 block/heading/line scroll 与高亮。
- [ ] AskView/MarkdownMessage 点击证据使用结构化 locator。
- [ ] 定位漂移显示 degraded 文案；路径越界 fail closed。

## Phase 5 — UI 和历史兼容

- [ ] 新回答渲染自然 Markdown + 末尾附录。
- [ ] 旧 structured-v1 使用 legacy renderer。
- [ ] 右侧 evidence/audit 保持可访问；短标题适配窄屏。
- [ ] 复制回答默认包含可读证据标题，不包含绝对路径。

## Phase 6 — 验证与编译

- [ ] Fixture：普通 Markdown、空输出、未知 E99、伪 file link、零证据、partial evidence、旧 structured JSON。
- [ ] Locator：block、heading fallback、line fallback、document fallback、path traversal。
- [ ] `cargo fmt --check`。
- [ ] `cargo test grounding --lib`（更新为 appendix integrity）和 answer/locator 测试。
- [ ] `npm run test:qa-evidence`。
- [ ] `npm run test:qa-settings`。
- [ ] `npm run build`。
- [ ] `cargo build --release`。
- [ ] 提交并记录旧 schema 删除的条件。

## 回滚

- `rag_answer_v2=false` 恢复旧 provider/renderer。
- 新消息保留 answerFormat，回滚后仍可用 generic Markdown renderer 阅读。
- 不删除旧 structured parser，直到最终 rollout 完成。
