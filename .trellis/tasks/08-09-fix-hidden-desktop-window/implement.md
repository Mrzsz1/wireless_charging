# Windows 窗口不可见 Bug 实施步骤

## 0. 基线与即时证据

1. 保存当前 Git 基线与任务目录。
2. 记录故障进程路径、PID、`GetWindowRect`、`GetWindowPlacement` 和当前显示器工作区。
3. 确认窗口矩形与所有工作区无交集，作为修复前失败样本。
4. 提交规划文件：`docs(trellis): plan visible desktop window recovery`。

## 1. 先写失败回归

1. 新建 `apps/desktop/src/lib/windowPlacement.ts` 的测试文件或扩展 P1 测试。
2. 用纯数据覆盖：
   - 单屏 `{0,0,2048,1104}` + 故障位置 `{-2858,381,2150,1208}`；
   - 左侧合法第二屏负坐标；
   - 分辨率从 4K 缩小到 1080p；
   - 保存尺寸大于工作区；
   - v2 迁移与 v3；
   - null、字符串数字、NaN 等价非法值、负尺寸、过小尺寸。
3. 断言输出完全位于一个工作区内、合法负坐标不被误判、最大化标志保留。
4. 在 `package.json` 增加或扩展可独立执行的窗口状态测试命令。

## 2. 实现纯窗口放置模块

1. 定义最小类型与常量，不导入 Tauri。
2. 实现有限数/尺寸验证、矩形交集面积、显示器选择、工作区 fit、主屏居中。
3. 实现 v2/v3 解析与 v3 序列化契约。
4. 运行 Node 测试和 TypeScript 类型检查。
5. Git 检查点：`fix(desktop): keep restored windows on an active monitor`。

## 3. 接入 App 启动与持久化

1. `App.tsx` 改用 `PhysicalSize`/`PhysicalPosition` 与监视器 API。
2. 恢复前解除最大化，应用安全矩形，再恢复最大化状态。
3. 执行 unminimize/show/setFocus；失败走 center/show 兜底。
4. 保存时忽略最小化几何；最大化只更新标志；正常态更新最后矩形。
5. move/resize 使用单一防抖调度，恢复结束后才注册监听。
6. `tauri.conf.json` 设置初始 `center: true`。
7. capability 与结构门禁同步新增权限。

## 4. 修复安装 smoke 进程泄漏

1. 将 `spawnSync --version + timeout` 替换为显式 `spawn` 生命周期。
2. 封装等待启动、终止、超时强杀和退出确认。
3. 卸载前断言应用进程已退出；卸载后断言路径消失。
4. 增加脚本级测试或 fixture，覆盖正常退出与强制终止。
5. 终止当前残留 `.tmp/p5-installer-verify/app.exe`，确认 PID 消失。

## 5. 版本与发布

1. 将 package/Cargo/Tauri/fallback/verify-config 版本统一更新为 0.7.2。
2. 更新 README、根 PRD 和 `logs/2026-08-09-window-visibility-fix.md`。
3. 运行 `npm run tauri build`，保留 0.7.1 安装包。
4. 记录 app/MSI/NSIS SHA-256。

## 6. 完整质量门

按顺序执行：

```powershell
git diff --check
cd apps/desktop
npm run test:p1
npm run test:p2
npm run build
npm run verify
npm run verify:p3
npm run verify:p4
npm run verify:p5
cd src-tauri
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cd ../../../..
python -m unittest discover -s tests -p "test_*.py" -v
python -m compileall -q tools tests
python tools/wiki_eval.py
python tools/core_book_eval.py
python tools/wiki_lint.py --strict-graphify
```

## 7. 严格 GUI 与安装验收

1. 用故障 v2 状态写入本地存储后启动 0.7.2 release。
2. GUI E2E 获取窗口矩形和全部显示器工作区，断言交集面积大于 0。
3. 在 1366×768、1920×1080 验证导航和无横向溢出。
4. 运行 NSIS 隔离目录安装、启动、进程退出、卸载；结束后检查无该路径 `app.exe` 进程。
5. 执行 `verify:p5:strict`，关键步骤不得出现 `SKIP`。

## 8. 收口

1. 运行 `trellis-check`，映射 R1-R5 与 AC1-AC10。
2. 评估并更新窗口状态恢复规范，记录物理/逻辑像素边界和显示器相交契约。
3. 更新 Graphify 派生图，不覆盖 `wiki/`。
4. 分批提交代码、测试、文档和派生图。
5. 归档 Trellis 任务并记录开发日志；最终 `git status --porcelain` 为空。

