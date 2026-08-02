# Ingest 检查清单（A 编译）

## 用户侧（投料）

1. [ ] 源类型在白名单内（论文/专利等；**非**网页/blog/PPT）  
2. [ ] 先分流：自动发现进 `raw/inbox/auto-discovered/`，手动投放进 `raw/inbox/manual-drop/`；确认后再进 canonical  
3. [ ] **一文一夹**：`raw/canonical/<PaperSlug>/`  
4. [ ] MinerU：夹内有 **同名/主** `.md` + **`images/`**（保持相对路径）  
5. [ ] PDF 建议同夹归档；HTML **可选**，仅人读，不作编译主源  
6. [ ] 在 Obsidian/编辑器打开 md，确认图片能显示（`![](images/...)`）  
7. [ ] md 可读；失败则 `ingest_status: convert_failed`，停止  
8. [ ] provenance 完整：`acquisition_method`；auto 项还要有 `discovered_via` / `discovery_run`  
9. [ ] 建议填写 `why_relevant`、确认 `year`  
10. [ ] 通知外部 agent：编译哪些夹/md  

## 图片相关

- [ ] 未把多篇 `images` 合并到同一目录  
- [ ] A 编译以 md 文本为主；关键图再读 `images/` 或 PDF  
- [ ] 不把整包图片复制进 `wiki/`（避免库膨胀）；必要时 wiki 链回 raw 路径

## Agent 侧（A 编译）

1. [ ] 只处理 `pending_ingest` 且 md 可读的 canonical 条目  
2. [ ] 读取 `schema/vocab.yaml`；缺词只写 `vocab-proposals.md`  
3. [ ] 新建/更新 `wiki/sources/src-*.md`，传播来源追踪字段  
4. [ ] 抽取作者 `Keywords` / `Index Terms`；没有则明确 `paper_keywords: []` 与 `keyword_source: not_found`  
5. [ ] 新关键词已补入 `map-domain-keywords` 并保留 source 证据；没有直接改 `vocab.yaml`  
6. [ ] 抽取并更新（若需要）concept / system-model / objective / method / dataset-or-sim  
7. [ ] 相关 synthesis 追加并列对照；**不裁断冲突**  
8. [ ] 已有 map **补链接**；新建 map 主题 → 列入「待用户确认」  
9. [ ] 更新 `wiki/maps/library-status.md`  
10. [ ] raw md frontmatter：`ingest_status: ingested`  
11. [ ] 写 `logs/YYYY-MM-DD-*.md`  
12. [ ] **不写** problem/idea（除非用户本轮明确要求并确认写入）  

## 验收（单篇）

- source 页有 year、source_type、匹配字段（能填则填）  
- source 页有 acquisition_method；auto 项可回到 discovery run  
- source 页有 `paper_keywords` 与 `keyword_source`；地图可回链到该 source  
- 至少 1 个指向 method 或 concept/system-model 的链接（若文献有可抽内容）  
- 无 B 类贡献句污染  
