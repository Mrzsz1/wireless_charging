from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


TOOLS = Path(__file__).resolve().parents[1] / "tools"
sys.path.insert(0, str(TOOLS))
SPEC = importlib.util.spec_from_file_location("paper_triage", TOOLS / "paper_triage.py")
assert SPEC and SPEC.loader
paper_triage = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = paper_triage
SPEC.loader.exec_module(paper_triage)


class PaperTriageTests(unittest.TestCase):
    def test_parse_indices_supports_ranges(self) -> None:
        self.assertEqual(paper_triage.parse_indices("1,3-5", 5), {1, 3, 4, 5})
        with self.assertRaises(paper_triage.TriageError):
            paper_triage.parse_indices("6", 5)

    def test_select_updates_manifest_and_materializes_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            run = root / "search-20260714-000000"
            run.mkdir()
            manifest = run / "results.json"
            manifest.write_text(
                json.dumps(
                    {
                        "kind": "paper_discovery_candidates",
                        "retrieved_at": "2026-07-14T00:00:00+00:00",
                        "providers": ["arxiv"],
                        "queries": [{"label": "test", "arxiv": "all:test", "openalex": "test"}],
                        "provider_counts": {"arxiv": 1},
                        "cache_hits": 0,
                        "errors": [],
                        "papers": [{"title": "Candidate Paper", "providers": ["arxiv"], "year": 2024}],
                    }
                ),
                encoding="utf-8",
            )
            papers_root = root / "papers"
            code = paper_triage.main(
                [str(manifest), "--select", "1", "--note", "relevant", "--papers-root", str(papers_root)]
            )
            self.assertEqual(code, 0)
            payload = json.loads(manifest.read_text(encoding="utf-8"))
            item = payload["papers"][0]
            self.assertEqual(item["acquisition_method"], "auto_discovery")
            self.assertEqual(item["triage_status"], "selected")
            self.assertTrue(item["selected_by_user"])
            self.assertEqual(item["triage_note"], "relevant")
            metadata_files = list(papers_root.glob("*/metadata.json"))
            self.assertEqual(len(metadata_files), 1)
            metadata = json.loads(metadata_files[0].read_text(encoding="utf-8"))
            self.assertIn("not canonical", metadata["boundary"])

            code = paper_triage.main(
                [str(manifest), "--promote", "1", "--papers-root", str(papers_root)]
            )
            self.assertEqual(code, 0)
            promoted = json.loads(manifest.read_text(encoding="utf-8"))["papers"][0]
            self.assertEqual("promoted", promoted["triage_status"])
            self.assertTrue(promoted["canonicalized_at"])
            metadata_files = list(papers_root.glob("*/metadata.json"))
            self.assertEqual(1, len(metadata_files))
            metadata = json.loads(metadata_files[0].read_text(encoding="utf-8"))
            self.assertIn("promoted to canonical", metadata["boundary"])


if __name__ == "__main__":
    unittest.main()
