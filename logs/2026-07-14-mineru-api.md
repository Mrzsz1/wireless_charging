# MinerU API 自动解析接入 — 2026-07-14

## 触发

用户提供 MinerU API 官方文档与本机 token 文件路径，要求自动把 PDF 解析为 Markdown。

## 官方协议核对

- 本地文件批量上传：`POST https://mineru.net/api/v4/file-urls/batch`
- 签名地址上传：`PUT <file_url>`，不附加 Authorization / Content-Type
- 批量轮询：`GET https://mineru.net/api/v4/extract-results/batch/{batch_id}`
- 完成结果：`full_zip_url`，其中 `full.md` 是 Markdown 主结果
- 限制：单文件 200MB / 200 页；单次申请最多 50 个上传链接

核对来源：<https://mineru.net/apiManage/docs>

## 新建 / 修改

- 新建：`tools/mineru_to_md.py`
- 新建：`tools/mineru-to-md.ps1`
- 新建：`tools/README-mineru-api.md`
- 新建：`tests/test_mineru_to_md.py`
- 修改：`raw/canonical/README.md`、`使用说明.md`、`.graphifyignore`、`logs/log.md`

## 安全边界

- token 只从环境变量或外部文本文件读取，不写入仓库
- API Authorization 不发送到 OSS 签名上传 URL
- ZIP 解压校验路径穿越、符号链接和解压总量
- 不修改已有 raw 正文；已有 `full.md` 默认跳过
- 不写 `wiki/`、`problem`、`idea` 或 `vocab.yaml`

## 验证

- Python 语法检查
- 5 个无网络单元测试（含 token 不泄漏到签名上传 URL）
- `--help` 与 `--dry-run` 本地流程
- token 文件存在且非空，不输出内容
- 只读鉴权探测返回 `-60012 task not found or expire`：Authorization 被接受，未上传文件、未创建解析任务
