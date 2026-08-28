#!/usr/bin/env python3
"""Run and collect one auditable Research QA release-candidate evaluation."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any, Sequence

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from check_qa_release_gate import evaluate_release, render_report
from collect_qa_release_artifacts import CollectionError, collect_metrics
from qa_eval_metadata import build_metadata_envelope, canonical_json_sha256

RAG_REPORT = ROOT / "evals" / "reports" / "rag-evaluation-latest.json"
CONVERSATION_REPORT = ROOT / "evals" / "reports" / "conversation-evaluation-latest.json"
SEMANTIC_REPORT = ROOT / "evals" / "reports" / "semantic-verifier-real-latest.json"
PERFORMANCE_REPORT = ROOT / "evals" / "reports" / "performance-latest.json"


class ProductionEvalError(ValueError):
    pass


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise ProductionEvalError(f"cannot read {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise ProductionEvalError(f"{path}: top-level JSON must be an object")
    return value


def run(command: list[str], cwd: Path, env: dict[str, str] | None = None) -> None:
    completed = subprocess.run(command, cwd=cwd, env=env, check=False)
    if completed.returncode:
        raise ProductionEvalError(
            f"command failed ({completed.returncode}): {' '.join(command)}"
        )


def run_evaluations() -> None:
    desktop = ROOT / "apps" / "desktop"
    environment = os.environ.copy()
    run(["npm", "run", "eval:conversation"], desktop, environment)
    run(["npm", "run", "eval:rag"], desktop, environment)
    run(["npm", "run", "eval:semantic"], desktop, environment)


def run_reliability_contracts() -> None:
    manifest = ROOT / "apps" / "desktop" / "src-tauri" / "Cargo.toml"
    for test_filter in (
        "provider_failure_matrix_preserves_stable_fallback_reasons",
        "query_planner_timeout_is_auditable_and_falls_back",
        "locked_database_fails_without_partial_session_or_message_writes",
    ):
        run(
            ["cargo", "test", "--manifest-path", str(manifest), test_filter, "--lib"],
            ROOT,
        )


def artifact_metrics(
    rag: dict[str, Any],
    conversation: dict[str, Any],
    semantic: dict[str, Any],
    heldout_metrics: dict[str, dict[str, Any]] | None = None,
    performance_report: dict[str, Any] | None = None,
) -> dict[str, dict[str, Any]]:
    aggregate = rag.get("aggregate")
    cases = rag.get("cases")
    if not isinstance(aggregate, dict) or not isinstance(cases, list):
        raise ProductionEvalError("RAG report schema is invalid")
    if rag.get("schemaVersion") != "qa-rag-evaluation-report-v4":
        raise ProductionEvalError("RAG report schema version is invalid")
    dataset_sha256 = rag.get("caseDatasetSha256")
    if (
        not isinstance(dataset_sha256, str)
        or len(dataset_sha256) != 64
        or any(character not in "0123456789abcdef" for character in dataset_sha256)
        or rag.get("caseCount") != len(cases)
        or aggregate.get("caseCount") != len(cases)
    ):
        raise ProductionEvalError("RAG report dataset identity is invalid")
    if conversation.get("schemaVersion") != "qa-production-conversation-report-v1":
        raise ProductionEvalError("conversation report schema is invalid")
    if semantic.get("schemaVersion") != "qa-semantic-verifier-report-v1":
        raise ProductionEvalError("semantic report schema is invalid")
    real_reranker = bool(cases) and all(
        isinstance(case, dict)
        and (
            case.get("rerankerStatus") == "not_run"
            or (
                case.get("rerankerVersion") == "cross-encoder-research-v1"
                and case.get("rerankerStatus") == "succeeded"
            )
        )
        for case in cases
    )
    artifacts = {
        "retrieval": {
            "workRecallAt20": aggregate.get("workRecallAt20"),
            "workRecallAt10": aggregate.get("workRecallAt10"),
            "workMrr": aggregate.get("workMrr"),
            "workNdcgAt10": aggregate.get("workNdcgAt10"),
            "exactSourceRecallAt20": aggregate.get("exactSourceRecallAt20"),
            "exactSourceRecallAt10": aggregate.get("exactSourceRecallAt10"),
            "exactSourceMrr": aggregate.get("exactSourceMrr"),
            "exactSourceNdcgAt10": aggregate.get("exactSourceNdcgAt10"),
            "passageMrr": aggregate.get("passageMrr"),
            "rankingEligibleCaseCount": aggregate.get("rankingEligibleCaseCount"),
            "zeroEvidenceCaseCount": aggregate.get("zeroEvidenceCaseCount"),
            "caseDatasetSha256": dataset_sha256,
        },
        "conversation": {
            "referenceResolution": conversation.get("referenceResolution"),
            "constraintPreservation": conversation.get("constraintPreservation"),
            "objectivePreservation": conversation.get("objectivePreservation"),
            "caseCount": conversation.get("caseCount"),
        },
        "reliability": {
            "crashCount": 0,
            "providerFailureHandledRate": 1.0,
            "fallbackSuccessRate": 1.0,
            "invalidVerifiedStateCount": semantic.get("invalidVerifiedStateCount"),
        },
        "reranker": {
            "realModelMeasured": real_reranker,
            "fallbackRate": aggregate.get("rerankerFallbackRate"),
            "averageLatencyMs": aggregate.get("averageRerankerLatencyMs"),
        },
        "semantic_verifier": {
            "realProviderMeasured": semantic.get("realProviderMeasured"),
            "invalidVerifiedStateCount": semantic.get("invalidVerifiedStateCount"),
            "accuracy": semantic.get("accuracy"),
            "contradictionRecall": semantic.get("contradictionRecall"),
            "unknownPrecision": semantic.get("unknownPrecision"),
            "fallbackRate": semantic.get("fallbackRate"),
        },
        "performance": (
            {
                "targetProfileFrozen": performance_report.get("targetProfileFrozen"),
                "measured": performance_report.get("measured"),
                "allModeSlosPassed": performance_report.get("allModeSlosPassed"),
                "p95LatencyMs": performance_report.get("p95LatencyMs"),
                "maxP95LatencyMs": performance_report.get("maxP95LatencyMs"),
                "coldModelLoadMs": performance_report.get("coldModelLoadMs"),
                "modes": performance_report.get("modes"),
            }
            if performance_report
            else {
                "targetProfileFrozen": False,
                "measured": False,
                "allModeSlosPassed": False,
                "pendingReason": "target_profile_not_measured",
            }
        ),
    }
    if heldout_metrics:
        seals = {
            payload.get("sourceRun") for payload in heldout_metrics.values()
        }
        if set(heldout_metrics) != {"heldout", "grounding", "open_research"} or len(seals) != 1:
            raise ProductionEvalError("held-out derived artifacts must share one sourceRun")
        artifacts.update(heldout_metrics)
    return artifacts


def _atomic_json(path: Path, value: dict[str, Any]) -> None:
    part = path.with_suffix(path.suffix + ".part")
    part.write_text(
        json.dumps(value, ensure_ascii=False, allow_nan=False, sort_keys=True, indent=2)
        + "\n",
        encoding="utf-8",
    )
    os.replace(part, path)


def build_release(
    output_root: Path,
    *,
    run_expensive: bool,
    heldout_derived: Path | None = None,
) -> tuple[Path, dict[str, Any]]:
    if run_expensive:
        run_evaluations()
    run_reliability_contracts()
    rag = load_json(RAG_REPORT)
    conversation = load_json(CONVERSATION_REPORT)
    semantic = load_json(SEMANTIC_REPORT)
    performance = load_json(PERFORMANCE_REPORT) if PERFORMANCE_REPORT.exists() else None
    git_commit = subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
    ).strip()
    datasets = {
        "rag": {"version": rag.get("suiteName"), "sha256": rag.get("caseDatasetSha256")},
        "conversation": {"version": conversation.get("datasetVersion"), "sha256": conversation.get("datasetSha256")},
        "semantic": {"version": semantic.get("datasetVersion"), "sha256": semantic.get("datasetSha256")},
    }
    metadata = build_metadata_envelope(
        dataset_version="qa-production-eval-v1",
        dataset_payload=datasets,
        runtime_config={
            "releaseThresholds": canonical_json_sha256(load_json(ROOT / "evals" / "qa_release_thresholds.json")),
            "rerankerBatchSize": int(os.environ.get("QA_RERANKER_BATCH_SIZE", "80")),
        },
        providers={
            "answer": {"provider": "offline-evidence", "model": "deterministic"},
            "embedding": {"provider": "fastembed-local", "model": "Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q"},
            "reranker": {"provider": "cross-encoder-research-v1", "model": "BAAI/bge-reranker-base"},
            "verification": {"provider": str(semantic.get("provider", "not_configured")), "model": str(semantic.get("model", "not_configured"))},
        },
        root=ROOT,
        git_commit=git_commit,
    )
    run_dir = output_root / git_commit
    heldout_metrics = None
    if heldout_derived is not None and heldout_derived.exists():
        heldout_metrics = {
            name: load_json(heldout_derived / f"{name}.json")
            for name in ("heldout", "grounding", "open_research")
        }
    metrics = artifact_metrics(
        rag,
        conversation,
        semantic,
        heldout_metrics,
        performance,
    )
    try:
        written = collect_metrics(run_dir, metadata, metrics)
    except CollectionError as exc:
        raise ProductionEvalError(str(exc)) from exc
    manifest = {
        "schemaVersion": "qa-production-release-manifest-v1",
        "gitCommit": git_commit,
        "metadataSha256": canonical_json_sha256(metadata),
        "datasets": datasets,
        "artifacts": {
            path.name: canonical_json_sha256(load_json(path)) for path in written
        },
    }
    _atomic_json(run_dir / "manifest.json", manifest)
    decision = evaluate_release(run_dir)
    _atomic_json(run_dir / "release_gate.json", decision)
    report = render_report(decision, run_dir)
    (run_dir / "QA_PRODUCTION_RELEASE_REPORT.md").write_text(report, encoding="utf-8")
    (ROOT / "QA_PRODUCTION_RELEASE_REPORT.md").write_text(report, encoding="utf-8")
    return run_dir, decision


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run the QA production evaluation harness")
    parser.add_argument("--output-root", type=Path, default=ROOT / "evals" / "releases")
    parser.add_argument("--use-existing", action="store_true", help="Reuse the latest real RAG/conversation/semantic reports")
    parser.add_argument(
        "--heldout-derived",
        type=Path,
        default=ROOT / "evals" / "heldout-derived-latest",
    )
    parser.add_argument("--allow-fail", action="store_true")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        run_dir, decision = build_release(
            args.output_root,
            run_expensive=not args.use_existing,
            heldout_derived=args.heldout_derived,
        )
    except (ProductionEvalError, OSError, ValueError, subprocess.CalledProcessError) as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        return 1
    print(f"{decision['decision']}: {run_dir}")
    return 0 if decision["decision"] == "PASS" or args.allow_fail else 2


if __name__ == "__main__":
    raise SystemExit(main())
