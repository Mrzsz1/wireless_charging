from __future__ import annotations

import argparse
import json
import re
from collections import Counter
from pathlib import Path
from typing import Any, Sequence


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_GOLD = ROOT / "evals" / "gold_questions.json"
EXPECTED_COUNTS = {"solve": 5, "novelty": 3, "relationship": 2}

# Gold questions use concise Chinese labels; answers may use a documented
# equivalent phrasing. Keep this mapping small and explicit so the check does
# not become a generic keyword-quality score.
MENTION_ALIASES: dict[str, tuple[str, ...]] = {
    "非线性叠加": ("非线性叠加", "干涉叠加"),
    "适用前提": ("适用前提", "前提", "假设"),
    "分段常数": ("分段常数", "分段功率"),
    "功率可加": ("功率可加", "可加功率", "加性功率"),
    "最大峰值 AoI": ("最大峰值 aoi", "峰值 aoi", "最大峰值信息年龄"),
    "基于当前知识库": ("基于当前知识库", "基于当前库", "就本库而言"),
    "未完整覆盖": ("未完整覆盖", "部分覆盖", "尚未完整覆盖"),
    "候选不计入水位": ("候选不计入水位", "不计入水位", "尚未计入水位"),
}


class EvalContractError(ValueError):
    pass


def load_gold(path: Path) -> dict[str, Any]:
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise EvalContractError(f"无法读取题集 {path}: {exc}") from exc
    if not isinstance(payload, dict) or not isinstance(payload.get("cases"), list):
        raise EvalContractError("题集必须是包含 cases 数组的 JSON 对象")
    return payload


def wiki_ids(wiki_root: Path) -> set[str]:
    ids: set[str] = set()
    for path in wiki_root.rglob("*.md"):
        rel = path.relative_to(wiki_root).with_suffix("").as_posix()
        ids.add(rel)
        ids.add(path.stem)
    return ids


def validate_contract(payload: dict[str, Any], wiki_root: Path) -> list[str]:
    errors: list[str] = []
    cases = payload["cases"]
    if len(cases) != 10:
        errors.append(f"应有 10 个用例，实际 {len(cases)}")

    ids = [case.get("id") for case in cases if isinstance(case, dict)]
    duplicates = sorted(key for key, count in Counter(ids).items() if key and count > 1)
    if duplicates:
        errors.append(f"重复 case id: {', '.join(duplicates)}")

    counts = Counter(case.get("type") for case in cases if isinstance(case, dict))
    if dict(counts) != EXPECTED_COUNTS:
        errors.append(f"类型配额应为 {EXPECTED_COUNTS}，实际 {dict(counts)}")

    existing = wiki_ids(wiki_root)
    for index, case in enumerate(cases, start=1):
        if not isinstance(case, dict):
            errors.append(f"第 {index} 个用例不是对象")
            continue
        case_id = case.get("id") or f"#{index}"
        for field in ("id", "type", "question", "expected_wikilinks", "must_mention"):
            if not case.get(field):
                errors.append(f"{case_id}: 缺少 {field}")
        if case.get("waterline_required") is not True:
            errors.append(f"{case_id}: waterline_required 必须为 true")
        links = case.get("expected_wikilinks", [])
        if "maps/library-status" not in links:
            errors.append(f"{case_id}: expected_wikilinks 缺少 maps/library-status")
        for link in links:
            if link not in existing:
                errors.append(f"{case_id}: wikilink 目标不存在: {link}")
    return errors


def validate_answers(payload: dict[str, Any], answers_dir: Path) -> list[str]:
    errors: list[str] = []
    library_status = ROOT / "wiki" / "maps" / "library-status.md"
    status_text = library_status.read_text(encoding="utf-8-sig")
    match = re.search(r"^source_count:\s*(\d+)", status_text, re.M)
    source_count = int(match.group(1)) if match else None
    for case in payload["cases"]:
        case_id = case["id"]
        answer_path = answers_dir / f"{case_id}.md"
        if not answer_path.exists():
            errors.append(f"{case_id}: 缺少答案文件 {answer_path}")
            continue
        answer = answer_path.read_text(encoding="utf-8")
        for link in case["expected_wikilinks"]:
            if f"[[{link}" not in answer and f"[[{Path(link).name}" not in answer:
                errors.append(f"{case_id}: 答案缺少 [[{link}]]")
        if case["waterline_required"] and "库水位" not in answer:
            errors.append(f"{case_id}: 答案未明确说明库水位")
        if case["waterline_required"] and source_count is not None:
            if not re.search(rf"\b{source_count}\s*篇\s*source\b", answer, re.I):
                errors.append(f"{case_id}: 答案水位不是当前 {source_count} 篇 source")
        normalized = answer.casefold()
        missing_mentions = []
        for mention in case.get("must_mention", []):
            variants = MENTION_ALIASES.get(mention, (mention,))
            if not any(variant.casefold() in normalized for variant in variants):
                missing_mentions.append(mention)
        if missing_mentions:
            errors.append(
                f"{case_id}: 答案缺少必提概念: {', '.join(missing_mentions)}"
            )
    return errors


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="校验无线充电 LLM Wiki 的 10 条问答回归契约")
    parser.add_argument("--gold", type=Path, default=DEFAULT_GOLD)
    parser.add_argument("--wiki-root", type=Path, default=ROOT / "wiki")
    parser.add_argument("--answers-dir", type=Path)
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        payload = load_gold(args.gold)
    except EvalContractError as exc:
        print(f"FAIL: {exc}")
        return 1

    errors = validate_contract(payload, args.wiki_root)
    if args.answers_dir is not None:
        errors.extend(validate_answers(payload, args.answers_dir))
    if errors:
        print("FAIL")
        for error in errors:
            print(f"- {error}")
        return 1
    print(f"PASS: {len(payload['cases'])} cases; type counts {EXPECTED_COUNTS}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
