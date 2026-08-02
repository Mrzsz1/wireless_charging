---
type: "query"
date: "2026-07-14T12:30:09.962051+00:00"
question: "接入 MinerU API，把本地 PDF 自动解析成 Markdown"
contributor: "graphify"
outcome: "useful"
source_nodes: ["MinerU PDF to markdown conversion", "raw/canonical README", "pending_ingest ingested convert_failed status"]
---

# Q: 接入 MinerU API，把本地 PDF 自动解析成 Markdown

## Answer

Expanded from original query via graph vocab: [api, miner, pdf, markdown, raw, canonical, ingest, conversion, files, pipeline, status, outputs]. 图用于确认 MinerU PDF→raw/canonical→pending_ingest→A 编译边界；HTTP 端点和状态机以 MinerU 官方 API 文档核验。已实现 tools/mineru_to_md.py 与 PowerShell 入口，且不自动写 wiki。

## Outcome

- Signal: useful

## Source Nodes

- MinerU PDF to markdown conversion
- raw/canonical README
- pending_ingest ingested convert_failed status