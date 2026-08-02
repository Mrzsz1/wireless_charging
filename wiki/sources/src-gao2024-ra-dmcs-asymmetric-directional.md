---
type: source
title: "Directional WPT Charging for Routing-Asymmetric WRSNs with a Mobile Charger"
status: active
epistemic: high
year: 2024
venue: "arXiv preprint"
doi: ""
source_type: preprint
acquisition_method: auto_discovery
discovered_via: [arxiv]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-204713"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
authors: ["Zhenguo Gao", "Qi Zhang", "Qingyu Gao", "Yunlong Zhao", "Hsiao-Chun Wu"]
paper_keywords: ["Charging scheduling", "wireless power transfer", "directional mobile chargers", "wireless rechargeable sensor networks", "asymmetric path planning"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, time_slot]
constraints: [mobility, causality_online]
objectives: [min_energy, max_efficiency, max_completion_rate]
method_family: heuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Directional_WPT_Charging_for_Routing-Asymmetric_WRSNs_with_a_Mobile_Charger/Directional_WPT_Charging_for_Routing-Asymmetric_WRSNs_with_a_Mobile_Charger.pdf"
raw_md: "raw/canonical/Directional_WPT_Charging_for_Routing-Asymmetric_WRSNs_with_a_Mobile_Charger/full.md"
why_relevant: "把移动定向充电的路径、位置、方向和传输时长联合到路由非对称WRSN调度中。"
ingest_status: ingested
updated: 2026-08-01
---

# 路由非对称WRSN中的定向移动充电

## 一句话问题

在地形、风场或水流导致双向移动代价不同的WRSN中，如何选择充电位置、方向、传输时长和移动闭环，使节点需求得到满足且总能量损失较小。

## 系统设定与假设

- 单个定向移动充电器（DMC），节点位于二维区域；移动时不传输能量。
- 路由代价为有向的，不能把任意两点间代价视为对称。
- 充电位置和方向空间连续，节点存在能量需求。

## 方法要点

- 将ADMCCS分为充电位置生成、功能等价方向集、传输时长优化和非对称路径规划四步。
- KCPG用K-means生成最小规模的候选充电位置。
- cMFRDS抽取功能等价方向；线性规划确定各方向传输时长。
- 改造LKH求解ATSP，得到能量代价较小的闭环路线；整体方法称RA-DMCS。

## 主要结果

- 论文证明ADMCCS及其位置/路径子问题具有NP-hard或NP-complete性质。
- 仿真和测试床实验显示RA-DMCS优于典型对比算法，具体幅度取决于网络和路由代价设定。

## 局限

- 未同时处理障碍物、节点间能量分配、节点分布动态变化，以及全向/定向移动充电器协同。
- 主要结果来自论文设定的路由、能量模型和K-means/LKH启发式组合，不能直接迁移为在线未知请求保证。

## 链接

- 概念：[[cpt-directional-charging]]
- 方法：[[mtd-ra-dmcs-asymmetric-mobile]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-mobile-uav-directional-scheduling]] · [[syn-mobility-online-service-scheduling]]

