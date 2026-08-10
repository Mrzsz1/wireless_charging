---
type: source
title: "Concurrent Charging with Wave Interference"
status: active
epistemic: high
year: 2023
venue: "IEEE INFOCOM"
doi: "10.1109/INFOCOM53939.2023.10228965"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Yuzhuo Ma", "Dié Wu", "Meixuan Ren", "Jian Peng", "Jilin Yang", "Tang Liu"]
paper_keywords: ["wave interference", "concurrent charging", "charger placement", "sensor deployment", "wireless power transfer"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver]
constraints: [interference]
objectives: [max_throughput]
method_family: heuristic
problem_class: charger_placement
pdf_path: "raw/canonical/Concurrent_Charging_with_Wave_Interference.pdf-a984b0a6-dc76-4283-951d-f75a915cd6eb/Concurrent_Charging_with_Wave_Interference.pdf"
raw_md: "raw/canonical/Concurrent_Charging_with_Wave_Interference.pdf-a984b0a6-dc76-4283-951d-f75a915cd6eb/full.md"
why_relevant: "GAIN：利用建设性干涉做充电器部署+传感器落点"
ingest_status: ingested
updated: 2026-08-11
---

# Concurrent Charging with Wave Interference（GAIN）

## TL;DR

GAIN 在部署阶段联合决定充电器位置和传感器在 PoI 周围可部署圆盘内的落点，使建设性干涉增强区靠近需求点并最大化总体充电效用。问题 NP-hard；方案使用最大覆盖集合、几何离散和边际收益贪心，没有给出全局近似比。

## 何时使用 / 何时不使用

- **使用**：部署前可调整充电器位置，传感器允许在 PoI 邻域内微移，场景几何长期稳定。
- **不使用**：传感器位置完全固定、请求在线变化，或部署误差远大于波长尺度。

## 系统模型与假设

场景包含固定数量全向充电器、PoI，以及每个 PoI 周围的 sensor deployable disk（SDD）。处于多个覆盖区的传感器接收相干叠加功率；效用函数对低于阈值的功率不计收益，并在高功率处饱和。

## 变量、目标与约束

- 充电器位置集合 $C$；
- 每个 PoI 的传感器位置位于对应 SDD 内；
- 目标：最大化所有传感器的总 charging utility；
- 约束：充电器数量固定，位置属于候选部署区域，传感器不离开 SDD。

问题 P1 及 NP-hard 结论见 raw §II-D，行 127–138。

## 算法流程

1. 从 PoI 覆盖关系提取 maximal covering sets（MCS）及候选充电器区域。
2. 用以 PoI 为中心的同心圆把连续候选区域离散为子区域。
3. 以加性功率基础的边际收益贪心选择固定数量的充电器位置，使增强区靠近 PoI。
4. 对两个充电器覆盖的 PoI 分析条纹状增强区；对三个及以上覆盖分析点状增强区。
5. 在每个 SDD 内寻找预测高功率位置放置传感器。

## 理论性质与复杂度

- GAIN 为 NP-hard（Theorem 1）。
- 候选区域和子区域数量受 PoI 覆盖组合及离散精度影响；传感器局部搜索还受并发波数量影响。
- 原文未报告整体算法的近似比，也未给出可直接跨规模引用的统一大 O 复杂度。

## 实验设置与基线

论文分别报告 simulation setup、baseline setup、performance comparison 和 field experiments；详见 raw §V–VI，行 366–424。仿真与实测应分开解释，不能把几何模型精度直接等同于现场部署精度。

## 定量结果

论文报告相对其比较算法，充电效用平均提高 **40.48%**（摘要、贡献及结论，raw 行 29、67、443）。该百分比依赖其 PoI、SDD、充电器数量和 baseline，不能直接与 TIDE 的在线提升百分比横比。

## 局限与失效条件

- 需要传感器在 SDD 内实现较精细的落点控制。
- 部署完成后功率分布近似固定，不能响应动态请求。
- 相位、反射与遮挡变化会移动增强/减弱区域。
- 使用加性功率作为部署代理与最终相干效用之间存在模型差异。

## 证据定位

- Raw：`raw/canonical/Concurrent_Charging_with_Wave_Interference.pdf-a984b0a6-dc76-4283-951d-f75a915cd6eb/full.md`
- 模型与问题：§II，行 69–138；部署算法：§III–IV，行 139–365；实验：§V–VI，行 366–424。

## 相关页面

- 模型：[[sys-interference-aware-concurrent-static]]
- 目标：[[obj-aggregate-charging-utility]]
- 概念：[[cpt-wave-interference]] · [[cpt-concurrent-charging]]
- 方法：[[mtd-gain-placement-interference]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]]

