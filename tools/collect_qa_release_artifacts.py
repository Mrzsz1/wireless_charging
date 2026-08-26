#!/usr/bin/env python3
"""Collect safe QA metrics into one immutable release-candidate run directory."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import sys
import math
from pathlib import Path
from typing import Any, Sequence

from check_qa_release_gate import evaluate_release, render_report
from qa_eval_metadata import MetadataValidationError, validate_metadata_envelope

ROOT = Path(__file__).resolve().parents[1]
ALLOWED_ARTIFACTS = {
    "retrieval",
    "conversation",
    "grounding",
    "open_research",
    "reliability",
    "reranker",
    "semantic_verifier",
    "heldout",
    "performance",
}
FORBIDDEN_METRIC_KEYS = {
    "apikey", "api_key", "authorization", "password", "secret", "token",
    "question", "answer", "conversation", "content", "snippet", "prompt",
    "response", "path", "url", "endpoint",
}


class CollectionError(ValueError):
    pass


def _load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise CollectionError(f"cannot read {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise CollectionError(f"{path}: top-level JSON must be an object")
    return value


def _atomic_json(path: Path, value: dict[str, Any]) -> None:
    temporary = path.with_suffix(path.suffix + ".part")
    temporary.write_text(
        json.dumps(value, ensure_ascii=False, allow_nan=False, sort_keys=True, indent=2)
        + "\n",
        encoding="utf-8",
    )
    os.replace(temporary, path)


def _parse_artifact(value: str) -> tuple[str, Path]:
    name, separator, raw_path = value.partition("=")
    if separator != "=" or name not in ALLOWED_ARTIFACTS or not raw_path:
        raise argparse.ArgumentTypeError(
            "artifact must be one of "
            + ", ".join(sorted(ALLOWED_ARTIFACTS))
            + " in NAME=PATH form"
        )
    return name, Path(raw_path)


def _validate_safe_metrics(value: Any, location: str = "metrics") -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            normalized = str(key).replace("-", "_").lower()
            if normalized in FORBIDDEN_METRIC_KEYS:
                raise CollectionError(f"unsafe metric key: {location}.{key}")
            _validate_safe_metrics(child, f"{location}.{key}")
    elif isinstance(value, list):
        for index, child in enumerate(value):
            _validate_safe_metrics(child, f"{location}[{index}]")
    elif isinstance(value, float) and not math.isfinite(value):
        raise CollectionError(f"non-finite metric: {location}")
    elif not isinstance(value, (str, int, float, bool, type(None))):
        raise CollectionError(f"unsupported metric value: {location}")


def collect(
    run_dir: Path,
    metadata: dict[str, Any],
    artifacts: list[tuple[str, Path]],
) -> list[Path]:
    metric_artifacts: dict[str, dict[str, Any]] = {}
    for name, source in artifacts:
        payload = _load_json(source)
        metrics = payload.get("metrics", payload)
        if not isinstance(metrics, dict):
            raise CollectionError(f"{source}: metrics must be an object")
        metric_artifacts[name] = metrics
    return collect_metrics(run_dir, metadata, metric_artifacts)


def collect_metrics(
    run_dir: Path,
    metadata: dict[str, Any],
    artifacts: dict[str, dict[str, Any]],
) -> list[Path]:
    validate_metadata_envelope(metadata)
    unknown = set(artifacts) - ALLOWED_ARTIFACTS
    if unknown:
        raise CollectionError(f"unsupported artifacts: {sorted(unknown)}")
    if run_dir.exists() and any(run_dir.iterdir()):
        raise CollectionError(f"run directory must be new or empty: {run_dir}")
    run_dir.mkdir(parents=True, exist_ok=True)
    written: list[Path] = []
    try:
        for name, metrics in artifacts.items():
            _validate_safe_metrics(metrics)
            destination = run_dir / f"{name}.json"
            _atomic_json(destination, {"metadata": metadata, "metrics": metrics})
            written.append(destination)
    except (OSError, ValueError, MetadataValidationError) as exc:
        shutil.rmtree(run_dir, ignore_errors=True)
        if isinstance(exc, CollectionError):
            raise
        raise CollectionError(str(exc)) from exc
    return written


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Collect QA release artifacts")
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--metadata", type=Path, required=True)
    parser.add_argument(
        "--artifact", action="append", type=_parse_artifact, default=[], metavar="NAME=PATH"
    )
    parser.add_argument("--report", type=Path, default=ROOT / "QA_PRODUCTION_RELEASE_REPORT.md")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        metadata = _load_json(args.metadata)
        collect(args.run_dir, metadata, args.artifact)
        decision = evaluate_release(args.run_dir)
        _atomic_json(args.run_dir / "release-decision.json", decision)
        args.report.write_text(render_report(decision, args.run_dir), encoding="utf-8")
    except (CollectionError, MetadataValidationError, ValueError, OSError) as exc:
        print(f"FAIL: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(decision, ensure_ascii=False, indent=2))
    return 0 if decision["decision"] == "PASS" else 1


if __name__ == "__main__":
    raise SystemExit(main())
