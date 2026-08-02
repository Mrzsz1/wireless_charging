# 手动投放文献区

把你手动获得的论文/预印本 PDF 放在这里，建议一文一夹。禁止网页、blog、PPT。

默认 provenance：

```yaml
acquisition_method: manual_upload
triage_status: pending
selected_by_user: false
```

确认相关后可直接用 MinerU 解析到 `raw/canonical/`；工具会把它标为 `manual_upload`、`promoted` 与 `pending_ingest`。

```powershell
.\tools\mineru-to-md.ps1 ".\raw\inbox\manual-drop"
```

## 已入库的核心专著

- `PDF_A.pdf` = **Approximation Algorithms**（Vijay V. Vazirani，396 页）
- `PDF_B.pdf` = **Algorithmic Game Theory**（Nisan / Roughgarden / Tardos / Vazirani 编，775 页）
- 两本书已按章节拆分并复制到 `raw/canonical/core-books/`；后续请使用 `tools/core_reference_search.py`，不要再次对整本 PDF 调用 MinerU（接口单文件上限 200 页）。
