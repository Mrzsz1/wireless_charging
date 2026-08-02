---
type: source
title: "ROSE: Robustly Safe Charging for Wireless Power Transfer"
status: active
epistemic: high
year: 2022
venue: "IEEE Transactions on Mobile Computing"
doi: "10.1109/TMC.2020.3032591"
source_type: paper
acquisition_method: auto_discovery
discovered_via: [serpapi]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-214003"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
authors: ["Haipeng Dai", "Yun Xu", "Guihai Chen", "Wanchun Dou", "Chen Tian", "Xiaobing Wu", "Tian He"]
paper_keywords: ["Robustly safe charging", "wireless power transfer", "approximation algorithm", "distribution algorithm"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, power_pool]
constraints: [thermal_or_sar, interference]
objectives: [max_throughput, max_efficiency]
method_family: convex_opt
problem_class: power_allocation
pdf_path: "raw/canonical/ROSE_Robustly_safe_charging_for_wireless_power_transfer/ROSE_Robustly_safe_charging_for_wireless_power_transfer.pdf"
raw_md: "raw/canonical/ROSE_Robustly_safe_charging_for_wireless_power_transfer/full.md"
why_relevant: "把EMR随机抖动导致的超阈值风险纳入无线充电功率调度，并给出集中式与分布式近似算法。"
ingest_status: ingested
updated: 2026-08-01
---

# ROSE：鲁棒安全无线充电

## 一句话问题

在EMR存在随机抖动时，如何最大化设备充电utility，同时保证任意区域超过EMR阈值的概率不高于给定风险。

## 系统设定与假设

- 多个静态无线充电器和设备位于二维平面。
- 充电功率与EMR受衰落、多径和阴影影响，安全要求是概率约束而非仅约束期望值。

## 方法要点

- 建立概率充电模型和EMR模型。
- 用EMR近似、区域离散化和二阶锥约束把问题转化为SOCP。
- 通过冗余约束删减降低集中式计算量，并提出可扩展的完全分布式算法。
- 对近似误差、区域划分和分布式求解损失给出界限。

## 主要结果

- 同时进行了仿真和现场实验；论文报告相对对比算法的显著提升。

## 局限

- 主要模型是静态充电器和二维区域，移动充电器等场景留作后续工作。
- 概率模型和安全阈值依赖实测分布、区域离散粒度与置信度设定。

## 链接

- 概念：[[cpt-probabilistic-emr-safety]] · [[cpt-wave-interference]]
- 方法：[[mtd-rose-robust-safe-power-scheduling]]
- 综合：[[syn-interference-aware-concurrent-wpt]]

