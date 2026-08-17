# Design

1. `chinese_query_fragments` 扩展到 4/3/2 字窗口；对纯问句功能片段应用通用 stop set，并继续受 `QUERY_TERM_LIMIT` 限制。
2. fastembed 模型切换为 `ParaphraseMLMiniLML12V2Q`，移除 E5 专用 query/passage 前缀，缓存键同步更名。
3. `SemanticState` 用 `retry_after: Option<Instant>` 替代永久失败 bool；失败后短暂退避，成功清零。
4. `RetrievalQuery` 新增稳定 `plannerStatus`：`not_requested | succeeded | failed_fallback`。将 RetrievalQuery 摘要写入 QaRunManifest，便于审计真实降级原因。
5. 添加精确问题与近义切片测试，不依赖模型下载或外部网络。

旧 E5 `.part` 不由程序自动删除，避免越权清理用户缓存；新模型使用独立缓存目录。
