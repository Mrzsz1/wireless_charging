from __future__ import annotations

import copy
import importlib.util
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
    return {
        "question": "fixture question",
        "answer": "fixture claim [E1].",
        "answerClaims": [
            {
                "claimId": "C1",
                "text": "fixture claim [E1].",
                "citedEvidenceIds": ["E1"],
            }
        ],
        "evidence": [item],
        "runManifest": {
            "evidenceChecksums": [
                {
                    "evidenceId": "E1",
                    "stableSourceId": qa_accuracy_eval.stable_source_id(item),
                    "sha256": qa_accuracy_eval.evidence_sha256(item),
                }
            ],
            "answerCompleteness": {"claimCount": 1, "complete": True},
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
                "claim": "fixture claim [E1].",
                "verdict": verdict,
            }
        ],
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
            run_fixture(), review_fixture(), "case-1"
        )
        self.assertEqual(totals.supported, 1)
        self.assertEqual(totals.reviewed_answers, 1)
        self.assertEqual(totals.cited_ids, 1)
        self.assertEqual(totals.known_cited_ids, 1)

    def test_single_reviewer_fails_closed(self) -> None:
        review = review_fixture()
        review["primary_reviews"] = review["primary_reviews"][:1]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run_fixture(), review, "case-1")

    def test_manifest_claim_count_99_with_one_verdict_fails_closed(self) -> None:
        run = run_fixture()
        run["runManifest"]["answerCompleteness"]["claimCount"] = 99
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), "case-1")

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
            qa_accuracy_eval.review_totals(run, review_fixture(), "case-1")

    def test_disagreement_requires_distinct_third_reviewer_adjudication(self) -> None:
        review = review_fixture("supported", "contradicted")
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run_fixture(), review, "case-1")

        review["adjudication"] = {
            "reviewer_id_hash": "c" * 64,
            "blinded": True,
            "independent": True,
            "claims": [
                {
                    "claim_id": "C1",
                    "claim": "fixture claim [E1].",
                    "verdict": "not_verifiable",
                }
            ],
        }
        totals = qa_accuracy_eval.review_totals(run_fixture(), review, "case-1")
        self.assertEqual(totals.not_verifiable, 1)

    def test_forged_checksum_fails_closed(self) -> None:
        run = run_fixture()
        run["runManifest"]["evidenceChecksums"][0]["sha256"] = "0" * 64
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), "case-1")

    def test_tampered_evidence_content_fails_closed(self) -> None:
        run = run_fixture()
        original = copy.deepcopy(run)
        run["evidence"][0]["snippet"] = "Tampered after manifest creation."
        self.assertEqual(
            run["runManifest"]["evidenceChecksums"],
            original["runManifest"]["evidenceChecksums"],
        )
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), "case-1")

    def test_missing_evidence_or_manifest_checksum_fails_closed(self) -> None:
        run = run_fixture()
        del run["evidence"]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), "case-1")

        run = run_fixture()
        run["evidence"] = []
        run["runManifest"]["evidenceChecksums"] = []
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), "case-1")

        run = run_fixture()
        del run["runManifest"]["evidenceChecksums"]
        with self.assertRaises(qa_accuracy_eval.AccuracyEvalError):
            qa_accuracy_eval.review_totals(run, review_fixture(), "case-1")


if __name__ == "__main__":
    unittest.main()
