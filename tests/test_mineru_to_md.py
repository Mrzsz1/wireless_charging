from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path


SCRIPT = Path(__file__).resolve().parents[1] / "tools" / "mineru_to_md.py"
SPEC = importlib.util.spec_from_file_location("mineru_to_md", SCRIPT)
assert SPEC and SPEC.loader
mineru = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = mineru
SPEC.loader.exec_module(mineru)


class FakeResponse:
    status_code = 200

    def __init__(self, payload=None) -> None:
        self.payload = payload or {}

    def json(self):
        return self.payload


class FakeSession:
    def __init__(self) -> None:
        self.post_headers = None
        self.put_headers = None

    def post(self, url, *, headers, json, timeout):
        self.post_headers = headers
        return FakeResponse(
            {
                "code": 0,
                "msg": "ok",
                "data": {"batch_id": "batch-1", "file_urls": ["https://oss/upload"]},
            }
        )

    def put(self, url, *, data, headers, timeout):
        self.put_headers = headers
        data.read(1)
        return FakeResponse()


class MinerUToMdTests(unittest.TestCase):
    def test_read_api_key_strips_bearer_without_printing_it(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            key_file = Path(temp_dir) / "key.txt"
            key_file.write_text("Bearer secret-token\n", encoding="utf-8")
            self.assertEqual(
                mineru.read_api_key(key_file, environ={}),
                "secret-token",
            )

    def test_read_api_key_keeps_legacy_first_token_in_shared_file(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            key_file = Path(temp_dir) / "keys.txt"
            key_file.write_text(
                "mineru-secret\nTavily_api_key=tavily-secret\n"
                "SERPAPI_API_KEY=serp-secret\nopenalex_apikey=openalex-secret\n",
                encoding="utf-8",
            )
            self.assertEqual(mineru.read_api_key(key_file, environ={}), "mineru-secret")

    def test_add_raw_frontmatter_is_idempotent(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            md_path = root / "full.md"
            pdf_path = root / "paper.pdf"
            md_path.write_text("# Paper\n", encoding="utf-8")
            pdf_path.write_bytes(b"pdf")

            mineru.add_raw_frontmatter(md_path, pdf_path)
            first = md_path.read_text(encoding="utf-8")
            mineru.add_raw_frontmatter(md_path, pdf_path)
            second = md_path.read_text(encoding="utf-8")

            self.assertEqual(first, second)
            self.assertIn("ingest_status: pending_ingest", first)
            self.assertIn("acquisition_method: manual_upload", first)
            self.assertIn("triage_status: promoted", first)
            self.assertTrue(first.endswith("# Paper\n"))

    def test_frontmatter_propagates_auto_discovery_sidecar(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            source_dir = root / "auto-discovered" / "papers" / "candidate"
            source_dir.mkdir(parents=True)
            source_pdf = source_dir / "paper.pdf"
            source_pdf.write_bytes(b"pdf")
            (source_dir / "metadata.json").write_text(
                json.dumps(
                    {
                        "title": "Selected Paper Title",
                        "acquisition_method": "auto_discovery",
                        "discovered_via": ["openalex", "serpapi"],
                        "discovery_run": "raw/inbox/auto-discovered/runs/search-1",
                        "acquired_at": "2026-07-14T00:00:00+00:00",
                    }
                ),
                encoding="utf-8",
            )
            output = root / "canonical"
            output.mkdir()
            md_path = output / "full.md"
            pdf_path = output / "paper.pdf"
            md_path.write_text("# Paper\n", encoding="utf-8")
            pdf_path.write_bytes(b"pdf")

            mineru.add_raw_frontmatter(md_path, pdf_path, source_pdf=source_pdf)
            content = md_path.read_text(encoding="utf-8")
            self.assertIn("acquisition_method: auto_discovery", content)
            self.assertIn('title: "Selected Paper Title"', content)
            self.assertIn('discovered_via: ["openalex", "serpapi"]', content)
            self.assertIn("search-1", content)

    def test_build_plan_uses_selected_metadata_title(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            source_dir = root / "auto-discovered" / "papers" / "candidate"
            source_dir.mkdir(parents=True)
            source_pdf = source_dir / "paper.pdf"
            source_pdf.write_bytes(b"%PDF-test")
            (source_dir / "metadata.json").write_text(
                json.dumps({"title": "A Useful: Paper Title"}), encoding="utf-8"
            )
            output = root / "canonical"

            plan = mineru.build_plan(source_dir, output, force=False)

            self.assertEqual(1, len(plan.jobs))
            self.assertEqual("A_Useful_Paper_Title", plan.jobs[0].output_dir.name)
            self.assertEqual("A_Useful_Paper_Title.pdf", plan.jobs[0].canonical_pdf.name)

    def test_build_plan_reuses_completed_metadata_title_folder(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            source_dir = root / "auto-discovered" / "papers" / "candidate"
            source_dir.mkdir(parents=True)
            source_pdf = source_dir / "paper.pdf"
            source_pdf.write_bytes(b"%PDF-test")
            (source_dir / "metadata.json").write_text(
                json.dumps({"title": "A Useful: Paper Title"}), encoding="utf-8"
            )
            output = root / "canonical"
            canonical_dir = output / "A_Useful_Paper_Title"
            canonical_dir.mkdir(parents=True)
            (canonical_dir / "A_Useful_Paper_Title.pdf").write_bytes(source_pdf.read_bytes())
            (canonical_dir / "full.md").write_text("# Parsed\n", encoding="utf-8")

            plan = mineru.build_plan(source_dir, output, force=False)

            self.assertEqual([], plan.jobs)
            self.assertEqual([(source_pdf.resolve(), "目标目录已有 full.md")], plan.skipped)

    def test_extract_nested_result_and_preserve_images(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            zip_path = root / "result.zip"
            output = root / "output"
            with zipfile.ZipFile(zip_path, "w") as archive:
                archive.writestr("paper/full.md", "# Result\n")
                archive.writestr("paper/images/figure.png", b"png")

            md_path = mineru.extract_result_zip(zip_path, output, force=False)

            self.assertEqual(md_path, output / "full.md")
            self.assertEqual(md_path.read_text(encoding="utf-8"), "# Result\n")
            self.assertEqual((output / "images" / "figure.png").read_bytes(), b"png")

    def test_rejects_zip_path_traversal(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            zip_path = root / "unsafe.zip"
            output = root / "output"
            with zipfile.ZipFile(zip_path, "w") as archive:
                archive.writestr("../outside.txt", "bad")
                archive.writestr("full.md", "# Result\n")

            with self.assertRaises(mineru.MinerUError):
                mineru.extract_result_zip(zip_path, output, force=False)
            self.assertFalse((root / "outside.txt").exists())

    def test_token_is_not_sent_to_signed_upload_url(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            pdf = Path(temp_dir) / "paper.pdf"
            pdf.write_bytes(b"pdf")
            job = mineru.Job(
                source_pdf=pdf,
                output_dir=Path(temp_dir),
                canonical_pdf=pdf,
                data_id="pdf-test",
                api_name="pdf-test-paper.pdf",
            )
            session = FakeSession()
            client = mineru.MinerUClient("secret-token", session=session)

            _, urls = client.create_batch(
                [job],
                model_version="vlm",
                language="en",
                enable_formula=True,
                enable_table=True,
                is_ocr=False,
                page_ranges=None,
                extra_formats=[],
            )
            client.upload(urls[0], pdf)

            self.assertEqual(session.post_headers["Authorization"], "Bearer secret-token")
            self.assertEqual(session.put_headers, {})


if __name__ == "__main__":
    unittest.main()
