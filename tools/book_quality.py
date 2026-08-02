#!/usr/bin/env python3
"""Measure chapter coverage against Poppler's local text extraction baseline.

This is a reproducible gate, not a claim that OCR is mathematically correct:
token recall/precision, page/chapter coverage, and missing chapters are written
to a JSON report.  A human review is still required for equations and figures.
"""
from __future__ import annotations

import argparse
import collections
import json
import re
import subprocess
import tempfile
import unicodedata
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]


def tokens(text: str) -> list[str]:
    text = unicodedata.normalize("NFKC", text).lower()
    # Keep words and numeric literals; punctuation/layout is intentionally
    # excluded so the metric is stable across Markdown renderers.
    return re.findall(r"[a-z]+|\d+(?:\.\d+)?", text)


def baseline(pdf: Path, start: int, end: int) -> str:
    with tempfile.NamedTemporaryFile(suffix=".txt", delete=False) as handle:
        out = Path(handle.name)
    subprocess.run(["pdftotext", "-layout", "-f", str(start), "-l", str(end), str(pdf), str(out)], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    text = out.read_text(encoding="utf-8", errors="replace")
    out.unlink(missing_ok=True)
    return text


def score(reference: str, candidate: str) -> dict[str, Any]:
    ref = collections.Counter(tokens(reference))
    got = collections.Counter(tokens(candidate))
    overlap = sum(min(n, got[t]) for t, n in ref.items())
    candidate_overlap = sum(min(n, ref[t]) for t, n in got.items())
    return {
        "reference_tokens": sum(ref.values()),
        "candidate_tokens": sum(got.values()),
        "token_recall": round(overlap / max(1, sum(ref.values())), 6),
        "token_precision": round(candidate_overlap / max(1, sum(got.values())), 6),
        "reference_chars": len(reference),
        "candidate_chars": len(candidate),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--work-dir", type=Path, default=ROOT / "work" / "core-books")
    parser.add_argument("--canonical-dir", type=Path, default=ROOT / "raw" / "canonical")
    parser.add_argument("--threshold", type=float, default=0.95)
    args = parser.parse_args()
    results: dict[str, Any] = {"schema": "core-book-quality-v1", "threshold": args.threshold, "books": []}
    for book_id, pdf_name in [("approximation-algorithms", "Approximation_Algorithms.pdf"), ("algorithmic-game-theory", "Algorithmic_Game_Theory.pdf")]:
        index = json.loads((args.canonical_dir / book_id / "chapter-index.json").read_text(encoding="utf-8"))
        rows = []
        for chapter in index["chapters"]:
            start, end = int(chapter["source_page_start"]), int(chapter["source_page_end"])
            candidate = (ROOT / chapter["path"]).read_text(encoding="utf-8", errors="replace")
            ref = baseline(args.work_dir / "inputs" / pdf_name, start, end)
            metrics = score(ref, candidate)
            metrics.update({"chapter_id": chapter["chapter_id"], "chapter_title": chapter["chapter_title"], "path": chapter["path"], "source_pages": [start, end]})
            rows.append(metrics)
        recalls = [r["token_recall"] for r in rows]
        precisions = [r["token_precision"] for r in rows]
        book = {
            "book_id": book_id,
            "chapter_count": len(rows),
            "chapters_measured": len(rows),
            "min_token_recall": min(recalls),
            "mean_token_recall": round(sum(recalls) / len(recalls), 6),
            "min_token_precision": min(precisions),
            "mean_token_precision": round(sum(precisions) / len(precisions), 6),
            "passes_95_recall": min(recalls) >= args.threshold,
            "passes_95_precision": min(precisions) >= args.threshold,
            "chapters": rows,
        }
        results["books"].append(book)
    results["passes_95_all"] = all(b["passes_95_recall"] and b["passes_95_precision"] for b in results["books"])
    out = args.canonical_dir / "core-books-quality.json"
    out.write_text(json.dumps(results, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"passes_95_all": results["passes_95_all"], "books": [{k: b[k] for k in ("book_id", "chapter_count", "min_token_recall", "mean_token_recall", "min_token_precision", "mean_token_precision", "passes_95_recall", "passes_95_precision")} for b in results["books"]]}, ensure_ascii=False, indent=2))
    return 0 if results["passes_95_all"] else 2


if __name__ == "__main__":
    raise SystemExit(main())
