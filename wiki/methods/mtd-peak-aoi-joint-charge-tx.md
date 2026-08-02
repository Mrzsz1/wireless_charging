---
type: method
subtype: algorithm
title: 峰值 AoI 充传联合调度
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [causality_online, qos]
objectives: [min_latency]
method_family: online_algorithm
problem_class: online_scheduling
updated: 2026-07-10
---

# Peak AoI 联合充电与传输

## 适用条件

- 无线供能边缘、定向充电  
- 目标为最大峰值 AoI，而非纯充电 utility  

## 要点

- 充电时延与峰值 AoI 的界  
- 单/多充电器与带宽约束算法  

## 来源

- [[src-chen-peak-aoi-wpt]]
