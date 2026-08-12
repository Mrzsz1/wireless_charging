# 无线充电调度研究工作台

面向无线充电调度研究的本地知识库与 Windows 桌面客户端。项目将论文和核心专著编译为可追溯的 Markdown Wiki，并结合 SQLite FTS5、Graphify 知识图和 AI 回答引擎完成文献管理、检索、问答、方法对比与证据核验。

> 当前桌面客户端版本：**0.12.2**  
> 数据原则：以 `wiki/` 正文为知识真相；`graphify-out/` 和本地 SQLite 均为可重建派生数据。

## 主要功能

- **智能问答**：先检索本地 Wiki、论文原文、两本核心专著和 Graphify，再生成带 `[E#]` 定位证据的回答。
- **三种回答引擎**：Codex 订阅、OpenAI-compatible API、仅离线证据。
- **文献库分类**：按论文、方法、综述、概念、系统模型、优化目标、数据实验和知识地图浏览。
- **文献入库**：支持手动添加、待确认候选和自动发现；正式入库执行受控 A 类编译流程。
- **核心专著检索**：按章节检索 *Algorithmic Game Theory* 与 *Approximation Algorithms*，保留书名、章节和 PDF physical page 锚点。
- **研究脉络**：根据当前页面、问题或搜索词展示证据链和相关方法。
- **知识图谱**：使用 Graphify 查询概念关系、路径和节点解释。
- **本地优先**：知识库保存在本机；搜索服务凭据使用 Windows Credential Manager，不写入 Wiki、SQLite 或日志。

## 界面入口

| 入口 | 用途 |
|---|---|
| 工作台 | 查看知识库状态、最近内容与常用操作 |
| 新建问答 / 智能问答 | 提交研究问题、查看证据与历史会话 |
| 文献库 | 分类浏览和搜索 Wiki 页面 |
| 文献入库 | 手动添加、确认候选或运行自动发现 |
| 方法库 | 集中查看 `type: method` 的方法与算法 |
| 核心书籍 | 检索两本核心算法专著的章节内容 |
| 知识图谱 | 浏览 Graphify 派生关系图 |
| 对比 | 对照模型、方法与适用边界 |
| 编译中心 | 查看入库、索引、Graphify 和回滚任务 |
| 设置 | 管理知识库目录、文献自动化、检索服务和回答引擎 |

## 快速开始

### 1. 获取源码

仓库为私有仓库，需要先登录有访问权限的 GitHub 账户：

```powershell
gh auth login
gh repo clone Mrzsz1/wireless_charging
cd wireless_charging
```

### 2. 准备开发环境

- Windows 10/11
- Node.js 20+
- Python 3.10+
- Rust stable toolchain
- Tauri 2 所需的 Windows/WebView2 构建环境
- 可选：Codex CLI、Graphify、MinerU

### 3. 启动开发版

```powershell
cd apps/desktop
npm ci
npm run data:build
npm run tauri dev
```

### 4. 构建 Windows 安装包

```powershell
cd apps/desktop
npm ci
npm run tauri build
```

构建完成后，主要产物位于：

```text
apps/desktop/src-tauri/target/release/app.exe
apps/desktop/src-tauri/target/release/bundle/msi/
apps/desktop/src-tauri/target/release/bundle/nsis/
```

### 5. 首次使用

1. 启动客户端，在“设置”中选择本项目或兼容结构的知识库根目录。
2. 执行“重建索引”，等待 Wiki、论文原文和核心书籍索引完成。
3. 在“设置 → AI 回答引擎”选择 Codex 订阅、兼容 API 或仅离线证据。
4. 在“智能问答”输入研究问题，检查回答中的 `[E#]` 证据和右侧研究脉络。
5. 新论文通过“文献入库”添加；候选只有确认后才进入正式知识库。

## Codex 订阅模式

Codex 模式复用本机 Codex CLI 的官方 ChatGPT 登录状态，不要求 OpenAI API Key：

```powershell
codex login
codex --version
```

客户端只检测安装、版本和登录状态，不读取或显示 token、cookie 或凭据文件。AI 回答前仍会先完成本地证据检索。

## 目录结构

```text
apps/desktop/    React + Tauri Windows 客户端
raw/             原始文献、候选与转换稿；正文只读
wiki/            结构化知识正文与导航索引
schema/          页面类型、frontmatter、词表和编译规则
graphify-out/    可重建的 Graphify 派生图
tools/           入库、索引、搜索、评测与维护脚本
evals/           固定问答与检索质量契约
logs/            知识库维护时间线和运行记录
.trellis/        项目任务、规范和开发工作流
AGENTS.md        Agent 操作边界
prd.md           产品需求与关键决策
HOME.md          Obsidian/知识库导航入口
```

## 文献与知识库工作流

```text
自动发现 / 手动添加
        ↓
raw/inbox 候选与人工确认
        ↓
raw/canonical + MinerU 转换
        ↓
A 类编译：source / method / concept / synthesis 等
        ↓
Lint + 索引 + Graphify 更新
        ↓
智能问答、研究脉络与可定位证据
```

`problem` 和 `idea` 属于 B 类页面，必须经过用户确认；客户端不会因自动发现而静默创建研究结论。

## 常用检查

```powershell
# Python 工具和知识库规则
py -3 -m unittest discover -s tests -p "test_*.py"
py -3 tools/wiki_lint.py --strict-graphify
py -3 tools/wiki_eval.py
py -3 tools/core_book_eval.py

# 桌面端
cd apps/desktop
npm run build
npm run verify
npm run test:p1
npm run test:p2
npm run test:pagination
npm run test:library-categories
```

## 安全与隐私

以下内容不应提交到 Git：

- `.env`、API Key、Token、Cookie、私钥和证书；
- Codex/ChatGPT 用户凭据与用户级配置；
- Windows Credential Manager 中的检索服务密钥；
- 本地 SQLite 数据库、缓存、临时任务和构建产物；
- 原始 PDF 与大体积图片资产（默认由 `.gitignore` 排除）。

仓库即使设为私有，也应继续执行秘密扫描和最小化提交原则。完整维护边界见 [`AGENTS.md`](AGENTS.md)，产品规则见 [`prd.md`](prd.md)。

## 更多文档

- [`HOME.md`](HOME.md)：知识库导航
- [`ARCHITECTURE.md`](ARCHITECTURE.md)：系统架构
- [`apps/desktop/README.md`](apps/desktop/README.md)：桌面端开发和故障诊断
- [`schema/agent-a-compile.md`](schema/agent-a-compile.md)：A 类编译流程
- [`schema/lint-checklist.md`](schema/lint-checklist.md)：Wiki 质量检查
- [`evals/README.md`](evals/README.md)：检索与问答评测

## License

当前仓库未声明开源许可证。未经仓库所有者明确授权，不应复制、分发或公开其中的源码、Wiki 内容及文献转换产物。
