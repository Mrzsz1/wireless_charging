# Design

1. `chinese_query_fragments` 保持 4/3 字窗口；仅当首轮没有非 Graph 候选时，使用带通用 stop set 的二字窗口追加一次同轮兜底检索，并继续受查询词边界限制。这样修复一字近义差异，同时不污染已有命中的正常排序。
2. fastembed 模型切换为 `ParaphraseMLMiniLML12V2Q`，移除 E5 专用 query/passage 前缀，缓存键同步更名。
3. `SemanticState` 用 `retry_after: Option<Instant>` 替代永久失败 bool；失败后短暂退避，成功清零。
4. `RetrievalQuery` 新增稳定 `plannerStatus`：`not_requested | succeeded | failed_fallback`。将 RetrievalQuery 摘要写入 QaRunManifest，便于审计真实降级原因。
5. 添加精确问题与近义切片测试，不依赖模型下载或外部网络。

旧 E5 `.part` 不由程序自动删除，避免越权清理用户缓存；新模型使用独立缓存目录。
