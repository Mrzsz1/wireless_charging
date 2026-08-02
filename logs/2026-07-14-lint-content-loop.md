# Lint Report — 2026-07-14

## Summary

🟢 本轮内容闭环修改通过确定性检查；Graphify 语义图仍待有 LLM backend 时全量重建。

## 1. Schema 完整性

- wiki 数量：source 9、concept 5、method 8、synthesis 3、system-model/objective/dataset 0、problem/idea 0。
- 9/9 source 均有非空 `year`、`venue`、`doi`，并保留 provenance 与 ingest 状态。
- 新 synthesis 只使用 `schema/vocab.yaml` 已有 id。

## 2. 过期 / 漂移

- `wiki/index.md` 与 `wiki/maps/library-status.md` 已更新为 9 source / 3 synthesis。
- 自动发现状态复核：38 pending、0 selected、`selected_by_user: true` 为 0。
- 年份跨度已从估计值修正为正式卷期 2017–2024。

## 3. 覆盖缺口

- 新增两篇跨文献 synthesis，覆盖干涉感知路线与移动/在线服务路线。
- system-model/objective/dataset 继续为空；按新准入规则不为填目录而建页。
- 页面准入与 Lint 已增加低复用单源拆页检查。

## 4. 孤儿页

- 全部 wiki wikilink 解析：0 个断链。
- 非 map/index 页面：0 个无入链孤儿页。

## 5. 重复

- wiki source 未发现新增重复页。
- 自动候选中发现 1 组完全同题名记录、1 组疑似版本重叠；只写入初筛建议，未自动合并。

## 6. A/B 污染

- A 类目录未检出“我们将贡献 / 我的 idea / 本文新颖之处在于”等用户贡献句。
- 正式 problem/idea 数仍为 0；A→B 草案仅在 `logs/`，等待确认。

## 7. 冲突表述

- 两篇新 synthesis 均按“并存不裁断”书写，并明确不同模型与目标不可直接横向排名。

## 8. Graphify 一致性

- 已用现有图导航并保存本轮 query memory。
- 当前图是旧 174-node 语义图且含历史 skill 噪声；环境未配置受支持的 LLM backend key。
- 未运行已知会把 Markdown 主库生成代码结构噪声的 `graphify update .`；正文为权威，图标记待重建。

## 验证

- `py -3 tools/wiki_eval.py`：PASS，10 cases，类型配额 5/3/2。
- `py -3 -m unittest discover -s tests -v`：22/22 PASS。
- `py -3 -m py_compile ...`：PASS。

## Next Steps

1. 需用户确认：候选 select/reject 具体序号；A→B problem 草案是否正式写入。
2. 可自动继续：用户选中并完成 MinerU 后，执行下一批 canonical A 编译。
3. 外部状态：配置支持 Graphify 文档语义抽取的 LLM backend 后做全量重建。
