# 技术设计

在 `AskView` 中将证据展示状态与活动请求状态分离：

```ts
type EvidenceDisplayMode = 'current' | 'previous-during-retrieval'

displayedEvidence       // 最近一次成功 retrieval_completed 的证据
evidenceRequestId       // 这批证据所属 requestId
activeRequestId         // 当前正在运行的 requestId
```

提交问题只切换展示模式，不重置证据。`retrieval_completed` 仅在事件 requestId 等于活动 requestId 时替换证据。仓库 generation 变化、新会话显式创建和打开其他会话时恢复对应会话状态或清空。

不修改后端数据流；隔离标识由前端根据 `phase`、`activeRequestId` 和 `evidenceRequestId` 推导。
