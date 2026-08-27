"""Shared canonical contract for independently frozen QA held-out datasets."""

from __future__ import annotations

import hashlib
import hmac
import json
import re
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
CONTRACT_PATH = ROOT / "evals" / "heldout_contract.json"
SHA256_RE = re.compile(r"^[0-9a-f]{64}$")


class HeldoutContractError(ValueError):
    pass


def load_contract(path: Path = CONTRACT_PATH) -> dict[str, Any]:
    try:
        contract = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise HeldoutContractError(f"读取 held-out contract 失败: {exc}") from exc
    if not isinstance(contract, dict) or contract.get("schemaVersion") != "qa-heldout-contract-v1":
        raise HeldoutContractError("held-out contract schemaVersion 不受支持")
    allowed = contract.get("allowedTypes")
    if (
        not isinstance(allowed, list)
        or not allowed
        or len(set(allowed)) != len(allowed)
        or any(not isinstance(value, str) or not value.strip() for value in allowed)
    ):
        raise HeldoutContractError("held-out contract allowedTypes 非法")
    minimum = contract.get("minimumCaseCount")
    if isinstance(minimum, bool) or not isinstance(minimum, int) or minimum < 30:
        raise HeldoutContractError("held-out contract minimumCaseCount 必须至少为 30")
    return contract


CONTRACT = load_contract()
VALID_HELDOUT_TYPES = frozenset(CONTRACT["allowedTypes"])


def canonical_cases_bytes(cases: list[dict[str, Any]]) -> bytes:
    try:
        return json.dumps(
            cases,
            ensure_ascii=False,
            allow_nan=False,
            sort_keys=True,
            separators=(",", ":"),
        ).encode("utf-8")
    except (TypeError, ValueError, UnicodeError) as exc:
        raise HeldoutContractError(f"held-out cases 无法规范序列化: {exc}") from exc


def cases_sha256(cases: list[dict[str, Any]]) -> str:
    return hashlib.sha256(canonical_cases_bytes(cases)).hexdigest()


def validate_dataset(
    dataset: dict[str, Any], *, require_frozen: bool = False
) -> list[dict[str, Any]]:
    if (
        dataset.get("dataset_role") != CONTRACT["datasetRole"]
        or dataset.get("split") != CONTRACT["split"]
    ):
        raise HeldoutContractError("数据集必须标记为 production_accuracy / heldout")
    minimum = dataset.get("minimum_case_count")
    if (
        isinstance(minimum, bool)
        or not isinstance(minimum, int)
        or minimum < CONTRACT["minimumCaseCount"]
    ):
        raise HeldoutContractError("minimum_case_count 必须至少为 30")
    schema = dataset.get("case_schema")
    if schema is not None:
        allowed = schema.get("allowed_types") if isinstance(schema, dict) else None
        if allowed != CONTRACT["allowedTypes"]:
            raise HeldoutContractError("case_schema.allowed_types 与 canonical contract 不一致")
    cases = dataset.get("cases")
    if not isinstance(cases, list):
        raise HeldoutContractError("held-out 数据集缺少 cases 数组")
    seen: set[str] = set()
    for case in cases:
        if not isinstance(case, dict) or not all(
            isinstance(case.get(key), str) and case[key].strip()
            for key in ("id", "type", "question")
        ):
            raise HeldoutContractError("每个 held-out case 必须包含非空 id/type/question")
        if case["type"] not in VALID_HELDOUT_TYPES:
            raise HeldoutContractError(f"{case['id']}: held-out type 非法")
        if case["id"] in seen:
            raise HeldoutContractError(f"重复 held-out case id: {case['id']}")
        seen.add(case["id"])

    frozen = dataset.get("status") == "frozen"
    if require_frozen and not frozen:
        raise HeldoutContractError("held-out 数据集尚未 frozen")
    if frozen:
        if len(cases) < minimum:
            raise HeldoutContractError("frozen held-out cases 少于 minimum_case_count")
        curation = dataset.get("curation")
        if not isinstance(curation, dict) or curation.get("independent") is not True:
            raise HeldoutContractError("frozen held-out 数据集缺少独立 curation 证明")
        curator_hash = curation.get("curator_id_hash")
        if not isinstance(curator_hash, str) or not SHA256_RE.fullmatch(curator_hash):
            raise HeldoutContractError("curation.curator_id_hash 必须为小写 SHA-256")
        frozen_at = curation.get("frozen_at")
        if not isinstance(frozen_at, str) or not frozen_at.strip():
            raise HeldoutContractError("curation.frozen_at 不能为空")
        expected = curation.get("cases_sha256")
        actual = cases_sha256(cases)
        if not isinstance(expected, str) or not hmac.compare_digest(expected, actual):
            raise HeldoutContractError("frozen held-out cases_sha256 校验失败")
    return cases
