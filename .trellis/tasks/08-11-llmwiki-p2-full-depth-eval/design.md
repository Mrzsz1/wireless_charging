# P2 技术设计

## 1. 分批深化

| 批次 | source | method |
|---|---|---|
| A 放置/安全 | Alzenad、WANDA、ROSE、HIPO | WANDA、ROSE、HIPO；Alzenad 作为理论锚点 |
| B 移动服务 | Wu、Xu、Rahaman、Gao RA-DMCS | tunable-power、CCS、obstacle-MCV、RA-DMCS |
| C AoI/聚类 | Chen、Tian、Liu DCHSA、Binh | Peak-AoI、DICCS、DCHSA/ADTSA、bilevel |
| D DWPT/UAV | Gao FELKH、Qaisar、Honma、Li DWC-BEB | FELKH、ISAC partial、intersection DWPT、integrated DWC-BEB |

每页只从其 `raw_md` 抽取事实；行号由 canonical Markdown 的实际一基行号生成。

## 2. Gold Contract v2

```json
{
  "evidence_contract": {
    "required_kinds": ["wiki", "paper"],
    "paper_sources": ["sources/src-..."],
    "source_location_required": true,
    "critical_constraints": ["..."],
    "boundary_statement_required": true
  }
}
```

- Python 层验证 JSON schema 和静态答案表达。
- Rust 层在真实 SQLite 派生索引上验证运行时证据包。
- `paper_sources` 使用稳定 page ID；`source_location` 必须含章节和 `原文第 x–y 行`。
- Graphify 不进入 required primary evidence；它仍是可选关系提示。

## 3. 编码恢复策略

现有乱码含替换字符，原字节信息已经丢失，因此不做 mojibake 逆转换。依据 case ID、既有 wikilink、
当前 source/method 正文和产品目标重新写入自然中文。写入后增加编码健康检查，拒绝替换字符和典型乱码片段。

## 4. 兼容性

- 保留 case ID、类型配额、`expected_wikilinks`、`must_mention` 和 `waterline_required`，
  旧调用方可继续读取。
- 新字段由新版 Python/Rust 测试强制；运行时 DTO 不增加外部 API 字段。
- 现有问答排序仅在必要时调整词项/证据限额，不改变聊天数据库 schema。

## 5. 回滚

- 评测契约、内容深化和 Graphify 派生物分阶段提交前检查。
- 若某个 source 无法从 raw 核验，保留明确缺失声明，不阻塞其他页面。
- Graphify 失败时 Wiki 为真相，记录缺页，不手编知识正文。

