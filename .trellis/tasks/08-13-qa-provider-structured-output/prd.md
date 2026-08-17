# 智能问答原生结构化输出约束

## Goal

在保留当前 Rust/Tauri 自研多轮 RAG 架构的前提下，先为 Codex 订阅智能问答增加完整的结构化回答示例和 Provider 原生 JSON Schema 约束，降低模型生成非法层级、重复章节、未知字段或未闭合 JSON 的概率，同时继续由后端执行证据与业务规则校验。

## Background

- 当前智能问答没有使用 LangChain、LlamaIndex、Haystack 或 Semantic Kernel。
- 当前链路由 React 聊天界面、Tauri/Rust 后端、本地多通道检索、证据包、Codex CLI / OpenAI-compatible Provider、Serde 结构解析和审计持久化组成。
- `apps/desktop/src-tauri/src/qa/context.rs:597-607` 当前只在提示词中提供压缩结构描述，没有提供一个覆盖全部嵌套层级和必需 role 的完整 JSON 示例。
- `apps/desktop/src-tauri/src/codex_subscription.rs:735-767` 当前调用 `codex exec --json`，尚未传入 `--output-schema`。
- `apps/desktop/src-tauri/src/qa/structured_answer.rs:9-44` 已通过 Serde `deny_unknown_fields` 对最终结构执行严格解析；这层校验必须保留。
- 已观察到的失败包括：Section 被错误嵌套到 Group、重复 Section ID、未知字段和 JSON 未闭合。

## Requirements

### R1. 完整 JSON 示例

- 有证据回答的提示词必须包含一个完整、合法的 `qa-structured-answer-v1` JSON 示例。
- 示例必须覆盖 root、sections、groups、claims、supplement 的完整层级。
- 示例必须覆盖当前意图的全部必需 section，并至少演示当前意图允许的全部必需 claim role。
- 示例明确说明：实际 `evidenceIds` 只能使用本轮证据包中的编号，示例编号不是固定业务数据。
- 提示词必须明确禁止 Section 嵌套进 Group、重复 Section、Markdown 代码围栏以及 JSON 前后解释文字。

### R2. 单一契约来源

- 完整示例、Provider JSON Schema 和 Rust 解析器必须以同一套结构契约为来源，避免三份手写结构长期漂移。
- 文献意图维持四个 section；其他意图维持现有六个 section。
- Provider Schema 负责结构约束；Rust 后端继续负责 section 顺序、唯一性、role 完整性、证据编号存在性和引用业务规则。

### R3. Codex 订阅原生约束

- Codex 订阅通道为每次隔离运行生成或写入 JSON Schema 文件。
- 调用 `codex exec` 时传递 `--output-schema <schema-path>`。
- Schema 文件必须位于本轮临时工作区，并随临时工作区清理。
- 不改变现有 read-only sandbox、never approval、ephemeral、取消和超时行为。

### R4. 审计与错误可见性

- 审计信息记录 Codex 实际使用 `codex-output-schema`。
- Provider 结构约束失败与后端业务校验失败必须保持可区分错误码。
- Provider 即使返回合法 JSON，也必须继续经过现有 Serde、引用和完整性审计。

### R5. 兼容性

- 保留现有三种回答 Provider：Codex 订阅、兼容 API、本地确定性回答。
- 不迁移当前检索、上下文、会话、证据或审计架构。
- 前端现有模型、推理强度、thinking 状态和回答渲染行为不因本任务退化。

## Acceptance Criteria

- [x] 提示词包含完整 JSON 示例，示例可以被 `serde_json` 与现有结构解析器接受。
- [x] 示例覆盖当前意图的全部 section 和必需 role，且 Group 中不允许出现 `id`、`title`、`groups` 等 Section 字段。
- [x] Codex 调用实际携带 `--output-schema`，临时 Schema 文件在运行结束后被清理。
- [x] 合法结构仍会执行引用、证据 ID、section、role 和完整性校验。
- [x] 对 Section 嵌套到 Group、重复/错误 Section ID、未知 role、未知 evidence ID、未闭合 JSON 均产生可定位错误。
- [x] 未新增 LangChain、LlamaIndex、Haystack、Semantic Kernel 或独立 Python/Node Provider 服务依赖。
- [x] 修改代码前创建 Git 检查点；任务完成后按项目规范编译桌面应用。

## Out of Scope

- 引入 LangChain 或迁移现有 Rust RAG 架构。
- 修改兼容 API 的请求结构或为其增加 `response_format.json_schema`；该通道后续单独设计。
- 实现“引用语义是否蕴含结论”的语义判定。
- 重写本地混合检索、Graphify 或证据排序算法。
- 改变知识库内容或 Raw/Wiki/Schema 三层权限规则。
