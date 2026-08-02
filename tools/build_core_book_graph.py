#!/usr/bin/env python3
"""Build a compact deterministic graph for the two chapter corpora."""
from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "graphify-out" / "core-books"


def main() -> int:
    nodes = []
    links = []
    for book_id in ("algorithmic-game-theory", "approximation-algorithms"):
        idx = json.loads((ROOT / "raw/canonical" / book_id / "chapter-index.json").read_text(encoding="utf-8"))
        book_node = f"book:{book_id}"
        nodes.append({"id": book_node, "label": book_id, "type": "book", "path": f"raw/canonical/{book_id}"})
        for chapter in idx["chapters"]:
            cid = f"{book_id}:{chapter['chapter_id']}"
            nodes.append({"id": cid, "label": chapter["chapter_title"], "type": "book-chapter", "book_id": book_id, "path": chapter["path"], "pdf_pages": [chapter["source_page_start"], chapter["source_page_end"]]})
            links.append({"source": book_node, "target": cid, "relation": "HAS_CHAPTER"})
            text = (ROOT / chapter["path"]).read_text(encoding="utf-8", errors="replace").lower()
            for term in sorted(set(re.findall(r"\b(?:nash|equilibrium|mechanism|incentive|utility|approximation|rounding|primal|dual|scheduling|lp|sdp|hardness)\b", text))):
                tid = f"term:{term}"
                if not any(n["id"] == tid for n in nodes):
                    nodes.append({"id": tid, "label": term, "type": "core-book-term"})
                links.append({"source": cid, "target": tid, "relation": "MENTIONS"})
    graph = {"schema": "core-book-graph-v1", "directed": True, "nodes": nodes, "links": links}
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "graph.json").write_text(json.dumps(graph, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (OUT / "GRAPH_REPORT.md").write_text(f"# Core-book graph\n\n- books: 2\n- chapters: {sum(1 for n in nodes if n['type']=='book-chapter')}\n- links: {len(links)}\n- generated: 2026-08-01\n", encoding="utf-8")
    print(json.dumps({"nodes": len(nodes), "links": len(links), "out": str((OUT / 'graph.json').relative_to(ROOT))}, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
