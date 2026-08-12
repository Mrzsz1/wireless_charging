# 实施计划：智能问答第二轮可靠性修复

## 0. 执行顺序与门禁

本任务使用 Codex inline 模式，由主会话直接实现和检查；不创建 implement/check 子代理。严格按以下顺序推进，每组先写失败回归，再改实现。任何 repository 隔离、引用契约或检索基线回退都应停止后续发布步骤并回滚当前组。

## 1. 契约与纯逻辑测试

- [ ] 扩展 Rust/TypeScript `AskRequest`，增加必填 client-generated `requestId`；补 UUID、空值和重复 active ID 测试。
- [ ] 扩展 `ChatMessage.status` 为 `unverified`，扩展 `CitationValidation` 的 grounding 字段并保持旧 JSON 可读。
- [ ] 定义 `RetrievalQuery`、带 ID 的 `ConversationTurn` 和 `FailedExchange`/failed event DTO。
- [ ] 在 `completionState.ts` 增加 request ID 创建/交换合并/failed 与 unverified 精确重试纯函数测试。
- [ ] 增加 grounding 矩阵测试：有证据合法引用、缺引用、未知引用、零证据固定声明、零证据伪 `[E#]`。

主要文件：

```text
apps/desktop/src/types.ts
apps/desktop/src/features/qa/completionState.ts
apps/desktop/tests/p1-state.test.ts
apps/desktop/src-tauri/src/qa.rs
```

## 2. 历史感知 RetrievalQuery

- [ ] 让 `conversation_history()` 返回 message ID/request ID，继续限定 repository、completed-only、8 条和 12,000 字符。
- [ ] 实现指代触发器、近期实体抽取、pages 标题匹配、`[E#]` 过滤、实体/字符预算和 `resolved_question`。
- [ ] 将问答路径改为先加载历史，再构造 query，再检索；保留无历史的 `prepare_question`/研究脉络兼容入口。
- [ ] Prompt 继续使用 original question；history 明示非 evidence；检索只使用结构化 resolved query。
- [ ] 增加 CCSP/GAIN 追问回归、旧引用排除、failed/unverified 历史排除和无指代不扩写测试。

回滚点：query rewrite 独立于原 `query_terms`；若新召回异常，可关闭历史补全并保留 DTO/测试基础。

## 3. Graphify 搜索索引和缓存

- [ ] 抽取 Graph JSON 解析为 `GraphSearchIndex`，兼容 `links|edges`、snake/camel source 字段和缺失可选字段。
- [ ] 搜索文档纳入 node description、source location、community/name、relation 和 neighbor label。
- [ ] 分别计算 node/relation/neighbor hit，更新 score、relation 和 retrieval reason。
- [ ] 保持中心节点真实 Wiki 文件 + SQLite page ID 双重安全闸门。
- [ ] 在 `AppState` 增加按 repository/path/mtime/length 失效的独立缓存；缓存解析在 blocking worker 中完成。
- [ ] 测试 node-only、relation-only、neighbor-only、community/source-location、无回链过滤、文件变化失效和 malformed graph 降级。

主要文件：

```text
apps/desktop/src-tauri/src/qa.rs
apps/desktop/src-tauri/src/lib.rs
```

## 4. 请求生命周期、blocking retrieval 与取消

- [ ] 前端提交前生成 request ID，并立即设置 state/ref、optimistic user 和可用停止按钮；请求体携带相同 ID。
- [ ] 后端在检索前校验/登记 ID，生成 reserved session ID，发送 `started` 与 `retrieval_started`。
- [ ] 增加 early-cancel tombstone、重复 active ID 拒绝、过期清理和统一终态 cleanup。
- [ ] 从 repository connection 快照 `root + db_path + repositoryId`；blocking worker 打开独立 SQLite 读连接完成 settings/history/query/retrieval。
- [ ] 在检索通道、Graphify、Codex status、provider、grounding 和 persistence 边界检查取消。
- [ ] 增加 ask-time Codex 30 秒 TTL cache；设置页显式刷新绕过缓存；offline/API 不探测。
- [ ] 补前端即时停止、切库前 started 取消、后端早到取消、重复 ID、大 graph worker 不占 repository mutex 的回归。

风险文件：

```text
apps/desktop/src/features/qa/AskView.tsx
apps/desktop/src/services/desktop.ts
apps/desktop/src-tauri/src/lib.rs
apps/desktop/src-tauri/src/codex_subscription.rs（仅在需要可测试探测接口时修改）
```

回滚点：保留 client request ID 和即时 UI；blocking worker/cache 可单独回滚，但不得恢复检索前不可取消状态。

## 5. 零证据 unverified 回答

- [ ] 添加 server-owned 固定无来源声明和 no-evidence Codex/API prompt；offline 返回确定性提示。
- [ ] 零证据 Codex/API仍调用所选模型，但禁止本库依据声明和 `[E#]`；最终规范化伪引用标记。
- [ ] `persist_exchange` 根据 grounding 分支保存 completed 或 unverified 成对交换；unverified user/assistant 均不进入 completed-only history。
- [ ] AskView 显示“无参考来源 · 未验证”，保留模型正文、历史打开和重试，证据面板为 0。
- [ ] 测试 Codex/API/离线三种零证据语义、固定声明不可由模型省略、无伪引用、保存/重开、后续历史排除。

回滚点：若远程 no-evidence prompt 不稳定，服务器固定声明和状态分型仍必须保留；可仅回退模型正文生成而不伪装 supported。

## 6. 失败交换与精确重试

- [ ] 将 `persist_failure` 替换为 `persist_failure_exchange`：首轮创建 reserved session，既有会话复用，同事务插入 failed user/assistant。
- [ ] 统一处理检索、Codex/API、引用验证和持久化前的 post-start 失败；取消/repository changed 不保存。
- [ ] failed event 携带可选持久化 exchange；前端有 exchange 时替换 optimistic message，无 exchange 时回滚。
- [ ] `retryQuestionFor` 优先匹配同 request ID 的相邻 user，支持 completed/failed/unverified 并保留 legacy fallback。
- [ ] 测试首轮失败、既有会话失败、刷新后展示、精确重试、failed/cancelled 不进入历史。

## 7. 跨层规范和版本同步

- [ ] 更新 `.trellis/spec/backend/qa-contract.md`，记录 client request ID、RetrievalQuery、unverified、失败成对持久化、blocking/cancellation 和 Graph cache 契约。
- [ ] 若形成稳定前端状态约定，补充 `.trellis/spec/frontend/` 对应 current 规范；不填写与本任务无关的模板章节。
- [ ] 同步 `0.12.3`：`package.json`、`package-lock.json`、`Cargo.toml`、app package `Cargo.lock`、`tauri.conf.json`、更新 manifest fixture 和 `verify-config.mjs`。
- [ ] 添加 0.12.3 release notes/变更记录；不修改 Raw/Wiki/Graphify 正文。

## 8. 定向验证

在 `apps/desktop`：

```powershell
npm run test:p1
npm run test:qa-settings
npm run build
cargo test qa::tests --manifest-path src-tauri/Cargo.toml
```

重点断言：

- CCSP/GAIN 追问的 RetrievalQuery 和双实体 evidence；
- zero-evidence `unverified` 与 supported/invalid 分型；
- immediate cancel、early cancel、duplicate ID、repository switch；
- first-turn failed exchange 与重试；
- Graphify relation-only/neighbor-only/cache invalidation。

## 9. 全量质量门禁

在 `apps/desktop`：

```powershell
npm run verify:p3
npm run verify:p5

cd src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

在仓库根目录：

```powershell
py -3 tools/wiki_lint.py
git diff --check
git status --short
```

验收基线：Wiki/paper 10/10；Algorithmic Game Theory Recall@5 ≥ 0.95；Approximation Algorithms Recall@5 ≥ 0.95；既有 completed 幂等、repository 隔离、原文定位和书籍页码测试全部通过。

## 10. 敏感信息与差异审查

- [ ] 审查 staged 文件列表和完整 staged diff。
- [ ] 执行秘密扫描，重点检查 API key/token/cookie/authorization/password/secret；排除构建目录和已知第三方锁文件噪声后人工复核所有命中。
- [ ] 确认未跟踪交接文档由用户拥有，不擅自纳入业务提交，除非用户另行要求。
- [ ] 确认未改 `raw/`、`wiki/`、`schema/vocab.yaml`、B 类页面和本机认证文件。

## 11. 发布、安装与启动验证

- [ ] 在所有测试通过后提交代码与 Trellis 任务变更。
- [ ] 执行 release 构建，确认 0.12.3 MSI 与 NSIS 文件名和产品元数据。
- [ ] 静默安装 NSIS；验证卸载注册表 `DisplayVersion=0.12.3`、安装 EXE ProductVersion 和主窗口可启动/响应。
- [ ] 运行严格 GUI smoke，至少覆盖问答发送、即时停止、历史失败/unverified 展示和设置入口。
- [ ] 记录产物路径、大小、SHA-256 和安装验证结果；生成产物不加入 Git。
- [ ] 最终更新任务结果、journal，按 Trellis 流程提交并归档。

