from __future__ import annotations

import hashlib
import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
sys.path.insert(0, str(TOOLS))

SPEC = importlib.util.spec_from_file_location(
    "qa_heldout_workflow", TOOLS / "qa_heldout_workflow.py"
)
assert SPEC and SPEC.loader
workflow = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = workflow
SPEC.loader.exec_module(workflow)

ACCURACY_TEST_SPEC = importlib.util.spec_from_file_location(
    "qa_accuracy_fixture", ROOT / "tests" / "test_qa_accuracy_eval.py"
)
assert ACCURACY_TEST_SPEC and ACCURACY_TEST_SPEC.loader
fixture = importlib.util.module_from_spec(ACCURACY_TEST_SPEC)
sys.modules[ACCURACY_TEST_SPEC.name] = fixture
ACCURACY_TEST_SPEC.loader.exec_module(fixture)


def frozen_dataset() -> dict[str, object]:
    cases = [
        {
            "id": f"case-{index}",
            "type": "direct_factual",
            "question": f"synthetic fixture question {index}",
        }
        for index in range(1, 31)
    ]
    digest = hashlib.sha256(
        json.dumps(cases, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
    ).hexdigest()
    return {
        "version": "fixture-v1",
        "dataset_role": "production_accuracy",
        "split": "heldout",
        "status": "frozen",
        "minimum_case_count": 30,
        "curation": {
            "independent": True,
            "curator_id_hash": "d" * 64,
            "frozen_at": "2026-08-26T00:00:00Z",
            "cases_sha256": digest,
        },
        "cases": cases,
    }


class QaHeldoutWorkflowTests(unittest.TestCase):
    def test_curator_template_has_fifty_empty_independent_slots(self) -> None:
        template = workflow.curator_template()
        self.assertEqual(len(template["cases"]), 50)
        self.assertFalse(template["independent"])
        self.assertTrue(all(not case["question"] for case in template["cases"]))
        self.assertEqual(
            template["allowedTypes"],
            list(workflow.accuracy.heldout_contract.CONTRACT["allowedTypes"]),
        )

    def test_freeze_rejects_blank_or_non_independent_draft(self) -> None:
        template = workflow.curator_template()
        with self.assertRaises(workflow.HeldoutWorkflowError):
            workflow.freeze_draft(template, "2026-08-26T00:00:00Z")

    def test_blind_export_hides_system_verdict_and_manifest(self) -> None:
        bundle = workflow.blind_review_bundle(
            frozen_dataset()["cases"][0], fixture.run_fixture()
        )
        serialized = json.dumps(bundle)
        self.assertNotIn("runManifest", bundle)
        self.assertNotIn("verificationStatus", serialized)
        self.assertNotIn("systemVerification", serialized)
        self.assertTrue(bundle["reviewerInstructions"]["systemVerdictHidden"])

    def test_same_run_derives_heldout_grounding_and_open_research(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            runs = root / "runs"
            reviews = root / "reviews"
            runs.mkdir()
            reviews.mkdir()
            dataset = root / "dataset.json"
            dataset.write_text(json.dumps(frozen_dataset()), encoding="utf-8")
            for case in frozen_dataset()["cases"]:
                run = fixture.run_fixture()
                run["question"] = case["question"]
                (runs / f"{case['id']}.json").write_text(
                    json.dumps(run), encoding="utf-8"
                )
                review = fixture.review_fixture()
                review["case_id"] = case["id"]
                (reviews / f"{case['id']}.json").write_text(
                    json.dumps(review), encoding="utf-8"
                )
            metrics = workflow.derive_metrics(dataset, runs, reviews)
        self.assertEqual(set(metrics), {"heldout", "grounding", "open_research"})
        seals = {payload["sourceRun"] for payload in metrics.values()}
        self.assertEqual(len(seals), 1)
        self.assertEqual(metrics["heldout"]["factualPrecision"], 1.0)
        self.assertEqual(metrics["grounding"]["citationCorrectness"], 1.0)


if __name__ == "__main__":
    unittest.main()
