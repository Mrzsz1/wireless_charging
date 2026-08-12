---
type: source
title: "Infinite Drive: Optimal Urban Location of Dynamic Wireless Charging at Signalized Intersections"
status: active
epistemic: medium
year: 2026
venue: "arXiv preprint"
doi: ""
source_type: preprint
acquisition_method: auto_discovery
discovered_via: [arxiv]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-204713"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
authors: ["Yudai Honma", "Daisuke Hasegawa", "Katsuhiro Hata", "Xuesong Zhou", "Michael J. Kuby", "Takashi Oguchi"]
paper_keywords: ["Dynamic wireless power transfer (DWPT)", "Charging infrastructure location", "Signalized intersections", "Mixed-integer programming", "Battery sizing", "Battery manufacturing emissions"]
keyword_source: author_keywords
scenario: [ev_dynamic_charging]
entities: [transmitter, receiver, battery, path_or_route, grid_or_source]
constraints: [mobility, min_soc, deadline]
objectives: [min_cost, min_energy, max_completion_rate]
method_family: ilp_milp
problem_class: charger_placement
pdf_path: "raw/canonical/Infinite_Drive_Optimal_Urban_Location_of_Dynamic_Wireless_Charging_at_Signalized_Intersections/Infinite_Drive_Optimal_Urban_Location_of_Dynamic_Wireless_Charging_at_Signalized_Int.pdf"
raw_md: "raw/canonical/Infinite_Drive_Optimal_Urban_Location_of_Dynamic_Wireless_Charging_at_Signalized_Intersections/full.md"
why_relevant: "把信号交叉口处DWPT设施选址、电池容量和连续运行能力联系起来。"
ingest_status: ingested
updated: 2026-08-12
---

# 信号交叉口动态无线充电的城市部署

## TL;DR

在城市信号交叉口的加减速、排队和信号随机性下，如何选择动态无线充电路段并联合确定EV电池容量，使车辆连续运行且基础设施、车辆和充电成本较低。

## 何时使用 / 何时不使用

- **使用**：城市信号交叉口的停车/减速形成集中补能机会，需要联合选址、信号模式和电池规模。
- **不使用**：需要秒级车辆派遣、随机拥堵控制或完整经济/环境生命周期结论时。

## 系统模型与假设

- 研究对象是城市道路上的电动车及信号交叉口动态无线充电。
- 车辆行程、信号模式、排队停留时间和电池容量共同影响可获得能量。
- 以Kawagoe City案例和多种OD/聚合方式评估部署。

## 变量、目标与约束

- **变量/状态**：候选路段/交叉口 DWPT 部署、energy balancing group、信号方案、车辆轨迹能量平衡与电池容量。
- **目标与约束**：在车辆连续行驶能量可行条件下优化 DWPT 位置与规模，并分析设施—电池权衡。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 建立考虑信号动态和排队停留时间的混合整数规划模型。
- 决策包含DWPT位置、铺设长度和车辆电池容量；连续行程通过Monte Carlo验证。
- 通过不同安全系数、聚合方案和电池容量做敏感性分析。

## 理论性质与复杂度

采用混合整数规划表达选址和能量平衡；原文报告求解规模与计算环境，但没有在线实时调度竞争保证。

## 实验设置与基线

使用川越市交通流和车辆能耗数据，比较优化与均匀部署，并做基础设施—电池权衡仿真。

## 定量结果

- 基线情形约2,233m、56个交叉口的DWPT可支撑连续城市运行；最严格测试约4,291m。
- 论文报告了基础设施长度和电池容量之间的权衡，并估算电池制造排放变化。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 案例依赖单一城市网络、交通和信号建模；不等价于普适部署保证。
- 车辆能耗和运行时刻表的不确定性仍需更细致建模。

## 证据定位

- Raw：`raw/canonical/Infinite_Drive_Optimal_Urban_Location_of_Dynamic_Wireless_Charging_at_Signalized_Intersections/full.md`
- 概念：§2，第 86–118 行；模型：§3，第 119–387 行；数据：第 388–428 行；结果与运营验证：第 429–525 行。

## 相关页面

- 概念：[[cpt-dynamic-wireless-charging]]
- 方法：[[mtd-dwpt-intersection-placement]] · [[mtd-integrated-dwpt-battery-scheduling]]
- 综合：[[syn-dynamic-roadway-wpt-infrastructure]] · [[syn-mobility-online-service-scheduling]]
