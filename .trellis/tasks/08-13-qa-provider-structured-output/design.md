# 技术设计：智能问答原生结构化输出约束

## 1. 设计结论

保留当前 React + Tauri/Rust + 本地混合检索 + Provider Adapter 架构，不引入 LangChain。新增一个由 Rust 契约生成的结构化输出描述，同时服务于：

1. 提示词中的完整 JSON 示例；
2. Codex CLI 的 `--output-schema`；
3. 现有后端解析和审计测试。

Provider Schema 是生成阶段的结构约束，不替代后端业务校验。

## 2. 当前数据流

```text
AskView
  -> Tauri ask command
  -> prepare_question_with_history_and_budget
  -> 最多三轮 Wiki / paper / book / graph 检索
  -> EvidenceItem[E1..En] + PromptEnvelope
  -> Codex CLI | compatible API | offline
  -> structured_answer::parse_and_render
  -> citation / completeness / audit
  -> SQLite conversation + audit bundle
```

## 3. 目标数据流

```text
AnswerContract(intent)
  ├─ complete_json_example(intent)
  ├─ provider_json_schema(intent)
  └─ backend_validation_contract(intent)

PromptEnvelope + complete_json_example
  -> Provider adapter + provider_json_schema
  -> raw structured JSON
  -> Serde deny_unknown_fields
  -> section/role/evidence/citation/completeness validation
  -> Markdown projection + evidence links
```

## 4. 契约边界

### JSON Schema 负责

- 根对象和必需字段；
- 数组/字符串类型；
- `additionalProperties: false`；
- Section、Group、Claim 的合法层级；
- `schemaVersion` 常量；
- section 数量边界；
- role 枚举；
- evidence ID 的基本格式。

### Rust 校验继续负责

- section ID 的准确顺序与唯一性；
- 当前意图要求的全部 role 是否出现；
- evidence ID 是否属于本轮证据包；
- 至少一个非 Graphify 证据；
- claim、citation、完整性与零证据业务逻辑；
- Markdown 渲染和超链接投影。

这样可以避免依赖不同 Provider 所支持的 JSON Schema 子集差异。

## 5. Codex Provider

- 在现有 `TempWorkspace` 中写入 schema 文件。
- `build_exec_args` 接收 schema 路径并追加：

```text
--output-schema <temporary-schema-path>
```

- 保留 `--json`，因为它控制 CLI 事件流；`--output-schema` 控制最终回答结构，两者职责不同。
- Schema 写入失败、CLI 不接受参数、最终解析失败分别使用不同错误前缀。

## 6. 完整示例策略

- 示例由当前 intent 的 section/role contract 构造，不手写两套长期分叉样例。
- 文本使用明显的占位事实，不包含具体论文和固定业务关键词，避免诱导模型照抄。
- 示例 evidence ID 使用 `E1`，同时提示只能替换为本轮存在的证据编号。
- 示例是合法 JSON 字符串，不使用伪代码中的 `string` 类型占位符。

## 7. 审计扩展

在 run manifest 中记录结构化输出模式：

- `codex-output-schema`
- `offline-deterministic`

本任务只改变 Codex 通道；兼容 API 和离线通道的执行方式保持不变。

## 8. 回滚

- Codex 可回滚为移除 `--output-schema` 参数，同时保留提示词示例和后端校验。
- 所有新能力位于 Provider 边界和契约生成模块，不改动检索数据结构。
