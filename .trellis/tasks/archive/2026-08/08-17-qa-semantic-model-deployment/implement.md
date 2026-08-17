# Implement

## Ordered steps

1. 在 `qa/semantic.rs` 抽取缓存目录状态、目录切换失效、静态部署检查、探针检查、下载修复和安全复制函数。
2. 在 Tauri 层实现全局 JSON 设置持久化、目录选择/打开及六个 semantic deployment commands，并在应用启动时加载配置。
3. 扩展 Rust/TypeScript DTO 与 `services/desktop.ts` 调用边界。
4. 在 `SettingsView` 增加独立语义模型部署卡片和对应样式，确保未选择知识库时仍可操作。
5. 添加 Rust 回归：默认/自定义路径、原子配置、missing/partial、路径切换清空状态、复制保留源目录、命令 DTO 序列化。
6. 添加前端回归：全局加载、状态展示、检查与下载动作分离、路径切换双策略。
7. 更新 `.trellis/spec/backend/qa-contract.md` 和前端组件契约。

## Validation

```powershell
cargo fmt --check --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml --lib
cd apps/desktop
node --experimental-strip-types --test tests/*.test.ts tests/*.test.mjs
npm run build
npm run tauri -- build
```

## Risk and rollback points

- 自定义目录必须经过绝对路径与可写验证，禁止把文件当目录。
- 复制目标不得等于源目录、位于源目录内部或反向包含源目录。
- 检查函数不得调用会下载缺失文件的路径；静态完整性必须先于 fastembed 初始化。
- 任何复制、下载或配置写入失败都不得删除旧缓存。
- 语义状态失效和缓存目录更新必须在一致的锁顺序下完成，避免检索并发看到半切换状态。
