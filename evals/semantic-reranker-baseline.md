# Cross-Encoder Reranker 基线

## 模型无关重排回归

- 状态：PASS
- 用例：10/10
- Recall@5：**0.000 → 1.000**
- MRR：**0.167 → 1.000**
- binary nDCG@10：**0.356 → 1.000**
- Recall@20：**1.000 → 1.000**
- Cross-encoder → embedding → deterministic fallback：PASS
- 实际 provider 版本可区分：PASS
- 候选语义重排上限：80
- 单候选输入上限：1200 字符

该组指标验证加权 RRF 与指标计算，不代表具体 cross-encoder 模型的生产准确率。

## 当前端到端仓库评测

- 状态：PASS
- 用例：13/13
- Document Recall@5/10/20：**0.808 / 0.885 / 1.000**
- MRR / nDCG@10：**0.811 / 0.798**
- Reranker fallback：**12/13**
- 实际 Reranker：**deterministic-research-v2**（cross-encoder 与 embedding 均未部署）
- 平均 reranker latency：**17.5 ms**

当前评测环境未部署本地 cross-encoder 与 embedding 模型，因此有候选的 12 个用例均走
deterministic fallback；零证据用例不运行重排。生产模型实际收益需在本地模型部署后使用同一冻结数据集复测。
