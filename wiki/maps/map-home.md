---
type: map
title: 知识库总图
status: active
updated: 2026-08-11
---

# 知识库总图 · 无线充电调度

> 当前水位：23 sources · 20 methods · 7 syntheses；另有 4 system-models、4 objectives、1 dataset-or-sim 和 2 本核心书籍的 61 个索引条目。

## 从问题进入

| 我现在知道什么 | 首选入口 | 下一步 |
|---|---|---|
| 系统里有哪些实体、约束和信息结构 | [[map-models-and-objectives|系统模型与目标]] | 选择 objective，再看 method/source |
| 需要在线响应请求 | [[map-online-scheduling|在线调度]] | 对照离线基线与因果边界 |
| 需要控制功率或干涉 | [[map-power-allocation|功率分配]] | 区分可加功率与相干干涉 |
| 有多个设备或充电器 | [[map-multi-device-wpt|多设备 WPT]] | 区分静态并发、移动与异构协同 |
| 只知道论文关键词 | [[map-domain-keywords|领域关键词]] | 进入 source 后查看证据定位 |
| 想确认库里是否覆盖 | [[library-status|库水位]] | 再用 `[[../index|Wiki 索引]]` 或桌面端问答 |

## 四层阅读路径

1. **Map**：确定问题属于哪个模型/目标；
2. **System model / Objective / Method**：理解变量、假设、算法和失效边界；
3. **Source / Synthesis**：核对论文结论与跨文献差异；
4. **Raw / Core books**：通过桌面端章节级证据返回原文行号或书籍 physical pages。

## 主题地图

- [[map-models-and-objectives]]
- [[map-online-scheduling]]
- [[map-power-allocation]]
- [[map-multi-device-wpt]]
- [[map-domain-keywords]]

## 综合路线

- [[syn-wrsn-scheduling-placement]]
- [[syn-interference-aware-concurrent-wpt]]
- [[syn-mobility-online-service-scheduling]]
- [[syn-mobile-uav-directional-scheduling]]
- [[syn-dynamic-roadway-wpt-infrastructure]]
- [[syn-adaptive-mobile-charger-coordination]]
- [[syn-core-books-atlas]]

## 快速入口

- Sources：`wiki/sources/`（23）
- Methods：`wiki/methods/`（20）
- Concepts：`wiki/concepts/`（7）
- System models：`wiki/system-models/`（4）
- Objectives：`wiki/objectives/`（4）
- Datasets / simulations：`wiki/datasets-sims/`（1）
- Syntheses：`wiki/syntheses/`（7）
- Problems / ideas：1 / 0（B 类仍需人工确认）
- 问答回归：`evals/gold_questions.json`（10）

## 健康与规则

- 水位：[[library-status]]
- 审查：[[../../logs/2026-08-11-llmwiki-structure-audit]]
- 时间线：`logs/log.md`
- Graphify：`graphify-out/graph.html`（派生物，Wiki 正文为真相）
- 规则：[[../../prd|prd.md]] · [[../../AGENTS|AGENTS.md]]
