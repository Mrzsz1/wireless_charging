# 多粒度 Embedding 与 pgvector 存储适配

## 范围

- 复用本机 `Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q` 量化模型，不在普通问答中下载模型。
- 对 Markdown Corpus v2 的 document、section 与 semantic ContentBlock 建立 384 维向量。
- 本地以 SQLite `embedding_records_v2` 保存逐块向量；可选把相同记录同步到 PostgreSQL + pgvector。

## 数据与增量契约

- 向量输入由语料索引生成，包含 canonical title、aliases、kind、heading path、role 与对应粒度正文。
- 复用键为 `(blockId, contentHash, modelId)`；未变化内容不会重复推理，变化内容只重算对应块。
- 删除内容转为 inactive；活动记录统一更新到当前 Markdown corpus snapshot。
- 旧 LUNAVEC1 在迁移期间保持只读回退，不自动删除。

## 远程配置

1. 在目标 PostgreSQL/Supabase 项目执行 `apps/desktop/src-tauri/migrations/pgvector_rag.sql`。
2. 设置页填写项目根地址（例如 `https://PROJECT.supabase.co`）和 PostgREST API Key。
3. endpoint 与启用状态写入机器级语义设置；Key 仅进入系统凭据管理器。
4. 设置页执行“构建/同步向量”，查看计算、复用、远程同步数量和进度。

远程存储是可选项。休眠、断网、超时、限流、认证失败或未配置时，本地向量保持可用；问答按远程、SQLite v2、旧语义缓存、词法通道逐级降级。

## 验证

- Rust 全量测试：147/147。
- pgvector fake HTTP：health、stats、upsert、过滤查询、snapshot cleanup 通过。
- 增量计划、取消不写完成标记、维度拒绝、凭据脱敏测试通过。
- 使用已部署本地模型运行跨语言探针，`移动路径规划` 在书籍过滤样本中将 `Euclidean TSP` 排在首位。
- 前端设置契约：7/7；TypeScript/Vite production build 通过。
- `cargo fmt --check` 与 Rust release build 通过。

Raw Markdown/PDF 正文、B 类页面与 `schema/vocab.yaml` 均未修改。
