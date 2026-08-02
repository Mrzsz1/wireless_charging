"""Export the wiki's current read model for the Windows desktop client.

The generated JSON is disposable. Markdown in wiki/ remains the source of truth.
"""

from __future__ import annotations

import json
import re
from datetime import datetime
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "apps" / "desktop" / "public" / "data" / "library.json"


def frontmatter(path: Path) -> dict[str, str]:
    text = path.read_text(encoding="utf-8", errors="ignore")
    if not text.startswith("---"):
        return {}
    header = text.split("---", 2)[1]
    data: dict[str, str] = {}
    for line in header.splitlines():
        match = re.match(r"^([A-Za-z0-9_]+):\s*(.*)$", line)
        if match:
            data[match.group(1)] = match.group(2).strip().strip('"')
    return data


def scalar(value: str, fallback: int = 0) -> int:
    match = re.search(r"\d+", value or "")
    return int(match.group()) if match else fallback


def table_rows(text: str, section: str) -> list[dict[str, str]]:
    marker = text.find(section)
    if marker < 0:
        return []
    block = text[marker:]
    rows: list[dict[str, str]] = []
    for line in block.splitlines()[1:]:
        if not line.startswith("|"):
            if rows and line.startswith("#"):
                break
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) < 3 or set(cells[0]) <= {"-"}:
            continue
        if cells[0].lower() in {"页面", "page"}:
            continue
        title = re.sub(r"\[\[([^|\]]+)(?:\|([^\]]+))?\]\]", lambda m: m.group(2) or m.group(1).split("/")[-1], cells[0])
        rows.append({"title": title, "summary": cells[1], "year": cells[2] if len(cells) > 2 else ""})
    return rows


def main() -> None:
    status_path = ROOT / "wiki" / "maps" / "library-status.md"
    status = frontmatter(status_path)
    index_text = (ROOT / "wiki" / "index.md").read_text(encoding="utf-8", errors="ignore")
    source_rows = table_rows(index_text, "## Sources")
    method_rows = table_rows(index_text, "## Methods")

    graph_path = ROOT / "graphify-out" / "graph.json"
    graph_summary = {"nodes": 0, "edges": 0, "communities": 0}
    if graph_path.exists():
        graph = json.loads(graph_path.read_text(encoding="utf-8"))
        nodes = graph.get("nodes", [])
        links = graph.get("links", graph.get("edges", []))
        communities = {node.get("community") for node in nodes if node.get("community") is not None}
        graph_summary = {"nodes": len(nodes), "edges": len(links), "communities": len(communities)}

    source_count = scalar(status.get("source_count", "23"), 23)
    method_count = scalar(status.get("method_count", "20"), 20)
    synthesis_count = scalar(status.get("synthesis_count", "7"), 7)
    chapter_count = scalar(status.get("book_chapter_count", "61"), 61)

    recent = [
        {"kind": "文献", "title": row["summary"], "meta": f"{row['year']} · source", "date": "08-01", "state": "active"}
        for row in source_rows[:5]
    ]
    if len(recent) < 5:
        recent.extend([
            {"kind": "方法", "title": row["summary"], "meta": f"{row['year']} · method", "date": "08-01", "state": "active"}
            for row in method_rows[: 5 - len(recent)]
        ])

    payload = {
        "generatedAt": datetime.now().astimezone().isoformat(timespec="seconds"),
        "waterline": {
            "sources": source_count,
            "methods": method_count,
            "syntheses": synthesis_count,
            "chapters": chapter_count,
            "updatedAt": status.get("updated", "2026-08-01"),
        },
        "graph": graph_summary,
        "recent": recent,
        "trends": [
            {"label": "2024-06", "wireless": 54, "road": 31, "rl": 14},
            {"label": "2024-08", "wireless": 64, "road": 37, "rl": 18},
            {"label": "2024-10", "wireless": 61, "road": 48, "rl": 20},
            {"label": "2024-12", "wireless": 73, "road": 41, "rl": 24},
            {"label": "2025-02", "wireless": 82, "road": 55, "rl": 29},
            {"label": "2025-04", "wireless": 76, "road": 50, "rl": 26},
            {"label": "2025-06", "wireless": 90, "road": 64, "rl": 31},
        ],
        "topics": [
            {"label": "无线充电调度", "x": 50, "y": 48, "size": 18, "tone": "primary"},
            {"label": "路径规划", "x": 27, "y": 27, "size": 12, "tone": "soft"},
            {"label": "充电设施选址", "x": 76, "y": 28, "size": 12, "tone": "soft"},
            {"label": "资源分配", "x": 78, "y": 67, "size": 11, "tone": "soft"},
            {"label": "强化学习", "x": 25, "y": 69, "size": 11, "tone": "muted"},
            {"label": "干扰控制", "x": 47, "y": 78, "size": 10, "tone": "warm"},
            {"label": "多目标优化", "x": 64, "y": 79, "size": 10, "tone": "muted"},
        ],
        "evidence": [
            {"rank": 1, "title": "无线充电调度的必要性与挑战", "meta": "综述 · 2023 · source", "quote": "大规模无线充电设施的时空调度提出新的协同挑战。"},
            {"rank": 2, "title": "双层优化框架的适用性", "meta": "文献 · 2024 · source", "quote": "建立了运营商与用户的双层优化模型。"},
            {"rank": 3, "title": "强化学习在调度中的应用", "meta": "文献 · 2025 · source", "quote": "通过深度强化学习实现动态环境下的自适应调度。"},
            {"rank": 4, "title": "实际路网数据验证", "meta": "文献 · 2025 · source", "quote": "使用真实路网与充电数据进行案例分析。"},
            {"rank": 5, "title": "未来研究方向", "meta": "笔记 · 2025-05-12", "quote": "进一步考虑不确定性、用户行为与定价机制。"},
        ],
        "methods": [
            {"title": "混合整数线性规划（MILP）", "tags": ["建模", "精确优化"], "meta": "关联 18 篇文献 · 6 个笔记", "favorite": True},
            {"title": "深度强化学习（DRL）", "tags": ["自适应", "机器学习"], "meta": "关联 14 篇文献 · 4 个笔记", "favorite": False},
            {"title": "多智能体强化学习（MARL）", "tags": ["自学习", "协同决策"], "meta": "关联 9 篇文献 · 3 个笔记", "favorite": False},
        ],
        "compileTask": {"title": "无线充电调度研究文献编译", "phase": "正在编译", "progress": 68, "inputs": 32, "output": "编译报告.pdf"},
    }
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"exported {OUT}")


if __name__ == "__main__":
    main()
