---
type: dataset-or-sim
title: WRSN 调度仿真与证据报告协议
status: active
epistemic: high
scenario: [sensor_rf_energy, uav_wpt, multi_device_wpt]
entities: [transmitter, receiver, battery, path_or_route, time_slot]
constraints: [interference, mobility, min_soc]
objectives: [min_latency, max_throughput, max_efficiency, min_energy, max_completion_rate, multi_objective]
method_family: ""
problem_class: ""
updated: 2026-08-11
---

# WRSN 调度仿真与证据报告协议

## TL;DR

这不是统一公开数据集，而是跨论文读取和复现实验时应记录的最小协议。没有场景、功率模型、请求过程、baseline 和随机性信息，单个百分比提升无法迁移。

## 最小记录表

| 维度 | 必须记录 |
|---|---|
| 空间 | 区域大小、节点/充电器数、位置分布、障碍、维度 |
| 能量 | 发射功率、接收阈值、充电模型、容量、初始能量、消耗率 |
| 时间 | 时隙/充电周期、请求到达、截止时间、总仿真时长 |
| 移动 | 速度、高度、能耗模型、充电半径、路径约束 |
| 算法 | 超参数、停止条件、训练步数、随机种子 |
| 对照 | baseline 名称、是否重实现、相同参数与预算 |
| 输出 | 每个物理指标、运行时间、方差/置信区间、失败案例 |
| 证据 | raw 章节与行号、表/图编号、PDF 页码（若映射可靠） |

## 已有可复核样板

- [[src-guo-concurrent-ccsp]]：Matlab，50m×50m，4W、效率 0.25、20s 周期、915MHz/0.33m、15μW 阈值；raw 行 357–371。
- [[src-ma-concurrent-gain]]：simulation setup、baseline、field experiment 分列于 raw §V–VI（行 366–424）。
- [[src-ma-tide-dynamic-power]]：simulation 与 field test 分列于 raw §IV–V（行 243–300）。
- [[src-yao2026-ihatrpo-heterogeneous-chargers]]：100m×100m、100 节点、3W、5mW 阈值、2J、6m 半径，以及网络/训练参数；raw 行 425–450，并报告多种种子和 95% 区间。

## 比较规则

1. 不跨不同 utility、死亡阈值或预算直接比较百分比；
2. 学习算法至少报告多个种子和各子目标；
3. 干涉模型必须说明是相干叠加、经验测量还是功率相加；
4. 仿真与实测结论分开陈述；
5. “原文未报告”的字段保持缺失，不用经验值补齐。

## 相关页面

- [[map-models-and-objectives]] · [[syn-wrsn-scheduling-placement]]
