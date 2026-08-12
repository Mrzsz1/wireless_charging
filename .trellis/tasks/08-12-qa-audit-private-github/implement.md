# 实施步骤

## A. 任务与基线

- [x] 创建任务并写入智能问答现状、P0/P1/P2 审查结论。
- [x] 记录 Git 状态、现有远端、GitHub CLI 登录账户和仓库体积。

## B. 敏感信息与推送边界

- [x] 审查 `.gitignore`、高风险文件名、当前已跟踪文件和未跟踪文件。
- [x] 使用 Gitleaks 扫描待提交文件与全部 Git 历史；37 条命中均已按上下文核验为 36 个 SerpApi 文献结果标识符和 1 个规范文档术语误报，未发现凭据。
- [x] 检查历史最大 blobs、当前最大文件和 GitHub 100 MiB 限制；最大 blob 5.25 MiB，无超过 50 MiB 的对象。
- [x] 补齐忽略规则并确保两个 discovery 失败目录不进入暂存区。
- [x] 对清理后的提交再次运行秘密扫描和 `git diff --cached --check`。

## C. 私有 GitHub 发布

- [x] 提交安全配置与任务文档。
- [x] 创建私有 GitHub 仓库，设置 `origin` 并推送默认分支。
- [x] 使用 GitHub CLI/API 确认可见性为 PRIVATE、默认分支 `master` 存在且 upstream 正确。
- [x] 更新任务结果，归档并记录 Trellis journal。

## 验证命令

```powershell
git status --short
git remote -v
gh auth status
gitleaks git --no-banner
git rev-list --objects --all
gh repo view --json visibility,url,defaultBranchRef
git branch -vv
```
