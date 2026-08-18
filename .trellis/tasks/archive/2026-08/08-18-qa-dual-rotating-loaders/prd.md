# 修复智能问答双旋转加载器

## Goal

将思考面板当前阶段图标和活动步骤图标统一为持续旋转加载器。

## Requirements

- 将当前思考阶段左侧的条形信号改为清晰的圆形旋转加载器。
- 将步骤链中唯一 active 步骤的图标使用同一旋转加载器。
- 动画样式由 QA 组件自身拥有，不依赖全局 `.spin` 类。
- 两处加载器使用同一旋转速度和颜色语义。
- 保留无障碍文案与 `prefers-reduced-motion` 降级。

## Acceptance Criteria

- [x] 主阶段加载器持续转圈。
- [x] active 步骤加载器持续转圈。
- [x] 两者均为装饰性图标，不会额外播报。
- [x] 前端测试和 Tauri release 编译通过。

## Notes

- Keep `prd.md` focused on requirements, constraints, and acceptance criteria.
- Lightweight tasks can remain PRD-only.
- For complex tasks, add `design.md` for technical design and `implement.md` for execution planning before `task.py start`.
