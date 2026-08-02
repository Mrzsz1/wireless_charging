---
type: map
title: 领域关键词地图 Domain Keywords
status: active
source_with_keywords_count: 15
paper_keyword_occurrence_count: 66
normalized_keyword_count: 58
updated: 2026-08-01
---

# 领域关键词地图

> 本地图由 canonical 论文的作者 `Keywords` / `Index Terms` 扩展而来。它用于发现、浏览和检索，不是 `vocab.yaml` 的替代品，也不代表词频越高越重要。治理规则见 [[../../schema/domain-keywords|论文关键词与领域关键词治理]]。

## 当前覆盖

- 16 篇 source 中 **15 篇**有明确作者 Keywords / Index Terms，共 **66 次**关键词出现。
- [[../sources/src-wu-charging-on-the-move]] 的正文未发现作者关键词，保留 `paper_keywords: []` / `keyword_source: not_found`。
- 重复支撑的核心信号：`wave interference`、`wireless power transfer`、`charger placement` 各 2 篇；方向性充电词族 3 篇；WRSN/WSN 词族 2 篇。

## 能量传输与网络

| 规范关键词 | 作者原词 / 别名 | 文献数 | 证据 |
|------------|-----------------|--------|------|
| 无线充电（Wireless charging） | `Wireless charging` | 1 | [[../sources/src-guo-concurrent-ccsp]] |
| 无线功率传输（Wireless power transfer） | `wireless power transfer` | 2 | [[../sources/src-ma-concurrent-gain]] · [[../sources/src-ma-tide-dynamic-power]] |
| 无线供能网络（Wireless-powered network） | `wireless-powered network` | 1 | [[../sources/src-chen-peak-aoi-wpt]] |
| 无线可充电传感器网络（WRSN/WSN） | `WRSN`; `wireless sensor networks (WSNs)` | 2 | [[../sources/src-xu-cooperative-ccs]] · [[../sources/src-guo-concurrent-ccsp]] |
| 定向无线充电（Directional charging） | `directional charging`; `directional wireless charging network` | 3 | [[../sources/src-ma-tide-dynamic-power]] · [[../sources/src-chen-peak-aoi-wpt]] · [[../sources/src-dai-wanda-multi-antenna]] |
| 并发充电（Concurrent charging） | `concurrent charging` | 1 | [[../sources/src-ma-concurrent-gain]] |
| 合作充电服务（Cooperative charging service） | `cooperative charging service` | 1 | [[../sources/src-xu-cooperative-ccs]] |

## 决策变量与系统能力

| 规范关键词 | 作者原词 / 别名 | 文献数 | 证据 |
|------------|-----------------|--------|------|
| 调度（Scheduling） | `scheduling` | 1 | [[../sources/src-guo-concurrent-ccsp]] |
| 充电器放置（Charger placement） | `charger placement`; `Charger placement` | 2 | [[../sources/src-ma-concurrent-gain]] · [[../sources/src-wang-hipo-obstacles]] |
| 传感器部署（Sensor deployment） | `sensor deployment` | 1 | [[../sources/src-ma-concurrent-gain]] |
| 功率分布控制（Power distribution control） | `power distribution controlling` | 1 | [[../sources/src-ma-tide-dynamic-power]] |
| 多天线（Multiple antennas） | `multiple antennas` | 1 | [[../sources/src-dai-wanda-multi-antenna]] |

## 物理效应与环境约束

| 规范关键词 | 作者原词 / 别名 | 文献数 | 证据 |
|------------|-----------------|--------|------|
| 波干涉（Wave interference） | `wave interference` | 2 | [[../sources/src-ma-concurrent-gain]] · [[../sources/src-ma-tide-dynamic-power]] |
| 无线电干涉（Radio interference） | `radio interference` | 1 | [[../sources/src-guo-concurrent-ccsp]] |
| 异构性（Heterogeneity） | `heterogeneity` | 1 | [[../sources/src-wang-hipo-obstacles]] |
| 障碍物（Obstacles） | `obstacles` | 1 | [[../sources/src-wang-hipo-obstacles]] |

## 目标、指标与方法

| 规范关键词 | 作者原词 / 别名 | 文献数 | 证据 |
|------------|-----------------|--------|------|
| 信息年龄（Age of information, AoI） | `Age of information (AoI)` | 1 | [[../sources/src-chen-peak-aoi-wpt]] |
| 最大峰值 AoI（Maximum peak AoI） | `maximum peak AoI` | 1 | [[../sources/src-chen-peak-aoi-wpt]] |
| 组合优化（Combinatorial optimization） | `Combinatorial optimization` | 1 | [[../sources/src-dai-wanda-multi-antenna]] |
| 子模函数（Submodular function） | `submodular function` | 1 | [[../sources/src-xu-cooperative-ccs]] |
| 联盟形成博弈（Coalition formation game） | `coalition formation game` | 1 | [[../sources/src-xu-cooperative-ccs]] |
| 纳什均衡（Nash Equilibrium） | `Nash Equilibrium` | 1 | [[../sources/src-xu-cooperative-ccs]] |

## 邻域与边界词

下列词来自 [[../sources/src-alzenad-uav-bs-qos]]。该文是 UAV 基站覆盖的边界文献，不是 WPT 调度证据，因此只作为相邻域导航。

| 规范关键词 | 作者原词 / 别名 | 文献数 | 范围 |
|------------|-----------------|--------|------|
| 无人机（UAV / drone） | `Unmanned aerial vehicles`; `drone` | 1 | 边界 |
| 覆盖（Coverage） | `coverage` | 1 | 边界 |
| 优化（Optimization） | `optimization` | 1 | 边界 |

## 如何让关键词推动知识库增长

1. 新 source 先保存作者原词，再补充到本地图；不根据标题自动生成“作者关键词”。
2. 同一词族达到 2 篇及以上时，优先检查是否需要 concept、synthesis 对照或问答测试；仍需满足 A 类页面准入规则。
3. 单篇低频词只有在正式 problem 或真实问答需要时才提升导航优先级。
4. 若现有匹配字段无法表达真实检索需求，再写 `schema/vocab-proposals.md`；本轮没有直接新增受控 id。

## 自动发现批次新增关键词（2026-08-01）

| 规范关键词 | 作者原词 / 别名 | 证据 |
|------------|-----------------|------|
| 充电调度 | `Charging scheduling`; `scheduling and trajectory optimization`; `charging schedule` | [[../sources/src-gao2024-ra-dmcs-asymmetric-directional]] · [[../sources/src-liu2021-joint-cuav-scheduling-trajectory]] · [[../sources/src-gao2025-felkh-3d-uav]] |
| 无线功率传输 | `wireless power transfer`; `Dynamic wireless power transfer (DWPT)`; `directional WPT` | [[../sources/src-gao2024-ra-dmcs-asymmetric-directional]] · [[../sources/src-honma2026-infinite-drive-dwpt]] · [[../sources/src-gao2025-felkh-3d-uav]] · [[../sources/src-dai2022-rose-robust-safe-charging]] |
| 路由非对称 | `asymmetric path planning` | [[../sources/src-gao2024-ra-dmcs-asymmetric-directional]] |
| 定向移动充电器 | `directional mobile chargers` | [[../sources/src-gao2024-ra-dmcs-asymmetric-directional]] |
| 无线可充电传感器网络 | `wireless rechargeable sensor networks`; `Wireless rechargeable sensor network`; `3D Wireless rechargeable sensor networks` | [[../sources/src-gao2024-ra-dmcs-asymmetric-directional]] · [[../sources/src-liu2021-joint-cuav-scheduling-trajectory]] · [[../sources/src-binh2025-bilevel-metaheuristic-charging]] · [[../sources/src-gao2025-felkh-3d-uav]] |
| 动态无线充电 | `Dynamic wireless power transfer (DWPT)`; `Dynamic wireless charging` | [[../sources/src-honma2026-infinite-drive-dwpt]] · [[../sources/src-li2024-dwc-beb-integrated-planning]] |
| 充电基础设施选址 | `Charging infrastructure location`; `Infrastructure planning` | [[../sources/src-honma2026-infinite-drive-dwpt]] · [[../sources/src-li2024-dwc-beb-integrated-planning]] |
| 信号交叉口 | `Signalized intersections` | [[../sources/src-honma2026-infinite-drive-dwpt]] |
| 混合整数规划 | `Mixed-integer programming` | [[../sources/src-honma2026-infinite-drive-dwpt]] |
| 电池容量 | `Battery sizing`; `Battery electric bus`; `Battery manufacturing emissions` | [[../sources/src-honma2026-infinite-drive-dwpt]] · [[../sources/src-li2024-dwc-beb-integrated-planning]] |
| 综合优化 | `Integrated optimization` | [[../sources/src-li2024-dwc-beb-integrated-planning]] |
| 无人机充电 | `unmanned aerial vehicle`; `UAV charger` | [[../sources/src-liu2021-joint-cuav-scheduling-trajectory]] · [[../sources/src-gao2025-felkh-3d-uav]] |
| 粒子群优化 | `particle swarm optimization` | [[../sources/src-liu2021-joint-cuav-scheduling-trajectory]] |
| 能量耗尽 | `energy depletion` | [[../sources/src-binh2025-bilevel-metaheuristic-charging]] |
| 双层优化 | `bi-level optimization` | [[../sources/src-binh2025-bilevel-metaheuristic-charging]] |
| 进化搜索 | `evolutionary strategy`; `multi-start local search`; `multitasking` | [[../sources/src-binh2025-bilevel-metaheuristic-charging]] |
| 鲁棒安全充电 | `Robustly safe charging` | [[../sources/src-dai2022-rose-robust-safe-charging]] |
| 近似算法 | `approximation algorithm` | [[../sources/src-dai2022-rose-robust-safe-charging]] |
| 分布式算法 | `distribution algorithm` | [[../sources/src-dai2022-rose-robust-safe-charging]] |

## 相关入口

- 问题：[[../problems/prob-joint-deployment-online-interference]]
- 综合：[[../syntheses/syn-interference-aware-concurrent-wpt]] · [[../syntheses/syn-mobility-online-service-scheduling]]
- 主题地图：[[map-online-scheduling]] · [[map-power-allocation]] · [[map-multi-device-wpt]] · [[map-models-and-objectives]]

## 2026-08-01 最新文献关键词

| source | 作者关键词 / Index Terms |
|---|---|
| [[../sources/src-yao2026-ihatrpo-heterogeneous-chargers]] | Wireless rechargeable sensor network; collaborative charging optimization; heterogeneous mobile chargers; trust region policy optimization |
| [[../sources/src-tian2025-diccs-clustering]] | Dynamic uneven clustering; Node mortality; Path planning; Wireless rechargeable sensor network |
| [[../sources/src-liu2026-dchsa-adtsa-clustered]] | Clustered wireless rechargeable sensor networks; cluster head rotation; adaptive dual-threshold; charging scheduling strategy; particle swarm optimization |
| [[../sources/src-qaisar2026-isac-uav-charging]] | Wireless rechargeable sensor networks; UAV charging; wireless power transfer; integrated sensing and communication; on-demand scheduling; partial charging |
| [[../sources/src-rahaman2023-obstacle-mcv]] | Wireless rechargeable sensor networks; charging scheduling; Mobile charging vehicles; Obstacles; Joint charging preference |
