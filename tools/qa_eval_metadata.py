#!/usr/bin/env python3
"""Safe, canonical metadata envelopes shared by QA evaluation artifacts."""

from __future__ import annotations

import ctypes
import hashlib
import json
import os
import platform
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path, PureWindowsPath
from typing import Any, Mapping

SCHEMA_VERSION = "qa-eval-metadata-v1"
PROVIDER_SLOTS = ("answer", "embedding", "reranker", "verification")
_HEX_64 = re.compile(r"^[0-9a-f]{64}$")
_GIT_SHA = re.compile(r"^[0-9a-f]{40}$")
_FORBIDDEN_KEYS = {
    "apikey",
    "api_key",
    "authorization",
    "password",
    "secret",
    "token",
    "question",
    "answer",
    "conversation",
    "content",
    "snippet",
    "prompt",
    "response",
    "path",
    "url",
    "endpoint",
}


class MetadataValidationError(ValueError):
    """Raised when an evaluation metadata envelope is unsafe or incomplete."""


def canonical_json_bytes(value: Any) -> bytes:
    """Serialize JSON deterministically and reject NaN/Infinity."""

    return json.dumps(
        value,
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
        allow_nan=False,
    ).encode("utf-8")


def canonical_json_sha256(value: Any) -> str:
    return hashlib.sha256(canonical_json_bytes(value)).hexdigest()


def canonical_json_file_sha256(path: Path) -> str:
    payload = json.loads(path.read_text(encoding="utf-8-sig"))
    return canonical_json_sha256(payload)


def _git_commit(root: Path) -> str:
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip().lower()


def _memory_bytes() -> int:
    if os.name == "nt":
        class MemoryStatus(ctypes.Structure):
            _fields_ = [
                ("length", ctypes.c_ulong),
                ("memory_load", ctypes.c_ulong),
                ("total_physical", ctypes.c_ulonglong),
                ("available_physical", ctypes.c_ulonglong),
                ("total_page_file", ctypes.c_ulonglong),
                ("available_page_file", ctypes.c_ulonglong),
                ("total_virtual", ctypes.c_ulonglong),
                ("available_virtual", ctypes.c_ulonglong),
                ("available_extended_virtual", ctypes.c_ulonglong),
            ]

        status = MemoryStatus()
        status.length = ctypes.sizeof(status)
        if ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(status)):
            return int(status.total_physical)
        return 0
    try:
        return int(os.sysconf("SC_PAGE_SIZE") * os.sysconf("SC_PHYS_PAGES"))
    except (AttributeError, OSError, ValueError):
        return 0


def machine_metadata() -> tuple[dict[str, Any], dict[str, Any]]:
    """Return bounded machine facts without host names, user names, or paths."""

    platform_info = {
        "system": platform.system() or "unknown",
        "release": (platform.release() or "unknown")[:120],
        "machine": (platform.machine() or "unknown")[:80],
        "python": platform.python_version(),
    }
    hardware_info = {
        "cpu": (platform.processor() or platform.machine() or "unknown")[:160],
        "logicalCpuCount": int(os.cpu_count() or 0),
        "memoryBytes": _memory_bytes(),
    }
    return platform_info, hardware_info


def build_metadata_envelope(
    *,
    dataset_version: str,
    dataset_payload: Any,
    runtime_config: Mapping[str, Any],
    providers: Mapping[str, Mapping[str, str]],
    root: Path | None = None,
    git_commit: str | None = None,
    generated_at_utc: str | None = None,
    platform_info: Mapping[str, Any] | None = None,
    hardware_info: Mapping[str, Any] | None = None,
) -> dict[str, Any]:
    """Build a hash-only envelope; raw datasets and runtime config are omitted."""

    detected_platform, detected_hardware = machine_metadata()
    envelope = {
        "schemaVersion": SCHEMA_VERSION,
        "gitCommit": (git_commit or _git_commit(root or Path.cwd())).lower(),
        "generatedAtUtc": generated_at_utc
        or datetime.now(timezone.utc).isoformat(timespec="seconds").replace("+00:00", "Z"),
        "dataset": {
            "version": dataset_version,
            "sha256": canonical_json_sha256(dataset_payload),
        },
        "runtimeConfigSha256": canonical_json_sha256(dict(runtime_config)),
        "providers": {
            slot: {
                "provider": str(providers.get(slot, {}).get("provider", "not_configured")),
                "model": str(providers.get(slot, {}).get("model", "not_configured")),
            }
            for slot in PROVIDER_SLOTS
        },
        "platform": dict(platform_info or detected_platform),
        "hardware": dict(hardware_info or detected_hardware),
    }
    validate_metadata_envelope(envelope)
    return envelope


def _looks_absolute_path(value: str) -> bool:
    return value.startswith(("/", "\\\\")) or PureWindowsPath(value).is_absolute()


def _validate_safe_tree(value: Any, location: str = "metadata") -> None:
    if isinstance(value, Mapping):
        for key, child in value.items():
            normalized = str(key).replace("-", "_").lower()
            provider_slot = location == "metadata.providers" and normalized in PROVIDER_SLOTS
            if normalized in _FORBIDDEN_KEYS and not provider_slot:
                raise MetadataValidationError(f"unsafe metadata key: {location}.{key}")
            _validate_safe_tree(child, f"{location}.{key}")
    elif isinstance(value, list):
        for index, child in enumerate(value):
            _validate_safe_tree(child, f"{location}[{index}]")
    elif isinstance(value, str):
        if _looks_absolute_path(value):
            raise MetadataValidationError(f"absolute path in metadata: {location}")
        if "-----BEGIN " in value or re.search(r"(?i)bearer\s+[a-z0-9._-]{8,}", value):
            raise MetadataValidationError(f"credential-like value in metadata: {location}")


def validate_metadata_envelope(envelope: Mapping[str, Any]) -> None:
    """Fail closed on missing identity fields, raw payloads, secrets, and paths."""

    expected_fields = {
        "schemaVersion",
        "gitCommit",
        "generatedAtUtc",
        "dataset",
        "runtimeConfigSha256",
        "providers",
        "platform",
        "hardware",
    }
    if set(envelope) != expected_fields:
        raise MetadataValidationError("metadata envelope fields are incomplete or unknown")
    if envelope.get("schemaVersion") != SCHEMA_VERSION:
        raise MetadataValidationError("unsupported metadata schemaVersion")
    if not _GIT_SHA.fullmatch(str(envelope.get("gitCommit", ""))):
        raise MetadataValidationError("gitCommit must be a full lowercase SHA-1")
    dataset = envelope.get("dataset")
    if not isinstance(dataset, Mapping) or not str(dataset.get("version", "")).strip():
        raise MetadataValidationError("dataset version is required")
    if not _HEX_64.fullmatch(str(dataset.get("sha256", ""))):
        raise MetadataValidationError("dataset sha256 is invalid")
    if not _HEX_64.fullmatch(str(envelope.get("runtimeConfigSha256", ""))):
        raise MetadataValidationError("runtime config sha256 is invalid")
    providers = envelope.get("providers")
    if not isinstance(providers, Mapping) or set(providers) != set(PROVIDER_SLOTS):
        raise MetadataValidationError("provider matrix is incomplete")
    for slot in PROVIDER_SLOTS:
        identity = providers[slot]
        if not isinstance(identity, Mapping) or set(identity) != {"provider", "model"}:
            raise MetadataValidationError(f"provider identity is invalid: {slot}")
        if not all(str(identity[field]).strip() for field in ("provider", "model")):
            raise MetadataValidationError(f"provider identity is blank: {slot}")
    timestamp = str(envelope.get("generatedAtUtc", ""))
    try:
        parsed = datetime.fromisoformat(timestamp.replace("Z", "+00:00"))
    except ValueError as exc:
        raise MetadataValidationError("generatedAtUtc is invalid") from exc
    if parsed.tzinfo is None or parsed.utcoffset() != timezone.utc.utcoffset(parsed):
        raise MetadataValidationError("generatedAtUtc must be UTC")
    platform_fields = envelope.get("platform")
    hardware_fields = envelope.get("hardware")
    if not isinstance(platform_fields, Mapping) or set(platform_fields) != {
        "system",
        "release",
        "machine",
        "python",
    }:
        raise MetadataValidationError("platform metadata is incomplete or unknown")
    if not isinstance(hardware_fields, Mapping) or set(hardware_fields) != {
        "cpu",
        "logicalCpuCount",
        "memoryBytes",
    }:
        raise MetadataValidationError("platform and hardware metadata are required")
    _validate_safe_tree(envelope)
    canonical_json_bytes(envelope)


__all__ = [
    "MetadataValidationError",
    "SCHEMA_VERSION",
    "build_metadata_envelope",
    "canonical_json_bytes",
    "canonical_json_file_sha256",
    "canonical_json_sha256",
    "machine_metadata",
    "validate_metadata_envelope",
]
