# Windows 窗口可见性恢复技术设计

## 1. 根因链

```text
innerSize/outerPosition（物理像素）
  → localStorage v2
  → LogicalSize/LogicalPosition（错误地解释为逻辑像素）
  → 高 DPI 二次缩放 / 旧显示器负坐标继续恢复
  → Windows 认为窗口 visible，但实际矩形不与当前屏幕相交
  → 只剩任务栏缩略图
```

并行问题：严格安装 smoke 用超时启动应用，但未建立“启动进程必须退出”的契约，导致临时安装目录中的 `app.exe` 在卸载后继续运行。

## 2. 窗口状态模型

新增纯 TypeScript 模块 `src/lib/windowPlacement.ts`，不依赖 React/Tauri，便于 Node 测试。

```ts
type PhysicalRect = { x: number; y: number; width: number; height: number }
type MonitorWorkArea = PhysicalRect & { primary: boolean }
type StoredWindowState = PhysicalRect & { version: 3; maximized: boolean }
```

核心函数：

- `parseStoredWindowState(value)`：验证数字、尺寸、版本与 maximized。
- `resolveWindowPlacement(state, monitors)`：返回受控物理矩形、是否回退和最大化标志。
- `fitRectToWorkArea(rect, workArea)`：限制尺寸并将整个正常窗口放入工作区。
- `selectIntersectingMonitor(rect, monitors)`：按交集面积选择显示器，支持负坐标。

持久化键改为 `desktop.window-state.v3`；若 v3 不存在，读取 v2 并按其真实来源作为物理像素迁移。v2 只读取一次，后续成功保存写 v3。

## 3. 恢复流程

```text
读取 v3 / 迁移 v2
  → availableMonitors + primaryMonitor
  → 映射 physical workArea
  → resolveWindowPlacement
  → unmaximize（恢复正常矩形前）
  → setSize(PhysicalSize)
  → setPosition(PhysicalPosition)
  → 按状态 maximize
  → 若 minimized 则 unminimize
  → show
  → setFocus
  → 注册 move/resize 监听
```

若显示器 API 或状态解析失败，使用 `center()`，再执行可见性恢复。`tauri.conf.json` 同时设置 `center: true`，避免 React 初始化前出现屏外窗口。

## 4. 保存流程

- 所有坐标通过 `innerSize`/`outerPosition` 读取并按物理像素写入。
- 保留内存中的 `lastNormalState`：
  - 正常状态更新矩形和 `maximized=false`；
  - 最大化状态复用最后正常矩形，仅写 `maximized=true`；
  - 最小化状态跳过写入。
- move/resize 事件只调度一次防抖保存；组件清理时取消定时器并发起最终保存。
- 恢复阶段不注册监听，避免 `setSize/setPosition` 的事件把中间状态写回。

## 5. 权限与兼容

`capabilities/default.json` 增加：

- `core:window:allow-unmaximize`
- `core:window:allow-unminimize`
- `core:window:allow-show`
- `core:window:allow-set-focus`

监视器查询、`isMinimized` 等读取权限由 `core:window:default` 提供。现有窗口按钮行为和 workspace localStorage 不变。

## 6. 安装 smoke 生命周期

`smoke-installer.mjs` 改用可控子进程启动：

1. `spawn(appPath)`；
2. 等待进程进入运行态并确认应用文件存在；
3. 请求终止并在限定时间内等待；
4. Windows 超时后使用 `taskkill /PID /T /F`；
5. 确认进程退出后执行卸载；
6. 检查应用路径消失。

测试目录仍由调用方显式提供，避免触碰正式安装。

## 7. 回滚

- 代码回滚可按独立提交撤销；v3 键与 v2 并存，不破坏旧版本回退。
- 0.7.1 构建产物保留；0.7.2 发布失败时不覆盖其 MSI/NSIS。
- 任何 GUI 相交断言或安装进程清理失败均停止发布。

