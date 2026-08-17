# 总体设计

```text
可信会话历史
  -> token 自适应上下文 + 相关旧对话召回
  -> 原始查询 / FTS / 本地向量 / Graphify
  -> 混合融合与重排
  -> Query Plan Facet 覆盖
  -> 必要时 Codex 受控扩展查询
  -> 本轮 EvidenceItem
  -> Codex 结构化回答
  -> Rust 结构/引用/完整性审计
  -> Markdown、证据侧栏与持久化
```

所有新增模型产物均使用独立 JSON Schema。Query Plan、查询扩展和历史压缩不具备事实权威，只有本轮 EvidenceItem 可以支持回答事实。

子任务独立提交和回滚；父任务只拥有跨子任务验收，不直接承载业务代码。
