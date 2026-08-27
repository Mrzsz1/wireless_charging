#!/usr/bin/env python3
"""Canonical independent held-out curation, blind review, and metric derivation."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import sys
from pathlib import Path
from typing import Any, Sequence

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

import qa_accuracy_eval as accuracy
import validate_research_question_dataset as research_dataset

SCHEMA_VERSION = "qa-independent-heldout-workflow-v2"
OPEN_RESEARCH_TYPES = {
    "method_improvement",
    "solution_search",
    "problem_modeling",
    "related_problem",
    "counterfactual",
    "novelty",
}


class HeldoutWorkflowError(ValueError):
    pass


def canonical_sha256(value: Any) -> str:
    return hashlib.sha256(
        json.dumps(
            value,
            ensure_ascii=False,
            allow_nan=False,
            sort_keys=True,
            separators=(",", ":"),
        ).encode("utf-8")
    ).hexdigest()


def atomic_json(path: Path, value: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    part = path.with_suffix(path.suffix + ".part")
    part.write_text(
        json.dumps(value, ensure_ascii=False, allow_nan=False, sort_keys=True, indent=2)
        + "\n",
        encoding="utf-8",
    )
    os.replace(part, path)


def curator_template(case_count: int = 50) -> dict[str, Any]:
    if case_count < 30:
        raise HeldoutWorkflowError("independent held-out template requires at least 30 cases")
    return {
        "schemaVersion": SCHEMA_VERSION,
        "status": "draft",
        "independent": False,
        "curatorIdHash": "",
        "datasetVersion": "",
        "canonicalIdPolicy": {
            "methodIds": "Use stable method-family IDs, not display-string exact match.",
            "constraintIds": "Use stable critical-constraint IDs evaluated through the final answer.",
        },
        "allowedTypes": list(accuracy.heldout_contract.CONTRACT["allowedTypes"]),
        "cases": [
            {
                "id": "",
                "type": "",
                "question": "",
                "stratum": "",
                "difficulty": "",
                "criticalConstraints": [],
                "acceptableMethodFamilies": [],
                "notesForReviewers": "",
            }
            for index in range(1, case_count + 1)
        ],
    }


def _load_heldout_candidate_pool(candidate_pool_path: Path) -> tuple[dict[str, dict[str, Any]], str]:
    try:
        payload = accuracy.load_json(candidate_pool_path)
        all_cases = research_dataset.validate_payload(payload)
    except (accuracy.AccuracyEvalError, research_dataset.ResearchQuestionDatasetError) as exc:
        raise HeldoutWorkflowError(f"candidate pool validation failed: {exc}") from exc
    heldout = [case for case in all_cases if case["split"] == "heldout"]
    if len(heldout) != 80:
        raise HeldoutWorkflowError("candidate pool must contain exactly 80 heldout cases")
    if any(case["intent"] not in accuracy.VALID_HELDOUT_TYPES for case in heldout):
        raise HeldoutWorkflowError("candidate pool heldout intent is not canonical")
    pool_hash = payload.get("casesSha256")
    if not isinstance(pool_hash, str) or not accuracy.SHA256_RE.fullmatch(pool_hash):
        raise HeldoutWorkflowError("candidate pool casesSha256 is invalid")
    return {str(case["id"]): case for case in heldout}, pool_hash


def freeze_draft(
    draft: dict[str, Any],
    frozen_at: str,
    candidate_pool_path: Path = research_dataset.DATASET,
) -> dict[str, Any]:
    if draft.get("schemaVersion") != SCHEMA_VERSION:
        raise HeldoutWorkflowError("unsupported curator template schema")
    if draft.get("independent") is not True:
        raise HeldoutWorkflowError("curator must attest independent=true")
    if draft.get("allowedTypes") != list(
        accuracy.heldout_contract.CONTRACT["allowedTypes"]
    ):
        raise HeldoutWorkflowError("curator template allowedTypes contract drift")
    curator_hash = draft.get("curatorIdHash")
    if not isinstance(curator_hash, str) or not accuracy.SHA256_RE.fullmatch(curator_hash):
        raise HeldoutWorkflowError("curatorIdHash must be a lowercase SHA-256")
    version = draft.get("datasetVersion")
    if not isinstance(version, str) or not version.strip():
        raise HeldoutWorkflowError("datasetVersion is required")
    cases = draft.get("cases")
    if not isinstance(cases, list) or len(cases) < 30:
        raise HeldoutWorkflowError("at least 30 independently curated cases are required")
    candidates, candidate_pool_hash = _load_heldout_candidate_pool(candidate_pool_path)
    seen: set[str] = set()
    for case in cases:
        if not isinstance(case, dict) or not all(case.get(key) for key in ("id", "type", "question")):
            raise HeldoutWorkflowError("every case requires non-empty id/type/question")
        if case["id"] in seen:
            raise HeldoutWorkflowError(f"duplicate case id: {case['id']}")
        seen.add(case["id"])
        if case["type"] not in accuracy.VALID_HELDOUT_TYPES:
            raise HeldoutWorkflowError(f"{case['id']}: type must use canonical ResearchIntent")
        candidate = candidates.get(case["id"])
        if candidate is None:
            raise HeldoutWorkflowError(f"{case['id']}: case is not in sealed heldout candidate pool")
        if case["question"] != candidate["question"]:
            raise HeldoutWorkflowError(f"{case['id']}: question differs from sealed candidate")
        if case["type"] != candidate["intent"]:
            raise HeldoutWorkflowError(
                f"{case['id']}: type differs from sealed candidate ResearchIntent"
            )
        for field in ("criticalConstraints", "acceptableMethodFamilies"):
            if not isinstance(case.get(field), list) or any(
                not isinstance(value, str) or not value.strip() for value in case[field]
            ) or len(case[field]) != len(set(case[field])):
                raise HeldoutWorkflowError(
                    f"{case['id']}: {field} must be a unique non-empty string array"
                )
        if case["type"] in OPEN_RESEARCH_TYPES and not (
            case["criticalConstraints"] or case["acceptableMethodFamilies"]
        ):
            raise HeldoutWorkflowError(
                f"{case['id']}: open-research case requires canonical constraints or methods"
            )
    frozen_cases = [
        {
            "id": case["id"],
            "type": case["type"],
            "question": case["question"],
            "stratum": case.get("stratum", ""),
            "difficulty": case.get("difficulty", ""),
            "criticalConstraints": case["criticalConstraints"],
            "acceptableMethodFamilies": case["acceptableMethodFamilies"],
            "notesForReviewers": case.get("notesForReviewers", ""),
        }
        for case in cases
    ]
    return {
        "version": version,
        "dataset_role": "production_accuracy",
        "split": "heldout",
        "status": "frozen",
        "minimum_case_count": 30,
        "candidate_pool": "research_questions_v1.json#split=heldout",
        "candidate_count": 80,
        "candidate_pool_cases_sha256": candidate_pool_hash,
        "curation": {
            "independent": True,
            "curator_id_hash": curator_hash,
            "frozen_at": frozen_at,
            "cases_sha256": canonical_sha256(frozen_cases),
        },
        "cases": frozen_cases,
    }


def blind_review_bundle(case: dict[str, Any], run: dict[str, Any]) -> dict[str, Any]:
    case_id = case["id"]
    known_ids, manifest = accuracy._validate_evidence_and_manifest(run, case_id)
    claims, dimensions, citations = accuracy._validate_answer_claims(
        run, manifest, known_ids, case_id
    )
    evidence = [
        {
            "id": item["id"],
            "title": item["title"],
            "snippet": item["snippet"],
            "sourceLocation": item["sourceLocation"],
        }
        for item in run["evidence"]
        if item["id"] in known_ids
    ]
    return {
        "schemaVersion": "qa-blind-heldout-review-v2",
        "caseId": case_id,
        "question": case["question"],
        "answer": run["answer"],
        "claims": [
            {
                "claimId": claim_id,
                "claim": text,
                "dimension": dimensions[claim_id],
                "citedEvidenceIds": citations[claim_id],
            }
            for claim_id, text in claims.items()
        ],
        "evidence": evidence,
        "allowedVerdicts": sorted(accuracy.VALID_VERDICTS),
        "expectedMethodFamilies": case["acceptableMethodFamilies"],
        "expectedCriticalConstraints": case["criticalConstraints"],
        "allowedMethodCoverageVerdicts": sorted(
            accuracy.VALID_METHOD_COVERAGE_VERDICTS
        ),
        "allowedConstraintCoverageVerdicts": sorted(
            accuracy.VALID_CONSTRAINT_COVERAGE_VERDICTS
        ),
        "reviewerInstructions": {
            "blinded": True,
            "systemVerdictHidden": True,
            "reviewEachClaimExactlyOnce": True,
            "reviewEachExpectedMethodFamilyExactlyOnce": True,
            "reviewEachExpectedCriticalConstraintExactlyOnce": True,
            "coverageDenominatorsComeOnlyFromFrozenCase": True,
        },
    }


def export_blind_reviews(dataset_path: Path, runs_dir: Path, output_dir: Path) -> int:
    dataset = accuracy.load_json(dataset_path)
    cases = accuracy.validate_dataset(dataset)
    if dataset.get("status") != "frozen" or len(cases) < dataset["minimum_case_count"]:
        raise HeldoutWorkflowError("held-out dataset is not independently frozen")
    for case in cases:
        run = accuracy.load_json(runs_dir / f"{case['id']}.json")
        if run.get("question") != case["question"]:
            raise HeldoutWorkflowError(f"{case['id']}: run question mismatch")
        atomic_json(output_dir / f"{case['id']}.json", blind_review_bundle(case, run))
    return len(cases)


def ratio(numerator: int, denominator: int) -> float:
    return numerator / denominator if denominator else 0.0


def derive_metrics(
    dataset_path: Path, runs_dir: Path, reviews_dir: Path
) -> dict[str, dict[str, Any]]:
    dataset = accuracy.load_json(dataset_path)
    cases = accuracy.validate_dataset(dataset)
    if dataset.get("status") != "frozen" or len(cases) < dataset["minimum_case_count"]:
        raise HeldoutWorkflowError("held-out dataset is not independently frozen")
    totals = accuracy.Totals()
    for case in cases:
        run = accuracy.load_json(runs_dir / f"{case['id']}.json")
        review = accuracy.load_json(reviews_dir / f"{case['id']}.json")
        if run.get("question") != case["question"]:
            raise HeldoutWorkflowError(f"{case['id']}: run question mismatch")
        totals = totals.add(accuracy.review_totals(run, review, case))
    factual_total = (
        totals.supported
        + totals.partially_supported
        + totals.unsupported
        + totals.contradicted
        + totals.not_verifiable
    )
    common = {
        "sourceRun": dataset["curation"]["cases_sha256"],
        "cases": totals.reviewed_answers,
    }
    heldout = {
        **common,
        "independentlyCurated": True,
        "factualPrecision": ratio(totals.supported, factual_total),
        "unsupportedFactualClaimRate": ratio(
            totals.unsupported + totals.not_verifiable, factual_total
        ),
        "contradictedClaimRate": ratio(totals.contradicted, factual_total),
        "citationIdPrecision": ratio(totals.known_cited_ids, totals.cited_ids),
        "citationCompleteness": ratio(totals.cited_claims, totals.applicable_claims),
    }
    grounding = {
        **common,
        "factualClaimPrecision": heldout["factualPrecision"],
        "unsupportedFactualClaimRate": heldout["unsupportedFactualClaimRate"],
        "contradictedClaimRate": heldout["contradictedClaimRate"],
        "citationCorrectness": heldout["citationIdPrecision"],
        "citationCompleteness": heldout["citationCompleteness"],
    }
    open_research = {
        **common,
        "relevantMethodRecall": ratio(totals.method_supported, totals.method_total),
        "criticalConstraintPreservation": ratio(
            totals.constraint_supported, totals.constraint_total
        ),
    }
    return {
        "heldout": heldout,
        "grounding": grounding,
        "open_research": open_research,
    }


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Independent held-out workflow")
    subparsers = parser.add_subparsers(dest="command", required=True)
    template = subparsers.add_parser("template")
    template.add_argument("--output", type=Path, required=True)
    template.add_argument("--cases", type=int, default=50)
    freeze = subparsers.add_parser("freeze")
    freeze.add_argument("--draft", type=Path, required=True)
    freeze.add_argument("--output", type=Path, required=True)
    freeze.add_argument("--frozen-at", required=True)
    export = subparsers.add_parser("export-review")
    export.add_argument("--dataset", type=Path, required=True)
    export.add_argument("--runs", type=Path, required=True)
    export.add_argument("--output", type=Path, required=True)
    derive = subparsers.add_parser("derive")
    derive.add_argument("--dataset", type=Path, required=True)
    derive.add_argument("--runs", type=Path, required=True)
    derive.add_argument("--reviews", type=Path, required=True)
    derive.add_argument("--output", type=Path, required=True)
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        if args.command == "template":
            atomic_json(args.output, curator_template(args.cases))
        elif args.command == "freeze":
            atomic_json(
                args.output,
                freeze_draft(accuracy.load_json(args.draft), args.frozen_at),
            )
        elif args.command == "export-review":
            export_blind_reviews(args.dataset, args.runs, args.output)
        else:
            metrics = derive_metrics(args.dataset, args.runs, args.reviews)
            for name, payload in metrics.items():
                atomic_json(args.output / f"{name}.json", payload)
    except (HeldoutWorkflowError, accuracy.AccuracyEvalError, OSError, ValueError) as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
