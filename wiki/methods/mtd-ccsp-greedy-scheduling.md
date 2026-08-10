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
updated: 2026-08-11
---

# CCSP 贪心调度

## TL;DR

先把每个可用充电器子集当成一个“服务动作”，再用子模集合覆盖贪心选择动作，直到所有节点达到容量。理论版有 $(\ln(ME)+1)$ 近似保证；平衡版更照顾低能量节点，但没有同类保证。

## 何时使用 / 适用条件

- 静态充电器与节点；组合功率已知；批量全充满；充电器数量足以枚举子集。
- 不适合在线未知请求、快速变化信道或大规模 $N$ 的直接枚举。

## 输入 / 输出与变量

- 输入：候选集合 $C_{\mathcal N}$、效用矩阵 $u_j^i$、节点容量 $E$、节点数 $M$。
- 状态：当前剩余需求 $E-er_j$。
- 输出：带重复次数的活跃集合序列；每个元素对应一个充电周期。

## 算法步骤

1. 计算截断边际效用 $\Pi_j^k=\min\{u_j^k,E-er_j\}$。
2. Greedy 每轮选择使 $f(C_K\cup\{S_i\})-f(C_K)$ 最大的集合。
3. 更新节点能量，直到 $f(C_K)=ME$。
4. Balanced 版选出最低能量的 $\lceil\alpha M\rceil$ 个节点，用“弱节点边际效用 + $\alpha$×全体边际效用”选择下一集合。

## 复杂度与理论保证

- 预处理需要处理 $2^N-1$ 个非空子集，是主要规模瓶颈。
- Greedy 的近似比为 $(\ln(ME)+1)$；证明依赖候选集合单周期至少贡献单位能量的离散化假设。
- Balanced 版与 GA 的严格近似/收敛保证原文未报告。

## 失效边界

- 组合功率测量错误会直接改变贪心边际收益。
- 总边际效用贪心可能让弱节点长期落后；Balanced 的 $\alpha$ 需要实验选择。
- 最短周期数不保证最低发射能耗或公平性。

## 证据与来源

- [[src-guo-concurrent-ccsp]]
- Raw §5.1–§5.2，行 218–302；Theorem 1 在行 265–283；实验 §6 行 355–477。
- 模型：[[sys-interference-aware-concurrent-static]]；目标：[[obj-full-charge-completion-time]]。
