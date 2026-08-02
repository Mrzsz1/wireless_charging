from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path


TOOLS = Path(__file__).resolve().parents[1] / "tools"
sys.path.insert(0, str(TOOLS))

import domain_keywords  # noqa: E402


class DomainKeywordsTest(unittest.TestCase):
    def test_project_keyword_contract(self) -> None:
        records, errors = domain_keywords.collect(domain_keywords.DEFAULT_SOURCES)
        errors.extend(domain_keywords.validate_map(records, domain_keywords.DEFAULT_MAP))
        self.assertEqual([], errors)
        stats = domain_keywords.summary(records)
        self.assertEqual(21, stats["source_count"])
        self.assertEqual(20, stats["source_with_keywords_count"])
        self.assertEqual(90, stats["paper_keyword_occurrence_count"])

    def test_not_found_cannot_have_keywords(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / "src-invalid.md"
            source.write_text(
                "---\npaper_keywords: [\"invented\"]\nkeyword_source: not_found\n---\n",
                encoding="utf-8",
            )
            _, errors = domain_keywords.collect(Path(directory))
            self.assertTrue(any("not_found" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
