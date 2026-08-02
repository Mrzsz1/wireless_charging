# LLM Wiki 来源追踪与结构优化

## 触发原因

用户希望把当前项目优化为长期维护的 LLM Wiki，并明确区分自动发现文献与手动投放文献。此前 `raw/inbox/search-*` 与 `raw/canonical` 只表达处理位置，文献晋升后会丢失采集来源；候选也缺少逐条人工筛选状态。

## 架构决策

采用两个正交维度：

- 采集来源：`manual_upload` / `auto_discovery`；
- 生命周期：`triage_status`（pending/selected/rejected/promoted）与 `ingest_status`（pending_convert/pending_ingest/ingested/convert_failed）。

目录分流：

```text
raw/inbox/
  auto-discovered/
    runs/search-*/
    papers/<selected-candidate>/
  manual-drop/
raw/canonical/
wiki/
```

## 实现

- `tools/paper_search.py`
  - 默认报告改到 `auto-discovered/runs`，缓存独立放在 `auto-discovered/.paper-search-cache`；
  - 每条候选写入 provenance、`triage_status: pending` 与运行路径；
  - 报告展示人工筛选状态。
- `tools/paper_triage.py` / `tools/paper-triage.ps1`
  - 支持 `--select`、`--reject`、`--pending`、范围序号和人工备注；
  - selected 项建立稳定 `metadata.json`，开放 PDF 下载仍需显式开启；
  - 明确停止在 inbox。
- `tools/mineru_to_md.py`
  - 从 selected sidecar 或输入路径推断来源；
  - 在 canonical `full.md` 写入 provenance 与 `pending_ingest`。

## 历史数据迁移

- `search-20260714-204713`：20 pending；
- `search-20260714-204721`：3 pending；
- `search-20260714-214003`：15 pending；
- 合计 38 条，均已补齐 `auto_discovery`、provider、discovery run、acquired time；
- 首批 9 篇 canonical/raw 与 9 个 source 标为 `manual_upload`、`promoted`；raw 为 `ingested`。

## Wiki 结构优化

- `wiki/index.md` 增加 source 采集来源列，并显式展示尚未实例化的 system-model/objective/dataset 槽位；
- `wiki/maps/library-status.md` 区分已编译水位、manual/auto canonical 与 inbox pending/selected；
- 不拆成两套 wiki：自动与手动文献在 canonical 后共享 source/method/synthesis 编译层，只通过 provenance 过滤。

## 验证

- Python 单元测试：19/19；
- `paper_search.py`、`paper_triage.py`、`mineru_to_md.py` 编译通过；
- 9/9 source 含来源字段；
- 9/9 raw 主稿含来源字段与 `ingested`；
- 3 次 discovery run、38/38 candidates 均为合法 provenance，当前 38 pending；
- 未创建 problem/idea，未新建 map，未改 raw 正文（只新增允许的 frontmatter）。
- Graphify 本地 `update` 只重建代码图并膨胀到 472 节点，已使用其自动备份恢复原 174/248 语义图并重生成 HTML；本次 Markdown 语义变更仍待有 LLM backend 时全量重建。

## 待用户动作

- 在任一候选报告上运行 `paper-triage.ps1 --select ...`，开始把真正相关论文送入已选队列；
- 多篇 source 的年份仍需从原文核验。
