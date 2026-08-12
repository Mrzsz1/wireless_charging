# 技术设计

## 1. 变更边界

- Rust QA 核心：`apps/desktop/src-tauri/src/qa.rs`
- Tauri provider 生命周期：必要时调整 `apps/desktop/src-tauri/src/lib.rs`
- TypeScript DTO：`apps/desktop/src/types.ts`
- QA UI：`apps/desktop/src/features/qa/AskView.tsx`、`AskView.css`
- 前端纯状态/展示辅助：新增 `apps/desktop/src/features/qa/qaPresentation.ts`
- 前端依赖：`react-markdown`、`remark-gfm`、`remark-math`、`rehype-katex`、`katex`
- 测试：Rust QA 单元/真实仓库回归、Node 状态测试与前端 build。

## 2. Intent 与检索查询

定义 `INTENT_SOLVE`、`INTENT_NOVELTY`、`INTENT_RELATIONSHIP` 常量，所有分类、加权、配额和 method 保底只引用常量。意图分类仍保持本地确定性，但扩充中英文表达并避免散落字面量。

`query_terms` 保留已验证领域 expansion，同时从中文连续片段生成有界 2–4 字符片段，使未知组合表达不再完全依赖整句规则。所有 term 去重并保持严格上限，避免 FTS OR 查询失控。

## 3. 多轮实体来源

内部实体改为 `{ value, source_message_id }`。大写/模型 token 直接绑定产生它的 user turn；页面 title/id 匹配逐条最近历史，不再先拼接全文。最终 `entities` 去重，`usedHistoryMessageIds` 只收集真正贡献至少一个实体的消息并保持最近优先。

## 4. 跨通道融合与去重

每个通道返回自身排序后，在合并前执行：

1. 通道内 min-max 标准化 raw score；
2. 加入缩放后的 reciprocal-rank 分数；
3. 记录 fusion reason；
4. 再应用 intent bonus。

最终选择不再简单 `take(N)`，而是使用确定性 MMR 风格选择：候选基础分减去与已选候选的 title/page/source token 相似度惩罚。类型保底只接受达到最低融合分的候选。Wiki-primary paper 配对仍优先于 graph 提示，并保留现有来源可审计契约。

## 5. Claim-level 引用门禁

`CitationValidation` 增加：

- `claimCount`
- `citedClaimCount`
- `citationCoverage`
- `unsupportedClaims`
- `graphOnlyClaims`
- `syntaxValid`
- `coverageValid`
- `entailmentChecked`

确定性 claim splitter 按换行和中英文句末标点切分，忽略 Markdown 标题、纯标签、水位元数据和固定流程提示。其余达到最小信息长度的陈述句视为 factual claim。

一个 claim 只有在以下条件同时满足时计为 cited：

1. 至少包含一个当前合法 `[E#]`；
2. 不含未知编号；
3. 合法引用不全是 `kind=graph`。

`supported = syntaxValid && coverageValid && claimCount > 0`。本轮不实现语义模型，因此 `entailmentChecked=false`，UI 必须显示“引用覆盖已校验，语义未自动核验”。零证据继续走 `unverified`，不要求 claim 引用。

## 6. 兼容 API 流式状态机

将 SSE 单行处理抽成可单测的 parser/state：

- 空行、comment、`event/id/retry` 元数据：忽略；
- `data: [DONE]`：合法终止；
- JSON delta/message content：输出 token；
- `finish_reason=stop`：合法终止；
- `finish_reason=length`：`LUNA_RESPONSE_TRUNCATED`；
- 其他非空 reason：`LUNA_FINISH_ERROR`；
- 非法 JSON：`LUNA_STREAM_PROTOCOL_ERROR`；
- EOF 且无 DONE/stop：`LUNA_STREAM_INCOMPLETE`。

错误继续走现有 paired failed exchange，不保留 partial completed。

## 7. 前端可信度与 Markdown

`qaPresentation.ts` 提供纯函数：

- 根据 phase/waterline/evidence 返回证据空态；
- 生成 citation badge 的等级与说明；
- 将裸 `[E#]` 转换为受控 `evidence:` Markdown link。

`react-markdown` 默认不启用 raw HTML；GFM、math 和 KaTeX 仅负责渲染。自定义 `a` component 拦截 `evidence:E#`，合法编号触发证据定位，未知编号显示错误样式；普通 http(s) 链接使用安全新窗口属性。

## 8. 兼容与回滚

- 新增 CitationValidation 字段均使用 serde default，旧数据库 JSON 仍能读取。
- 不改变表结构和 IPC command 签名。
- 排序变更由 Gold retrieval 和 mixed-channel regression 守护。
- 若 Markdown 依赖导致构建问题，可回滚到纯文本组件而不影响后端 DTO。

