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
            "acceptableMethodFamilies": ["A", "B", "C", "D"] if index == 1 else [],
            "criticalConstraints": ["X", "Y", "Z"] if index == 1 else [],
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
        "candidate_pool": "research_questions_v1.json#split=heldout",
        "candidate_count": 80,
        "candidate_pool_cases_sha256": "e" * 64,
        "curation": {
            "independent": True,
            "curator_id_hash": "d" * 64,
            "frozen_at": "2026-08-26T00:00:00Z",
            "cases_sha256": digest,
        },
        "cases": cases,
    }


def sealed_candidate_pool() -> dict[str, object]:
    intents = list(workflow.accuracy.heldout_contract.CONTRACT["allowedTypes"])
    cases = []
    for index in range(360):
        split = "development" if index < 160 else "regression" if index < 280 else "heldout"
        cases.append(
            {
                "id": f"candidate-{index + 1:03d}",
                "split": split,
                "intent": intents[index % len(intents)],
                "domain": f"domain-{index % 12}",
                "difficulty": "medium" if index % 2 == 0 else "hard",
                "question": f"synthetic sealed candidate question number {index + 1:03d}",
            }
        )
    return {
        "schemaVersion": "research-question-dataset-v1",
        "status": "sealed",
        "totalCount": len(cases),
        "casesSha256": workflow.research_dataset.canonical_cases_sha256(cases),
        "cases": cases,
    }


def valid_draft(pool: dict[str, object]) -> dict[str, object]:
    draft = workflow.curator_template(30)
    draft["independent"] = True
    draft["curatorIdHash"] = "d" * 64
    draft["datasetVersion"] = "fixture-v1"
    candidates = [case for case in pool["cases"] if case["split"] == "heldout"][:30]
    draft["cases"] = [
        {
            "id": candidate["id"],
            "type": candidate["intent"],
            "question": candidate["question"],
            "stratum": "fixture",
            "difficulty": candidate["difficulty"],
            "criticalConstraints": (
                ["X"]
                if index == 0
                else ["expected-constraint"]
                if candidate["intent"] in workflow.OPEN_RESEARCH_TYPES
                else []
            ),
            "acceptableMethodFamilies": ["A"] if index == 0 else [],
            "notesForReviewers": "",
        }
        for index, candidate in enumerate(candidates)
    ]
    return draft


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

    def test_freeze_verifies_sealed_80_candidate_pool_and_case_identity(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            pool_path = Path(temporary) / "research_questions_v1.json"
            pool = sealed_candidate_pool()
            pool_path.write_text(json.dumps(pool), encoding="utf-8")
            frozen = workflow.freeze_draft(
                valid_draft(pool), "2026-08-26T00:00:00Z", pool_path
            )
            self.assertEqual(frozen["candidate_count"], 80)
            self.assertEqual(frozen["candidate_pool_cases_sha256"], pool["casesSha256"])

            for field, value in (
                ("id", "candidate-not-heldout"),
                ("question", "synthetic sealed candidate question drifted"),
                ("type", "comparison"),
            ):
                with self.subTest(field=field):
                    drifted = valid_draft(pool)
                    drifted["cases"][0][field] = value
                    with self.assertRaises(workflow.HeldoutWorkflowError):
                        workflow.freeze_draft(
                            drifted, "2026-08-26T00:00:00Z", pool_path
                        )

            tampered_pool = sealed_candidate_pool()
            tampered_pool["casesSha256"] = "0" * 64
            pool_path.write_text(json.dumps(tampered_pool), encoding="utf-8")
            with self.assertRaises(workflow.HeldoutWorkflowError):
                workflow.freeze_draft(
                    valid_draft(tampered_pool), "2026-08-26T00:00:00Z", pool_path
                )

    def test_blind_export_hides_system_verdict_and_manifest(self) -> None:
        bundle = workflow.blind_review_bundle(
            frozen_dataset()["cases"][0], fixture.run_fixture()
        )
        serialized = json.dumps(bundle)
        self.assertNotIn("runManifest", bundle)
        self.assertNotIn("verificationStatus", serialized)
        self.assertNotIn("systemVerification", serialized)
        self.assertTrue(bundle["reviewerInstructions"]["systemVerdictHidden"])
        self.assertEqual(bundle["expectedMethodFamilies"], ["A", "B", "C", "D"])
        self.assertEqual(bundle["expectedCriticalConstraints"], ["X", "Y", "Z"])

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
                for primary in review["primary_reviews"]:
                    primary["method_coverage"] = [
                        {
                            "method_family": method,
                            "verdict": "covered" if method == "A" else "not_covered",
                        }
                        for method in case["acceptableMethodFamilies"]
                    ]
                    primary["constraint_coverage"] = [
                        {
                            "constraint": constraint,
                            "verdict": "preserved" if constraint == "X" else "not_preserved",
                        }
                        for constraint in case["criticalConstraints"]
                    ]
                (reviews / f"{case['id']}.json").write_text(
                    json.dumps(review), encoding="utf-8"
                )
            metrics = workflow.derive_metrics(dataset, runs, reviews)
        self.assertEqual(set(metrics), {"heldout", "grounding", "open_research"})
        seals = {payload["sourceRun"] for payload in metrics.values()}
        self.assertEqual(len(seals), 1)
        self.assertEqual(metrics["heldout"]["factualPrecision"], 1.0)
        self.assertEqual(metrics["grounding"]["citationCorrectness"], 1.0)
        self.assertEqual(metrics["open_research"]["relevantMethodRecall"], 0.25)
        self.assertAlmostEqual(
            metrics["open_research"]["criticalConstraintPreservation"], 1 / 3
        )


if __name__ == "__main__":
    unittest.main()
