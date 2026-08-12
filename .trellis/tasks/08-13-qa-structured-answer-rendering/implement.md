# 实施计划

- [x] 定义 structured answer serde 契约、严格解析和结构验证。
- [x] 用结构化 claim 直接生成 CitationValidation，绕开 Markdown 断句器。
- [x] 程序生成正文与末尾紧凑参考证据 Markdown。
- [x] 更新 prompt/schema version，要求 Codex/API 输出 JSON。
- [x] 隔离 supplement 并生成 trusted context。
- [x] 前端隐藏 JSON token 预览，引用使用稳定颜色和短链接。
- [x] 更新 Trellis QA 前后端契约。
- [x] 运行 fmt、cargo check、tsc、diff-check，不运行测试。
- [x] 编译 Tauri release。
