# Design — 本地混合向量检索

## Boundary

新增 `qa/semantic.rs`，封装语料读取、快照缓存、本地嵌入和余弦排序。`qa.rs` 只负责把语义候选接入当前多通道融合，不直接管理模型生命周期。

## Model and privacy

- 使用 fastembed-rs 的 `MultilingualE5Small` 本地 ONNX 模型。
- 查询采用 `query: ...`，文档采用 `passage: ...`，与 E5 检索约定一致。
- 模型文件由 fastembed 本地缓存；不调用远程推理 API。
- 首次缺少模型时允许下载模型文件；初始化或推理失败转成通道诊断，不提升为整个问答错误。

## Corpus contract

统一内部 `SemanticDocument`：

- `kind`: wiki / paper / book
- 稳定身份：page_id、section_id 或 chapter_id
- `title`、`body`、来源路径、PDF/行号/页码
- 嵌入文本只取标题、类型/章节名和有界正文，保留足够语义且限制成本

SQLite 表可能因索引阶段不同而缺失，因此逐表检查后加载；单表缺失不影响其他通道。

## Cache and invalidation

进程内缓存键为规范化仓库路径与 `context::index_snapshot_id`。缓存保存文档元数据和归一化向量；快照变化时替换该仓库条目。模型由互斥锁串行访问，避免并发初始化。

## Retrieval and fusion

1. 对当前 resolved question 生成一次查询向量。
2. 与缓存语料做 cosine similarity，筛选有限 top-k。
3. 映射回现有 `Candidate`，保持证据类型和定位。
4. 以独立 `semantic` 通道调用 `extend_fused_channel`，再由现有去重和多样性选择处理。

语义分数不得伪装为事实置信度，只代表召回相关度。

## Failure and rollback

- 锁污染、模型初始化、下载、嵌入或数据读取错误均只让语义通道返回空结果并记录安全诊断。
- 删除模块、依赖和 `retrieve_pass` 接线即可回滚，原 FTS/Graphify 流程保持完整。
