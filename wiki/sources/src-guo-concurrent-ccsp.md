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
updated: 2026-07-14
---

# Concurrent Charging Scheduling（CCSP）

## 一句话问题

多静态充电器**并发**远距离射频充电时，如何调度开关组合，使所有节点尽快充到能量阈值 $E$。

## 系统设定与假设

- 静态充电器 + 固定传感器
- 远距离 RF 能量收集；充电器功率受 FCC 等限制 → 需多充电器协作
- **同频/窄带**并发 → 建设性/破坏性**波干涉**
- 单充电器对节点的 utility **不可独立加总**（非线性叠加）

## 方法要点

- 形式化 **CCSP**，证明 NP-hard（归约 set cover）
- 基于并发充电模型建立复合功率公式（相位差/距离差相关）
- 两种高效 **greedy**；其一给出近似比；另有 **genetic algorithm** 作对照
- 目标：最小时间使全网节点能量 ≥ $E$（类 TDMA 全充满后再通信）

## 主要结果

- 贪心接近 GA/小规模 brute force，运行时间远短于 GA
- 仿真 + 充电器 testbed 验证

## 局限

- 理想化同频/同 PSD 假设会放大干涉
- 与 on-demand RF-MAC 场景不同（本文偏预先充满）
- 论文采用 2017 年卷期年份；DOI 在线发表年份为 2016

## 链接

- 概念：[[cpt-wave-interference]] · [[cpt-concurrent-charging]]
- 方法：[[mtd-ccsp-greedy-scheduling]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]]

