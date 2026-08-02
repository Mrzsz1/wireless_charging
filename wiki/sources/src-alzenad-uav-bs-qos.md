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
updated: 2026-07-14
tags: [peripheral]
---

# UAV-BS 3D 部署（多 QoS 覆盖）— 边界文献

## 一句话问题

在发射功率受限、用户具有**不同 QoS（SNR）需求**时，如何 3D 放置 UAV 基站以最大化被覆盖用户数。

## 与本库关系（重要）

- **主题是蜂窝覆盖/空地信道，不是无线能量传输调度**
- 按 PRD：仅作相邻几何/覆盖优化参考；`/solve` 默认降权
- `status: needs_review`：是否保留在核心库由你决定

## 系统设定与假设

- 拥塞区域临时 UAV-BS；用户近似静止
- A2G 概率 LoS/NLoS 路径损耗
- 多 QoS → 多重圆盘覆盖（multiple circles placement）

## 方法要点

- 最优：对 1D 参数闭区间 **exhaustive search (ES)**
- 低复杂度 **MWA（maximal weighted area）**，仿真接近 ES

## 主要结果

- MWA 接近 ES 且复杂度显著降低

## 局限

- 非充电；单 UAV；用户同构移动模型简化

## 链接

- 综合：[[syn-wrsn-scheduling-placement]]（边界提及）

