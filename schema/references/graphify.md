# 参考：Graphify

- **仓库**：https://github.com/Graphify-Labs/graphify  
- **PyPI 包名**：`graphifyy`（双 y）；CLI 命令：`graphify`  
- **角色**：本库的**可查询知识图层**（非向量库；真图遍历）  
- **对齐日期**：2026-07-10  

> 说明：Graphify 主要是 **AI 编程助手 Skill + CLI**，不是传统「Obsidian 社区插件」。  
> **本库主用 Codex CLI + Grok CLI**（亦可 Cursor / Claude 等）；Obsidian 仍负责浏览 wiki 与 Claudian 问答。  
> 社区在 Karpathy LLM Wiki 讨论中亦将其用于 Markdown 关系导航。

## 它解决什么

| 能力 | 用途（对本库） |
|------|----------------|
| 把文件夹打成 knowledge graph | 在 `wiki/` + `raw/**/*.md` 上建图 |
| `graphify query "..."` | `/solve` `/novelty` 前缩小候选页 |
| `graphify path A B` | 两概念/两方法之间的关联路径 |
| `graphify explain "X"` | 单节点邻居与出处 |
| `graph.html` | 人眼看社区、枢纽、意外连接 |
| 边标签 EXTRACTED / INFERRED | 区分显式链接 vs 推断 |

## 与 Karpathy 三层的关系

```text
raw/ (immutable) ──A 编译──► wiki/ (LLM 维护的正文)
                                │
                                ▼
                         graphify 建图
                                │
                                ▼
                      graphify-out/graph.json
                      （查询索引层，可重建，非第二套正文真相）
```

- **正文真相**永远是 `wiki/` + `raw/`  
- **Graphify 输出**是派生索引：丢了可重建，**不要**手改当知识源  
- **不要**用 `graphify --wiki` 覆盖本库已设计的 `wiki/` 目录结构（本库 wiki 由 A 编译按 schema 生成）

## 安装（本机）

```powershell
# 推荐
uv tool install "graphifyy[pdf,chinese]"
uv tool update-shell   # 若找不到 graphify 命令

# 或
pipx install "graphifyy[pdf,chinese]"

# 本库已做项目级注册（可重复执行）：
#   graphify install --platform codex --project   → .codex/skills/graphify/
#   graphify install --platform agents --project  → .agents/skills/graphify/（Grok 等通用）
#   graphify cursor install                       → 可选，不强制
```

可选：PDF 直接进图用 `[pdf]`；中文查询分词用 `[chinese]`。  
本库 PDF 主路径仍是 **MinerU → md → A 编译**；Graphify 优先扫 **md**。

## 本库推荐入口（按你的工具）

| 你用的工具 | 首次建图 / 更新 | 说明 |
|------------|-----------------|------|
| **Codex CLI** | 在项目根对 Codex 说：`$graphify .` 或「用 graphify 建图」 | **首选**；Codex 用 `$graphify`（不是 `/`） |
| **Grok CLI** | 说「按 `.agents/skills/graphify` 对当前库建图」或「运行 graphify 全量抽取」 | Graphify 官方列表无独立 `grok` 平台；靠 **Agent Skills + `AGENTS.md`** |
| 任意终端 + API key | `graphify extract .` | 需 `OPENAI_API_KEY` / `ANTHROPIC_API_KEY` / `GEMINI_API_KEY` 等 |
| Cursor / Claude | `/graphify .` | 可选，非必须 |

```powershell
# 查询（图建好后，任何终端）
cd E:\知识库\wireless_charging
graphify query "在线调度 功率分配 公平"
graphify path "A" "B"
graphify explain "fairness"
```

输出默认：

```text
graphify-out/
  graph.html
  GRAPH_REPORT.md
  graph.json
```

**已验证**：无 API key 时裸 `graphify extract .` 会失败（md 要语义抽取）。  
请用 **Codex `$graphify .`** 或 **Grok 按 skill 建图**（借用当前 CLI 会话模型）。

## 忽略规则

见仓库根目录 `.graphifyignore`：排除 schema/模板/日志噪声；**纳入** `wiki/` 与 `raw/canonical/**/*.md`（inbox 默认排除）。

## 何时重建图

| 时机 | 动作 |
|------|------|
| 每完成一批 A 编译 | 助手 `/graphify . --update` 或有 key 时 `graphify extract . --update` |
| Lint 发现大量改链后 | 重建 |
| 克隆仓库后无 graph.json | 全量 `/graphify .` |

## 与 Claudian 的分工

| 工具 | 场景 |
|------|------|
| **Claudian** | 日常 `/solve` `/novelty`，读 wiki 正文，Obsidian 内 |
| **Graphify** | Agent/终端先 `query`/`path` 缩小范围；或看 `graph.html` 找枢纽与孤儿 |
| 理想组合 | Graphify 出候选节点 → Claudian/Agent 精读对应 `[[wiki 页]]` → 作答 |

Claudian 若不能直接调 CLI，用户可先在终端跑 `graphify query`，把结果粘贴进对话；Agent 侧应优先 shell 调用。
