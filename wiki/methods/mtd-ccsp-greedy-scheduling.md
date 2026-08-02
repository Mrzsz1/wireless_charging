---
type: method
subtype: algorithm
title: CCSP 贪心并发充电调度
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver]
constraints: [interference]
objectives: [min_latency]
method_family: heuristic
problem_class: multi_tx_coordination
updated: 2026-07-10
---

# CCSP 贪心调度

## 适用条件

- 多静态充电器并发 RF 充电  
- 必须建模**波干涉**（utility 非独立可加）  
- 目标：尽快使所有节点能量达到阈值  

## 输入 / 输出

- 入：充电器/节点位置、并发功率模型  
- 出：充电器开关组合的调度序列（时间上）  

## 要点

- 归约 set cover 证 NP-hard  
- 子模集合覆盖视角的贪心 + 另一平衡充电贪心  
- GA 作对照  

## 来源

- [[src-guo-concurrent-ccsp]]
