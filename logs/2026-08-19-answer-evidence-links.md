# 自然回答、后端证据附录与 Markdown 深链

## 目标

移除最终智能问答对固定中文章节、claim 数量和 Provider 证据 ID 的依赖，让模型输出自然 Markdown；可信来源由后端根据本轮选中的 Markdown ContentBlock 确定性追加，并可打开到仓库内原文位置。

## 已实现

- 回答契约升级为 `qa-natural-markdown-v2`，最终 Codex/compatible API 调用不再提交回答 JSON Schema；检索 Planner 仍独立使用 Provider 原生 Schema。
- 新增自然回答安全归一化：移除模型伪造的 `[E#]` 与 `参考证据`、阻止不可信协议链接、隐藏可见绝对路径，并保留零证据未核验语义。
- 后端仅从本轮已选择且带有效 `SourceLocator` 的非 Graph evidence 生成短 `参考证据` 链接；`CitationValidation` 通过 appendix integrity/IDs 描述来源完整性，不宣称语义蕴含已核验。
- `EvidenceItem` 和 `qa-run-v5` 持久化 locator/answerFormat；旧 EvidenceItem、旧 manifest 和 `qa-structured-answer-v1` 继续可读。
- 新增 `read_source_locator` 命令与内部只读 Markdown 来源视图，按 block → heading → line → document 降级定位并显示原因；MarkdownReader 自动滚动和高亮目标。
- QA 回答页显示自然流式 Markdown，证据链接显示短标题，不在可见界面暴露超长文件名或绝对路径。旧 `[E#]` 仍可导航。
- trusted history 在入库前移除模型补充和后端证据附录，失败/未核验/取消轮次仍不进入后续上下文。

## 兼容与回滚

- 生产默认 `natural-markdown-v2`。
- `LUNAWIKI_RAG_ANSWER_V2=false`（兼容 `RAG_ANSWER_V2` / `rag_answer_v2`，以及 `0/off/no`）恢复 legacy generation。
- 旧 structured parser 暂不删除；只有在灰度评测完成、历史消息迁移和回滚窗口结束后才能移除。

## 验证

- Rust：162 passed，1 ignored；`cargo fmt --check` 和 `cargo clippy --all-targets -- -D warnings` 通过。
- 前端 QA evidence：5/5；QA settings：8/8；TypeScript/Vite production build 通过。
- 准确率审计工具：12/12。
- Rust release build：通过。

## 边界

- 不实现引用语义蕴含或逐 claim coverage。
- 不修改检索、embedding、知识库 Markdown 正文或用户未跟踪的自动检索结果。
