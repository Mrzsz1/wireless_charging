# Design

## Backend

- 新增 `SemanticDownloadProgress` DTO，状态为 `starting | downloading | verifying | complete | skipped | failed`，阶段为 `runtime | model | tokenizer | inference`。
- `repair_deployment_with_progress` 接收进度回调；无回调包装器保留供测试和兼容调用。
- Windows Runtime 下载改为流式 `Read` 循环，在每个 chunk 写入后上报真实累计字节和瞬时平均速度。
- 将 `hf-hub 0.4.3` 提升为直接依赖，使用其同步 `ApiRepo::download_with_progress` 与自定义 `Progress` 实现下载模型必需文件。它继续使用 fastembed 相同缓存结构和 `.part` 续传机制。
- 文件集合固定为 `model_optimized.onnx`、`tokenizer.json`、`config.json`、`special_tokens_map.json`、`tokenizer_config.json`。模型权重归类 model，其余归类 tokenizer。
- 下载完成后 fastembed 仅从本地缓存初始化并执行探针；检查命令仍严格离线。

## Tauri and frontend

- `repair_semantic_model_deployment(onEvent: Channel<SemanticDownloadProgress>)` 在 blocking worker 内将回调事件发送到 Channel。
- desktop service 创建 Channel，并把 `onmessage` 连接到 SettingsView 状态。
- UI 进度区放在 `.semantic-actions` 内主按钮后面，使用 `minmax(220px, 1fr)`，展示阶段文案、文件名、百分比、速度和进度条。
- skipped/complete 事件立即更新；failed 由命令异常映射，保留最后事件的字节数据。

## Safety

- 进度事件不包含 URL、缓存绝对文件路径、远程响应正文或凭据。
- 总字节未知时使用 indeterminate 样式且不伪造百分比。
- 进度回调发送失败不应破坏下载，但命令失败必须正常返回错误。
