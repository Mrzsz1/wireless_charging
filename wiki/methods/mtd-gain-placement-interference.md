---
type: method
subtype: algorithm
title: GAIN 干涉感知并发放置
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver]
constraints: [interference]
objectives: [max_throughput]
method_family: heuristic
problem_class: charger_placement
updated: 2026-08-11
---

# GAIN 放置方案

## TL;DR

GAIN 把连续部署空间压缩为由覆盖集合和几何圆环形成的有限子区域，先贪心放置充电器，再在每个 SDD 内寻找建设性干涉高功率落点。

## 何时使用 / 适用条件

- 部署阶段可控制充电器位置，传感器可在 PoI 周围移动。
- 场景几何和 RF 参数足够稳定，可离线测量或计算干涉分布。
- 不适合固定传感器的在线请求；该情况参考 [[mtd-tide-online-orientation]]。

## 输入 / 输出与变量

- 输入：PoI、SDD、覆盖半径、充电器预算、相干充电模型和 utility。
- 输出：充电器位置以及各 SDD 中的传感器落点。
- 中间结构：MCS、候选充电器区域、离散子区域、干涉增强区。

## 算法步骤

1. 提取 maximal covering sets 及其候选区域。
2. 通过与各 PoI 的距离分层离散连续区域。
3. 在候选点中按加性功率基础的边际增益贪心选充电器。
4. 根据两波条纹和多波点状增强区规律，在 SDD 内寻找局部最高相干功率点。

## 复杂度与理论保证

- GAIN 问题 NP-hard。
- 候选规模随覆盖组合和离散精度增长；更细离散提高空间精度但增加计算。
- 原文未报告端到端近似比或统一复杂度上界，不能把贪心步骤本身称为有保证的近似算法。

## 失效边界

- 部署误差、反射或遮挡使预测增强区偏移。
- 传感器不可移动时，第二阶段失去可控变量。
- 最大总 utility 可能忽略弱节点，需额外公平/最低功率约束。

## 证据与来源

- [[src-ma-concurrent-gain]]
- Raw §III–IV，行 139–365；Algorithm 1 行 159 起，Algorithm 2 行 205 起；实验 §V–VI 行 366–424。
- 模型：[[sys-interference-aware-concurrent-static]]；目标：[[obj-aggregate-charging-utility]]。
