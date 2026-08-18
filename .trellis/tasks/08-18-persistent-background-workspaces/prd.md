# 跨页面持续运行问答与文献任务

## Goal

修复减少动态效果下静态加载器，并让智能问答、文献入库和编译任务在功能切换时保持挂载和持续运行。

## Requirements

- 主阶段与 active 步骤的圆形加载器在用户当前环境中必须持续旋转。
- `prefers-reduced-motion` 下降低旋转速度，不再将这两个必要的忙碌指示器完全停止。
- 智能问答、文献入库和编译中心在应用内切换到其他功能时不得卸载。
- 隐藏页面仍保持 Channel、计时器、请求 ID、日志和运行状态，返回后继续呈现同一任务。
- 切换知识库时仍按现有安全边界取消或清理旧知识库任务。
- 隐藏工作区不参与布局、焦点和无障碍树；显示时恢复原有 QA/编译中心高度与滚动契约。

## Acceptance Criteria

- [ ] 两个 QA 圆形忙碌指示器在当前系统设置下持续旋转。
- [ ] 问答运行中切换到文献入库后再返回，任务和进度保留。
- [ ] 文献检索运行中切换到其他页面后再返回，日志和运行状态保留。
- [ ] 编译中心正在运行的任务在页面切换后保持连接。
- [ ] 前端测试、构建和 Tauri release 编译通过。

## Notes

- Keep `prd.md` focused on requirements, constraints, and acceptance criteria.
- Lightweight tasks can remain PRD-only.
- For complex tasks, add `design.md` for technical design and `implement.md` for execution planning before `task.py start`.
