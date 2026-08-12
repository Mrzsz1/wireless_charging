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
updated: 2026-08-12
---

# ROSE 鲁棒安全功率调度

## TL;DR

把来源论文的连续/组合决策压缩为可执行的输入—输出流程；使用前必须先核对物理模型、信息结构和目标口径。

## 何时使用 / 何时不使用

- 静态充电器和设备，EMR存在抖动、衰落或多径不确定性。
- 需要概率安全阈值和整体充电utility的权衡。

- 不满足来源论文的移动性、可加性、干涉、安全或在线信息假设时，不应只按算法名迁移。

## 输入 / 输出与变量

- **输入**：充电/EMR 概率模型、安全阈值、风险与误差参数。
- **输出**：各充电器安全功率。
- 中间变量及符号沿用来源原文；实现时应保存随机种子、离散粒度、停止条件和不可行原因。

## 算法步骤

- 概率模型经近似和区域离散化后转为SOCP。
- 集中式算法删减冗余二阶锥约束；分布式算法按区域分解并给出近似界。

执行顺序应保持“候选空间构造 → 可行性/约束处理 → 优化或排序 → 结果核验”，不能跳过候选空间与物理约束校验。

## 复杂度与理论保证

SOCP 近似；集中式/分布式均有误差与性能分析。未由原文给出的复杂度、近似比或收敛性质均视为**原文未报告**。

## 实验验证与基线

采用来源论文的实例、基线和指标验证；百分比只对该实验口径有效。若用于新数据，应重跑消融、随机重复和敏感性分析。

## 失效边界

- 物理模型、请求到达方式或目标函数改变时，原保证可能失效。
- 元启发式/启发式的单次最好结果不能替代统计重复；离线算法不能自动声称在线保证。

## 证据与来源

- [[src-dai2022-rose-robust-safe-charging]]

- 关键算法位置：原文第 159–461 行。
