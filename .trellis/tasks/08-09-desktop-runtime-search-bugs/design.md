# 文献执行卡死与全局搜索失败：技术设计

## 1. 根因链

### 文献检索

`LiteratureIngestView.run` → `start_literature_run` → `execute_compile_request` → `compile_center::execute_run` → `Command::spawn(py.exe)`。

现有同步 Tauri command 在返回前执行完整网络/编译任务；Windows 子进程继承控制台策略且 Python 继承 GBK 文本编码。任务结束后页面刷新候选，又经 `literature_ingest::run_python().output()` 同步启动 Python，最终在中文 JSON 输出处失败。

### 全局搜索

`search_pages` 直接执行四参数 `snippet()`。FTS5 的 `snippet` 不是可变参数函数，必须传入表、列、高亮起止、截断符和 token 数共六项，因此 SQL 在取结果前失败。

## 2. 进程边界

新增共享 Windows 进程配置模块：

- `configure_background_command(&mut Command)`：Windows 设置 `CREATE_NO_WINDOW`；其他平台为空操作。
- `configure_python_command(&mut Command)`：在后台配置基础上增加 `PYTHONUTF8=1`、`PYTHONIOENCODING=utf-8`。

编译中心、候选 Python 调用、能力探测与 `taskkill` 使用同一后台配置。只有确定为 Python 的任务设置 Python 编码环境；其他可执行程序不注入无关环境。

## 3. 异步执行边界

将 `execute_compile_request` 改为 async：

1. 在当前命令上下文完成仓库路径、数据库和取消令牌登记。
2. 把拥有所有权的 `root`、`db_path`、request、channel、cancel flag 移入 `tauri::async_runtime::spawn_blocking`。
3. 阻塞线程执行既有 `compile_center::execute_run`，其日志线程和取消轮询保持不变。
4. await 后清理取消表并返回最终摘要。

`start_compile_run`、`retry_compile_run`、`start_literature_run` 改为 async 并 await。候选列举、triage 和能力检查先从状态中复制所需值并释放锁，再用 `spawn_blocking` 执行外部命令，避免在持有仓库锁时等待进程。

## 4. 搜索查询边界

从 Tauri State 包装中抽出可测试的 `query_pages(&Connection, &str, usize)`。命令仅获取连接并调用该函数。FTS SQL 改为：

```sql
snippet(pages_fts, 2, '<mark>', '</mark>', ' … ', 24)
```

BM25 和 fallback SQL 不变。内存数据库测试通过现有 `db_schema` 与 `rebuild_connection` 构建真实 FTS 表，避免只断言 SQL 字符串的弱测试。

## 5. Python CLI 编码兜底

`tools/literature_ingest.py` 在 CLI 主入口调用小型 `configure_stdio_utf8()`：仅当 stream 支持 `reconfigure` 时设为 UTF-8，并使用可替换错误策略，兼容测试替换流和非标准终端。Rust 环境变量是主保证，Python 入口为直接运行时的第二道保护。

## 6. 兼容与回滚

- 不修改任务 manifest、数据库 schema、候选状态或 Wiki 内容。
- async 仅移动执行线程，不改变 command 名称、请求/响应 DTO 和前端调用方式。
- 如异步改造出现回归，可单独回退 async 部分；UTF-8、隐藏窗口和搜索 SQL 修复互相独立。
- 用户失败运行目录保持原状，不自动清理。
