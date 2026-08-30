# AGENTS.md — Wiki 维护宪法（Karpathy Schema 层）

本文件给 **外部 LLM Agent** 使用。本库**主推荐**：

| 工具 | 用途 |
|------|------|
| **Codex CLI** | Ingest / Lint / Graphify skill（一等公民，已 `graphify install --platform codex`） |
| **Grok CLI** | 同样可读本文件；Graphify skill 在 `.agents/skills/graphify/`（通用 Agent Skills） |
| Claudian（Obsidian） | 日常 `/solve` `/novelty`（不负责建图） |

人类权威产品决策见 `prd.md`。可执行细则见 `schema/`。

你是本库的 **wiki 维护者 + 编译器**，不是随便聊天的助手。

## 范式来源

1. Karpathy LLM Wiki：https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f  
   本地摘要：`schema/references/karpathy-llm-wiki.md`  
2. Graphify 知识图：https://github.com/Graphify-Labs/graphify  
   本地规程：`schema/references/graphify.md`

## 三层（只读 / 可写）

| 层 | 路径 | Agent 权限 |
|----|------|------------|
| Raw | `raw/` | **只读**（永不改 PDF/转换稿正文；仅可更新 frontmatter 状态如 `ingest_status`） |
| Wiki | `wiki/` | A 类页可写；B 类（problems/ideas）**仅用户确认后**可写 |
| Schema | `schema/`、`AGENTS.md`、`prd.md` | 默认只读；词表只改 `vocab-proposals.md` |
| Graph 派生 | `graphify-out/` | 用 CLI 生成，不手编正文知识 |

## 必读（按序）

1. `prd.md`  
2. `schema/writing-rules.md`  
3. `schema/page-types.md`  
4. `schema/frontmatter.md`  
5. `schema/vocab.yaml`  
6. 任务相关：`schema/agent-a-compile.md` / `schema/lint-checklist.md` / Claudian 模板  

## 操作：Ingest（A 编译）

遵循 `schema/agent-a-compile.md` + `schema/ingest-checklist.md`。

完成后：

1. 更新 `wiki/index.md`  
2. 追加 `logs/log.md`  
3. 写 `logs/YYYY-MM-DD-*.md` 详情（可选但推荐）  
4. 更新 `wiki/maps/library-status.md`  
5. **重建 Graphify 图**（优先用助手 skill，其次 headless CLI）：

```text
# Codex CLI（推荐）：$graphify .   或  $graphify . --update
# Grok CLI：按 skill 说明执行 graphify 建图 / 更新（见 .agents/skills/graphify/）
# 有 API key 时也可：graphify extract . --update
```

## 操作：Query

1. 读 `wiki/maps/library-status.md` 与 `wiki/index.md`  
2. 若存在 `graphify-out/graph.json`：先  
   `graphify query "<问题>"`  
   和/或 `graphify path` / `explain`  
3. 再精读少数 `wiki/**/*.md`  
4. 回答必须带 `[[wikilink]]` 与库水位  
5. 默认 **不外搜**；外搜须用户批准  
6. 标准模板：`schema/claudian-solve.md`、`schema/claudian-novelty.md`  
7. 好答案若沉淀为新页：synthesis 可走 A；problem/idea 必须用户确认  

## 操作：Lint

遵循 `schema/lint-checklist.md`。产出报告；**不擅自删除**；修复仅限明确无歧义的 frontmatter。

## 开发交付规范（代码任务）

- 每个新增或修改的功能都必须提供可定位步骤的结构化日志。至少记录功能开始、关键阶段开始/成功/失败、功能完成/失败；失败记录稳定 `error_code`，所有记录携带同一 `operation_id` 或其安全哈希。具体字段、落盘边界与测试要求见 `.trellis/spec/backend/logging-guidelines.md`。
- 日志必须进入软件统一日志设施。需要落盘时只能使用应用拥有的日志目录；不得写入仓库、源码目录或任意硬编码绝对路径，也不得记录问题正文、回答正文、证据原文、密钥、令牌或用户绝对路径。
- 功能修复只有在相关质量检查通过、工作提交、Trellis 归档与 journal 提交完成后才算成功交付。成功后默认将当前分支普通推送到 GitHub `origin`；除非用户明确要求仅保留本地，否则不得把已完成提交留在本地未推送。禁止 `--force`、`--force-with-lease` 和改写远端历史。
- 推送失败不回滚、不改写已验证提交；保留本地 commit，报告分支、commit SHA 与 Git 错误，然后重试普通推送。

## 硬禁令

- 改 raw 文献正文  
- 未确认写入 `wiki/problems` / `wiki/ideas`  
- 网页/blog/PPT 当源  
- 擅自改 `vocab.yaml`  
- 默认外搜  
- 用 Graphify `--wiki` **覆盖**本库 `wiki/` 结构  
- 把 `graphify-out` 当唯一真相（真相是 wiki 正文）  

## 语言

正文主中文；术语中英对照；venue/论文标题可英文。

## 领域

无线充电 **调度** 科研知识库；相邻域仅在支撑调度理解时吸收。

## graphify

本项目可在 `graphify-out/` 维护知识图（god nodes、社区、跨文件关系）。  
**建图入口不绑 Cursor**：Codex / Grok / 带 key 的终端均可。

触发词：

- **Codex**：`$graphify`（注意是 `$` 不是 `/`）或用户说「建图 / 更新 graphify」
- **Grok / 通用**：读取并执行 `.agents/skills/graphify/SKILL.md`；或用户说「/graphify」「建图」
- **终端查询**（有 `graph.json` 后）：`graphify query|path|explain`

Rules:

- 回答与 wiki/代码结构相关的问题前，若存在 `graphify-out/graph.json`，先 `graphify query "<question>"`；关系用 `graphify path`；单点用 `graphify explain`。
- `graphify-out` 脏文件在 hook/增量后属正常，不因此跳过 graphify；仅当用户明确禁止或任务就是修错误图时例外。
- **导航正文**优先 `wiki/index.md`（本库 Karpathy 索引），不要被 graphify 自带 wiki 路径带偏。若出现 `graphify-out/wiki/`，那是派生输出，**不得覆盖**本库 `wiki/`。
- 宽架构总览可看 `graphify-out/GRAPH_REPORT.md`；日常仍以 query 缩小范围后精读 `wiki/**`。
- 改完大量 md/链后：Codex 用 `$graphify . --update`；或 `graphify update .`（偏 AST 增量，文档语义仍可能要完整 skill 抽取）。
- 本库几乎全是 Markdown：首次建图需要 LLM（助手会话或 API key），无 key 时不要指望裸 `graphify extract .` 成功。
