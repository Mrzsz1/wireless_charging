# Wireless Charging Research Workbench 0.10.0

Windows 本地科研工作台：以 Wiki 正文为真相，使用 SQLite FTS5、核心专著章节索引、Graphify 和 Luna 完成阅读、检索、问答与受控编译。

## 环境

- Node.js 20+
- Python 3.10+
- Rust stable 与 Tauri CLI
- Windows GUI E2E 需要 `tauri-driver` 和匹配的 `msedgedriver.exe`

## 开发

```powershell
cd apps/desktop
npm ci
npm run data:build
npm run dev
```

客户端打开本地知识库后，SQLite 只保存可重建派生索引；`wiki/**/*.md`、`raw/` 和 `schema/` 的写入边界遵循根目录 `AGENTS.md` 与 `prd.md`。

## 测试与质量门

```powershell
cd apps/desktop
npm run test:p1
npm run test:p2
npm run test:research-trail
npm run test:ingest
npm run test:installer-lifecycle
npm run build
npm run verify
npm run verify:p3
npm run verify:p4
npm run verify:p5

cd ../..
py -3 -m unittest discover -s tests -p "test_*.py"
py -3 tools/wiki_eval.py
py -3 tools/core_book_eval.py
py -3 tools/wiki_lint.py --strict-graphify
```

`test:p2` 覆盖全局搜索最新请求守卫、旧请求失败和清空查询；`test:research-trail` 覆盖固定证据损坏恢复、仓库/上下文隔离与自动排名去重；Rust 测试覆盖专著片段、研究脉络混合检索、路径边界、回滚补偿和 watcher 批次重试。

## 文献入库

左侧“文献入库”包含三个入口：

- **手动添加**：选择一个或多个 PDF，先查看格式、大小、SHA-256 与重复预检；“确认添加并完整入库”会继续执行 MinerU、A 编译、Lint、Graphify 和本地快照更新。
- **待确认**：查看自动发现清单，可搜索、筛选、排序、写备注、稍后处理、忽略、仅下载或确认添加。“仅下载”不会把论文变成正式 Wiki 证据。
- **自动添加**：默认只检索并准备候选；显式开启“允许自动完整入库”后，才会处理满足全部资格规则的候选，默认阈值 8、单次上限 3。

知识库打开后可显示启动询问，按钮为“本次运行 / 今天不再提醒 / 取消”。“今天不再提醒”只影响当前本地自然日；“立即检索”按钮始终保留。客户端不安装 Windows 服务或计划任务。

正式入库依赖 Python、MinerU、Codex CLI 与 Graphify；缺失能力会在“自动添加”的依赖检查中显示。候选发现和仅下载可在后续编译能力缺失时独立使用。任务日志、退出码、失败原因与生成物统一在“编译中心”查看。

## 设置、搜索服务与分页

0.10.0 起，“启动时询问是否运行”“允许自动完整入库”、相关度阈值、单次上限、起始年份和检索源统一在左侧“设置 → 文献自动化”中管理。“文献入库 → 自动添加”只显示运行状态、资格边界、依赖检查和任务入口，并提供“前往设置”快捷入口。

“设置 → 论文搜索服务”显示 arXiv、OpenAlex、Tavily 与 Google Scholar（SerpApi）：

- arXiv 无需 API Key；其他服务可保存、测试或显式清除 Key。
- Key 保存到当前 Windows 用户的 Credential Manager，不写入 SQLite、知识库、任务参数、manifest 或日志。
- 已保存的 Key 只显示“已安全配置”状态，客户端不回显原值；空输入不会覆盖已有值。
- 受控检索任务启动时才把所需 Key 注入子进程。原环境变量与外部 Key 文件继续作为兼容回退。

文献库、方法库和全局检索结果使用同一分页逻辑：默认每页 10 条，可切换 10/20/50 条；搜索、筛选、排序或页大小变化会回到第一页，结果缩减时当前页自动收敛到有效范围。

## 上下文研究脉络

右侧“研究脉络”跟随当前 Wiki 页面、已提交的研究问题或文献库搜索词切换。证据链融合页面出链/反链、Wiki FTS5、两本核心书籍与 Graphify 一跳关系；每项显示关系、归一化分数与检索理由，“相关方法”只返回 `type: method` 页面。Graphify 或书籍索引缺失时面板显示降级通道，不用目录前几项伪装结果。

“添加证据”会并行搜索本地 Wiki 与核心书籍，并按知识库路径和上下文键保存固定项（`desktop.research-trail.pins.v1`）；不会修改 `wiki/`、`raw/` 或 `schema/`。点击证据可打开 Wiki、书籍章节或聚焦 Graphify 节点。

## 窗口显示恢复

0.7.2 起，窗口位置与尺寸按物理像素保存和恢复，并在启动时与当前显示器工作区求交。移除副屏、修改分辨率/DPI 或任务栏工作区后，完全位于屏幕外的旧窗口会回到主显示器中央；合法的负坐标副屏位置仍会保留。最小化状态不会覆盖最后一个正常窗口矩形，启动恢复结束后会执行取消最小化、显示与聚焦。

若旧版本只在任务栏显示缩略图，直接安装并启动 0.10.0 即会迁移 `desktop.window-state.v2`；无需手工清理本地存储。

## GUI E2E

```powershell
cd apps/desktop
# 可选覆盖；未设置时自动优先发现 release，再发现 debug。
$env:TAURI_APP_PATH="ABSOLUTE_PATH_TO_APP.exe"
$env:TAURI_DRIVER="tauri-driver"
npm run test:e2e-config
npm run e2e:gui:strict
```

GUI E2E 会按 `TAURI_APP_PATH`、构建产物、PATH、`$CARGO_HOME/bin/tauri-driver.exe` 的顺序发现依赖。Windows 下 `tauri-driver` 还需要匹配的 `msedgedriver.exe`；可通过 `TAURI_NATIVE_DRIVER` 指定完整路径。普通 smoke 在缺少依赖时显示可诊断的 `SKIP`，strict 模式以非零状态失败。

## 知识库监听与恢复

目录监听批次在 SQLite 索引事务成功前保持 in-flight。读取或事务失败会保留同一批次并退避重试，达到上限后显示 blocked；可执行“完整重建”清理已覆盖的失败批次。rename 会同时保存旧路径和新路径。

## 编译中心回滚

多文件回滚执行“全量预检 → staging → 文件操作日志 → 逆序补偿 → SQLite 终态落账”。成功后原任务变为 `rolled_back`；补偿成功的失败任务标记 `failed`，补偿仍失败标记 `failed_partial`，并在事件和 `result_json` 中记录受影响路径。临时材料位于受控 `compile-backups/` 目录。

## 核心专著

Algorithmic Game Theory 与 Approximation Algorithms 按章节 Markdown 建索引。章节路径必须位于当前知识库根目录内；检索片段按字符边界生成，避免标题偏移和 Unicode 截断。发布门要求两书 Recall@5 均不低于 95%，physical-page 锚点保持 100%。

## 编译与打包

```powershell
cd apps/desktop
npm run tauri build
```

MSI、NSIS 与 release 可执行文件位于 `src-tauri/target/release/bundle/`。生产更新仍要求 HTTPS endpoint、公钥和签名私钥由构建环境注入；私钥不得进入 Git、应用配置或日志。

## 故障诊断

- **索引失败**：查看界面通知中的 retry/blocked 状态；先解除文件占用，再等待重试或执行完整重建。
- **回滚失败**：在编译中心查看 `failed`/`failed_partial`、失败事件和 result JSON；不要手动删除 staging manifest。
- **章节无法打开**：检查 `raw/canonical/<book-id>/chapter-index.json` 的 `path` 是否为仓库内相对路径，且目标 Markdown 可读。
- **GUI E2E 缺依赖**：运行 `npm run test:e2e-config`，再安装 `tauri-driver` 和匹配的 `msedgedriver.exe`。
- **文献发现失败**：在“自动添加”检查检索来源和依赖；联网或来源 Key 缺失不会改变已有候选。详细错误在编译中心对应 `literature_*` 任务中查看。
- **搜索服务未配置**：打开“设置 → 论文搜索服务”检查状态；OpenAlex、Tavily 和 Google Scholar（SerpApi）可分别保存并测试 Key，arXiv 不需要 Key。
- **文献运行弹出 `py.exe` 或出现 GBK 编码错误**：升级到 0.9.1；该版本将长期任务移出界面线程，隐藏内部 Windows 子进程并固定 Python UTF-8 输出。
- **搜索提示 `wrong number of arguments to function snippet()`**：升级到 0.9.1 后重试；修复不需要删除知识库或 SQLite 索引。
- **手动 PDF 被排除**：查看预检中的格式、200MB 上限和重复路径；只有显式勾选重复覆盖后才能重新处理重复 PDF。
- **窗口只在任务栏**：确认运行的是 0.7.2 或更高版本；严格 GUI E2E 会在导航前断言窗口与当前显示器工作区存在交集。
