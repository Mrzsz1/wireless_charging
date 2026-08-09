from __future__ import annotations

import importlib.util
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
sys.path.insert(0, str(TOOLS))
SPEC = importlib.util.spec_from_file_location("literature_ingest", TOOLS / "literature_ingest.py")
assert SPEC and SPEC.loader
literature_ingest = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = literature_ingest
SPEC.loader.exec_module(literature_ingest)


def repository(root: Path) -> None:
    root.mkdir(parents=True, exist_ok=True)
    (root / "AGENTS.md").write_text("fixture", encoding="utf-8")
    (root / "wiki" / "sources").mkdir(parents=True)
    (root / "raw" / "canonical").mkdir(parents=True)
    (root / "raw" / "inbox" / "auto-discovered" / "runs").mkdir(parents=True)


def manifest(root: Path, papers: list[dict]) -> Path:
    path = root / "raw" / "inbox" / "auto-discovered" / "runs" / "search-20260809-120000" / "results.json"
    path.parent.mkdir(parents=True)
    path.write_text(
        json.dumps(
            {
                "kind": "paper_discovery_candidates",
                "retrieved_at": "2026-08-09T12:00:00+00:00",
                "providers": ["arxiv"],
                "papers": papers,
            },
            ensure_ascii=False,
        ),
        encoding="utf-8",
    )
    return path


def candidate(**overrides):
    base = {
        "title": "Online Wireless Charging Scheduling",
        "authors": ["A"],
        "year": 2026,
        "doi": "10.1000/demo",
        "arxiv_id": "",
        "pdf_url": "https://example.test/demo.pdf",
        "is_open_access": True,
        "providers": ["arxiv"],
        "matched_queries": ["wireless charging scheduling"],
        "score": 9.0,
        "title_matches": ["scheduling"],
        "abstract_matches": [],
        "triage_status": "pending",
    }
    base.update(overrides)
    return base


class LiteratureIngestTests(unittest.TestCase):
    def test_cli_forces_utf8_when_parent_console_uses_gbk(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            repository(root)
            manifest(root, [candidate(title="中文无线充电调度")])
            environment = os.environ.copy()
            environment["PYTHONIOENCODING"] = "gbk"
            completed = subprocess.run(
                [
                    sys.executable,
                    str(TOOLS / "literature_ingest.py"),
                    "--repository",
                    str(root),
                    "list-candidates",
                    "--json",
                ],
                capture_output=True,
                check=False,
                env=environment,
            )
            self.assertEqual(completed.returncode, 0, completed.stderr.decode("utf-8", errors="replace"))
            payload = json.loads(completed.stdout.decode("utf-8"))
            self.assertEqual(payload["candidates"][0]["title"], "中文无线充电调度")

    def test_candidate_id_is_stable_across_metadata_changes(self) -> None:
        first = candidate(title="Title A")
        second = candidate(title="Changed title", authors=["B"])
        self.assertEqual(
            literature_ingest.stable_candidate_id(first),
            literature_ingest.stable_candidate_id(second),
        )

    def test_qualification_is_explainable_and_obeys_score_boundary(self) -> None:
        result = literature_ingest.qualification(candidate(score=8.0), [], {"minScore": 8.0})
        self.assertTrue(result["eligible"])
        self.assertEqual(
            {reason["code"] for reason in result["reasons"]},
            {"topic", "score", "title_match", "identifier", "open_pdf", "duplicate"},
        )
        failed = literature_ingest.qualification(candidate(score=7.99), [], {"minScore": 8.0})
        self.assertFalse(failed["eligible"])
        self.assertFalse(next(reason for reason in failed["reasons"] if reason["code"] == "score")["passed"])

    def test_duplicate_source_blocks_automatic_qualification(self) -> None:
        record = literature_ingest.ExistingRecord(
            "doi", "10.1000/demo", "sources/existing", "wiki/sources/existing.md", "Existing"
        )
        result = literature_ingest.qualification(candidate(), [record], {"minScore": 8.0})
        self.assertFalse(result["eligible"])
        self.assertEqual(result["duplicates"][0]["kind"], "doi")

    def test_local_pdf_hash_duplicate_blocks_qualification(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            repository(root)
            canonical = root / "raw" / "canonical" / "existing" / "paper.pdf"
            canonical.parent.mkdir(parents=True)
            canonical.write_bytes(b"%PDF-1.4\nsame")
            local = root / "raw" / "inbox" / "candidate.pdf"
            local.parent.mkdir(parents=True, exist_ok=True)
            local.write_bytes(canonical.read_bytes())
            item = candidate(doi="10.1000/different", title="Different title", local_pdf=local.relative_to(root).as_posix())
            result = literature_ingest.qualification(
                {**item, "_repository": root},
                literature_ingest.existing_records(root),
                {"minScore": 8.0},
            )
            self.assertFalse(result["eligible"])
            self.assertTrue(any(match["kind"] == "sha256" for match in result["duplicates"]))

    def test_old_manifest_is_migrated_and_triage_is_persisted(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            repository(root)
            path = manifest(root, [candidate(title_matches=None)])
            # Simulate an old manifest by removing every new field.
            payload = json.loads(path.read_text(encoding="utf-8"))
            payload["papers"][0].pop("title_matches")
            payload["papers"][0].pop("triage_status")
            path.write_text(json.dumps(payload), encoding="utf-8")
            items = literature_ingest.collect_candidates(root, migrate=True)
            self.assertEqual(len(items), 1)
            candidate_id = items[0]["candidate_id"]
            self.assertTrue(candidate_id)
            self.assertEqual(literature_ingest.update_triage(root, {candidate_id}, "rejected", "not relevant"), 1)
            saved = json.loads(path.read_text(encoding="utf-8"))
            self.assertEqual(saved["papers"][0]["triage_status"], "rejected")
            self.assertEqual(saved["papers"][0]["manual_note"], "not relevant")

    def test_manual_batch_detects_file_change(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "repo"
            source = Path(temp) / "paper.pdf"
            repository(root)
            source.write_bytes(b"%PDF-1.4\nfixture")
            stat = source.stat()
            payload = {
                "batchId": "batch",
                "files": [
                    {
                        "path": str(source),
                        "size": stat.st_size,
                        "mtimeNs": stat.st_mtime_ns,
                        "sha256": literature_ingest.sha256_file(source),
                        "selected": True,
                    }
                ],
            }
            source.write_bytes(b"%PDF-1.4\nchanged")
            with self.assertRaises(literature_ingest.LiteratureIngestError):
                literature_ingest.staged_manual_files(root, payload)

    def test_fixture_run_records_all_manual_stages(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            root = base / "repo"
            repository(root)
            source = base / "paper.pdf"
            source.write_bytes(b"%PDF-1.4\nfixture")
            stat = source.stat()
            run_manifest = base / "run.json"
            run_manifest.write_text(
                json.dumps(
                    {
                        "kind": "literature_ingest_run",
                        "mode": "manual",
                        "batchId": "fixture",
                        "files": [
                            {
                                "path": str(source),
                                "size": stat.st_size,
                                "mtimeNs": stat.st_mtime_ns,
                                "sha256": literature_ingest.sha256_file(source),
                                "selected": True,
                            }
                        ],
                    }
                ),
                encoding="utf-8",
            )
            fixture = base / "fixture"
            result = literature_ingest.run_ingest(root, run_manifest, fixture)
            self.assertFalse(result["failed"])
            stages = [json.loads(line)["stage"] for line in (fixture / "stages.jsonl").read_text(encoding="utf-8").splitlines()]
            self.assertEqual(stages, ["parse", "compile_a", "lint", "graphify_update", "rebuild_snapshot"])


if __name__ == "__main__":
    unittest.main()
