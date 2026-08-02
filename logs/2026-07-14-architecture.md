# 架构文档维护详情 — 2026-07-14

## 触发

用户要求理解当前项目及架构，并写出架构文档。

## 读取与核对

- 权威与规则：`prd.md`、`AGENTS.md`、`schema/` 核心规范
- 人类入口：`HOME.md`、`使用说明.md`
- 内容状态：`wiki/index.md`、`wiki/maps/library-status.md`、样例 source/method/synthesis
- 维护记录：`logs/log.md`、首批 ingest 详情
- 派生导航：CodeGraph explore、Graphify query、`graphify-out/GRAPH_REPORT.md`

## 新建 / 修改

- 新建：`ARCHITECTURE.md`
- 新建：`logs/2026-07-14-architecture.md`
- 修改：`HOME.md`、`使用说明.md`、`.graphifyignore`、`logs/log.md`

## 关键判断

1. 架构文档是项目级元文档，放在根目录，不伪装成 9 类领域 wiki 页面，也不新建 map。
2. 正文真相仍在 `wiki/` 与其 raw 溯源；Graphify / CodeGraph 只作为派生导航。
3. 当前图快照含多平台 Graphify skill 副本，中心性不能代表领域重要性；已补忽略规则，待后续全量重建验证。
4. raw 主转换稿未检出 `ingest_status`，本次仅记录，不修改 raw 正文。

## 未做

- 未修改 `prd.md`、`AGENTS.md`、`schema/` 或 `vocab.yaml`
- 未写 `wiki/problems/`、`wiki/ideas/`
- 未外搜
- 未重建 Graphify 图：本次领域 wiki 正文未变，且旧图清噪更适合后续全量重建而非把当前快照当正确结果
