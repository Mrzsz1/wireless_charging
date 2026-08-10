# 发布并安装 LLM Wiki P0/P1 客户端

## Goal

将已提交的论文原文章节检索和 P1 知识层功能打包为 Windows 客户端，
从 0.11.0 升级到 0.12.0，安装后能启动并触发派生索引升级。

## Requirements

- 统一更新 npm、Cargo 和 Tauri 版本为 `0.12.0`。
- 执行 release 级 Tauri 打包，至少生成 NSIS `.exe` 安装包。
- 静默安装新版客户端，不把两个既有 raw discovery 目录纳入 Git。
- 核验安装包版本、安装后可执行文件与进程启动。

## Acceptance Criteria

- [x] npm/Cargo/Tauri 版本均为 0.12.0。
- [x] `npm --prefix apps/desktop run tauri build` 成功。
- [x] NSIS 安装包存在并完成静默安装。
- [x] 已安装的 0.12.0 可执行文件能启动。
- [x] 版本修改、发布记录与 Trellis 任务提交到 Git。

## Constraints

- 不重新修改 P0/P1 功能代码。
- 不纳入 `raw/inbox/auto-discovered/runs/search-20260809-204315/` 与
  `search-20260809-211516/`。
- 安装前保留可回滚的 0.11.0 安装包（如现有构建目录中存在）。
