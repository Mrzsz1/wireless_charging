# 深化 LLM Wiki 结构与检索 — 实施计划

## 阶段 A：记录与基线

- [x] 写结构审查报告，追加根 PRD 阶段条目。
- [x] 保存当前 Wiki Lint、页面类型计数、Graphify 缺页和评测基线。
- [x] 核对全部 source 的 `raw_md`，确认 book 与 paper 分流。

## 阶段 B：P0 一致性

- [x] 更新 `wiki/index.md`：集中列全 23 sources、20 methods、7 syntheses。
- [x] 更新 `wiki/maps/map-home.md` 与 `wiki/maps/library-status.md`。
- [x] 修复固定评测答案的旧水位；增强评测以从实际 `library-status` 校验水位。
- [x] 增强 Wiki Lint：详细度结构和硬编码水位漂移报告。

## 阶段 C：P0 论文原文章节索引

- [x] 在 Tauri SQLite schema 增加 `paper_sections` / `paper_sections_fts`。
- [x] 实现 raw 路径边界校验、Markdown 章节解析和字符安全分块。
- [x] 在完整重建流程中从 source frontmatter 只读构建论文索引。
- [x] 在 QA 中新增 `paper_candidates`、渠道感知去重和多渠道保留。
- [x] 更新 prompt 的证据级别说明。
- [x] 添加 Rust 回归：分块、路径边界、重建、双语 FTS、Wiki+paper 并存、聊天保留。

## 阶段 D：P1 Schema 与知识页

- [x] 更新 `schema/page-types.md`、`schema/agent-a-compile.md`、`schema/lint-checklist.md`。
- [x] 新建 4 个 system-model 页面。
- [x] 新建 4 个 objective 页面。
- [x] 新建 1 个通用 WRSN 仿真/证据协议页面。
- [x] 深化 CCSP、GAIN、TIDE、CUAV、IHATRPO 的 source/method 页面。
- [x] 重写 `map-models-and-objectives`，更新各相关 map/synthesis 链接。

## 阶段 E：派生图与验证

- [x] 运行 Wiki Lint 并修复本任务引入的问题。
- [x] 运行 Wiki Eval 与核心书籍检索评测。
- [x] 运行 Rust fmt、Clippy、完整测试；运行前端 build。
- [x] 执行 Graphify 增量重建并复核此前缺少的 8 个 source。
- [x] 追加 `logs/log.md` 和完成报告。

## 验证命令

```powershell
py -3 tools/wiki_lint.py
py -3 tools/wiki_eval.py --answers-dir evals/answers
py -3 tools/core_book_eval.py
cargo fmt --check --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm --prefix apps/desktop run build
graphify extract . --update  # no-key 时按 Graphify skill 由 host agent 抽取后增量合并
```

## 回滚点

1. 文档/Wiki 改动与 Rust 索引代码分批暂存，便于分别回滚。
2. 不纳入两个既有未跟踪 discovery 运行目录。
3. Graphify 是派生输出，失败时保留 Wiki 真相并记录失败。
