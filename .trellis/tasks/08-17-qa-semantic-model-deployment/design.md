# Design

## 1. Ownership and storage

- 新增本机级 `SemanticModelSettings { cacheDir }`，与知识库级 `LunaSettings` 分离。
- 配置原子写入 Tauri `app_local_data_dir()/semantic-model-settings.json`；空值表示使用 `%LOCALAPPDATA%/LunaWiki/fastembed` 默认目录。
- 应用启动时读取该文件并调用语义模块的缓存目录配置入口。即使未选择知识库，设置页也可读取、修改和检查它。

## 2. Backend contracts

- `get_semantic_model_settings`：返回持久化路径、解析后的有效路径和是否使用默认值。
- `choose_semantic_model_cache_directory`：复用 `rfd::FileDialog::pick_folder`。
- `save_semantic_model_settings`：验证绝对路径、创建目录、执行可写探针、原子保存配置，然后使进程内语义状态失效。
- `check_semantic_model_deployment`：只读、离线检查并返回 `SemanticDeploymentStatus`。
- `repair_semantic_model_deployment`：唯一允许下载 ONNX Runtime/模型的命令；在 blocking worker 中执行，完成后返回新的状态。
- `copy_semantic_model_cache_and_switch`：拒绝相同目录和相互嵌套目录；跳过锁文件，逐文件临时复制并重命名，保留旧目录，成功后保存并切换。
- `open_semantic_model_cache_directory`：以参数方式调用系统文件管理器，不经过 shell。

`SemanticDeploymentStatus` 至少包含：

- `state`: `missing | partial | invalid | ready | error`
- `modelName`、`cacheDir`、`defaultCacheDir`
- `runtimeReady`、`modelFilesReady`、`tokenizerReady`
- `partialDownloadCount`、`totalBytes`、`probeDimension`
- `checkedAt`、`diagnostic`

## 3. Deployment validation

1. 解析并验证缓存目录，但检查操作不创建或下载缺失文件。
2. 检查 `onnxruntime-1.20.1/onnxruntime.dll`。
3. 定位 `models--Qdrant--paraphrase-multilingual-MiniLM-L12-v2-onnx-Q/snapshots/*`。
4. 要求同一 snapshot 中存在 `model_optimized.onnx`、`tokenizer.json`、`config.json`、`special_tokens_map.json`、`tokenizer_config.json`。
5. 任意 `.part` 计入未完成下载；缺文件与 `.part` 映射为 `partial` 或 `missing`。
6. 静态文件完整后才允许调用 fastembed 从缓存加载；执行固定探针文本的单条 embedding，并要求维度为 384 且所有值有限。
7. 加载或探针失败映射为 `invalid`，不得在检查路径自动联网修复。

## 4. Runtime cache switching

- 增加进程级缓存目录覆盖状态。`model_cache_dir()` 始终读取该状态；未覆盖时使用现有默认值。
- 切换目录时等待当前 `SemanticState` 锁，清空 `model`、`corpus` 和 `modelRetryAfter` 后更新路径，保证下一次检索不会继续使用旧目录。
- fastembed 的 `HF_HOME` 与有效缓存目录保持一致，防止环境变量覆盖用户设置。
- 下载/初始化失败继续返回空语义候选，FTS5、Graphify 和中文零候选兜底照常运行。

## 5. UI

- 设置页新增独立“本地语义模型”卡片，不嵌入 Codex/API provider tab，因为 embedding 模型对所有回答引擎共享。
- 页面加载时并行读取设置和离线状态；状态卡展示部署状态、模型、路径、空间、组件检查和最近检查时间。
- 路径编辑区提供“选择目录”“恢复默认”。路径变化后显示“复制现有缓存并切换”和“切换并重新部署”。
- 常驻操作为“检查部署”“下载/修复”“打开目录”。检查和下载使用不同 busy 状态及文案。

## 6. Compatibility and rollback

- 没有配置文件的用户继续使用旧默认目录。
- 旧 E5 `.part` 不属于新模型 snapshot，只计入目录空间，不影响新模型 ready 判定，也不自动删除。
- 配置文件损坏时返回可见错误并保留默认路径作为 UI 建议，不静默覆盖原文件。
- 复制/下载失败不切换配置；已切换但重新部署失败时旧目录仍保留，可手动选回。
