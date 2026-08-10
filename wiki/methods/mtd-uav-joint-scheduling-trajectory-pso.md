---
type: method
subtype: algorithm
title: PSOFKP/PSOD2P 充电UAV调度轨迹联合优化
status: active
epistemic: high
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy]
method_family: metaheuristic
problem_class: routing_with_charging
updated: 2026-08-11
---

# PSOFKP/PSOD2P 充电 UAV 调度轨迹优化

## TL;DR

PSOFKP 处理悬停点数量可变的连续—离散调度解，PSOD2P 处理已选悬停点的离散访问顺序。两者串联解决 JSTOP 的两个子问题，而不是在一个粒子中同步优化全部变量。

## 何时使用 / 适用条件

- 单 CUAV 离线规划；节点位置已知；悬停点数未知；访问顺序需要优化。
- 不适合必须实时响应、需要理论近似比或有复杂飞行动力学的任务。

## 输入 / 输出与变量

- PSOFKP 输入节点坐标、覆盖/充电半径和服务约束；输出悬停点集合及覆盖分配。
- PSOD2P 输入悬停点；输出访问排列和路径。
- 可变维度来自悬停点数量，离散维度来自访问顺序。

## 算法步骤

1. PSOFKP 初始化不同维度粒子；用 K-means 产生/调整悬停点。
2. 通过惩罚—补偿修复覆盖不足、重复覆盖和不可行解。
3. 固定调度结果后初始化 PSOD2P 的离散路径粒子。
4. 用离散化因子、2-opt 与路径交叉缩减改进访问顺序。
5. 达到迭代上限或收敛条件后输出两个阶段结果。

## 复杂度与理论保证

原文在 raw §V-E（行 485–532）按种群、迭代、节点和路径规模分析两种算法；2-opt 与粒子适应度计算是重要成本。两种算法均无确定性近似比和全局最优保证。

## 失效边界

- 第一阶段的悬停点误差会传递到第二阶段，路径优化无法纠正覆盖决策。
- 随机初始化和超参数影响稳定性。
- 二维欧氏路径不能代表障碍、风场或禁飞区能耗。

## 证据与来源

- [[src-liu2021-joint-cuav-scheduling-trajectory]]
- Raw §V-C–E，行 320–532；2-opt 描述从行 410 开始；仿真 §VI 行 533–672。
- 模型：[[sys-mobile-uav-routing-scheduling]]；目标：[[obj-energy-and-mobility-cost]]。
