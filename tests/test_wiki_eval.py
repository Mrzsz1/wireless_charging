from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
SPEC = importlib.util.spec_from_file_location("wiki_eval", TOOLS / "wiki_eval.py")
assert SPEC and SPEC.loader
wiki_eval = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = wiki_eval
SPEC.loader.exec_module(wiki_eval)


class WikiEvalTests(unittest.TestCase):
    def test_repository_gold_contract(self) -> None:
        payload = wiki_eval.load_gold(ROOT / "evals" / "gold_questions.json")
        self.assertEqual(wiki_eval.validate_contract(payload, ROOT / "wiki"), [])
        self.assertEqual(payload["version"], wiki_eval.GOLD_VERSION)
        self.assertEqual(payload["dataset_role"], "development_regression")
        self.assertEqual(payload["split"], "development")
        self.assertTrue(
            all(case["evidence_contract"]["paper_sources"] for case in payload["cases"])
        )

    def test_invalid_type_quota_is_reported(self) -> None:
        payload = wiki_eval.load_gold(ROOT / "evals" / "gold_questions.json")
        payload = json.loads(json.dumps(payload))
        payload["cases"][0]["type"] = "novelty"
        errors = wiki_eval.validate_contract(payload, ROOT / "wiki")
        self.assertTrue(any("类型配额" in error for error in errors))

    def test_answer_check_requires_links_and_waterline(self) -> None:
        payload = wiki_eval.load_gold(ROOT / "evals" / "gold_questions.json")
        one_case = {"cases": [payload["cases"][0]]}
        with tempfile.TemporaryDirectory() as temp_dir:
            answer = Path(temp_dir) / f"{one_case['cases'][0]['id']}.md"
            answer.write_text("没有证据链接。", encoding="utf-8")
            errors = wiki_eval.validate_answers(one_case, Path(temp_dir))
        self.assertTrue(any("wikilink" in error or "答案缺少" in error for error in errors))
        self.assertTrue(any("库水位" in error for error in errors))

    def test_answer_check_requires_must_mention(self) -> None:
        payload = wiki_eval.load_gold(ROOT / "evals" / "gold_questions.json")
        one_case = {"cases": [payload["cases"][0]]}
        with tempfile.TemporaryDirectory() as temp_dir:
            answer = Path(temp_dir) / f"{one_case['cases'][0]['id']}.md"
            answer.write_text("库水位。", encoding="utf-8")
            errors = wiki_eval.validate_answers(one_case, Path(temp_dir))
        self.assertTrue(any("必提概念" in error for error in errors))

    def test_contract_requires_primary_source_location_fields(self) -> None:
        payload = wiki_eval.load_gold(ROOT / "evals" / "gold_questions.json")
        payload = json.loads(json.dumps(payload))
        del payload["cases"][0]["evidence_contract"]["paper_sources"]
        errors = wiki_eval.validate_contract(payload, ROOT / "wiki")
        self.assertTrue(any("paper_sources" in error for error in errors))

    def test_load_gold_rejects_mojibake(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            path = Path(temp_dir) / "bad.json"
            path.write_text('{"cases": [], "note": "���"}', encoding="utf-8")
            with self.assertRaises(wiki_eval.EvalContractError):
                wiki_eval.load_gold(path)

    def test_answer_requires_raw_location_and_boundary(self) -> None:
        payload = wiki_eval.load_gold(ROOT / "evals" / "gold_questions.json")
        one_case = {"cases": [payload["cases"][0]]}
        with tempfile.TemporaryDirectory() as temp_dir:
            answer = Path(temp_dir) / f"{one_case['cases'][0]['id']}.md"
            answer.write_text(
                "库水位：23 篇 source。" + " ".join(one_case["cases"][0]["must_mention"]),
                encoding="utf-8",
            )
            errors = wiki_eval.validate_answers(one_case, Path(temp_dir))
        self.assertTrue(any("原文行号" in error for error in errors))
        self.assertTrue(any("边界" in error for error in errors))

    def test_answer_check_rejects_stale_source_waterline(self) -> None:
        status = (ROOT / "wiki" / "maps" / "library-status.md").read_text(encoding="utf-8-sig")
        source_count = int(next(line.split(":", 1)[1] for line in status.splitlines() if line.startswith("source_count:")))
        payload = {
            "cases": [{
                "id": "stale-waterline",
                "expected_wikilinks": [],
                "waterline_required": True,
                "must_mention": [],
            }]
        }
        with tempfile.TemporaryDirectory() as temp_dir:
            answer = Path(temp_dir) / "stale-waterline.md"
            answer.write_text(f"库水位：{source_count - 1} 篇 source。", encoding="utf-8")
            errors = wiki_eval.validate_answers(payload, Path(temp_dir))
        self.assertTrue(any(f"{source_count} 篇 source" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
