---
type: source
title: "Concurrently Wireless Charging Sensor Networks with Efficient Scheduling"
status: active
epistemic: high
year: 2017
venue: "IEEE Transactions on Mobile Computing"
doi: "10.1109/TMC.2016.2624731"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Peng Guo", "Xuefeng Liu", "Shaojie Tang", "Jiannong Cao"]
paper_keywords: ["Wireless charging", "wireless sensor networks (WSNs)", "scheduling", "radio interference"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [interference, peak_power]
objectives: [min_latency, max_throughput]
method_family: heuristic
problem_class: multi_tx_coordination
pdf_path: "raw/canonical/Concurrently_Wireless_Charging_Sensor_Networks_with_Efficient_Scheduling.pdf-ab89513d-dd68-4bde-843f-8060d98fba2e/Concurrently_Wireless_Charging_Sensor_Networks_with_Efficient_Scheduling.pdf"
raw_md: "raw/canonical/Concurrently_Wireless_Charging_Sensor_Networks_with_Efficient_Scheduling.pdf-ab89513d-dd68-4bde-843f-8060d98fba2e/full.md"
why_relevant: "并发充电调度 CCSP；射频干涉导致非线性叠加"
ingest_status: ingested
updated: 2026-08-11
---

# Concurrent Charging Scheduling（CCSP）

## TL;DR

CCSP 将每个充电周期内开启的充电器表示为一个集合，在非线性波干涉下选择集合序列及持续周期，使所有传感器达到能量阈值且总周期数最少。问题是 NP-hard；SSCP 贪心具有 $(\ln(ME)+1)$ 近似保证，balanced greedy 通常更好但原文没有给出同类近似比。

## 何时使用 / 何时不使用

- **使用**：静态同频 RF 充电器、固定节点、组合功率不可加、目标是批量全充满。
- **不使用**：请求在线到达、节点持续移动、允许不同频段正交并发，或只需最大化当期总收能。

## 系统模型与假设

充电器集合 $\mathcal C=\{c_i\mid 1\le i\le N\}$，节点集合 $S=\{s_j\mid 1\le j\le M\}$。每个充电周期长度为 $\Delta$，节点容量统一为 $E$。活跃集合 $S_k\subseteq\mathcal C$ 在节点 $j$ 产生组合效用 $u_j^k$；该值由组合波形决定，而非单充电器效用之和。

论文采用先充电、后通信的 TDMA 周期，并讨论充电与通信共存；因此它不是一般的边充边传 RF-MAC 模型。

## 变量、目标与约束

决策是活跃集合 $S_1,\ldots,S_p$ 及各集合使用的整数周期数 $\gamma_1,\ldots,\gamma_p$：

$$
\min \sum_{k=1}^{p}\gamma_k,
\qquad
\text{s.t. }\sum_{k=1}^{p}\gamma_k u_j^k\ge E,\ \forall j=1,\ldots,M.
$$

完整定义见 raw §4.1，行 162–188。若节点需求不同，应扩展为 $E_j$；该扩展不属于原文已验证结论。

## 算法流程

1. 枚举 $N$ 个充电器的所有非空子集，预计算每个集合对每个节点的效用；候选数为 $2^N-1$。
2. 定义带电池饱和截断的边际收能，并构造 submodular set cover（SSCP）。
3. **Greedy CCSP** 每轮选择带来最大总边际收能的集合。
4. **Balanced CCSP** 额外关注当前能量最低的 $\lceil\alpha M\rceil$ 个节点，在总收能与弱节点收能之间平衡。
5. GA 作为高计算成本对照。

## 理论性质与复杂度

- CCSP 由 set cover 归约证明 NP-hard（raw §4.2，行 190–212）。
- Greedy CCSP 是 $(\ln(ME)+1)$-approximation（Theorem 1，raw 行 265–283）。
- 候选集合枚举随充电器数指数增长；论文假设实际 $N$ 不大。Balanced CCSP 的同类近似比原文未报告。

## 实验设置与基线

- Matlab 随机部署，50m×50m；发射功率 4W，转换效率 0.25，周期 20s，915MHz 对应波长约 0.33m，收能阈值 15μW。
- 对比 Greedy CCSP、Balanced CCSP、GA，并在小规模上参考 brute force。
- 实测 testbed：1 个 RF 信号源、3 个充电器、4 个自制收能节点。

参数与 testbed 见 raw §6，行 357–477。

## 定量结果

论文报告两种贪心的调度长度接近 GA/小规模最优参考，但运行时间显著更低。表 3 的一个实例中，Greedy、Balanced、GA 的发射总能量分别为 18.88kJ、20.24kJ、16.32kJ；这说明“完成时间较短”不自动等于“发射能耗最低”。

## 局限与失效条件

- $2^N-1$ 组合预计算限制大规模充电器数量。
- 同频、相位与距离模型需要校准；环境变化会使预计算效用失真。
- 统一容量、空初始能量和批量全充满不覆盖异质请求与截止时间。
- 论文年份按 2017 卷期记录，DOI 在线发表为 2016。

## 证据定位

- Raw：`raw/canonical/Concurrently_Wireless_Charging_Sensor_Networks_with_Efficient_Scheduling.pdf-ab89513d-dd68-4bde-843f-8060d98fba2e/full.md`
- 模型：§3–§4，行 77–212；算法与近似比：§5，行 218–304；实验：§6，行 355–477。

## 相关页面

- 模型：[[sys-interference-aware-concurrent-static]]
- 目标：[[obj-full-charge-completion-time]] · [[obj-aggregate-charging-utility]]
- 概念：[[cpt-wave-interference]] · [[cpt-concurrent-charging]]
- 方法：[[mtd-ccsp-greedy-scheduling]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]]

