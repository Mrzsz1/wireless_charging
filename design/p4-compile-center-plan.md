# P4 编译中心与工作区可靠性实施计划

> 目标版本：Windows 客户端 `0.6.0`  
> 权威范围：`prd.md` §12、§13.7、§13.8  
> 执行原则：先修复阻塞性导航缺陷，再接入已有发现、下载、解析、A 编译、Lint 与 Graphify 工具链。

## 1. 阶段目标

P4 建立“选择知识库 → 运行受控任务 → 实时观察日志 → 查看结果 → 失败重试或安全回滚”的本地编译闭环。客户端只编排仓库内已经存在、可审计的工具，不在前端拼接任意 Shell 命令，也不改变 Raw / Wiki / Schema / Graph 的治理边界。

本阶段同时修复两个阻塞缺陷：

1. 选择知识库后进入“文献库”，页面列表仍提示“请先选择知识库目录”；
2. 左侧“我的空间”下的车路协同、调度算法、实验数据无法点击展开。

## 2. 非目标与硬边界

1. 不提供任意命令终端；所有任务必须来自后端允许列表。
2. 不把网页、博客或 PPT 直接编译为 Wiki 来源。
3. 不修改 `raw/` 文献正文；仅由既有流程更新允许的状态元数据和派生输出。
4. 不绕过 `wiki/problems`、`wiki/ideas` 的用户确认闸门。
5. 不用 Graphify `--wiki` 覆盖本库 `wiki/`。
6. 不在 SQLite、前端状态、任务日志或错误消息中保存 API Token。
7. 回滚不执行模糊递归删除；只处理任务清单中明确记录、且仍位于知识库根目录内的本次新建派生文件。

## 3. P4-00：阻塞缺陷修复

### 3.1 仓库选择与文献库加载

统一仓库生命周期：

```text
未选择
→ 正在选择
→ 仓库验证
→ 保存 repositoryPath
→ 读取/建立索引
→ 发布 repository-ready(repositoryPath, generation)
→ 当前视图按 generation 重新加载
```

实施要求：

- `repositoryPath` 只有一个权威状态源；不得在组件内复制一份不同步的路径。
- 选择成功后，即使当前已经位于文献库，也必须重新执行列表加载。
- 页面列表请求显式携带当前仓库代次或依赖仓库就绪状态，忽略旧仓库的迟到响应。
- 错误状态区分“未选择仓库”“索引失败”“列表加载失败”，每种状态提供可执行入口。
- 切换仓库后清理失效页面标签、筛选器结果和旧证据，不清理属于其他仓库的持久化会话。

### 3.2 “我的空间”树展开

树节点契约：

```ts
type WorkspaceNode = {
  id: string;
  label: string;
  kind: "folder" | "shortcut";
  children?: WorkspaceNode[];
  target?: NavigationTarget;
};
```

实施要求：

- 文件夹行整行可点击展开/收起，箭头按钮同步状态；收藏按钮不触发展开。
- 展开状态按稳定 `node.id` 存储，多个节点可以同时展开。
- 子节点缩进展示；空文件夹显示“暂无内容”，不假装加载失败。
- `Enter`/`Space` 切换，`aria-expanded`、`aria-controls` 和按钮名称完整。
- 点击子快捷方式执行真实导航；没有目标的数据节点仍可展开，不吞掉事件。
- 侧栏滚动条不得覆盖箭头或点击区域。

## 4. 编译中心信息架构

### 4.1 任务目录

| 任务类型 | 目的 | 既有能力入口 | 联网 |
|---|---|---|---|
| `discover` | 获取最新候选文献与来源追踪 | 仓库既有发现脚本/配置 | 可选 |
| `download` | 下载已选候选 PDF | 既有下载流程 | 是 |
| `parse` | PDF → 规范 Markdown/资源 | MinerU 工具链 | 是或本地 |
| `compile_a` | Raw → Wiki A 类页面 | Agent A 编译规程 | 否 |
| `lint` | frontmatter、链接、词表与边界检查 | lint 清单/脚本 | 否 |
| `graphify_update` | 更新派生知识图 | `graphify update . --force` | 否 |
| `full_pipeline` | 按依赖顺序串行执行以上受控阶段 | 后端编排器 | 视子任务 |

任务不可用时必须显示缺失依赖、缺失配置或前置条件；不得点击后静默失败。

### 4.2 页面布局

```text
左列：任务目录、预设流程、依赖状态
中列：任务运行列表、实时日志、阶段进度、停止/重试
右列：输入参数、结果摘要、生成物、诊断、回滚入口
```

状态统一为：`queued | running | succeeded | failed | cancelled | interrupted | rolled_back`。

## 5. 数据契约与持久化

SQLite 新增：

### `compile_runs`

- `id`：UUID
- `repository_path`：规范化仓库根
- `task_kind`、`display_name`
- `status`、`current_stage`
- `created_at`、`started_at`、`finished_at`
- `exit_code`、`failure_reason`
- `parameters_json`：脱敏后的结构化参数
- `result_json`：计数、报告路径、结果摘要
- `retry_of`、`rollback_of`

### `compile_run_events`

- `id`、`run_id`、`sequence`
- `event_kind`、`stage`
- `message`、`created_at`

### `compile_artifacts`

- `id`、`run_id`
- `artifact_kind`
- `relative_path`
- `operation`：`created | modified | derived`
- `before_hash`、`after_hash`
- `rollback_eligible`

索引按 `repository_path/status/created_at` 建立。知识索引重建不得删除任务历史。

## 6. Tauri 命令与事件

命令：

- `get_compile_capabilities(repositoryPath)`
- `list_compile_runs(repositoryPath, filters)`
- `get_compile_run(repositoryPath, runId)`
- `start_compile_run(repositoryPath, request, channel)`
- `cancel_compile_run(repositoryPath, runId)`
- `retry_compile_run(repositoryPath, runId, channel)`
- `rollback_compile_run(repositoryPath, runId, channel)`
- `open_compile_artifact(repositoryPath, artifactId)`

流式事件：

```text
accepted
stage_started
stdout
stderr
progress
artifact
stage_completed
completed
failed
cancelled
rollback_started
rollback_completed
```

每个事件携带 `runId`、单调递增 `sequence`、`stage` 与时间戳。前端按 `sequence` 去重和排序。

## 7. 后端执行器

1. 使用任务枚举映射到固定可执行文件和固定参数模板，不接受前端传入命令字符串。
2. 路径全部规范化并验证位于当前知识库根目录内。
3. 子进程分别读取 stdout/stderr，按行推送并写入 SQLite；日志设长度上限和批量刷新。
4. Token、Authorization、API Key、签名 URL 等按键名和常见格式双重脱敏。
5. 同一仓库默认只允许一个写任务；Lint 可与只读能力并发，Graphify 与 A 编译互斥。
6. 取消先发送正常终止，再按超时升级；最终状态和退出码必须落库。
7. 客户端重启时将遗留 `running` 标记为 `interrupted`，允许显式重试。
8. 重试创建新运行并保留 `retry_of`，不覆盖旧日志。

## 8. 安全回滚

1. 任务开始前生成变更基线：相对路径、大小、mtime、必要时哈希。
2. 成功后计算变更清单并标注是否可回滚。
3. 回滚前再次校验路径、当前哈希和仓库边界；文件已被用户再次修改时跳过并报告冲突。
4. 自动回滚只删除本次明确新建的派生文件，或从本次专属备份恢复明确修改的允许文件。
5. `raw/` PDF、Raw 正文、B 类页面和 Schema 文件默认不进入自动回滚。
6. 回滚本身作为新任务保存完整日志与结果。

## 9. 前端行为

1. “编译中心”从占位页升级为真实工作区，并复用现有标签栏。
2. 首页展示工具可用性、最近任务、失败数量和最后一次成功时间。
3. 启动前展示参数摘要、预计写入范围和联网状态。
4. 运行中自动跟随日志，允许暂停自动滚动、复制、筛选 stdout/stderr。
5. 失败卡显示阶段、退出码、失败原因、最后日志和“使用相同参数重试”。
6. 成功卡显示新增/修改文件、Wiki/书籍/Graph 节点变化及报告入口。
7. 危险或不可逆操作使用二次确认；普通只读任务不增加多余确认。

## 10. 自动验收

### 缺陷回归

- 在文献库视图内选择仓库后，自动出现当前全部 source，不需切换页面。
- 快速连续选择两个仓库时，最终只显示第二个仓库数据。
- 车路协同、调度算法、实验数据均可通过鼠标和键盘独立展开/收起。
- 收藏按钮不改变展开状态，滚动后点击区域仍准确。

### 编译中心

- 状态迁移合法，退出码、开始/结束时间和失败原因完整。
- stdout/stderr 顺序稳定，重启后历史可恢复。
- 取消不会留下 `running`；重试不覆盖原任务。
- 命令允许列表、路径越界、日志脱敏和仓库隔离测试全部通过。
- 回滚仅处理清单内文件，哈希冲突时不覆盖用户修改。
- `npm run build`、结构/行为验证、`cargo test`、Wiki 10 题、两书 Recall@5、Tauri release 和启动冒烟全部通过。

## 11. 实施顺序

1. 修复仓库就绪状态和文献库重载。
2. 修复工作区树数据模型、展开状态、点击与键盘行为。
3. 新增 SQLite migration、任务 DTO、状态机和允许列表。
4. 新增子进程流式执行、脱敏、取消、历史恢复与互斥。
5. 接入最小闭环：Lint、Graphify update。
6. 接入发现、下载、解析、A 编译及 full pipeline。
7. 完成编译中心三栏 UI、任务详情、重试和回滚。
8. 建立 fixture 行为测试、Rust 集成测试和发布门禁。
9. 独立子代理核验，主代理按证据修复。
10. 更新 PRD、日志、Graphify，构建 `0.6.0` 安装包。

## 12. 完成定义

两个截图缺陷均有自动回归覆盖；编译中心至少能真实运行 Lint 与 Graphify 更新，并能对发现、下载、解析、A 编译显示明确可用性及真实执行结果；所有任务可审计、可取消、可重试，符合条件的变更可安全回滚；完整发布验收通过。
