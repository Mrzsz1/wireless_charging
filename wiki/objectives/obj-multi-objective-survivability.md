---
type: objective
title: 动态网络的多目标生存性优化
status: active
epistemic: medium
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_efficiency, min_energy, max_completion_rate, multi_objective]
method_family: rl
problem_class: energy_harvesting_scheduling
updated: 2026-08-11
---

# 动态网络的多目标生存性优化

## TL;DR

动态 WRSN 通常同时追求高充电效率、低移动代价和低节点死亡率。加权和可以训练策略，但权重改变会改变问题本身；reward 提升不等同于每个物理目标都同比改善。

## 统一表达

IHATRPO 使用形如

$$r_i^t=\lambda_1 f_{1,t}^i-\lambda_2 f_{2,t}^i-\lambda_3 f_{3,t}$$

的奖励，其中 $f_1$ 对应充电效率、$f_2$ 对应移动距离、$f_3$ 对应节点死亡指标。报告结果时必须同时给出各子目标，不能只给总 reward。

## 使用检查

- 权重是否归一、是否按 agent 异构设置；
- 死亡率定义及能量阈值；
- 训练分布与部署分布是否一致；
- 是否报告随机种子、置信区间和敏感性；
- 是否存在硬性生存约束，不能被高效用补偿。

## 证据

- [[src-yao2026-ihatrpo-heterogeneous-chargers]]：reward raw 行 261–329，仿真与敏感性 raw 行 425–556。
- [[src-liu2026-dchsa-adtsa-clustered]]：动态簇头和双阈值策略关注节点能量状态。
- [[src-qaisar2026-isac-uav-charging]]：在线队列与部分充电提供另一种生存性控制方式。
- [[src-binh2025-bilevel-metaheuristic-charging]]：能量耗尽最小化视角。

