# Windows 客户端窗口可见性修复（0.7.2）

## 现场证据

- 当前显示器工作区：`{x:0,y:0,width:2048,height:1104}`。
- 故障窗口矩形：`{left:-2858,top:381,right:-708,bottom:1589}`，与当前工作区无交集。
- 残留进程来源：`apps/desktop/.tmp/p5-installer-verify/app.exe`。
- 旧实现以物理像素保存 `innerSize` / `outerPosition`，却用逻辑像素类型恢复，存在 DPI 二次缩放。

## 修复

1. 新增纯函数窗口放置模块，统一物理像素、当前显示器工作区校验与 v2→v3 迁移。
2. 合法负坐标副屏位置继续保留；离屏位置回到主屏中央；过大窗口限制在工作区内。
3. 最小化时不覆盖正常窗口矩形，最大化只保存标志；启动恢复后取消最小化、显示并聚焦。
4. Tauri capability 补齐窗口恢复权限，初始窗口启用居中兜底。
5. 安装 smoke 显式终止进程树并等待退出后再卸载。

## 验证

- `npm run test:p1`：8/8。
- `npm run test:p2`：3/3。
- `npm run test:installer-lifecycle`：2/2。
- Rust：fmt、Clippy `-D warnings`、32/32 测试通过。
- Python 工具链：37/37；Wiki 固定问题 10/10；Lint 0 errors。
- 核心专著 295 条：Algorithmic Game Theory Recall@5=1.000；Approximation Algorithms Recall@5=0.986667。
- 严格 GUI E2E：启动窗口 `{x:348,y:169,width:1367,height:769}` 与工作区 `{x:0,y:0,width:2048,height:1104}` 相交；关键导航与 1366×768、1920×1080 视口通过。
- NSIS 严格安装/启动/退出/卸载通过，安装目录对应进程无残留。

## 产物

- `apps/desktop/src-tauri/target/release/app.exe`
  - SHA-256: `B4595386B5ACC1F35CAE64295704BBD2295FEDF89928353927291273FF6D672A`
- `apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.7.2_x64_en-US.msi`
  - SHA-256: `0C8AE406B6A62DDA250410F86C6F5F1581C201C9980C4D0565200F7F6748D88B`
- `apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.7.2_x64-setup.exe`
  - SHA-256: `A368FE9789F6C1A5808CFE4EB655210916269B071BCEA9714CAC227AE4CA55C4`

`raw/` 与 `wiki/` 正文未改动。
