# Wiki 问答回归集

`gold_questions.json` 固定 10 个真实使用问题，用来防止 wiki 改动后 `/solve`、`/novelty` 和跨文献关系回答退化。

## 用法

只检查题集结构、链接目标和类型配额：

```powershell
py -3 tools/wiki_eval.py
```

若已把 Claudian/LLM 的答案保存为 `evals/answers/<case-id>.md`，可进一步检查每个答案是否包含预期 wikilink、库水位和题集 `must_mention` 必提概念：

```powershell
py -3 tools/wiki_eval.py --answers-dir evals/answers
```

脚本做确定性链接、水位和必提概念契约检查，不替代人工判断答案是否真正理解了方法边界。每次修改核心 synthesis、问答模板或导航结构后运行一次。

当前答案基线与维护者初审见 `evals/answers/REVIEW.md`；切换问答模型（例如 Luna）后应保留旧基线并重新运行评测。
