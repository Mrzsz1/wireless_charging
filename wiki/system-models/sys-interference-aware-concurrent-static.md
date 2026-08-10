---
type: system-model
title: 干涉感知的多静态充电器并发模型
status: active
epistemic: high
scenario: [sensor_rf_energy, multi_device_wpt]
entities: [transmitter, receiver, time_slot]
constraints: [interference, peak_power]
objectives: [min_latency, max_throughput]
method_family: ""
problem_class: multi_tx_coordination
updated: 2026-08-11
---

# 干涉感知的多静态充电器并发模型

## TL;DR

多个同频 RF 充电器同时工作时，接收功率取决于幅度和相位叠加，不能把各充电器的独立功率简单相加。调度变量可以是每时隙的开关集合，也可以是部署位置或朝向。

## 何时使用

- 多个静态充电器的覆盖区重叠；
- 目标是缩短全充满时间或提高总体充电效用；
- 环境允许估计距离、相位差或直接测量组合功率。

## 何时不使用

- 充电器工作在正交频段/时段，组合功率确实近似可加；
- 只研究单充电器或近场互感电路，不涉及远距离 RF 波叠加；
- 位置和相位变化快到离线组合功率表失效。

## 形式化骨架

设充电器集合 $C=\{c_i\}$、接收节点集合 $S=\{s_j\}$。对于活跃充电器子集 $A\subseteq C$，节点 $j$ 的收能效用写作 $u_j(A)$。关键性质是通常存在

$$u_j(A)\neq\sum_{c_i\in A}u_j(\{c_i\}),$$

且加入一个充电器可能因破坏性干涉降低 $u_j(A)$。CCSP 进一步用整数时段数 $\gamma_k$ 选择活跃集合 $A_k$，满足每个节点累计能量阈值；GAIN 把决策移到充电器与传感器位置；TIDE 把决策移到定向充电器朝向。

## 模型变体

| 变体 | 决策 | 代表证据 |
|---|---|---|
| 开关调度 | 活跃集合及持续时段 | [[src-guo-concurrent-ccsp]] |
| 联合部署 | 充电器位置与传感器局部落点 | [[src-ma-concurrent-gain]] |
| 在线朝向 | 请求到达后的候选方向组合 | [[src-ma-tide-dynamic-power]] |

## 证据位置

- CCSP：`raw/.../Concurrently.../full.md`，§3.1–§4.2，尤其行 162–192。
- GAIN：`raw/.../Concurrent_Charging_with_Wave_Interference.../full.md`，§II-B–D，行 77–138。
- TIDE：`raw/.../Dynamic_Power_Distribution.../full.md`，§II-B–D，行 75–128。

## 相关页面

- 目标：[[obj-full-charge-completion-time]] · [[obj-aggregate-charging-utility]]
- 方法：[[mtd-ccsp-greedy-scheduling]] · [[mtd-gain-placement-interference]] · [[mtd-tide-online-orientation]]
- 概念：[[cpt-wave-interference]] · [[cpt-concurrent-charging]]
- 综合：[[syn-interference-aware-concurrent-wpt]]

