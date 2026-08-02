# 自动发现文献区

这里只保存检索产生的候选与人工筛选结果，**不是** wiki 证据层。

```text
auto-discovered/
  runs/search-*/        每次检索的 README + results.json 审计快照
  papers/<paper>/       人工 selected 后生成的 metadata.json，可选 paper.pdf
  .paper-search-cache/  远端响应缓存（Git 忽略）
```

状态流：`pending → selected | rejected`；进入 `raw/canonical/` 后才是 `promoted`。自动发现来源必须贯穿 canonical 与 wiki/source。

```powershell
.\tools\paper-search.ps1
.\tools\paper-triage.ps1 ".\raw\inbox\auto-discovered\runs\search-...\results.json" --select 1,3-5
```

筛选命令只写 inbox，不会自动晋升、调用 MinerU 或编译 wiki。

当前水位（2026-07-14）：3 次主题发现共 38 条记录，**14 pending / 3 selected / 14 rejected / 7 promoted**。10 份候选 metadata 均保留；其中 7 份公开 PDF 已下载、校验并完成 MinerU，3 份因没有合法公开全文保持 selected。另有 2 次 exact-title resolution run 用于审计全文来源，不计入 38 条原始候选。
