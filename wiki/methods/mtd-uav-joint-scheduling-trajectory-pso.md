---
type: method
subtype: algorithm
title: PSOFKP/PSOD2P 充电UAV调度轨迹联合优化
status: active
epistemic: high
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy]
method_family: metaheuristic
problem_class: routing_with_charging
updated: 2026-08-01
---

# PSOFKP/PSOD2P 充电UAV调度轨迹联合优化

## 适用条件

- CUAV需要选择悬停点并规划访问轨迹。
- 连续和离散决策同时存在，解维度随悬停点数变化。

## 要点

- PSOFKP处理可变维度调度，使用K-means及惩罚-补偿机制。
- PSOD2P处理离散轨迹，使用离散化、2-opt和交叉缩减。

## 来源

- [[src-liu2021-joint-cuav-scheduling-trajectory]]
