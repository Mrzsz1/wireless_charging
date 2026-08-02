# 论文自动发现功能

- 日期：2026-07-14
- 类型：note / tooling
- 用户授权：增加相关领域论文自动搜索，包含 arXiv 等来源

## 目标

为无线充电调度知识库增加受控的文献发现层，在不绕过现有 `raw → wiki` 治理链的前提下自动检索、去重、排序并生成待筛选候选。

## 实现

- 新增 `tools/paper_search.py`：
  - arXiv Atom API；
  - 可选 OpenAlex Works API；
  - 元数据归一化；
  - DOI → arXiv ID → 标题三级去重；
  - 透明词项/时间排序；
  - 24 小时响应缓存与 arXiv 3 秒节流；
  - `--new-only` 历史增量发现；
  - 显式 `--download` 开放 PDF 下载，单文件上限 200MB；
  - 单来源失败不阻断其他来源，错误写入报告。
- 新增 `tools/paper_search_topics.json`：项目默认 5 组检索主题。
- 新增 `tools/paper-search.ps1`：Windows 命令入口。
- 新增 `tools/install-paper-search-task.ps1`：用户主动运行后可安装每日增量搜索任务。
- 新增 `tools/README-paper-search.md` 与测试 `tests/test_paper_search.py`。
- 更新 `raw/inbox/README.md`、`使用说明.md`、`ARCHITECTURE.md` 和 `.gitignore`。

## 治理边界

- 默认输出仅进入 `raw/inbox/search-*/`；
- 报告明确标记 `discovery_status: candidate` 与 `retrieved_at`；
- 不自动晋升 `raw/canonical`，不调用 MinerU，不写 wiki，不更新 Graphify；
- `raw/inbox/**` 继续由 `.graphifyignore` 排除；
- OpenAlex key 不写缓存、报告或日志，也不随 PDF 下载请求发给第三方域名；
- PDF 自动下载默认关闭。

## 验证

- Python 单元测试：11/11 通过（含既有 MinerU 回归测试）；
- `py_compile` 通过；
- 两个 PowerShell 脚本语法解析通过；
- 默认主题 dry-run 通过；
- arXiv 真实联网检索：5 个主题、25 条原始命中、23 条去重且通过年份过滤；
- 生成候选报告：
  - `raw/inbox/search-20260714-204713/`：前 20 条；
  - `raw/inbox/search-20260714-204721/`：`--new-only` 补充 3 条；
- 再次 `--new-only`：缓存命中 5，已见 23、新增 0，未生成空报告；
- 未下载 PDF，未写 canonical/wiki，未更新知识图。

## 官方接口依据

- arXiv API User's Manual：https://info.arxiv.org/help/api/user-manual.html
- arXiv API Basics：https://info.arxiv.org/help/api/basics.html
- OpenAlex Authentication：https://developers.openalex.org/api-reference/authentication
- OpenAlex List Works：https://developers.openalex.org/api-reference/works/list-works
