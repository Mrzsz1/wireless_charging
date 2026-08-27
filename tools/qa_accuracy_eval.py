from __future__ import annotations

import argparse
import hashlib
import hmac
import json
import math
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Sequence

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools"
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

import heldout_contract

DEFAULT_DATASET = ROOT / "evals" / "heldout_questions.json"
VALID_VERDICTS = {
    "supported",
    "partially_supported",
    "unsupported",
    "contradicted",
    "not_applicable",
    # Kept for audit bundles exported before the production schema was frozen.
    "not_verifiable",
}
VALID_DIMENSIONS = {"factual", "reference", "method", "constraint"}
VALID_METHOD_COVERAGE_VERDICTS = {"covered", "not_covered"}
VALID_CONSTRAINT_COVERAGE_VERDICTS = {"preserved", "not_preserved"}
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")
VALID_HELDOUT_TYPES = heldout_contract.VALID_HELDOUT_TYPES

# This is the serde field order of apps/desktop/src-tauri/src/qa.rs::EvidenceItem.
# QaRunManifest hashes serde_json::to_vec(EvidenceItem), so the evaluator must
# rebuild exactly that byte representation rather than hashing the arbitrary key
# order found in a hand-edited JSON audit bundle.
EVIDENCE_FIELDS_V4 = (
    "id",
    "kind",
    "tier",
    "title",
    "snippet",
    "score",
    "rank",
    "pageId",
    "pageType",
    "sourcePath",
    "wikilink",
    "bookId",
    "chapterId",
    "physicalPageStart",
    "physicalPageEnd",
    "markdownPath",
    "pdfPath",
    "nodeId",
    "sourceLocation",
    "relation",
    "retrievalReason",
)
EVIDENCE_FIELDS = EVIDENCE_FIELDS_V4 + ("locator",)
LOCATOR_FIELDS = (
    "documentId",
    "blockId",
    "headingPath",
    "markdownPath",
    "lineStart",
    "lineEnd",
    "contentHash",
    "snapshotId",
)
EVIDENCE_STRING_FIELDS = set(EVIDENCE_FIELDS_V4) - {
    "score",
    "rank",
    "physicalPageStart",
    "physicalPageEnd",
}


class AccuracyEvalError(ValueError):
    pass


@dataclass(frozen=True)
class Totals:
    supported: int = 0
    partially_supported: int = 0
    unsupported: int = 0
    contradicted: int = 0
    not_verifiable: int = 0
    not_applicable: int = 0
    cited_ids: int = 0
    known_cited_ids: int = 0
    applicable_claims: int = 0
    cited_claims: int = 0
    reference_supported: int = 0
    reference_total: int = 0
    method_supported: int = 0
    method_total: int = 0
    constraint_supported: int = 0
    constraint_total: int = 0
    complete_answers: int = 0
    reviewed_answers: int = 0

    def add(self, other: "Totals") -> "Totals":
        return Totals(
            **{
                field: getattr(self, field) + getattr(other, field)
                for field in self.__dataclass_fields__
            }
        )


def _reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise AccuracyEvalError(f"JSON 包含重复字段: {key}")
        value[key] = item
    return value


def _reject_non_finite_number(value: str) -> None:
    raise AccuracyEvalError(f"JSON 包含非有限数值: {value}")


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(
            path.read_text(encoding="utf-8"),
            object_pairs_hook=_reject_duplicate_keys,
            parse_constant=_reject_non_finite_number,
        )
    except AccuracyEvalError:
        raise
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise AccuracyEvalError(f"读取 JSON 失败 {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise AccuracyEvalError(f"JSON 顶层必须是对象: {path}")
    return value


def wilson_interval(
    successes: int, total: int, z: float = 1.959963984540054
) -> tuple[float, float]:
    if total <= 0:
        return (0.0, 0.0)
    estimate = successes / total
    denominator = 1 + z * z / total
    center = (estimate + z * z / (2 * total)) / denominator
    margin = (
        z
        * math.sqrt(estimate * (1 - estimate) / total + z * z / (4 * total * total))
        / denominator
    )
    return (max(0.0, center - margin), min(1.0, center + margin))


def validate_dataset(dataset: dict[str, Any]) -> list[dict[str, Any]]:
    try:
        return heldout_contract.validate_dataset(dataset)
    except heldout_contract.HeldoutContractError as exc:
        raise AccuracyEvalError(str(exc)) from exc


def _validate_evidence_item(item: Any, case_id: str) -> dict[str, Any]:
    if not isinstance(item, dict):
        raise AccuracyEvalError(f"{case_id}: evidence item 必须是对象")
    keys = set(item)
    current = set(EVIDENCE_FIELDS)
    legacy = set(EVIDENCE_FIELDS_V4)
    if keys not in (current, legacy):
        missing = sorted(current - keys)
        extra = sorted(keys - current)
        raise AccuracyEvalError(
            f"{case_id}: evidence schema 不匹配; missing={missing}, extra={extra}"
        )
    for field in EVIDENCE_STRING_FIELDS:
        if not isinstance(item[field], str):
            raise AccuracyEvalError(f"{case_id}: evidence.{field} 必须是字符串")
    if not item["id"].strip():
        raise AccuracyEvalError(f"{case_id}: evidence.id 不能为空")
    score = item["score"]
    if (
        isinstance(score, bool)
        or not isinstance(score, float)
        or not math.isfinite(score)
    ):
        raise AccuracyEvalError(f"{case_id}: evidence.score 必须是有限浮点数")
    rank = item["rank"]
    if isinstance(rank, bool) or not isinstance(rank, int) or rank < 0:
        raise AccuracyEvalError(f"{case_id}: evidence.rank 必须是非负整数")
    for field in ("physicalPageStart", "physicalPageEnd"):
        page = item[field]
        if page is not None and (
            isinstance(page, bool) or not isinstance(page, int) or page < 0
        ):
            raise AccuracyEvalError(
                f"{case_id}: evidence.{field} 必须是非负整数或 null"
            )
    if "locator" in item and item["locator"] is not None:
        locator = item["locator"]
        if not isinstance(locator, dict) or set(locator) != set(LOCATOR_FIELDS):
            raise AccuracyEvalError(f"{case_id}: evidence.locator schema 不匹配")
        for field in ("documentId", "blockId", "markdownPath", "contentHash", "snapshotId"):
            if not isinstance(locator[field], str):
                raise AccuracyEvalError(f"{case_id}: evidence.locator.{field} 必须是字符串")
        if not isinstance(locator["headingPath"], list) or not all(
            isinstance(value, str) for value in locator["headingPath"]
        ):
            raise AccuracyEvalError(f"{case_id}: evidence.locator.headingPath 必须是字符串数组")
        for field in ("lineStart", "lineEnd"):
            value = locator[field]
            if value is not None and (
                isinstance(value, bool) or not isinstance(value, int) or value < 0
            ):
                raise AccuracyEvalError(
                    f"{case_id}: evidence.locator.{field} 必须是非负整数或 null"
                )
    return item


def canonical_evidence_bytes(item: dict[str, Any], case_id: str = "evidence") -> bytes:
    """Reproduce serde_json::to_vec(EvidenceItem) with a fixed field order."""

    validated = _validate_evidence_item(item, case_id)
    fields = EVIDENCE_FIELDS if "locator" in validated else EVIDENCE_FIELDS_V4
    ordered = {field: validated[field] for field in fields}
    if "locator" in ordered and ordered["locator"] is not None:
        locator = ordered["locator"]
        ordered["locator"] = {field: locator[field] for field in LOCATOR_FIELDS}
    try:
        return json.dumps(
            ordered,
            ensure_ascii=False,
            allow_nan=False,
            separators=(",", ":"),
        ).encode("utf-8")
    except (TypeError, ValueError, UnicodeError) as exc:
        raise AccuracyEvalError(f"{case_id}: evidence 无法规范序列化: {exc}") from exc


def evidence_sha256(item: dict[str, Any], case_id: str = "evidence") -> str:
    return hashlib.sha256(canonical_evidence_bytes(item, case_id)).hexdigest()


def stable_source_id(item: dict[str, Any]) -> str:
    locator = item.get("locator")
    if isinstance(locator, dict):
        return ":".join(
            (
                locator["documentId"],
                " > ".join(locator["headingPath"]),
                locator["blockId"],
            )
        )

    def rust_option(value: int | None) -> str:
        return "None" if value is None else f"Some({value})"

    return ":".join(
        (
            item["kind"],
            item["pageId"],
            item["nodeId"],
            item["chapterId"],
            item["sourceLocation"],
            rust_option(item["physicalPageStart"]),
            rust_option(item["physicalPageEnd"]),
        )
    )


def _validate_evidence_and_manifest(
    run: dict[str, Any], case_id: str
) -> tuple[set[str], dict[str, Any]]:
    if "evidence" not in run or not isinstance(run["evidence"], list):
        raise AccuracyEvalError(f"{case_id}: 缺少 evidence 数组")
    if "runManifest" not in run or not isinstance(run["runManifest"], dict):
        raise AccuracyEvalError(f"{case_id}: 缺少 runManifest 对象")
    evidence = run["evidence"]
    manifest = run["runManifest"]
    if not evidence:
        raise AccuracyEvalError(f"{case_id}: evidence 数组不能为空")
    checksums = manifest.get("evidenceChecksums")
    if not isinstance(checksums, list) or not checksums:
        raise AccuracyEvalError(f"{case_id}: manifest.evidenceChecksums 必须是非空数组")

    evidence_by_id: dict[str, dict[str, Any]] = {}
    for raw_item in evidence:
        item = _validate_evidence_item(raw_item, case_id)
        evidence_id = item["id"]
        if evidence_id in evidence_by_id:
            raise AccuracyEvalError(f"{case_id}: 重复 evidence id: {evidence_id}")
        evidence_by_id[evidence_id] = item

    checksum_by_id: dict[str, dict[str, Any]] = {}
    for checksum in checksums:
        if not isinstance(checksum, dict):
            raise AccuracyEvalError(f"{case_id}: evidence checksum 必须是对象")
        if set(checksum) != {"evidenceId", "stableSourceId", "sha256"}:
            raise AccuracyEvalError(f"{case_id}: evidence checksum schema 不匹配")
        evidence_id = checksum.get("evidenceId")
        digest = checksum.get("sha256")
        stable_id = checksum.get("stableSourceId")
        if not isinstance(evidence_id, str) or not evidence_id:
            raise AccuracyEvalError(f"{case_id}: checksum evidenceId 为空")
        if evidence_id in checksum_by_id:
            raise AccuracyEvalError(
                f"{case_id}: 重复 checksum evidenceId: {evidence_id}"
            )
        if not isinstance(stable_id, str) or not stable_id:
            raise AccuracyEvalError(f"{case_id}: checksum stableSourceId 为空")
        if not isinstance(digest, str) or not SHA256_RE.fullmatch(digest):
            raise AccuracyEvalError(f"{case_id}: checksum sha256 格式非法")
        checksum_by_id[evidence_id] = checksum

    if set(evidence_by_id) != set(checksum_by_id):
        raise AccuracyEvalError(
            f"{case_id}: evidence 与 manifest checksum ID 集合不一致"
        )
    for evidence_id, item in evidence_by_id.items():
        checksum = checksum_by_id[evidence_id]
        expected_digest = evidence_sha256(item, case_id)
        if not hmac.compare_digest(checksum["sha256"], expected_digest):
            raise AccuracyEvalError(
                f"{case_id}: evidence {evidence_id} sha256 校验失败"
            )
        expected_stable_id = stable_source_id(item)
        if checksum["stableSourceId"] != expected_stable_id:
            raise AccuracyEvalError(
                f"{case_id}: evidence {evidence_id} stableSourceId 校验失败"
            )
    return set(evidence_by_id), manifest


def _validate_answer_claims(
    run: dict[str, Any],
    manifest: dict[str, Any],
    known_evidence_ids: set[str],
    case_id: str,
) -> tuple[dict[str, str], dict[str, str], dict[str, list[str]]]:
    answer = run.get("answer")
    if not isinstance(answer, str) or not answer.strip():
        raise AccuracyEvalError(f"{case_id}: run 缺少非空 answer")
    claims = run.get("answerClaims")
    if not isinstance(claims, list) or not claims:
        raise AccuracyEvalError(f"{case_id}: answerClaims 必须是非空数组")

    completeness = manifest.get("answerCompleteness")
    if not isinstance(completeness, dict):
        raise AccuracyEvalError(f"{case_id}: 缺少 manifest.answerCompleteness")
    claim_count = completeness.get("claimCount")
    if (
        isinstance(claim_count, bool)
        or not isinstance(claim_count, int)
        or claim_count < 1
    ):
        raise AccuracyEvalError(f"{case_id}: answerCompleteness.claimCount 非法")
    if claim_count != len(claims):
        raise AccuracyEvalError(
            f"{case_id}: manifest claimCount={claim_count} 与 answerClaims={len(claims)} 不一致"
        )

    claim_text_by_id: dict[str, str] = {}
    claim_dimension_by_id: dict[str, str] = {}
    citations_by_claim: dict[str, list[str]] = {}
    for claim in claims:
        if not isinstance(claim, dict) or not {
            "claimId",
            "text",
            "citedEvidenceIds",
        }.issubset(claim) or set(claim) - {
            "claimId",
            "text",
            "citedEvidenceIds",
            "dimension",
        }:
            raise AccuracyEvalError(f"{case_id}: answer claim schema 不匹配")
        claim_id = claim.get("claimId")
        text = claim.get("text")
        citations = claim.get("citedEvidenceIds")
        if not isinstance(claim_id, str) or not claim_id.strip():
            raise AccuracyEvalError(f"{case_id}: answer claimId 为空")
        if claim_id in claim_text_by_id:
            raise AccuracyEvalError(f"{case_id}: 重复 answer claimId: {claim_id}")
        if not isinstance(text, str) or not text.strip() or text not in answer:
            raise AccuracyEvalError(
                f"{case_id}: claim {claim_id} 未逐字出现在 answer 中"
            )
        if not isinstance(citations, list) or any(
            not isinstance(citation, str) or not citation for citation in citations
        ):
            raise AccuracyEvalError(
                f"{case_id}: claim {claim_id} citedEvidenceIds 非法"
            )
        if len(citations) != len(set(citations)):
            raise AccuracyEvalError(f"{case_id}: claim {claim_id} 包含重复 citation ID")
        unknown_citations = sorted(set(citations) - known_evidence_ids)
        if unknown_citations:
            raise AccuracyEvalError(
                f"{case_id}: claim {claim_id} 包含未知 citedEvidenceIds: {unknown_citations}"
            )
        claim_text_by_id[claim_id] = text
        dimension = claim.get("dimension", "factual")
        if dimension not in VALID_DIMENSIONS:
            raise AccuracyEvalError(f"{case_id}: claim {claim_id} dimension 非法")
        claim_dimension_by_id[claim_id] = dimension
        citations_by_claim[claim_id] = citations
    return claim_text_by_id, claim_dimension_by_id, citations_by_claim


def _validate_reviewer_identity(reviewer: Any, case_id: str, label: str) -> str:
    if not isinstance(reviewer, dict):
        raise AccuracyEvalError(f"{case_id}: {label} 必须是对象")
    if reviewer.get("blinded") is not True or reviewer.get("independent") is not True:
        raise AccuracyEvalError(
            f"{case_id}: {label} 必须 blinded=true 且 independent=true"
        )
    reviewer_hash = reviewer.get("reviewer_id_hash")
    if not isinstance(reviewer_hash, str) or not SHA256_RE.fullmatch(reviewer_hash):
        raise AccuracyEvalError(
            f"{case_id}: {label} reviewer_id_hash 必须为小写 SHA-256"
        )
    return reviewer_hash


def _validate_claim_verdicts(
    claims: Any,
    expected_claims: dict[str, str],
    case_id: str,
    label: str,
) -> dict[str, str]:
    if not isinstance(claims, list):
        raise AccuracyEvalError(f"{case_id}: {label} 必须是数组")
    verdicts: dict[str, str] = {}
    for claim in claims:
        if not isinstance(claim, dict) or set(claim) != {
            "claim_id",
            "claim",
            "verdict",
        }:
            raise AccuracyEvalError(f"{case_id}: {label} claim schema 不匹配")
        claim_id = claim.get("claim_id")
        text = claim.get("claim")
        verdict = claim.get("verdict")
        if claim_id in verdicts:
            raise AccuracyEvalError(f"{case_id}: {label} 重复 claim_id: {claim_id}")
        if claim_id not in expected_claims:
            raise AccuracyEvalError(f"{case_id}: {label} 包含未知 claim_id: {claim_id}")
        if text != expected_claims[claim_id]:
            raise AccuracyEvalError(f"{case_id}: {label} claim {claim_id} 文本不匹配")
        if verdict not in VALID_VERDICTS:
            raise AccuracyEvalError(f"{case_id}: {label} claim verdict 非法")
        verdicts[claim_id] = verdict
    if set(verdicts) != set(expected_claims):
        missing = sorted(set(expected_claims) - set(verdicts))
        extra = sorted(set(verdicts) - set(expected_claims))
        raise AccuracyEvalError(
            f"{case_id}: {label} 未完整覆盖预期 claims; missing={missing}, extra={extra}"
        )
    return verdicts


def _validate_coverage_verdicts(
    entries: Any,
    expected_items: Sequence[str],
    item_key: str,
    valid_verdicts: set[str],
    case_id: str,
    label: str,
) -> dict[str, str]:
    if not isinstance(entries, list):
        raise AccuracyEvalError(f"{case_id}: {label} 必须是数组")
    expected = set(expected_items)
    verdicts: dict[str, str] = {}
    for entry in entries:
        if not isinstance(entry, dict) or set(entry) != {item_key, "verdict"}:
            raise AccuracyEvalError(f"{case_id}: {label} coverage schema 不匹配")
        item = entry.get(item_key)
        verdict = entry.get("verdict")
        if not isinstance(item, str) or item not in expected:
            raise AccuracyEvalError(f"{case_id}: {label} 包含未知 {item_key}: {item}")
        if item in verdicts:
            raise AccuracyEvalError(f"{case_id}: {label} 重复 {item_key}: {item}")
        if verdict not in valid_verdicts:
            raise AccuracyEvalError(f"{case_id}: {label} verdict 非法")
        verdicts[item] = verdict
    if set(verdicts) != expected:
        missing = sorted(expected - set(verdicts))
        extra = sorted(set(verdicts) - expected)
        raise AccuracyEvalError(
            f"{case_id}: {label} 未完整覆盖冻结期望集合; missing={missing}, extra={extra}"
        )
    return verdicts


def _validate_expected_coverage(
    case: dict[str, Any], field: str, case_id: str
) -> list[str]:
    values = case.get(field)
    if (
        not isinstance(values, list)
        or any(not isinstance(value, str) or not value.strip() for value in values)
        or len(values) != len(set(values))
    ):
        raise AccuracyEvalError(
            f"{case_id}: frozen case {field} 必须为无重复非空字符串数组"
        )
    return values


def _validate_reviewer(
    reviewer: Any,
    expected_claims: dict[str, str],
    expected_methods: Sequence[str],
    expected_constraints: Sequence[str],
    case_id: str,
    label: str,
) -> tuple[str, dict[str, str], dict[str, str], dict[str, str]]:
    reviewer_hash = _validate_reviewer_identity(reviewer, case_id, label)
    claims = _validate_claim_verdicts(
        reviewer.get("claims"), expected_claims, case_id, f"{label}.claims"
    )
    methods = _validate_coverage_verdicts(
        reviewer.get("method_coverage"),
        expected_methods,
        "method_family",
        VALID_METHOD_COVERAGE_VERDICTS,
        case_id,
        f"{label}.method_coverage",
    )
    constraints = _validate_coverage_verdicts(
        reviewer.get("constraint_coverage"),
        expected_constraints,
        "constraint",
        VALID_CONSTRAINT_COVERAGE_VERDICTS,
        case_id,
        f"{label}.constraint_coverage",
    )
    return reviewer_hash, claims, methods, constraints


def _final_verdicts(
    review: dict[str, Any],
    expected_claims: dict[str, str],
    expected_methods: Sequence[str],
    expected_constraints: Sequence[str],
    case_id: str,
) -> tuple[dict[str, str], dict[str, str], dict[str, str]]:
    if review.get("case_id") != case_id:
        raise AccuracyEvalError(f"{case_id}: review case_id 不匹配")
    primary = review.get("primary_reviews")
    if not isinstance(primary, list) or len(primary) != 2:
        raise AccuracyEvalError(f"{case_id}: 必须恰有两个独立 primary_reviews")
    first_hash, first_claims, first_methods, first_constraints = _validate_reviewer(
        primary[0],
        expected_claims,
        expected_methods,
        expected_constraints,
        case_id,
        "primary_reviews[0]",
    )
    second_hash, second_claims, second_methods, second_constraints = _validate_reviewer(
        primary[1],
        expected_claims,
        expected_methods,
        expected_constraints,
        case_id,
        "primary_reviews[1]",
    )
    if hmac.compare_digest(first_hash, second_hash):
        raise AccuracyEvalError(f"{case_id}: 两名 primary reviewer 必须不同")

    claim_disagreements = {
        claim_id
        for claim_id in expected_claims
        if first_claims[claim_id] != second_claims[claim_id]
    }
    method_disagreements = {
        method
        for method in expected_methods
        if first_methods[method] != second_methods[method]
    }
    constraint_disagreements = {
        constraint
        for constraint in expected_constraints
        if first_constraints[constraint] != second_constraints[constraint]
    }
    adjudication = review.get("adjudication")
    if not (claim_disagreements or method_disagreements or constraint_disagreements):
        if adjudication is not None:
            raise AccuracyEvalError(
                f"{case_id}: 无分歧时 adjudication 必须省略或为 null"
            )
        return first_claims, first_methods, first_constraints

    if not isinstance(adjudication, dict):
        raise AccuracyEvalError(f"{case_id}: primary reviewer 有分歧，缺少第三人裁决")
    adjudicator_hash = _validate_reviewer_identity(adjudication, case_id, "adjudication")
    if any(
        hmac.compare_digest(adjudicator_hash, value)
        for value in (first_hash, second_hash)
    ):
        raise AccuracyEvalError(
            f"{case_id}: adjudicator 必须不同于两名 primary reviewer"
        )
    adjudicated_claims = _validate_claim_verdicts(
        adjudication.get("claims"),
        {claim_id: expected_claims[claim_id] for claim_id in claim_disagreements},
        case_id,
        "adjudication.claims",
    )
    adjudicated_methods = _validate_coverage_verdicts(
        adjudication.get("method_coverage"),
        sorted(method_disagreements),
        "method_family",
        VALID_METHOD_COVERAGE_VERDICTS,
        case_id,
        "adjudication.method_coverage",
    )
    adjudicated_constraints = _validate_coverage_verdicts(
        adjudication.get("constraint_coverage"),
        sorted(constraint_disagreements),
        "constraint",
        VALID_CONSTRAINT_COVERAGE_VERDICTS,
        case_id,
        "adjudication.constraint_coverage",
    )
    return (
        {
            claim_id: adjudicated_claims.get(claim_id, first_claims[claim_id])
            for claim_id in expected_claims
        },
        {
            method: adjudicated_methods.get(method, first_methods[method])
            for method in expected_methods
        },
        {
            constraint: adjudicated_constraints.get(
                constraint, first_constraints[constraint]
            )
            for constraint in expected_constraints
        },
    )


def review_totals(
    run: dict[str, Any], review: dict[str, Any], case: dict[str, Any]
) -> Totals:
    case_id = case.get("id") if isinstance(case, dict) else None
    if not isinstance(case_id, str) or not case_id:
        raise AccuracyEvalError("frozen case 缺少非空 id")
    expected_methods = _validate_expected_coverage(
        case, "acceptableMethodFamilies", case_id
    )
    expected_constraints = _validate_expected_coverage(
        case, "criticalConstraints", case_id
    )
    known_ids, manifest = _validate_evidence_and_manifest(run, case_id)
    expected_claims, dimensions, citations_by_claim = _validate_answer_claims(
        run, manifest, known_ids, case_id
    )
    final_verdicts, final_methods, final_constraints = _final_verdicts(
        review,
        expected_claims,
        expected_methods,
        expected_constraints,
        case_id,
    )
    counts = {
        verdict: sum(value == verdict for value in final_verdicts.values())
        for verdict in VALID_VERDICTS
    }
    completeness = manifest["answerCompleteness"]
    applicable = {
        claim_id
        for claim_id, verdict in final_verdicts.items()
        if verdict != "not_applicable"
    }
    cited_ids = [
        citation
        for claim_id in applicable
        for citation in citations_by_claim[claim_id]
    ]
    reference_claims = [
        claim_id for claim_id in applicable if dimensions[claim_id] == "reference"
    ]
    reference_supported = sum(
        final_verdicts[claim_id] in {"supported", "partially_supported"}
        for claim_id in reference_claims
    )
    return Totals(
        supported=counts["supported"],
        partially_supported=counts["partially_supported"],
        unsupported=counts["unsupported"],
        contradicted=counts["contradicted"],
        not_verifiable=counts["not_verifiable"],
        not_applicable=counts["not_applicable"],
        cited_ids=len(cited_ids),
        known_cited_ids=sum(citation in known_ids for citation in cited_ids),
        applicable_claims=len(applicable),
        cited_claims=sum(bool(citations_by_claim[claim_id]) for claim_id in applicable),
        reference_supported=reference_supported,
        reference_total=len(reference_claims),
        method_supported=sum(verdict == "covered" for verdict in final_methods.values()),
        method_total=len(expected_methods),
        constraint_supported=sum(
            verdict == "preserved" for verdict in final_constraints.values()
        ),
        constraint_total=len(expected_constraints),
        complete_answers=int(completeness.get("complete") is True),
        reviewed_answers=1,
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="聚合 held-out 智能问答的独立双评审逐 claim 准确率"
    )
    parser.add_argument("--dataset", type=Path, default=DEFAULT_DATASET)
    parser.add_argument(
        "--runs-dir", type=Path, default=ROOT / "evals" / "heldout-runs"
    )
    parser.add_argument(
        "--reviews-dir", type=Path, default=ROOT / "evals" / "heldout-reviews"
    )
    parser.add_argument("--allow-pending", action="store_true")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        dataset = load_json(args.dataset)
        cases = validate_dataset(dataset)
        minimum = dataset["minimum_case_count"]
        if dataset.get("status") != "frozen" or len(cases) < minimum:
            print(
                "PENDING: held-out 集尚未独立冻结；"
                f"status={dataset.get('status')!r}, cases={len(cases)}, minimum={minimum}"
            )
            return 0 if args.allow_pending else 2
        totals = Totals()
        for case in cases:
            case_id = case["id"]
            run = load_json(args.runs_dir / f"{case_id}.json")
            review = load_json(args.reviews_dir / f"{case_id}.json")
            if run.get("question") != case["question"]:
                raise AccuracyEvalError(f"{case_id}: run question 与冻结题目不一致")
            totals = totals.add(review_totals(run, review, case))
    except AccuracyEvalError as exc:
        print(f"FAIL: {exc}")
        return 1

    supported_or_partial = totals.supported + totals.partially_supported
    factual_denominator = (
        supported_or_partial
        + totals.unsupported
        + totals.contradicted
        + totals.not_verifiable
    )
    factual_precision = (
        totals.supported / factual_denominator if factual_denominator else 0.0
    )
    low, high = wilson_interval(totals.supported, factual_denominator)
    citation_precision = (
        totals.known_cited_ids / totals.cited_ids if totals.cited_ids else 0.0
    )
    completeness_rate = totals.complete_answers / totals.reviewed_answers

    def ratio(numerator: int, denominator: int) -> float | None:
        return numerator / denominator if denominator else None

    print(
        json.dumps(
            {
                "status": "evaluated",
                "independentlyCurated": True,
                "datasetVersion": dataset.get("version"),
                "casesSha256": dataset.get("curation", {}).get("cases_sha256"),
                "cases": totals.reviewed_answers,
                "claims": {
                    "supported": totals.supported,
                    "partiallySupported": totals.partially_supported,
                    "unsupported": totals.unsupported,
                    "contradicted": totals.contradicted,
                    "notVerifiable": totals.not_verifiable,
                    "notApplicable": totals.not_applicable,
                },
                "factualPrecision": factual_precision,
                "factualPrecisionWilson95": [low, high],
                "claimSupportRate": ratio(
                    supported_or_partial, factual_denominator
                ),
                "partialSupportRate": ratio(
                    totals.partially_supported, factual_denominator
                ),
                "unsupportedFactualClaimRate": ratio(
                    totals.unsupported + totals.not_verifiable,
                    factual_denominator,
                ),
                "contradictedClaimRate": ratio(
                    totals.contradicted, factual_denominator
                ),
                "notApplicableRate": ratio(
                    totals.not_applicable,
                    totals.applicable_claims + totals.not_applicable,
                ),
                "citationIdPrecision": citation_precision,
                "citationCompleteness": ratio(
                    totals.cited_claims, totals.applicable_claims
                ),
                "referenceSupportRate": ratio(
                    totals.reference_supported, totals.reference_total
                ),
                "relevantMethodRecall": ratio(
                    totals.method_supported, totals.method_total
                ),
                "criticalConstraintPreservation": ratio(
                    totals.constraint_supported, totals.constraint_total
                ),
                "answerCompletenessRate": completeness_rate,
                "semanticEntailmentChecked": False,
                "reviewProtocol": "independent_double_review_with_adjudication",
            },
            ensure_ascii=False,
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
