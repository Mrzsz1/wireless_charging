# 技术设计

## 1. Toast 边界

新增纯前端 `AppToast` 组件，接收当前 `message` 与 `onDismiss`。组件内部维护 `visible/exiting`、悬停状态与两个计时器。`message` 变化时取消旧计时器、恢复显示状态并启动新的生命周期。

组件放在应用根容器中、`.app-body` 之外，使用 `position: fixed`。右侧偏移通过 CSS 变量/响应式规则保持在窗口边缘内，不依赖研究脉络是否展开，从而避免布局抖动。

## 2. 生命周期

状态：

- `idle/hidden`：无消息，不渲染。
- `visible`：显示并接受悬停、关闭。
- `exiting`：添加退出类；动画完成后调用 `onDismiss`。

自动流程：显示 3600ms → exiting 450ms → 清空当前通知。悬停时清除停留定时器，移出后重新给足停留时间，避免用户来不及阅读。新消息以 `message` 为 key 重置状态。

## 3. 可访问性

- 容器 `role="status"`、`aria-live="polite"`、`aria-atomic="true"`。
- 关闭按钮含 `aria-label="关闭通知"`。
- `prefers-reduced-motion` 下退出动画压缩为近即时，但生命周期保持。

## 4. 右栏控件

保留 `ResearchTrailPanel` 的 `context-collapsed-rail` 结构。将其布局从顶部对齐改为垂直居中；按钮采用窄胶囊、半透明背景和横向微位移反馈。窄栏仍是 `.app-body` 最后一个 flex item，绝不覆盖主内容。

## 5. 自动化

- 静态 verify 检查 Toast 组件、live region、退出类和居中栏样式。
- GUI E2E 触发已有通知，断言其为 fixed、位于右上角且主工作区顶部不变；再断言自动消失。
- GUI E2E 检查折叠栏按钮中心与栏中心的偏差在允许范围内。

