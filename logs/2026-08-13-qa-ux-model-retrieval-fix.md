# 2026-08-13 智能问答交互、模型与文献检索修复

## 变更

- 修复条件错误横幅缺失时 composer 被 CSS Grid 自动放入弹性消息行的问题；输入框保持三行起始高度并限制自动增长。
- Codex 订阅状态投影本机默认模型、reasoning effort 与 list-visible 模型目录；设置页支持自动跟随或显式选择模型/推理强度，执行仍保持忽略用户规则和只读沙箱。
- 引用解析在事实切分前规范化已知证据的复合引用与来源定位，未知编号、范围和 Markdown 隐藏区域继续 fail closed；规范化结果统一用于审计、持久化与前端显示。
- 新增 literature lookup 意图及四段回答结构；“波干扰/波干涉”统一召回 wave interference 论文。
- 新增 `validation_started` 流事件，以及 Thinking 阶段链和实时耗时。
- 发布版本同步为 0.12.4。

## 验证

- `npm run build`
- `npm run test:qa-settings`：5/5
- `npm run test:p1`：17/17
- `cargo test`：96/96
- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `npm run verify:p3`：通过；Wiki 10/10，两书 Recall@5 为 1.000000 / 0.986667
- `npm run verify:p5`：通过；真实 GUI 启动/导航探针通过
- `py -3 tools/wiki_lint.py`：75 页，0 errors，1 个既有 warning
- `py -3 -m unittest tests.test_qa_accuracy_eval`：11/11

## 边界

- 未修改 Raw、Wiki、正式词表、Graphify 派生正文或 B 类页面。
- 未读取、返回或持久化 Codex token/cookie/API Key。
- 用户未跟踪的智能问答交接文档未纳入提交。

## 0.12.4 构建产物

- Release EXE：`apps/desktop/src-tauri/target/release/app.exe`，26,875,904 bytes，SHA-256 `069DC929F0A54A71F788C0325430A95A4A571D50B1769F242AFEC78802391AF2`。
- MSI：`apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.12.4_x64_en-US.msi`，15,384,576 bytes，SHA-256 `C977DC577C94D7A4D4FF54EC549BAA38028508B637F0687D1ADD34C21154143A`。
- NSIS：`apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.12.4_x64-setup.exe`，11,285,990 bytes，SHA-256 `ACF30EFFB5BDDAB8EF9C758E5C78EB09D73FAF84757C7C9F0E234A31CA4C3BF7`。
- `app.exe` 与 NSIS ProductVersion 均为 0.12.4；Release EXE 启动后窗口句柄非零且进程响应正常。
