# Ingest 详情 — 2026-07-10 batch1

## 触发

用户声明 `raw/canonical` 已就绪，执行首批 A 编译。

## 处理 raw（9 夹，主文件 full.md）

1. Charging_on_the_Move_... → src-wu-charging-on-the-move  
2. Concurrently_Wireless_Charging_... → src-guo-concurrent-ccsp  
3. Concurrent_Charging_with_Wave_Interference... → src-ma-concurrent-gain  
4. Dynamic_Power_Distribution_... → src-ma-tide-dynamic-power  
5. Cooperative_Charging_as_Service_... → src-xu-cooperative-ccs  
6. Peak_AoI_Minimization_... → src-chen-peak-aoi-wpt  
7. Placing_Wireless_Chargers_With_Multiple_Antennas... → src-dai-wanda-multi-antenna  
8. Practical_Heterogeneous_Wireless_Charger_Placement... → src-wang-hipo-obstacles  
9. 3-D_Placement_of_an_Unmanned_Aerial_Vehicle... → src-alzenad-uav-bs-qos（边界）

## 新建 wiki

- sources: 9  
- concepts: 5  
- methods: 8  
- syntheses: 1  

## 词表提案

无强制新 id；AoI 以 concept 页承载。可选未来：`problem_class: aoi_scheduling` → vocab-proposals。

## 待用户确认

1. 是否保留 UAV-BS 边界文在核心库  
2. 多篇 `year: null` 是否需你从 PDF 页眉补全  
3. Graphify 重建：`$graphify . --update`  
4. 文件夹名含 UUID 很长——是否重命名为短 slug（可选，改则需同步 source frontmatter 路径）

## 未做（按规程）

- 未写 problem/idea  
- 未改 vocab.yaml  
- 未外搜  
