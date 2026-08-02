# 自动发现候选第一轮初筛建议 — 2026-07-14

> **已执行并推进全文**：用户于 2026-07-14 授权 Agent 代为裁决并明确要求下载。当前状态为 **7 promoted / 3 selected / 14 rejected / 14 pending**；7 篇公开 PDF 已完成 MinerU，3 篇未发现合法公开全文。本文件下方保留执行前建议，作为决策依据审计。

## 边界

- 输入：3 次发现运行中的 **38 条记录**，只使用各运行保存的题名、摘要、年份、DOI、来源与匹配查询。
- 本报告是机器辅助建议，**未修改**任何候选的 `triage_status`，也未写入 `selected_by_user: true`。
- 正式 `selected` / `rejected` 仍需用户确认；确认后再用 `tools/paper_triage.py` 落盘。
- 本轮未外搜，也未把候选当作 wiki 事实。

## 去重发现

| 记录 | 判断 | 建议 |
|------|------|------|
| `search-20260714-204713#5` 与 `search-20260714-214003#3` | 题名完全相同；后者含 DOI `10.1109/JIOT.2021.3132015`，年份字段为 2021，前者 arXiv 记录显示 2023 | 作为同一文献身份合并审查，优先保留含 DOI 的书目记录，并核对 arXiv 版本关系 |
| `search-20260714-204713#1` 与 `#6` | 题名不同但摘要、作者/协议描述高度重叠 | 暂不自动合并；人工核对是否为同一工作的期刊/会议版本 |

## 第一优先：建议进入人工精读池

共 **10 个文献身份、11 条记录**。建议先从其中选择 6–10 篇，使 canonical source 从 9 篇增长到 15–20 篇。

| 候选 | 年份 | 建议理由 |
|------|------|----------|
| `search-20260714-204713#3` — *Minimizing the energy depletion in WRSNs using bi-level metaheuristic charging schemes* | 2025 | 直接覆盖“部分充电 + 路径 + 充电时间”，对应当前 synthesis 明确 gap |
| `search-20260714-204713#4` — *Directional WPT Charging for Routing-Asymmetric WRSNs with a Mobile Charger* | 2024 | 定向移动充电、非对称路径与时间分配，能补当前固定充电器偏重 |
| `search-20260714-204713#5` / `search-20260714-214003#3` — *Joint Scheduling and Trajectory Optimization of Charging UAV in WRSNs* | 2021/2023 | 直接联合 UAV 充电调度与轨迹；两条先合并身份再选 |
| `search-20260714-204713#8` — *Optimal 3D Directional WPT Charging via UAV for 3D WRSNs* | 2025 | 覆盖 3D、定向、UAV 与充电巡回，能替代当前非 WPT 的 UAV-BS 边界页作为核心证据 |
| `search-20260714-204713#12` — *Infinite Drive: Optimal Urban Location of Dynamic Wireless Charging at Signalized Intersections* | 2026 | 补齐当前 EV/DWPT 为 0 的缺口；重点是基础设施位置与运行约束 |
| `search-20260714-214003#1` — *Planning dynamic wireless charging infrastructure for battery electric bus systems with the joint optimization of charging scheduling* | 2024 | 同时覆盖 DWPT 基础设施与公交充电调度，且有 DOI |
| `search-20260714-214003#2` — *Spatiotemporal optimization for charging scheduling in WRSNs* | 2023 | 与本库核心“时空充电调度”高度一致；需先补齐稳定书目和全文 |
| `search-20260714-214003#4` — *On Charging Scheduling Optimization for a Wirelessly Charged Electric Bus System* | 2017 | EV 无线充电运营调度与电价/时隙，提供不同于 WRSN 的成熟对照 |
| `search-20260714-214003#5` — *Wireless Charging Scheduling for Long-term Utility ...* | 待核 | 长期平均 utility 与调度直接相关；标题被截断，选前必须补齐书目 |
| `search-20260714-214003#9` — *ROSE: Robustly safe charging for wireless power transfer* | 2020 | 补充安全辐射约束下的多充电器功率调度，当前库仅在背景中提及 |

## 第二优先：需要人工看摘要/全文再决定

| 候选 | 需要确认的边界 |
|------|----------------|
| `search-20260714-204713#1`、`#6` | ISAC、多移动充电车、部分充电很相关，但两条可能是同一工作的不同版本 |
| `search-20260714-204713#2` | 综述 + pivot cluster head 方案；需判断是作为领域综述还是算法源 |
| `search-20260714-204713#7` | 重点是为充电 UAV 部署 PAD，属于“给充电器充电”的二级基础设施问题 |
| `search-20260714-204713#9` | 无线充电服务于无人机任务，主体不是 WRSN 节点充电 |
| `search-20260714-204713#10` | RF 能量收集通信的传输调度，属可迁移相邻域而非主动 WPT 调度 |
| `search-20260714-204713#11` | UAV/UGV 充电与持续监视相关，但目标是区域访问 age |
| `search-20260714-204713#14` | DWPT 基础设施规划相关，但未从元数据确认是否包含运行调度 |
| `search-20260714-204713#20` | WPT 资源分配相关，偏 IRS/物理层；可用于公平功率分配对照 |
| `search-20260714-214003#7` | 多天线移动充电调度相关，但当前仅有截断摘要/题名信息 |
| `search-20260714-214003#8` | 分布式移动无线充电车辆调度相关，需确认是 WPT 还是普通充电桩运营 |
| `search-20260714-214003#11` | WRSN 移动充电器路径与电池约束相关，建议补齐全文后评估 |
| `search-20260714-214003#12` | on-demand WRSN 移动充电调度相关，建议补齐全文后评估 |
| `search-20260714-214003#14` | 标题直接相关，但仅 Tavily 摘要片段，书目可靠性不足 |

## 低优先：建议本轮拒绝或移出核心池

| 候选 | 原因 |
|------|------|
| `search-20260714-204713#13` | SWIPT 调制与接收灵敏度，偏物理层 |
| `search-20260714-204713#15` | 认知无线电 RF harvesting 时间共享，非主动无线充电网络 |
| `search-20260714-204713#16`、`search-20260714-204721#1` | 电气化公路负荷建模/频谱分析，不以充电调度为核心 |
| `search-20260714-204713#17` | RIS-UAV WPT 能耗最小化，偏通信/轨迹物理层 |
| `search-20260714-204713#18` | Massive MIMO-NOMA 资源分配，偏通信物理层 |
| `search-20260714-204713#19` | RIS-UAV 通信与 WPT，偏吞吐/物理层联合优化 |
| `search-20260714-204721#2` | 电气化道路收费与交通分配，离无线充电调度主轴较远 |
| `search-20260714-204721#3` | 近场 MIMO WPT 硬件实现与失配容忍，非调度 |
| `search-20260714-214003#6` | 插电/无线混合 EV 路由，需另有明确 DWPT 调度证据才进入核心池 |
| `search-20260714-214003#10` | 充电站选址标题片段，未确认无线功率传输与调度核心性 |
| `search-20260714-214003#13` | IRS 通信教程，不是无线充电调度源 |
| `search-20260714-214003#15` | ISAC 综述，不是无线充电调度源 |

## 建议的确认动作

1. 先确认“第一优先”中的具体条目，不建议一次全选。
2. 对两个重复组先合并身份，再执行 triage，避免一篇论文占两个 canonical 名额。
3. 选中后先下载/投放 PDF → MinerU → canonical → A 编译；候选元数据本身不进入 wiki。
4. 用户已授权执行本轮建议；当前官方候选水位为 **14 pending / 3 selected / 14 rejected / 7 promoted**。promoted 项已完成 MinerU，仍须 A 编译。
