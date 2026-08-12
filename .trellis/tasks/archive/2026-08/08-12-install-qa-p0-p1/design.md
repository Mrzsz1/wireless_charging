# 技术设计

## 构建

使用 Tauri 2 Release 构建链，限定 `--bundles nsis`，避免生成本次安装不使用的其他平台包。前端由 `beforeBuildCommand` 自动执行 `npm run build`，Rust 使用 release profile。

## 安装

从 `apps/desktop/src-tauri/target/release/bundle/nsis/` 选择本次构建产生的 `0.12.2` 安装器。先终止产品标识对应的旧进程，再用 NSIS `/S` 静默安装。安装位置以卸载注册表和已安装 EXE 实际路径为准，不猜测固定目录。

## 验证

1. 对安装器运行 `npm run smoke:installer`，传入 `INSTALLER_PATH`。
2. 检查安装目录和主 EXE。
3. 启动主 EXE，等待窗口创建并确认进程未提前退出。
4. 记录安装器 SHA-256 和大小。

## 回滚

安装失败时保留安装器与构建日志；不删除用户知识库和本地会话数据库。若新版启动失败，使用原安装器重新安装前一版本。
