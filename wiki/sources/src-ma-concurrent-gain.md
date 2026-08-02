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
updated: 2026-07-14
---

# Concurrent Charging with Wave Interference（GAIN）

## 一句话问题

如何在并发充电下**同时**利用建设性干涉的高功率区、规避破坏性干涉，联合部署充电器与传感器位置以最大化充电 utility。

## 系统设定与假设

- 固定数量全向充电器；PoI 周围有传感器可部署圆盘（SDD）
- 多充电器覆盖重叠 → 波干涉；功率非简单可加
- 传感器可在 PoI 附近有限范围内微调位置

## 方法要点

- **GAIN** 问题：充电器放置 + 传感器选最高功率点
- 建立含干涉的实用充电模型；研究增强区分布规律
- 充电器放置：最大化到达各 SDD 中心的“波功率基础”
- 部署区域离散/划分子区，在有限候选中选最优传感器位置
- 文称相对对比算法平均提升约 **40.48%** utility

## 主要结果

- 仿真 + 现场实验
- 模型与增强区规律可用于其他场景

## 局限

- 依赖传感器可微移；随机抛洒难达 mm 级精度
- 部署后功率分布相对固定（对比动态定向控制见 TIDE）
- 固定部署依赖场景几何稳定；跨场景复用需重新测量干涉分布

## 链接

- 概念：[[cpt-wave-interference]] · [[cpt-concurrent-charging]]
- 方法：[[mtd-gain-placement-interference]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]]

