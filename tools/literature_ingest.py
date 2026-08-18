#!/usr/bin/env python3
"""Governed literature intake helpers for the Windows research workbench.

Discovery manifests remain the candidate source of truth.  This module adds a
stable candidate identity, explainable qualification, triage updates, trusted
manual-batch staging, and a fixed end-to-end runner used by the Tauri compile
centre.  It never creates problem/idea pages or edits schema/vocab.yaml.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence

import paper_search


PROJECT_ROOT = Path(__file__).resolve().parents[1]
MAX_PDF_BYTES = 200 * 1024 * 1024
TRIAGE_STATUSES = {"pending", "selected", "rejected", "promoted"}


class LiteratureIngestError(RuntimeError):
    pass


def codex_executable() -> str:
    return os.environ.get("CODEX_CLI_PATH") or shutil.which("codex") or "codex"


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def atomic_write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def stable_candidate_id(item: Mapping[str, Any]) -> str:
    existing = str(item.get("candidate_id") or "").strip()
    if existing:
        return existing
    doi = paper_search.normalize_doi(str(item.get("doi") or ""))
    arxiv_id = paper_search.normalize_arxiv_id(str(item.get("arxiv_id") or ""))
    title = paper_search.normalize_title(str(item.get("title") or ""))
    identity = doi or arxiv_id or title or "paper"
    return hashlib.sha256(identity.casefold().encode("utf-8")).hexdigest()[:20]


def manifest_paths(root: Path) -> list[Path]:
    runs = root / "raw" / "inbox" / "auto-discovered" / "runs"
    return sorted(runs.glob("search-*/results.json"), key=lambda path: path.as_posix())


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        payload = json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise LiteratureIngestError(f"无法读取候选 manifest：{path}（{exc}）") from exc
    if payload.get("kind") != "paper_discovery_candidates" or not isinstance(payload.get("papers"), list):
        raise LiteratureIngestError(f"不是候选 manifest：{path}")
    return payload


def normalize_item(item: dict[str, Any], manifest: Path, payload: Mapping[str, Any]) -> bool:
    changed = False
    defaults: dict[str, Any] = {
        "candidate_id": stable_candidate_id(item),
        "title_matches": [],
        "abstract_matches": [],
        "triage_status": "pending",
        "selected_by_user": False,
        "local_pdf": "",
        "failure_stage": "",
        "failure_reason": "",
        "last_run_id": "",
        "manual_note": "",
    }
    for key, value in defaults.items():
        if key not in item:
            item[key] = value
            changed = True
    status = str(item.get("triage_status") or "pending")
    if status not in TRIAGE_STATUSES:
        item["triage_status"] = "pending"
        changed = True
    run = paper_search.display_path(manifest.parent)
    if item.get("discovery_run") != run:
        item["discovery_run"] = run
        changed = True
    item.setdefault("discovered_via", list(item.get("providers") or payload.get("providers") or []))
    item.setdefault("acquired_at", str(payload.get("retrieved_at") or ""))
    return changed


def update_counts(payload: dict[str, Any]) -> None:
    counts = {status: 0 for status in sorted(TRIAGE_STATUSES)}
    for item in payload.get("papers", []):
        if isinstance(item, dict):
            status = str(item.get("triage_status") or "pending")
            counts[status if status in counts else "pending"] += 1
    payload["triage_counts"] = counts


@dataclass
class ExistingRecord:
    kind: str
    value: str
    existing_id: str
    existing_path: str
    title: str


def frontmatter_scalar(text: str, key: str) -> str:
    normalized = text.lstrip("\ufeff")
    if not normalized.startswith("---"):
        return ""
    block = normalized.split("---", 2)[1]
    prefix = key + ":"
    for raw in block.splitlines():
        line = raw.strip()
        if line.startswith(prefix):
            return line[len(prefix) :].strip().strip("\"'")
    return ""


def existing_records(root: Path) -> list[ExistingRecord]:
    records: list[ExistingRecord] = []
    for source in sorted((root / "wiki" / "sources").glob("*.md")):
        try:
            text = source.read_text(encoding="utf-8-sig")
        except OSError:
            continue
        title = frontmatter_scalar(text, "title")
        doi = paper_search.normalize_doi(frontmatter_scalar(text, "doi"))
        arxiv_id = paper_search.normalize_arxiv_id(frontmatter_scalar(text, "arxiv_id"))
        values = [("doi", doi), ("arxiv", arxiv_id), ("title", paper_search.normalize_title(title))]
        for kind, value in values:
            if value:
                records.append(ExistingRecord(kind, value, source.stem, source.relative_to(root).as_posix(), title))
    canonical = root / "raw" / "canonical"
    for metadata_path in sorted(canonical.glob("**/metadata.json")):
        try:
            data = json.loads(metadata_path.read_text(encoding="utf-8-sig"))
        except (OSError, UnicodeDecodeError, json.JSONDecodeError):
            continue
        title = str(data.get("title") or metadata_path.parent.name)
        for kind, value in [
            ("doi", paper_search.normalize_doi(str(data.get("doi") or ""))),
            ("arxiv", paper_search.normalize_arxiv_id(str(data.get("arxiv_id") or ""))),
            ("title", paper_search.normalize_title(title)),
        ]:
            if value:
                records.append(ExistingRecord(kind, value, metadata_path.parent.name, metadata_path.relative_to(root).as_posix(), title))
    for pdf_path in sorted(canonical.glob("**/*.pdf")):
        try:
            digest = sha256_file(pdf_path)
        except OSError:
            continue
        records.append(
            ExistingRecord(
                "sha256",
                digest,
                pdf_path.parent.name,
                pdf_path.relative_to(root).as_posix(),
                pdf_path.stem,
            )
        )
    return records


def duplicate_matches(item: Mapping[str, Any], records: Sequence[ExistingRecord]) -> list[dict[str, str]]:
    wanted = {
        "doi": paper_search.normalize_doi(str(item.get("doi") or "")),
        "arxiv": paper_search.normalize_arxiv_id(str(item.get("arxiv_id") or "")),
        "title": paper_search.normalize_title(str(item.get("title") or "")),
        "sha256": "",
    }
    local_pdf = str(item.get("local_pdf") or "")
    repository = item.get("_repository")
    if local_pdf and isinstance(repository, Path):
        local_path = (repository / local_pdf).resolve()
        if local_path.is_file():
            wanted["sha256"] = sha256_file(local_path)
    matches: list[dict[str, str]] = []
    seen: set[tuple[str, str]] = set()
    for record in records:
        if wanted.get(record.kind) and wanted[record.kind] == record.value:
            key = (record.kind, record.existing_path)
            if key in seen:
                continue
            seen.add(key)
            matches.append(
                {
                    "kind": record.kind,
                    "value": record.value,
                    "existingId": record.existing_id,
                    "existingPath": record.existing_path,
                    "title": record.title,
                }
            )
    return matches


def qualification(item: Mapping[str, Any], records: Sequence[ExistingRecord], settings: Mapping[str, Any]) -> dict[str, Any]:
    score = float(item.get("score") or 0.0)
    threshold = float(settings.get("minScore", settings.get("min_score", 8.0)))
    title_matches = [str(value) for value in (item.get("title_matches") or []) if str(value)]
    identifier = bool(str(item.get("doi") or "").strip() or str(item.get("arxiv_id") or "").strip())
    pdf_url = str(item.get("pdf_url") or "").strip()
    open_pdf = bool(pdf_url) and item.get("is_open_access") is not False
    topics = [str(value) for value in item.get("matched_queries", []) if str(value)]
    duplicates = duplicate_matches(item, records)
    checks = [
        ("topic", bool(topics), "命中已配置研究主题" if topics else "没有命中已配置研究主题"),
        ("score", score >= threshold, f"相关度 {score:.2f} / 阈值 {threshold:.2f}"),
        ("title_match", bool(title_matches), "标题命中：" + ", ".join(title_matches) if title_matches else "标题未命中领域关键词"),
        ("identifier", identifier, "具有 DOI 或 arXiv ID" if identifier else "缺少 DOI/arXiv ID"),
        ("open_pdf", open_pdf, "具有开放 PDF" if open_pdf else "没有可下载的开放 PDF"),
        ("duplicate", not duplicates, "未发现正式库重复" if not duplicates else "与正式库重复"),
    ]
    return {
        "eligible": all(passed for _, passed, _ in checks),
        "score": score,
        "reasons": [{"code": code, "passed": passed, "message": message} for code, passed, message in checks],
        "duplicates": duplicates,
    }


def collect_candidates(root: Path, settings: Mapping[str, Any] | None = None, *, migrate: bool = False) -> list[dict[str, Any]]:
    settings = settings or {"minScore": 8.0, "maxAutoIngest": 3}
    merged: dict[str, dict[str, Any]] = {}
    for manifest in manifest_paths(root):
        try:
            payload = load_manifest(manifest)
        except LiteratureIngestError:
            continue
        changed = False
        for rank, raw in enumerate(payload.get("papers", []), start=1):
            if not isinstance(raw, dict):
                continue
            changed = normalize_item(raw, manifest, payload) or changed
            candidate_id = str(raw["candidate_id"])
            candidate = dict(raw)
            candidate["rank"] = rank
            candidate["manifestPath"] = manifest.relative_to(root).as_posix()
            candidate["discoveryRuns"] = [candidate.get("discovery_run", "")]
            existing = merged.get(candidate_id)
            if existing:
                runs = list(dict.fromkeys([*existing.get("discoveryRuns", []), *candidate["discoveryRuns"]]))
                # Later manifests carry fresher URLs/metadata; promoted status is never lost.
                previous_status = str(existing.get("triage_status") or "pending")
                candidate["discoveryRuns"] = runs
                if previous_status == "promoted" and candidate.get("triage_status") != "promoted":
                    candidate["triage_status"] = "promoted"
                merged[candidate_id] = candidate
            else:
                merged[candidate_id] = candidate
        if changed and migrate:
            update_counts(payload)
            atomic_write_json(manifest, payload)
    records = existing_records(root)
    results: list[dict[str, Any]] = []
    for item in merged.values():
        qualified_item = {**item, "_repository": root}
        item["qualification"] = qualification(qualified_item, records, settings)
        item["duplicateMatches"] = item["qualification"]["duplicates"]
        # The manifest remains snake_case, while the Tauri/TypeScript boundary is
        # camelCase. Keep both so pipeline internals retain their stable schema.
        for source, target in {
            "candidate_id": "candidateId",
            "arxiv_id": "arxivId",
            "pdf_url": "pdfUrl",
            "source_url": "sourceUrl",
            "triage_status": "triageStatus",
            "local_pdf": "localPdf",
            "manual_note": "manualNote",
            "title_matches": "titleMatches",
            "abstract_matches": "abstractMatches",
            "matched_queries": "matchedQueries",
        }.items():
            item[target] = item.get(source, "" if source not in {"title_matches", "abstract_matches", "matched_queries"} else [])
        results.append(item)
    return sorted(results, key=lambda item: (float(item.get("score") or 0), int(item.get("year") or 0), str(item.get("title") or "").casefold()), reverse=True)


def update_triage(root: Path, candidate_ids: set[str], status: str, note: str = "") -> int:
    if status not in TRIAGE_STATUSES:
        raise LiteratureIngestError(f"无效筛选状态：{status}")
    updated = 0
    timestamp = utc_now()
    for manifest in manifest_paths(root):
        payload = load_manifest(manifest)
        changed = False
        for item in payload.get("papers", []):
            if not isinstance(item, dict):
                continue
            normalize_item(item, manifest, payload)
            if str(item.get("candidate_id")) not in candidate_ids:
                continue
            item["triage_status"] = status
            item["selected_by_user"] = status in {"selected", "promoted"}
            if status == "selected":
                item["selected_at"] = timestamp
            if status == "promoted":
                item["canonicalized_at"] = timestamp
            if note:
                item["manual_note"] = note
            changed = True
            updated += 1
        if changed:
            update_counts(payload)
            atomic_write_json(manifest, payload)
    return updated


def candidate_folder(root: Path, item: Mapping[str, Any]) -> Path:
    title = str(item.get("title") or "paper")
    year = str(item.get("year") or "unknown")
    return root / "raw" / "inbox" / "auto-discovered" / "papers" / f"{year}-{paper_search.safe_component(title, 70)}-{stable_candidate_id(item)[:8]}"


def download_candidate(root: Path, item: dict[str, Any]) -> Path:
    url = str(item.get("pdf_url") or "").strip()
    if not url:
        raise LiteratureIngestError("候选没有 PDF URL")
    folder = candidate_folder(root, item)
    folder.mkdir(parents=True, exist_ok=True)
    destination = folder / "paper.pdf"
    if destination.is_file():
        size = destination.stat().st_size
        with destination.open("rb") as stream:
            header = stream.read(5)
        if size <= 0 or size > MAX_PDF_BYTES or header != b"%PDF-":
            destination.unlink(missing_ok=True)
    if not destination.is_file():
        paper_search.download_pdf(paper_search.build_session(), url, destination)
    metadata = dict(item)
    metadata["local_pdf"] = destination.relative_to(root).as_posix()
    metadata["boundary"] = "selected inbox candidate; not canonical; not wiki evidence"
    metadata["downloaded_at"] = utc_now()
    atomic_write_json(folder / "metadata.json", metadata)
    return destination


def run_command(stage: str, command: Sequence[str], cwd: Path, fixture_dir: Path | None = None) -> None:
    print(f"PIPELINE_STAGE_START {stage}", flush=True)
    if fixture_dir:
        fixture_dir.mkdir(parents=True, exist_ok=True)
        with (fixture_dir / "stages.jsonl").open("a", encoding="utf-8") as stream:
            stream.write(json.dumps({"stage": stage, "command": list(command)}, ensure_ascii=False) + "\n")
        config_path = fixture_dir / "config.json"
        config = json.loads(config_path.read_text(encoding="utf-8")) if config_path.exists() else {}
        if config.get("fail_stage") == stage:
            print(f"PIPELINE_STAGE_FAILED {stage} 17", flush=True)
            raise LiteratureIngestError(f"fixture failure: {stage}")
        print(f"PIPELINE_STAGE_COMPLETED {stage}", flush=True)
        return
    result = subprocess.run(list(command), cwd=cwd, check=False)
    if result.returncode:
        print(f"PIPELINE_STAGE_FAILED {stage} {result.returncode}", flush=True)
        raise LiteratureIngestError(f"{stage} 失败，退出码 {result.returncode}")
    print(f"PIPELINE_STAGE_COMPLETED {stage}", flush=True)


def staged_manual_files(root: Path, manifest: Mapping[str, Any]) -> list[Path]:
    batch_id = str(manifest.get("batchId") or datetime.now().strftime("%Y%m%d-%H%M%S"))
    destination_root = root / "raw" / "inbox" / "manual-drop" / f"desktop-{paper_search.safe_component(batch_id, 48)}"
    staged: list[Path] = []
    for index, item in enumerate(manifest.get("files", []), start=1):
        if not isinstance(item, dict) or item.get("selected") is False:
            continue
        source = Path(str(item.get("path") or "")).expanduser().resolve()
        if not source.is_file() or source.suffix.casefold() != ".pdf":
            raise LiteratureIngestError(f"手动文件无效：{source}")
        stat = source.stat()
        if stat.st_size <= 0 or stat.st_size > MAX_PDF_BYTES:
            raise LiteratureIngestError(f"手动文件大小无效：{source.name}")
        if int(item.get("size") or -1) != stat.st_size or int(item.get("mtimeNs") or -1) != stat.st_mtime_ns:
            raise LiteratureIngestError(f"选择后文件已变化：{source.name}")
        expected_hash = str(item.get("sha256") or "")
        if expected_hash and sha256_file(source) != expected_hash:
            raise LiteratureIngestError(f"选择后文件哈希已变化：{source.name}")
        destination_root.mkdir(parents=True, exist_ok=True)
        destination = destination_root / source.name
        if destination.exists() and sha256_file(destination) != sha256_file(source):
            destination = destination_root / f"{source.stem}-{index}{source.suffix}"
        shutil.copy2(source, destination)
        staged.append(destination)
    if not staged:
        raise LiteratureIngestError("手动批次没有可入库 PDF")
    return staged


def scoped_compile_prompt(canonical_roots: Sequence[Path], root: Path) -> str:
    scopes = ", ".join(path.relative_to(root).as_posix() for path in canonical_roots)
    return (
        "Read AGENTS.md and schema/agent-a-compile.md. Compile only pending_ingest material under: "
        f"{scopes}. Never write wiki/problems or wiki/ideas, never edit vocab.yaml, never delete files. "
        "Update wiki/index.md, wiki/maps/library-status.md and logs. Do not compile unrelated pending folders."
    )


def parse_and_compile(root: Path, pdfs: Sequence[Path], acquisition: str, fixture_dir: Path | None) -> None:
    before = {path.resolve() for path in (root / "raw" / "canonical").glob("*/full.md")}
    for pdf in pdfs:
        run_command(
            "parse",
            [sys.executable, "tools/mineru_to_md.py", str(pdf), "--output-root", str(root / "raw" / "canonical"), "--acquisition-method", acquisition],
            root,
            fixture_dir,
        )
    after = {path.resolve() for path in (root / "raw" / "canonical").glob("*/full.md")}
    scopes = sorted({path.parent for path in after - before}, key=lambda path: path.as_posix())
    if fixture_dir and not scopes:
        scopes = [root / "raw" / "canonical" / "fixture-paper"]
    if not scopes:
        # MinerU may have reused a pre-existing canonical folder; locate by task source.
        for status in (root / "raw" / "canonical").glob("*/.mineru-task.json"):
            try:
                data = json.loads(status.read_text(encoding="utf-8-sig"))
            except (OSError, json.JSONDecodeError, UnicodeDecodeError):
                continue
            if any(str(data.get("source_pdf") or "").endswith(pdf.name) for pdf in pdfs):
                scopes.append(status.parent)
    if not scopes:
        raise LiteratureIngestError("解析完成后没有找到本批 canonical Markdown")
    codex = codex_executable()
    run_command(
        "compile_a",
        [codex, "-a", "never", "-s", "workspace-write", "exec", "-C", str(root), "--skip-git-repo-check", "--ephemeral", scoped_compile_prompt(scopes, root)],
        root,
        fixture_dir,
    )


def finalize_repository(root: Path, fixture_dir: Path | None) -> None:
    run_command("lint", [sys.executable, "tools/wiki_lint.py", "--write-report"], root, fixture_dir)
    run_command("graphify_update", [sys.executable, "tools/graphify_refresh.py"], root, fixture_dir)
    run_command("rebuild_snapshot", [sys.executable, "tools/export_desktop_data.py"], root, fixture_dir)


def load_run_manifest(path: Path) -> dict[str, Any]:
    try:
        payload = json.loads(path.read_text(encoding="utf-8-sig"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise LiteratureIngestError(f"运行清单无效：{exc}") from exc
    if payload.get("kind") != "literature_ingest_run":
        raise LiteratureIngestError("运行清单类型无效")
    return payload


def run_ingest(root: Path, run_manifest: Path, fixture_dir: Path | None = None) -> dict[str, Any]:
    manifest = load_run_manifest(run_manifest)
    mode = str(manifest.get("mode") or "")
    settings = manifest.get("settings") if isinstance(manifest.get("settings"), dict) else {}
    completed: list[str] = []
    failed: list[dict[str, str]] = []

    if mode in {"prepare", "automatic"}:
        command = [sys.executable, "tools/paper_search.py", "--preset", "wireless-charging-scheduling", "--new-only", "--download"]
        since_year = settings.get("sinceYear")
        if since_year:
            command.extend(["--since-year", str(int(since_year))])
        for provider in settings.get("providers", []):
            command.extend(["--provider", str(provider)])
        run_command("discover", command, root, fixture_dir)
        if mode == "prepare" or not bool(settings.get("autoPromoteEnabled")):
            return {"mode": "prepare", "completed": [], "failed": []}

    candidates = {str(item.get("candidate_id")): item for item in collect_candidates(root, settings, migrate=True)}
    candidate_ids = [str(value) for value in manifest.get("candidateIds", [])]
    if mode == "automatic":
        maximum = max(1, min(20, int(settings.get("maxAutoIngest", 3))))
        candidate_ids = [str(item["candidate_id"]) for item in candidates.values() if item.get("qualification", {}).get("eligible")][:maximum]

    if mode == "manual":
        pdfs = staged_manual_files(root, manifest)
        parse_and_compile(root, pdfs, "manual_upload", fixture_dir)
        finalize_repository(root, fixture_dir)
        return {"mode": mode, "completed": [pdf.name for pdf in pdfs], "failed": []}

    if mode == "download":
        for candidate_id in candidate_ids:
            item = candidates.get(candidate_id)
            if not item:
                failed.append({"id": candidate_id, "stage": "resolve", "reason": "候选不存在"})
                continue
            try:
                print(f"LITERATURE_ITEM_START {candidate_id}", flush=True)
                pdf = download_candidate(root, item)
                update_candidate_local_pdf(root, candidate_id, pdf)
                completed.append(candidate_id)
                print(f"LITERATURE_ITEM_COMPLETED {candidate_id}", flush=True)
            except Exception as exc:
                failed.append({"id": candidate_id, "stage": "download", "reason": str(exc)})
                mark_failure(root, candidate_id, "download", str(exc))
                print(f"LITERATURE_ITEM_FAILED {candidate_id} download", flush=True)
        return {"mode": mode, "completed": completed, "failed": failed}

    for candidate_id in candidate_ids:
        item = candidates.get(candidate_id)
        if not item:
            failed.append({"id": candidate_id, "stage": "resolve", "reason": "候选不存在"})
            continue
        try:
            print(f"LITERATURE_ITEM_START {candidate_id}", flush=True)
            update_triage(root, {candidate_id}, "selected")
            local = str(item.get("local_pdf") or "")
            pdf = (root / local).resolve() if local and (root / local).is_file() else download_candidate(root, item)
            update_candidate_local_pdf(root, candidate_id, pdf)
            if mode == "automatic":
                after_download = {**item, "local_pdf": pdf.relative_to(root).as_posix(), "_repository": root}
                post_download_qualification = qualification(after_download, existing_records(root), settings)
                if not post_download_qualification["eligible"]:
                    update_triage(root, {candidate_id}, "pending", "自动下载后资格检查未通过")
                    failed.append({"id": candidate_id, "stage": "qualify", "reason": "自动下载后资格检查未通过"})
                    print(f"LITERATURE_ITEM_FAILED {candidate_id} qualify", flush=True)
                    continue
            parse_and_compile(root, [pdf], "auto_discovery", fixture_dir)
            update_triage(root, {candidate_id}, "promoted")
            completed.append(candidate_id)
            print(f"LITERATURE_ITEM_COMPLETED {candidate_id}", flush=True)
        except Exception as exc:
            failed.append({"id": candidate_id, "stage": "ingest", "reason": str(exc)})
            mark_failure(root, candidate_id, "ingest", str(exc))
            print(f"LITERATURE_ITEM_FAILED {candidate_id} ingest", flush=True)
    if completed:
        finalize_repository(root, fixture_dir)
    return {"mode": mode, "completed": completed, "failed": failed}


def mutate_candidate(root: Path, candidate_id: str, changes: Mapping[str, Any]) -> None:
    for manifest in manifest_paths(root):
        payload = load_manifest(manifest)
        changed = False
        for item in payload.get("papers", []):
            if isinstance(item, dict) and stable_candidate_id(item) == candidate_id:
                item.update(changes)
                item["candidate_id"] = candidate_id
                changed = True
        if changed:
            update_counts(payload)
            atomic_write_json(manifest, payload)


def update_candidate_local_pdf(root: Path, candidate_id: str, pdf: Path) -> None:
    mutate_candidate(root, candidate_id, {"local_pdf": pdf.relative_to(root).as_posix(), "downloaded_at": utc_now(), "failure_stage": "", "failure_reason": ""})


def mark_failure(root: Path, candidate_id: str, stage: str, reason: str) -> None:
    mutate_candidate(root, candidate_id, {"failure_stage": stage, "failure_reason": reason[:2000], "failed_at": utc_now()})


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="客户端文献入库领域工具")
    parser.add_argument("--repository", type=Path, default=PROJECT_ROOT)
    sub = parser.add_subparsers(dest="command", required=True)
    listing = sub.add_parser("list-candidates")
    listing.add_argument("--settings", type=Path)
    listing.add_argument("--min-score", type=float, default=8.0)
    listing.add_argument("--max-auto-ingest", type=int, default=3)
    listing.add_argument("--migrate", action="store_true")
    listing.add_argument("--json", action="store_true")
    triage = sub.add_parser("triage")
    triage.add_argument("--ids", required=True)
    triage.add_argument("--status", choices=sorted(TRIAGE_STATUSES), required=True)
    triage.add_argument("--note", default="")
    run = sub.add_parser("run")
    run.add_argument("--manifest", type=Path, required=True)
    run.add_argument("--fixture-dir", type=Path)
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    configure_stdio_utf8()
    args = build_parser().parse_args(argv)
    root = args.repository.expanduser().resolve()
    if not (root / "AGENTS.md").is_file() or not (root / "wiki").is_dir():
        raise LiteratureIngestError(f"不是有效知识库：{root}")
    if args.command == "list-candidates":
        settings: dict[str, Any] = {"minScore": args.min_score, "maxAutoIngest": args.max_auto_ingest}
        if args.settings:
            settings.update(json.loads(args.settings.read_text(encoding="utf-8-sig")))
        candidates = collect_candidates(root, settings, migrate=args.migrate)
        payload = {"candidates": candidates, "count": len(candidates)}
        print(json.dumps(payload, ensure_ascii=False, indent=2) if args.json else f"候选：{len(candidates)}")
        return 0
    if args.command == "triage":
        ids = {value.strip() for value in args.ids.split(",") if value.strip()}
        print(json.dumps({"updated": update_triage(root, ids, args.status, args.note)}, ensure_ascii=False))
        return 0
    if args.command == "run":
        result = run_ingest(root, args.manifest.resolve(), args.fixture_dir.resolve() if args.fixture_dir else None)
        print("LITERATURE_RESULT " + json.dumps(result, ensure_ascii=False), flush=True)
        if result["failed"] and result["completed"]:
            return 3
        return 0 if not result["failed"] else 2
    return 1


def configure_stdio_utf8() -> None:
    for stream in (sys.stdout, sys.stderr):
        reconfigure = getattr(stream, "reconfigure", None)
        if callable(reconfigure):
            reconfigure(encoding="utf-8", errors="backslashreplace")


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except LiteratureIngestError as exc:
        print(f"文献入库失败：{exc}", file=sys.stderr)
        raise SystemExit(2)
