from __future__ import annotations

import json
import subprocess
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class CoreBookTests(unittest.TestCase):
    def test_book_manifests_are_chapter_first_and_page_bounded(self) -> None:
        for book_id, expected in [("approximation-algorithms", 30), ("algorithmic-game-theory", 29)]:
            manifest = json.loads((ROOT / "work/core-books" / book_id / "part-manifest.json").read_text(encoding="utf-8"))
            self.assertEqual(expected, manifest["chapter_count"])
            self.assertTrue(all(p["page_count"] <= 180 for p in manifest["parts"]))
            self.assertGreaterEqual(len(manifest["parts"]), expected)


    def test_book_quality_gate_is_above_95_percent(self) -> None:
        report = json.loads((ROOT / "raw/canonical/core-books-quality.json").read_text(encoding="utf-8"))
        self.assertTrue(report["passes_95_all"])
        for book in report["books"]:
            self.assertGreaterEqual(book["min_token_recall"], 0.95)
            self.assertGreaterEqual(book["min_token_precision"], 0.95)


    def test_core_search_returns_page_anchored_hit(self) -> None:
        proc = subprocess.run([sys.executable, str(ROOT / "tools/core_reference_search.py"), "Nash equilibrium mechanism", "--limit", "5"], capture_output=True, text=True, encoding="utf-8", check=True)
        payload = json.loads(proc.stdout)
        self.assertTrue(payload["hits"])
        self.assertTrue(all(hit["chapter_id"] and len(hit["pdf_pages"]) == 2 and hit["path"].endswith(".md") for hit in payload["hits"]))

    def test_retrieval_regression_has_95_percent_book_recall(self) -> None:
        report = json.loads((ROOT / "evals/core-book-retrieval-report.json").read_text(encoding="utf-8"))
        self.assertGreaterEqual(report["query_count"], 200)
        self.assertTrue(report["passes_95_book_recall"])
        for book in report["books"].values():
            self.assertGreaterEqual(book["recall_at_5"], 0.95)
