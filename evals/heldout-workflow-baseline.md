# Independent Held-out Workflow 基线

- Schema：`qa-independent-heldout-workflow-v1`
- Curator template：50 个空白 case slot；不包含模型生成问题或答案。
- Freeze：要求 `independent=true`、curator SHA-256、dataset version、≥30 非空唯一问题和 cases seal。
- Open Research annotation：canonical critical-constraint IDs 与 acceptable method-family IDs。
- Blind export：Reviewer 只看到 Question/Answer/Claim/Evidence/Citation，不看到 run manifest 或 system verdict。
- Review：A/B 必须为不同 blinded independent reviewer；分歧由不同的 C reviewer 裁决。
- Derivation：同一 dataset/run seal 同时生成 heldout、grounding、open-research，禁止三套数据漂移。
- Contract tests：template/freeze/blind export/same-run derivation 与既有 checksum/adjudication tests 全部 PASS。

当前真实外部题目和人工 reviewer verdict 尚未提供，因此工具链完成后生产工件仍保持缺失，Release
Gate 继续 FAIL。这是独立证据边界，不以 Codex 自建、自答、自审的数据替代。
