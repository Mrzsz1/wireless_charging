# 回归答案初审 — 2026-08-01

本轮答案基线由当前会话按 `gold_questions.json` 逐题生成，并以10分制做证据链、边界和完整性初审。确定性契约由 `tools/wiki_eval.py` 执行。

| case | 链接/水位 | 必提概念 | 方法边界 | 完整性 | 总分 |
|---|---:|---:|---:|---:|---:|
| solve-interference-switching | 2 | 2 | 2 | 2 | 8 |
| solve-mobile-known-trajectory | 2 | 2 | 2 | 2 | 8 |
| solve-online-directional-requests | 2 | 2 | 2 | 2 | 8 |
| solve-peak-aoi | 2 | 2 | 2 | 2 | 8 |
| solve-paid-cooperative-service | 2 | 2 | 2 | 2 | 8 |
| novelty-joint-placement-online-interference | 2 | 2 | 2 | 2 | 8 |
| novelty-mobile-partial-deadline | 2 | 2 | 2 | 2 | 8 |
| novelty-dwpt-ev-scheduling | 2 | 2 | 2 | 2 | 8 |
| relation-gain-tide | 2 | 2 | 2 | 2 | 8 |
| relation-additive-vs-interference | 2 | 2 | 2 | 2 | 8 |

## 结果

- 10/10 答案文件存在。
- 10/10 通过预期wikilink、库水位和`must_mention`检查。
- 初审最低分8/10，未发现把库内结论升级为全球新颖性或混淆功率可加/波干涉边界的情况。
- 后续若切换到Luna生成新答案，应保留本目录基线并再次运行同一评测；不要覆盖基线而丢失回归对照。
