#!/usr/bin/env python3
"""Fail-closed structural gate for the frozen Research QA question corpus."""

from __future__ import annotations

import hashlib
import json
from collections import Counter
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATASET = ROOT / "evals" / "research_questions_v1.json"
ALLOWED_SPLITS = {"development", "regression", "heldout"}
ALLOWED_DIFFICULTIES = {"medium", "hard"}
REQUIRED_FIELDS = {"id", "split", "intent", "domain", "difficulty", "question"}
FORBIDDEN_HELDOUT_FIELDS = {
    "answer",
    "expectedAnswer",
    "expectedDocuments",
    "expectedEvidence",
    "mustMention",
}


class ResearchQuestionDatasetError(ValueError):
    pass


def canonical_cases_sha256(cases: list[dict[str, object]]) -> str:
    canonical = json.dumps(
        cases, ensure_ascii=False, separators=(",", ":"), sort_keys=True
    ).encode()
    return hashlib.sha256(canonical).hexdigest()


def validate_payload(payload: object) -> list[dict[str, object]]:
    if not isinstance(payload, dict):
        raise ResearchQuestionDatasetError("dataset top level must be an object")
    if payload.get("schemaVersion") != "research-question-dataset-v1":
        raise ResearchQuestionDatasetError("unsupported schemaVersion")
    if payload.get("status") != "sealed":
        raise ResearchQuestionDatasetError("dataset status must be sealed")
    cases = payload.get("cases")
    if not isinstance(cases, list) or not 300 <= len(cases) <= 500:
        raise ResearchQuestionDatasetError("dataset must contain 300-500 cases")
    if payload.get("totalCount") != len(cases):
        raise ResearchQuestionDatasetError("totalCount does not match cases")

    ids: set[str] = set()
    questions: set[str] = set()
    split_counts: Counter[str] = Counter()
    intent_counts: Counter[str] = Counter()
    domain_counts: Counter[str] = Counter()
    for index, case in enumerate(cases):
        if not isinstance(case, dict) or not REQUIRED_FIELDS <= case.keys():
            raise ResearchQuestionDatasetError(f"case[{index}] missing required fields")
        case_id = case["id"]
        question = case["question"]
        if not isinstance(case_id, str) or not case_id.strip() or case_id in ids:
            raise ResearchQuestionDatasetError(f"case[{index}] has invalid or duplicate id")
        if (
            not isinstance(question, str)
            or len(question.strip()) < 12
            or question in questions
        ):
            raise ResearchQuestionDatasetError(
                f"case[{index}] has invalid or duplicate question"
            )
        ids.add(case_id)
        questions.add(question)
        split = case["split"]
        difficulty = case["difficulty"]
        intent = case["intent"]
        domain = case["domain"]
        if split not in ALLOWED_SPLITS:
            raise ResearchQuestionDatasetError(f"{case_id}: invalid split")
        if difficulty not in ALLOWED_DIFFICULTIES:
            raise ResearchQuestionDatasetError(f"{case_id}: invalid difficulty")
        if not isinstance(intent, str) or not intent:
            raise ResearchQuestionDatasetError(f"{case_id}: invalid intent")
        if not isinstance(domain, str) or not domain:
            raise ResearchQuestionDatasetError(f"{case_id}: invalid domain")
        if split == "heldout" and FORBIDDEN_HELDOUT_FIELDS & case.keys():
            raise ResearchQuestionDatasetError(f"{case_id}: heldout case leaks answer data")
        split_counts[split] += 1
        intent_counts[intent] += 1
        domain_counts[domain] += 1

    if split_counts != Counter(development=160, regression=120, heldout=80):
        raise ResearchQuestionDatasetError("split counts must be 160/120/80")
    if len(intent_counts) != 10 or min(intent_counts.values()) < 30:
        raise ResearchQuestionDatasetError("intent distribution is invalid")
    if len(domain_counts) != 12 or min(domain_counts.values()) < 30:
        raise ResearchQuestionDatasetError("domain distribution is invalid")
    if canonical_cases_sha256(cases) != payload.get("casesSha256"):
        raise ResearchQuestionDatasetError("casesSha256 verification failed")
    return cases


def main() -> int:
    payload = json.loads(DATASET.read_text(encoding="utf-8"))
    cases = validate_payload(payload)
    split_counts = Counter(case["split"] for case in cases)
    intent_counts = Counter(case["intent"] for case in cases)
    domain_counts = Counter(case["domain"] for case in cases)
    print(
        "Research question dataset PASS: "
        f"total={len(cases)} splits={dict(split_counts)} "
        f"intents={len(intent_counts)} domains={len(domain_counts)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
