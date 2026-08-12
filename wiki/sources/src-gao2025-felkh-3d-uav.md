---
type: source
title: "Optimal 3D Directional WPT Charging via UAV for 3D Wireless Rechargeable Sensor Networks"
status: active
epistemic: medium
year: 2025
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
authors: ["Zhenguo Gao", "Hui Li", "Yiqin Chen", "Qingyu Gao", "Zhufang Kuang", "Shih-Hau Fang", "Hsiao-Chun Wu"]
paper_keywords: ["3D Wireless rechargeable sensor networks", "charging schedule", "UAV charger", "directional WPT"]
keyword_source: index_terms
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy, max_completion_rate]
method_family: heuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Optimal_3D_Directional_WPT_Charging_via_UAV_for_3D_Wireless_Rechargeable_Sensor_Networks/Optimal_3D_Directional_WPT_Charging_via_UAV_for_3D_Wireless_Rechargeable_Sensor_Netw.pdf"
raw_md: "raw/canonical/Optimal_3D_Directional_WPT_Charging_via_UAV_for_3D_Wireless_Rechargeable_Sensor_Networks/full.md"
why_relevant: "把定向WPT移动充电从二维扩展到三维WRSN，并处理球面方向空间。"
ingest_status: ingested
updated: 2026-08-12
---

# 三维WRSN中的定向UAV充电

## TL;DR

在三维传感器分布和方向空间无限的条件下，如何生成功能等价的最小方向集并规划UAV充电路径。

## 何时使用 / 何时不使用

- **使用**：传感器与 UAV 处于三维空间，定向 WPT 的球面方向连续且需与 UAV 路径联合。
- **不使用**：含动态请求、禁飞区、强风或多 UAV 冲突；方向集结构最优不代表 tour 全局最优。

## 系统模型与假设

- 节点位于三维空间，UAV作为定向无线充电器移动服务。
- 充电方向覆盖整个球面，路径和方向共同影响充电计划。

## 变量、目标与约束

- **变量/状态**：三维悬停位置、球面方向角、代表方向集合、各位置—方向充电时长及 UAV tour。
- **目标与约束**：满足三维节点充电需求并降低飞行/悬停能耗；先压缩无限方向空间，再求路径与时长。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 先证明DCS-3D的NP-hard性质。
- cMFEDS生成与无限方向集功能等价的最小方向集。
- FELKH-3D使用LKH规划UAV充电tour，组合方向集与路径优化。

## 理论性质与复杂度

DCS-3D 为 NP-hard；cMFEDS 构造功能等价的最小代表方向集，FELKH-3D 的 tour 部分使用 LKH 启发式。

## 实验设置与基线

含测试床和仿真，分别比较位置生成、方向选择、tour 与完整方案，不能把子模块优势等同于总体最优。

## 定量结果

- 仿真显示FELKH-3D优于经典对比算法；方向集最小性给出结构上的可解释性。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 主要是仿真结果，三维信道、飞行安全、障碍物和动态请求尚未纳入。
- LKH和方向离散化效果依赖节点分布与模型参数。

## 证据定位

- Raw：`raw/canonical/Optimal_3D_Directional_WPT_Charging_via_UAV_for_3D_Wireless_Rechargeable_Sensor_Networks/full.md`
- 模型：§III，第 68–135 行；问题：§IV，第 136–193 行；方向压缩：§V，第 194–451 行；FELKH：第 452–523 行；实验：第 524–655 行。

## 相关页面

- 概念：[[cpt-directional-charging]]
- 方法：[[mtd-felkh-3d-directional-uav]]
- 综合：[[syn-mobile-uav-directional-scheduling]] · [[syn-wrsn-scheduling-placement]]
