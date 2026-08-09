# 设置集中管理与列表分页：技术设计

## 1. 前端边界

新增独立 `SettingsView`，由 `App` 传入仓库、主题、字号、更新器和导航回调。组件内部加载并保存文献自动化配置与搜索凭据状态，避免继续扩大 `App.tsx`。

`LiteratureIngestView` 继续只读加载 `LiteratureIngestSettings`，用于决定按钮文案、运行模式和资格说明；删除本地编辑/保存分支。自动页改为单栏，展示配置摘要和“前往设置”。

## 2. API Key 数据流

```text
SettingsView password input（瞬时 React state）
  → save_search_provider_key(provider, key)
  → Rust provider validation
  → Windows Credential Manager / keyring entry
  → return configured=true（不返回 key）

start literature/discover task
  → compile_center builds trusted TaskSpec
  → search_credentials loads configured entries
  → Command.env(OPENALEX_API_KEY / TAVILY_API_KEY / SERPAPI_API_KEY)
  → Python child inherits environment
  → paper_search existing precedence/env handling
```

服务名固定为应用级常量，用户名使用受控 provider ID。仅允许 `openalex`、`tavily`、`serpapi`，arXiv 无凭据条目。`keyring::Error::NoEntry` 解释为未配置，其他安全存储错误返回不包含秘密的诊断。

`SearchProviderStatus` 只包含 `id`、显示名、说明、`requiresKey`、`configured`。保存命令接收一次性 Key；前端成功后立即清空输入。清除为独立命令，防止空字符串误删。

## 3. 连接测试

测试命令在阻塞线程池执行：arXiv 发最小公开查询；OpenAlex、Tavily、SerpApi 从 Credential Manager 取 Key 后发最小请求。所有请求设置短超时和固定 User-Agent；错误只返回 provider、HTTP 状态或泛化网络原因，不拼接含 Key 的 URL/请求体。

测试不是保存前置条件，避免临时网络故障阻止用户配置；结果仅驻留当前 UI 消息。

## 4. 兼容回退

系统凭据只在存在时覆盖对应子进程环境变量。未保存系统凭据时不写空环境变量，因此既有进程环境变量、`PAPER_SEARCH_KEY_FILE` 和专用 Key 文件继续生效。凭据不写进运行 manifest，重试时重新从安全存储读取当前值。

只有 `discover` 与 `literature_*` TaskSpec 标记为需要搜索凭据；Lint、Graphify、MinerU、Codex 等进程不接收这些秘密。

## 5. 分页模型

新增无副作用分页 helper：

- `normalizePage(page,total,pageSize)`
- `paginate(items,page,pageSize)` 返回 `items/page/pageCount/start/end/total`
- `visiblePageNumbers(page,pageCount,maxButtons=5)`

`LibraryView` 在完成 page type、年份和状态筛选后调用 helper。组件状态为 `page` 与 `pageSize`；依赖 `query/pageType/filters` 的 effect 将 page 重置为 1，另一个效果在结果收缩时收敛页码。结果 key 与排序保持不变。

## 6. 测试与发布

- TypeScript 单元测试覆盖空列表、边界页、最后一页、页大小变化和有限页码窗口。
- Rust 单元测试使用 keyring mock credential builder 或内存存储抽象验证状态、保存/清除和命令环境注入，不访问真实用户凭据。
- GUI E2E 覆盖设置导航、自动设置区域、凭据卡不回显、文献库分页和自动页设置迁移。
- 升级 0.10.0，严格 GUI 使用 release EXE，NSIS 使用隔离安装目录。

## 7. 回滚

凭据模块、SettingsView 与分页 helper 相互独立，可分别回退。SQLite schema 不变；回退客户端不会丢失既有文献设置。Windows Credential Manager 中的条目即使由旧版看不到也不会影响旧环境变量路径，用户可在 0.10.0 设置页显式清除。
