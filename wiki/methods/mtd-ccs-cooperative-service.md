---
type: method
subtype: algorithm
title: CCS 合作充电服务调度
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, user_or_request]
constraints: [qos]
objectives: [min_cost]
method_family: game_theory
problem_class: offline_scheduling
updated: 2026-07-10
---

# CCS / CCSA / CCSGA

## 适用条件

- 固定充电器付费服务 + 移动设备选站  
- 目标最小化综合成本（费用+移动）  
- 需组内成本分摊稳定性  

## 要点

- CCSA：子模 + 贪心近似  
- CCSGA：联盟形成博弈，Nash  

## 来源

- [[src-xu-cooperative-ccs]]
