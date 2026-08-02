---
type: method
subtype: algorithm
title: ROSE 鲁棒安全功率调度
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

# ROSE 鲁棒安全功率调度

## 适用条件

- 静态充电器和设备，EMR存在抖动、衰落或多径不确定性。
- 需要概率安全阈值和整体充电utility的权衡。

## 要点

- 概率模型经近似和区域离散化后转为SOCP。
- 集中式算法删减冗余二阶锥约束；分布式算法按区域分解并给出近似界。

## 来源

- [[src-dai2022-rose-robust-safe-charging]]
