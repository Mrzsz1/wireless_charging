# 自动发现全文下载与 MinerU 转换 — 2026-07-14

## 触发

用户明确指出 selected 候选应继续下载并调用 MinerU。此前流程错误地停在 triage；本轮补齐 selected → PDF → canonical Markdown。

## 下载结果

10 条首选候选中成功取得 **7 篇合法公开全文**：

1. `arXiv:2505.16482` — Minimizing the energy depletion in WRSNs
2. `arXiv:2409.07994` — Directional WPT Charging for Routing-Asymmetric WRSNs
3. `arXiv:2512.19075` — Optimal 3D Directional WPT Charging via UAV
4. `arXiv:2607.04585` — Infinite Drive
5. `arXiv:2310.00396` — Joint Scheduling and Trajectory Optimization of Charging UAV
6. MIT Senseable City Lab author manuscript — Planning dynamic wireless charging infrastructure for BEB systems
7. Nanjing University author-hosted copy — ROSE: Robustly Safe Charging for WPT

所有文件均通过 `%PDF-` 签名、非零大小和首页/前三页标题核验。MIT 站点的证书链在本机校验失败；仅对已核验的 `senseable.mit.edu` 精确 PDF URL 单次关闭 TLS 证书验证，下载后再次执行 PDF 签名与标题验证。

## 未取得公开全文（3）

| 标题 | DOI | 复核结果 |
|------|-----|----------|
| Spatiotemporal Optimization for Charging Scheduling in WRSNs | `10.1109/JIOT.2023.3294434` | OpenAlex / Semantic Scholar 均标为 closed；未发现作者公开 PDF |
| On Charging Scheduling Optimization for a Wirelessly Charged Electric Bus System | `10.1109/TITS.2017.2740329` | IEEE / PolyU 仅元数据页面；未发现公开 PDF |
| Wireless Charging Scheduling for Long-term Utility Optimization | `10.1145/3708990` | ACM PDF 返回 403；OpenAlex / DBLP / Semantic Scholar 均为 closed |

没有绕过付费墙、机构登录或访问控制。这 3 条保持 `selected`。

## MinerU

- 修复 `paper.pdf` 导致 canonical 目录被命名为 `paper-*` 的问题：现在读取同目录 `metadata.json.title`。
- 修复重复运行时把既有标题目录误判为重名冲突的问题；会复用同一 canonical 目录，并跳过已有 `full.md`。
- 新增测试覆盖真实标题目录、canonical PDF 名称、frontmatter title 和重复运行幂等性。
- MinerU batch：`dfdcd133-4150-44b3-bb32-d30224d30212`。
- 结果：**完成 7 / 失败 0 / 跳过 0**。
- 7 个 `full.md` 均包含：`acquisition_method: auto_discovery`、`triage_status: promoted`、`ingest_status: pending_ingest`。
- 图片资源合计 509 个；每夹保留 canonical PDF、MinerU `_origin.pdf` 与解析 JSON。

## 生命周期水位

- 原始候选 38：14 pending / 3 selected / 14 rejected / 7 promoted。
- 自动发现 canonical：7，全部 pending_ingest。
- 已编译自动 source：0；因此 wiki `source_count` 仍为 9。

## 验收

- `tests.test_mineru_to_md`：9/9。
- `tests.test_paper_triage`：2/2。
- 全量单元测试：26/26；相关脚本 `py_compile` 通过。
- 对 7 篇候选重新执行 dry-run：0 个待解析、7 个因已有 `full.md` 正确跳过，未生成哈希后缀重复目录。
- PDF 与 canonical Markdown：7/7 对应。
- candidate metadata / manifest / canonical frontmatter provenance 一致。
