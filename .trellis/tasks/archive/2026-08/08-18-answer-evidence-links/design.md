# 自然回答、证据附录与深链技术设计

## 1. 回答管线

```text
QuestionContext + selected ContentBlocks
 -> AnswerPromptBuilder
 -> Provider natural Markdown stream
 -> sanitize unknown citation-like tokens
 -> minimal AnswerValidation
 -> EvidenceAppendixBuilder(selected evidence)
 -> persist body + appendix metadata + audit
 -> UI MarkdownMessage + EvidenceAppendix
```

Provider prompt 明确“不要生成参考证据列表、文件路径或证据 ID，系统会追加”。这样避免模型伪造 locator，也避免模型为了满足复杂 JSON 而出错。

## 2. 最低校验

`AnswerValidation` 只检查：非空/合理长度、流结束状态、无未知可执行链接、无仓库越界路径、零证据声明、Provider 错误。旧 citation coverage/claim count/固定章节 validation 从主成功条件移除，但可保留 legacy audit 字段用于读取历史。

## 3. EvidenceAppendix

结构化数据：

```text
EvidenceLink {
  evidenceId, shortLabel, kind, headingPath,
  locator, available, degradedReason?
}
```

短标题规则优先 canonical title + 最深有意义 heading，设字符上限；不得显示绝对路径。Markdown 文本和结构化 links 同时持久化，UI 点击使用结构化 locator，不解析 href 中的任意文件路径。

## 4. 深链解析

Rust command `resolve_source_locator` 返回：resolved path、matchedBy(block|heading|line|document)、heading、line range、content hash match。前端调用现有 `onOpenPage/onOpenBook/onOpenPath` 的统一 adapter；MarkdownReader 支持 scroll target/highlight。

## 5. 兼容

- `answerFormat = natural-markdown-v2 | structured-v1 | legacy-markdown` 写 run manifest。
- 历史 structured JSON 继续通过旧 renderer；新请求不再生成 structured-v1。
- `CitationValidation` 类型字段先保留，v2 将其状态映射为 appendix_integrity，而非 claim coverage。

## 6. 安全与可信上下文

- 模型正文不允许创建 `file://`、绝对盘符路径或任意 app protocol。
- EvidenceAppendix 的 locator 必须来自本轮后端候选并通过仓库边界检查。
- unverified/failed/cancelled 不进入可信多轮记忆；partial evidence 的自然回答可进入时必须保留边界摘要和来源快照。

## 7. UI 状态

默认显示回答与附录。右侧面板显示 snippet、role、来源类型、heading path、retrieval reason。诊断折叠区显示 channel attempts、round count、degraded channels、stop reason，不显示模型隐藏推理过程。
