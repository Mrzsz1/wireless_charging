---
title: 无线充电调度 · LLM Wiki
status: active
updated: 2026-08-01
---

# 无线充电调度知识库

Karpathy 式 **LLM Wiki** + **Graphify** 知识图：文献编译为持久结构化笔记；Obsidian + Claudian 日常问答；Agent + Graphify 做检索与维护。

## 范式与工具

| 层 | 工具 | 说明 |
|----|------|------|
| 范式 | [Karpathy LLM Wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) | 编译型 wiki，非每次 RAG |
| 浏览 | Obsidian | 人读、图谱、双向链 |
| 问答 | Claudian | `/solve` `/novelty` |
| 编译 | **Codex CLI / Grok CLI** + `AGENTS.md` | Ingest / Lint |
| 图查询 | [Graphify](https://github.com/Graphify-Labs/graphify) | Codex `$graphify` · Grok 用 `.agents/skills` · 终端 `query/path/explain` |
| PDF→md | MinerU | 你执行 |

## 从这里开始

| 我想… | 去哪 |
|--------|------|
| 看总体导航 | [[wiki/maps/map-home|知识库总图]] |
| 看内容目录 | [[wiki/index|Wiki 索引]] |
| 看库有多新 | [[wiki/maps/library-status|库水位]] |
| 按领域关键词浏览 | [[wiki/maps/map-domain-keywords|领域关键词地图]] |
| 看时间线 | [[logs/log|log]] |
| **怎么用（必读）** | [[使用说明]] |
| Windows 快捷启动 | `tools/launch-wiki.ps1`（桌面：无线充电 LLM Wiki） |
| 了解系统架构 | [[ARCHITECTURE]] |
| 了解产品约定 | [[prd]] |
| Agent 宪法 | [[AGENTS]] |
| Karpathy 映射 | [[schema/references/karpathy-llm-wiki]] |
| Graphify 用法 | [[schema/references/graphify]] |
| 导入论文 | [[raw/canonical/README|canonical 说明]] |
| 自动发现 / 手动投放 | [[raw/inbox/README|inbox 分流说明]] |
| 配置找解法 | [[schema/claudian-solve]] |
| 配置新颖性 | [[schema/claudian-novelty]] |
| 让 agent 编译 | [[schema/agent-a-compile]] |
| 健康检查 | [[schema/lint-checklist]] |
| 问答回归测试 | [[evals/README]]（10 条真实用例） |

## 当前阶段

**A 编译完成（23 sources / 7 syntheses / 1 problem）· 20/21论文source有作者关键词 · 10条问答回归答案就位**

1. ~~投料 + 首批 A 编译~~ → 见 [[wiki/index]] · [[wiki/syntheses/syn-wrsn-scheduling-placement]]  
2. 候选累计为：**46 pending / 6 selected（全文受限）/ 14 rejected / 12 promoted**；12篇已完成 MinerU 与 A 编译
3. 用 `py -3 tools/wiki_eval.py` 校验回归契约，再让 Claudian 回答 `evals/gold_questions.json` 中的问题  
4. 首个正式问题：[[wiki/problems/prob-joint-deployment-online-interference]]；算法 idea 仍待硬件动作假设锁定  
5. 关键词入口：[[wiki/maps/map-domain-keywords]]；受控词表仍走提案确认  
6. Graphify 已按忽略规则重建为 798 nodes / 820 links；当前为结构化抽取，配置文档LLM backend后可补语义边

## 目录一览

```text
raw/            原文（auto-discovered + manual-drop + canonical）— 正文不可变
wiki/           编译后的结构化知识 + index + maps
schema/         规则、词表、提示词、references
templates/      新建笔记模板
logs/           log.md 时间线 + 运行详情
graphify-out/   Graphify 派生图（可重建）
AGENTS.md       Agent 总宪法
prd.md          产品需求与决策锁
```

## 核心专著（2026-08-01）

知识库现在包含两本核心参考书：`Algorithmic Game Theory`（775 页、29 章）和 `Approximation Algorithms`（396 页、30 章）。原始 PDF 保留在 `raw/inbox/manual-drop/`；按章节拆分的 Markdown 在 `raw/canonical/algorithmic-game-theory/chapters/` 与 `raw/canonical/approximation-algorithms/chapters/`。查询模型、算法、解决办法、近似比、均衡或机制时，先运行：

```powershell
py -3 tools/core_reference_search.py "<问题>" --limit 8
```

回答必须携带书名、章节和 PDF physical pages；质量门禁见 `raw/canonical/core-books-quality.json`。
