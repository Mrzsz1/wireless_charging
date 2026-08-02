#!/usr/bin/env python3
"""Batch-convert local PDFs to MinerU Markdown results.

The script uses MinerU's precise parsing API:
1. POST /api/v4/file-urls/batch to obtain signed upload URLs.
2. PUT each local PDF to its signed URL.
3. Poll /api/v4/extract-results/batch/{batch_id}.
4. Download and safely extract the result ZIP into one-paper-one-folder layout.

The API token is read from MINERU_API_KEY or --api-key-file. It is never
written to disk, printed, or sent to the signed OSS upload URL.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import stat
import sys
import tempfile
import time
import zipfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path, PurePosixPath
from typing import Any, Iterable, Mapping, Sequence

try:
    import requests
    from requests import Response, Session
    from requests.adapters import HTTPAdapter
    from urllib3.util.retry import Retry
except ImportError as exc:  # pragma: no cover - depends on the user's runtime
    raise SystemExit(
        "缺少 requests。请运行：py -3 -m pip install requests"
    ) from exc


PROJECT_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_INPUT = PROJECT_ROOT / "raw" / "canonical"
DEFAULT_OUTPUT_ROOT = DEFAULT_INPUT
DEFAULT_KEY_FILE = Path(os.environ.get("MINERU_API_KEY_FILE", r"E:\知识库\aoikey.txt"))

API_BASE = "https://mineru.net/api/v4"
MAX_BATCH_SIZE = 50
MAX_FILE_BYTES = 200 * 1024 * 1024
MAX_EXTRACTED_BYTES = 2 * 1024 * 1024 * 1024
DONE_STATES = {"done", "failed"}
AUXILIARY_PDF_SUFFIXES = ("_origin.pdf", "_layout.pdf", "_span.pdf")


class MinerUError(RuntimeError):
    """Base error for conversion failures."""


class MinerUApiError(MinerUError):
    """MinerU returned an API-level error."""

    def __init__(self, code: Any, message: str, trace_id: str = "") -> None:
        suffix = f" (trace_id={trace_id})" if trace_id else ""
        super().__init__(f"MinerU API 错误 {code}: {message}{suffix}")
        self.code = code
        self.trace_id = trace_id


@dataclass
class Job:
    source_pdf: Path
    output_dir: Path
    canonical_pdf: Path
    data_id: str
    api_name: str
    state: str = "planned"
    error: str = ""

    @property
    def status_path(self) -> Path:
        return self.output_dir / ".mineru-task.json"


@dataclass
class Plan:
    jobs: list[Job]
    skipped: list[tuple[Path, str]]


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def is_relative_to(path: Path, root: Path) -> bool:
    try:
        path.relative_to(root)
        return True
    except ValueError:
        return False


def display_path(path: Path) -> str:
    resolved = path.resolve()
    root = PROJECT_ROOT.resolve()
    if is_relative_to(resolved, root):
        return resolved.relative_to(root).as_posix()
    return str(resolved)


def yaml_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def safe_component(value: str, max_length: int = 96) -> str:
    value = re.sub(r'[<>:"/\\|?*\x00-\x1f]', "_", value.strip())
    value = re.sub(r"\s+", "_", value)
    value = re.sub(r"_+", "_", value).strip(" ._")
    if not value:
        value = "document"
    return value[:max_length].rstrip(" ._") or "document"


def load_sidecar_metadata(source_pdf: Path | None) -> dict[str, Any]:
    """Read a selected-candidate metadata sidecar without guessing fields."""

    if not source_pdf or not source_pdf.name:
        return {}
    sidecar = source_pdf.parent / "metadata.json"
    if not sidecar.is_file():
        return {}
    try:
        loaded = json.loads(sidecar.read_text(encoding="utf-8-sig"))
    except (OSError, json.JSONDecodeError, UnicodeDecodeError):
        return {}
    return loaded if isinstance(loaded, dict) else {}


def document_title(source_pdf: Path) -> str:
    metadata = load_sidecar_metadata(source_pdf)
    title = str(metadata.get("title") or "").strip()
    return title or source_pdf.stem


def canonical_pdf_name(source_pdf: Path) -> str:
    metadata_title = str(load_sidecar_metadata(source_pdf).get("title") or "").strip()
    return f"{safe_component(metadata_title, 84)}.pdf" if metadata_title else source_pdf.name


def candidate_matches_source(candidate: Path, source_pdf: Path) -> bool:
    """Recognize a canonical folder already allocated to the same source PDF."""

    status_path = candidate / ".mineru-task.json"
    if status_path.is_file():
        try:
            status = json.loads(status_path.read_text(encoding="utf-8-sig"))
        except (OSError, json.JSONDecodeError, UnicodeDecodeError):
            status = {}
        if isinstance(status, dict) and status.get("source_pdf") == display_path(source_pdf):
            return True

    canonical_pdf = candidate / canonical_pdf_name(source_pdf)
    try:
        return canonical_pdf.is_file() and canonical_pdf.stat().st_size == source_pdf.stat().st_size
    except OSError:
        return False


def data_id_for(path: Path) -> str:
    resolved = path.resolve()
    stat_info = resolved.stat()
    identity = f"{str(resolved).casefold()}|{stat_info.st_size}|{stat_info.st_mtime_ns}"
    digest = hashlib.sha256(identity.encode("utf-8")).hexdigest()[:24]
    return f"pdf-{digest}"


def api_name_for(path: Path, data_id: str) -> str:
    stem = safe_component(path.stem, max_length=120)
    return f"{data_id}-{stem}.pdf"


def read_api_key(path: Path, environ: Mapping[str, str] | None = None) -> str:
    environment = os.environ if environ is None else environ
    value = environment.get("MINERU_API_KEY", "").strip()
    if not value:
        try:
            content = path.expanduser().read_text(encoding="utf-8-sig")
        except FileNotFoundError as exc:
            raise MinerUError(
                f"未找到 API key 文件：{path}。也可以设置 MINERU_API_KEY。"
            ) from exc
        except OSError as exc:
            raise MinerUError(f"无法读取 API key 文件：{path}（{exc}）") from exc
        labeled: dict[str, str] = {}
        unlabeled: list[str] = []
        for raw_line in content.splitlines():
            line = raw_line.strip()
            if not line or line.startswith(("#", ";")):
                continue
            match = re.match(r"^([A-Za-z][A-Za-z0-9_-]{1,50})\s*[:=]\s*(.+?)\s*$", line)
            if match:
                label = re.sub(r"[^A-Za-z0-9]", "", match.group(1)).upper()
                labeled[label] = match.group(2).strip()
            else:
                unlabeled.append(line)
        value = labeled.get("MINERUAPIKEY") or labeled.get("MINERUTOKEN") or ""
        # Backward compatibility: the original shared file stored MinerU as its
        # first token-only line. Other providers now use explicit labels.
        if not value and unlabeled:
            value = unlabeled[0]

    if value.lower().startswith("bearer "):
        value = value[7:].strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in {'"', "'"}:
        value = value[1:-1].strip()
    if not value:
        raise MinerUError("API key 为空。")
    if "\n" in value or "\r" in value:
        raise MinerUError("API key 文件应只包含一个 token。")
    return value


def atomic_write_json(path: Path, payload: Mapping[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp_path = path.with_name(f"{path.name}.tmp")
    temp_path.write_text(
        json.dumps(payload, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    temp_path.replace(path)


def write_job_status(job: Job, batch_id: str = "", **extra: Any) -> None:
    payload: dict[str, Any] = {
        "api": "MinerU precise parsing API v4",
        "batch_id": batch_id,
        "data_id": job.data_id,
        "api_name": job.api_name,
        "source_pdf": display_path(job.source_pdf),
        "canonical_pdf": display_path(job.canonical_pdf),
        "output_dir": display_path(job.output_dir),
        "state": job.state,
        "error": job.error,
        "updated_at": utc_now(),
    }
    payload.update(extra)
    atomic_write_json(job.status_path, payload)


def choose_pdf_per_canonical_folder(
    pdfs: Sequence[Path], output_root: Path, force: bool
) -> tuple[list[Path], list[tuple[Path, str]]]:
    """Avoid parsing MinerU's duplicate *_origin.pdf inside canonical folders."""

    output_root = output_root.resolve()
    selected: list[Path] = []
    skipped: list[tuple[Path, str]] = []
    grouped: dict[Path, list[Path]] = {}
    external: list[Path] = []

    for pdf in pdfs:
        resolved = pdf.resolve()
        parent = resolved.parent
        if is_relative_to(parent, output_root) and parent != output_root:
            grouped.setdefault(parent, []).append(resolved)
        else:
            external.append(resolved)

    selected.extend(external)
    for folder, folder_pdfs in sorted(grouped.items(), key=lambda item: str(item[0])):
        full_md = folder / "full.md"
        if full_md.exists() and not force:
            for pdf in folder_pdfs:
                skipped.append((pdf, "目录已有 full.md"))
            continue

        primary = [
            pdf
            for pdf in folder_pdfs
            if not pdf.name.lower().endswith(AUXILIARY_PDF_SUFFIXES)
        ]
        candidates = primary or folder_pdfs
        if len(candidates) > 1:
            names = ", ".join(pdf.name for pdf in candidates)
            raise MinerUError(
                f"一文一夹目录中发现多个候选 PDF：{folder}（{names}）。"
                "请整理目录，或直接把某个 PDF 路径作为命令参数。"
            )
        chosen = candidates[0]
        selected.append(chosen)
        for pdf in folder_pdfs:
            if pdf != chosen:
                skipped.append((pdf, "同目录辅助/重复 PDF"))

    return sorted(set(selected), key=lambda path: str(path).casefold()), skipped


def discover_pdfs(input_path: Path, output_root: Path, force: bool) -> tuple[list[Path], list[tuple[Path, str]]]:
    input_path = input_path.expanduser().resolve()
    if not input_path.exists():
        raise MinerUError(f"输入路径不存在：{input_path}")
    if input_path.is_file():
        if input_path.suffix.lower() != ".pdf":
            raise MinerUError(f"当前脚本只处理 PDF：{input_path}")
        return [input_path], []

    pdfs = [
        path.resolve()
        for path in input_path.rglob("*")
        if path.is_file() and path.suffix.lower() == ".pdf"
    ]
    return choose_pdf_per_canonical_folder(pdfs, output_root, force)


def output_dir_for(source_pdf: Path, output_root: Path, allocated: set[Path]) -> Path:
    source_pdf = source_pdf.resolve()
    output_root = output_root.resolve()
    if is_relative_to(source_pdf.parent, output_root) and source_pdf.parent != output_root:
        return source_pdf.parent

    title = document_title(source_pdf)
    candidate = output_root / safe_component(title)
    collision = candidate in allocated
    if candidate.exists() and not candidate_matches_source(candidate, source_pdf):
        existing_pdfs = [path for path in candidate.glob("*.pdf") if path.is_file()]
        collision = collision or bool(existing_pdfs) or (candidate / "full.md").exists()
    if collision:
        suffix = hashlib.sha256(str(source_pdf).encode("utf-8")).hexdigest()[:8]
        candidate = output_root / f"{safe_component(title, 84)}-{suffix}"
    allocated.add(candidate)
    return candidate


def build_plan(
    input_path: Path,
    output_root: Path,
    force: bool,
) -> Plan:
    output_root = output_root.expanduser().resolve()
    pdfs, skipped = discover_pdfs(input_path, output_root, force)
    jobs: list[Job] = []
    allocated: set[Path] = set()

    for source_pdf in pdfs:
        if source_pdf.stat().st_size == 0:
            skipped.append((source_pdf, "空文件"))
            continue
        if source_pdf.stat().st_size > MAX_FILE_BYTES:
            skipped.append((source_pdf, "超过 MinerU 200MB 限制"))
            continue

        output_dir = output_dir_for(source_pdf, output_root, allocated)
        if (output_dir / "full.md").exists() and not force:
            skipped.append((source_pdf, "目标目录已有 full.md"))
            continue

        canonical_pdf = output_dir / canonical_pdf_name(source_pdf)
        if source_pdf.parent == output_dir:
            canonical_pdf = source_pdf
        data_id = data_id_for(source_pdf)
        jobs.append(
            Job(
                source_pdf=source_pdf,
                output_dir=output_dir,
                canonical_pdf=canonical_pdf,
                data_id=data_id,
                api_name=api_name_for(source_pdf, data_id),
            )
        )
    return Plan(jobs=jobs, skipped=skipped)


def prepare_job(job: Job, copy_source: bool, force: bool) -> None:
    job.output_dir.mkdir(parents=True, exist_ok=True)
    if job.source_pdf.resolve() == job.canonical_pdf.resolve():
        return
    if not copy_source:
        job.canonical_pdf = job.source_pdf
        return
    if job.canonical_pdf.exists():
        same_size = job.canonical_pdf.stat().st_size == job.source_pdf.stat().st_size
        if same_size and not force:
            return
        if not force:
            raise MinerUError(f"目标 PDF 已存在且不同：{job.canonical_pdf}")
    shutil.copy2(job.source_pdf, job.canonical_pdf)


class MinerUClient:
    def __init__(self, token: str, session: Session | None = None) -> None:
        self.token = token
        self.session = session or requests.Session()
        if session is None:
            retry = Retry(
                total=3,
                connect=3,
                read=3,
                status=3,
                backoff_factor=1.0,
                status_forcelist=(429, 500, 502, 503, 504),
                allowed_methods=frozenset({"GET", "PUT"}),
                raise_on_status=False,
            )
            adapter = HTTPAdapter(max_retries=retry)
            self.session.mount("https://", adapter)

    @property
    def api_headers(self) -> dict[str, str]:
        return {
            "Authorization": f"Bearer {self.token}",
            "Content-Type": "application/json",
            "Accept": "application/json",
        }

    @staticmethod
    def _decode_api_response(response: Response) -> dict[str, Any]:
        try:
            payload = response.json()
        except ValueError as exc:
            raise MinerUError(
                f"MinerU 返回了非 JSON 响应（HTTP {response.status_code}）。"
            ) from exc
        if not isinstance(payload, dict):
            raise MinerUError("MinerU 返回结构不是 JSON object。")
        if response.status_code >= 400:
            raise MinerUApiError(
                payload.get("code", response.status_code),
                str(payload.get("msg", f"HTTP {response.status_code}")),
                str(payload.get("trace_id", "")),
            )
        if payload.get("code") != 0:
            raise MinerUApiError(
                payload.get("code"),
                str(payload.get("msg", "未知错误")),
                str(payload.get("trace_id", "")),
            )
        data = payload.get("data")
        if not isinstance(data, dict):
            raise MinerUError("MinerU 成功响应缺少 data object。")
        return payload

    def create_batch(
        self,
        jobs: Sequence[Job],
        *,
        model_version: str,
        language: str,
        enable_formula: bool,
        enable_table: bool,
        is_ocr: bool,
        page_ranges: str | None,
        extra_formats: Sequence[str],
    ) -> tuple[str, list[str]]:
        files: list[dict[str, Any]] = []
        for job in jobs:
            item: dict[str, Any] = {
                "name": job.api_name,
                "data_id": job.data_id,
                "is_ocr": is_ocr,
            }
            if page_ranges:
                item["page_ranges"] = page_ranges
            files.append(item)

        payload: dict[str, Any] = {
            "files": files,
            "model_version": model_version,
            "language": language,
            "enable_formula": enable_formula,
            "enable_table": enable_table,
        }
        if extra_formats:
            payload["extra_formats"] = list(extra_formats)

        response = self.session.post(
            f"{API_BASE}/file-urls/batch",
            headers=self.api_headers,
            json=payload,
            timeout=(20, 120),
        )
        result = self._decode_api_response(response)["data"]
        batch_id = str(result.get("batch_id", ""))
        urls = result.get("file_urls")
        if not batch_id or not isinstance(urls, list) or len(urls) != len(jobs):
            raise MinerUError("MinerU 上传链接响应缺少 batch_id，或 file_urls 数量不匹配。")
        return batch_id, [str(url) for url in urls]

    def upload(self, signed_url: str, pdf_path: Path) -> None:
        # Deliberately do not send Authorization or Content-Type to the OSS URL.
        with pdf_path.open("rb") as handle:
            response = self.session.put(
                signed_url,
                data=handle,
                headers={},
                timeout=(30, 900),
            )
        if response.status_code not in {200, 201}:
            raise MinerUError(f"文件上传失败：HTTP {response.status_code}")

    def get_batch_results(self, batch_id: str) -> list[dict[str, Any]]:
        response = self.session.get(
            f"{API_BASE}/extract-results/batch/{batch_id}",
            headers=self.api_headers,
            timeout=(20, 120),
        )
        result = self._decode_api_response(response)["data"]
        items = result.get("extract_result")
        if not isinstance(items, list):
            raise MinerUError("MinerU 批量结果缺少 extract_result list。")
        return [item for item in items if isinstance(item, dict)]

    def download(self, url: str, destination: Path) -> None:
        with self.session.get(url, stream=True, timeout=(30, 900)) as response:
            if response.status_code >= 400:
                raise MinerUError(f"结果 ZIP 下载失败：HTTP {response.status_code}")
            with destination.open("wb") as handle:
                for chunk in response.iter_content(chunk_size=1024 * 1024):
                    if chunk:
                        handle.write(chunk)


def validate_zip_members(archive: zipfile.ZipFile, destination: Path) -> None:
    root = destination.resolve()
    total_size = 0
    for member in archive.infolist():
        pure = PurePosixPath(member.filename.replace("\\", "/"))
        if pure.is_absolute() or ".." in pure.parts:
            raise MinerUError(f"结果 ZIP 包含不安全路径：{member.filename}")
        mode = member.external_attr >> 16
        if stat.S_ISLNK(mode):
            raise MinerUError(f"结果 ZIP 包含符号链接：{member.filename}")
        total_size += member.file_size
        if total_size > MAX_EXTRACTED_BYTES:
            raise MinerUError("结果 ZIP 解压后超过 2GB 安全上限。")
        target = (root / Path(*pure.parts)).resolve()
        if not is_relative_to(target, root):
            raise MinerUError(f"结果 ZIP 路径越界：{member.filename}")


def copy_result_tree(result_root: Path, output_dir: Path, force: bool) -> Path:
    full_md = result_root / "full.md"
    if not full_md.is_file():
        raise MinerUError("MinerU 结果中未找到 full.md。")

    files = sorted(
        (path for path in result_root.rglob("*") if path.is_file()),
        key=lambda path: (path.name == "full.md", str(path)),
    )
    # full.md is intentionally copied last: its presence is the completion marker.
    files.sort(key=lambda path: path.name == "full.md")
    for source in files:
        relative = source.relative_to(result_root)
        target = output_dir / relative
        if target.exists() and not force:
            raise MinerUError(f"结果文件已存在：{target}。如需覆盖请使用 --force。")
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, target)
    return output_dir / "full.md"


def extract_result_zip(zip_path: Path, output_dir: Path, force: bool) -> Path:
    output_dir.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(
        prefix=".mineru-extract-", dir=output_dir.parent
    ) as temp_dir_name:
        temp_dir = Path(temp_dir_name)
        try:
            with zipfile.ZipFile(zip_path) as archive:
                validate_zip_members(archive, temp_dir)
                archive.extractall(temp_dir)
        except zipfile.BadZipFile as exc:
            raise MinerUError("MinerU 结果不是有效 ZIP。") from exc

        full_markdowns = list(temp_dir.rglob("full.md"))
        if len(full_markdowns) != 1:
            raise MinerUError(
                f"MinerU 结果应有且只有一个 full.md，实际为 {len(full_markdowns)} 个。"
            )
        return copy_result_tree(full_markdowns[0].parent, output_dir, force)


def has_yaml_frontmatter(content: str) -> bool:
    normalized = content.lstrip("\ufeff")
    if not normalized.startswith("---\n"):
        return False
    closing = normalized.find("\n---\n", 4)
    if closing < 0:
        return False
    header = normalized[4:closing]
    return any(":" in line for line in header.splitlines())


def load_acquisition_metadata(
    source_pdf: Path | None,
    acquisition_method: str | None = None,
) -> dict[str, Any]:
    """Load provenance from a selected auto-discovery sidecar when available."""

    source_pdf = source_pdf or Path()
    metadata = load_sidecar_metadata(source_pdf)
    inferred = acquisition_method
    if not inferred:
        inferred = str(metadata.get("acquisition_method") or "")
    if not inferred:
        parts = {part.casefold() for part in source_pdf.parts}
        inferred = "auto_discovery" if "auto-discovered" in parts else "manual_upload"
    providers = metadata.get("discovered_via") or metadata.get("providers") or []
    if not isinstance(providers, list):
        providers = [str(providers)] if providers else []
    return {
        "acquisition_method": inferred,
        "discovered_via": [str(item) for item in providers],
        "discovery_run": str(metadata.get("discovery_run") or metadata.get("source_manifest") or ""),
        "acquired_at": str(metadata.get("acquired_at") or ""),
    }


def add_raw_frontmatter(
    md_path: Path,
    pdf_path: Path,
    source_pdf: Path | None = None,
    acquisition_method: str | None = None,
) -> None:
    content = md_path.read_text(encoding="utf-8-sig")
    if has_yaml_frontmatter(content):
        return
    provenance = load_acquisition_metadata(source_pdf or pdf_path, acquisition_method)
    canonicalized_at = datetime.now(timezone.utc).date().isoformat()
    frontmatter = "\n".join(
        [
            "---",
            f"title: {yaml_string(document_title(source_pdf or pdf_path))}",
            "year: null",
            "source_type: paper",
            'why_relevant: ""',
            f"acquisition_method: {provenance['acquisition_method']}",
            f"discovered_via: {json.dumps(provenance['discovered_via'], ensure_ascii=False)}",
            f"discovery_run: {yaml_string(provenance['discovery_run'])}",
            "triage_status: promoted",
            "selected_by_user: true",
            f"acquired_at: {yaml_string(provenance['acquired_at'])}",
            f"canonicalized_at: {canonicalized_at}",
            "ingest_status: pending_ingest",
            f"pdf_path: {yaml_string(display_path(pdf_path))}",
            f"raw_md: {yaml_string(display_path(md_path))}",
            "---",
            "",
        ]
    )
    temp_path = md_path.with_name(f"{md_path.name}.tmp")
    temp_path.write_text(frontmatter + content.lstrip("\ufeff"), encoding="utf-8")
    temp_path.replace(md_path)


def chunks(items: Sequence[Job], size: int) -> Iterable[Sequence[Job]]:
    for index in range(0, len(items), size):
        yield items[index : index + size]


def result_for_job(results: Sequence[dict[str, Any]], job: Job) -> dict[str, Any] | None:
    for item in results:
        if item.get("data_id") == job.data_id:
            return item
    for item in results:
        if item.get("file_name") == job.api_name:
            return item
    return None


def complete_job(
    client: MinerUClient,
    job: Job,
    batch_id: str,
    result: Mapping[str, Any],
    force: bool,
    frontmatter: bool,
    acquisition_method: str | None,
) -> None:
    zip_url = str(result.get("full_zip_url", ""))
    if not zip_url:
        raise MinerUError("任务状态为 done，但没有 full_zip_url。")

    job.output_dir.parent.mkdir(parents=True, exist_ok=True)
    fd, temp_name = tempfile.mkstemp(
        prefix=".mineru-result-", suffix=".zip", dir=job.output_dir.parent
    )
    os.close(fd)
    temp_zip = Path(temp_name)
    try:
        client.download(zip_url, temp_zip)
        md_path = extract_result_zip(temp_zip, job.output_dir, force=force)
        if frontmatter:
            add_raw_frontmatter(
                md_path,
                job.canonical_pdf,
                source_pdf=job.source_pdf,
                acquisition_method=acquisition_method,
            )
    finally:
        temp_zip.unlink(missing_ok=True)

    job.state = "done"
    job.error = ""
    write_job_status(job, batch_id=batch_id, markdown=display_path(job.output_dir / "full.md"))


def run_batch(
    client: MinerUClient,
    jobs: Sequence[Job],
    args: argparse.Namespace,
) -> tuple[int, int]:
    batch_id, upload_urls = client.create_batch(
        jobs,
        model_version=args.model,
        language=args.language,
        enable_formula=args.enable_formula,
        enable_table=args.enable_table,
        is_ocr=args.ocr,
        page_ranges=args.page_ranges,
        extra_formats=args.extra_format,
    )
    print(f"创建批次：{batch_id}（{len(jobs)} 个文件）")

    pending: dict[str, Job] = {}
    failures = 0
    for job, signed_url in zip(jobs, upload_urls, strict=True):
        try:
            job.state = "uploading"
            write_job_status(job, batch_id=batch_id)
            client.upload(signed_url, job.canonical_pdf)
            job.state = "waiting-file"
            write_job_status(job, batch_id=batch_id)
            pending[job.data_id] = job
            print(f"  已上传：{job.canonical_pdf.name}")
        except Exception as exc:  # keep the rest of the batch usable
            job.state = "upload_failed"
            job.error = str(exc)
            write_job_status(job, batch_id=batch_id)
            failures += 1
            print(f"  上传失败：{job.canonical_pdf.name} — {exc}", file=sys.stderr)

    completed = 0
    last_progress: dict[str, str] = {}
    started = time.monotonic()
    while pending and time.monotonic() - started < args.timeout:
        results = client.get_batch_results(batch_id)
        for data_id, job in list(pending.items()):
            result = result_for_job(results, job)
            if result is None:
                continue
            state = str(result.get("state", "unknown"))
            progress = result.get("extract_progress") or {}
            progress_text = state
            if isinstance(progress, dict) and progress.get("total_pages"):
                progress_text += (
                    f" {progress.get('extracted_pages', 0)}/{progress.get('total_pages')} 页"
                )
            if last_progress.get(data_id) != progress_text:
                print(f"  {job.canonical_pdf.name}: {progress_text}")
                last_progress[data_id] = progress_text

            job.state = state
            job.error = str(result.get("err_msg", "")) if state == "failed" else ""
            write_job_status(job, batch_id=batch_id, progress=progress)

            if state == "failed":
                failures += 1
                pending.pop(data_id)
                print(f"  解析失败：{job.canonical_pdf.name} — {job.error}", file=sys.stderr)
            elif state == "done":
                try:
                    complete_job(
                        client,
                        job,
                        batch_id,
                        result,
                        force=args.force,
                        frontmatter=args.frontmatter,
                        acquisition_method=args.acquisition_method,
                    )
                    completed += 1
                    print(f"  完成：{display_path(job.output_dir / 'full.md')}")
                except Exception as exc:
                    job.state = "result_failed"
                    job.error = str(exc)
                    write_job_status(job, batch_id=batch_id)
                    failures += 1
                    print(f"  结果处理失败：{job.canonical_pdf.name} — {exc}", file=sys.stderr)
                pending.pop(data_id)

        if pending:
            time.sleep(args.poll_interval)

    if pending:
        for job in pending.values():
            job.state = "timeout"
            job.error = f"轮询超过 {args.timeout} 秒；batch_id={batch_id}"
            write_job_status(job, batch_id=batch_id)
            failures += 1
            print(f"  超时：{job.canonical_pdf.name}（batch_id={batch_id}）", file=sys.stderr)

    return completed, failures


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="用 MinerU 精准解析 API 批量把本地 PDF 转成 Markdown。",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        "input",
        nargs="?",
        type=Path,
        default=DEFAULT_INPUT,
        help="PDF 文件或包含 PDF 的目录",
    )
    parser.add_argument(
        "--output-root",
        type=Path,
        default=DEFAULT_OUTPUT_ROOT,
        help="一文一夹输出根目录",
    )
    parser.add_argument(
        "--api-key-file",
        type=Path,
        default=DEFAULT_KEY_FILE,
        help="只包含 MinerU token 的文本文件",
    )
    parser.add_argument("--model", choices=("pipeline", "vlm"), default="vlm")
    parser.add_argument("--language", default="en", help="MinerU language 值；英文论文建议 en")
    parser.add_argument("--ocr", action="store_true", help="启用 OCR")
    parser.add_argument(
        "--enable-formula",
        action=argparse.BooleanOptionalAction,
        default=True,
        help="启用公式识别",
    )
    parser.add_argument(
        "--enable-table",
        action=argparse.BooleanOptionalAction,
        default=True,
        help="启用表格识别",
    )
    parser.add_argument("--page-ranges", help='页码范围，如 "1-20" 或 "2,4-6"')
    parser.add_argument(
        "--extra-format",
        action="append",
        choices=("docx", "html", "latex"),
        default=[],
        help="额外导出格式，可重复使用",
    )
    parser.add_argument(
        "--batch-size",
        type=int,
        default=MAX_BATCH_SIZE,
        help="每次申请上传链接的文件数，官方上限 50",
    )
    parser.add_argument("--poll-interval", type=float, default=10.0, help="轮询间隔秒数")
    parser.add_argument("--timeout", type=int, default=3600, help="每批最长轮询秒数")
    parser.add_argument(
        "--copy-source",
        action=argparse.BooleanOptionalAction,
        default=True,
        help="输入位于 canonical 外时，将 PDF 复制进一文一夹目录",
    )
    parser.add_argument(
        "--frontmatter",
        action=argparse.BooleanOptionalAction,
        default=True,
        help="给 full.md 添加 pending_ingest frontmatter",
    )
    parser.add_argument(
        "--acquisition-method",
        choices=("manual_upload", "auto_discovery"),
        help="覆盖自动推断的采集来源；默认从输入路径/metadata.json 推断",
    )
    parser.add_argument("--force", action="store_true", help="重新解析并覆盖同名结果文件")
    parser.add_argument("--dry-run", action="store_true", help="只显示计划，不读取 key、不调用 API")
    return parser


def validate_args(args: argparse.Namespace) -> None:
    if not 1 <= args.batch_size <= MAX_BATCH_SIZE:
        raise MinerUError("--batch-size 必须在 1 到 50 之间。")
    if args.poll_interval <= 0:
        raise MinerUError("--poll-interval 必须大于 0。")
    if args.timeout <= 0:
        raise MinerUError("--timeout 必须大于 0。")


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        validate_args(args)
        plan = build_plan(args.input, args.output_root, force=args.force)
        for path, reason in plan.skipped:
            print(f"跳过：{display_path(path)} — {reason}")

        if not plan.jobs:
            print("没有需要解析的 PDF。")
            return 0

        print(f"待解析：{len(plan.jobs)} 个 PDF")
        for job in plan.jobs:
            print(f"  {display_path(job.source_pdf)} -> {display_path(job.output_dir / 'full.md')}")
        if args.dry_run:
            print("dry-run 完成：未读取 API key，未调用 MinerU。")
            return 0

        prepared: list[Job] = []
        local_failures = 0
        for job in plan.jobs:
            try:
                prepare_job(job, copy_source=args.copy_source, force=args.force)
                prepared.append(job)
            except Exception as exc:
                job.state = "prepare_failed"
                job.error = str(exc)
                job.output_dir.mkdir(parents=True, exist_ok=True)
                write_job_status(job)
                local_failures += 1
                print(f"本地准备失败：{job.source_pdf.name} — {exc}", file=sys.stderr)

        if not prepared:
            return 1

        token = read_api_key(args.api_key_file)
        client = MinerUClient(token)
        completed = 0
        failures = local_failures
        for batch_jobs in chunks(prepared, args.batch_size):
            batch_completed, batch_failures = run_batch(client, batch_jobs, args)
            completed += batch_completed
            failures += batch_failures

        print(f"汇总：完成 {completed}，失败 {failures}，跳过 {len(plan.skipped)}。")
        if completed:
            print("下一步：按 schema/agent-a-compile.md 对 pending_ingest 条目执行 A 编译。")
        return 1 if failures else 0
    except KeyboardInterrupt:
        print("已中断；可查看各输出目录的 .mineru-task.json。", file=sys.stderr)
        return 130
    except MinerUError as exc:
        print(f"错误：{exc}", file=sys.stderr)
        return 1
    except requests.RequestException as exc:
        print(f"网络请求失败：{exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
