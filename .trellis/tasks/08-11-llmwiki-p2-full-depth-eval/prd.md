# LLM Wiki P2：全量研究档案与证据型评测

## Goal

在 P0/P1 已建立的论文原文检索和五组深度样板之上，把剩余论文 source/method
升级为可供研究者阅读的研究档案，并修复已经发生编码损坏的固定评测集，使评测真正检查
“召回了什么证据、证据位于哪里、关键约束是否覆盖”，而不是只检查链接和乱码关键词。

## Background

- 当前 21 篇非 book source 中只有 5 篇按新结构深化；其余 16 篇正文约 519–773 字符。
- 20 个 method 中只有 5 页深化；其余 15 页正文约 156–366 字符。
- `evals/gold_questions.json` 和 10 份 `evals/answers/*.md` 的中文正文已经发生不可逆替换字符/乱码，
  现有 Wiki Eval 仍可通过，说明当前契约会把“同源乱码匹配”误判为有效质量。
- 当前 Rust 固定问题回归只要求至少命中一个预期 Wiki 页面，不检查 paper 原文证据、
  `sourceLocation`、关键约束或证据渠道多样性。

## Requirements

### R1 修复评测语料

- 以产品语义重新写入 10 个中文问题、must-mention 和 10 份中文答案，禁止尝试从替换字符反推原字节。
- 保持 5 solve / 3 novelty / 2 relationship 配额和稳定 case ID。
- DWPT novelty 用例必须反映当前库已有 Honma 2026 与 Li 2024 证据，不再沿用“库内没有 DWPT”旧判断。

### R2 证据型 Gold Contract v2

- 每个 case 增加 `evidence_contract`：必须证据类型、允许的 primary source、是否要求原文位置、关键约束。
- `wiki_eval.py` 校验结构、答案中的当前库水位、关键约束、原文证据位置和无证据边界措辞。
- Rust 真实仓库回归必须按 case 验证 Wiki 命中、paper 命中、paper source ID 和非空章节/行号。

### R3 全量 source 深化

- 深化剩余 16 篇论文 source：Alzenad、Binh、Chen、Dai/WANDA、ROSE、Gao RA-DMCS、
  Gao FELKH、Honma、Li DWC-BEB、Liu DCHSA/ADTSA、Qaisar、Rahaman、Tian DICCS、
  Wang HIPO、Wu Charging on the Move、Xu CCS。
- 每页包含 TL;DR、适用/不适用、系统模型、变量、目标与约束、算法、理论/复杂度、实验、
  定量结果、局限和 raw 行号。原文未报告的字段明确标记。

### R4 全量 method 深化

- 深化对应的 15 个剩余 method，形成可复用的输入/输出、步骤、复杂度/保证、适用边界和证据锚点。
- 每页链接适用的 system-model、objective、source；不得把单篇实验结论扩写成普适保证。

### R5 导航与派生图

- 更新 index、模型目标地图、相关 synthesis/map，使新增细节可从任务路径进入。
- 重建 Graphify 语义图；严格 Lint 不得出现 Wiki 未纳入图的新 warning。

## Acceptance Criteria

- [ ] 10 个 gold 问题和答案为正常 UTF-8 中文，不包含 `U+FFFD` 或大段乱码。
- [ ] Gold Contract 升级到 v2，10 个 case 均声明 paper/source-location/critical-constraint 要求。
- [ ] 真实仓库 Rust 回归证明 10 个 case 满足各自证据契约。
- [ ] 剩余 16 source 和 15 method 全部通过新详细度门禁。
- [ ] Wiki Lint 0 errors、0 断链、0 孤页；只允许既有 B 类 warning。
- [ ] Wiki Eval 10/10，核心书籍 295 条 Recall@5 继续高于 95%。
- [ ] Graphify 覆盖全部 Wiki 页面。
- [ ] Python、Rust、前端构建通过；提交不包含两个既有 raw discovery 目录。

## Constraints

- raw 正文只读；不得补造 PDF 页码、复杂度、定理或实验数字。
- 不修改 `schema/vocab.yaml`，不新增或改写 B 类 problem/idea。
- 不默认外搜；仅使用 canonical raw、两本核心书和当前 Wiki。
- 不用 Graphify `--wiki` 覆盖本库 Wiki。

