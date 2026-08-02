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
updated: 2026-07-14
---

# WANDA：多天线无线充电器放置

## 一句话问题

充电器配备**多根定向天线**时，如何确定充电器位置与各天线朝向，最大化整体充电 utility。

## 系统设定与假设

- 多天线定向充电器：可同时向多方向辐射
- 子问题：
  - **WANDA-ROF**：天线相对朝向固定
  - **WANDA-ROU**：相对朝向可调
- 充电器-传感器需位置与朝向“互匹配”才有非零功率
- 非线性功率 + 连续位置 → 无限搜索空间

## 方法要点

- 分段常数近似功率；区域离散化有限候选
- 证明单调子模；贪心近似
  - ROF：约 $1/2-\epsilon$
  - ROU：约 $1/6-\epsilon$
- 多项式时间算法
- DOI：10.1109/TMC.2023.3338563；在线发表于 2023，正式卷期为 2024

## 主要结果

- 仿真+实验：相对对比至少约 **16%（ROF）/ 12%（ROU）**

## 局限

- 放置问题为主，非在线请求调度
- 与波干涉 concurrent 模型不同（多天线几何覆盖为主）

## 链接

- 概念：[[cpt-directional-charging]]
- 方法：[[mtd-wanda-multi-antenna-placement]]
- 综合：[[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]]

