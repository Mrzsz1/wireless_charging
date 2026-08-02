#!/usr/bin/env python3
"""Apply human triage decisions to paper-discovery manifests.

The command updates candidate state in-place and materializes selected metadata
under raw/inbox/auto-discovered/papers. It never promotes a paper to canonical.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from dataclasses import fields
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Mapping, Sequence

import paper_search


PROJECT_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_PAPERS_ROOT = PROJECT_ROOT / "raw" / "inbox" / "auto-discovered" / "papers"
TRIAGE_STATUSES = ("pending", "selected", "rejected", "promoted")


class TriageError(RuntimeError):
    pass


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def parse_indices(value: str, total: int) -> set[int]:
    indices: set[int] = set()
    if not value:
        return indices
    for raw_part in value.split(","):
        part = raw_part.strip()
        if not part:
            continue
        if "-" in part:
            start_text, end_text = part.split("-", 1)
            try:
                start, end = int(start_text), int(end_text)
            except ValueError as exc:
                raise TriageError(f"无效序号范围：{part}") from exc
            if start > end:
                raise TriageError(f"序号范围起点大于终点：{part}")
            indices.update(range(start, end + 1))
        else:
            try:
                indices.add(int(part))
            except ValueError as exc:
                raise TriageError(f"无效序号：{part}") from exc
    invalid = sorted(index for index in indices if index < 1 or index > total)
    if invalid:
        raise TriageError(f"序号超出 1..{total}：{invalid}")
    return indices


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        payload = json.loads(path.read_text(encoding="utf-8-sig"))
    except FileNotFoundError as exc:
        raise TriageError(f"找不到 manifest：{path}") from exc
    except (OSError, json.JSONDecodeError, UnicodeDecodeError) as exc:
        raise TriageError(f"无法读取 manifest：{exc}") from exc
    if payload.get("kind") != "paper_discovery_candidates" or not isinstance(payload.get("papers"), list):
        raise TriageError("不是 paper_search.py 生成的候选 manifest。")
    return payload


def normalize_manifest(payload: dict[str, Any], manifest: Path) -> None:
    run_path = paper_search.display_path(manifest.parent)
    acquired_at = str(payload.get("retrieved_at") or utc_now())
    providers = [str(item) for item in payload.get("providers", [])]
    payload["acquisition_method"] = "auto_discovery"
    for item in payload["papers"]:
        if not isinstance(item, dict):
            continue
        item["acquisition_method"] = "auto_discovery"
        item.setdefault("discovered_via", list(item.get("providers") or providers))
        item["discovery_run"] = run_path
        item.setdefault("triage_status", "pending")
        item.setdefault("selected_by_user", item["triage_status"] in {"selected", "promoted"})
        item.setdefault("acquired_at", acquired_at)
        item.setdefault("selected_at", "")
        item.setdefault("canonicalized_at", "")
    update_counts(payload)


def update_counts(payload: dict[str, Any]) -> None:
    counts = {status: 0 for status in TRIAGE_STATUSES}
    for item in payload.get("papers", []):
        status = str(item.get("triage_status", "pending"))
        if status not in counts:
            status = "pending"
            item["triage_status"] = status
        counts[status] += 1
    payload["triage_counts"] = counts


def candidate_folder(item: Mapping[str, Any], papers_root: Path) -> Path:
    title = str(item.get("title") or "paper")
    identity = str(item.get("doi") or item.get("arxiv_id") or paper_search.normalize_title(title))
    digest = hashlib.sha256(identity.encode("utf-8")).hexdigest()[:8]
    year = str(item.get("year") or "unknown")
    return papers_root / f"{year}-{paper_search.safe_component(title, 70)}-{digest}"


def existing_candidate_folder(item: Mapping[str, Any], papers_root: Path) -> Path | None:
    """Reuse a materialized folder even when later metadata adds DOI/arXiv IDs."""

    wanted = paper_search.normalize_title(str(item.get("title") or ""))
    if not wanted or not papers_root.exists():
        return None
    for metadata_path in papers_root.glob("*/metadata.json"):
        try:
            metadata = json.loads(metadata_path.read_text(encoding="utf-8-sig"))
        except (OSError, json.JSONDecodeError, UnicodeDecodeError):
            continue
        if paper_search.normalize_title(str(metadata.get("title") or "")) == wanted:
            return metadata_path.parent
    return None


def materialize_selected(
    payload: dict[str, Any], manifest: Path, papers_root: Path, download: bool
) -> tuple[int, list[str]]:
    selected = 0
    errors: list[str] = []
    session = paper_search.build_session() if download else None
    for rank, item in enumerate(payload.get("papers", []), start=1):
        status = str(item.get("triage_status") or "pending")
        if status not in {"selected", "promoted"}:
            continue
        folder = existing_candidate_folder(item, papers_root) or candidate_folder(item, papers_root)
        folder.mkdir(parents=True, exist_ok=True)
        metadata = dict(item)
        metadata.update(
            {
                "candidate_rank": rank,
                "source_manifest": paper_search.display_path(manifest),
                "boundary": (
                    "promoted to canonical; pending wiki ingest"
                    if status == "promoted"
                    else "selected inbox candidate; not canonical; not wiki evidence"
                ),
            }
        )
        pdf_url = str(item.get("pdf_url") or "")
        if download and pdf_url:
            destination = folder / "paper.pdf"
            try:
                paper_search.download_pdf(session, pdf_url, destination)
                relative = paper_search.display_path(destination)
                item["local_pdf"] = relative
                metadata["local_pdf"] = relative
            except Exception as exc:  # keep other selections usable
                errors.append(f"{item.get('title', 'Untitled')}: {exc}")
        paper_search.atomic_write_json(folder / "metadata.json", metadata)
        selected += 1
    return selected, errors


def rebuild_report(payload: dict[str, Any], manifest: Path) -> None:
    paper_fields = {field.name for field in fields(paper_search.Paper)}
    papers = [
        paper_search.Paper(**{key: value for key, value in item.items() if key in paper_fields})
        for item in payload.get("papers", [])
        if isinstance(item, dict)
    ]
    queries = [paper_search.SearchQuery(**item) for item in payload.get("queries", [])]
    outcome = paper_search.SearchOutcome(
        papers=papers,
        errors=[str(item) for item in payload.get("errors", [])],
        provider_counts=dict(payload.get("provider_counts", {})),
        cache_hits=int(payload.get("cache_hits", 0)),
    )
    report = paper_search.render_report(
        papers,
        queries,
        [str(item) for item in payload.get("providers", [])],
        outcome,
        str(payload.get("retrieved_at") or ""),
    )
    paper_search.atomic_write_text(manifest.parent / "README.md", report)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="人工筛选论文发现候选；只更新 inbox，不自动晋升 canonical。",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("manifest", type=Path, help="search-*/results.json")
    parser.add_argument("--select", default="", help="选择序号，如 1,3-5")
    parser.add_argument("--reject", default="", help="拒绝序号，如 2,7")
    parser.add_argument("--pending", default="", help="恢复待筛选序号")
    parser.add_argument("--promote", default="", help="标记已进入 canonical 的序号")
    parser.add_argument("--note", default="", help="写入本次选择/拒绝项的人工备注")
    parser.add_argument("--papers-root", type=Path, default=DEFAULT_PAPERS_ROOT)
    parser.add_argument("--download-selected", action="store_true", help="下载已选且有开放 PDF URL 的候选")
    parser.add_argument("--migrate", action="store_true", help="只补齐旧 manifest 的来源与状态字段")
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        manifest = args.manifest.expanduser().resolve()
        payload = load_manifest(manifest)
        normalize_manifest(payload, manifest)
        total = len(payload["papers"])
        actions = {
            "selected": parse_indices(args.select, total),
            "rejected": parse_indices(args.reject, total),
            "pending": parse_indices(args.pending, total),
            "promoted": parse_indices(args.promote, total),
        }
        claimed: dict[int, str] = {}
        for status, indices in actions.items():
            for index in indices:
                if index in claimed:
                    raise TriageError(f"序号 {index} 同时出现在 {claimed[index]} 与 {status}。")
                claimed[index] = status
        if not claimed and not args.migrate:
            raise TriageError("请至少提供 --select、--reject、--pending 或 --migrate。")
        changed_at = utc_now()
        for index, status in claimed.items():
            item = payload["papers"][index - 1]
            item["triage_status"] = status
            item["selected_by_user"] = status in {"selected", "promoted"}
            if status == "selected":
                item["selected_at"] = changed_at
                item["canonicalized_at"] = ""
            elif status == "promoted":
                item["selected_at"] = str(item.get("selected_at") or changed_at)
                item["canonicalized_at"] = changed_at
            else:
                item["selected_at"] = ""
                item["canonicalized_at"] = ""
            if args.note:
                item["triage_note"] = args.note
        update_counts(payload)
        selected, errors = materialize_selected(
            payload, manifest, args.papers_root.expanduser().resolve(), args.download_selected
        )
        payload["triage_updated_at"] = changed_at
        if errors:
            payload.setdefault("errors", []).extend(f"triage PDF / {error}" for error in errors)
        paper_search.atomic_write_json(manifest, payload)
        rebuild_report(payload, manifest)
        counts = payload["triage_counts"]
        print(
            "筛选完成："
            + "，".join(f"{status}={counts[status]}" for status in TRIAGE_STATUSES)
            + f"；已选队列写入 {selected} 项。"
        )
        if errors:
            print(f"PDF 下载失败 {len(errors)} 项，详见 manifest errors。", file=sys.stderr)
        if counts["promoted"]:
            print("边界：promoted 表示已进入 canonical；仍未完成 A 编译，不自动成为 wiki 证据。")
        else:
            print("边界：仍在 raw/inbox；未晋升 canonical、未调用 MinerU、未编译 wiki。")
        return 0
    except (TriageError, OSError, json.JSONDecodeError) as exc:
        print(f"错误：{exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
