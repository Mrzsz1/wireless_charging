# 技术设计

## 1. 导航表现

- `App.tsx` 停止渲染 `TabBar`，左侧 `activateView` 和内部 `openPage` 继续作为唯一可见导航入口。
- 暂时保留内部 `tabs/activeTab` 持久化结构，兼容已有用户工作区状态、页面滚动 key 和打开资源逻辑；它不再构成 UI。
- GUI E2E 从“标签语义与视觉”改为“标签不存在 + 侧栏直接切换”。

## 2. 研究脉络布局

- `contextOpen=false` 时，`ResearchTrailPanel` 返回一个 `aside.context-collapsed-rail`，参与 flex 布局并固定在最右侧。
- 窄栏宽度约 42px，展开按钮置顶；不使用 absolute 定位，不覆盖主工作区。
- `App.tsx` 在 `view` 切换为 `qa` 时执行一次 `setContextOpen(false)`；用户随后可主动展开。
- 初始工作区就是 `qa` 时，`contextOpen` 初始值直接为 false，避免首帧闪动。

## 3. 英文眉题清理

- 删除所有 TSX 中 `className="eyebrow"` 的文本节点，并删除全局 `.eyebrow` 样式。
- 中文标题、必要的 API/Codex/Graphify 技术术语和数据字段不属于眉题清理范围。

## 4. 兼容与测试

- 不改 Tauri 命令、SQLite、类型和持久化版本。
- 现有单元测试仅运行一次；GUI E2E 验证无标签栏、QA 自动折叠、最右窄栏、展开/刷新/折叠。
- release 和安装版使用同一 GUI E2E 合同。
