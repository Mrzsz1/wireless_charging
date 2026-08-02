---
type: source
title: "Dynamic Power Distribution Controlling for Directional Chargers"
status: active
epistemic: high
year: 2024
venue: "IEEE INFOCOM"
doi: "10.1109/INFOCOM52122.2024.10621233"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Yuzhuo Ma", "Die Wu", "Jing Gao", "Wen Sun", "Jilin Yang", "Tang Liu"]
paper_keywords: ["directional charging", "power distribution controlling", "wave interference", "wireless power transfer"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [interference, causality_online]
objectives: [max_throughput]
method_family: online_algorithm
problem_class: online_scheduling
pdf_path: "raw/canonical/Dynamic_Power_Distribution_Controlling_for_Directional_Chargers.pdf-c51176d4-83bc-464b-8fc3-befdb45d519e/Dynamic_Power_Distribution_Controlling_for_Directional_Chargers.pdf"
raw_md: "raw/canonical/Dynamic_Power_Distribution_Controlling_for_Directional_Chargers.pdf-c51176d4-83bc-464b-8fc3-befdb45d519e/full.md"
why_relevant: "TIDE：定向充电器在线请求下动态朝向/功率分布控制，含干涉"
ingest_status: ingested
updated: 2026-07-14
---

# TIDE：定向充电器动态功率分布控制

## 一句话问题

传感器位置固定且发起**在线充电请求**时，如何动态调整**可旋转定向充电器**朝向（功率分布），在波干涉下最大化总体充电 utility。

## 系统设定与假设

- 静态可旋转定向充电器（扇区覆盖）+ 全向传感器
- 请求：剩余寿命低于阈值时发起；请求队列按时隙更新
- 时隙离散；朝向调整可在时隙内完成
- 重叠覆盖 → 干涉使“更多充电器覆盖”未必更高功率

## 方法要点

- 问题 **TIDE**：dynamic power distribution controlling
- 含干涉的定向充电模型；从连续朝向中抽取候选
- neighbor set 划分缩小耦合计算
- 在线更新 neighbor set 并选朝向
- 相对对比算法平均提升约 **142.62%**（文中）

## 主要结果

- 仿真 + 现场实验
- 强调相对“固定功率分布/仅部署优化”的灵活性

## 局限

- 在线决策复杂度与候选朝向离散化有关
- 依赖请求模型与时隙长度设定
- 实验结论依赖论文中的请求到达与设备布置设定

## 链接

- 概念：[[cpt-wave-interference]] · [[cpt-directional-charging]]
- 方法：[[mtd-tide-online-orientation]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]] · [[syn-mobility-online-service-scheduling]]

