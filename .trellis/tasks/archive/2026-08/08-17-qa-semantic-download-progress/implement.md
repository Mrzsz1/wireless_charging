# Implement

1. 添加 hf-hub 直接依赖、进度 DTO 和可复用进度计算器。
2. 重构 Runtime 与模型文件下载为逐字节/Progress trait 上报。
3. Tauri repair command 接入 Channel，扩展 TypeScript 类型与 service。
4. SettingsView 在按钮右侧渲染实时进度和响应式样式。
5. 添加 Rust 进度计算/缓存跳过测试及前端契约测试。
6. 更新 QA 与组件规范，执行全量测试和 release 编译。
