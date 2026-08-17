# 实施计划：智能问答原生结构化输出约束

## 1. 开始前

- [x] 范围确认为仅实现 Codex Provider；兼容 API 后续单独设计。
- [x] 审阅 PRD 与设计并获得实施批准。
- [ ] 检查工作区并创建修改前 Git 检查点。
- [ ] 加载 backend/frontend Trellis 规范。

## 2. 统一结构契约

- [ ] 从现有 section/role contract 生成完整合法 JSON 示例。
- [ ] 生成 Provider 可接受的 JSON Schema。
- [ ] 避免示例、Schema 和 Serde 结构三份手写定义漂移。
- [ ] 为文献意图四章节和其他意图六章节分别覆盖。

## 3. 提示词

- [ ] 用完整 JSON 示例替换当前压缩伪结构说明。
- [ ] 添加层级、唯一性、纯 JSON、证据 ID 使用规则。
- [ ] 更新提示词版本和相关审计字段。

## 4. Codex Provider

- [ ] 在临时工作区写入 JSON Schema。
- [ ] 扩展 `build_exec_args` 和 `stream_answer` 参数链。
- [ ] 传递 `--output-schema`。
- [ ] 保持取消、超时、进程树终止和临时目录清理。

## 5. 审计与错误

- [ ] run manifest 记录实际结构化输出模式。
- [ ] 区分 Provider Schema、JSON 解析和业务校验错误。
- [ ] 确保失败回答不进入可信多轮上下文。

## 6. 验证

- [ ] 验证完整示例可被现有解析器接受。
- [ ] 验证错误嵌套、未知字段、重复章节、未知 role/evidence ID 被拒绝。
- [ ] 验证 Codex 参数包含 `--output-schema`。
- [ ] 验证兼容 API 与离线通道未被改变。
- [ ] 运行相关 Rust/前端定向检查。
- [ ] 完成桌面应用编译。
- [ ] 检查 Git diff 并提交任务改动。

## 7. 风险文件与回滚点

- `apps/desktop/src-tauri/src/qa/context.rs`：提示词与动态契约。
- `apps/desktop/src-tauri/src/qa/structured_answer.rs`：结构解析契约。
- `apps/desktop/src-tauri/src/codex_subscription.rs`：Codex CLI 参数和临时文件。
- `apps/desktop/src-tauri/src/lib.rs`：Provider 调用参数链。

Codex Provider 约束形成独立可回滚提交，避免同时改动兼容 API、检索或渲染逻辑。
