# 技术设计：顶部页签重设计与侧栏空间伸缩

## 1. 现状证据

### 页签

- `apps/desktop/src/components/TabBar.tsx` 负责渲染 `work-tabs`、选择按钮和关闭按钮。
- `apps/desktop/src/App.tsx` 持有 `tabs`、`activeTab`，并通过 `selectTab`/`closeTab` 传入 `TabBar`。
- `apps/desktop/src/styles.css` 当前 `.work-tab` 使用完整边框和顶部圆角，活动项模拟浏览器标签。

### 侧栏

- `apps/desktop/src/App.tsx` 将“我的空间”直接渲染为 `.sidebar-expanded-content`，其后是可增长的 `.sidebar-spacer` 和固定 `.sidebar-footer`。
- `.sidebar-expanded-content` 当前只有 `min-height: 0` 和内部滚动，没有显式高度、拖动柄或持久化状态。
- 应用已经通过 `readStored` 和多个 `useEffect` 使用 `localStorage` 持久化侧栏折叠、主题、窗口与工作区状态，可沿用这一错误容忍模式。
- 仓库内没有现成的垂直 Pane Resizer 组件或拖动库，因此使用原生 Pointer Events 最轻量。

## 2. 组件设计

### 2.1 `TabBar.tsx`

保持现有 props 契约不变，避免影响 `App.tsx` 的页签状态流。

DOM 结构调整为：

```text
div.work-tabs[role=tablist]
  div.work-tab-shell
    button.work-tab-trigger[role=tab]
    button.work-tab-close
```

关键点：

- `role="tab"` 放在真正可操作的选择按钮上，而不是包裹两个按钮的 `div`。
- 活动 tab 使用 `aria-selected=true` 和 `tabIndex=0`，其他 tab 使用 `tabIndex=-1`。
- 关闭按钮阻止事件冒泡，避免关闭时先选中。
- 外层轨道 `overflow-x: auto; overflow-y: hidden`，页签壳体 `flex: 0 0 auto`。
- 用伪元素或壳体底边实现 2 px 活动指示条，不改变布局高度。
- 关闭按钮通过 `opacity`/`visibility` 控制；`:hover`、`.active` 和 `:focus-within` 下显示。
- 使用 `scrollbar-width: thin`，并让滚动条保持在轨道内部。

不在本任务引入页签排序或新增状态模型。

### 2.2 `SidebarWorkspacePane.tsx`

新建小型展示/交互组件，接收目录内容作为 `children`，避免继续膨胀 `App.tsx`：

```ts
type SidebarWorkspacePaneProps = {
  collapsed: boolean
  children: ReactNode
}
```

组件内部职责：

- 读取并持有用户期望高度。
- 渲染 `.sidebar-expanded-content` 与 `.sidebar-workspace-resizer`。
- 在 Pointer Down 时读取实际高度、面板顶部、侧栏底部、footer 高度和拖动柄高度，计算本次拖动的动态最大值。
- Pointer Move 时通过 `startHeight + (clientY - startY)` 计算新高度，并调用纯函数夹取。
- 使用 `setPointerCapture`/`releasePointerCapture`，在 Pointer Up/Cancel 清理拖动态。
- 将拖动中的高度同步到内联 CSS 自定义属性或 `style.height`。
- 仅在提交后的有效高度变化时持久化，避免每个像素移动都频繁写存储。
- 双击恢复默认值；键盘操作复用同一夹取函数。

`App.tsx` 只负责把现有标题和 `workspace-tree` 作为 children 传入，不拥有拖动细节。

## 3. 状态与纯函数

新建 `apps/desktop/src/lib/sidebarWorkspaceSize.ts`，避免把边界规则写死在事件回调中：

```ts
export const SIDEBAR_WORKSPACE_SIZE_KEY = 'desktop.sidebar-workspace-height.v1'
export const SIDEBAR_WORKSPACE_DEFAULT = 280
export const SIDEBAR_WORKSPACE_MIN = 132
export const SIDEBAR_WORKSPACE_STEP = 16

export function parseSidebarWorkspaceHeight(value: unknown): number | null
export function clampSidebarWorkspaceHeight(value: number, min: number, max: number): number
export function resizeSidebarWorkspaceByKey(...): number
```

说明：

- 存储值使用带版本的 JSON 对象，例如 `{ "version": 1, "height": 280 }`，便于后续迁移。
- 解析函数拒绝 `NaN`、无穷值、字符串和越界结构。
- 默认高度 280 px；最小高度 132 px；最大高度运行时根据侧栏剩余空间动态得出。
- 浏览器窗口缩小时，CSS 允许面板收缩到当前可用空间；保存的“期望高度”不被窗口临时缩小覆盖，窗口恢复后仍可回到用户设置。

## 4. 动态边界算法

Pointer Down 时计算：

```text
min = min(132, 当前实际可用高度)
max = sidebar.bottom
      - footer.height
      - panel.top
      - resizer.height
      - spacer.minHeight
```

所有值都以 DOM `getBoundingClientRect()` 的 CSS 像素为准，天然适配 Windows 缩放比例。若可用空间小于标准最小值，则以当前可用高度作为临时上下限，保证布局不溢出。

## 5. CSS 布局

- `.sidebar-expanded-content`：`flex: 0 1 auto`、显式 `height`、`min-height: 0`、内部滚动。
- `.sidebar-workspace-resizer`：约 9 px 高，中央显示 32 px × 2 px 的弱提示线，命中区域大于视觉线。
- hover/focus/dragging：提示线和背景切换为天蓝色，`cursor: row-resize`。
- `.app-shell.is-resizing-sidebar` 或组件局部 dragging 类阻止拖动时文本选中。
- `.sidebar-spacer` 继续吸收剩余高度，`.sidebar-footer` 保持固定。
- `.work-tabs` 从浏览器式边框改为单一底部分隔线；活动指示条与现有 `--sky-*` 变量一致。

## 6. 可访问性

- 页签选择按钮使用 `role=tab`、`aria-selected`、roving `tabIndex`。
- 拖动柄使用：
  - `role=separator`
  - `tabIndex=0`
  - `aria-orientation=horizontal`
  - `aria-label=调整我的空间高度`
  - `aria-valuemin`、`aria-valuemax`、`aria-valuenow`
- 键盘：ArrowUp 缩小、ArrowDown 扩大、Home 最小、End 最大。
- 双击恢复默认高度，并用 `title` 提供提示。

## 7. 验证设计

根据用户决定，本任务不新增或删除单元测试。纯函数与交互代码通过 TypeScript 严格编译、现有回归测试和 GUI E2E 联合验证。

### GUI E2E

扩展 `apps/desktop/e2e/gui-smoke.mjs`：

- 验证 `.work-tabs`、活动 tab 与关闭规则存在。
- 获取拖动前高度，通过 Pointer Actions 向上/向下拖动，验证高度变化。
- 验证 `role=separator` 与 ARIA 值。
- 折叠侧栏后验证拖动柄隐藏；重新展开后验证恢复。
- 保留 1366×768 和 1920×1080 无页面级溢出检查。

如果 WebDriver 对原生 Pointer Actions 的支持不稳定，E2E 可在页面上下文派发 `PointerEvent`，但必须真实触发组件事件并验证 DOM 几何变化，不能只修改样式。

## 8. 风险与处理

| 风险 | 处理 |
|---|---|
| 小窗口中固定高度挤压 footer | 最大值基于实际 DOM 剩余空间动态计算，面板允许 Flex 收缩 |
| 拖动离开句柄后中断 | Pointer Capture + Pointer Cancel 清理 |
| localStorage 损坏导致异常 | 纯解析函数拒绝非法值并回退默认 |
| 页签关闭按钮隐藏后键盘不可达 | `:focus-within` 强制显示，保留独立 aria-label |
| CSS 改动影响 QA/编译中心 Flex 页面 | 仅改变页签自身固定高度与边距，并在两类特殊主工作区上回归验证 |
| E2E 在高 DPI 下像素误差 | 使用方向性变化和容差断言，不断言精确设备像素 |

## 9. 影响文件

预计修改：

- `apps/desktop/src/components/TabBar.tsx`
- `apps/desktop/src/components/SidebarWorkspacePane.tsx`（新增）
- `apps/desktop/src/lib/sidebarWorkspaceSize.ts`（新增）
- `apps/desktop/src/App.tsx`
- `apps/desktop/src/styles.css`
- `apps/desktop/e2e/gui-smoke.mjs`
