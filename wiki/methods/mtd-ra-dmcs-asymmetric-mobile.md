---
type: method
subtype: algorithm
title: RA-DMCS 路由非对称定向移动充电
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, time_slot]
constraints: [mobility]
objectives: [min_energy, max_completion_rate]
method_family: heuristic
problem_class: routing_with_charging
updated: 2026-08-01
---

# RA-DMCS 路由非对称定向移动充电

## 适用条件

- 定向移动充电器、WRSN节点和有向移动代价。
- 需要同时决定位置、方向、传输时间和访问闭环。

## 输入 / 输出

- 入：节点位置与需求、方向性传输模型、有向路径代价。
- 出：候选充电位置、功能等价方向、传输时长和ATSP闭环路线。

## 要点

- KCPG缩减连续位置空间；cMFRDS缩减方向空间。
- 线性规划分配传输时长；LKH处理非对称路径。

## 来源

- [[src-gao2024-ra-dmcs-asymmetric-directional]]
