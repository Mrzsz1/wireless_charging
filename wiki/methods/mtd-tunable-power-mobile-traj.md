---
type: method
subtype: algorithm
title: 可调功率 + 移动轨迹充电调度（CM）
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route]
constraints: [peak_power, mobility]
objectives: [max_throughput]
method_family: heuristic
problem_class: power_allocation
updated: 2026-07-10
---

# Charging on the Move 功率调度

## 适用条件

- 静态充电器、**移动**接收端、已知轨迹  
- 功率分档可调、总预算  
- 功率可加模型  

## 要点

- 轨迹分段常数功率近似  
- 子模最大化近似算法  

## 来源

- [[src-wu-charging-on-the-move]]
