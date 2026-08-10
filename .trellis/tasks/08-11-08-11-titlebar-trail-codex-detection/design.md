# 技术设计

## 现状与根因假设

1. `ResearchTrailPanel` 在关闭状态仍渲染固定宽度的 `context-collapsed-rail`，开关因此悬在右侧内容边缘；这与用户要求的标题栏级控制冲突，并永久占用横向空间。
2. 标题栏已有独立拖拽区和原生窗口控制按钮，可以把研究脉络状态开关放在二者之间，既不破坏拖拽，也能稳定贴近窗口按钮。
3. Codex 检测若只执行 `Command::new("codex")`，在 Windows GUI 环境会同时受到两类问题影响：GUI 进程 PATH 不完整；`.cmd`/`.bat` 不是可由 CreateProcess 直接执行的 PE 文件。

## 前端设计

- 在 `App.tsx` 标题栏拖拽区之后增加 `titlebar-context-toggle`。
- 该按钮直接绑定 `contextOpen`，用 `PanelRightClose` / `PanelRightOpen` 表达当前动作。
- `ResearchTrailPanel` 关闭时返回 `null`；删除面板头部折叠按钮及 `onOpen`/`onClose` props。
- 删除 `.context-collapsed-rail`、`.context-reopen` 样式；新增与标题栏窗口控件同一视觉密度的按钮样式。
- 保留自动进入问答时收起研究脉络的行为，但用户始终可通过标题栏恢复。

## 后端设计

- 把 Codex CLI 发现拆成纯候选构造和实际探测两层：
  1. 显式 `CODEX_CLI_PATH`；
  2. 当前 PATH 中的可执行/脚本启动器；
  3. Windows 常见目录：npm prefix、AppData npm、WinGet Links、用户 `.local/bin`、Cargo、Scoop；
  4. WindowsApps 中 Codex 桌面应用资源目录。
- 候选按优先级去重，只对存在的文件执行 `--version`。
- `.exe`/无扩展可直接执行；`.cmd`/`.bat` 通过 `COMSPEC /D /S /C` 执行固定参数，不拼接用户输入。
- 版本探测成功即认定已安装；登录状态沿用相同已解析路径执行 `login status`。
- 失败信息保持结构化，不暴露凭据。

## 验证设计

- 前端契约检查标题栏位置、唯一控制入口和旧轨道彻底删除。
- E2E 验证按钮在窗口控制区之前、折叠后主内容占满、再次展开恢复面板。
- Rust 单元测试验证候选顺序、去重、扩展名识别和固定命令构造。
- 在本机调用设置状态命令或相关后端测试，确认能解析实际 `F:\nodejs\codex.cmd`。

## 风险控制

- WindowsApps 目录可能无权限枚举：忽略单个目录错误，继续其他候选。
- PATH 中可能同时存在 npm shim 与桌面版二进制：以首个能返回版本的候选为准。
- `cmd.exe` 引号规则复杂：只传可信本地候选路径和固定参数，并针对带空格路径添加测试。
