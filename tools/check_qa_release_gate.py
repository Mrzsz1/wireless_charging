#!/usr/bin/env python3
"""Fail-closed production release gate for Research QA evaluation artifacts."""

from __future__ import annotations

import argparse
import json
import math
import operator
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Sequence

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT / "tools") not in sys.path:
    sys.path.insert(0, str(ROOT / "tools"))
from qa_eval_metadata import MetadataValidationError, validate_metadata_envelope

DEFAULT_THRESHOLDS = ROOT / "evals" / "qa_release_thresholds.json"
DEFAULT_ARTIFACTS = ROOT / "evals" / "runs" / "current"
DEFAULT_REPORT = ROOT / "QA_PRODUCTION_RELEASE_REPORT.md"


class ReleaseGateError(ValueError):
    pass


@dataclass(frozen=True)
class GateResult:
    gate_id: str
    passed: bool
    core: bool
    actual: Any
    expected: str
    reason: str


def load_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise ReleaseGateError(f"cannot read {path}: {exc}") from exc
    if not isinstance(value, dict):
        raise ReleaseGateError(f"{path}: top-level JSON must be an object")
    return value


def _metric(metrics: dict[str, Any], path: str) -> Any:
    current: Any = metrics
    for component in path.split("."):
        if not isinstance(current, dict) or component not in current:
            raise ReleaseGateError(f"missing metric {path}")
        current = current[component]
    if isinstance(current, float) and not math.isfinite(current):
        raise ReleaseGateError(f"non-finite metric {path}")
    if not isinstance(current, (bool, int, float)):
        raise ReleaseGateError(f"metric {path} must be numeric or boolean")
    return current


def _expected_text(gate: dict[str, Any], threshold: Any) -> str:
    return f"{gate['operator']} {threshold!r}"


def _compare(actual: Any, op: str, threshold: Any) -> bool:
    operations = {">=": operator.ge, "<=": operator.le, "==": operator.eq}
    if op not in operations:
        raise ReleaseGateError(f"unsupported operator {op!r}")
    if isinstance(threshold, bool):
        if not isinstance(actual, bool):
            raise ReleaseGateError("boolean threshold requires boolean metric")
    elif isinstance(actual, bool) or not isinstance(actual, (int, float)):
        raise ReleaseGateError("numeric threshold requires numeric metric")
    return bool(operations[op](actual, threshold))


def validate_thresholds(config: dict[str, Any]) -> list[dict[str, Any]]:
    if config.get("schemaVersion") != 1 or config.get("frozen") is not True:
        raise ReleaseGateError("release thresholds must be schemaVersion=1 and frozen=true")
    policy = config.get("policy")
    if not isinstance(policy, dict) or policy.get("missingArtifactDecision") != "FAIL":
        raise ReleaseGateError("threshold policy must fail closed on missing artifacts")
    gates = config.get("gates")
    if not isinstance(gates, list) or not gates:
        raise ReleaseGateError("threshold config has no gates")
    seen: set[str] = set()
    for gate in gates:
        if not isinstance(gate, dict):
            raise ReleaseGateError("gate entry must be an object")
        required = {"id", "artifact", "metric", "operator", "core"}
        if not required.issubset(gate):
            raise ReleaseGateError(f"gate missing fields: {sorted(required - set(gate))}")
        gate_id = gate["id"]
        if not isinstance(gate_id, str) or not gate_id or gate_id in seen:
            raise ReleaseGateError(f"invalid or duplicate gate id {gate_id!r}")
        seen.add(gate_id)
        if gate["operator"] == "<=metric":
            if not isinstance(gate.get("thresholdMetric"), str):
                raise ReleaseGateError(f"{gate_id}: thresholdMetric required")
        elif gate["operator"] not in {">=", "<=", "=="} or "threshold" not in gate:
            raise ReleaseGateError(f"{gate_id}: invalid threshold operator")
    return gates


def evaluate_release(
    artifacts_dir: Path, thresholds_path: Path = DEFAULT_THRESHOLDS
) -> dict[str, Any]:
    config = load_json(thresholds_path)
    gates = validate_thresholds(config)
    cache: dict[str, dict[str, Any] | None] = {}
    results: list[GateResult] = []
    for gate in gates:
        artifact_name = gate["artifact"]
        if artifact_name not in cache:
            artifact_path = artifacts_dir / artifact_name
            try:
                artifact = load_json(artifact_path)
                metrics = artifact.get("metrics")
                metadata = artifact.get("metadata")
                if not isinstance(metrics, dict):
                    raise ReleaseGateError("missing metrics object")
                if not isinstance(metadata, dict):
                    raise ReleaseGateError("missing metadata envelope")
                try:
                    validate_metadata_envelope(metadata)
                except MetadataValidationError as exc:
                    raise ReleaseGateError(f"invalid metadata envelope: {exc}") from exc
                cache[artifact_name] = artifact
            except ReleaseGateError:
                cache[artifact_name] = None
        artifact = cache[artifact_name]
        expected: Any = gate.get("threshold")
        expected_text = (
            f"<= metric {gate['thresholdMetric']}"
            if gate["operator"] == "<=metric"
            else _expected_text(gate, expected)
        )
        try:
            if artifact is None:
                raise ReleaseGateError(f"missing or invalid artifact {artifact_name}")
            metrics = artifact["metrics"]
            actual = _metric(metrics, gate["metric"])
            op = gate["operator"]
            if op == "<=metric":
                threshold_metric = gate["thresholdMetric"]
                expected = _metric(metrics, threshold_metric)
                op = "<="
                expected_text = f"<= {threshold_metric} ({expected!r})"
            passed = _compare(actual, op, expected)
            reason = (
                "threshold satisfied"
                if passed
                else f"actual {actual!r} does not satisfy {expected_text}"
            )
        except ReleaseGateError as exc:
            actual = None
            passed = False
            reason = str(exc)
        results.append(
            GateResult(
                gate_id=gate["id"],
                passed=passed,
                core=gate["core"] is True,
                actual=actual,
                expected=expected_text,
                reason=reason,
            )
        )

    failed = [result for result in results if not result.passed]
    conditional_enabled = config["policy"].get("conditionalPassEnabled") is True
    if not failed:
        decision = "PASS"
    elif conditional_enabled and all(not result.core for result in failed):
        decision = "CONDITIONAL PASS"
    else:
        decision = "FAIL"
    return {
        "schemaVersion": 1,
        "decision": decision,
        "thresholds": str(thresholds_path),
        "artifacts": str(artifacts_dir),
        "summary": {
            "passed": len(results) - len(failed),
            "failed": len(failed),
            "total": len(results),
        },
        "gates": [result.__dict__ for result in results],
    }


def _git_commit() -> str:
    try:
        return subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def render_report(decision: dict[str, Any], artifacts_dir: Path) -> str:
    metadata: dict[str, Any] = {}
    for path in sorted(artifacts_dir.glob("*.json")) if artifacts_dir.exists() else []:
        try:
            candidate = load_json(path).get("metadata")
        except ReleaseGateError:
            continue
        if isinstance(candidate, dict):
            metadata = candidate
            break
    providers = metadata.get("providers", {}) if isinstance(metadata, dict) else {}
    models = {
        slot: identity.get("model", "missing")
        for slot, identity in providers.items()
        if isinstance(identity, dict)
    }
    dataset = metadata.get("dataset", {}) if isinstance(metadata, dict) else {}
    platform_info = metadata.get("platform", {}) if isinstance(metadata, dict) else {}
    hardware = metadata.get("hardware", {}) if isinstance(metadata, dict) else {}
    lines = [
        "# QA Production Release Report",
        "",
        f"- **Decision:** `{decision['decision']}`",
        f"- **Git commit:** `{metadata.get('gitCommit', _git_commit())}`",
        f"- **Generated:** `{datetime.now(timezone.utc).isoformat()}`",
        f"- **Build:** `git:{metadata.get('gitCommit', 'missing')}`",
        f"- **Dataset version/hash:** `{dataset.get('version', 'missing')}` / `{dataset.get('sha256', 'missing')}`",
        f"- **Runtime config hash:** `{metadata.get('runtimeConfigSha256', 'missing')}`",
        f"- **Platform / CPU / memory:** `{platform_info.get('system', 'missing')} {platform_info.get('release', '')}` / `{hardware.get('cpu', 'missing')}` / `{hardware.get('memoryBytes', 'missing')}`",
        f"- **Providers:** `{json.dumps(providers, ensure_ascii=False, sort_keys=True)}`",
        f"- **Models:** `{json.dumps(models, ensure_ascii=False, sort_keys=True)}`",
        "",
        "## Gate Results",
        "",
        "| Gate | Result | Actual | Requirement | Reason |",
        "|---|---:|---:|---:|---|",
    ]
    for gate in decision["gates"]:
        status = "PASS" if gate["passed"] else "FAIL"
        lines.append(
            f"| `{gate['gate_id']}` | {status} | `{gate['actual']}` | `{gate['expected']}` | {gate['reason']} |"
        )
    failures = [gate for gate in decision["gates"] if not gate["passed"]]
    lines.extend(["", "## Fallbacks and Limitations", ""])
    if failures:
        lines.extend(f"- `{gate['gate_id']}`: {gate['reason']}" for gate in failures)
    else:
        lines.append("- No release-gate limitation was reported by the supplied artifacts.")
    lines.extend(
        [
            "",
            "## Final Decision",
            "",
            f"`{decision['decision']}` — {decision['summary']['passed']}/{decision['summary']['total']} frozen gates passed.",
            "",
        ]
    )
    return "\n".join(lines)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Evaluate frozen QA production gates")
    parser.add_argument("--artifacts", type=Path, default=DEFAULT_ARTIFACTS)
    parser.add_argument("--thresholds", type=Path, default=DEFAULT_THRESHOLDS)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--report", type=Path)
    parser.add_argument("--allow-fail", action="store_true")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        decision = evaluate_release(args.artifacts, args.thresholds)
    except ReleaseGateError as exc:
        print(f"FAIL: {exc}")
        return 1
    rendered = json.dumps(decision, ensure_ascii=False, indent=2)
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(rendered + "\n", encoding="utf-8")
    print(rendered)
    if args.report:
        args.report.write_text(render_report(decision, args.artifacts), encoding="utf-8")
    return 0 if decision["decision"] == "PASS" or args.allow_fail else 1


if __name__ == "__main__":
    raise SystemExit(main())
