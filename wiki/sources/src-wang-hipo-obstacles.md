---
type: source
title: "Practical Heterogeneous Wireless Charger Placement with Obstacles"
status: active
epistemic: high
year: 2020
venue: "IEEE Transactions on Mobile Computing"
doi: "10.1109/TMC.2019.2916384"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Xiaoyu Wang", "Haipeng Dai", "Weijun Wang", "Jiaqi Zheng", "Nan Yu", "Guihai Chen", "Wanchun Dou", "Xiaobing Wu"]
paper_keywords: ["Charger placement", "heterogeneity", "obstacles"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver]
constraints: [interference]
objectives: [max_throughput]
method_family: heuristic
problem_class: charger_placement
pdf_path: "raw/canonical/Practical_Heterogeneous_Wireless_Charger_Placement_with_Obstacles.pdf-9d3d72a1-5370-40da-8eac-87463b92da63/Practical_Heterogeneous_Wireless_Charger_Placement_with_Obstacles.pdf"
raw_md: "raw/canonical/Practical_Heterogeneous_Wireless_Charger_Placement_with_Obstacles.pdf-9d3d72a1-5370-40da-8eac-87463b92da63/full.md"
why_relevant: "HIPO：异构定向充电器+任意障碍物下的放置（扇环模型）"
ingest_status: ingested
updated: 2026-08-12
---

# HIPO：有障碍的异构充电器放置

## TL;DR

在含**任意形状障碍**的平面上，给定各类充电器数量预算，部署**异构定向**充电器的位置与朝向，最大化设备充电 utility。

## 何时使用 / 何时不使用

- **使用**：异构定向充电器有类型配额，设备/充电器朝向固定，且任意形状障碍阻挡视距。
- **不使用**：相位相关波干涉、在线请求或设备移动主导时；多源功率可加是关键模型边界。

## 系统模型与假设

- 设备位置与朝向固定、可异构
- 充电器类型与数量有配额
- **扇环（sector ring）**充电/接收模型：$d_{min}$–$d_{max}$（过近可能为 0 功率，贴近 Powercast 实测）
- 障碍阻挡 LOS，无反射
- 多充电器功率可加

## 变量、目标与约束

- **变量/状态**：各类型充电器的位置与朝向、PDCS 候选、设备覆盖效用和类型预算。
- **目标与约束**：在障碍、扇环接收区和异构类型配额下最大化饱和总 charging utility。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 分段常数近似功率 → 多可行几何区域
- **PDCS** 抽取有限候选策略
- 建模为划分拟阵约束下的单调子模最大化 → 贪心 $(1-1/e)$ 级近似
- 相对对比至少约 **33.49%**

## 理论性质与复杂度

分段常数近似、区域离散和 PDCS 把连续空间有限化；划分拟阵约束下子模贪心达到 (1−1/e) 级保证。

## 实验设置与基线

仿真考察充电器/设备数、角度、阈值和并行计算，并有现场测试；重部署和成本是扩展讨论。

## 定量结果

- 仿真+现场；扇环+障碍+异构联合

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 正式卷期为 2020；DOI 在线发表年份为 2019
- 功率可加，未用波干涉非线性模型

## 证据定位

- Raw：`raw/canonical/Practical_Heterogeneous_Wireless_Charger_Placement_with_Obstacles.pdf-9d3d72a1-5370-40da-8eac-87463b92da63/full.md`
- 模型：§3，第 63–109 行；PDCS/重构：§4，第 110–378 行；分布式算法：第 379–461 行；仿真/实测：第 462–593 行。

## 相关页面

- 概念：[[cpt-directional-charging]] · [[cpt-charging-utility]]
- 方法：[[mtd-hipo-placement-obstacles]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]]
