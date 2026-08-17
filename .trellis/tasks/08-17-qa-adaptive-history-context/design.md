# 技术设计

`conversation_history` 读取当前仓库/会话的全部可信配对消息。`build_context_plan` 先将证据压入受控预算，再将剩余动态预算全部作为历史池：从最新完整 exchange 向前连续装载；第一条放不下的 exchange 及更老内容进入 `qa-session-memory-v1`。

```json
{
  "schemaVersion": "qa-session-memory-v1",
  "exchanges": [
    {
      "sourceMessageIds": ["u1", "a1"],
      "userQuestion": "...",
      "trustedAnswerSummary": "..."
    }
  ],
  "truncated": false
}
```

记忆使用确定性截取，不调用 LLM，不增加新事实。`build_prompt_envelope` 将其作为 JSON object 而非双重编码字符串写入 `session_memory_json`。

证据最多占动态预算的 55%；实际证据低于上限时，剩余空间全部归历史。新近原文优先，剩余碎片预算用于结构化记忆。
