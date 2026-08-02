#!/usr/bin/env python3
"""Search the two core books and emit chapter/page-grounded citations.

No external model or network is needed.  Retrieval is a deterministic lexical
first pass; Luna can use the returned chapter files as the evidence context.
"""
from __future__ import annotations

import argparse
import json
import math
import re
import sys
import unicodedata
from collections import Counter
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8")


def terms(text: str) -> list[str]:
    text = unicodedata.normalize("NFKC", text).lower()
    return re.findall(r"[a-z][a-z0-9-]{1,}|\d+(?:\.\d+)?|[\u4e00-\u9fff]", text)


def load() -> list[dict[str, Any]]:
    rows = []
    for book_id in ("algorithmic-game-theory", "approximation-algorithms"):
        index = json.loads((ROOT / "raw/canonical" / book_id / "chapter-index.json").read_text(encoding="utf-8"))
        for chapter in index["chapters"]:
            chapter["book_id"] = book_id
            chapter["text"] = (ROOT / chapter["path"]).read_text(encoding="utf-8", errors="replace")
            chapter["terms"] = Counter(terms(chapter["chapter_title"] + " " + chapter["text"]))
            rows.append(chapter)
    return rows


def search(query: str, rows: list[dict[str, Any]], limit: int) -> list[dict[str, Any]]:
    q = Counter(terms(query))
    n = len(rows)
    df = Counter(t for row in rows for t in row["terms"])
    out = []
    for row in rows:
        score = 0.0
        matched = []
        length = sum(row["terms"].values()) or 1
        for term, qn in q.items():
            tf = row["terms"].get(term, 0)
            if not tf:
                continue
            matched.append(term)
            idf = math.log((n + 1) / (df[term] + 1)) + 1
            score += idf * ((tf * 2.2) / (tf + 1.2 * (0.75 + 0.25 * length / 10000))) * qn
            if term in terms(row["chapter_title"]):
                score += 2.5
        if score:
            out.append({"score": round(score, 4), "matched_terms": matched, "book_id": row["book_id"], "chapter_id": row["chapter_id"], "chapter_title": row["chapter_title"], "pdf_pages": [row["source_page_start"], row["source_page_end"]], "printed_pages": [row.get("printed_page_start"), row.get("printed_page_end")], "path": row["path"]})
    return sorted(out, key=lambda x: (-x["score"], x["book_id"], x["chapter_id"]))[:limit]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("query")
    parser.add_argument("--limit", type=int, default=8)
    args = parser.parse_args()
    hits = search(args.query, load(), args.limit)
    print(json.dumps({"query": args.query, "hits": hits, "luna_instruction": "仅依据命中的 chapter Markdown 回答；保留 book_id、chapter_id、chapter_title、pdf_pages 引用。"}, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
