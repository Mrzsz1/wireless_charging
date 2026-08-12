# 结构化声明证据回答与统一渲染

## Goal

由模型输出结构化声明—证据数据，后端按结构校验并生成最终 Markdown，彻底取消对模型自然语言标点的事实边界猜测。

## Requirements

- 有证据的 Codex/API 回答必须是 `qa-structured-answer-v1` JSON。
- 每条 verified claim 显式携带 `evidenceIds`；后端校验编号存在、至少一个非 Graphify 来源。
- 章节、分组标题和参考证据由程序渲染，不参与事实覆盖率。
- 模型补充使用独立数组，禁止携带证据 ID，且不进入可信历史上下文。
- 正文只显示短 `[E#]` 链接；完整证据统一在末尾以短来源类型和定位列出，不显示长论文名或本地路径。
- 相同证据颜色稳定一致，颜色不能替代编号与可访问标签。
- 零证据与离线证据模式保持既有兼容路径。

## Acceptance Criteria

- [x] 同一结构化 claim 中的标点、分号、英文缩写不影响引用覆盖率。
- [x] 未绑定证据、未知证据、Graphify-only claim 均 fail closed。
- [x] 最终 Markdown 章节完整，正文引用与末尾参考证据均可点击。
- [x] 页面不展示长论文名和本地原文路径。
- [x] mixed supplement 被显示但不进入 trusted context。
- [x] Release 编译成功。

## Constraints

- 用户明确要求不运行测试；仅进行格式、类型、编译检查。
- 不实现引用语义蕴含。
