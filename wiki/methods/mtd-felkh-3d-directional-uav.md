---
type: method
subtype: algorithm
title: FELKH-3D 三维定向UAV充电
status: active
epistemic: medium
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy]
method_family: heuristic
problem_class: routing_with_charging
updated: 2026-08-01
---

# FELKH-3D 三维定向UAV充电

## 要点

- cMFEDS从三维球面方向中生成最小功能等价方向集。
- LKH规划UAV充电tour，避免直接搜索无限方向空间。
- 主要证据来自三维WRSN仿真，需区分方向集最优性和整体启发式路径质量。

## 来源

- [[src-gao2025-felkh-3d-uav]]
