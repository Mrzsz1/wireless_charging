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
updated: 2026-08-12
---

# 路由非对称WRSN中的定向移动充电

## TL;DR

在地形、风场或水流导致双向移动代价不同的WRSN中，如何选择充电位置、方向、传输时长和移动闭环，使节点需求得到满足且总能量损失较小。

## 何时使用 / 何时不使用

- **使用**：移动代价具有方向不对称，且定向充电的位置、方向、时长和闭环路线必须联合生成。
- **不使用**：未知请求在线到达、动态障碍或多移动充电器协同时；LKH 结果不等于全局最优。

## 系统模型与假设

- 单个定向移动充电器（DMC），节点位于二维区域；移动时不传输能量。
- 路由代价为有向的，不能把任意两点间代价视为对称。
- 充电位置和方向空间连续，节点存在能量需求。

## 变量、目标与约束

- **变量/状态**：候选充电位置、功能等价方向集合、每个位置—方向对的传输时长，以及有向图上的访问闭环。
- **目标与约束**：满足节点能量需求，同时降低移动、WPT 与设备能量损失；约束包含有向移动代价和定向能量系数。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 将ADMCCS分为充电位置生成、功能等价方向集、传输时长优化和非对称路径规划四步。
- KCPG用K-means生成最小规模的候选充电位置。
- cMFRDS抽取功能等价方向；线性规划确定各方向传输时长。
- 改造LKH求解ATSP，得到能量代价较小的闭环路线；整体方法称RA-DMCS。

## 理论性质与复杂度

ADMCCS 为 NP-hard，位置子问题 P2 为 NP-complete，非对称路径子问题 P4 为 NP-hard；整体 RA-DMCS 是分解启发式。

## 实验设置与基线

比较位置生成、方向选择和非对称 tour 方法，含 toy network 与规模仿真；结果依赖 K-means 与 LKH 参数。

## 定量结果

- 论文证明ADMCCS及其位置/路径子问题具有NP-hard或NP-complete性质。
- 仿真和测试床实验显示RA-DMCS优于典型对比算法，具体幅度取决于网络和路由代价设定。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 未同时处理障碍物、节点间能量分配、节点分布动态变化，以及全向/定向移动充电器协同。
- 主要结果来自论文设定的路由、能量模型和K-means/LKH启发式组合，不能直接迁移为在线未知请求保证。

## 证据定位

- Raw：`raw/canonical/Directional_WPT_Charging_for_Routing-Asymmetric_WRSNs_with_a_Mobile_Charger/full.md`
- 模型：§III，第 90–201 行；问题/复杂度：§IV，第 202–251 行；分解与 RA-DMCS：第 252–387 行；实验：第 388–481 行。

## 相关页面

- 概念：[[cpt-directional-charging]]
- 方法：[[mtd-ra-dmcs-asymmetric-mobile]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-mobile-uav-directional-scheduling]] · [[syn-mobility-online-service-scheduling]]
