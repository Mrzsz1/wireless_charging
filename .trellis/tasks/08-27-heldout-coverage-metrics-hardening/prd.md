# 修复 Held-out 方法召回与关键约束覆盖评测

## Goal

修复 Independent Held-out 评测中方法召回率和关键约束保留率的分母污染问题，使两个指标严格由冻结 case 的期望集合定义，并让冻结来源、双评审与分歧裁决全程可审计、失败关闭。

## Requirements

- `relevantMethodRecall` 的分母只能来自每个 frozen case 的 `acceptableMethodFamilies`；系统输出的 method claim 数量不得影响分母。
- `criticalConstraintPreservation` 的分母只能来自每个 frozen case 的 `criticalConstraints`；必须逐项判定冻结约束是否被最终回答保留。
- 两名独立盲评 reviewer 必须分别完整提交 claim verdict、method-family coverage verdict 和 critical-constraint coverage verdict。
- 任一 claim、method family 或 critical constraint 的双评审结果不一致时，必须由第三名不同 reviewer 仅裁决全部分歧项。
- `answerClaims.dimension` 明确为 claim 分类元数据；缺省按 `factual` 兼容旧 runner，但不得用于 method/constraint 指标分母或分子。
- freeze workflow 只允许从 sealed `evals/research_questions_v1.json#split=heldout` 的 80 个候选中选题，并校验 ID、question、ResearchIntent/type 完全一致。
- 不读取真实 heldout 题目来构造测试；单元测试使用自建 sealed 80-candidate fixture。
- 保持 pending 公共入口和现有 runner 兼容；无正式冻结数据时仍为 pending。

## Acceptance Criteria

- [x] `Expected methods=[A,B,C,D]` 且回答只覆盖 A 时，`relevantMethodRecall == 0.25`，即使唯一 claim 的 dimension 为 `factual` 或 `method` 也不变。
- [x] `Expected constraints=[X,Y,Z]` 且回答只保留 X 时，`criticalConstraintPreservation == 1/3`。
- [x] coverage 缺项、重复项、未知项、非法 verdict 均失败关闭。
- [x] method/constraint 双评审分歧缺少第三人裁决时失败；第三人必须与两名 primary reviewer 不同且只覆盖全部分歧项。
- [x] freeze 对非 sealed pool、heldout 候选数不等于 80、pool seal 错误、ID/question/type 漂移全部失败关闭。
- [x] blind review bundle 明确包含 expected method families、expected critical constraints 及各自 verdict 契约。
- [x] Python evaluator/workflow tests 通过，并更新 `evals/heldout_questions.json` 与 backend QA contract。

## Notes

- 本任务不生成或修改正式 heldout case 内容，只强化 schema、校验、评审和指标派生。
- 本任务不上传 GitHub；按阶段使用本地 Git commit 保存。
