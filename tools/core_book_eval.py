#!/usr/bin/env python3
"""Generate and score deterministic chapter-grounded retrieval checks."""
from __future__ import annotations

import json
from pathlib import Path

from core_reference_search import load, search

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    rows = load()
    queries = []
    for row in rows:
        if row["chapter_id"] == "front-matter":
            continue
        title = row["chapter_title"]
        # Five paraphrase-like lexical probes per chapter. They are seed
        # regression checks; human queries remain necessary for semantic recall.
        for suffix in ("model", "algorithm", "solution", "assumptions", "complexity"):
            queries.append({"id": f"{row['book_id']}-{row['chapter_id']}-{suffix}", "query": f"{title} {suffix}", "expected_book": row["book_id"], "expected_chapter": row["chapter_id"]})
    checks = []
    for q in queries:
        hits = search(q["query"], rows, 5)
        checks.append({**q, "hit": bool(hits), "book_recall": any(h["book_id"] == q["expected_book"] for h in hits), "chapter_hit": any(h["book_id"] == q["expected_book"] and h["chapter_id"] == q["expected_chapter"] for h in hits), "top": hits[:1]})
    by_book = {}
    for book in ("algorithmic-game-theory", "approximation-algorithms"):
        subset = [x for x in checks if x["expected_book"] == book]
        by_book[book] = {"queries": len(subset), "recall_at_5": round(sum(x["book_recall"] for x in subset) / len(subset), 6), "chapter_hit_at_5": round(sum(x["chapter_hit"] for x in subset) / len(subset), 6)}
    report = {"schema": "core-book-retrieval-report-v1", "query_count": len(checks), "minimum_queries_per_book": 100, "books": by_book, "passes_95_book_recall": all(x["recall_at_5"] >= 0.95 for x in by_book.values()), "checks": checks}
    target = ROOT / "evals/core-book-retrieval-report.json"
    target.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"query_count": report["query_count"], "books": by_book, "passes_95_book_recall": report["passes_95_book_recall"]}, ensure_ascii=False, indent=2))
    return 0 if report["passes_95_book_recall"] else 2


if __name__ == "__main__":
    raise SystemExit(main())
