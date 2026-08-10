---
type: source
title: "Dynamic Power Distribution Controlling for Directional Chargers"
status: active
epistemic: high
year: 2024
venue: "IEEE INFOCOM"
doi: "10.1109/INFOCOM52122.2024.10621233"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Yuzhuo Ma", "Die Wu", "Jing Gao", "Wen Sun", "Jilin Yang", "Tang Liu"]
paper_keywords: ["directional charging", "power distribution controlling", "wave interference", "wireless power transfer"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [interference, causality_online]
objectives: [max_throughput]
method_family: online_algorithm
problem_class: online_scheduling
pdf_path: "raw/canonical/Dynamic_Power_Distribution_Controlling_for_Directional_Chargers.pdf-c51176d4-83bc-464b-8fc3-befdb45d519e/Dynamic_Power_Distribution_Controlling_for_Directional_Chargers.pdf"
raw_md: "raw/canonical/Dynamic_Power_Distribution_Controlling_for_Directional_Chargers.pdf-c51176d4-83bc-464b-8fc3-befdb45d519e/full.md"
why_relevant: "TIDE：定向充电器在线请求下动态朝向/功率分布控制，含干涉"
ingest_status: ingested
updated: 2026-08-11
---

# TIDE：定向充电器动态功率分布控制

## TL;DR

TIDE 面向固定传感器的在线充电请求，把可旋转定向充电器的连续角度压缩为候选 sensor sets/朝向，再按 neighbor set 分组选择方向组合，在波干涉下最大化当前请求节点的总体 charging utility。

## 何时使用 / 何时不使用

- **使用**：节点位置固定、请求随时间到达、方向切换相对时隙足够快、多个定向充电器互相干涉。
- **不使用**：部署位置仍可优化时先考虑 GAIN；请求和未来状态全已知时应对比离线最优；方向切换成本不可忽略时需扩展模型。

## 系统模型与假设

静态定向充电器在扇区内发射，传感器全向接收。节点在剩余寿命低于阈值时提出请求；系统按队列更新时刻重新控制功率分布。相邻充电器覆盖重叠时，接收功率由幅度关系与相位共同决定。

## 变量、目标与约束

- 状态：请求节点集合、能量/寿命状态、候选覆盖集合和 neighbor sets。
- 动作：每个充电器的候选朝向。
- 目标：最大化当前请求节点总体 utility。
- 约束：只能使用当前信息；一个充电器一次选择一个方向；干涉使邻居动作耦合。

问题 RP1 及 NP-hard 结论位于 raw §II-D–§III-B，行 117–195。

## 算法流程

1. 连续旋转充电器，提取 dominant sensor sets，并加入必要子集形成候选 sensor sets/朝向。
2. 把具有相互影响的充电器划入 neighbor set。
3. 在每个 neighbor set 内按请求 utility 选择组合；过大的 set 进一步拆分以抑制指数增长。
4. 每次请求队列更新后重新执行，形成动态功率分布。

## 理论性质与复杂度

- 朝向选择问题 NP-hard（Theorem 1，raw 行 181–195）。
- 未分组时组合数随 neighbor set 内充电器候选数呈指数增长；分组和候选抽取是计算压缩手段。
- 原文未报告竞争比、近似比或对任意请求序列的性能保证。

## 实验设置与基线

仿真设置、比较方案位于 raw §IV（行 243–278）；小型现场 testbed 与结果位于 §V（行 279–300）。论文同时比较固定/较弱动态功率分布策略，但百分比必须放回其请求、拓扑和 utility 定义解释。

## 定量结果

论文报告相对比较算法，平均 charging utility 提升 **142.62%**（raw 行 25、60、322）。该数字不是对 GAIN 的直接同比，因为二者决策阶段、可控变量和测试分布不同。

## 局限与失效条件

- 候选方向和 neighbor 划分决定实时计算量与可能损失。
- 假定位置固定、方向切换及时，未显式计入机械磨损与切换能耗。
- 请求过程和寿命阈值变化可能改变结论。
- 干涉模型失配会导致方向选择错误。

## 证据定位

- Raw：`raw/canonical/Dynamic_Power_Distribution_Controlling_for_Directional_Chargers.pdf-c51176d4-83bc-464b-8fc3-befdb45d519e/full.md`
- 模型：§II，行 62–128；算法：§III，行 129–242；实验：§IV–V，行 243–300。

## 相关页面

- 模型：[[sys-online-directional-request]] · [[sys-interference-aware-concurrent-static]]
- 目标：[[obj-aggregate-charging-utility]]
- 概念：[[cpt-wave-interference]] · [[cpt-directional-charging]]
- 方法：[[mtd-tide-online-orientation]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]] · [[syn-mobility-online-service-scheduling]]

