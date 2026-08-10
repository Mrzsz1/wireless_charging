---
type: source
title: "Collaborative Charging Optimization for Wireless Rechargeable Sensor Networks via Heterogeneous Mobile Chargers"
status: active
epistemic: medium
year: 2025
venue: "arXiv preprint; IEEE Internet of Things Journal early-access DOI (2026)"
doi: "10.1109/JIOT.2026.3696627"
source_type: preprint
acquisition_method: auto_discovery
discovered_via: [arxiv]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-08-01
canonicalized_at: 2026-08-01
authors: ["Jianhang Yao", "Hui Kang", "Geng Sun", "Jiahui Li", "Hongjuan Li", "Jiacheng Wang", "Yinqiu Liu"]
paper_keywords: ["Wireless rechargeable sensor network", "collaborative charging optimization", "heterogeneous mobile chargers", "trust region policy optimization"]
keyword_source: index_terms
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_efficiency, min_energy, max_completion_rate, multi_objective]
method_family: rl
problem_class: routing_with_charging
pdf_path: "raw/canonical/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_Heterogeneous/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_He.pdf"
raw_md: "raw/canonical/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_Heterogeneous/full.md"
why_relevant: "以空中AAV和地面SV组成异构移动充电器，联合优化充电效率、移动能耗和节点死亡率。"
ingest_status: ingested
updated: 2026-08-11
---

# 异构移动充电器协同优化

## TL;DR

论文把空中 AAV 与地面 SV 的协同充电视为异构 Markov game，用共享全局状态、独立策略、信赖域更新、自注意力和 Beta 有界动作采样，联合优化充电效率、移动距离和节点死亡率。结果是仿真中的策略性能，不是确定性调度保证。

## 何时使用 / 何时不使用

- **使用**：空地充电器能力不同、动作连续、节点能量动态变化、可承担离线训练。
- **不使用**：缺少全局状态、必须分散执行、需要硬实时/可证明近似比，或环境障碍与动力学未建模。

## 系统模型与假设

AAV 与 SV 为两个异构 agent，服务二维区域内的传感器。系统记录节点位置与能量、AAV/SV 位置和能量预算；充电与移动能耗模型分别定义。论文当前采用全局可观测状态，且 raw 行 394 明确说明集中训练与集中执行，不能误标为 CTDE。

模型与符号见 raw §III，行 87–160。

## 变量、目标与约束

- 状态：所有节点能量与位置、AAV/SV 位置；
- 动作：各 agent 的有界连续移动方向和距离；
- 三个目标：提高充电效率 $f_1$、降低移动距离 $f_2$、降低节点死亡指标 $f_3$；
- 奖励：$r_i^t=\lambda_1f_{1,t}^i-\lambda_2f_{2,t}^i-\lambda_3f_{3,t}$。

完整问题与 reward 见 raw §IV–§V-A，行 161–329。权重改变会改变目标偏好，reward 提升不能替代各物理指标报告。

## 算法流程

1. 两个 agent 用自注意力从全局状态提取节点关系特征。
2. Actor 构造 Beta 分布，在动作边界内采样方向和距离。
3. 环境执行动作并计算充电效率、移动距离、死亡率组合奖励。
4. 轨迹缓冲区计算 GAE；critic 更新价值网络。
5. Actor 使用 HATRPO/TRPO 信赖域、共轭梯度和线搜索更新。

Algorithm 1 见 raw 行 270–322。

## 理论性质与复杂度

raw §V-C（行 388–419）给出训练和执行的时间/空间复杂度。注意力动作选择含 $N^2h$ 项；训练成本还受 agent 数、episode、步数、网络参数、共轭梯度和线搜索影响。执行期主要保留策略参数并计算注意力/动作。原文未给近似比或对分布外状态的保证。

## 实验设置与基线

- 100m×100m、100 个节点、AAV/SV 发射功率 3W、接收阈值 5mW、节点容量 2J、充电半径 6m。
- Actor/Critic 各两层 256 单元，自注意力 4 heads、embedding 256，训练 $6.5\times10^5$ iterations。
- Baselines：PPO、DDPG、MADDPG、HAPPO、HATRPO。
- 测试节点密度、区域大小、半径、能量预算、初始能量/位置分布、多个随机种子和 95% 置信区间。

详见 raw §VI-A，行 425–450。

## 定量结果

- 自注意力与 Beta sampling 组合相对原始 HATRPO 的总体 reward 提升约 **51%**（raw 行 556）。
- 曲线约在 200k iterations 后呈收敛趋势；论文同时报告各子目标与敏感性。
- “死亡率降至 10% 以下”等图形结论必须结合对应配置和图读取；不把单个曲线读数当作跨场景保证。

## 局限与失效条件

- 仅仿真，未纳入障碍、通信延迟、定位误差和真实飞行动力学。
- 依赖全局状态与集中执行。
- 加权 reward、训练分布和随机种子影响策略；不存在确定性安全保证。
- 论文年份按 2025 arXiv，DOI 对应 2026 early access，版本差异需保留。

## 证据定位

- Raw：`raw/canonical/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_Heterogeneous/full.md`
- 模型：§III，行 87–160；问题/reward：§IV–§V-A，行 161–329；算法/复杂度：§V-B–C，行 331–419；实验：§VI，行 421–636。

## 相关页面

- 模型：[[sys-heterogeneous-mobile-charger-coordination]] · [[sys-mobile-uav-routing-scheduling]]
- 目标：[[obj-energy-and-mobility-cost]] · [[obj-multi-objective-survivability]]
- 方法：[[mtd-ihatrpo-heterogeneous-charging]]
- 综合：[[syn-adaptive-mobile-charger-coordination]] · [[syn-mobile-uav-directional-scheduling]]
