# Windows 客户端文献入库 0.9.0

## 范围

- 新增一级导航“文献入库”，包含“手动添加 / 待确认 / 自动添加”。
- 默认自动准备候选；用户显式开启后，才允许满足全部规则的候选自动完整入库。
- 启动询问提供“本次运行 / 今天不再提醒 / 取消”，不安装服务、计划任务或后台定时器。
- “确认添加”执行完整入库；“仅下载”只保存候选 PDF，不产生正式 Wiki 证据。

## 实现

- `tools/literature_ingest.py`：稳定候选 ID、旧清单兼容、资格原因、DOI/arXiv/标题去重、triage、可信运行清单、手动批次二次校验、单篇执行与固定 A 编译范围。
- `tools/paper_search.py`：保存标题/摘要关键词命中与稳定候选 ID。
- Tauri：新增 repository-scoped 自动设置和手动导入 session 表；PDF 头、200MB、SHA-256 与重复预检；新增 9 个受控命令和 5 个 `literature_*` 编译任务。
- React：新增三标签工作区、候选搜索/状态/来源筛选/排序/备注、重复覆盖、依赖状态、实时任务输出与编译中心跳转。
- GUI E2E：覆盖启动询问取消、“文献入库”导航及三个标签。

## 治理边界

- Raw 正文未由本次实施改写，测试只使用临时目录/fixture。
- 自动正式入库只允许 A 类页面；提示词明确禁止 `wiki/problems`、`wiki/ideas`、新 Map、`vocab.yaml` 和删除操作。
- 前端只传 session/candidate ID；外部 PDF 路径与运行清单由 Rust 后端生成，普通编译入口拒绝 `literature_*` 任务。

## 验证

- Python：`tests.test_literature_ingest`、`tests.test_paper_search`、`tests.test_paper_triage` 共 19 项通过；全量 Python 44/44 通过。
- Rust：40/40，通过 `cargo fmt --check`、`cargo clippy --all-targets -- -D warnings`。
- 前端：`test:p1` 8/8、`test:p2` 3/3、`test:research-trail` 3/3、`test:ingest` 4/4、installer lifecycle 2/2；TypeScript/Vite build 与 verify/P3/P4/P5 通过。
- Release GUI：0.9.0 release EXE 在 1366×768 与 1920×1080 通过 strict GUI E2E；启动询问、入库导航和三标签均由真实 WebDriver 操作。
- NSIS：隔离目录静默安装、启动存活、进程退出与卸载通过。
- 两本书评测保持 Algorithmic Game Theory Recall@5=1.000、Approximation Algorithms Recall@5=0.986667。

## 发布产物

| 产物 | 字节 | SHA-256 |
|------|------|---------|
| `apps/desktop/src-tauri/target/release/app.exe` | 20,017,152 | `806C7C48542B55D7E9E4A8652048DE8164999B97C4C343E762DBD5A779FB5F09` |
| `apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.9.0_x64_en-US.msi` | 9,494,528 | `7CFEE878F1B569BD2551FB252A39A6F0881024A9E0F94B7B889F47BC478F9887` |
| `apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.9.0_x64-setup.exe` | 6,666,257 | `88DA4A0FC307AE6FF337A88BF5B12E1EECFA3DF801A46042AD43EE0D0D8B59B0` |
