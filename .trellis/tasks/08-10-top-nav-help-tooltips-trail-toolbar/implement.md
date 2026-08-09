# 实施计划

## 阶段 A：基线与结构

- [x] 记录当前 Git 状态并确认仅有两个无关 raw 未跟踪目录。
- [x] 运行前端 baseline build。
- [x] 新建通用 `DelayedHelp` 组件及局部样式。

## 阶段 B：顶部全局入口

- [x] 重排 `App.tsx` 标题栏：产品名、设置、帮助、拖动区、窗口控制。
- [x] 删除侧栏 footer DOM 与不再使用的图标导入/样式。
- [x] 调整 `SidebarWorkspacePane` 的可用高度测量以兼容无 footer。

## 阶段 C：信息层级

- [x] 将设置页总说明替换为问号帮助提示。
- [x] 将知识库与外观、文献自动化、论文搜索服务、AI 回答引擎、客户端更新五个分区说明替换为问号帮助提示。
- [x] 保留字段级说明、状态和安全信息。

## 阶段 D：研究脉络工具栏

- [x] 刷新控件改为图标+文本胶囊。
- [x] 收起和恢复控件改为 PanelRightClose/PanelRightOpen。
- [x] 完成 hover、active、focus-visible 与窄屏样式。

## 阶段 E：自动化与交付

- [x] 更新现有 GUI smoke 脚本，验证顶部入口、说明延迟和研究脉络折叠/恢复。
- [x] 运行 build、lint/type-check（若项目脚本提供）、现有测试、严格 GUI E2E。
- [x] 构建 Tauri release/NSIS，静默安装，运行安装版严格 GUI E2E。
- [x] 启动安装版并确认窗口响应。
- [x] 更新本计划结果，执行 Trellis 检查，提交 Git；不暂存 raw 未跟踪目录。

## 验证结果

- `npm run build`：通过（包含 TypeScript 检查与 Vite production build）。
- 现有 9 组 Node 回归测试：全部通过，未新增或删除单元测试。
- `npm run verify`：全部检查通过。
- `npm run tauri build`：生成 0.11.0 MSI 与 NSIS。
- release `app.exe` 严格 GUI E2E：通过。
- 安装目录 `app.exe` 严格 GUI E2E：通过。
- 安装版进程 PID 11340：`Responding=True`。
