# P5.4 技术设计

## 1. 总体边界

本阶段保持现有 Tauri 2 + React/TypeScript + Rust + SQLite 架构。修改限定在桌面客户端、测试、发布元数据和项目文档；不改动 Raw/Wiki 正文。

```text
React App
├─ LatestRequestGuard：只提交最新搜索结果
└─ 既有 invoke API（命令名和响应结构保持兼容）

Rust/Tauri
├─ repository_watcher：in-flight 批次、ack/retry/blocked
├─ repository index：可测试的 apply_repository_changes
├─ core books：统一安全路径解析 + 字符安全 snippet
└─ compile_center：预检、staging、补偿日志、终态落账

SQLite / filesystem
├─ SQLite：索引事务与 compile run/event 审计
└─ 文件系统：同卷 staging + 反向补偿，正文仍以文件为真相
```

## 2. 全局搜索并发合同

### 2.1 状态模型

在 `App.tsx` 中用 `useRef<number>` 保存 `searchGeneration`：

1. `handleSearch(value)` 进入时递增 generation，并保存当前 token。
2. 输入为空时更新 query、清空结果并返回；递增动作已经使所有旧请求失效。
3. `searchPages` 完成或失败后先比较 token 与当前 generation。
4. token 过期时完全忽略，不写 `results`、`notice` 或 loading 状态。

为便于 Node 单元测试，将“生成 token / 判断当前 token / 主动失效”提取为无 React 依赖的小模块，例如 `src/lib/latestRequest.ts`。不引入 AbortController，因为 Tauri `invoke` 不保证可取消；丢弃旧响应是最小且确定的机制。

### 2.2 兼容性

- `searchPages(query, limit)` 接口不变。
- 不改变 debounce、排序和结果 DTO。
- 若后续增加 loading，loading 也必须绑定 token。

## 3. 专著路径与片段合同

### 3.1 安全路径解析

新增单一内部函数，例如：

```rust
fn resolve_repository_file(
    root: &Path,
    relative: &str,
    context: &str,
) -> Result<PathBuf, String>
```

处理顺序：

1. 解析 `Path` components，拒绝空路径、absolute/root/prefix 和 `ParentDir`。
2. `root.join(relative)` 后要求目标存在且为普通文件。
3. canonicalize 仓库根和目标；要求目标 `starts_with(canonical_root)`。
4. Windows 下同样拒绝 drive、UNC、junction/symlink 逃逸。
5. 返回 canonical path，所有列表、读取、搜索、FTS 建库入口只使用该返回值。

章节索引本身仍位于受控固定路径 `raw/canonical/<book-id>/chapter-index.json`。错误信息包含 book/chapter 上下文，但不读取或输出越界目标内容。

### 3.2 字符安全片段

新增纯函数，例如：

```rust
fn build_book_snippet(title: &str, body: &str, terms: &[String]) -> String
```

规则：

1. 先独立判断 title/body 命中；title 不参与 body 偏移。
2. body 命中位置必须映射回原字符串字符边界。
3. 窗口按字符数取命中前约 90 字符、后约 180 字符，再折叠换行/连续空白。
4. 仅标题命中或无法定位正文时，返回正文前 260 字符；空正文返回空字符串。
5. 排名 hits 逻辑保持不变，避免无关的 Recall 漂移。

实现优先使用标准库字符迭代和显式原始 byte boundary 映射，不新增正则或 Unicode 依赖。单元测试是纯函数测试，不依赖真实两本书；真实评测作为集成门。

## 4. 回滚失败原子性设计

### 4.1 基本原则

跨多个文件无法依赖单个 OS 原子操作，因此采用 **staging + 操作日志 + 反向补偿** 达成用户可观察的失败原子性。数据库事务不能替代文件系统恢复。

### 4.2 阶段

#### A. 全量预检

- 验证原 run 属于当前 repository 且状态为 `succeeded`。
- 验证 artifact operation 仅为 created/modified/deleted。
- 通过统一边界函数验证所有 `relative_path`。
- 验证当前目标状态和 `after_hash`。
- 对 modified/deleted 验证 backup 存在、可读且 hash 等于 `before_hash`。
- 预检全部通过前不创建或修改目标文件。

#### B. 建立 rollback run 与 staging

- 创建 rollback run，状态 `running`。
- 在仓库受控备份区建立唯一 staging 目录。
- 将所有待恢复 backup 复制到 staging 并再次校验 hash。
- 为会被替换/删除的当前目标准备 compensation 副本或同卷 quarantine 路径。

#### C. 应用与操作日志

- 每完成一个 artifact，立即把 `relative_path`、operation、前后 hash、compensation 路径加入内存 journal。
- 文件替换尽量使用同卷 rename；Windows 目标已存在时先移入 quarantine，再把 staged 文件 rename 到目标。
- 原 run 状态此时仍不变。

#### D. 失败补偿

- 任一文件或数据库步骤失败，按 journal 逆序恢复。
- 补偿完全成功：rollback run=`failed`，记录原始错误和 compensated=true；原 run 保持 `succeeded`。
- 补偿仍失败：rollback run=`failed_partial`，记录已恢复、未恢复、当前 hash 和人工处理提示；原 run 不标记 `rolled_back`。
- 每条失败路径都写 `compile_run_events` 终态事件。

#### E. 成功落账

- 所有文件成功后，在一个 SQLite transaction 中更新 rollback run=`succeeded`、原 run=`rolled_back` 并写 completed event。
- 若数据库 transaction 失败，先补偿文件，再另行写失败终态；不得留下永久 `running`。
- 清理 staging/quarantine；清理失败只记录 warning，不把已成功回滚反转为失败。

### 4.3 崩溃恢复

staging 目录包含最小 manifest（rollback_id、source run、journal、阶段）。应用启动时沿用 interrupted recovery：发现未终结 rollback run 时标记 `interrupted` 并保留 manifest，设置页/编译中心可提示“重试回滚或清理”。本阶段不实现自动无提示恢复，以免在不确定文件状态下再次写入。

## 5. Watcher 至少一次处理设计

### 5.1 批次状态机

`RepositoryWatcher` 增加内部状态：

```text
collecting → ready → in_flight → acknowledged
                         ├─ retry_wait → in_flight
                         └─ blocked → manual retry/full rebuild
```

建议 API：

- `begin_batch(now) -> Option<ChangeBatch>`：收集新事件；若已有 in-flight，返回同一批次而不是丢弃。
- `ack_batch(batch_id)`：仅在 SQLite/完整重建成功后清除。
- `fail_batch(batch_id, error)`：增加 attempt，保留 changes，计算下一次重试时间。
- `clear_after_full_rebuild()`：手动完整重建成功后确认所有已覆盖变化。

### 5.2 去重和 rename

- 同路径 create/modify 可合并为最终 upsert。
- remove 覆盖同路径之前的 modify。
- rename 必须作为包含 `previous_path` 与 `path` 的单元保留，不拆丢任一侧。
- graph-only、schema/core full-rebuild 标记沿用现有优先级。

### 5.3 应用层解耦

将 `process_repository_changes` 内部 SQLite 部分提取为可测试函数：

```rust
fn apply_repository_changes(
    connection: &mut Connection,
    root: &Path,
    changes: &[IndexChange],
) -> Result<ApplyStats, String>
```

Tauri command 只负责编排 begin/apply/ack/fail 和事件通知。这样可用临时仓库注入不可读文件、损坏 Markdown 或事务失败，验证批次保留和重试，不需要构造 `AppHandle`。

自动重试采用有限次数与封顶退避；达到上限后批次仍保存在 watcher 内存中并显示 blocked，而不是被丢弃。手动完整重建是最终兜底。

## 6. 数据与接口变更

1. 不改 Tauri 命令名；现有前端调用保持兼容。
2. `RepositoryWatchStatus` 增加可选字段：`pending_changes`、`retry_attempt`、`blocked`、`last_error`。前端按缺省值兼容旧响应。
3. compile run `status` 允许 `failed_partial`；当前 DTO 使用字符串，无需破坏性 schema migration。若查询过滤有枚举映射，必须同步。
4. rollback `result_json` 记录 restored/compensated/failed artifact 列表；不记录文件正文。

## 7. 测试设计

### 前端

- 抽取 latest-request helper 的 Node 测试：乱序成功、乱序失败、清空失效、连续三请求。
- 结构/GUI 测试确认 `Ctrl+K` 和结果点击无回归。

### Rust 核心专著

- snippet：标题、正文头中尾、ASCII 大小写、中文、emoji、空正文、短正文。
- 路径：合法文件、绝对路径、`..`、Windows prefix（按平台条件）、symlink/junction 越界。
- 真实两书评测保持 295 条与 Recall 门限。

### Rust 回滚

- created/modified/deleted 混合多文件成功。
- 预检失败时零文件变化、零 running 残留。
- 第 2 个 artifact 应用失败，补偿后所有 hash 与回滚前一致。
- 补偿失败产生 `failed_partial` 和完整事件/result_json。
- 数据库落账失败路径不留下错误的 `rolled_back`。

### Rust watcher

- 首次 apply 失败、第二次成功，同批 batch id/changes 保持。
- rename 双路径在重试后仍完整。
- 批量事件去重、退避、blocked、manual full rebuild clear。
- 增量最终结果与全量重建一致。

## 8. 风险与取舍

1. **不切换书籍搜索引擎**：只修 snippet 和路径，避免影响 95% Recall 基线。
2. **不声称跨文件 OS 事务**：用可验证补偿实现失败原子性，并显式暴露 `failed_partial`。
3. **不无限高频重试 watcher**：有限自动重试 + 保留 blocked 批次 + 一键全量重建，兼顾一致性和资源占用。
4. **不依赖请求取消**：Tauri invoke 不提供可靠取消，前端用 generation guard 保证状态正确。

## 9. 发布与回退

- 每个工作包独立 Git 提交；实施前记录基线 commit。
- 若 watcher 新状态机不稳定，可单独回退 watcher 提交，不影响搜索/书籍/rollback 修复。
- 0.7.1 发布前保留 0.7.0 安装包和 updater 回退路径。
- 任何质量门失败都不生成“已完成”记录，不更新稳定通道版本。
