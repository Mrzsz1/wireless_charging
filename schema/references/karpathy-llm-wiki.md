# 参考：Karpathy LLM Wiki

- **原文**：https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f  
- **抓取/对齐日期**：2026-07-10  
- **角色**：范式权威参考（非本库实现细则；细则以 `prd.md` + `schema/` 为准）

## 核心思想（摘要）

1. **不是 RAG 主路径**：不要每次提问都从 raw 碎片重推一遍。  
2. **编译型 wiki**：LLM 增量维护持久、可交叉引用的 Markdown；知识编译一次并保持更新。  
3. **人机分工**：人负责投料、探索、提问；LLM 负责摘要、交叉引用、归档、簿记。  
4. **Obsidian = IDE**；LLM = 程序员；wiki = 代码库。

## 三层架构（本库映射）

| Karpathy | 本库路径 |
|----------|----------|
| Raw sources（不可变） | `raw/canonical/`、`raw/inbox/` |
| Wiki（LLM 写） | `wiki/` |
| Schema（纪律文件） | `schema/` + 根目录 `AGENTS.md` |

## 三种操作

| 操作 | 含义 | 本库落地 |
|------|------|----------|
| **Ingest** | 新源读入 → 摘要 → 更新实体/概念/索引/日志 | A 编译 + `schema/agent-a-compile.md` |
| **Query** | 对 wiki 提问；好答案可回写为新页 | Claudian `/solve` `/novelty`；可选回写（B 需确认） |
| **Lint** | 健康检查：矛盾、过期、孤儿页、缺页、缺链 | `schema/lint-checklist.md` |

## 两个特殊文件

| Karpathy | 本库 |
|----------|------|
| `index.md`（内容目录） | `wiki/index.md` |
| `log.md`（时间线） | `logs/log.md` |

## 本库相对原文的特化（不违背范式）

- 领域：无线充电**调度**科研  
- 问答主入口：Obsidian **Claudian**  
- 图查询层：**Graphify**（见 `schema/references/graphify.md`）  
- 受控词表 + 半自动闸门（problem/idea/外搜需确认）  
- PDF 经 **MinerU** 转 md 后再编译  

## 原文强调、本库已采纳

- Raw 只读、不改  
- Schema 共演化  
- 好 query 结果可沉淀回 wiki（本库：B 类需确认）  
- 定期 lint  
- git 管理 markdown  

## 原文可选、本库暂缓

- Obsidian Web Clipper（网页源本库禁止进 canonical）  
- qmd 搜索引擎（中期：先 index + Graphify）  
- Marp 幻灯片  
