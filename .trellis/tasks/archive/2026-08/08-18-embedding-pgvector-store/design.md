# 多粒度 Embedding 与 pgvector 技术设计

## 1. 当前基线

`qa/semantic.rs` 使用 `Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q`、384 维向量、本地 ONNX runtime 和 LUNAVEC1 文件。当前 corpus 主要以 Wiki 页、paper section 和 book chapter 为一个向量，长内容会截断，查询时加载进内存做余弦遍历。

## 2. 目标模块

```text
qa/embedding.rs          # model lifecycle + batch embed
qa/vector_store.rs       # trait/contracts
qa/vector_store/local.rs # SQLite/blob or versioned local files
qa/vector_store/pg.rs    # PostgreSQL + pgvector adapter
qa/vector_sync.rs        # snapshot/incremental synchronization
```

保留 `qa/semantic.rs` 的部署检查与下载功能，逐步把 corpus/persistence/ranking 移出；避免一次性重写模型下载路径。

## 3. 契约

```rust
VectorRecord {
  repository_id, snapshot_id, block_id, document_id,
  kind, granularity, role, model_id, dimension,
  content_hash, embedding, active
}

VectorQuery {
  embedding, limit, min_score?, kinds?, document_ids?,
  granularities?, roles?, snapshot_id
}

VectorHit { block_id, score, store, model_id }
```

`VectorStore` 的错误分成 configuration、authentication、timeout、rate_limit、unavailable、corrupt 和 cancelled，用户文本不得包含连接串。

## 4. embeddingText

```text
文档：<canonical title>
别名：<aliases>
类型：<paper/book/wiki>
位置：<heading path>
角色：<role>
正文：<content>
```

document 级正文使用摘要/目录/标签，section 级使用完整小节，semantic 级使用语义块。不得对所有粒度机械复制整份长正文。

## 5. 本地存储

优先采用可随机访问、按 block key 更新的版本化存储；可以是 SQLite blob 表或带索引的分片文件。必须能按单条 content hash 复用，不能继续仅以 repository snapshot 作为整个文件键。旧 LUNAVEC1 只读迁移：命中旧 key 时可复用，成功迁移后写 v2，失败则重算。

## 6. 远程 pgvector

远程 schema 由迁移脚本创建 `rag_embeddings`，向量列 dimension 与当前模型一致。查询通过参数化 SQL/RPC 完成 cosine distance，并把 repository/snapshot/filter 放在数据库侧。配置层保存 endpoint/project metadata；敏感令牌使用 OS keyring 或等价安全存储。

远程同步状态写本地 `embedding_records_v2`：pending/synced/failed、last attempt、脱敏错误类别。同步采用有界 batch、指数退避和幂等 upsert。

## 7. 路由

```text
remote configured + healthy -> remote query
remote failed + local ready  -> local query, mark degraded
no remote + local ready      -> local query
neither ready                -> no dense hits, lexical continues
```

不在单次问答里触发全库下载或全量重算。问答只允许短时 lazy embed query；语料 embedding 由索引/修复任务完成。

## 8. 观测与 UI

状态返回模型部署、模型 ID、dimension、各粒度向量数、本地/远程状态、pending sync、last success、last sanitized error。下载和同步使用现有事件总线提供进度。
