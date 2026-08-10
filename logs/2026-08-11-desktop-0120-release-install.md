# Windows 客户端 0.12.0 发布与安装

## 发布内容

- 包含 commit `877e262` 实现的 canonical 论文章节 FTS5 检索、
  `paper / primary_source` 证据显示和 P1 知识层。
- npm、Cargo、Tauri 和 updater fixture 版本从 0.11.0 统一升级到 0.12.0。

## 构建产物

| 类型 | 路径 | 大小 | SHA-256 |
|---|---|---:|---|
| MSI | `apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.12.0_x64_en-US.msi` | 11,534,336 bytes | `39881C69D9886868928F37F6EB6CDE753D53B28D9E25D121405D0084BD319A0A` |
| NSIS | `apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.12.0_x64-setup.exe` | 8,022,508 bytes | `35058C7E0A365A91DA0C4460715A22DA632D26348FD1A9B70C12496ADDF4DABE` |

## 验证

- `verify-config.mjs`：PASS，Tauri 0.12.0。
- `verify-updater-release.mjs`：PASS，no-update/update/tampered 三类 fixture 有效。
- `npm --prefix apps/desktop run tauri build`：PASS，生成 MSI 与 NSIS。
- NSIS `/S` 静默安装：exit code 0。
- Windows 卸载注册表：`DisplayVersion = 0.12.0`。
- 已安装程序：
  `C:\Users\qq155\AppData\Local\Wireless Charging Research Workbench\app.exe`，
  ProductVersion 0.12.0。
- 启动验证：进程正常响应，主窗口标题为
  `Wireless Charging Research Workbench`。
