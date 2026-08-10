# 实施计划

## A. 通知组件

- [x] 新建 `AppToast.tsx`，实现显示、停留、悬停暂停、退出和清理计时器。
- [x] 将 `App.tsx` 的内联 `.notice` 替换为根级 Toast。
- [x] 添加右上角定位、进入/退出动画、暗色主题与 reduced-motion 样式。

## B. 右栏控件

- [x] 将 `context-collapsed-rail` 改为纵向居中布局。
- [x] 调整展开按钮尺寸、边界、背景和悬停反馈，使其融入窄栏。
- [x] 保持 42px 固定栏和现有 QA 防遮挡合同。

## C. 自动化与验证

- [x] 更新静态 verify 合同。
- [x] 更新 GUI E2E：Toast 固定定位、自动消失、右栏按钮居中。
- [x] 完成代码后执行一次单元测试集合。
- [x] 执行 build、verify、Tauri release、发布版 GUI E2E。
- [x] 覆盖安装 NSIS，执行安装版 GUI E2E并启动应用。

## D. 收口

- [x] 按 Trellis 质量门检查变更。
- [x] 更新前端规范。
- [x] Git 功能提交、归档任务、记录 journal；排除无关 raw 目录。

