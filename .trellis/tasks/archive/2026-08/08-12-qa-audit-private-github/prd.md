# 智能问答审查与私有 GitHub 发布

## 目标

1. 把 2026-08-12 对客户端智能问答的代码审查结论固化为可执行任务。
2. 审计当前 Git 工作树与全部待推送历史，排除凭据、个人配置、运行产物和不适合进入 GitHub 的敏感信息。
3. 在当前已登录的 GitHub 账户下创建私有仓库并推送当前项目，保留现有 Git 历史。

## 智能问答现状

当前链路为“问题规范化与意图识别 → Wiki/论文原文/核心书籍/Graphify 多路召回 → 确定性评分和 E# 编号 → Codex 订阅、兼容 API 或离线证据回答 → 会话/证据/库水位事务持久化”。核心实现位于 `AskView.tsx`、`qa.rs`、`codex_subscription.rs` 与 Tauri `ask_luna` 编排。

### 已有能力

- 会话创建、搜索、重命名、删除、流式回答、停止、复制、重试和证据定位。
- Wiki FTS5、canonical 论文原文章节、两本核心书籍和 Graphify 节点四类证据。
- Codex ChatGPT 订阅、OpenAI-compatible Chat Completions SSE、仅离线证据三种引擎。
- 编号引用、库水位、知识库隔离、事务保存、Codex 只读临时工作区和凭据不落库。
- 现有 P3 门禁通过：QA 设置 4/4、Rust 56/56、Gold Contract 10/10、两书 Recall@5 1.000 / 0.986667。

### P0 问题

1. 会话历史未进入回答 Prompt，界面是多轮会话，生成逻辑仍是单轮问答。
2. 只有前端未知引用提示，没有后端引用完整性、事实覆盖率或证据支持校验。
3. `solve/novelty/relationship` 意图只写入 Prompt，没有控制检索权重和来源配额。
4. 生成期间切换知识库缺少 repository generation 校验和主动取消，存在旧请求结果进入新界面的竞态。

### P1 问题

1. 历史回答的重试按钮统一使用最后一个用户问题。
2. 取消或失败后乐观插入的本地消息未清理或标记。
3. 初始化和每次提问会重复探测 Codex 状态，离线/API 模式也承担探测延迟。
4. Graphify 只做节点文本匹配，未使用边、路径、邻居、社区和来源位置硬门槛。
5. Graphify 的 `sourcePath` 未转换为 Wiki page ID，打开来源可能失败。
6. 远程模型失败被保存为普通 completed 离线回答，失败原因和主动离线模式未结构化区分。

### P2 优化与扩展

- BM25 + 本地 embedding 混合召回、意图感知重排、MMR 和每论文章节上限。
- Markdown、KaTeX、表格和代码块；窄窗口证据抽屉；证据分类与“仅看已引用”。
- 后端会话搜索/分页、Markdown/BibTeX 导出、解决办法/比较/新颖性/精读等回答模式。
- Provider Adapter 扩展 Responses API、Ollama/LM Studio；显式授权的外部论文检索只能进入候选层。
- 引入 citation precision/coverage、unsupported claim rate、nDCG、首证据和首 Token 延迟等端到端指标。

## GitHub 发布安全要求

1. 仓库可见性必须为 `PRIVATE`，创建后用 GitHub API/CLI 再次读取确认。
2. 推送前审计工作树、已跟踪文件和全部 Git 历史，不只检查最新提交。
3. 排除 `.env`、API Key、token、cookie、credential、用户级 Codex 配置、SQLite/日志运行数据库、安装包、target/node_modules、临时目录和自动发现失败运行目录。
4. 不打印或写入发现的秘密值；报告只记录文件、规则类别和处理结果。
5. 对历史中真实秘密必须先清理历史再推送；仅命中文档占位符、变量名或测试假密钥时记录为误报依据。
6. 检查 GitHub 单文件 100 MiB 限制和仓库总体积；必要时使用忽略、历史清理或 Git LFS，但不得擅自公开原始 PDF。
7. 保留现有两个未跟踪 discovery 失败目录，不提交。

## 验收标准

- [ ] 智能问答审查结论完整保存在本任务。
- [ ] 敏感信息扫描覆盖当前文件、已跟踪文件和全部 Git 历史，发现项完成逐条定性。
- [ ] `.gitignore` 覆盖本地凭据、数据库、构建产物和已知运行目录；没有敏感文件进入暂存区。
- [ ] GitHub 仓库创建为私密，默认分支完整推送并建立 upstream。
- [ ] 通过 `gh repo view` 或等价 API 确认 `visibility=PRIVATE`。
- [ ] 本地提交、Trellis 归档和 journal 完成；既有两个 raw discovery 未跟踪目录保持未提交。

## 不在本次范围

- 本次不实施智能问答 P0/P1/P2 修复。
- 不公开仓库，不发布 Release，不上传安装包。
- 不修改 Raw/Wiki 正文、B 类页面或正式词表。
