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
updated: 2026-08-01
---

# 三维WRSN中的定向UAV充电

## 一句话问题

在三维传感器分布和方向空间无限的条件下，如何生成功能等价的最小方向集并规划UAV充电路径。

## 系统设定与假设

- 节点位于三维空间，UAV作为定向无线充电器移动服务。
- 充电方向覆盖整个球面，路径和方向共同影响充电计划。

## 方法要点

- 先证明DCS-3D的NP-hard性质。
- cMFEDS生成与无限方向集功能等价的最小方向集。
- FELKH-3D使用LKH规划UAV充电tour，组合方向集与路径优化。

## 主要结果

- 仿真显示FELKH-3D优于经典对比算法；方向集最小性给出结构上的可解释性。

## 局限

- 主要是仿真结果，三维信道、飞行安全、障碍物和动态请求尚未纳入。
- LKH和方向离散化效果依赖节点分布与模型参数。

## 链接

- 概念：[[cpt-directional-charging]]
- 方法：[[mtd-felkh-3d-directional-uav]]
- 综合：[[syn-mobile-uav-directional-scheduling]] · [[syn-wrsn-scheduling-placement]]

