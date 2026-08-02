# 2026-08-01 最新文献自动发现、下载与A编译

## 触发

用户要求执行自动获取最新文献功能，并将可取得的最新全文下载到知识库完成编译。

## 发现

- 命令：`paper-search.ps1 --since-year 2025 --top 40 --new-only --download --download-limit 20`
- 来源：arXiv、OpenAlex、Tavily、Google Scholar via SerpApi
- 主题：5
- 原始结果：373
- 去重及年份过滤：269
- 历史去重：已见8，新增269
- 报告：`raw/inbox/auto-discovered/runs/search-20260801-214329/`

## 筛选与全文

选择8项直接相关候选；其中5项取得并校验PDF，3项因出版站点返回HTML/403保持selected：

- Springer priority-driven heap scheduling：未取得PDF
- 2025 WRSN综述：仓储链接403
- IRS-assisted UAV DRL：ACM PDF 403

完成canonical与MinerU的5项：

1. [[../wiki/sources/src-yao2026-ihatrpo-heterogeneous-chargers]]
2. [[../wiki/sources/src-tian2025-diccs-clustering]]
3. [[../wiki/sources/src-liu2026-dchsa-adtsa-clustered]]
4. [[../wiki/sources/src-qaisar2026-isac-uav-charging]]
5. [[../wiki/sources/src-rahaman2023-obstacle-mcv]]

MinerU：5完成 / 0失败；raw frontmatter已由pending_ingest更新为ingested，正文未修改。

## A编译

- 新增source：5
- 新增method：5
- 新增synthesis：`wiki/syntheses/syn-adaptive-mobile-charger-coordination.md`
- 更新maps：online-scheduling、multi-device-wpt、models-and-objectives、domain-keywords
- 更新：`wiki/index.md`、`wiki/maps/library-status.md`
- 未写入problem/idea。

## 水位与验证

- 23 source = 21 papers/preprints + 2 books
- 20/21论文source有Keywords/Index Terms；90次出现，74个原词
- `wiki_eval.py`：10/10
- unittest：31/31

## 待处理队列

- 本轮Top 40：32 pending / 3 selected / 0 rejected / 5 promoted
- 3个selected等待新的合法开放PDF落点后再进入canonical。
