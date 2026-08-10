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
updated: 2026-08-11
---

# IHATRPO 异构移动充电器协同策略优化

## TL;DR

IHATRPO 在 HATRPO 上增加自注意力状态编码和 Beta 有界动作采样，为 AAV/SV 分别训练策略；信赖域约束提高更新稳定性，但仍是依赖训练分布的多智能体强化学习方案。

## 何时使用 / 适用边界

- 动作连续、agent 能力不同、目标多维且显式优化难。
- 需要全局状态；当前实现是集中训练与集中执行。
- 不适合缺少训练预算、需要形式化安全/近似保证或存在未建模障碍的部署。

## 输入 / 输出与变量

- 输入：节点位置/能量、AAV/SV 位置与预算、目标权重 $\lambda_1,\lambda_2,\lambda_3$。
- 动作：每个 agent 的方向和距离，由 Beta 分布限制在边界内。
- 输出：AAV 与 SV 的策略参数和逐时隙移动/服务动作。

## 算法步骤

1. 构造异构 Markov game 和共享全局状态。
2. 自注意力编码节点之间的依赖。
3. Actor 输出 Beta 参数并采样有界连续动作。
4. 收集轨迹，按充电效率、距离和死亡率计算奖励。
5. 用 GAE 更新 critic；用 HATRPO/TRPO 的 KL 信赖域、共轭梯度和线搜索更新 actor。

## 复杂度与理论保证

- 注意力部分含节点规模的 $N^2h$ 项；训练还受 agent 数、episodes、horizon、网络参数、共轭梯度和线搜索影响。
- raw §V-C（行 388–419）给出完整训练/执行及空间复杂度表达式。
- 原文未报告收敛到全局最优、近似比或硬安全保证。

## 实验验证

对比 PPO、DDPG、MADDPG、HAPPO、HATRPO；报告密度、区域、半径、预算、初始状态、随机种子和消融。完整组件相对 HATRPO 总 reward 提高约 51%，但应同时查看三个子目标。

## 失效边界

- 分布外地形、障碍和状态噪声需要重新训练/验证。
- 集中执行的通信或状态收集失败会破坏策略输入。
- reward 权重可隐藏某一物理指标退化。

## 证据与来源

- [[src-yao2026-ihatrpo-heterogeneous-chargers]]
- Raw Markov game/reward 行 235–329；Algorithm 1 行 270–322；复杂度行 388–419；实验行 425–556。
- 模型：[[sys-heterogeneous-mobile-charger-coordination]]；目标：[[obj-multi-objective-survivability]]。
