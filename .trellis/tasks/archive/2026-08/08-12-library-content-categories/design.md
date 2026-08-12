# 技术设计

## 边界

分类属于前端派生视图：后端继续返回既有 `PageSummary.pageType` / `SearchResult.pageType`，不修改数据库和 IPC。分类定义与过滤逻辑放入独立纯函数模块，`LibraryView` 只负责状态和渲染。

## 类型映射

| pageType | 中文分类 |
|---|---|
| source | 论文文献 |
| method | 方法 |
| synthesis | 综述 |
| concept | 概念 |
| system-model | 系统模型 |
| objective | 优化目标 |
| dataset-or-sim | 数据与实验 |
| problem | 研究问题 |
| map | 知识地图 |

`all` 是 UI 虚拟分类，不写入数据。未知类型不生成独立按钮，但保留在“全部”中，卡片兜底显示“其他页面”。

## 数据流

1. `query` 为空：把 catalog 转为 SearchResult，作为 base results。
2. `query` 非空：使用后端搜索 results，作为 base results。
3. 从 base results 计算分类计数。
4. 文献库按当前 category 过滤；方法库固定按 method 过滤。
5. 过滤后的结果进入现有 `paginate`，分类变化触发 `setPage(1)`。

## UI

在搜索框下、年份/状态筛选上方增加可横向滚动/换行的轻量分段按钮。激活态使用天蓝底、蓝色文字和细边框；按钮展示标签与计数。结果卡左栏宽度适配“数据与实验/知识地图”等中文长标签。

## 兼容与回滚

不改变后端契约和持久化数据；回滚只需移除纯函数模块、分类栏和对应 CSS。现有搜索、分页、打开页面回调保持原签名。
