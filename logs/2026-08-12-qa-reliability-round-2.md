# 智能问答第二轮可靠性修复与 0.12.3 发布

日期：2026-08-12  
Trellis 任务：`.trellis/tasks/08-12-qa-reliability-round-2/`

## 实现结果

1. 多轮问答在检索前读取同仓库、同会话的 completed 历史，构造可观察的 `RetrievalQuery`；确定性指代补全仅加入受限实体，不复用旧 `[E#]`。
2. 零证据回答使用独立 `unverified` 状态。Codex/API 可输出模型一般知识，但后端强制添加无来源声明并清理伪引用与 wikilink；离线模式只返回确定性提示。unverified 记录可重开和重试，但不进入后续对话上下文。
3. 客户端在提交前生成 request ID 并立即启用停止；后端先登记取消再检索，SQLite/FTS 与 Graphify 进入 blocking worker，Codex 状态使用 30 秒缓存。
4. 首轮和既有会话失败均以相同 request ID 成对保存 user/assistant；前端恢复精确原问题并支持重试，取消与切库不落库。
5. Graphify 搜索覆盖节点描述、来源位置、community、边关系和一跳邻居，并分别支持 relation-only、neighbor-only 命中；候选仍须回链真实且已索引的 Wiki 页面。

## 质量门禁

- 前端 P1：13/13；QA Settings：4/4；TypeScript/Vite build 通过。
- Rust：67/67；QA 定向测试：16/16；`cargo fmt --check`、Clippy `-D warnings` 通过。
- Wiki Lint：75 页，0 errors，1 个既有 warning。
- Wiki/paper Gold Contract：10/10。
- 核心书籍 Recall@5：Algorithmic Game Theory = 1.000000；Approximation Algorithms = 0.986667。
- `verify:p3`、`verify:p5` 和针对真实 0.12.3 release EXE 的严格 GUI E2E 通过。

## 发布与安装

- Release EXE：`apps/desktop/src-tauri/target/release/app.exe`
  - 23,533,056 bytes
  - SHA-256 `FEB7DBAEC9DF7590E55B48CD4D4F39ED7C0A6A591AE92590315DF5CB98912FE5`
- MSI：`apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.12.3_x64_en-US.msi`
  - 12,333,056 bytes
  - SHA-256 `494947E18C33ED222B99B365D86BC9B0216F9DCB1C0E50F575A38E684BB655DC`
- NSIS：`apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.12.3_x64-setup.exe`
  - 8,489,800 bytes
  - SHA-256 `B49C0B2286B7851FDCE30FFD2E0AEFBA506DDDFC7A05D893AC9D9ACE885E52FA`

NSIS 已静默安装到 `C:\Users\qq155\AppData\Local\Wireless Charging Research Workbench\app.exe`。卸载注册表 `DisplayVersion`、安装 EXE 的 ProductVersion/FileVersion 均为 `0.12.3`；隐藏启动探针取得非零主窗口句柄且进程响应正常。

## 治理边界

- 未修改 `raw/`、`wiki/`、`schema/vocab.yaml`、B 类 problem/idea 页面或 Graphify 派生正文。
- 用户交接文档 `智能问答交接文档-2026-08-12.md` 保持未跟踪，未纳入提交。
- 未执行默认外搜，未持久化或回显 API key、Codex token/cookie。
