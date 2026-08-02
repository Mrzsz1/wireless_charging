from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).resolve().parents[1] / "tools" / "paper_search.py"
SPEC = importlib.util.spec_from_file_location("paper_search", SCRIPT)
assert SPEC and SPEC.loader
paper_search = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = paper_search
SPEC.loader.exec_module(paper_search)


ARXIV_XML = b'''<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom" xmlns:arxiv="http://arxiv.org/schemas/atom">
  <entry>
    <id>http://arxiv.org/abs/2401.01234v2</id>
    <updated>2024-02-01T00:00:00Z</updated>
    <published>2024-01-02T00:00:00Z</published>
    <title>Wireless Charging Scheduling</title>
    <summary>A resource allocation method.</summary>
    <author><name>Alice Zhang</name></author>
    <arxiv:doi>10.1000/example</arxiv:doi>
    <link href="https://arxiv.org/abs/2401.01234" rel="alternate" type="text/html"/>
    <link title="pdf" href="https://arxiv.org/pdf/2401.01234" rel="related" type="application/pdf"/>
  </entry>
</feed>'''


class PaperSearchTests(unittest.TestCase):
    def test_shared_key_file_loads_all_labeled_providers(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            key_file = Path(temp_dir) / "keys.txt"
            key_file.write_text(
                "legacy-mineru-token\n"
                "Tavily_api_key = tavily-secret\n"
                "SERPAPI_API_KEY: serp-secret\n"
                "openalex_apikey=openalex-secret\n",
                encoding="utf-8",
            )
            self.assertEqual(
                paper_search.read_provider_key("tavily", key_file, environ={}),
                "tavily-secret",
            )
            self.assertEqual(
                paper_search.read_provider_key("serpapi", key_file, environ={}),
                "serp-secret",
            )
            self.assertEqual(
                paper_search.read_provider_key("openalex", key_file, environ={}),
                "openalex-secret",
            )

    def test_parse_arxiv_feed(self) -> None:
        papers = paper_search.parse_arxiv_feed(ARXIV_XML, "charging")
        self.assertEqual(len(papers), 1)
        self.assertEqual(papers[0].arxiv_id, "2401.01234")
        self.assertEqual(papers[0].doi, "10.1000/example")
        self.assertEqual(papers[0].authors, ["Alice Zhang"])
        self.assertTrue(papers[0].pdf_url.endswith("2401.01234"))

    def test_parse_openalex_and_rebuild_abstract(self) -> None:
        payload = {
            "results": [{
                "id": "https://openalex.org/W123",
                "doi": "https://doi.org/10.1000/example",
                "display_name": "Wireless Charging Scheduling",
                "publication_year": 2024,
                "publication_date": "2024-01-02",
                "abstract_inverted_index": {"resource": [1], "A": [0], "method": [2]},
                "authorships": [{"author": {"display_name": "Alice Zhang"}}],
                "open_access": {"is_oa": True},
                "best_oa_location": {
                    "landing_page_url": "https://example.org/work",
                    "pdf_url": "https://example.org/work.pdf",
                    "license": "cc-by"
                }
            }]
        }
        paper = paper_search.parse_openalex_payload(payload, "charging")[0]
        self.assertEqual(paper.abstract, "A resource method")
        self.assertEqual(paper.doi, "10.1000/example")
        self.assertTrue(paper.is_open_access)

    def test_parse_tavily_academic_result(self) -> None:
        payload = {
            "results": [{
                "title": "Wireless Charging Scheduling",
                "url": "https://arxiv.org/abs/2401.01234",
                "content": "A scheduling method with DOI 10.1000/example.",
                "score": 0.9,
                "published_date": "2024-01-02",
            }]
        }
        paper = paper_search.parse_tavily_payload(payload, "charging")[0]
        self.assertEqual(paper.arxiv_id, "2401.01234")
        self.assertEqual(paper.doi, "10.1000/example")
        self.assertEqual(paper.providers, ["tavily"])

    def test_parse_serpapi_google_scholar_result(self) -> None:
        payload = {
            "organic_results": [{
                "title": "Wireless Charging Scheduling",
                "result_id": "scholar-1",
                "link": "https://example.org/paper",
                "snippet": "A resource allocation method.",
                "publication_info": {"summary": "A Zhang, B Li - Journal, 2024 - example.org"},
                "resources": [{
                    "title": "example.org",
                    "file_format": "PDF",
                    "link": "https://example.org/paper.pdf",
                }],
            }]
        }
        paper = paper_search.parse_serpapi_payload(payload, "charging")[0]
        self.assertEqual(paper.year, 2024)
        self.assertEqual(paper.authors, ["A Zhang", "B Li"])
        self.assertEqual(paper.pdf_url, "https://example.org/paper.pdf")
        self.assertTrue(paper.is_open_access)

    def test_deduplicate_merges_providers_by_doi(self) -> None:
        first = paper_search.Paper(
            title="Wireless Charging Scheduling",
            doi="10.1000/example",
            providers=["arxiv"],
            arxiv_id="2401.01234",
        )
        second = paper_search.Paper(
            title="Wireless charging scheduling",
            doi="https://doi.org/10.1000/EXAMPLE",
            providers=["openalex"],
            abstract="Longer abstract",
        )
        merged = paper_search.deduplicate([first, second])
        self.assertEqual(len(merged), 1)
        self.assertEqual(merged[0].providers, ["arxiv", "openalex"])
        self.assertEqual(merged[0].abstract, "Longer abstract")

    def test_cache_key_does_not_include_api_key(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            cache = paper_search.ResponseCache(Path(temp_dir))
            first = cache._path("openalex", {"search": "x", "api_key": "secret-1"}, "json")
            second = cache._path("openalex", {"search": "x", "api_key": "secret-2"}, "json")
            self.assertEqual(first, second)
            self.assertNotIn("secret", str(first))

    def test_save_run_marks_candidates_not_canonical(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            paper = paper_search.Paper(title="Candidate", providers=["arxiv"], score=1.0)
            outcome = paper_search.SearchOutcome([paper], [], {"arxiv": 1})
            query = paper_search.SearchQuery("test", "all:test", "test")
            run_dir, downloaded, errors = paper_search.save_run(
                outcome, [query], ["arxiv"], Path(temp_dir), 10, False, 1
            )
            payload = json.loads((run_dir / "results.json").read_text(encoding="utf-8"))
            report = (run_dir / "README.md").read_text(encoding="utf-8")
            self.assertEqual(payload["discovery_status"], "candidate")
            self.assertEqual(payload["acquisition_method"], "auto_discovery")
            self.assertEqual(payload["triage_counts"]["pending"], 1)
            self.assertEqual(payload["papers"][0]["triage_status"], "pending")
            self.assertEqual(payload["papers"][0]["discovered_via"], ["arxiv"])
            self.assertFalse(payload["papers"][0]["selected_by_user"])
            self.assertIn("不是 `raw/canonical`", report)
            self.assertEqual(downloaded, 0)
            self.assertEqual(errors, [])

    def test_previously_seen_identities_reads_manifests(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            root = Path(temp_dir)
            run = root / "search-20260101-000000"
            run.mkdir()
            (run / "results.json").write_text(
                json.dumps({"papers": [{"title": "Known", "doi": "10.1/KNOWN"}]}),
                encoding="utf-8",
            )
            seen = paper_search.previously_seen_identities(root)
            self.assertIn("10.1/known", seen)

    def test_default_discovery_paths_are_separated(self) -> None:
        self.assertEqual(paper_search.DEFAULT_OUTPUT_ROOT.name, "runs")
        self.assertEqual(paper_search.DEFAULT_OUTPUT_ROOT.parent.name, "auto-discovered")
        self.assertEqual(paper_search.DEFAULT_CACHE_ROOT.parent.name, "auto-discovered")
        self.assertNotEqual(paper_search.DEFAULT_OUTPUT_ROOT, paper_search.DEFAULT_CACHE_ROOT)


if __name__ == "__main__":
    unittest.main()
