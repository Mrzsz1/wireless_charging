# Windows 客户端 UI 参考稿

生成日期：2026-08-01  
用途：确认信息架构、密度、布局和视觉语言；不是最终像素稿，界面中的示例论文与统计应在实现时由真实知识库数据替换。

## 参考方向

- 飞书：清晰的应用级导航、页面树、克制分隔和高密度生产力布局。
- Zotero：文献列表、阅读区、元数据/证据侧栏组成的研究工作流。
- Obsidian：双向链接、局部图谱和知识关系探索。
- 本项目：三栏科研工作台、证据优先回答、Wiki 为正文真相、Graphify 为派生关系。

只参考交互规律与信息架构，不复制任何品牌标志、专有图标或逐像素界面。

## 图片

| 文件 | 页面 | 主要验证内容 |
|------|------|--------------|
| `01-research-dashboard.png` | 研究工作台 | 全局导航、库水位、近期研究、主题图谱、编译任务、研究脉络 |
| `02-evidence-first-qa.png` | 智能问答 | 结构化回答、证据侧栏、检索轨迹、库水位、继续追问 |
| `03-core-book-reader.png` | 核心书籍 | 目录树、Markdown/PDF 对照、页码锚点、局部图谱、用于当前问题 |

## 视觉令牌草案

```text
Canvas        #F7F7F4
Surface       #FFFFFF
Text Primary  #202225
Text Muted    #69707A
Accent        #287C82
Pending       amber，仅用于待处理与不确定状态
Divider       1px 冷灰
Radius        6–10px
Shadow        极少，仅用于窗口层级与临时浮层
```

## 生成提示词摘要

1. `ui-mockup / 研究工作台`：Windows 原生桌面窗口，56px 图标栏 + 230px 导航 + 中央工作区 + 320px 研究脉络；暖灰、石墨、低饱和蓝青；拒绝渐变、玻璃拟态和卡片套卡片。
2. `ui-mockup / 智能问答`：证据优先，不使用聊天气泡；回答拆为直接可用、可迁移方法、核心书籍依据、库内尚未覆盖；右侧显示文献、书籍、图谱与检索轨迹。
3. `ui-mockup / 核心书籍`：仅展示 Algorithmic Game Theory 与 Approximation Algorithms；章节树、Markdown/PDF 对照、物理页码、公式、局部图谱和关联研究。

生成方式：Codex 内置 `image_gen`，第三张为新图生成；第二张在初稿基础上使用 `text-localization` 编辑，将示意书籍修正为库内真实核心书籍 `Algorithmic Game Theory, Chapter 14, p. 384`。

