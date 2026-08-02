---
type: method
subtype: algorithm
title: TIDE 在线朝向/功率分布控制
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [interference, causality_online]
objectives: [max_throughput]
method_family: online_algorithm
problem_class: online_scheduling
updated: 2026-07-10
---

# TIDE 在线定向控制

## 适用条件

- 定向可旋转充电器 + 在线充电请求  
- 传感器位置固定  
- 需要应对干涉下的动态功率分布  

## 输入 / 输出

- 入：请求队列、剩余能量、充电器候选朝向  
- 出：各时隙朝向组合  

## 要点

- 候选朝向抽取 + neighbor set  
- 在线更新并最大化请求节点 utility  

## 来源

- [[src-ma-tide-dynamic-power]]
