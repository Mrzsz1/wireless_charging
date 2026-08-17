# Implement — 本地混合向量检索

1. 加入 fastembed 依赖并确认 Multilingual E5 API。
2. 新建 `qa/semantic.rs`：语料读取、快照缓存、嵌入、余弦 top-k、失败降级。
3. 在 `qa.rs` 暴露必要的候选映射边界并接入 `semantic` 融合通道。
4. 补充语义排名、缓存失效和缺表降级测试。
5. 更新后端 QA contract。
6. 运行目标 Rust 测试、完整 Rust 测试与前端构建。
7. Git 提交并归档子任务。
