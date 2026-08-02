# raw/canonical

**已确认相关**的正式文献。主链路：**inbox triage → PDF → MinerU →（md + images）→ A 编译**。无论来自自动发现还是手动投放，采集来源都必须保留。

## API 自动解析

项目已提供 `tools/mineru_to_md.py`，可调用 MinerU 精准解析 API，将本地 PDF 自动上传、轮询、下载并整理为一文一夹的 `full.md + images + JSON`：

```powershell
# 从外部目录批量导入到 raw/canonical
.\tools\mineru-to-md.ps1 "E:\待解析论文"

# 只查看计划，不调用 API
.\tools\mineru-to-md.ps1 "E:\待解析论文" --dry-run

# 扫描当前 canonical，只处理尚无 full.md 的 PDF
.\tools\mineru-to-md.ps1
```

默认从 `E:\知识库\aoikey.txt` 读取 token，不会把 token 写进项目或日志。完整说明见 [[../../tools/README-mineru-api|MinerU API 自动解析]]。

## 规则

1. 仅：论文、预印本、专利、技术报告、标准等。  
2. **禁止**：网页、blog、PPT。  
3. **一文一夹**（必须）：避免多篇论文的 `images/` 互相覆盖、路径错乱。  
4. A 编译主输入是 **`.md`**，不是 HTML。  
5. 状态：`pending_ingest` | `ingested` | `convert_failed`。
6. provenance：`acquisition_method` 永久不变；canonical 的 `triage_status` 为 `promoted`。

## 推荐目录（一文一夹）

```text
raw/canonical/
  Zhang2023_OnlineWPT_Scheduling/
    Zhang2023_OnlineWPT_Scheduling.pdf      # 建议保留
    Zhang2023_OnlineWPT_Scheduling.md       # MinerU 输出，编译主源
    images/                                 # MinerU 图片，勿挪出本夹
      xxx.jpg
    Zhang2023_OnlineWPT_Scheduling.html     # 可选，仅人读
```

### 你怎么从 MinerU 收纳

1. 用 MinerU 导出某篇论文（会得到 md + `images/`，有时还有 html）。  
2. 新建文件夹，名与论文 slug 一致（建议英文/拼音+年份，稳定即可）。  
3. 把 **md、images 整个文件夹、可选 pdf/html** 都放进该夹。  
4. 打开 md 看一眼：图片是否为 `![](images/...)` 且能预览。  
5. 通知 agent：编译该夹下的 md。

### 不要这样做

| 做法 | 问题 |
|------|------|
| 所有 md 平铺在 `canonical/`，共用一个 `images/` | 文件名冲突、图张冠李戴 |
| 只留 HTML、丢掉 md | Agent 编译与 wikilink 差；和 wiki 层重复 |
| 把 images 挪到 vault 根或 Obsidian 附件库且不改 md 链接 | 预览全断 |

## HTML 要不要下？

- **可以下**，方便你浏览器里带图通读。  
- **不要**替代 md 作为 ingest 主源。  
- 需要「图+文一体阅读」时：优先在 **Obsidian 打开 md**（相对路径正确时图片会显示）；HTML 当备用。

## 编译时图怎么用

- 默认：从 md **文本**抽问题/方法/结果（够用大部分调度论文）。  
- 图/表关键（架构图、对比曲线）：agent 可打开同目录 `images/*` 或 PDF 对应页。  
- wiki 的 `source` 页一般 **不复制**大图；需要时用相对链接指回 raw，例如：  
  `![](../../raw/canonical/Zhang2023_.../images/fig3.jpg)`  
  （是否链图由编译时决定，非必须。）

## frontmatter 示例（写在 md 顶部）

```yaml
---
title: "..."
year: 2023
source_type: paper
why_relevant: "在线多设备功率分配"
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
ingest_status: pending_ingest
pdf_path: "raw/canonical/Zhang2023_OnlineWPT_Scheduling/Zhang2023_OnlineWPT_Scheduling.pdf"
raw_md: "raw/canonical/Zhang2023_OnlineWPT_Scheduling/Zhang2023_OnlineWPT_Scheduling.md"
---
```
