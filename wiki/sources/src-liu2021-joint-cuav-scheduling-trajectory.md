---
type: source
title: "Joint Scheduling and Trajectory Optimization of Charging UAV in Wireless Rechargeable Sensor Networks"
status: active
epistemic: high
year: 2021
venue: "IEEE Internet of Things Journal"
doi: "10.1109/JIOT.2021.3132015"
source_type: paper
acquisition_method: auto_discovery
discovered_via: [openalex]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-214003"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
authors: ["Yanheng Liu", "Hongyang Pan", "Geng Sun", "Aimin Wang", "Jiahui Li", "Shuang Liang"]
paper_keywords: ["Wireless rechargeable sensor networks", "scheduling and trajectory optimization", "unmanned aerial vehicle", "particle swarm optimization"]
keyword_source: index_terms
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy, max_completion_rate]
method_family: metaheuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeable_Sensor_Net/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeabl.pdf"
raw_md: "raw/canonical/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeable_Sensor_Net/full.md"
why_relevant: "将充电UAV悬停点调度与飞行轨迹联合优化，补充移动WRSN的空中充电场景。"
ingest_status: ingested
updated: 2026-08-11
---

# 充电 UAV 的调度与轨迹联合优化

## TL;DR

论文把 CUAV 的悬停点选择、重复覆盖控制和访问路径统一为 JSTOP，但因连续/离散变量和目标依赖关系，将其分解为 CSOP 与 CTOP，再分别用可变维 PSOFKP 和离散 PSOD2P 求解。它是联合建模、分阶段求解，不是全局最优联合算法。

## 何时使用 / 何时不使用

- **使用**：二维 WRSN、单 CUAV、需要选择悬停点并规划访问顺序、允许元启发式离线计算。
- **不使用**：在线请求、强飞行动力学/障碍、多个异构充电器或必须证明近似比的场景。

## 系统模型与假设

传感器节点分布在二维区域；CUAV 在候选/连续位置悬停并覆盖一定半径内节点。CUAV 能耗包括悬停和飞行部分，充电时间与被服务节点需求相关。调度变量包含悬停点数、位置及覆盖关系，轨迹变量是悬停点访问顺序。

系统、无线充电和 CUAV 能耗模型见 raw §III，行 62–119。

## 变量、目标与约束

JSTOP 包含三个目标：

1. 最小化悬停点数量，以降低总悬停/充电时间；
2. 最小化被重复覆盖的传感器数量；
3. 在悬停点确定后最小化 CUAV 飞行路径。

约束要求所有传感器获得服务，并维持悬停点与覆盖关系可行。目标定义与公式见 raw §IV-A，行 120–205。

## 算法流程

1. 将 JSTOP 转换/分解为 charging scheduling optimization problem（CSOP）与 charging trajectory optimization problem（CTOP）。
2. PSOFKP 用可变维粒子表示不同数量悬停点，引入 K-means 算子和惩罚—补偿机制搜索 CSOP。
3. PSOD2P 用离散化因子表示访问顺序，引入 2-opt 和路径交叉缩减搜索 CTOP。
4. 先得到悬停点，再优化其访问路线。

## 理论性质与复杂度

- JSTOP 被证明为 NP-hard，且同时含连续、离散和可变维决策。
- 论文在 raw §V-E（行 485–532）分析 PSOFKP/PSOD2P 的收敛与复杂度；复杂度取决于粒子数、迭代次数、节点/悬停点规模和局部算子。
- 元启发式没有确定性近似比或全局最优保证。

## 实验设置与基线

raw §VI（行 533–672）分别评测 CSOP、CTOP 和稳定性，覆盖不同网络规模与设置；对照包括传统/其他 PSO 与路径优化方案。比较必须保持相同 CUAV、节点和迭代预算。

## 定量结果

论文报告 PSOFKP 与 PSOD2P 在多个规模和设置下优于对比算法，并通过稳定性试验支持 K-means、惩罚—补偿、2-opt 和交叉缩减的作用。Wiki 不摘录单个图上的 OCR 数值，以免把图读数当作表格精确值；具体数值应回到 raw §VI 的图表核验。

## 局限与失效条件

- 单 CUAV、二维几何和既定高度限制真实飞行迁移。
- 分解会固定前一阶段结果，无法保证联合全局最优。
- PSO 对粒子数、迭代次数和随机种子敏感。
- 未覆盖在线到达、障碍、通信冲突和多充电器协作。

## 证据定位

- Raw：`raw/canonical/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeable_Sensor_Net/full.md`
- 模型：§III，行 62–119；问题：§IV，行 120–280；算法与复杂度：§V，行 280–532；实验：§VI，行 533–672。

## 相关页面

- 模型：[[sys-mobile-uav-routing-scheduling]]
- 目标：[[obj-energy-and-mobility-cost]] · [[obj-full-charge-completion-time]]
- 方法：[[mtd-uav-joint-scheduling-trajectory-pso]]
- 综合：[[syn-mobile-uav-directional-scheduling]] · [[syn-mobility-online-service-scheduling]]

