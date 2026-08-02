---
type: method
subtype: algorithm
title: HIPO 障碍场景异构放置
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver]
constraints: []
objectives: [max_throughput]
method_family: heuristic
problem_class: charger_placement
updated: 2026-07-10
---

# HIPO / PDCS

## 适用条件

- 障碍 + 异构充电器类型配额  
- 扇环定向模型  

## 要点

- 多可行几何区 + PDCS 候选  
- 划分拟阵 + 子模贪心  

## 来源

- [[src-wang-hipo-obstacles]]
