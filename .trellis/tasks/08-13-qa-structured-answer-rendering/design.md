# 技术设计

## 数据契约

模型输出 JSON：`schemaVersion`、`sections[]`、`supplement[]`。每个 section 含固定 `title` 与 `groups[]`；group 含短 `label` 和 `claims[]`；claim 含 `text` 与 `evidenceIds[]`。

## 后端边界

新增 `structured_answer` 模块负责 fenced JSON 解包、serde 严格解析、结构校验、CitationValidation 构造和确定性 Markdown 渲染。Codex/API 且本轮有证据时只接受结构化契约；旧 Markdown parser 仅保留零证据、离线和旧记录兼容。

## 渲染

后端输出标准 Markdown：固定二级标题、可选短分组标题、声明文本及 `[E#]`，末尾自动追加“参考证据”。参考项仅显示证据编号、来源类型和 sourceLocation/page/wikilink 等短定位。前端按 evidence ID 稳定映射 8 色，并使用真实锚点交互打开来源。

## 兼容与回滚

不新增数据库列；持久化内容仍是 Markdown，审计包 EvidenceItem 保留完整元数据。关闭结构化 prompt/审计分支即可回滚，历史记录无需迁移。
