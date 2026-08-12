# P2 实施步骤

## A. 基线与契约

- [x] 记录 source/method 详细度和评测编码损坏基线。
- [x] 重写 `gold_questions.json` 为 UTF-8 v2，并补齐 evidence contract。
- [x] 重写 10 份固定答案，包含当前水位、关键约束、原文证据与边界。
- [x] 增强 `wiki_eval.py` 和 Python 测试。
- [x] 增强 Rust 真实仓库 gold evidence contract 回归。

## B. 内容深化

- [x] 完成批次 A：放置/安全。
- [x] 完成批次 B：移动服务。
- [x] 完成批次 C：AoI/聚类。
- [x] 完成批次 D：DWPT/UAV。
- [x] 更新 index、maps、syntheses 和 library-status 内容健康度。

## C. 派生与验证

- [x] Graphify 增量语义重建。
- [x] Wiki Lint strict、Wiki Eval、核心书籍评测。
- [x] Python unittest、Rust fmt/clippy/test、前端 build。
- [x] 写完成日志、更新根 PRD 和 Trellis spec。
- [x] Git 提交、归档任务和记录 journal。（Phase 3.4）

## 验证命令

```powershell
py -3 tools/wiki_lint.py --strict-graphify
py -3 tools/wiki_eval.py --answers-dir evals/answers
py -3 tools/core_book_eval.py
py -3 -m unittest discover -s tests -p "test_*.py"
cargo fmt --check --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
npm --prefix apps/desktop run build
```
