# 设置集中管理与列表分页：实施计划

## 1. 建立回归与状态 helper

- [x] 新增分页 helper 与 TypeScript 测试：空列表、66 条默认页、末页、越界、10/20/50、有限页码。
- [x] 增加设置/凭据 UI 结构门禁，先断言自动页仍包含配置表单、设置页缺少自动化和 API Key 区域。
- [x] 为搜索凭据 provider 元数据、无秘密状态 DTO 和环境注入增加 Rust 测试。

验证：`npm run test:pagination`、`cargo test search_credentials`。

## 2. 实现 Windows 安全凭据层

- [x] 引入兼容 Rust 1.77 的 `keyring 3.6` Windows-native backend。
- [x] 新建 `search_credentials.rs`，封装 provider allowlist、状态、保存、清除、读取、连接测试与命令环境注入。
- [x] Tauri 注册 list/save/delete/test 四个受控命令；所有阻塞/联网操作使用 `spawn_blocking`。
- [x] `compile_center::TaskSpec` 增加搜索凭据标志，仅 discovery/literature 任务向 Python 子进程注入已配置环境变量。
- [x] 保留环境变量和旧 Key 文件回退，不将空值覆盖进子进程。

回滚点：移除模块、命令和 TaskSpec 标志即可，数据库无 migration。

## 3. 建立正式设置页

- [x] 新增 `SettingsView.tsx` 与 CSS，迁入知识库、主题、字号、更新器等现有设置内容。
- [x] 添加文献自动化表单，加载/保存既有 `LiteratureIngestSettings`，含校验、错误、成功和未选库状态。
- [x] 添加四个搜索源卡片；Key 输入只保留当前组件内存，保存后清空，支持显示/隐藏、清除与连接测试。
- [x] `App.tsx` 改为渲染新组件并保留原回调/测试 ID。

## 4. 精简自动添加页

- [x] 删除 `LiteratureIngestView` 的配置编辑、保存函数和配置侧栏。
- [x] 保留只读设置加载、启动自动运行与完成后刷新。
- [x] 自动页改为单栏，增加配置摘要和“前往设置”按钮。
- [x] 更新 Props、CSS 和启动提示文字，避免仍指向“自动设置卡片”。

## 5. 文献库/方法库分页

- [x] `LibraryView` 使用分页 helper 切片可见结果。
- [x] 增加总范围、每页数量、上一页/下一页和有限页码按钮。
- [x] 查询、筛选、排序、页面类型、页大小变化重置；结果收缩自动收敛。
- [x] 添加可访问标签、禁用边界状态、分页 test IDs 和响应式 CSS。

## 6. 集成验证

- [x] 运行前端分页/状态测试、P1/P2/ingest/research-trail、构建和 verify P3/P4/P5。
- [x] 运行 Rust fmt、Clippy `-D warnings`、完整 tests；运行 Python 45 tests、Wiki Eval/Lint、两书 Recall@5。
- [x] 密钥扫描：SQLite/manifest/logs/diff 不出现测试秘密；raw 失败目录保持未跟踪。
- [x] strict GUI 覆盖设置、文献分页、自动页；两种目标视口通过。

## 7. 版本、打包与 Git

- [x] 版本提升到 0.10.0，同步 package/Cargo/Tauri/updater fixtures/README/PRD/log。
- [x] 构建 release、MSI、NSIS，计算大小和 SHA-256。
- [x] strict NSIS 安装/启动/退出/卸载通过。
- [x] 按 Trellis check 复核，显式暂存且排除 `raw/inbox/auto-discovered/runs/`，提交、归档任务并记录 journal。

停止条件：任何 Key 出现在持久化/日志/任务参数/Git，自动运行不再读取最新设置，分页改变排序或搜索结果，或用户 raw 目录进入暂存区时不发布。


## 实施结果

- 版本：0.10.0。
- 前端：分页 4/4、设置结构 3/3，P1/P2/research-trail/ingest、build、verify P3/P4/P5 全部通过。
- 后端：Rust fmt、Clippy `-D warnings`、45/45 tests；Python 45/45。
- 知识门禁：Wiki Eval 10/10；两书 Recall@5 1.000 / 0.986667；Wiki Lint 0 errors、2 个既有 warning 保留。
- GUI/安装：真实 release strict GUI 覆盖 1366×768 与 1920×1080；NSIS 隔离安装/启动/退出/卸载通过。
- 用户数据：两个 `raw/inbox/auto-discovered/runs/search-*` 目录未修改、未暂存。
