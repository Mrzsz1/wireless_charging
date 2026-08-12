---
type: source
title: "Peak AoI Minimization at Wireless-Powered Network Edge: From the Perspective of Both Charging and Transmitting"
status: active
epistemic: high
year: 2024
venue: "IEEE/ACM Transactions on Networking"
doi: "10.1109/TNET.2023.3303266"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Quan Chen", "Song Guo", "Zhipeng Cai", "Jing Li", "Tuo Shi", "Hong Gao"]
paper_keywords: ["Age of information (AoI)", "maximum peak AoI", "directional charging", "wireless-powered network"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot, user_or_request]
constraints: [causality_online, qos]
objectives: [min_latency]
method_family: online_algorithm
problem_class: online_scheduling
pdf_path: "raw/canonical/Peak_AoI_Minimization_at_Wireless-Powered_Network_Edge_From_the_Perspective_of_Both_Charging_and_Transmitting.pdf-a84be74f-fb17-427c-9f09-73ff009d6e42/Peak_AoI_Minimization_at_Wireless-Powered_Network_Edge_From_the_Perspective_of_Both_Charging_and_Transmitting.pdf"
raw_md: "raw/canonical/Peak_AoI_Minimization_at_Wireless-Powered_Network_Edge_From_the_Perspective_of_Both_Charging_and_Transmitting.pdf-a84be74f-fb17-427c-9f09-73ff009d6e42/full.md"
why_relevant: "定向充电 + 数据传输联合调度，优化最大峰值 AoI"
ingest_status: ingested
updated: 2026-08-12
---

# Peak AoI 最小化（充电与传输联合）

## TL;DR

在无线供能网络边缘，如何**同时**调度定向充电器的充电朝向/时序与源节点数据传输，以优化**最大峰值 Age of Information**。

## 何时使用 / 何时不使用

- **使用**：无线供能边缘同时存在充电与数据上传，目标是控制最坏节点信息新鲜度而不是只最大化收能。
- **不使用**：只关心电池补能、没有数据更新过程，或物理层干涉是主矛盾时。

## 系统模型与假设

- 多源节点 + BS；时隙系统
- **定向充电器**：扇区覆盖，朝向决定可充节点集合
- 能量不可控采集模型不够 → 主动 WPT 调度
- 考虑单充电器 / 多充电器 / 带宽约束等场景
- 采样模型：周期、随机等

## 变量、目标与约束

- **变量/状态**：充电顺序/方向、各节点充电完成时刻、采样与上传顺序、并行通道/带宽分配；状态量是每个源的 AoI。
- **目标与约束**：最小化所有节点中的最大峰值 AoI，同时满足能量因果、充电时延、传输时间和网络带宽约束。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 推导最大峰值 AoI 与**充电时延**的理论关系/界
- 单充电器：先最小化充电时延的最优调度，再设计传输策略；近似比可达 **1.5**
- 多充电器与带宽约束：近似算法；考虑充传并行
- DOI：10.1109/TNET.2023.3303266；在线发表于 2023，正式卷期为 2024

## 理论性质与复杂度

论文给出最大峰值 AoI 的上下界及单充电器算法分析；单充电器调度达到 1.5 近似，多充电器/带宽版本的保证须按各节定理解释。

## 实验设置与基线

分别评估单充电器、多充电器和带宽受限场景，并比较 AoI 与 latency；不同采样模型分开报告。

## 定量结果

- 仿真：相对基线最大峰值 AoI 可约减半（文中）
- 文称首篇联合充传做峰值 AoI

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 与“纯充电 utility 最大化”目标不同（信息新鲜度）
- 定向模型为主；干涉未作为核心

## 证据定位

- Raw：`raw/canonical/Peak_AoI_Minimization_at_Wireless-Powered_Network_Edge_From_the_Perspective_of_Both_Charging_and_Transmitting.pdf-a84be74f-fb17-427c-9f09-73ff009d6e42/full.md`
- 模型：§III，第 63–164 行；单充电器：§IV，第 165–371 行；多充电器：§V，第 372–417 行；带宽：§VI，第 418–540 行；实验：第 541–659 行。

## 相关页面

- 概念：[[cpt-directional-charging]] · [[cpt-aoi-peak]]
- 方法：[[mtd-peak-aoi-joint-charge-tx]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-mobility-online-service-scheduling]]
