# 自然回答、末尾证据与 Markdown 深链

## Goal

让最终用户看到自然、可读的科研回答，而不是 Provider JSON 或固定模板校验错误；系统根据真实选中 ContentBlock 在回答末尾生成短证据链接，并能打开对应 Markdown 精确定位。

## Requirements

### A1 自然 Markdown 输出

- Answer Provider 直接输出 Markdown 正文。
- 不要求固定中文章节、固定 claim 数、逐 claim evidenceIds 或 `qa-structured-answer-v1`。
- 回答应先直接回答，再说明模型/方法、边界或未覆盖信息；具体标题由问题和模型自然决定。

### A2 边界表达

- 模型只依据提供的证据陈述库内事实。
- 允许模型补充一般知识，但必须置于明确“可能不准确/未由本库核验”的区域。
- 零证据状态明确显示且不生成伪来源。
- 不实现引用语义蕴含检查。

### A3 后端证据附录

- 后端从 selected evidence 生成 `参考证据`，不信任模型提供的 ID/路径。
- 每条显示短标题、来源类型、heading path；隐藏过长文件名和绝对路径。
- 附录证据 ID 与持久化 EvidenceItem/SourceLocator 一致。
- 未被选择或已失效的 locator 不能成为可点击证据。

### A4 Markdown 深链

- 点击 Wiki、paper Markdown 和 book Markdown 使用统一 open locator 行为。
- 优先 block ID，其次 heading path，最后 line range。
- 定位变化时显示可理解的降级提示和文档级打开，不静默跳错位置。

### A5 结构校验替换

- 删除最终 Markdown 的固定短语、章节数组、intent element 和 minimum claim count 阻断。
- 保留 JSON/Markdown 安全、未知引用清理、路径边界、证据存在性和零证据状态校验。
- 旧会话的 structured answer 仍可读取和渲染。

### A6 UI

- 证据附录位于回答末尾；右侧证据面板继续提供详细片段和检索理由。
- 引用采用当前软件视觉风格，不展示原始超长文档名。
- 完整 audit 默认折叠，简要显示来源范围、检索轮数和停止原因。

## Acceptance Criteria

- [ ] AC1：Provider 返回普通 Markdown 时完成状态成功，不再报 STRUCTURED_ANSWER_VALIDATION_FAILED。
- [ ] AC2：没有固定“结论/模型与方法/证据边界”等标题仍可成功，只要内容非空且安全。
- [ ] AC3：模型输出未知 `[E99]` 不会变成有效链接；后端附录只包含已选 locator。
- [ ] AC4：用户只看到短链接标题，点击可打开对应 Markdown block。
- [ ] AC5：PDF 字段为空时 book/paper Markdown 链接正常。
- [ ] AC6：block 编辑漂移时按 heading/line fallback，并显示定位降级状态。
- [ ] AC7：零证据回答带未核验标识、不带证据附录且不进入 trusted history。
- [ ] AC8：旧 structured answer 会话和旧 `[E#]` 消息仍可打开。
- [ ] AC9：前端证据测试、QA 设置测试、build 和 release cargo build 通过。

## Out of Scope

- 语义蕴含和逐 claim citation coverage。
- 重新实现检索或 embedding。
- 修改知识库 Markdown 正文。
