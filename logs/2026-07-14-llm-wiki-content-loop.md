# LLM Wiki 内容闭环优化 — 2026-07-14

## 触发原因

把知识库从“基础设施可运行”推进到“内容可比较、问答可回归、B 阶段可审阅”。

## 输入与证据

- 9 个 `wiki/sources/` source 页。
- 9 份 `raw/canonical/*/full.md`，以及本地 canonical PDF 第一页文本用于核验卷期、venue、DOI。
- 3 次 `raw/inbox/auto-discovered/runs/*/results.json`，共 38 条候选记录。
- 未外搜；候选元数据未作为 wiki 事实。

## 修改

### 元数据

- 9/9 source 补齐并核验 `year`、`venue`、`doi`。
- 同步更新 9 份 raw `full.md` 的 frontmatter；未改 raw 正文。
- 对在线年份与正式卷期不同的条目，`year` 采用正式卷期年份，并在 source 正文注明 DOI 在线年份。

### 内容密度

- 新建 `wiki/syntheses/syn-interference-aware-concurrent-wpt.md`。
- 新建 `wiki/syntheses/syn-mobility-online-service-scheduling.md`。
- 更新首批 synthesis、8 个核心 source 反链与 4 个已有 map；未新建 map。

### 页面治理

- `schema/page-types.md` 增加 A 类页面准入规则：concept/method 原则上需跨 2 个 source 复用，synthesis 至少覆盖 2 个 source 并给出对照与 gap。
- `schema/lint-checklist.md` 增加过度拆页与低复用检查。
- `schema/frontmatter.md` 增加可选 `doi` 字段。
- `ARCHITECTURE.md` 补入 Query 回归层、页面准入、最新水位与已知风险。

### 候选初筛

- 新建 `logs/2026-07-14-auto-candidate-triage-recommendations.md`。
- 识别 1 组题名完全重复记录和 1 组疑似版本重叠记录。
- 推荐 10 个文献身份进入第一优先精读池，但未修改官方 triage 状态。

### 回归评测

- 新建 `evals/gold_questions.json`：5 solve、3 novelty、2 relationship。
- 新建 `tools/wiki_eval.py`：校验 10 条类型配额、wikilink 目标、库水位要求；可选检查保存答案。
- 新建 `tests/test_wiki_eval.py`。

### A→B 试运行

- 新建 `logs/2026-07-14-ab-pilot-review-draft.md`，形成“联合部署与在线干涉调度”problem/idea 审阅草案。
- 未写入 `wiki/problems/` 或 `wiki/ideas/`；等待用户对具体草案确认。

## 水位变化

- source：9（不变）。
- synthesis：1 → 3。
- source 元数据：3/9 已有年份 → 9/9 year、venue、DOI 完整。
- 回归用例：0 → 10。
- 自动候选：仍为 38 pending / 0 selected；机器建议不改变人工状态。

## 待用户确认

1. 从候选初筛报告中确认要 select/reject 的具体运行与序号。
2. 是否把 A→B 草案正式写入 `wiki/problems/prob-joint-deployment-online-interference.md`。
3. UAV-BS 非 WPT 边界 source 是否继续保留在核心索引。

## Graphify

本轮先用现有图做导航。当前 174-node 图含旧工具 skill 噪声；在没有可用文档语义抽取 backend 时，不用结构型 `graphify update .` 覆盖它。正文与链接已更新，语义图全量重建状态将在验证阶段记录。
