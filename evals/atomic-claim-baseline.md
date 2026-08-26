# Atomic Claim Extraction 基线

- Schema：`qa-atomic-claim-cases-v1`
- Extractor：`atomic-claim-extractor-v1`
- 用例：50/50 PASS
- 覆盖：建议+事实、事实+建议、并列事实、带不确定性限定的综合推断、通用知识、英文复合句、citation laundering、转折事实和句界引用。
- Evidence 规则：句尾 citation 只保留在最后一个本地原子 clause；前置 citation 只属于其所在 clause。
- 初始状态：所有提取结果固定为 `unverified`，confidence 为 `None`；verification 阶段另行决定最终状态。

该基线只证明 deterministic atomic segmentation/typing/mapping contract，不代表 semantic entailment 或 production factual precision。
