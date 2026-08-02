---
type: synthesis
title: WRSN 充电调度与放置 — 首批文献对照
status: active
epistemic: high
covers:
  - "[[src-wu-charging-on-the-move]]"
  - "[[src-guo-concurrent-ccsp]]"
  - "[[src-ma-concurrent-gain]]"
  - "[[src-ma-tide-dynamic-power]]"
  - "[[src-xu-cooperative-ccs]]"
  - "[[src-chen-peak-aoi-wpt]]"
  - "[[src-dai-wanda-multi-antenna]]"
  - "[[src-wang-hipo-obstacles]]"
  - "[[src-alzenad-uav-bs-qos]]"
gaps:
  - "在线公平+部分充电+功率预算的统一框架仍少见"
  - "干涉模型与可加模型结论如何迁移尚无系统对照"
  - "车载/动态无线充电（DWPT）本批未覆盖"
updated: 2026-07-14
---

# WRSN 充电调度与放置 — 首批对照

> 冲突与差异**只并列，不裁断**（PRD）。

## 问题族对照

| 问题族 | 代表文献 | 决策变量 | 目标 |
|--------|----------|----------|------|
| 移动端轨迹下静态充电器功率调度 | [[src-wu-charging-on-the-move]] | 功率档位 | 充电 utility |
| 并发 on/off 调度（干涉） | [[src-guo-concurrent-ccsp]] | 充电器开关组合时序 | 全充满时间 |
| 干涉感知放置 + 传感器落点 | [[src-ma-concurrent-gain]] | 充电器/传感器位置 | utility |
| 在线朝向控制（干涉） | [[src-ma-tide-dynamic-power]] | 定向朝向 | utility（在线请求） |
| 合作付费服务选站 | [[src-xu-cooperative-ccs]] | 设备→充电器分配 | 综合成本 |
| 充传联合峰值 AoI | [[src-chen-peak-aoi-wpt]] | 充电朝向+传输 | 最大峰值 AoI |
| 多天线放置 | [[src-dai-wanda-multi-antenna]] | 位置+多朝向 | utility |
| 障碍+异构扇环放置 | [[src-wang-hipo-obstacles]] | 位置+朝向 | utility |
| UAV-BS QoS 覆盖（边界） | [[src-alzenad-uav-bs-qos]] | 3D 位置 | 覆盖用户数 |

## 模型假设并列

| 主张/设定 | 文献 |
|-----------|------|
| 多充电器功率**可加** | CM、CCS、HIPO、WANDA（几何）等 |
| 功率**非可加**（波干涉） | CCSP、GAIN、TIDE |
| 接收端移动、轨迹已知 | CM；CCS（设备走向充电器） |
| 接收端固定、充电器动朝向 | TIDE、Peak AoI |
| 放置阶段一次性优化 | GAIN、WANDA、HIPO |
| 经济成本/合作分摊 | CCS |
| 信息新鲜度（AoI）优先于纯能量 | Peak AoI |

## Gaps（供后续 B，非 idea）

1. 干涉模型与可加模型在同一场景下的可迁移边界  
2. 在线请求 + 公平/截止时间 + 部分充电 的联合  
3. EV/路侧动态无线充电调度本批为 0  
4. UAV **充电**（非基站覆盖）本批仅有边界蜂窝文  

## 专题下钻

- [[syn-interference-aware-concurrent-wpt]]：开关调度、空间部署与在线朝向如何分别处理波干涉。
- [[syn-mobility-online-service-scheduling]]：移动轨迹、在线请求、成本与 AoI 目标的适用边界。

## 边界文献

[[src-alzenad-uav-bs-qos]] 非 WPT；是否移出核心库待你确认。
