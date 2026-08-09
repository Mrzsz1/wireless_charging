# 实施计划：顶部页签重设计与侧栏空间伸缩

## 总体执行顺序

严格按“基线 → 尺寸规则 → 侧栏组件 → 页签组件 → 视觉整合 → 全量验证 → 构建安装 → Git 收口”执行。每一步完成后再进入下一步，避免在视觉问题尚未隔离时同时修改状态逻辑。

## 阶段 0：任务启动与基线

1. 用户批准本计划后运行 Trellis `task.py start`，把任务从 `planning` 切换为 `in_progress`。
2. 运行 `trellis-before-dev`，读取 frontend index、component、state、type-safety、quality 规范。
3. 检查 `git status --short`，记录并保护两个与本任务无关的 Raw 自动检索目录；后续禁止暂存它们。
4. 运行当前基线：
   - `npm run test:p1`
   - `npm run build`
   - 可用时运行 `npm run e2e:gui:strict`
5. 记录现有页签高度、我的空间实际高度，以及 1366×768 视口是否溢出，作为改动后比较基线。

## 阶段 1：高度规则

1. 新建 `src/lib/sidebarWorkspaceSize.ts`，定义存储键、版本、默认值、最小值和键盘步长。
2. 实现合法值解析、上下限夹取、键盘命令计算三个纯函数。
3. 不新增或删除单元测试；使用 TypeScript 严格编译、现有回归测试和 GUI E2E 验证这些规则。

## 阶段 2：实现可伸缩“我的空间”组件

1. 新建 `SidebarWorkspacePane.tsx`，把面板和拖动柄封装为单一组件。
2. 使用防御性读取恢复 `desktop.sidebar-workspace-height.v1`，非法值采用默认高度。
3. 实现动态最大高度测量：读取侧栏、面板、拖动柄、footer 和 spacer 的几何信息。
4. 实现 Pointer Down/Move/Up/Cancel：
   - 捕获起始 Y 和起始高度；
   - 启用 Pointer Capture；
   - 每次移动经 clamp 后实时设置高度；
   - 结束/取消时释放 capture、清除 dragging 状态并持久化最终值。
5. 实现键盘 ArrowUp/ArrowDown/Home/End 和双击默认值。
6. 添加完整 separator ARIA 属性和可见焦点。
7. 在 `App.tsx` 中用该组件包裹当前“我的空间”标题和目录树；给侧栏、footer、spacer 增加必要的 ref 或 data-testid。
8. 确认折叠侧栏时组件不渲染，展开后恢复高度和目录展开状态。

## 阶段 3：重构页签语义与行为

1. 在 `TabBar.tsx` 中把 `role=tab` 移到选择按钮，外壳改为无语义布局容器。
2. 为活动/非活动 tab 设置 roving `tabIndex`。
3. 为页签标题添加省略和 `title`；关闭按钮保留明确 `aria-label`。
4. 关闭事件执行 `stopPropagation`，确保关闭动作不会产生额外选择。
5. 保持 props 和 App 页签状态逻辑不变，避免扩大变更面。
6. 手工验证：两页签切换、关闭活动页签、关闭非活动页签、最后一个页签不显示关闭按钮。

## 阶段 4：视觉样式整合

1. 重写 `.work-tabs`、`.work-tab-shell`、`.work-tab-trigger`、`.work-tab-close`：
   - 40–42 px 轨道；
   - 无完整卡片边框；
   - 活动浅蓝背景 + 2 px 底部线；
   - hover/focus/active 分层；
   - 关闭按钮条件显示；
   - 轨道内部横向滚动。
2. 调整 `main-workspace` 与页签栏的上下边距，保证页面标题与页签之间节奏紧凑但不拥挤。
3. 新增 `.sidebar-workspace-resizer` 及 dragging/focus 样式；视觉线细、命中区足够大。
4. 调整 `.sidebar-expanded-content`、`.sidebar-spacer`、`.sidebar-footer` 的 flex 约束，确保拖动只消耗 spacer 的剩余空间。
5. 检查浅色/深色主题变量，避免硬编码仅适用于浅色背景的颜色。
6. 在 1366×768、1600×900、1920×1080 三种尺寸视觉检查：
   - 顶部不再有厚重浏览器标签感；
   - 拖动柄可发现但不喧宾夺主；
   - Settings/Help 始终可见；
   - 主区和侧栏无页面级横向溢出。

## 阶段 5：自动化验证

1. 扩展 `gui-smoke.mjs`，新增稳定的 data-testid 和断言：
   - `work-tabs` 与活动 tab；
   - `sidebar-workspace-resizer` 存在及 ARIA；
   - 向上/向下调整后面板高度发生预期方向变化；
   - 高度不突破动态边界；
   - 折叠隐藏、展开恢复。
2. 运行：
   - `npm run test:p1`
   - `npm run test:p2`
   - `npm run test:pagination`
   - `npm run test:settings-pagination`
   - `npm run test:qa-settings`
   - `npm run build`
   - `npm run verify`
   - `npm run e2e:gui:strict`
3. 使用 `trellis-check` 复核规范、类型、状态流、测试和跨视口布局。
4. 若发现同类布局问题，先修复后重新执行完整质量门，不以部分通过代替全量验证。

## 阶段 6：Windows 构建、安装与实机验证

1. 结束仍在运行的旧安装版进程，避免安装器文件锁定。
2. 运行 Tauri release bundle，生成新版 NSIS 与 MSI。
3. 使用 NSIS 安装器覆盖安装到当前用户目录。
4. 启动安装版 `app.exe`，验证：
   - 窗口可见且不是后台空壳；
   - 页签新样式生效；
   - 我的空间可上下拖动；
   - 重启后高度恢复；
   - 搜索、设置和目录展开等既有入口仍可用。
5. 对安装版再次执行严格 GUI E2E 或等价安装版冒烟测试。
6. 记录 NSIS、MSI 和安装路径的绝对路径。

## 阶段 7：Git 与 Trellis 收口

1. `git diff --check`，检查空白错误。
2. `git status --short`，逐项核对变更文件。
3. 只暂存本任务源码、测试与 Trellis 任务文件；明确排除两个 Raw 自动检索目录。
4. 创建独立 Git 提交，建议提交信息：`feat(desktop): redesign tabs and resize workspace pane`。
5. 运行 Trellis 完成/归档流程并写开发日志。
6. 最终报告包含：实现摘要、测试结果、安装包路径、提交哈希和仍存在的限制。

## 完成定义

只有同时满足以下条件才可标记完成：

- PRD 全部验收项通过；
- 现有回归测试、构建、严格 GUI E2E 通过；
- 安装版已更新且实机可见；
- Git 已提交且未包含无关 Raw 目录；
- Trellis 任务已归档并记录日志。
