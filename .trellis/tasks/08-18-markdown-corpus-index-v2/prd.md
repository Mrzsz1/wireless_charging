# Markdown 多粒度语料索引与稳定定位

## Goal

建立统一、可增量重建的 Markdown 语料层，使 Wiki、论文原文和核心专著都能以文档、章节/小节和语义段落粒度被检索，并为后续向量检索、证据附录和原文跳转提供稳定的 `SourceLocator`。

## Requirements

### C1 统一文档模型

- `DocumentRecord` 统一描述 `wiki | paper | book`。
- Markdown 路径为必填正文地址；PDF 和页码字段仅兼容旧数据，允许为空。
- 文档保存 canonical title、aliases、authors、year、tags、provenance、content hash 和 index snapshot。
- 同一 canonical 论文目录中的 `full.md` 与重复导出稿必须确定性选出正文或合并去重，不能当作两份来源。

### C2 Markdown 结构解析

- 解析 YAML frontmatter 和 ATX/Setext 标题树。
- 识别段落、列表、引用、表格、公式和 fenced code，切块不能破坏未闭合结构。
- 每个块保存完整 `headingPath`、父块、序号、行号范围和内容哈希。
- 稳定 block ID 不依赖绝对路径和单独行号。

### C3 多粒度块

- 文档级：标题、别名、作者、摘要/目录和标签。
- 章节/小节级：完整标题路径、角色和小节正文摘要。
- 语义段落级：目标 400–800 tokens、重叠 60–120 tokens；公式/定理/算法步骤/列表尽量原子化。
- 极短小节与父级合并，超长小节继续语义分块。

### C4 内容角色

- 支持父任务列出的 18 个角色。
- 标题规则提供确定性初值；不确定时可标 `general_content`，后续模型标注是增强而非入库前置条件。
- `reference` 块可检索但默认降权，不能与正文模型/方法证据同权。

### C5 别名

- 从 frontmatter aliases、标题、Wiki wikilink/别名和已有书籍 manifest 建立 `document_aliases_v2`。
- 别名归一化保留原文，同时生成大小写、空白和标点归一键。
- 禁止只为“近似算法”写特例；回归必须由通用别名数据通过。

### C6 增量与兼容

- 新表使用版本化 schema；旧表保留双读兼容期。
- 未变 content hash 不重建块；删除/重命名文档将旧块标 inactive 或按 snapshot 清理。
- 索引失败不能删除上一个可用 snapshot。
- 不修改任何 `raw/**/*.md` 正文。

## Acceptance Criteria

- [ ] AC1：真实仓库构建后，Wiki、论文 Markdown、两本专著均存在 DocumentRecord。
- [ ] AC2：`Approximation Algorithms` 有中文别名 `近似算法`，来自可审计元数据而非查询代码特判。
- [ ] AC3：Chapter 3 和 Chapter 11 都产生章节块及多个语义块，embeddingText 包含书名和 heading path。
- [ ] AC4：随机抽取论文的摘要、研究背景/目的、模型、方法、实验和结论能获得合理角色或 `general_content` 降级。
- [ ] AC5：每个 active 内容块的 Markdown 路径位于仓库内，block ID 唯一且重建稳定。
- [ ] AC6：PDF 文件不存在或字段为空时，索引、FTS、详情读取和 locator 测试仍通过。
- [ ] AC7：同一内容连续构建两次，第二次不重写未变块；修改一个 Markdown 只更新该文档。
- [ ] AC8：旧 SQLite 会话表和旧知识索引仍可读取；迁移失败保留旧 snapshot。
- [ ] AC9：Rust 局部测试和 `cargo build --release` 通过。

## Out of Scope

- 生成 embedding 或接入 pgvector。
- 检索规划、RRF、reranker 和多轮补查。
- 最终回答生成和前端证据展示。
