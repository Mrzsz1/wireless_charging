from __future__ import annotations

import argparse
import hashlib
import hmac
import json
import math
import re
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Sequence

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_DATASET = ROOT / "evals" / "heldout_questions.json"
VALID_VERDICTS = {"supported", "contradicted", "not_verifiable"}
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")

# This is the serde field order of apps/desktop/src-tauri/src/qa.rs::EvidenceItem.
# QaRunManifest hashes serde_json::to_vec(EvidenceItem), so the evaluator must
# rebuild exactly that byte representation rather than hashing the arbitrary key
# order found in a hand-edited JSON audit bundle.
EVIDENCE_FIELDS = (
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
EVIDENCE_STRING_FIELDS = set(EVIDENCE_FIELDS) - {
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
    contradicted: int = 0
    not_verifiable: int = 0
    cited_ids: int = 0
    known_cited_ids: int = 0
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
    if (
        dataset.get("dataset_role") != "production_accuracy"
        or dataset.get("split") != "heldout"
    ):
        raise AccuracyEvalError("数据集必须标记为 production_accuracy / heldout")
    minimum = dataset.get("minimum_case_count")
    if isinstance(minimum, bool) or not isinstance(minimum, int) or minimum <= 0:
        raise AccuracyEvalError("minimum_case_count 必须是正整数")
    cases = dataset.get("cases")
    if not isinstance(cases, list):
        raise AccuracyEvalError("held-out 数据集缺少 cases 数组")
    allowed_types = {"solve", "novelty", "relationship"}
    seen: set[str] = set()
    for case in cases:
        if not isinstance(case, dict) or not all(
            case.get(key) for key in ("id", "type", "question")
        ):
            raise AccuracyEvalError("每个 held-out case 必须包含 id/type/question")
        if case["type"] not in allowed_types:
            raise AccuracyEvalError(f"{case['id']}: held-out type 非法")
        if case["id"] in seen:
            raise AccuracyEvalError(f"重复 held-out case id: {case['id']}")
        seen.add(case["id"])
    return cases


def _validate_evidence_item(item: Any, case_id: str) -> dict[str, Any]:
    if not isinstance(item, dict):
        raise AccuracyEvalError(f"{case_id}: evidence item 必须是对象")
    keys = set(item)
    required = set(EVIDENCE_FIELDS)
    if keys != required:
        missing = sorted(required - keys)
        extra = sorted(keys - required)
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
    return item


def canonical_evidence_bytes(item: dict[str, Any], case_id: str = "evidence") -> bytes:
    """Reproduce serde_json::to_vec(EvidenceItem) with a fixed field order."""

    validated = _validate_evidence_item(item, case_id)
    ordered = {field: validated[field] for field in EVIDENCE_FIELDS}
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
    run: dict[str, Any], manifest: dict[str, Any], case_id: str
) -> tuple[dict[str, str], list[str]]:
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
    cited_ids: list[str] = []
    for claim in claims:
        if not isinstance(claim, dict) or set(claim) != {
            "claimId",
            "text",
            "citedEvidenceIds",
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
        for citation in citations:
            if f"[{citation}]" not in text:
                raise AccuracyEvalError(
                    f"{case_id}: claim {claim_id} 的 [{citation}] 未出现在该 claim 文本中"
                )
        claim_text_by_id[claim_id] = text
        cited_ids.extend(citations)
    return claim_text_by_id, cited_ids


def _validate_reviewer(
    reviewer: Any,
    expected_claims: dict[str, str],
    case_id: str,
    label: str,
) -> tuple[str, dict[str, str]]:
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
    claims = reviewer.get("claims")
    if not isinstance(claims, list):
        raise AccuracyEvalError(f"{case_id}: {label}.claims 必须是数组")
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
            f"{case_id}: {label} 未完整覆盖 answer claims; missing={missing}, extra={extra}"
        )
    return reviewer_hash, verdicts


def _final_verdicts(
    review: dict[str, Any], expected_claims: dict[str, str], case_id: str
) -> dict[str, str]:
    if review.get("case_id") != case_id:
        raise AccuracyEvalError(f"{case_id}: review case_id 不匹配")
    primary = review.get("primary_reviews")
    if not isinstance(primary, list) or len(primary) != 2:
        raise AccuracyEvalError(f"{case_id}: 必须恰有两个独立 primary_reviews")
    first_hash, first = _validate_reviewer(
        primary[0], expected_claims, case_id, "primary_reviews[0]"
    )
    second_hash, second = _validate_reviewer(
        primary[1], expected_claims, case_id, "primary_reviews[1]"
    )
    if hmac.compare_digest(first_hash, second_hash):
        raise AccuracyEvalError(f"{case_id}: 两名 primary reviewer 必须不同")

    disagreements = {
        claim_id for claim_id in expected_claims if first[claim_id] != second[claim_id]
    }
    adjudication = review.get("adjudication")
    if not disagreements:
        if adjudication is not None:
            raise AccuracyEvalError(
                f"{case_id}: 无分歧时 adjudication 必须省略或为 null"
            )
        return first

    if not isinstance(adjudication, dict):
        raise AccuracyEvalError(f"{case_id}: primary reviewer 有分歧，缺少第三人裁决")
    if (
        adjudication.get("blinded") is not True
        or adjudication.get("independent") is not True
    ):
        raise AccuracyEvalError(
            f"{case_id}: adjudication 必须 blinded=true 且 independent=true"
        )
    adjudicator_hash = adjudication.get("reviewer_id_hash")
    if not isinstance(adjudicator_hash, str) or not SHA256_RE.fullmatch(
        adjudicator_hash
    ):
        raise AccuracyEvalError(
            f"{case_id}: adjudicator reviewer_id_hash 必须为小写 SHA-256"
        )
    if any(
        hmac.compare_digest(adjudicator_hash, value)
        for value in (first_hash, second_hash)
    ):
        raise AccuracyEvalError(
            f"{case_id}: adjudicator 必须不同于两名 primary reviewer"
        )
    claims = adjudication.get("claims")
    if not isinstance(claims, list):
        raise AccuracyEvalError(f"{case_id}: adjudication.claims 必须是数组")
    adjudicated: dict[str, str] = {}
    for claim in claims:
        if not isinstance(claim, dict) or set(claim) != {
            "claim_id",
            "claim",
            "verdict",
        }:
            raise AccuracyEvalError(f"{case_id}: adjudication claim schema 不匹配")
        claim_id = claim.get("claim_id")
        if claim_id in adjudicated:
            raise AccuracyEvalError(
                f"{case_id}: adjudication 重复 claim_id: {claim_id}"
            )
        if claim_id not in disagreements:
            raise AccuracyEvalError(
                f"{case_id}: adjudication 包含非分歧 claim: {claim_id}"
            )
        if claim.get("claim") != expected_claims[claim_id]:
            raise AccuracyEvalError(
                f"{case_id}: adjudication claim {claim_id} 文本不匹配"
            )
        verdict = claim.get("verdict")
        if verdict not in VALID_VERDICTS:
            raise AccuracyEvalError(f"{case_id}: adjudication verdict 非法")
        adjudicated[claim_id] = verdict
    if set(adjudicated) != disagreements:
        missing = sorted(disagreements - set(adjudicated))
        raise AccuracyEvalError(f"{case_id}: adjudication 未覆盖全部分歧: {missing}")

    return {
        claim_id: adjudicated.get(claim_id, first[claim_id])
        for claim_id in expected_claims
    }


def review_totals(run: dict[str, Any], review: dict[str, Any], case_id: str) -> Totals:
    known_ids, manifest = _validate_evidence_and_manifest(run, case_id)
    expected_claims, cited_ids = _validate_answer_claims(run, manifest, case_id)
    final_verdicts = _final_verdicts(review, expected_claims, case_id)
    counts = {
        verdict: sum(value == verdict for value in final_verdicts.values())
        for verdict in VALID_VERDICTS
    }
    completeness = manifest["answerCompleteness"]
    return Totals(
        supported=counts["supported"],
        contradicted=counts["contradicted"],
        not_verifiable=counts["not_verifiable"],
        cited_ids=len(cited_ids),
        known_cited_ids=sum(citation in known_ids for citation in cited_ids),
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
            totals = totals.add(review_totals(run, review, case_id))
    except AccuracyEvalError as exc:
        print(f"FAIL: {exc}")
        return 1

    verifiable = totals.supported + totals.contradicted
    all_claims = verifiable + totals.not_verifiable
    factual_precision = totals.supported / verifiable if verifiable else 0.0
    low, high = wilson_interval(totals.supported, verifiable)
    citation_precision = (
        totals.known_cited_ids / totals.cited_ids if totals.cited_ids else 0.0
    )
    completeness_rate = totals.complete_answers / totals.reviewed_answers
    print(
        json.dumps(
            {
                "status": "evaluated",
                "cases": totals.reviewed_answers,
                "claims": {
                    "supported": totals.supported,
                    "contradicted": totals.contradicted,
                    "notVerifiable": totals.not_verifiable,
                },
                "factualPrecision": factual_precision,
                "factualPrecisionWilson95": [low, high],
                "claimSupportRate": (
                    totals.supported / all_claims if all_claims else 0.0
                ),
                "notVerifiableRate": (
                    totals.not_verifiable / all_claims if all_claims else 0.0
                ),
                "citationIdPrecision": citation_precision,
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
