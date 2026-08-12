---
type: method
subtype: algorithm
title: 峰值 AoI 充传联合调度
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [causality_online, qos]
objectives: [min_latency]
method_family: online_algorithm
problem_class: online_scheduling
updated: 2026-08-12
---

# Peak AoI 联合充电与传输

## TL;DR

把来源论文的连续/组合决策压缩为可执行的输入—输出流程；使用前必须先核对物理模型、信息结构和目标口径。

## 何时使用 / 何时不使用

- 无线供能边缘、定向充电  
- 目标为最大峰值 AoI，而非纯充电 utility

- 不满足来源论文的移动性、可加性、干涉、安全或在线信息假设时，不应只按算法名迁移。

## 输入 / 输出与变量

- **输入**：充电/传输时延、采样模型、带宽和能量因果。
- **输出**：充电顺序、上传顺序与最大峰值 AoI。
- 中间变量及符号沿用来源原文；实现时应保存随机种子、离散粒度、停止条件和不可行原因。

## 算法步骤

- 充电时延与峰值 AoI 的界  
- 单/多充电器与带宽约束算法

执行顺序应保持“候选空间构造 → 可行性/约束处理 → 优化或排序 → 结果核验”，不能跳过候选空间与物理约束校验。

## 复杂度与理论保证

单充电器算法为 1.5 近似；其他保证按场景定理。未由原文给出的复杂度、近似比或收敛性质均视为**原文未报告**。

## 实验验证与基线

采用来源论文的实例、基线和指标验证；百分比只对该实验口径有效。若用于新数据，应重跑消融、随机重复和敏感性分析。

## 失效边界

- 物理模型、请求到达方式或目标函数改变时，原保证可能失效。
- 元启发式/启发式的单次最好结果不能替代统计重复；离线算法不能自动声称在线保证。

## 证据与来源

- [[src-chen-peak-aoi-wpt]]

- 关键算法位置：原文第 165–540 行。
