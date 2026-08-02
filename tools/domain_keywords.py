#!/usr/bin/env python3
"""Inspect source-level paper keywords and validate the domain-keyword map.

This tool is deliberately read-only. It never invents keywords, edits source
frontmatter, or promotes terms into schema/vocab.yaml.
"""

from __future__ import annotations

import argparse
import ast
import json
import re
import sys
from collections import Counter
from pathlib import Path
from typing import Any, Sequence


PROJECT_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_SOURCES = PROJECT_ROOT / "wiki" / "sources"
DEFAULT_MAP = PROJECT_ROOT / "wiki" / "maps" / "map-domain-keywords.md"
ALLOWED_SOURCES = {"author_keywords", "index_terms", "not_found"}


def frontmatter(text: str) -> str:
    if not text.startswith("---"):
        return ""
    parts = text.split("---", 2)
    return parts[1] if len(parts) == 3 else ""


def scalar(block: str, key: str) -> str | None:
    match = re.search(rf"(?m)^{re.escape(key)}:\s*(.*?)\s*$", block)
    if not match:
        return None
    return match.group(1).strip().strip('"\'')


def keyword_list(block: str) -> list[str] | None:
    raw = scalar(block, "paper_keywords")
    if raw is None:
        return None
    try:
        value = ast.literal_eval(raw)
    except (SyntaxError, ValueError):
        return None
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        return None
    return [item.strip() for item in value if item.strip()]


def collect(sources_dir: Path) -> tuple[list[dict[str, Any]], list[str]]:
    records: list[dict[str, Any]] = []
    errors: list[str] = []
    for path in sorted(sources_dir.glob("src-*.md")):
        block = frontmatter(path.read_text(encoding="utf-8-sig"))
        # Paper keyword coverage intentionally excludes core books. Books use
        # chapter/index terms and are measured by core-book retrieval tests.
        if scalar(block, "source_type") == "book":
            continue
        keywords = keyword_list(block)
        provenance = scalar(block, "keyword_source")
        if keywords is None:
            errors.append(f"{path.name}: missing or invalid paper_keywords")
            keywords = []
        if provenance not in ALLOWED_SOURCES:
            errors.append(f"{path.name}: invalid keyword_source={provenance!r}")
        if provenance == "not_found" and keywords:
            errors.append(f"{path.name}: not_found must use an empty paper_keywords list")
        if provenance in {"author_keywords", "index_terms"} and not keywords:
            errors.append(f"{path.name}: {provenance} must contain at least one keyword")
        records.append(
            {
                "source": path.stem,
                "keyword_source": provenance,
                "paper_keywords": keywords,
            }
        )
    return records, errors


def validate_map(records: list[dict[str, Any]], map_path: Path) -> list[str]:
    if not map_path.exists():
        return [f"missing keyword map: {map_path}"]
    map_text = map_path.read_text(encoding="utf-8-sig").casefold()
    errors: list[str] = []
    for record in records:
        source = str(record["source"])
        keywords = list(record["paper_keywords"])
        if keywords and source.casefold() not in map_text:
            errors.append(f"{source}: source link missing from keyword map")
        for keyword in keywords:
            if keyword.casefold() not in map_text:
                errors.append(f"{source}: keyword missing from map: {keyword}")
    return errors


def summary(records: list[dict[str, Any]]) -> dict[str, Any]:
    counts = Counter(
        keyword.casefold()
        for record in records
        for keyword in record["paper_keywords"]
    )
    return {
        "source_count": len(records),
        "source_with_keywords_count": sum(bool(record["paper_keywords"]) for record in records),
        "paper_keyword_occurrence_count": sum(counts.values()),
        "distinct_literal_keyword_count": len(counts),
        "repeated_literal_keywords": [
            {"keyword": keyword, "count": count}
            for keyword, count in sorted(counts.items(), key=lambda item: (-item[1], item[0]))
            if count >= 2
        ],
    }


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="检查论文关键词元数据与领域关键词地图。")
    parser.add_argument("--sources-dir", type=Path, default=DEFAULT_SOURCES)
    parser.add_argument("--map", type=Path, default=DEFAULT_MAP)
    parser.add_argument("--check", action="store_true", help="检查字段、来源枚举与地图覆盖")
    parser.add_argument("--json", action="store_true", help="输出 JSON 摘要")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    records, errors = collect(args.sources_dir.resolve())
    if args.check:
        errors.extend(validate_map(records, args.map.resolve()))
    payload = summary(records)
    payload["errors"] = errors
    if args.json:
        print(json.dumps(payload, ensure_ascii=False, indent=2))
    else:
        print(
            "关键词覆盖："
            f"{payload['source_with_keywords_count']}/{payload['source_count']} sources，"
            f"{payload['paper_keyword_occurrence_count']} 次出现，"
            f"{payload['distinct_literal_keyword_count']} 个原词（大小写归一）。"
        )
        for item in payload["repeated_literal_keywords"]:
            print(f"- {item['keyword']}: {item['count']}")
        if errors:
            print("检查失败：", file=sys.stderr)
            for error in errors:
                print(f"- {error}", file=sys.stderr)
        elif args.check:
            print("关键词字段与地图覆盖检查通过。")
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
