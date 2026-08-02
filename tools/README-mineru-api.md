# MinerU API 自动解析

脚本 `mineru_to_md.py` 使用 MinerU **精准解析 API v4**，将本地 PDF 自动整理成项目要求的“一文一夹”格式：

```text
raw/canonical/<论文名>/
  <论文名>.pdf
  full.md
  images/
  *_content_list.json
  ...
  .mineru-task.json        # 本地任务状态；已被 .gitignore 忽略
```

`full.md` 会自动添加 `ingest_status: pending_ingest` 与 provenance。输入来自 `manual-drop` 时标 `manual_upload`；来自自动发现已选目录且有 `metadata.json` 时，会传播 `auto_discovery`、来源 provider 与 discovery run。脚本只完成 PDF→Markdown，**不会自动写 wiki**；解析后再按 `schema/agent-a-compile.md` 做 A 编译。

自动发现目录中的 PDF 通常统一命名为 `paper.pdf`；脚本会读取同目录 `metadata.json.title` 来命名 canonical 文件夹、PDF 和 frontmatter，避免生成无意义的 `paper-*` 目录。

重复运行是幂等的：若同一来源对应的 canonical 目录已经存在 `full.md`，脚本会报告跳过，不会创建带哈希后缀的重复目录，也不会再次提交 MinerU 任务。

运行环境：Python 3.10+ 与 `requests`。当前机器的 `py -3` 已满足；其他机器缺依赖时运行 `py -3 -m pip install requests`。

## 快速使用

在项目根目录运行：

```powershell
# 先看计划，不调用 API
.\tools\mineru-to-md.ps1 "E:\待解析论文" --dry-run

# 批量解析目录中的 PDF，输出到 raw/canonical
.\tools\mineru-to-md.ps1 "E:\待解析论文"

# 解析手动投放区（自动标 manual_upload）
.\tools\mineru-to-md.ps1 ".\raw\inbox\manual-drop"

# 解析自动发现的已选队列（读取每篇 metadata.json）
.\tools\mineru-to-md.ps1 ".\raw\inbox\auto-discovered\papers"

# 解析单个 PDF
.\tools\mineru-to-md.ps1 "E:\待解析论文\paper.pdf"

# 扫描 raw/canonical，只处理尚无 full.md 的 PDF
.\tools\mineru-to-md.ps1
```

默认从 `E:\知识库\aoikey.txt` 读取 token。也可以设置：

```powershell
$env:MINERU_API_KEY_FILE = "D:\secrets\mineru.txt"
# 或仅在当前终端设置，不写入文件
$env:MINERU_API_KEY = "你的 token"
```

不要把 token 放进命令参数、脚本、Markdown 或日志。

## 常用参数

```powershell
# 中文或中英混排
.\tools\mineru-to-md.ps1 "E:\待解析论文" --language ch

# 扫描件启用 OCR
.\tools\mineru-to-md.ps1 "E:\待解析论文" --ocr

# 只解析指定页
.\tools\mineru-to-md.ps1 "E:\待解析论文\paper.pdf" --page-ranges "1-20"

# 重新解析并覆盖同名 MinerU 结果
.\tools\mineru-to-md.ps1 "E:\待解析论文\paper.pdf" --force

# 不复制外部 PDF，只把结果写入 canonical
.\tools\mineru-to-md.ps1 "E:\待解析论文\paper.pdf" --no-copy-source
```

运行完整参数说明：

```powershell
py -3 .\tools\mineru_to_md.py --help
```

默认配置：`vlm` 模型、英文 `en`、公式和表格识别开启、每批 50 个、每 10 秒轮询、最长等待 1 小时。MinerU 单文件限制为 200MB / 200 页；脚本会自动把超过 50 个文件的目录拆成多个批次。

## 安全与恢复

- Token 只发送到 `https://mineru.net/api/v4/*`，不会随 PUT 请求发送到 OSS 签名上传地址。
- ZIP 在临时目录中下载和校验，拒绝路径穿越、符号链接和超大解压结果。
- `full.md` 最后写入，以其存在作为转换完成标记。
- 每篇目录保存 `.mineru-task.json`，包含 `batch_id`、状态和错误，但不含 token、上传 URL 或结果 URL。
- 已有 `full.md` 默认跳过；网络或解析失败不会修改 wiki。

## API 流程

1. `POST /api/v4/file-urls/batch`：申请本地文件上传地址；
2. 对签名 URL 执行 `PUT`：上传 PDF，不附加 Authorization / Content-Type；
3. `GET /api/v4/extract-results/batch/{batch_id}`：轮询状态；
4. `done` 后下载 `full_zip_url`，解压 `full.md`、图片与结构化 JSON。

官方文档：<https://mineru.net/apiManage/docs>
