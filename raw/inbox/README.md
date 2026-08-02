# raw/inbox

待 triage 的正式文献（仍须是论文/专利等，**禁止**网页/blog/PPT）。本层按**采集来源**分流，不代表文献已经 canonical。

| 入口 | 路径 | 来源字段 |
|------|------|----------|
| 自动发现 | `auto-discovered/` | `acquisition_method: auto_discovery` |
| 手动投放 | `manual-drop/` | `acquisition_method: manual_upload` |

- 自动发现入口：`tools/paper-search.ps1`；筛选：`tools/paper-triage.ps1`。  
- **不**自动 A 编译进 wiki；下载了 PDF 也仍是 inbox。  
- 确认相关后：晋升 `../canonical/`，保留 provenance，MinerU 转 md，再编译。  
- 不相关项标 `rejected`；审计报告不要求删除。  
