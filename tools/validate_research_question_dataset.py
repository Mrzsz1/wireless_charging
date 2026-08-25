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


def main() -> int:
    payload = json.loads(DATASET.read_text(encoding="utf-8"))
    assert payload.get("schemaVersion") == "research-question-dataset-v1"
    assert payload.get("status") == "sealed"
    cases = payload.get("cases")
    assert isinstance(cases, list)
    assert 300 <= len(cases) <= 500
    assert payload.get("totalCount") == len(cases)

    ids: set[str] = set()
    questions: set[str] = set()
    split_counts: Counter[str] = Counter()
    intent_counts: Counter[str] = Counter()
    domain_counts: Counter[str] = Counter()
    for case in cases:
        assert isinstance(case, dict)
        assert REQUIRED_FIELDS <= case.keys()
        assert case["id"] not in ids
        assert case["question"] not in questions
        ids.add(case["id"])
        questions.add(case["question"])
        assert case["split"] in ALLOWED_SPLITS
        assert case["difficulty"] in ALLOWED_DIFFICULTIES
        assert len(case["question"].strip()) >= 12
        if case["split"] == "heldout":
            assert not (FORBIDDEN_HELDOUT_FIELDS & case.keys())
        split_counts[case["split"]] += 1
        intent_counts[case["intent"]] += 1
        domain_counts[case["domain"]] += 1

    assert split_counts == Counter(development=160, regression=120, heldout=80)
    assert len(intent_counts) == 10 and min(intent_counts.values()) >= 30
    assert len(domain_counts) == 12 and min(domain_counts.values()) >= 30
    canonical = json.dumps(
        cases, ensure_ascii=False, separators=(",", ":"), sort_keys=True
    ).encode()
    assert hashlib.sha256(canonical).hexdigest() == payload.get("casesSha256")
    print(
        "Research question dataset PASS: "
        f"total={len(cases)} splits={dict(split_counts)} "
        f"intents={len(intent_counts)} domains={len(domain_counts)}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
