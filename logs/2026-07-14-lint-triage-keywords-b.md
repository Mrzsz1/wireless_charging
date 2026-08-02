# Lint Report — 2026-07-14（候选 / 关键词 / B problem）

## Summary

🟢 正文、状态、关键词与链接契约通过；🟡 Graphify 语义图仍是旧快照，未包含本轮新增 map/problem。

## 1. Schema 完整性

- 9/9 source 均有 `paper_keywords` 与 `keyword_source`。
- 8 篇 `index_terms` 均有非空关键词；1 篇 `not_found` 使用空列表。
- 正式 problem 有 `inspired_by`、`user_confirmed: true`、库水位边界与非主张。
- 未修改 `vocab.yaml`；关键词自由元数据未进入受控匹配字段。

## 2. 状态与水位

- 3 个 manifest 合计 38：14 pending / 10 selected / 14 rejected。
- `selected_by_user: true` 与 selected 状态均为 10；selected 元数据夹为 10。
- source 9、synthesis 3、problem 1、idea 0、map 7；index 与 library-status 已同步。

## 3. 覆盖缺口

- 领域关键词覆盖 8/9 source、32 次出现、28 个原词、25 个规范导航词。
- 高频原词：charger placement、directional charging、wave interference、wireless power transfer，各 2 次。
- Wu 论文缺作者关键词，保留 `not_found`，不自动推断。

## 4. 链接与 A/B 隔离

- Wiki 断链：0。
- A 类页面中“我们将贡献 / 我的 idea / 本文新颖之处”扫描：0 命中。
- 正式 problem 只定义问题与评测轴，不声称算法已验证或全球新颖。

## 5. 自动化验收

- `py -3.12 tools/domain_keywords.py --check`：通过。
- `py -3.12 tools/wiki_eval.py`：10 个 Query 契约通过。
- `py -3.12 -m unittest discover -s tests -v`：24/24 通过。

## 6. Graphify 一致性

- 旧图查询可定位 wiki/index/library-status/vocab，但没有本轮新增领域关键词节点，证明图快照已过期。
- 未运行会重新引入工具噪声的代码型增量更新；待文档语义 backend 可用后全量重建。
- 在此之前 Query 应以 `wiki/index.md`、`library-status.md`、`map-domain-keywords.md` 和正文为准。

## Next Steps

1. 人工精读 10 个 selected 候选并核对版本/DOI；再决定哪些晋升 canonical。
2. 若准备把 problem 孵化为 idea，先锁定硬件动作、移动性、公平定义和切换成本。
3. 文档语义 Graphify backend 可用后全量刷新图，并重新检查工具噪声。
