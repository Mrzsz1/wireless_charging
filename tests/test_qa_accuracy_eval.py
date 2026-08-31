from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "qa_accuracy_eval", ROOT / "tools" / "qa_accuracy_eval.py"
)
assert SPEC and SPEC.loader
qa_accuracy_eval = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = qa_accuracy_eval
SPEC.loader.exec_module(qa_accuracy_eval)

CASE = {
    "id": "case-1",
    "acceptableMethodFamilies": [],
    "criticalConstraints": [],
}


def evidence() -> dict[str, object]:
    return {
        "id": "E1",
        "kind": "paper",
        "tier": "primary_source",
        "title": "Fixture paper",
        "snippet": "The fixture supports the claim.",
        "score": 0.75,
        "rank": 1,
        "pageId": "sources/src-fixture",
        "pageType": "source",
        "sourcePath": "wiki/sources/src-fixture.md",
        "wikilink": "[[src-fixture]]",
        "bookId": "",
        "chapterId": "",
        "physicalPageStart": None,
        "physicalPageEnd": None,
        "markdownPath": "raw/canonical/fixture/full.md",
        "pdfPath": "raw/canonical/fixture/fixture.pdf",
        "nodeId": "",
        "sourceLocation": "原文第 10–12 行",
        "relation": "wiki_source_to_primary",
        "retrievalReason": "query-matched primary section",
    }


def run_fixture() -> dict[str, object]:
    item = evidence()
    visible_body = "fixture claim."
    visible_body_sha256 = hashlib.sha256(visible_body.encode("utf-8")).hexdigest()
    final_claim = {
        "id": "C1",
        "text": "fixture claim [E1].",
        "evidenceIds": ["E1"],
        "claimType": "knowledge_fact",
        "verificationStatus": "supported",
        "confidence": 1.0,
        "verificationMethod": "fixture",
        "alignmentScore": 1.0,
        "reason": "synthetic",
    }
    return {
        "question": "fixture question",
        "answer": "fixture claim.",
        "answerClaims": [
            {
                "claimId": "C1",
                "text": "fixture claim.",
                "citedEvidenceIds": ["E1"],
            }
        ],
        "evidence": [item],
        "runManifest": {
            "schemaVersion": "qa-run-v22",
            "evidenceChecksums": [
                {
                    "evidenceId": "E1",
                    "stableSourceId": qa_accuracy_eval.stable_source_id(item),
                    "sha256": qa_accuracy_eval.evidence_sha256(item),
                }
            ],
            "answerCompleteness": {"claimCount": 1, "complete": True},
            "finalGroundingAudit": {
                "schemaVersion": "final-grounding-audit-v2",
                "auditStatus": "succeeded",
                "groundingStatus": "supported",
                "factualClaimCount": 1,
                "supportedCount": 1,
                "unsupportedCount": 0,
                "notApplicableCount": 0,
                "citedClaimCount": 1,
                "citedEvidenceIds": ["E1"],
                "unknownEvidenceIds": [],
                "citationPrecision": 1.0,
                "citationCoverage": 1.0,
                "claims": [final_claim],
                "claimSources": [
                    {
                        "finalClaimId": "C1",
                        "sourceDraftClaimId": "D1",
                        "textSha256": hashlib.sha256(b"fixture claim.").hexdigest(),
                        "evidenceIds": ["E1"],
                        "draftVerificationMethod": "fixture",
                        "draftAlignmentScore": 1.0,
                        "draftConfidence": 1.0,
                    }
                ],
                "visibleProjectionValid": True,
                "auditedBodySha256": visible_body_sha256,
                "visibleBodySha256": visible_body_sha256,
            },
        },
    }


def primary_review(reviewer: str, verdict: str = "supported") -> dict[str, object]:
    return {
        "reviewer_id_hash": reviewer * 64,
        "blinded": True,
        "independent": True,
        "claims": [
            {
                "claim_id": "C1",
                "claim": "fixture claim.",
                "verdict": verdict,
            }
        ],
        "method_coverage": [],
        "constraint_coverage": [],
    }


def review_fixture(
    first: str = "supported", second: str = "supported"
) -> dict[str, object]:
    return {
        "case_id": "case-1",
        "primary_reviews": [
            primary_review("a", first),
            primary_review("b", second),
        ],
    }


class QaAccuracyEvalTests(unittest.TestCase):
    def test_repository_heldout_entry_is_explicit_and_pending(self) -> None:
        dataset = qa_accuracy_eval.load_json(ROOT / "evals" / "heldout_questions.json")
        self.assertEqual(qa_accuracy_eval.validate_dataset(dataset), [])
        self.assertEqual(dataset["dataset_role"], "production_accuracy")
        self.assertEqual(dataset["split"], "heldout")
        self.assertEqual(dataset["status"], "awaiting_independent_curation")
        self.assertEqual(
            dataset["case_schema"]["allowed_types"],
            qa_accuracy_eval.heldout_contract.CONTRACT["allowedTypes"],
        )

    def test_wilson_interval_is_bounded_and_not_point_estimate_only(self) -> None:
        low, high = qa_accuracy_eval.wilson_interval(8, 10)
        self.assertGreater(low, 0)
        self.assertLess(high, 1)
        self.assertLess(low, 0.8)
        self.assertGreater(high, 0.8)

    def test_canonical_evidence_serialization_matches_rust_fixture(self) -> None:
        self.assertEqual(
            qa_accuracy_eval.evidence_sha256(evidence()),
            "5c828dded3b4340ec00cf5983677341704daa777de8bb245fb38b77b41297ae0",
        )

    def test_locator_evidence_uses_v5_stable_block_identity(self) -> None:
        item = evidence()
        item["locator"] = {
            "documentId": "paper:fixture",
            "blockId": "block-1",
            "headingPath": ["Model", "Objective"],
            "markdownPath": "raw/canonical/fixture/full.md",
            "lineStart": 10,
            "lineEnd": 12,
            "contentHash": "content-hash",
            "snapshotId": "snapshot-1",
        }
        self.assertEqual(
            qa_accuracy_eval.stable_source_id(item),
            "paper:fixture:Model > Objective:block-1",
        )
        canonical = qa_accuracy_eval.canonical_evidence_bytes(item)
        self.assertIn(b'"locator":{"documentId":"paper:fixture"', canonical)

    def test_two_agreeing_independent_reviews_count_each_claim_once(self) -> None:
        totals = qa_accuracy_eval.review_totals(
            run_fixture(), review_fixture(), CASE
        )
        self.assertEqual(totals.supported, 1)
        self.assertEqual(totals.reviewed_answers, 1)
        self.assertEqual(totals.cited_ids, 1)
        self.assertEqual(totals.known_cited_ids, 1)

    def test_qa_run_v24_keeps_final_grounding_validation_enabled(self) -> None:
        run = run_fixture()
        run["runManifest"]["schemaVersion"] = "qa-run-v24"

        totals = qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

        self.assertEqual(totals.supported, 1)
        self.assertEqual(totals.known_cited_ids, 1)

    def test_visible_claim_keeps_structured_citation_without_inline_token(self) -> None:
        run = run_fixture()
        run["answer"] = "fixture claim.\n\n## 参考证据\n\n- [知识库](evidence:E1)"
        run["answerClaims"][0]["text"] = "fixture claim."
        review = review_fixture()
        for primary in review["primary_reviews"]:
            primary["claims"][0]["claim"] = "fixture claim."

        totals = qa_accuracy_eval.review_totals(run, review, CASE)

        self.assertEqual(totals.supported, 1)
        self.assertEqual(totals.known_cited_ids, 1)

    def test_python_evaluator_independently_rejects_unknown_cited_evidence_id(self) -> None:
        run = run_fixture()
        run["answerClaims"][0]["citedEvidenceIds"] = ["E99"]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

    def test_python_evaluator_requires_successful_final_grounding_audit_for_v22(self) -> None:
        missing = run_fixture()
        del missing["runManifest"]["finalGroundingAudit"]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(missing, review_fixture(), CASE)

        unsupported = run_fixture()
        unsupported["runManifest"]["finalGroundingAudit"]["unsupportedCount"] = 1
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(unsupported, review_fixture(), CASE)

    def test_python_evaluator_rejects_final_audit_unknown_evidence_and_count_tampering(self) -> None:
        unknown = run_fixture()
        final_claim = unknown["runManifest"]["finalGroundingAudit"]["claims"][0]
        final_claim["evidenceIds"] = ["E99"]
        unknown["runManifest"]["finalGroundingAudit"]["citedEvidenceIds"] = ["E99"]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(unknown, review_fixture(), CASE)

        extra = run_fixture()
        extra["answer"] = "fixture claim. tampered extra claim."
        extra["answerClaims"].append(
            {"claimId": "C2", "text": "tampered extra claim.", "citedEvidenceIds": ["E1"]}
        )
        extra["runManifest"]["answerCompleteness"]["claimCount"] = 2
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(extra, review_fixture(), CASE)

    def test_python_evaluator_rejects_visible_hash_and_source_mapping_tampering(self) -> None:
        bad_hash = run_fixture()
        bad_hash["runManifest"]["finalGroundingAudit"]["visibleBodySha256"] = "0" * 64
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(bad_hash, review_fixture(), CASE)

        bad_source = run_fixture()
        bad_source["runManifest"]["finalGroundingAudit"]["claimSources"][0][
            "sourceDraftClaimId"
        ] = ""
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(bad_source, review_fixture(), CASE)

    def test_single_reviewer_fails_closed(self) -> None:
        review = review_fixture()
        review["primary_reviews"] = review["primary_reviews"][:1]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run_fixture(), review, CASE)

    def test_duplicate_primary_reviewer_fails_closed(self) -> None:
        review = review_fixture()
        review["primary_reviews"][1]["reviewer_id_hash"] = "a" * 64
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run_fixture(), review, CASE)

    def test_manifest_claim_count_99_with_one_verdict_fails_closed(self) -> None:
        run = run_fixture()
        run["runManifest"]["answerCompleteness"]["claimCount"] = 99
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

    def test_each_primary_review_must_cover_every_declared_claim(self) -> None:
        run = run_fixture()
        run["answer"] = "fixture claim [E1]. second claim [E1]."
        run["answerClaims"].append(
            {
                "claimId": "C2",
                "text": "second claim [E1].",
                "citedEvidenceIds": ["E1"],
            }
        )
        run["runManifest"]["answerCompleteness"]["claimCount"] = 2
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

    def test_disagreement_requires_distinct_third_reviewer_adjudication(self) -> None:
        review = review_fixture("supported", "contradicted")
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run_fixture(), review, CASE)

        review["adjudication"] = {
            "reviewer_id_hash": "c" * 64,
            "blinded": True,
            "independent": True,
            "claims": [
                {
                    "claim_id": "C1",
                    "claim": "fixture claim.",
                    "verdict": "not_verifiable",
                }
            ],
            "method_coverage": [],
            "constraint_coverage": [],
        }
        totals = qa_accuracy_eval.review_totals(run_fixture(), review, CASE)
        self.assertEqual(totals.not_verifiable, 1)

    def test_forged_checksum_fails_closed(self) -> None:
        run = run_fixture()
        run["runManifest"]["evidenceChecksums"][0]["sha256"] = "0" * 64
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

    def test_tampered_evidence_content_fails_closed(self) -> None:
        run = run_fixture()
        original = copy.deepcopy(run)
        run["evidence"][0]["snippet"] = "Tampered after manifest creation."
        self.assertEqual(
            run["runManifest"]["evidenceChecksums"],
            original["runManifest"]["evidenceChecksums"],
        )
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

    def test_missing_evidence_or_manifest_checksum_fails_closed(self) -> None:
        run = run_fixture()
        del run["evidence"]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

    def test_partial_unsupported_and_dimension_metrics_are_counted(self) -> None:
        run = run_fixture()
        run["answerClaims"][0]["dimension"] = "method"
        review = review_fixture("partially_supported", "partially_supported")
        totals = qa_accuracy_eval.review_totals(run, review, CASE)
        self.assertEqual(totals.partially_supported, 1)
        self.assertEqual(totals.method_supported, 0)
        self.assertEqual(totals.method_total, 0)
        self.assertEqual(totals.cited_claims, 1)

        review = review_fixture("unsupported", "unsupported")
        totals = qa_accuracy_eval.review_totals(run, review, CASE)
        self.assertEqual(totals.unsupported, 1)

    def test_method_and_constraint_totals_come_from_frozen_case_not_claim_dimension(self) -> None:
        case = {
            "id": "case-1",
            "acceptableMethodFamilies": ["A", "B", "C", "D"],
            "criticalConstraints": ["X", "Y", "Z"],
        }
        review = review_fixture()
        for primary in review["primary_reviews"]:
            primary["method_coverage"] = [
                {"method_family": method, "verdict": "covered" if method == "A" else "not_covered"}
                for method in case["acceptableMethodFamilies"]
            ]
            primary["constraint_coverage"] = [
                {"constraint": constraint, "verdict": "preserved" if constraint == "X" else "not_preserved"}
                for constraint in case["criticalConstraints"]
            ]

        for dimension in ("factual", "method"):
            with self.subTest(dimension=dimension):
                run = run_fixture()
                run["answerClaims"][0]["dimension"] = dimension
                totals = qa_accuracy_eval.review_totals(run, review, case)
                self.assertEqual((totals.method_supported, totals.method_total), (1, 4))
                self.assertEqual((totals.constraint_supported, totals.constraint_total), (1, 3))

    def test_coverage_must_exactly_cover_frozen_expectations(self) -> None:
        case = {
            "id": "case-1",
            "acceptableMethodFamilies": ["A", "B"],
            "criticalConstraints": ["X"],
        }
        review = review_fixture()
        for primary in review["primary_reviews"]:
            primary["method_coverage"] = [
                {"method_family": "A", "verdict": "covered"}
            ]
            primary["constraint_coverage"] = [
                {"constraint": "X", "verdict": "preserved"}
            ]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run_fixture(), review, case)

    def test_coverage_disagreement_requires_exact_third_reviewer_adjudication(self) -> None:
        case = {
            "id": "case-1",
            "acceptableMethodFamilies": ["A"],
            "criticalConstraints": ["X"],
        }
        review = review_fixture()
        review["primary_reviews"][0]["method_coverage"] = [
            {"method_family": "A", "verdict": "covered"}
        ]
        review["primary_reviews"][1]["method_coverage"] = [
            {"method_family": "A", "verdict": "not_covered"}
        ]
        for primary in review["primary_reviews"]:
            primary["constraint_coverage"] = [
                {"constraint": "X", "verdict": "preserved"}
            ]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run_fixture(), review, case)

        review["adjudication"] = {
            "reviewer_id_hash": "c" * 64,
            "blinded": True,
            "independent": True,
            "claims": [],
            "method_coverage": [{"method_family": "A", "verdict": "not_covered"}],
            "constraint_coverage": [],
        }
        totals = qa_accuracy_eval.review_totals(run_fixture(), review, case)
        self.assertEqual((totals.method_supported, totals.method_total), (0, 1))

    def test_frozen_dataset_requires_independent_curation_and_case_hash(self) -> None:
        cases = [
            {
                "id": f"case-{index}",
                "type": "direct_factual",
                "question": f"q{index}",
                "acceptableMethodFamilies": [],
                "criticalConstraints": [],
            }
            for index in range(30)
        ]
        dataset = {
            "dataset_role": "production_accuracy",
            "split": "heldout",
            "status": "frozen",
            "minimum_case_count": 30,
            "cases": cases,
        }
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.validate_dataset(dataset)

    def test_frozen_dataset_hash_tampering_fails_closed(self) -> None:
        cases = [
            {
                "id": f"case-{index}",
                "type": "direct_factual",
                "question": f"q{index}",
                "acceptableMethodFamilies": [],
                "criticalConstraints": [],
            }
            for index in range(30)
        ]
        dataset = {
            "dataset_role": "production_accuracy",
            "split": "heldout",
            "status": "frozen",
            "minimum_case_count": 30,
            "candidate_pool": "research_questions_v1.json#split=heldout",
            "candidate_count": 80,
            "candidate_pool_cases_sha256": "e" * 64,
            "cases": cases,
            "curation": {
                "independent": True,
                "curator_id_hash": "d" * 64,
                "frozen_at": "2026-08-26T00:00:00Z",
                "cases_sha256": hashlib.sha256(
                    json.dumps(cases, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()
                ).hexdigest(),
            },
        }
        self.assertEqual(qa_accuracy_eval.validate_dataset(dataset), cases)
        dataset["cases"][0]["question"] = "tampered"
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.validate_dataset(dataset)

        run = run_fixture()
        run["evidence"] = []
        run["runManifest"]["evidenceChecksums"] = []
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)

        run = run_fixture()
        del run["runManifest"]["evidenceChecksums"]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), CASE)


if __name__ == "__main__":
    unittest.main()
