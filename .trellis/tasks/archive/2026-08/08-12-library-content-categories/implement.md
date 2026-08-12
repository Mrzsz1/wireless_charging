# 实施步骤

## A. 分类契约

- [x] 新建内容分类纯函数模块：类型映射、计数、过滤、兜底标签。
- [x] 新增单元测试覆盖 75 页类型基线、map 中文标签、未知类型与过滤行为。

## B. 文献库界面

- [x] `LibraryView` 增加分类状态、分类栏、计数与分页重置。
- [x] 方法库固定 method；文献库默认全部。
- [x] 更新卡片类型图标和中文标签，`map` 显示“知识地图”。
- [x] 更新 CSS：紧凑分类栏、激活/焦点态、窄窗口横向滚动。

## C. 验证与发布

- [x] 运行分类测试、现有分页/搜索测试、前端 build、P5/GUI E2E。
- [x] 运行 Wiki/Python/Rust必要门禁并检查 raw/vocab 边界。
- [x] 版本升至 0.12.2，构建 MSI/NSIS、静默安装并验证版本和启动。
- [x] 更新 PRD/日志/Trellis spec，Git 提交、归档与 journal。

## 验证命令

```powershell
node --experimental-strip-types --test apps/desktop/tests/library-categories.test.ts
npm --prefix apps/desktop run test:pagination
npm --prefix apps/desktop run build
npm --prefix apps/desktop run verify:p5
cargo fmt --check --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
python -X utf8 -m unittest discover -s tests -p "test_*.py"
```
