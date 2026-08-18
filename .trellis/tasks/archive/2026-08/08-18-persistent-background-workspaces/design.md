# Design

## Persistent workspace ownership

- `App` 始终挂载 `AskView`、`LiteratureIngestView` 和 `CompileCenterView`，不再由 `renderContent()` 条件创建/销毁。
- 每个长任务页面放入 `.persistent-workspace`；当前 view 为 active，其他使用 `hidden` 从布局、焦点与无障碍树移除。
- 页面组件实例不变，因此 Channel 回调、React state、request ID、计时器和日志继续存活。
- 普通短页面仍使用 `renderContent()`，不扩大持久挂载范围。

## Layout

- QA active 包装层承担原 `.qa-view` 的 flex、高度和负 margin 契约。
- Compile active 包装层承担原 `.compile-center-view` 的 flex/min-height 契约。
- Ingest 使用普通 block 布局。

## Loader motion

- 正常环境保持 720ms 线性旋转。
- reduced-motion 下使用 1800ms 低速线性旋转，其他流光、脉冲和光标仍可停止。
