# Design

## State projection

- 将现有条件散落的 `thinkingSteps` 改为一个确定性 `activeStepIndex`：检索、证据整理、模型 Thinking、生成回答、引用校验。
- `retrieving` 对应检索；`generating && !hasFirstToken` 对应 Thinking；`generating && hasFirstToken` 对应生成；`validating` 对应校验。证据整理在 retrieval completed 时直接标为完成，不虚构停留时间。
- 从 active index 派生每一步 `done | active | waiting`，保证互斥。
- 当前阶段标题与说明是固定运营文案，不包含模型内部思维内容。

## Visual loading process

- 在消息元信息和步骤链之间加入 `.qa-thinking-loader`：活动信号、阶段标题、动态三点、说明、耗时与连续流动轨道。
- 活动步骤卡使用低幅脉冲、边框流光和旋转图标；已完成步骤以检查图标稳定展示。
- `hasFirstToken` 后显示 `.qa-stream-cursor`，但继续隐藏未完成 JSON。
- 动画仅使用 transform/opacity，避免布局抖动；窄屏步骤改为单列/双列。

## Accessibility

- 外层 `role=status` 与 `aria-live=polite` 的可读文本只包含真实阶段标题；动画点和轨道 `aria-hidden=true`。
- `prefers-reduced-motion: reduce` 关闭脉冲、流光、旋转和光标闪烁，保留静态状态差异。
