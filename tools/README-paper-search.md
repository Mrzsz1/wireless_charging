# 论文自动发现工具

`paper_search.py` 从学术接口检索相关论文，归一化元数据、跨来源去重并生成候选报告。它是一个**发现层**：默认只写 `raw/inbox/auto-discovered/runs/search-*/`，不会自动移动到 `raw/canonical/`、调用 MinerU、编译 wiki 或更新 Graphify。

## 1. 快速开始

在项目根目录运行：

```powershell
.\tools\paper-search.ps1
```

如果希望“点击按钮才搜索”，运行一次下面的命令创建桌面快捷方式：

```powershell
.\tools\create-paper-search-shortcut.ps1
```

以后双击桌面的 **“无线充电论文搜索”** 即开始搜索。窗口会显示执行进度和最新候选报告位置，并在完成后保持打开；关闭窗口即可。该按钮默认附加 `--new-only`，只生成未见候选报告，不自动下载、不调用 MinerU、不写入 Wiki，也不会创建定时任务。

默认使用项目主题 `wireless-charging-scheduling`。当前共享 Key 文件可自动启用四个来源：

- arXiv：官方预印本 API，无需 Key；
- OpenAlex：结构化论文元数据；
- Tavily：仅搜索配置好的学术出版/预印本域名白名单；
- Google Scholar：通过 SerpApi 的 `google_scholar` 引擎访问。

每次成功运行产生：

```text
raw/inbox/auto-discovered/runs/search-YYYYMMDD-HHMMSS/
  README.md       # 适合人工阅读的候选报告
  results.json    # 完整结构化元数据
```

报告带抓取时间、来源、DOI/arXiv ID、开放获取状态、匹配原因、逐条 `triage_status` 和边界声明。响应缓存保存在 `raw/inbox/auto-discovered/.paper-search-cache/`，同一请求默认 24 小时内不重复访问远端。

## 2. 常用命令

自定义检索词并保留默认主题：

```powershell
.\tools\paper-search.ps1 --query "age of information wireless charging scheduling"
```

只用自定义检索词：

```powershell
.\tools\paper-search.ps1 --no-preset --query "inductive charging task scheduling"
```

只显示检索计划，不联网：

```powershell
.\tools\paper-search.ps1 --dry-run
```

执行真实检索但不生成报告：

```powershell
.\tools\paper-search.ps1 --no-save --limit 5 --top 10
```

周期运行时只报告历史报告中未出现的候选：

```powershell
.\tools\paper-search.ps1 --new-only
```

显式下载排名靠前且来源给出开放 PDF URL 的文献：

```powershell
.\tools\paper-search.ps1 --download --download-limit 10
```

PDF 会进入该次报告的 `papers/<排名>-<标题>/`。脚本限制单个 PDF 最大 200MB，并检查文件类型。下载时不会把 OpenAlex key 发给第三方 PDF 地址。

## 3. 人工筛选候选

候选序号来自报告标题。选择、拒绝或恢复待筛选状态：

```powershell
.\tools\paper-triage.ps1 ".\raw\inbox\auto-discovered\runs\search-...\results.json" --select 1,3-5
.\tools\paper-triage.ps1 ".\raw\inbox\auto-discovered\runs\search-...\results.json" --reject 2,6 --note "偏离调度主题"
.\tools\paper-triage.ps1 ".\raw\inbox\auto-discovered\runs\search-...\results.json" --pending 6
```

`selected` 项会在 `raw/inbox/auto-discovered/papers/<paper>/metadata.json` 建立稳定队列；加 `--download-selected` 才下载开放 PDF。该命令仍不会晋升 canonical。

当 PDF 已经校验并由 MinerU 写入 canonical 后，用 `--promote 1,3-5` 同步 manifest 与候选 sidecar 状态。`promoted` 仍不等于已完成 A 编译。

## 4. 搜索源与 Key

默认共享配置文件为 `E:\知识库\aoikey.txt`。脚本只识别带标签的论文搜索 Key，不会打印或写回 Key：

```text
Tavily_api_key=...
SERPAPI_API_KEY=...
openalex_apikey=...
```

旧版第一行 MinerU Token 仍然兼容；三个搜索 Key 不会被发送给 MinerU。也可以使用标准环境变量：


```powershell
$env:TAVILY_API_KEY = "你的 Tavily key"
$env:SERPAPI_API_KEY = "你的 SerpApi key"
$env:OPENALEX_API_KEY = "你的 OpenAlex key"
.\tools\paper-search.ps1
```

也可以分别使用专用文件：

```powershell
.\tools\paper-search.ps1 `
  --openalex-key-file "E:\keys\openalex.txt" `
  --tavily-key-file "E:\keys\tavily.txt" `
  --serpapi-key-file "E:\keys\serpapi.txt"
```

未显式指定来源时，程序启用 arXiv 和所有已找到 Key 的来源。也可手工选择：

```powershell
.\tools\paper-search.ps1 --provider arxiv --provider openalex --provider tavily --provider serpapi
```

OpenAlex Key 只发往 `api.openalex.org`，Tavily Key 只作为 Bearer Token 发往 `api.tavily.com`，SerpApi Key 只发往 `serpapi.com`。所有 Key 都从缓存标识、报告、日志和第三方 PDF 下载请求中排除。

## 5. 每日自动运行（可选，不适用于“点击才搜索”模式）

以下命令会创建一个 Windows 当前用户定时任务，每天 09:00 运行，并只输出未见候选：

```powershell
.\tools\install-paper-search-task.ps1
```

自定义时间：

```powershell
.\tools\install-paper-search-task.ps1 -DailyAt "20:30"
```

安装脚本只有在你主动运行时才会修改 Windows 任务计划。删除任务：

```powershell
Unregister-ScheduledTask -TaskName "WirelessChargingPaperSearch" -Confirm:$false
```

如果你希望严格保持“点击按钮才搜索”，不要运行本节的安装命令；桌面按钮本身不会安装或触发计划任务。

定时任务默认读取共享 Key 文件，因此当前会自动使用全部四个来源。

## 6. 主题预设

预设位于 `tools/paper_search_topics.json`。当前默认主题覆盖：

- wireless power transfer scheduling；
- wireless rechargeable sensor networks；
- mobile charger scheduling；
- dynamic wireless charging；
- RF energy harvesting scheduling / AoI。

每个主题分别保存 arXiv 查询语法与通用自然语言检索词；后者供 OpenAlex、Tavily 和 Google Scholar 使用。修改或增加预设时应保持 JSON 合法，并先执行 `--dry-run`。

## 7. 治理边界

```text
arXiv / OpenAlex / Tavily / Google Scholar via SerpApi
  → raw/inbox/auto-discovered/runs/search-* 候选报告
  → paper-triage 记录 pending / selected / rejected
  → selected 元数据进入 auto-discovered/papers
  → raw/canonical/<paper>/
  → MinerU Markdown
  → A 编译
  → wiki + Graphify 更新
```

- 自动发现结果不是 canonical，也不是“全球查新”结论；
- `raw/inbox/**` 已从 Graphify 输入中排除；
- 只有人工确认相关的正式论文/预印本才能晋升；
- 外部摘要和元数据需回到论文原文复核后才能形成 wiki claim；
- 自动下载默认关闭，避免无选择地堆积 PDF。

## 8. 参数速查

```powershell
.\tools\paper-search.ps1 --help
```

重要参数：`--preset`、`--no-preset`、`--query`、`--provider`、`--key-file`、`--since-year`、`--limit`、`--top`、`--new-only`、`--download`、`--no-save`、`--output-root`、`--cache-root` 与 `--cache-hours`。

## 9. 接口约束

- arXiv 返回 Atom 1.0；脚本在连续远端请求之间至少等待 3 秒，并默认缓存 24 小时；
- OpenAlex 每页最多取 100 条，处理限流响应并使用退避重试；
- Tavily 使用 `basic` 深度，每主题 1 credit，最多返回 20 条，并限制在学术域名白名单；
- Google Scholar 通过 SerpApi 获取结构化 `organic_results`，每主题最多返回 20 条，并使用 `as_ylo` 做年份下限过滤；
- 任一来源失败时，其他来源仍可继续，告警会写入报告；
- 当前排序是透明的词项与时间启发式，不等价于论文质量、引用影响或严格语义相关度。
