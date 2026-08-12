# 生成并安装 P0-P1 客户端

## 目标

将已合入私有 `master` 的智能问答 P0-P1 版本构建为 Windows Release 安装包，安装到本机，并验证已安装程序可以启动且版本与产物一致。

## 范围

- 保持当前应用版本 `0.12.2`，本任务只负责交付已提交代码，不引入功能改动。
- 运行 Release 打包，优先使用 NSIS `.exe` 安装包。
- 安装前关闭正在运行的旧客户端进程，使用静默安装参数完成安装。
- 验证安装目录、可执行文件版本、启动进程和窗口存活。
- 使用项目安装包 smoke 脚本验证进程生命周期。
- 记录安装产物绝对路径、大小和 SHA-256。

## 验收标准

- [x] `.\\node_modules\\.bin\\tauri.cmd build --bundles nsis` 成功。
- [x] 生成版本 `0.12.2` 的 NSIS 安装包。
- [x] 安装包 smoke 检查通过。
- [x] 本机安装成功，安装目录存在新版 EXE。
- [x] 新版客户端启动并保持运行，窗口可见。
- [x] 产物路径、大小和 SHA-256 已记录。
- [x] Trellis 任务归档、Journal 记录和 Git 推送完成。

## 非目标

- 不实施 P2。
- 不修改问答业务代码。
- 不启用在线 updater 或代码签名。

## 安装结果

- 安装器：`E:\知识库\wireless_charging\apps\desktop\src-tauri\target\release\bundle\nsis\Wireless Charging Research Workbench_0.12.2_x64-setup.exe`
- 安装器大小：`8,382,658` bytes
- SHA-256：`EEF320982B7C789DBF984DD8987B78C489AB7F5B01F4A6A76680F6BA825B2C7A`
- 已安装程序：`C:\Users\qq155\AppData\Local\Wireless Charging Research Workbench\app.exe`
- 文件版本：`0.12.2`
- 启动验证：进程响应正常，窗口标题为 `Wireless Charging Research Workbench`。
