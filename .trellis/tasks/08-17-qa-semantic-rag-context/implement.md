# 总体实施计划

- [x] 完成证据侧栏轮次隔离。
- [x] 完成 token 自适应历史和分层压缩。
- [x] 完成本地混合向量检索。
- [x] 完成 Query Planner、Facet 覆盖和受控扩展。
- [x] 完成语义 RAG 回归评估。
- [x] 执行跨子任务集成检查和 release 编译。

集成验证：Rust 124/124、前端 Node 56/56、TypeScript/Vite build、Tauri release、MSI 与 NSIS bundle 均通过。

每个子任务开始前独立创建 Git 检查点，完成后运行相关测试并提交。
