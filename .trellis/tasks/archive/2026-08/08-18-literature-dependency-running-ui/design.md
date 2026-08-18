# Design

## Dependency projection

- 在 `codex_subscription` 暴露经版本探测确认的 CLI 路径，`literature_ingest` 和 `compile_center` 均复用该结果。
- 启动 Python 管道时注入 `CODEX_CLI_PATH`；Python 工具优先取该值，再回退到 `PATH`。
- 页面从当前模式派生必需 capability id：`prepare` 为 discovery/download，`automatic` 为 full_ingest。

## Running UI

- 运行按钮使用独立 `.ingest-action-spinner`，避免依赖其他功能 CSS 中的全局 `.spin`。
- 加载器使用不对称圆弧和线性旋转，可直接感知运动。
- 任务条标题提升到 12px，入口 11px，日志 11px/1.65 行高，同时增加内边距。

## Accessibility

- 运行按钮用 `aria-busy`，装饰加载器用 `aria-hidden`。
- 底部运行栏使用 `role=status` 和 `aria-live=polite`。
- reduced-motion 下停止旋转，保留运行文案。
