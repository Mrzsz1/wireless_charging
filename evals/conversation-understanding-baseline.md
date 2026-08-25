# Conversation Understanding 基线

- 状态：PASS
- 冻结矩阵：10 个研究对象 × 5 类 follow-up = **50/50**
- 序数边界：**2/2**
- Standalone entity preservation：**1.000**
- Used history message ID accuracy：**1.000**
- ResearchIntent accuracy：**1.000**
- ExecutionMode accuracy：**1.000**
- Provider failure deterministic fallback：PASS
- 未知历史消息 ID fail closed：PASS

## 覆盖范围

1. 普通指代：`它`；
2. 方法改进：`这个方法还能改进吗`；
3. 来源推导：`这个模型怎么来的`；
4. 方案比较：`它和另一种方案有什么区别`；
5. 解法迁移：`它有没有解法可以迁移`；
6. 序数指代：`第二种方法`、`第三个方案`。

## 运行命令

```powershell
cd apps/desktop/src-tauri
cargo test frozen_follow_up_matrix_covers_fifty_resolution_and_routing_cases --lib
```

本基线只验证问题理解与路由契约，不声明最终回答事实准确率。
