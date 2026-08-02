---
type: method
subtype: algorithm
title: GAIN 干涉感知并发放置
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver]
constraints: [interference]
objectives: [max_throughput]
method_family: heuristic
problem_class: charger_placement
updated: 2026-07-10
---

# GAIN 放置方案

## 适用条件

- 可调整充电器位置，传感器可在 PoI 附近微调  
- 希望利用建设性干涉  

## 要点

- 充电器放置抬高增强区基础功率  
- 传感器落点选局部最高功率  

## 来源

- [[src-ma-concurrent-gain]]
