# 桌面端设置集中管理、搜索凭据与分页（0.10.0）

## 范围

- 将文献自动化长期配置从“文献入库 → 自动添加”迁入“设置 → 文献自动化”。
- 在设置中增加 arXiv、OpenAlex、Tavily 与 Google Scholar（SerpApi）状态、保存、清除和连接测试。
- 为 `LibraryView` 的最终筛选结果增加统一分页，覆盖文献库、方法库和搜索结果。

## 安全实现

- OpenAlex、Tavily、SerpApi Key 保存到 Windows Credential Manager；arXiv 无需 Key。
- 前端只接收 `configured` 状态，输入草稿保存后清空，已保存值不回显。
- Rust 使用 provider allowlist；未知 provider 拒绝处理。
- 仅 discovery/literature 任务获得已配置环境变量，其他编译任务不接收搜索凭据。
- 空值不覆盖旧环境变量或外部 Key 文件回退；连接错误不包含带 Key 的 URL、Header 或响应正文。

## 分页契约

- 分页发生在筛选与排序之后，默认 10 条，可切换 10/20/50。
- 查询、页面类型、年份、状态、排序和每页数量变化回到第一页。
- 结果缩减时页码自动限制到最后有效页；空结果继续使用原空状态。

## 验证

- TypeScript：分页 4/4、设置结构 3/3，并通过 P1、P2、research-trail、ingest、构建与 P3/P4/P5 门禁。
- Rust：`cargo fmt --check`、Clippy `-D warnings`、45/45 tests。
- Python/Wiki：45/45 tests、Wiki Eval 10/10；Algorithmic Game Theory Recall@5 1.000，Approximation Algorithms 0.986667。
- GUI：真实 0.10.0 release 在 1366×768、1920×1080 下完成分页、自动页迁移和四个 provider 卡片检查，无主体横向溢出。
- 安装：NSIS 隔离安装、启动、退出、卸载通过。
- Wiki Lint：0 errors，保留 2 个既有 warning；未修改受限 B 页面或派生 Graphify 正文。

## 发布产物

| 产物 | Bytes | SHA-256 |
|---|---:|---|
| `apps/desktop/src-tauri/target/release/app.exe` | 20,796,416 | `FDCDFCD1E1468C44DA1F9097F7CBED0AB8B274BF98048B61B93F03063F2484C9` |
| `apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.10.0_x64_en-US.msi` | 9,998,336 | `98F21B2D338331179606ECE4ACA1C6879E082B4A14C56A6E56F6DBAFC4BB3C84` |
| `apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.10.0_x64-setup.exe` | 6,945,631 | `2A39AA80592D5F10CABA573EBD04AC66620926C323597ECBFAD9BF5ED8DE7205` |

## 数据边界

`raw/inbox/auto-discovered/runs/search-20260809-204315/` 与 `raw/inbox/auto-discovered/runs/search-20260809-211516/` 是用户运行数据，保持未跟踪、未改动、未提交。
