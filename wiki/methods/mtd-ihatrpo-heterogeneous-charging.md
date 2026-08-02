---
type: method
subtype: algorithm
title: IHATRPO异构移动充电器协同策略优化
status: active
epistemic: medium
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_efficiency, min_energy, max_completion_rate, multi_objective]
method_family: rl
problem_class: routing_with_charging
updated: 2026-08-01
---

# IHATRPO异构移动充电器协同策略优化

## 输入与输出

- 输入：节点状态、空中AAV与地面SV状态、能量预算和多目标权重。
- 输出：两类充电器的连续移动与服务策略。

## 算法骨架

1. 将异构协作表述为Markov game；
2. 自注意力编码复杂环境状态；
3. 在HATRPO信赖域更新中引入Beta采样，处理连续动作；
4. 以充电效率、移动能耗和死亡率的组合reward训练协同策略。

## 适用边界

- 适合动作连续、agent能力不同且难以手工分解的空地协同问题。
- 训练结果不提供确定性近似比，障碍和分布外状态需要重新验证。

## 来源

- [[../sources/src-yao2026-ihatrpo-heterogeneous-chargers]]
