# 论文自动发现扩展为四源

- 日期：2026-07-14
- 类型：note / tooling
- 用户授权：将已配置的 Tavily、SerpApi、OpenAlex Key 接入论文自动搜索

## 变更

- 默认共享配置文件：`E:\知识库\aoikey.txt`；
- 自动识别标签：`Tavily_api_key`、`SERPAPI_API_KEY`、`openalex_apikey`，同时支持标准环境变量与三个专用 Key 文件参数；
- 默认来源改为：arXiv + 共享配置中所有已找到 Key 的来源；
- OpenAlex 使用 Works Search API；
- Tavily 使用 Search API `basic` 深度，并限制在学术出版/预印本域名白名单；
- SerpApi 使用 `engine=google_scholar`，即 Google Scholar 结构化结果；
- 四源结果继续使用 DOI → arXiv ID → 标题三级去重和统一排序；
- 终端与报告保留各来源命中统计；
- 修复 MinerU 对共享多 Key 文件的兼容：继续读取旧版首个无标签 Token，或优先读取 `MINERU_API_KEY` / `MINERU_TOKEN` 标签。

## 安全边界

- 每个 Key 只发送到对应官方 API 域名；
- Key 不进入缓存键、结果 JSON、Markdown 报告、日志或第三方 PDF 请求；
- Tavily 的普通 Web 搜索能力被学术域名白名单约束；结果仍只是 `raw/inbox` 候选；
- Google Scholar 通过 SerpApi 官方接口，不直接抓取 Scholar HTML；
- PDF 下载默认关闭，canonical/wiki/Graphify 边界不变。

## 验证

- 共享文件中成功加载 3 个论文搜索 Key，未输出其内容；
- 单元与回归测试：15/15 通过；
- Python 编译检查通过；
- 四源真实联合检索无错误：
  - arXiv：0（该次自定义整句检索；默认主题预设此前已验证可返回）；
  - OpenAlex：5；
  - Tavily：5；
  - Google Scholar / SerpApi：5；
- 生成 `raw/inbox/search-20260714-214003/`，共 15 条 candidate、0 个下载 PDF；
- 扫描报告与缓存共 16 个文件，Key 泄漏数为 0。

## 官方接口依据

- Tavily Authentication：https://docs.tavily.com/documentation/api-reference/introduction
- Tavily Search：https://docs.tavily.com/documentation/api-reference/endpoint/search
- SerpApi Google Scholar：https://serpapi.com/google-scholar-api
- OpenAlex Authentication：https://developers.openalex.org/api-reference/authentication
