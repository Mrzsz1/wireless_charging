---
type: concept
title: 概率电磁辐射安全 Probabilistic EMR Safety
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, power_pool]
constraints: [thermal_or_sar, interference]
objectives: [max_throughput, max_efficiency]
method_family: convex_opt
problem_class: power_allocation
updated: 2026-08-01
---

# 概率电磁辐射安全 Probabilistic EMR Safety

概率EMR安全把无线充电的电磁辐射抖动、衰落和多径影响纳入约束，要求超过阈值的概率不高于给定风险，而不只是限制期望EMR。

## 证据

- [[src-dai2022-rose-robust-safe-charging]] 建立概率充电/EMR模型，并将问题转为SOCP。
- 与[[cpt-wave-interference]]相关，但概率安全约束不能由波干涉标签直接推出。

## 使用边界

该概念目前主要由ROSE支撑，适用于检索安全约束和鲁棒功率分配，不代表所有WPT调度工作都采用概率安全模型。
