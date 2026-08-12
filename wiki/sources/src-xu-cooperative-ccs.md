---
type: source
title: "Cooperative Charging as Service: Scheduling for Mobile Wireless Rechargeable Sensor Networks"
status: active
epistemic: high
year: 2021
venue: "IEEE International Conference on Distributed Computing Systems (ICDCS)"
doi: "10.1109/ICDCS51616.2021.00071"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Jia Xu", "Suyi Hu", "Sixu Wu", "Kaijun Zhou", "Haipeng Dai", "Lijie Xu"]
paper_keywords: ["WRSN", "cooperative charging service", "submodular function", "coalition formation game", "Nash Equilibrium"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, user_or_request]
constraints: [qos]
objectives: [min_cost]
method_family: game_theory
problem_class: offline_scheduling
pdf_path: "raw/canonical/Cooperative_Charging_as_Service_Scheduling_for_Mobile_Wireless_Rechargeable_Sensor_Networks.pdf-8bd73c94-27f3-4b2a-aafb-4d318e567433/Cooperative_Charging_as_Service_Scheduling_for_Mobile_Wireless_Rechargeable_Sensor_Networks.pdf"
raw_md: "raw/canonical/Cooperative_Charging_as_Service_Scheduling_for_Mobile_Wireless_Rechargeable_Sensor_Networks.pdf-8bd73c94-27f3-4b2a-aafb-4d318e567433/full.md"
why_relevant: "合作充电服务经济模型 CCS：设备调度到充电器、成本分摊"
ingest_status: ingested
updated: 2026-08-12
---

# Cooperative Charging Scheduling（CCS）

## TL;DR

把无线充电当作**付费服务**时，如何把移动可充电设备调度到固定全向充电器，并做组内成本分摊，最小化综合成本（充电费+移动成本）。

## 何时使用 / 何时不使用

- **使用**：无线充电按服务收费，移动设备可选择固定充电器，并需要稳定的组内成本分摊。
- **不使用**：无价格机制、相干干涉或实时 deadline 是主问题时；Nash 均衡不等于社会最优。

## 系统模型与假设

- 固定位置全向充电器，可属不同 CSP、不同单价
- 移动设备需移动到充电器固定充电距离处充电
- 同充电器组共享充电时段成本（按最长充电时间计费）
- Friis 型充电功率模型；功率可加假设

## 变量、目标与约束

- **变量/状态**：设备—充电器分组、移动距离、共享服务时长、比例/Shapley 成本份额和联盟结构。
- **目标与约束**：最小化充电服务成本与设备移动成本之和，同时维持分摊的个体理性/联盟稳定性。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 问题 **CCS**；两种组内成本分摊方案维持合作稳定
- **CCSA**：贪心 + 子模函数最小化，近似比 $(\ln n+1)/(1-\varepsilon)$
- 大规模：**CCS 博弈 + CCSGA**，收敛到纯 Nash 均衡
- 仿真：CCSA 综合成本比非合作低约 27.3%，距最优约 +7.3%
- 现场：5 充电器 + 8 节点，综合成本优于非合作约 42.9%

## 理论性质与复杂度

CCSA 近似比为 (ln n+1)/(1−ε)；CCSGA 是联盟形成博弈并收敛到纯 Nash 均衡。

## 实验设置与基线

仿真比较综合成本与运行时间，并用 5 个充电器、8 个节点做现场实验；价格模型决定外部有效性。

## 定量结果

- 强调“商业化充电服务”视角，区别于传统部署/移动车路径
- 大网络更适合博弈算法速度

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 全向、固定充电距离；经济模型假设
- 与干涉/定向模型正交
- 成本分摊与均衡结论依赖论文给定的服务价格和移动成本模型

## 证据定位

- Raw：`raw/canonical/Cooperative_Charging_as_Service_Scheduling_for_Mobile_Wireless_Rechargeable_Sensor_Networks.pdf-8bd73c94-27f3-4b2a-aafb-4d318e567433/full.md`
- 模型：§III，第 84–149 行；成本分摊：§IV，第 150–187 行；CCSA：第 188–380 行；博弈：第 381–474 行；实验：第 475–551 行。

## 相关页面

- 概念：[[cpt-charging-utility]]
- 方法：[[mtd-ccs-cooperative-service]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-mobility-online-service-scheduling]]
