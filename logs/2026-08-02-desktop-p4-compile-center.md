# Windows 客户端 P4 编译中心与工作区可靠性实施记录

- 日期：2026-08-02
- 版本：0.6.0
- 计划：`design/p4-compile-center-plan.md`

## 完成内容

- 仓库就绪代次驱动文献库、方法库和对比数据重新加载；启动恢复空索引时自动重建。
- “我的空间”目录支持独立展开、真实子导航、鼠标/键盘操作和 ARIA 状态。
- 编译中心接入 Lint、Graphify、论文发现/下载、MinerU 解析和 Codex Agent A 编译。
- SQLite 保存任务、事件、生成物、退出码、失败原因和重试关系。
- 固定命令允许列表、仓库内路径校验、实时日志、取消、重启中断恢复和秘密脱敏。

## 自动验收

| 项目 | 结果 |
|---|---|
| `npm run build` | PASS |
| `npm run verify:p4` | PASS |
| `cargo test` | PASS，16/16 |
| `cargo clippy --all-targets -- -D warnings` | PASS |
| Python 工具链测试 | PASS，31/31 |
| Wiki 固定问题 | PASS，10/10 |
| Algorithmic Game Theory Recall@5 | 1.000000 |
| Approximation Algorithms Recall@5 | 0.986667 |
| Graphify | 1471 nodes / 2225 edges / 130 communities |
| Tauri release | PASS，MSI + NSIS |
| release 启动冒烟 | PASS，稳定运行 8 秒 |

## 交付物

```text
apps/desktop/src-tauri/target/release/app.exe
apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.6.0_x64_en-US.msi
apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.6.0_x64-setup.exe
```

- MSI：6,356,992 bytes；SHA-256 `2FCB6380B6F144076D4D8EC44C465CB8C56EBA056DC0C48E34CEFD83BCC5963B`
- NSIS：4,575,132 bytes；SHA-256 `6A3998AB373ED6E77C8BE563CE33ABBF3472EFE8D22971B3973D8A4E2FED0BD8`

## 边界

- 未修改 Raw 文献正文。
- 未写入 `wiki/problems`、`wiki/ideas` 或 `schema/vocab.yaml`。
- SQLite 与 Graphify 仍为可重建派生层；Wiki 正文保持事实权威。
