---
type: source
title: "Charging on the Move: Scheduling Static Chargers with Tunable Power for Mobile Devices"
status: active
epistemic: high
year: 2021
venue: "IEEE/ACM International Symposium on Quality of Service (IWQoS)"
doi: "10.1109/IWQOS52092.2021.9521299"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Tao Wu", "Panlong Yang", "Haipeng Dai"]
paper_keywords: []
keyword_source: not_found
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, power_pool]
constraints: [peak_power, mobility]
objectives: [max_throughput]
method_family: heuristic
problem_class: power_allocation
pdf_path: "raw/canonical/Charging_on_the_Move_Scheduling_Static_Chargers_with_Tunable_Power_for_Mobile_Devices.pdf-b04f915c-4c38-444c-8103-1c9d81579ab7/Charging_on_the_Move_Scheduling_Static_Chargers_with_Tunable_Power_for_Mobile_Devices.pdf"
raw_md: "raw/canonical/Charging_on_the_Move_Scheduling_Static_Chargers_with_Tunable_Power_for_Mobile_Devices.pdf-b04f915c-4c38-444c-8103-1c9d81579ab7/full.md"
why_relevant: "静态充电器可调功率 + 移动接收端轨迹充电（Charging on the Move）"
ingest_status: ingested
updated: 2026-08-12
---

# Charging on the Move（可调功率静态充电器调度）

## TL;DR

给定静态充电器布局，如何为**沿轨迹移动**的可充电设备调度充电器发射功率档位，以提升整体充电 utility。

## 何时使用 / 何时不使用

- **使用**：静态充电器功率分档可调，移动设备轨迹已知或可预测，并有总发射功率预算。
- **不使用**：轨迹未知在线到达、速度强随机或相干波干涉导致功率不可加时。

## 系统模型与假设

- 静态充电器集合，功率离散为有限档位（含关闭）
- 移动设备沿**已知/可预测轨迹**移动，速度常数假设
- 充电模型：经验距离衰减公式；多充电器功率**可加**
- 设备有电池容量上限 → utility 饱和
- 总功率预算约束（budget）

## 变量、目标与约束

- **变量/状态**：每个充电器在各时段的离散功率档、轨迹分段、设备累计收能与饱和 utility。
- **目标与约束**：在各时刻总功率预算和电池容量下最大化所有移动设备的总充电效用。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 问题 **CM（Charging on the Move）**：可调功率调度 + 移动性导致时变接收功率
- 将时变功率近似为**分段常数**，按轨迹分段累计能量
- 固定功率档时证明目标**子模**，给出 $(1-1/e)/2$ 近似
- 可调多档进一步给出含 $\varepsilon,T$ 的近似保证

## 理论性质与复杂度

分段常数近似和轨迹离散把连续问题有限化；固定功率档得到 (1−1/e)/2 近似，多档保证含 ε 与离散粒度。

## 实验设置与基线

数值仿真与真实轨迹驱动评估，对比固定/可调功率基线；结果依赖轨迹预测和档位设置。

## 定量结果

- 数值仿真 + 轨迹驱动评估验证算法有效
- 相对基线提升充电 utility（文中报告）

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 轨迹已知/规则化假设；速度常数
- 功率可加，**未建模波干涉**非线性叠加
- 轨迹驱动结果依赖轨迹可预测性与功率档位设置

## 证据定位

- Raw：`raw/canonical/Charging_on_the_Move_Scheduling_Static_Chargers_with_Tunable_Power_for_Mobile_Devices.pdf-b04f915c-4c38-444c-8103-1c9d81579ab7/full.md`
- 模型/问题/硬度：§III，第 60–135 行；近似与离散：§IV-A–C，第 136–306 行；算法：第 307–562 行；实验：第 563–619 行。

## 相关页面

- 概念：[[cpt-charging-utility]] · [[cpt-wave-interference]]（本文未用干涉）
- 方法：[[mtd-tunable-power-mobile-traj]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-mobility-online-service-scheduling]]
