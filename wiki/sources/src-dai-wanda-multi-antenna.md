---
type: source
title: "Placing Wireless Chargers With Multiple Antennas"
status: active
epistemic: high
year: 2024
venue: "IEEE Transactions on Mobile Computing"
doi: "10.1109/TMC.2023.3338563"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
authors: ["Haipeng Dai", "Yikang Zhang", "Weijun Wang", "Rong Gu", "Yuben Qu", "Chi Lin", "Lijie Xu", "Jiaqi Zheng", "Wanchun Dou", "Guihai Chen"]
paper_keywords: ["Combinatorial optimization", "directional wireless charging network", "multiple antennas"]
keyword_source: index_terms
scenario: [sensor_rf_energy]
entities: [transmitter, receiver]
constraints: [mutual_coupling]
objectives: [max_throughput]
method_family: heuristic
problem_class: charger_placement
pdf_path: "raw/canonical/Placing_Wireless_Chargers_With_Multiple_Antennas.pdf-bafca2d9-e008-4ed3-9747-5a6b9783c669/Placing_Wireless_Chargers_With_Multiple_Antennas.pdf"
raw_md: "raw/canonical/Placing_Wireless_Chargers_With_Multiple_Antennas.pdf-bafca2d9-e008-4ed3-9747-5a6b9783c669/full.md"
why_relevant: "WANDA：多定向天线充电器部署（位置+多朝向）"
ingest_status: ingested
updated: 2026-08-12
---

# WANDA：多天线无线充电器放置

## TL;DR

充电器配备**多根定向天线**时，如何确定充电器位置与各天线朝向，最大化整体充电 utility。

## 何时使用 / 何时不使用

- **使用**：多天线定向充电器的位置与每根天线朝向需要联合部署，目标是最大化有界充电效用。
- **不使用**：请求在线到达、发射器波相位产生非线性干涉，或需要随时间切换动作时。

## 系统模型与假设

- 多天线定向充电器：可同时向多方向辐射
- 子问题：
  - **WANDA-ROF**：天线相对朝向固定
  - **WANDA-ROU**：相对朝向可调
- 充电器-传感器需位置与朝向“互匹配”才有非零功率
- 非线性功率 + 连续位置 → 无限搜索空间

## 变量、目标与约束

- **变量/状态**：充电器位置集合、每根天线朝向；ROF 固定相对朝向，ROU 允许相对朝向变化。连续区域经离散化变为有限候选。
- **目标与约束**：在充电器/天线预算和定向覆盖模型下最大化总 charging utility；设备饱和使目标呈次模结构。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 分段常数近似功率；区域离散化有限候选
- 证明单调子模；贪心近似
  - ROF：约 $1/2-\epsilon$
  - ROU：约 $1/6-\epsilon$
- 多项式时间算法
- DOI：10.1109/TMC.2023.3338563；在线发表于 2023，正式卷期为 2024

## 理论性质与复杂度

区域离散与最大覆盖集抽取后使用贪心；ROF 约为 (1/2−ε)，ROU 约为 (1/6−ε)。保证依赖论文的几何与可加效用假设。

## 实验设置与基线

仿真比较位置/朝向基线，并有多天线测试床；重部署、障碍和成本只在讨论中扩展。

## 定量结果

- 仿真+实验：相对对比至少约 **16%（ROF）/ 12%（ROU）**

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 放置问题为主，非在线请求调度
- 与波干涉 concurrent 模型不同（多天线几何覆盖为主）

## 证据定位

- Raw：`raw/canonical/Placing_Wireless_Chargers_With_Multiple_Antennas.pdf-bafca2d9-e008-4ed3-9747-5a6b9783c669/full.md`
- 模型：§III，第 68–129 行；ROF：§IV，第 130–297 行；ROU：§V，第 298–517 行；仿真/实测：第 518–672 行。

## 相关页面

- 概念：[[cpt-directional-charging]]
- 方法：[[mtd-wanda-multi-antenna-placement]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]]
