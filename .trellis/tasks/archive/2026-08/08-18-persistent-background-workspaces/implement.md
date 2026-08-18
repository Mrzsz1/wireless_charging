# Implement

1. 调整 QA reduced-motion 覆盖，保留两个忙碌指示器的低速旋转。
2. 在 `App.tsx` 提取问答、文献入库和编译中心的持久挂载工作区。
3. 更新 `styles.css` 保持 QA/编译中心布局契约。
4. 增加静态前端回归，断言长任务页面不再由 `renderContent` 条件卸载。
5. 运行前端全测、构建与 Tauri release 编译。
