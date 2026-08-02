# 论文自动发现候选报告

> **边界声明：** 本报告只是外部检索产生的 triage 候选，不是 `raw/canonical`，
> 未经人工确认与 A 编译不得作为 wiki 硬事实，也不代表完整的全球查新。

- 抓取时间（UTC）：`2026-07-14T12:47:21+00:00`
- 来源：`arxiv`
- 原始命中：`25`；去重后：`3`
- 缓存命中：`5`
- 排序：标题/摘要词项命中 + 轻量时间加分；不是语义相关性或质量判定

## 检索主题

- wireless power transfer scheduling
- wireless rechargeable sensor networks
- mobile charger scheduling
- dynamic wireless charging
- RF energy harvesting scheduling

## 候选列表

### 1. Frequency-Domain Characterization of Load Demand from Electrified Highways

- 作者：Ashutossh Gupta, Vassilis Kekatos, Ruoyu Yang, Dionysios Aliprantis, Steve Pekarek
- 日期：2025-09-02
- 来源：arxiv
- 筛选状态：`rejected`；人工选择：`false`
- DOI：—；arXiv：2509.02426
- 开放获取：True；许可：未提供
- 相关度分数：4.00（摘要命中：dynamic；近年文献）
- 命中主题：dynamic wireless charging
- 页面：https://arxiv.org/abs/2509.02426v3
- PDF：https://arxiv.org/pdf/2509.02426v3
- 本地 PDF：未下载

Electrified roadways (ER) equipped with dynamic wireless power transfer (DWPT) capabilities can patently extend the driving range and reduce the battery size of electric vehicles (EVs). However, due to the spatial arrangement of the transmitter coils in the ER, the DWPT load exhibits frequency content that could excite power system frequency dynamics. In this context, this work aims to study the spectrum of DWPT loads under different traffic conditions. Under simplifying assumptions, we develop statistical models to identify the location and relative magnitude of DWPT load harmonics. Our analysis reveals that the fundamental frequency depends on ER coil spacing and average EV speed. In the worst-case yet unlikely scenario that EVs move in a synchronized fashion, the amplitude of harmonics scales with the EV count. On the contrary, when EVs move freely, harmonics scale with the square root of the EV count. Platoon formations can accentuate harmonics. The spectral content around harmonics decreases in magnitude and increases in bandwidth with the harmonic index. The load of a single EV moving at a time-varying speed can be modeled as a frequency-modulated (FM) signal. Despite the simplifying assumptions, the derived models offer valuable insights for ER planners and grid operators. Dynamic simulations of a modified WECC model with DWPT loads synthesized from realistic EV trajectories and ER specifications corroborate some of these insights.

### 2. A Fundamental Analysis of the Impact on Traffic Assignment by Toll System of Electric Road System

- 作者：Wataru Nakanishi, Noriko Kaneko
- 日期：2024-02-11
- 来源：arxiv
- 筛选状态：`rejected`；人工选择：`false`
- DOI：—；arXiv：2402.07144
- 开放获取：True；许可：未提供
- 相关度分数：3.75（摘要命中：dynamic；近年文献）
- 命中主题：dynamic wireless charging
- 页面：https://arxiv.org/abs/2402.07144v1
- PDF：https://arxiv.org/pdf/2402.07144v1
- 本地 PDF：未下载

Electric road system (ERS) is expected to make electric vehicles (EVs) more popular as EVs with Dynamic Wireless Power Transfer (DWPT) system can be charged while driving on ERS. Although some studies dealt with ERS implementation, its toll system has not been explored yet. This paper aims at a fundamental analysis on impact of ERS toll system on a traffic assignment. We conduct assignments on a simple network where two vehicle types (EVs with DWPT and others) are co-existing. The results under two toll systems showed some undesirable situations, such as total travel time was not minimised, total charged volume was not optimised, and ERS was not utilised. The occurrence of them depended on the ratio of EVs, battery level, value of electricity, and toll price. The difficulty to control such situations by toll price was discussed as the battery level and value of electricity may vary over time.

### 3. Implementation of a Misalignment-Tolerant MIMO Near Field Wireless Power Transfer System

- 作者：Taroh Hijikata, Allan Jr Mesa, Charleston Dale Ambatali
- 日期：2026-06-03
- 来源：arxiv
- 筛选状态：`rejected`；人工选择：`false`
- DOI：—；arXiv：2606.04565
- 开放获取：True；许可：未提供
- 相关度分数：3.50（近年文献）
- 命中主题：wireless power transfer scheduling
- 页面：https://arxiv.org/abs/2606.04565v1
- PDF：https://arxiv.org/pdf/2606.04565v1
- 本地 PDF：未下载

The efficiency of reactive near-field wireless power transfer (WPT) systems degrades rapidly with increasing separation distance and is highly sensitive to misalignment between transmitting and receiving coils. These limitations restrict the mobility of powered devices and confine many near-field WPT applications to static scenarios. To address these challenges, a multiple-input multiple-output (MIMO) WPT configuration is investigated due to its capability to shape the magnetic field distribution between the transmitter and receiver. Maximum power transfer efficiency can be achieved by appropriately setting the amplitude and phase of each transmitting coil; however, determining these optimal settings requires accurate knowledge of the system's S-parameters. This paper presents the use of the Nelder-Mead iterative optimization algorithm to estimate the input amplitude and phase settings that maximize transfer efficiency in a near-field WPT system. The implementation comprises a four-element transmitter and a two-element receiver. Based on measured S-parameters, the proposed approach significantly improves WPT efficiency under both aligned and misaligned conditions.

## 下一步

1. 用 `tools/paper-triage.ps1 <results.json> --select <序号>` 标记真正相关的候选；
2. 从 `raw/inbox/auto-discovered/papers/` 选择项晋升 `raw/canonical/`；
3. 用 MinerU 生成 Markdown，并保留 provenance；
4. 按 `schema/agent-a-compile.md` 执行 A 编译，再更新 Graphify。
