---
type: map
title: Wiki 索引 index
status: active
updated: 2026-08-12
---

# Wiki 索引

> 先读本页 + [[maps/library-status|库水位]]，再钻取。图查询：`graphify query`。

> 2026-08-12：21 篇论文 source 与 20 个 method 均已升级为可核验研究档案。进入页面后优先读“何时使用 / 何时不使用”和“证据定位”，再迁移算法。

## 导航

- [[maps/map-home|总图]]
- [[maps/library-status|库水位]]
- [[maps/map-online-scheduling|在线调度]]
- [[maps/map-power-allocation|功率分配]]
- [[maps/map-multi-device-wpt|多设备 WPT]]
- [[maps/map-models-and-objectives|模型与目标]]
- [[maps/map-domain-keywords|领域关键词]]
- [[syntheses/syn-wrsn-scheduling-placement|首批综合对照]]
- [[syntheses/syn-interference-aware-concurrent-wpt|干涉感知并发 WPT]]
- [[syntheses/syn-mobility-online-service-scheduling|移动、在线与服务调度]]
- [[syntheses/syn-mobile-uav-directional-scheduling|移动、UAV与三维定向调度]]
- [[syntheses/syn-dynamic-roadway-wpt-infrastructure|道路DWPT基础设施与调度]]

## Sources（21篇论文/预印本）

| 页面 | 一句话 | year | 采集来源 | 状态 |
|------|--------|------|----------|------|
| [[sources/src-wu-charging-on-the-move]] | 移动轨迹 + 可调功率静态充电器 | 2021 | manual | active |
| [[sources/src-guo-concurrent-ccsp]] | 并发充电开关调度 CCSP（干涉） | 2017 | manual | active |
| [[sources/src-ma-concurrent-gain]] | GAIN 干涉增强放置 | 2023 | manual | active |
| [[sources/src-ma-tide-dynamic-power]] | TIDE 在线定向功率分布 | 2024 | manual | active |
| [[sources/src-xu-cooperative-ccs]] | 合作充电服务 CCS | 2021 | manual | active |
| [[sources/src-chen-peak-aoi-wpt]] | 充传联合峰值 AoI | 2024 | manual | active |
| [[sources/src-dai-wanda-multi-antenna]] | 多天线放置 WANDA | 2024 | manual | active |
| [[sources/src-wang-hipo-obstacles]] | 障碍异构放置 HIPO | 2020 | manual | active |
| [[sources/src-alzenad-uav-bs-qos]] | UAV-BS 多 QoS 覆盖（边界） | 2018 | manual | needs_review |
| [[sources/src-gao2024-ra-dmcs-asymmetric-directional]] | 路由非对称定向移动充电 RA-DMCS | 2024 | auto | active |
| [[sources/src-honma2026-infinite-drive-dwpt]] | 信号交叉口动态无线充电部署 | 2026 | auto | active |
| [[sources/src-liu2021-joint-cuav-scheduling-trajectory]] | CUAV 调度与轨迹联合优化 | 2021 | auto | active |
| [[sources/src-binh2025-bilevel-metaheuristic-charging]] | 双层元启发式能量耗尽规避 | 2025 | auto | active |
| [[sources/src-gao2025-felkh-3d-uav]] | 三维定向 UAV 充电 FELKH-3D | 2025 | auto | active |
| [[sources/src-li2024-dwc-beb-integrated-planning]] | BEB 动态充电设施与调度联合规划 | 2024 | auto | active |
| [[sources/src-dai2022-rose-robust-safe-charging]] | 概率EMR安全功率调度 ROSE | 2022 | auto | active |
| [[sources/src-yao2026-ihatrpo-heterogeneous-chargers]] | 异构 AAV/SV 协同充电与 IHATRPO | 2025/2026 | auto | active |
| [[sources/src-tian2025-diccs-clustering]] | 动态非均匀聚类与混合优先级 | 2025 | auto | active |
| [[sources/src-liu2026-dchsa-adtsa-clustered]] | 动态簇头选择与双阈值充电 | 2026 | auto | active |
| [[sources/src-qaisar2026-isac-uav-charging]] | ISAC 状态估计、在线队列与部分充电 | 2026 | auto | active |
| [[sources/src-rahaman2023-obstacle-mcv]] | 有障碍多 MCV 分区与绕行调度 | 2023/2026 | auto | active |

## Core books

| 页面 | 用途 | year | 页数 / 章节 |
|------|------|------|------|
| [[sources/src-book-algorithmic-game-theory]] | 博弈模型、均衡、机制与激励 | 2007 | 775 / 29 |
| [[sources/src-book-approximation-algorithms]] | 近似比、LP/SDP、舍入与原始-对偶 | 2001 | 396 / 30 |

章节 Markdown 位于 `raw/canonical/<book-id>/chapters/`；查询入口见 [[syntheses/syn-core-books-atlas]]。

## Concepts

| 页面 | 一句话 |
|------|--------|
| [[concepts/cpt-wave-interference]] | 建设性/破坏性干涉 |
| [[concepts/cpt-concurrent-charging]] | 多充电器并发 |
| [[concepts/cpt-directional-charging]] | 定向/扇环充电 |
| [[concepts/cpt-charging-utility]] | 有界充电效用 |
| [[concepts/cpt-aoi-peak]] | 峰值信息年龄 |
| [[concepts/cpt-dynamic-wireless-charging]] | 动态无线充电 |
| [[concepts/cpt-probabilistic-emr-safety]] | 概率EMR安全 |

## Methods

| 页面 | 一句话 | method_family |
|------|--------|---------------|
| [[methods/mtd-ccsp-greedy-scheduling]] | CCSP 贪心 | heuristic |
| [[methods/mtd-tide-online-orientation]] | TIDE 在线朝向 | online_algorithm |
| [[methods/mtd-tunable-power-mobile-traj]] | CM 轨迹功率 | heuristic |
| [[methods/mtd-gain-placement-interference]] | GAIN 放置 | heuristic |
| [[methods/mtd-ccs-cooperative-service]] | CCS 合作服务 | game_theory |
| [[methods/mtd-wanda-multi-antenna-placement]] | WANDA | heuristic |
| [[methods/mtd-hipo-placement-obstacles]] | HIPO/PDCS | heuristic |
| [[methods/mtd-peak-aoi-joint-charge-tx]] | 峰值 AoI 联合 | online_algorithm |
| [[methods/mtd-ra-dmcs-asymmetric-mobile]] | RA-DMCS 非对称移动充电 | heuristic |
| [[methods/mtd-dwpt-intersection-placement]] | 交叉口DWPT选址 | ilp_milp |
| [[methods/mtd-uav-joint-scheduling-trajectory-pso]] | CUAV调度轨迹PSO | metaheuristic |
| [[methods/mtd-bilevel-metaheuristic-charging]] | 双层元启发式充电 | metaheuristic |
| [[methods/mtd-felkh-3d-directional-uav]] | FELKH-3D定向UAV | heuristic |
| [[methods/mtd-integrated-dwpt-battery-scheduling]] | DWPT设施电池调度联合规划 | ilp_milp |
| [[methods/mtd-rose-robust-safe-power-scheduling]] | ROSE鲁棒安全功率 | convex_opt |
| [[methods/mtd-ihatrpo-heterogeneous-charging]] | IHATRPO 异构移动充电器协同 | rl |
| [[methods/mtd-diccs-dynamic-clustering]] | DICCS 动态聚类与混合优先级 | heuristic |
| [[methods/mtd-dchsa-adtsa-dec]] | 动态簇头与自适应双阈值 | heuristic |
| [[methods/mtd-isac-uav-priority-partial-charging]] | ISAC-UAV 在线队列与部分充电 | online_algorithm |
| [[methods/mtd-obstacle-aware-multi-mcv]] | 有障碍多 MCV 时空协同 | heuristic |

## Syntheses

| 页面 | 一句话 |
|------|--------|
| [[syntheses/syn-wrsn-scheduling-placement]] | 首批 9 篇问题族与假设并列 |
| [[syntheses/syn-interference-aware-concurrent-wpt]] | 开关、部署与在线朝向三条干涉感知路线 |
| [[syntheses/syn-mobility-online-service-scheduling]] | 移动性、在线请求、成本与 AoI 的适用边界 |
| [[syntheses/syn-mobile-uav-directional-scheduling]] | 移动、UAV与三维定向WPT调度 |
| [[syntheses/syn-dynamic-roadway-wpt-infrastructure]] | 道路动态无线充电基础设施与调度 |
| [[syntheses/syn-adaptive-mobile-charger-coordination]] | 异构、聚类、部分充电与障碍下的移动协同 |
| [[syntheses/syn-core-books-atlas]] | 两本核心书籍的理论章节导航 |

## 可复用系统模型

- [[system-models/sys-interference-aware-concurrent-static]]：静态多充电器相干干涉。
- [[system-models/sys-online-directional-request]]：在线请求与定向朝向控制。
- [[system-models/sys-mobile-uav-routing-scheduling]]：移动/UAV 路径—调度耦合。
- [[system-models/sys-heterogeneous-mobile-charger-coordination]]：异构移动充电器协同。

## 优化目标

- [[objectives/obj-full-charge-completion-time]]：全充满完成时间。
- [[objectives/obj-aggregate-charging-utility]]：总体充电效用。
- [[objectives/obj-energy-and-mobility-cost]]：能效与移动成本。
- [[objectives/obj-multi-objective-survivability]]：多目标生存性。

## 数据与仿真

- [[datasets-sims/data-wrsn-simulation-evidence-protocol]]：WRSN 调度仿真和证据报告的最小协议。

## Problems / Ideas

| 页面 | 一句话 |
|------|--------|
| [[problems/prob-joint-deployment-online-interference]] | 联合慢时标部署与快时标在线干涉调度；当前为问题定义，不含已验证算法 |
| — | 正式 idea 页仍为 0；候选算法骨架保留在审阅日志 |

## 统计

- source 页：**23**（21篇论文/预印本 + 2本核心专著；含1篇边界 needs_review）
- method / concept：**20 / 7**；system-model / objective / dataset-or-sim：**4 / 4 / 1**
- synthesis 数：**7**；10 条真实问答回归用例已建立
- 最新增量5篇均已保留正式DOI、arXiv或预印本版本provenance；版本差异在source页单列。
- 论文关键词：**20/21** paper/preprint source 有作者关键词，见 [[maps/map-domain-keywords]]
- 来源：论文已编译 source 为 **9 manual / 12 auto-discovery**，另有 2 本 manual 核心专著；自动发现 canonical 待编译数为 **0**
- 自动发现状态：**46 pending / 6 selected（全文受限）/ 14 rejected / 12 promoted**
- B 层：**1 problem / 0 idea**；problem 已由用户授权 Agent 正式化
- 上次内容更新：2026-08-12 完成 P2 全量研究档案与 Gold Contract v2 原文证据门禁
