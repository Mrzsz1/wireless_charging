---
type: method
subtype: algorithm
title: 双层元启发式移动充电
status: active
epistemic: medium
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, time_slot]
constraints: [mobility, min_soc]
objectives: [max_completion_rate, min_energy]
method_family: metaheuristic
problem_class: routing_with_charging
updated: 2026-08-01
---

# 双层元启发式移动充电

## 要点

- MLSGA以多起点和遗传算法搜索路径/时间联合空间。
- MTBCS以多任务和协方差自适应进化策略优化低层充电时间。
- 适合死亡节点数等离散服务结果为主要指标的场景。

## 来源

- [[src-binh2025-bilevel-metaheuristic-charging]]
