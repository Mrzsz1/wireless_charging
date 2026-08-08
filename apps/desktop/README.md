# Wireless Charging Research Workbench 0.7.1

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

`test:p2` 覆盖全局搜索最新请求守卫、旧请求失败和清空查询；Rust 测试覆盖专著片段、路径边界、回滚补偿和 watcher 批次重试。

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
