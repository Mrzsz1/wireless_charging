# 修复文献自动检索依赖检查与运行态 UI

## Goal

让自动文献检索页面按当前运行模式正确判定依赖，与 Codex 订阅的 Windows CLI 发现结果保持一致，并使运行中反馈清晰、持续可见。

## Requirements

- 自动准备候选模式只要求发现与下载能力，不得因编译、MinerU 或 Graphify 缺失而标记当前模式受限。
- 自动完整入库模式仍要求发现、下载、解析、Codex 编译和 Graphify。
- Codex 依赖检查复用问答 Provider 的 Windows 发现逻辑，支持 Codex Desktop、持久 PATH、npm/Scoop shim 和显式路径。
- 将已解析的 Codex 可执行路径传入 Python 管道，避免检查通过但运行时仍找不到 CLI。
- “立即检索最新文献”运行时左侧显示持续旋转的加载器，并有明确运行文案。
- 提高底部任务运行栏标题、入口和日志字体，保证可读性。
- 动画支持 `prefers-reduced-motion` 降级。

## Acceptance Criteria

- [x] 当前模式流水线状态不再被非必需依赖误判。
- [x] Codex 依赖与 Provider 状态使用同一发现结果。
- [x] Python 文献/全流水线优先使用已解析 Codex 路径。
- [x] 检索按钮加载器在运行期间持续旋转。
- [x] 任务运行栏在常规缩放下清晰可读。
- [x] 相关测试与发布编译通过。

## Out of Scope

- 修改文献发现与入库算法。
- 伪造任务进度百分比。
