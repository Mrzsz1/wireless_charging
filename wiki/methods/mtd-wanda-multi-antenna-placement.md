---
type: method
subtype: algorithm
title: WANDA 多天线充电器放置
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

# WANDA-ROF / WANDA-ROU

## 适用条件

- 多定向天线充电器部署  
- 相对朝向固定或可调  

## 要点

- 区域离散 + 子模贪心  
- ROF ≈ 1/2，ROU ≈ 1/6 近似  

## 来源

- [[src-dai-wanda-multi-antenna]]
