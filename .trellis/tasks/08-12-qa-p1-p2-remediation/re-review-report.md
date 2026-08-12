# 智能问答 P1/P2 修复复审报告

## 结论

本轮按当前工作区实际代码重新审查。原审查列出的 P1/P2 已落实；复审额外发现 2 个 P1 边界缺陷和 3 个 P2 一致性缺陷，均已直接修复并补充回归。全量质量门禁通过后，当前范围内没有遗留 P1/P2。

## 业务逻辑

1. 前端生成 `requestId`，写入乐观 user message，并立即开放停止操作。
2. 后端校验仓库身份与请求生命周期，只读取同仓库、`completed`、有界的历史消息。
3. 仅在显式指代/续问时解析历史实体，输出精确 `usedHistoryMessageIds`；历史引用编号不进入本轮证据。
4. Wiki、paper、linked-paper、book、Graphify 各自检索，先做通道内归一化和 RRF，再按 `solve | novelty | relationship` 加权。
5. 候选经过去重、MMR 风格多样化、类型上限、method 保底和 Wiki/primary-paper 配对，最终生成本轮 `[E#]` 证据包。
6. provider 使用同一证据包生成回答；兼容 API 只有在 `[DONE]` 或 `finish_reason=stop` 后才算完整。
7. 后端按事实 claim 检查同句引用、未知编号和 Graphify-only 引用。结构覆盖通过才保存 `completed`；零证据保存 `unverified`；协议/引用/provider 错误保存成对 `failed`；取消和切库不保存。
8. 前端安全渲染 Markdown/GFM/KaTeX，显示引用覆盖率与“语义未自动核验”，并提供 `[E#]` 到本轮证据详情的定位。

## 复审中发现并修复

### P1-1：数字结尾句可绕过逐 claim 覆盖

- 原因：句点前是数字时一律不分句，`There are 2. Next claim [E1].` 被合并为一个 claim。
- 修复：按“句点后为空白或文本结束”识别边界；小数点和 URL 点因后继不是空白自然保留。
- 回归：断言上述文本产生两个 claim，只有一个被引用。

### P1-2：终止 SSE 帧可能丢失最后 token

- 原因：同一帧同时包含 content 和 `finish_reason=stop` 时先返回完成态，content 未消费。
- 修复：新增 token-and-complete 状态，先发出/累加 token，再标记合法终止。
- 回归：终止帧 content=`final` 时最终答案和流式事件均保留 `final`。

### P2-1：后置配额修复可能互相驱逐

- 原因：required kind、method 和 Wiki/paper 配对使用尾部弹出，后一个修复可能删除前一个刚保留的结果。
- 修复：只移除最低分且未受保护的候选；保护最后一个 required kind、唯一 method 和已选择的 paper/Wiki 配对；没有安全槽位时跳过修复。
- 回归：真实仓库 Gold 合约仍同时命中预期 Wiki 与 primary paper。

### P2-2：GFM 表格会被引用门禁误判

- 原因：表头被当作无引用事实，短数据行反而可能因长度阈值被忽略。
- 修复：表头和分隔行按结构处理，每个数据行独立作为 factual claim。
- 回归：带引用的数据行通过，无引用的数据行失败。

### P2-3：引用投影会污染代码、公式和已有链接

- 原因：全局正则会把代码/数学表达式中的 `[E#]` 改为链接，也会制造嵌套 Markdown 链接。
- 修复：引用扫描器跳过 code、math、转义 token 和已有链接标签。
- 回归：只转换普通正文中的 `[E#]`。

## 优点

- 证据生命周期清晰：历史上下文、当前证据和 Graphify 提示边界明确。
- 失败语义可审计：截断、非法 JSON、异常 finish reason、EOF 断流均有稳定错误码。
- 零证据语义符合产品要求：明确告知无来源，回答标记 `unverified`，且不进入后续上下文。
- 证据选择同时兼顾相关度、多样性、method 可复用性和 primary-source 可核验性。
- Markdown 展示与后端引用门禁形成闭环，语义蕴含能力没有被夸大。

## 缺点与后续优化

### P3

1. `qa.rs` 同时承担检索、排序、provider、引用校验和持久化，文件体积大，后续宜按 retrieval/grounding/provider/persistence 拆模块。
2. Graphify 查询仍是缓存后的线性扫描；大图应增加倒排索引或预计算 token index。
3. 会话列表和详情仍缺少后端分页，长期使用会增加启动和切换成本。
4. 排序参数仍以启发式常量为主；Gold 10 题能防回归，但不足以衡量 NDCG、MRR、来源多样性与真实用户问题覆盖。
5. claim gate 只验证结构覆盖，不验证证据与 claim 的语义蕴含；当前 UI 已准确披露该边界。

## 复审门禁

- Rust fmt / Clippy `-D warnings` / 71 tests
- Node P1、P2、QA settings tests
- TypeScript + Vite production build
- Wiki Gold eval 10/10
- Trellis task validation
- `git diff --check`
