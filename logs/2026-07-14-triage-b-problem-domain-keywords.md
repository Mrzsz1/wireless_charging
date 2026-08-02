# 首轮候选裁决、B problem 与领域关键词层 — 2026-07-14

## 触发原因

用户将两项人工闸门交由 Agent 决定：自动发现候选的首轮 select/reject，以及 A→B 草案是否正式化；同时要求利用论文关键词扩展领域关键词。

## 1. 自动发现候选裁决

边界：只修改 `raw/inbox/auto-discovered` 的候选状态和 selected 元数据，不下载 PDF，不晋升 canonical，不调用 MinerU，不把候选写成 wiki 事实。

| 运行 | selected | rejected | pending |
|------|----------|----------|---------|
| `search-20260714-204713` | 3, 4, 8, 12 | 5, 13, 15–19 | 其余 9 条 |
| `search-20260714-204721` | — | 1–3 | — |
| `search-20260714-214003` | 1–5, 9 | 6, 10, 13, 15 | 其余 5 条 |
| **合计** | **10** | **14** | **14** |

`search-20260714-204713#5` 与 `search-20260714-214003#3` 为同题记录，保留含 DOI 的后者进入 selected，前者按重复记录 rejected。10 个 selected 候选已各自生成 `metadata.json`，没有 `paper.pdf`。

## 2. B 阶段决定

- 新建：`wiki/problems/prob-joint-deployment-online-interference.md`。
- 证据：CCSP、GAIN、TIDE 与两篇 synthesis。
- `user_confirmed: true` 的依据：用户明确授权 Agent 代为决定本次 A→B 落盘。
- 未新建 idea：双时间尺度算法骨架仍依赖充电器独立开关/旋转、切换成本、接收端移动性与公平定义，当前不应伪装成成熟候选方案。

## 3. 论文关键词三层治理

```text
source.paper_keywords + keyword_source
  → wiki/maps/map-domain-keywords.md
  → schema/vocab-proposals.md（仅匹配字段缺词时）
  → 用户确认后才进入 vocab.yaml
```

- 8/9 source 有明确 Index Terms；共 32 次出现、28 个大小写归一原词。
- 地图归并为 25 个规范导航词，保留作者原词、别名、source 链和核心/边界范围。
- `src-wu-charging-on-the-move` 未检出作者关键词，明确标记 `not_found`，没有摘要推断。
- 本轮没有修改 `schema/vocab.yaml`，也没有新增 vocab proposal。
- 新增只读检查：`tools/domain_keywords.py --check`；新增 2 个测试。

## 4. 更新范围

- Wiki：9 个 source frontmatter、1 个 problem、1 个领域关键词 map、2 个 synthesis 反链、index、map-home、library-status。
- Schema：关键词治理、frontmatter、写作规则、A 编译、ingest/lint 清单、source/raw 模板、PRD 决策日志。
- 文档：`ARCHITECTURE.md`、`HOME.md`、`使用说明.md`、自动发现 README。
- Raw：未改 canonical 正文；只通过既有 triage 工具更新 inbox JSON 与 selected metadata。

## 5. 验收

- 候选状态：14 pending / 10 selected / 14 rejected；selected 元数据夹 10。
- 关键词：`domain_keywords.py --check` 通过。
- Query 契约：10/10 通过。
- 单元测试：24/24 通过。
- Wiki 断链：0。
- A 类贡献句扫描：0 命中。

## 6. Graphify

本轮先用既有图查询确认“领域关键词”是旧图缺口。当前语义图快照仍早于本轮变更，且历史图含多平台 skill 噪声；未使用代码型增量更新冒充 Markdown 语义重建。待可用的文档语义抽取 backend 后做一次受控全量刷新，wiki 正文继续作为权威。

## 后续推进：公开全文与 MinerU

用户随后明确要求继续下载。10 条 selected 中取得 7 篇合法公开 PDF，全部通过文件签名与首页标题核验，并由 MinerU 7/7 转换成功；状态更新为 7 promoted / 3 selected。3 篇受限全文分别为 Spatiotemporal Optimization、Wirelessly Charged Electric Bus 与 Long-term Utility Optimization，未绕过访问控制。
