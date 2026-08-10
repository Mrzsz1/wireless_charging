---
type: objective
title: 总体充电效用最大化
status: active
epistemic: high
scenario: [sensor_rf_energy, multi_device_wpt]
entities: [transmitter, receiver, user_or_request]
constraints: [interference]
objectives: [max_throughput]
method_family: ""
problem_class: power_allocation
updated: 2026-08-11
---

# 总体充电效用最大化

## TL;DR

把各节点在当前部署、请求或时隙中的有效收能映射成有界 utility 后求和。它适合比较部署和在线控制，但必须说明饱和函数、请求集合和公平性，否则“总效用提高”可能来自少数高功率节点。

## 统一表达

$$\max \sum_{j\in R_t} U_j(P_j),$$

其中 $R_t$ 是当前请求/服务节点，$P_j$ 是干涉后的接收功率，$U_j$ 可包含接收阈值和饱和上限。GAIN 在部署阶段最大化所有可部署传感器的总体效用；TIDE 在请求到达后逐次选择方向组合；CCSP 则把效用作为达到能量阈值的约束系数。

## 使用检查

1. utility 是功率、能量还是归一化收益；
2. 是否只统计当前请求节点；
3. 是否有饱和和最低接收阈值；
4. 是否需要加入公平性或最小服务约束；
5. 报告的百分比提升是否使用同一 baseline 与场景。

## 证据

- [[src-ma-concurrent-gain]]：GAIN 问题与 utility，raw 行 117–138、250–271。
- [[src-ma-tide-dynamic-power]]：TIDE 请求效用与方向选择，raw 行 107–128、181–242。
- [[src-guo-concurrent-ccsp]]：组合 utility 作为能量约束系数，raw 行 168–188。

