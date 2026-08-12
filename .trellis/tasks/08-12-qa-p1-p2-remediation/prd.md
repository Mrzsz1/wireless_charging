# 修复智能问答 P1/P2 审查问题

## Goal

修复桌面端 0.12.3 智能问答审查报告中的全部 P1/P2 问题，使解决类问题稳定保留方法证据、可信度状态与实际校验能力一致，并提升多轮检索、跨通道排序、兼容 API 流式完整性和科研回答可读性。

## Background

父任务 `.trellis/tasks/08-12-qa-feature-audit` 已确认当前实现有 2 个 P1 和 7 个 P2：`solve/solution` 意图字面量不一致、引用校验仅验证编号、规则检索覆盖不足、异构分数直接混排、多轮来源 ID 不准确、兼容 API 静默吞协议错误、可信度信息未展示、零证据空态错误、纯文本回答缺少科研格式渲染。

## Requirements

1. 统一 canonical intent 为 `solve | novelty | relationship`，solve/novelty 召回 method 时最终证据必须保留至少一个 method。
2. 引用校验增加 claim-level 引用覆盖率与 graph-only 门禁；`supported` 不得再只由“至少一个有效编号”决定。语义蕴含尚未执行时必须以结构化字段和 UI 文案明确标识。
3. 改善中文未知表达的检索覆盖，减少对固定整句 expansion 的依赖；保持既有 Gold Contract 10/10。
4. 对 Wiki/paper/linked-paper/book/Graphify 候选做通道内标准化与 rank fusion，最终选择进行相似结果去重，并且低相关通道不得仅因类型配额挤掉高相关结果。
5. 多轮实体必须记录实际贡献实体的历史消息 ID，并扩展常见中文省略/指代触发方式；旧引用仍不得复用。
6. 兼容 API 必须区分合法结束、限长截断、异常 finish reason、非法 JSON 和 EOF 断流；不完整回答不得持久化为 completed。
7. 前端展示引用校验范围、覆盖率和未支持 claim 数量；零证据完成态不得显示“等待检索”。
8. 回答支持安全 Markdown、GFM table、代码块和 KaTeX，同时保留 `[E#]` 点击定位；原始 HTML 不执行。
9. 保持取消、repository isolation、completed/unverified/failed 终态、离线回答、首轮失败恢复和密钥边界不回归。
10. 不修改 Raw/Wiki/Graphify 正文，不发起真实付费模型请求，不处理 P3 Graphify 性能和会话分页项。

## Acceptance Criteria

- [x] 所有 canonical intent 使用同一组常量，solve/novelty method 保底有单元回归。
- [x] 引用校验输出 claimCount、citedClaimCount、citationCoverage、unsupportedClaims、graphOnlyClaims、syntaxValid、coverageValid、entailmentChecked；缺引或仅 Graphify 支撑的事实 claim 会失败。
- [x] 历史实体能返回精确 usedHistoryMessageIds，并覆盖“那它呢/继续比较/第二个方法”等常见追问。
- [x] 候选排序使用通道标准化/rank fusion 与相似性去重；Gold retrieval 10/10 和 Wiki/paper/book 多样性回归通过。
- [x] 兼容 API parser 对 stop/DONE 正常完成，对 length、异常 reason、malformed JSON、EOF 断流给出可测试的错误。
- [x] completed 消息显示引用覆盖率与“语义未自动核验”；unverified/invalid 状态文案准确。
- [x] 零证据回答结束后证据面板显示“本轮未检索到参考来源”。
- [x] Markdown、GFM table、代码块、行内/块公式正常构建；`[E#]` 仍可点击，未知编号仍有错误样式。
- [x] Rust fmt、Clippy `-D warnings`、完整 Rust tests、前端 type-check/build、P1 状态测试和 Wiki Gold eval 全部通过。

## Out of Scope

- 不引入在线 embedding 服务或新的外部问答请求。
- 不实现 P3 Graphify 倒排索引、会话后端分页和全量运维指标。
- 不宣称 deterministic claim coverage 等同于语义蕴含或事实真实性。
