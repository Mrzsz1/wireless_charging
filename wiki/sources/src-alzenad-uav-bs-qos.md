---
type: source
title: "3-D Placement of an Unmanned Aerial Vehicle Base Station for Maximum Coverage of Users With Different QoS Requirements"
status: needs_review
epistemic: medium
year: 2018
venue: "IEEE Wireless Communications Letters"
doi: "10.1109/LWC.2017.2752161"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Mohamed Alzenad", "Amr El-Keyi", "Halim Yanikomeroglu"]
paper_keywords: ["Unmanned aerial vehicles", "drone", "coverage", "optimization"]
keyword_source: index_terms
scenario: [uav_wpt]
entities: [transmitter, receiver]
constraints: [qos, peak_power]
objectives: [max_completion_rate]
method_family: convex_opt
problem_class: charger_placement
pdf_path: "raw/canonical/3-D_Placement_of_an_Unmanned_Aerial_Vehicle_Base_Station_for_Maximum_Coverage_of_Users_With_Different_QoS_Requirements.pdf-22862a8f-519b-48c9-a820-695a0df5d5e5/3-D_Placement_of_an_Unmanned_Aerial_Vehicle_Base_Station_for_Maximum_Coverage_of_Users_With_Different_QoS_Requirements.pdf"
raw_md: "raw/canonical/3-D_Placement_of_an_Unmanned_Aerial_Vehicle_Base_Station_for_Maximum_Coverage_of_Users_With_Different_QoS_Requirements.pdf-22862a8f-519b-48c9-a820-695a0df5d5e5/full.md"
why_relevant: "边界文献：UAV 基站 3D 部署与 QoS 覆盖，非 WPT 充电；几何部署思路可迁移"
ingest_status: ingested
updated: 2026-08-12
tags: [peripheral]
---

# UAV-BS 3D 部署（多 QoS 覆盖）— 边界文献

## TL;DR

在发射功率受限、用户具有**不同 QoS（SNR）需求**时，如何 3D 放置 UAV 基站以最大化被覆盖用户数。

## 何时使用 / 何时不使用

- **使用**：需要把三维高度、水平位置与异质 QoS 覆盖联合考虑，并把它作为 UAV 几何部署的边界类比。
- **不使用**：问题本身不是 WPT、没有电池或充电时序；不得把蜂窝 SNR 覆盖直接当作收能保证。

## 系统模型与假设

- 拥塞区域临时 UAV-BS；用户近似静止
- A2G 概率 LoS/NLoS 路径损耗
- 多 QoS → 多重圆盘覆盖（multiple circles placement）

## 变量、目标与约束

- **变量/状态**：UAV 水平位置、高度、各 QoS 类用户的覆盖半径/权重，以及发射功率上限；原文将三维搜索约化为高度相关的一维候选搜索。
- **目标与约束**：在不同 SNR/QoS 门槛下最大化被覆盖用户数；约束来自空地 LoS/NLoS 路损、发射功率和可行高度。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 最优：对 1D 参数闭区间 **exhaustive search (ES)**
- 低复杂度 **MWA（maximal weighted area）**，仿真接近 ES

## 理论性质与复杂度

ES 在闭区间枚举高度并求平面多圆覆盖；MWA 是低复杂度启发式。原文未给出 WPT 意义的近似比或充电调度保证。

## 实验设置与基线

仿真比较 ES 与 MWA 在不同用户/QoS 分布下的覆盖数；没有充电硬件、收能效率或电池实验。

## 定量结果

- MWA 接近 ES 且复杂度显著降低

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 非充电；单 UAV；用户同构移动模型简化

## 证据定位

- Raw：`raw/canonical/3-D_Placement_of_an_Unmanned_Aerial_Vehicle_Base_Station_for_Maximum_Coverage_of_Users_With_Different_QoS_Requirements.pdf-22862a8f-519b-48c9-a820-695a0df5d5e5/full.md`
- 模型：§II，原文第 35–92 行；问题与 ES/MWA：§III，原文第 93–178 行；仿真与结论：原文第 179–199 行。

## 相关页面

- 综合：[[syn-wrsn-scheduling-placement]]（边界提及）
