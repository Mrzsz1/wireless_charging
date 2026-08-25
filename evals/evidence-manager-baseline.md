# Evidence Manager 基线

- 状态：PASS
- 稳定去重：PASS
- 单个 primary-paper 文档最多两个候选：PASS
- primary source / theory / direct 权威平分加权：PASS
- 类型与文档多样性：PASS
- semantic block 父上下文扩展：PASS
- evidence token 估算与 ContextPlan 最终预算：PASS
- 遥测：input / deduplicated / selected / document / parent expansion / estimated tokens

## 端到端检索门禁

- 用例：13/13 PASS
- Document Recall@5/10/20：**0.808 / 0.885 / 1.000**
- MRR / nDCG@10：**0.811 / 0.798**
- Locator validity：**1.000**
- Zero-evidence FN/FP：**0 / 0**

Evidence Manager 没有降低冻结检索基线。
