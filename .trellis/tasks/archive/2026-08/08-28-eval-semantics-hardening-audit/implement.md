# Implementation Plan

1. 建立需求证据矩阵
   - 对照附件第 40 节逐条定位实现、测试、文档与报告字段。
   - 核对已归档任务和提交 `53ee2ba`，区分已完成项与真实缺口。

2. 聚焦语义复核
   - 核对 zero-evidence eligibility、Option/null 和 aggregate denominator。
   - 核对 canonical work/exact-source views、expected/returned dedup 和 metric consistency。
   - 核对 zero-evidence confusion definitions、report v4 identity、Markdown renderer 和 release mapping。

3. 缺口补齐（仅在发现缺口时）
   - 只修改 evaluation、metric helpers、report、tests、docs 或 release metric mapping。
   - 每个补丁必须绑定一个失败测试或可复现的契约缺口。
   - 不修改 production QA pipeline 和 held-out 数据/运行。

4. 验证
   - `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check`
   - 聚焦 `qa::evaluation::tests` 与 `qa::metrics::tests`
   - `py -3 -m unittest tests.test_qa_release_gate`
   - `cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --lib -- -D warnings`
   - 只有跨层输出需要重新生成时才执行 `npm run eval:rag`。

5. 完成
   - 更新 QA contract/spec（仅当发现新契约）。
   - 记录审计结论、测试结果、是否产生代码修改和真实指标。
   - 本地 Git commit 保存；收到明确上传指令后再推送 GitHub。

## Rollback Points

- 审计记录与代码补丁分开提交，便于撤销无必要的实现改动。
- 任一补丁触及 production Retrieval 行为时停止并回到规划边界复核。

## Completion Record

- Evidence matrix: complete in `audit.md`.
- Confirmed gap: production optional-average denominator and real-report null serialization lacked direct regression assertions.
- Fix: behavior-preserving `average_present` helper plus production-path unit/integration assertions.
- Tests: evaluation 10 passed; metrics 4 passed; real 13-case suite integration 1 passed; Python 8 passed.
- Quality: fmt and clippy passed.
- Production/held-out: no production Retrieval/QA behavior changed; no held-out data used.
