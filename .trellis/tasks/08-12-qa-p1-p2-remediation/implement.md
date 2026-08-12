# 实施计划

## 1. 后端意图、多轮与排序

- [x] 引入 canonical intent 常量并修复 method 保底。
- [x] 扩展有界 query term 生成与 intent 分类回归。
- [x] 重构实体提取，精确记录 source message IDs。
- [x] 实现通道内 score normalization/RRF 和 MMR 选择。
- [x] 运行 QA/Gold retrieval 回归，必要时调整阈值但不放宽证据契约。

## 2. 可信度门禁

- [x] 扩展 Rust/TypeScript CitationValidation DTO，保持旧 JSON 默认兼容。
- [x] 实现 claim splitter、coverage、graph-only 检查。
- [x] 更新 prompt，使输出格式与门禁一致。
- [x] 覆盖 supported、missing claim、unknown ID、graph-only、metadata、zero-evidence 测试。

## 3. Provider 完整性

- [x] 抽取 SSE parser/state。
- [x] 对 DONE/stop/length/error/malformed/EOF 建立单元测试。
- [x] 确认 provider 错误继续落 paired failed exchange。

## 4. 前端展示

- [x] 安装并接入安全 Markdown/GFM/math/KaTeX renderer。
- [x] 保留 citation 点击和未知 citation 样式。
- [x] 展示结构化引用覆盖状态与语义核验边界。
- [x] 修复 retrieving 与 completed-zero-evidence 空态。
- [x] 为展示纯函数新增 Node tests。

## 5. 全量验证

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --lib --tests -- -D warnings`
- [x] `cargo test --lib`
- [x] `npm run test:p1`
- [x] `npm run build`
- [x] `py -3 tools/wiki_eval.py --answers-dir evals/answers`
- [x] `git diff --check`
- [x] 更新 QA code-spec 与任务验收状态。

## 6. 修复后复审

- [x] 修复数字结尾句合并导致的 claim 覆盖绕过。
- [x] 保留携带 `finish_reason=stop` 的终止帧最终 token。
- [x] 让 required kind、method 与 Wiki/paper 配对保持单调，不再互相驱逐。
- [x] 正确校验 GFM 表头、分隔行与逐行事实引用。
- [x] 引用投影不改写代码、公式、转义 token 和已有 Markdown 链接。
- [x] 重新运行全量 Rust、Node、构建、Gold、Trellis 与 diff 门禁。
- [x] 输出 `re-review-report.md` 并同步 QA code-spec。
