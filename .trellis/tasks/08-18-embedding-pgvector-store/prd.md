# 多粒度 Embedding 与 pgvector 存储适配

## Goal

基于统一 ContentBlock 构建文档、章节/小节和语义段落 embedding，提供可配置 PostgreSQL + pgvector 存储以及可靠的本地离线降级，替换当前“整章/整页单向量 + 仓库级 LUNAVEC1 文件”的局限。

## Requirements

### E1 模型生命周期

- 继续复用已部署的本地多语言 MiniLM 模型及自定义缓存目录。
- 模型只在部署/修复或首次需要时下载，不能每轮对话下载。
- 部署检查必须验证 ONNX runtime、模型、tokenizer 和真实推理探针。

### E2 多粒度输入

- embedding 输入包含 canonical title、aliases、heading path、role 和正文。
- 对 document、section/subsection、semantic block 分别生成向量。
- 向量记录绑定 block ID、content hash、model ID、dimension 和 index version。

### E3 增量计算

- 未变 `(blockId, contentHash, modelId)` 复用向量。
- 批量计算，支持取消和进度；单块失败不破坏已有可用向量。
- 删除或 inactive block 从 active 查询集合排除。

### E4 VectorStore 抽象

- 能力包括 health、stats、upsert batch、query、delete snapshot 和 close/cancel。
- 本地和远程返回同一 `VectorHit` 契约。
- 检索层不知道具体存储供应商。

### E5 PostgreSQL + pgvector

- 远程表按 repository ID、snapshot ID 和模型隔离。
- 相似度查询支持 document kind/document IDs/granularity/role 过滤。
- 凭据不写日志、审计包或普通 SQLite；连接失败脱敏。
- 免费实例休眠、超时、限流或断网时 fail-soft。

### E6 本地降级

- 无远程配置时可使用本地块级向量缓存。
- 远程失败时优先查询本地；本地不可用时返回明确 degraded 状态，由词法通道继续。
- 设置页显示模型部署、向量数、待同步数、当前存储和最后同步错误。

## Acceptance Criteria

- [x] AC1：三种粒度的向量均可按 block ID 检查，向量维度与模型一致。
- [x] AC2：第二次索引未变语料时 embedding 计算数为 0；单文件修改只重算对应块。
- [x] AC3：查询 `移动路径规划` 可在书籍过滤下返回 TSP/Euclidean TSP 相关块。
- [x] AC4：远程 pgvector 可完成 upsert、filtered query 和 snapshot cleanup。
- [x] AC5：远程超时/断网后自动走本地或 lexical-only，问答不报“知识库没有来源”这种错误结论。
- [x] AC6：取消同步不会留下标记为 complete 的半成 snapshot。
- [x] AC7：日志、错误、SQLite 和导出的审计中不含远程密钥。
- [x] AC8：设置页部署检查与下载进度不回归。
- [x] AC9：Rust 测试、前端设置测试、前端 build 和 release cargo build 通过。

## Out of Scope

- RetrievalContract、RRF、reranker 和 Agentic 补查。
- 回答提示词和证据 UI。
- 强制用户配置远程数据库。
