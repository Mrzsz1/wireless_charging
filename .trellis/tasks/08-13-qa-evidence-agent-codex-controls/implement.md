# 实施计划

## 1. 查询规划与诊断

- [x] 抽取组合式 query term builder，删除完整问题短语 alias。
- [x] 新增单轮 channel retrieval helper、标题驱动扩展和最多三轮 orchestration。
- [x] 定义充分性/低增益/无新词/上限停止条件。
- [x] 扩展 privacy-safe diagnostics、Rust/TS DTO 与 Thinking 投影。
- [x] 增加多主题、同义改写、负向漂移和停止条件测试。

## 2. 证据分级与可信上下文

- [x] 扩展 heading-aware grounding parser 与 `mixed` 状态。
- [x] 更新 prompt/answer contract，加入可选模型补充章节和固定风险提示。
- [x] 增加 `trusted_context` 幂等 SQLite 迁移及生成投影。
- [x] 调整历史查询/完整轮配对，使 mixed 只注入验证部分。
- [x] 扩展前端 citation summary、消息状态和审计展示。
- [x] 测试 supported/mixed/unverified/invalid、伪引用、Graphify-only 和历史隔离。

## 3. Codex 超时

- [x] 将 Codex total-only timeout 改为 idle + hard deadline，活动事件刷新 idle clock。
- [x] 提升默认值并保持 API provider 原语义。
- [x] 增加持续活动、静默、总时限、取消和部分输出测试。

## 4. 模型/推理控制

- [x] 校正 model cache/config projection、effort allowlist、排序与 capability fallback。
- [x] 扩展 AskRequest 和后端 request snapshot 解析。
- [x] 把模型/effort 控件迁入 AskView composer，保存默认但当前请求显式携带快照。
- [x] 移除 SettingsView 重复控件，保留登录/刷新/高级设置。
- [x] 重构标题状态和 CSS，使控件与现有软件视觉一致。
- [x] 增加 TypeScript 模型联动、请求快照、目录缺失和样式契约测试。

## 5. 验证与交付

- [x] `cargo fmt --check`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] `cargo test --lib`
- [x] `npm test` / QA settings/P1/P3/P5 定向测试
- [x] `npm run build`
- [x] `python tools/wiki_lint.py`
- [x] `git diff --check` 与秘密/未跟踪文件审查
- [x] 更新 `.trellis/spec/backend/qa-contract.md` 与 frontend component contract
- [x] Git 提交并编译桌面 release，验证产物

## 风险与回滚点

- 查询循环改动集中在 retrieval helper；保留单 pass 测试基线，可独立回滚。
- `mixed` 只放宽显式模型补充章节，不修改普通声明门禁；若解析异常，整轮仍 invalid。
- `trusted_context` 新列是附加列，回滚代码不破坏旧 content。
- Composer 选择通过请求快照传递，不依赖异步设置保存成功。
- 超时测试使用可控 fixture 进程/事件，不访问真实 provider。
