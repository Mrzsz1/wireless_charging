# 技术设计

## 解析边界

在 `claim_segments` 识别句末标点时，仅向后吸附同一行内立即相邻的一组合法 `[E数字]`，允许水平空白及一层中英文括号。吸附完成后立即结束当前 claim；遇到换行则不跨越。

## UI

`MarkdownMessage` 继续使用 Markdown-aware citation projection，但内部 `evidence:` 链接渲染为 `<a>` 而不是按钮。点击阻止 hash 导航并调用现有来源打开逻辑。证据详情以短文本链接替代原始路径代码块。

## Compatibility

持久化回答、EvidenceItem、审计包和 Tauri RPC 均不改 schema。失败门禁强度不降低，仅修复同句边界归属。
