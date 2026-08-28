from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "check_qa_release_gate", ROOT / "tools" / "check_qa_release_gate.py"
)
assert SPEC and SPEC.loader
gate = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = gate
SPEC.loader.exec_module(gate)
COLLECT_SPEC = importlib.util.spec_from_file_location(
    "collect_qa_release_artifacts", ROOT / "tools" / "collect_qa_release_artifacts.py"
)
assert COLLECT_SPEC and COLLECT_SPEC.loader
collector = importlib.util.module_from_spec(COLLECT_SPEC)
sys.modules[COLLECT_SPEC.name] = collector
COLLECT_SPEC.loader.exec_module(collector)
PRODUCTION_SPEC = importlib.util.spec_from_file_location(
    "qa_production_eval", ROOT / "tools" / "qa_production_eval.py"
)
assert PRODUCTION_SPEC and PRODUCTION_SPEC.loader
production = importlib.util.module_from_spec(PRODUCTION_SPEC)
sys.modules[PRODUCTION_SPEC.name] = production
PRODUCTION_SPEC.loader.exec_module(production)


METADATA = {
    "schemaVersion": "qa-eval-metadata-v1",
    "gitCommit": "a" * 40,
    "generatedAtUtc": "2026-08-26T00:00:00Z",
    "dataset": {"version": "fixture-v1", "sha256": "b" * 64},
    "runtimeConfigSha256": "c" * 64,
    "providers": {
        name: {"provider": "fixture", "model": "fixture"}
        for name in ("answer", "embedding", "reranker", "verification")
    },
    "platform": {"system": "fixture", "release": "fixture", "machine": "fixture", "python": "fixture"},
    "hardware": {"cpu": "fixture", "logicalCpuCount": 1, "memoryBytes": 1},
}


PASSING = {
    "retrieval.json": {"workRecallAt20": 0.95, "workRecallAt10": 0.90, "workMrr": 0.85, "workNdcgAt10": 0.85},
    "conversation.json": {"referenceResolution": 0.95, "constraintPreservation": 0.97, "objectivePreservation": 0.97},
    "grounding.json": {"factualClaimPrecision": 0.97, "unsupportedFactualClaimRate": 0.02, "contradictedClaimRate": 0.01, "citationCorrectness": 0.98},
    "open_research.json": {"relevantMethodRecall": 0.90, "criticalConstraintPreservation": 0.97},
    "reliability.json": {"crashCount": 0, "providerFailureHandledRate": 1.0, "fallbackSuccessRate": 0.99, "invalidVerifiedStateCount": 0},
    "reranker.json": {"realModelMeasured": True, "fallbackRate": 0.05},
    "semantic_verifier.json": {"realProviderMeasured": True, "invalidVerifiedStateCount": 0},
    "heldout.json": {"independentlyCurated": True, "cases": 30, "factualPrecision": 0.97, "unsupportedFactualClaimRate": 0.02, "citationIdPrecision": 0.98, "citationCompleteness": 0.98},
    "performance.json": {"targetProfileFrozen": True, "measured": True, "allModeSlosPassed": True, "p95LatencyMs": 900, "maxP95LatencyMs": 1000},
}


def write_artifacts(directory: Path) -> None:
    for filename, metrics in PASSING.items():
        (directory / filename).write_text(
            json.dumps({"metadata": METADATA, "metrics": metrics}), encoding="utf-8"
        )


class QaReleaseGateTests(unittest.TestCase):
    def test_current_repository_state_fails_closed_with_reasons(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            result = gate.evaluate_release(Path(temporary))
        self.assertEqual(result["decision"], "FAIL")
        self.assertGreater(result["summary"]["failed"], 0)
        self.assertTrue(all(item["reason"] for item in result["gates"]))

    def test_complete_qualified_fixture_passes(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary)
            write_artifacts(path)
            result = gate.evaluate_release(path)
        self.assertEqual(result["decision"], "PASS")
        self.assertEqual(result["summary"]["failed"], 0)

    def test_core_reliability_failure_is_never_conditional(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary)
            write_artifacts(path)
            artifact = json.loads((path / "reliability.json").read_text(encoding="utf-8"))
            artifact["metrics"]["crashCount"] = 1
            (path / "reliability.json").write_text(json.dumps(artifact), encoding="utf-8")
            result = gate.evaluate_release(path)
        self.assertEqual(result["decision"], "FAIL")
        failed = {item["gate_id"] for item in result["gates"] if not item["passed"]}
        self.assertEqual(failed, {"reliability.crashes"})

    def test_non_finite_and_missing_metrics_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary)
            write_artifacts(path)
            artifact = json.loads((path / "retrieval.json").read_text(encoding="utf-8"))
            artifact["metrics"]["workMrr"] = float("nan")
            del artifact["metrics"]["workNdcgAt10"]
            (path / "retrieval.json").write_text(json.dumps(artifact), encoding="utf-8")
            result = gate.evaluate_release(path)
        failed = {item["gate_id"] for item in result["gates"] if not item["passed"]}
        self.assertIn("retrieval.mrr", failed)
        self.assertIn("retrieval.ndcg10", failed)

    def test_report_contains_required_audit_sections(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary)
            write_artifacts(path)
            result = gate.evaluate_release(path)
            report = gate.render_report(result, path)
        for required in (
            "Git commit",
            "Build",
            "Dataset version/hash",
            "Providers",
            "Models",
            "Gate Results",
            "Fallbacks and Limitations",
            "Final Decision",
        ):
            self.assertIn(required, report)

    def test_collector_writes_enveloped_artifact_and_rejects_raw_content(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "source.json"
            source.write_text(json.dumps({"mrr": 0.9}), encoding="utf-8")
            run_dir = root / "run"
            written = collector.collect(run_dir, METADATA, [("retrieval", source)])
            self.assertEqual(written, [run_dir / "retrieval.json"])
            payload = json.loads(written[0].read_text(encoding="utf-8"))
            self.assertEqual(payload["metadata"], METADATA)
            self.assertEqual(payload["metrics"], {"mrr": 0.9})

            unsafe = root / "unsafe.json"
            unsafe.write_text(json.dumps({"prompt": "raw prompt"}), encoding="utf-8")
            with self.assertRaises(collector.CollectionError):
                collector.collect(root / "unsafe-run", METADATA, [("grounding", unsafe)])

    def test_production_harness_derives_metrics_from_machine_reports(self) -> None:
        metrics = production.artifact_metrics(
            {
                "schemaVersion": "qa-rag-evaluation-report-v4",
                "caseDatasetSha256": "d" * 64,
                "caseCount": 1,
                "aggregate": {
                    "caseCount": 1,
                    "workRecallAt20": 1.0,
                    "workRecallAt10": 0.96,
                    "workMrr": 0.962,
                    "workNdcgAt10": 0.851,
                    "exactSourceRecallAt20": 0.9,
                    "exactSourceRecallAt10": 0.8,
                    "exactSourceMrr": 0.75,
                    "exactSourceNdcgAt10": 0.70,
                    "passageMrr": 0.90,
                    "rankingEligibleCaseCount": 1,
                    "zeroEvidenceCaseCount": 0,
                    "rerankerFallbackRate": 0.0,
                    "averageRerankerLatencyMs": 1200.0,
                },
                "cases": [
                    {
                        "rerankerVersion": "cross-encoder-research-v1",
                        "rerankerStatus": "succeeded",
                    }
                ],
            },
            {
                "schemaVersion": "qa-production-conversation-report-v1",
                "referenceResolution": 1.0,
                "constraintPreservation": 0.99,
                "objectivePreservation": 0.98,
                "caseCount": 50,
            },
            {
                "schemaVersion": "qa-semantic-verifier-report-v1",
                "realProviderMeasured": True,
                "invalidVerifiedStateCount": 0,
                "accuracy": 0.82,
                "contradictionRecall": 1.0,
                "unknownPrecision": 1.0,
                "fallbackRate": 0.0,
            },
        )
        self.assertEqual(metrics["retrieval"]["workMrr"], 0.962)
        self.assertEqual(metrics["retrieval"]["caseDatasetSha256"], "d" * 64)
        self.assertTrue(metrics["reranker"]["realModelMeasured"])
        self.assertTrue(metrics["semantic_verifier"]["realProviderMeasured"])
        self.assertFalse(metrics["performance"]["targetProfileFrozen"])

    def test_retrieval_threshold_values_are_unchanged_after_metric_rename(self) -> None:
        thresholds = json.loads(
            (ROOT / "evals" / "qa_release_thresholds.json").read_text(encoding="utf-8")
        )
        retrieval = {
            item["metric"]: item["threshold"]
            for item in thresholds["gates"]
            if item["id"].startswith("retrieval.")
        }
        self.assertEqual(
            retrieval,
            {
                "workRecallAt20": 0.95,
                "workRecallAt10": 0.90,
                "workMrr": 0.85,
                "workNdcgAt10": 0.85,
            },
        )


if __name__ == "__main__":
    unittest.main()
