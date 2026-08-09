# 技术设计

## 1. 组件边界

- `App.tsx`：移动设置/帮助入口到自定义标题栏，删除侧栏 footer。
- `DelayedHelp.tsx`：提供统一的 1000ms 延迟帮助提示，负责计时、键盘、点击和 ARIA。
- `SettingsView.tsx`：只替换页面级及卡片级概览说明，不改变业务表单和服务调用。
- `ResearchTrailPanel.tsx`：替换刷新、折叠、恢复控件的表现层，保留原回调和刷新 nonce。
- `SidebarWorkspacePane.tsx`：允许 `.sidebar-footer` 不存在，使用 footer 高度 0 计算最大可拖动范围。
- CSS：在既有 `styles.css` / `SettingsView.css` 中增加局部样式，不引入组件库。

## 2. 交互合同

### 顶部菜单

- `data-testid=settings` 保持不变，避免现有自动化和导航契约失效。
- 新增 `data-testid=help`；两个按钮根据当前 view 提供 `aria-current=page`。
- 标题栏结构为：侧栏开关 → 产品名 → 设置/帮助 → 可拖动空白区 → 窗口控制。

### 延迟帮助

- `pointerenter` 启动 1000ms 计时；`pointerleave` 清除计时并关闭。
- `focus` 立即显示，`blur` 关闭；点击切换，兼容键盘和触屏。
- 按钮用 `aria-label`，展开时用 `aria-describedby` 指向 `role=tooltip`。
- 组件卸载时清理 timer，避免卸载后更新状态。

### 研究脉络工具栏

- 刷新按钮显示 Refresh 图标与“刷新”文字，保留 nonce +1 行为。
- 收起按钮使用 `PanelRightClose`；恢复使用 `PanelRightOpen`。
- 所有控件有 `aria-label`、focus-visible 和 pressed feedback。

## 3. 兼容与回滚

- 不改变持久化 key、Rust command、数据类型或仓库结构。
- 回滚只需恢复前端组件/CSS/E2E 文件；未引入迁移。
- 若标题栏空间不足，文字菜单保持固定宽度，拖动区缩小但不消失。

## 4. 验证

- TypeScript/Vite build。
- 现有 Node/Vitest 测试套件，不新增和不删除单元测试。
- GUI E2E 验证菜单位置、Tooltip 延迟、面板折叠/恢复和工作区拖动。
- Tauri release + NSIS 安装 + 安装版严格 GUI E2E。
