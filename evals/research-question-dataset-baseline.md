# 冻结科研问题集基线

- 状态：PASS
- schema：`research-question-dataset-v1`
- 总题数：360
- development：160
- regression：120
- heldout：80
- domain：12（每类 30）
- intent：10（每类 36）
- 难度：medium / hard
- 重复 ID：0
- 重复问题：0
- heldout 期望答案/证据泄漏：0
- canonical cases SHA-256：`6e9b2279cca1ac18cdf029367e12b092bfd2174cbb06feb943eafb1a70af1e32`

该 heldout split 从现在起封存，不参与规则调参。它用于结构、路由、检索与成本的盲测候选；
任何生产事实准确率声明仍需把题目按 `heldout_questions.json` 独立研究者流程冻结，并完成匿名
双人逐 claim 复核与分歧裁决。
