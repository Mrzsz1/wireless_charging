# P2 全量研究档案与原文证据评测

## 触发原因

P0/P1 已建立论文原文章节检索与五组深度样板，但其余 16 篇 source、15 个 method 仍偏摘要卡；固定问答只校验 Wiki 链接，无法证明召回了哪篇原文及其位置。

## 读取的 raw

只读使用 16 个 source frontmatter 中声明的 `raw_md`，覆盖 Alzenad、Binh、Chen、WANDA、ROSE、RA-DMCS、FELKH-3D、Honma、DWC-BEB、DCHSA/ADTSA、ISAC-UAV、obstacle-MCV、DICCS、HIPO、Charging on the Move 与 CCS。未改 raw 正文或状态。

## 修改范围

- `wiki/sources/`：16 篇剩余论文研究档案化。
- `wiki/methods/`：15 个剩余方法补齐输入输出、步骤、保证、实验与失效边界。
- `wiki/index.md`、`wiki/maps/library-status.md`、模型地图与 5 个 synthesis：同步导航和内容水位。
- `evals/gold_questions.json`、10 份答案、`tools/wiki_eval.py` 与 Rust 回归：升级 Gold Contract v2。
- `prd.md`、Trellis 任务与日志：记录范围、约束和验收。

## 质量边界

- 定量结果只保留已有 source 或 raw 可回溯陈述；不补造 PDF 页码、复杂度或实验数值。
- 原文未报告的保证明确标记；启发式与元启发式不写成普适最优。
- 未新建/修改 problem、idea、新 Map 或正式词表。

## 验收结果

- Wiki Lint：75 页、0 errors、1 个既有 B 层 warning；Gold Contract v2：10/10。
- 核心书籍：Algorithmic Game Theory Recall@5=1.0；Approximation Algorithms Recall@5=0.986667。
- Graphify：3263 nodes / 5464 edges / 199 communities；多重边诊断 0 dangling、0 duplicate，查询可命中 P2 source/method、原文证据与评测节点。
- Python：49/49；Rust：56/56；`cargo fmt`、Clippy `-D warnings`、前端 build、P5 与真实 GUI E2E 均通过。
- 客户端同步到 0.12.1：生成 MSI/NSIS，NSIS 已静默安装；注册表、ProductVersion 与启动探针均为 0.12.1。

## 发布产物

- app：22,598,144 bytes；SHA-256 `7C6F01381A729837C22F58E8F568E4D19CFD11FF3801EB7EB99DCF069B6C27EA`
- MSI：11,554,816 bytes；SHA-256 `445A5C59DDA43CE8875184FAE69FA37CC546DA309D96FCE808819F0B81120F59`
- NSIS：8,022,943 bytes；SHA-256 `2CBFD6916616607A7CCAA03069DAC0925A34DF68F5ED40D5EC13FC4AAF238814`
