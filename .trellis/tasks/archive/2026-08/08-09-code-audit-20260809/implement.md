# 审查执行计划

## 阶段 1：任务激活与基线

1. 复核 `prd.md`、`design.md`、Trellis backend/frontend 规范索引。
2. 确认 `git status --short` 干净，记录基线提交和运行环境版本。
3. 执行 `task.py start 08-09-code-audit-20260809`，将任务置为 `in_progress`。

## 阶段 2：结构化定位

1. 用 Graphify 查询核心路径：索引、QA、会话、Tauri 命令、前端调用和工具入口。
2. 用 CodeGraph 获取符号源代码、调用关系和准确 `file:line`。
3. 盘点 Rust、TypeScript/React、Python、配置和测试文件，建立审查清单。

## 阶段 3：静态审查

1. 沿 Storage → Rust service/command → frontend invoke → UI 状态追踪数据流。
2. 审查路径、错误、并发、序列化、数据库事务、缓存和凭据边界。
3. 逐条核对测试断言是否覆盖发现的风险；记录已验证缺陷与待验证风险。

## 阶段 4：动态验证

按仓库现有入口执行并记录版本、命令、结果和失败日志：

```powershell
python -m pytest -q
python -m unittest discover -s tests -p "test_*.py" -v
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
```

前端/打包命令从 `apps/desktop/package.json`、`apps/desktop/src-tauri/tauri.conf.json` 和既有 CI/脚本中读取后执行；不存在的命令标记为“未配置”，不自行猜测替代命令。

## 阶段 5：报告编写

1. 将每个发现写入 `review-report.md`，按 P0/P1/P2 和置信度排序。
2. 为 P0/P1 逐条补充直接源码证据、复现步骤或静态推导。
3. 汇总测试通过/失败/跳过项、覆盖盲区、未发现的高风险类别和修复优先级。
4. 对任何争议结论做二次源文件抽查，排除 Trellis 指南中列出的 AI 误报模式。

## 阶段 6：质量门与收尾

1. `git diff --check`；确认只有当前任务文档/报告变化。
2. 重新运行必要的最小验证，确保报告中的命令和行号可复核。
3. 若发现非显而易见的审查规则或项目约定，提出待用户确认的 `.trellis/spec/` 更新建议，不直接修改规范。
4. 通过质量门后提交任务文档与报告，再归档任务并记录会话日志。

## 风险点与回滚点

- 大型 Rust/Python 文件可能导致审查遗漏：按 CodeGraph 符号分片，记录未覆盖区域。
- 前端依赖安装可能改变 lockfile：默认不安装依赖；仅使用已存在的依赖和脚本。
- 测试可能写入真实知识库：优先使用项目测试夹具/临时目录，发现测试会改写 `raw/` 或 `wiki/` 时立即停止该命令并记录。
- 所有业务代码修改均为禁止项；若工具自动生成改动，立即恢复到审查前状态并保留失败证据。
