# Schema — 人与 Agent 的宪法

本目录是 vault 的**规则层**（Karpathy 所称 schema）。  
外部 agent（A 编译）与 Claudian（问答）都必须遵守。  
Agent 总入口另见根目录 `AGENTS.md`。

权威优先级：

1. [[../prd|prd.md]] — 产品与架构决策  
2. `AGENTS.md` + 本目录 — 可执行约定  
3. 参考原文 — `references/`（冲突时以 prd 特化条款为准）  
4. 临时对话 — 不得与以上冲突  

| 文件 | 用途 |
|------|------|
| `page-types.md` | 9 类页面职责、可写/禁写、命名前缀 |
| `writing-rules.md` | A/B 隔离、冲突、外搜、日志红线 |
| `frontmatter.md` | 字段规范 |
| `domain-keywords.md` | 论文原词 → 领域导航 → 受控词表提案的三层治理 |
| `vocab.yaml` | **受控词表唯一权威**（frontmatter 只写 id） |
| `vocab-proposals.md` | 新词提案（须用户确认后合并） |
| `ingest-checklist.md` | PDF→md→A 编译检查清单 |
| `lint-checklist.md` | Karpathy Lint 健康检查 |
| `claudian-solve.md` | `/solve` 系统提示词模板 |
| `claudian-novelty.md` | `/novelty` 系统提示词模板 |
| `agent-a-compile.md` | 外部 agent 做 A 编译时的操作规程 |
| `references/karpathy-llm-wiki.md` | Karpathy gist 摘要与映射 |
| `references/graphify.md` | Graphify 安装与本库用法 |
