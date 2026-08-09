#!/usr/bin/env python3
"""Search scholarly APIs and save triage-only candidates under raw/inbox.

The discovery layer deliberately stops before canonical ingestion:

1. Search arXiv and configured OpenAlex, Tavily, and SerpApi sources.
2. Normalize and deduplicate metadata.
3. Rank candidates with a transparent lexical/recency heuristic.
4. Write a dated report and JSON manifest to raw/inbox/auto-discovered/runs.
5. Optionally download openly accessible PDFs into one-paper-one-folder paths.

No result is promoted to raw/canonical, compiled into wiki, or added to Graphify.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import time
import unicodedata
import xml.etree.ElementTree as ET
from dataclasses import asdict, dataclass, field
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence
from urllib.parse import urlparse

try:
    import requests
    from requests import Session
    from requests.adapters import HTTPAdapter
    from urllib3.util.retry import Retry
except ImportError as exc:  # pragma: no cover - runtime dependency
    raise SystemExit("缺少 requests。请运行：py -3 -m pip install requests") from exc


PROJECT_ROOT = Path(__file__).resolve().parents[1]
AUTO_DISCOVERY_ROOT = PROJECT_ROOT / "raw" / "inbox" / "auto-discovered"
DEFAULT_OUTPUT_ROOT = AUTO_DISCOVERY_ROOT / "runs"
DEFAULT_CACHE_ROOT = AUTO_DISCOVERY_ROOT / ".paper-search-cache"
DEFAULT_PRESETS = Path(__file__).with_name("paper_search_topics.json")
DEFAULT_KEY_FILE = Path(os.environ.get("PAPER_SEARCH_KEY_FILE", r"E:\知识库\aoikey.txt"))
ARXIV_ENDPOINT = "https://export.arxiv.org/api/query"
OPENALEX_ENDPOINT = "https://api.openalex.org/works"
TAVILY_ENDPOINT = "https://api.tavily.com/search"
SERPAPI_ENDPOINT = "https://serpapi.com/search.json"
USER_AGENT = "wireless-charging-wiki-paper-discovery/1.0"
MAX_PDF_BYTES = 200 * 1024 * 1024
ARXIV_MIN_INTERVAL_SECONDS = 3.0
PROVIDER_ENV_KEYS = {
    "openalex": "OPENALEX_API_KEY",
    "tavily": "TAVILY_API_KEY",
    "serpapi": "SERPAPI_API_KEY",
}
PROVIDER_LABELS = {
    "openalex": {"OPENALEXAPIKEY", "OPENALEXKEY"},
    "tavily": {"TAVILYAPIKEY", "TAVILYKEY"},
    "serpapi": {"SERPAPIAPIKEY", "SERPAPIKEY", "SERPKEY"},
}
ACADEMIC_DOMAINS = [
    "arxiv.org",
    "doi.org",
    "ieeexplore.ieee.org",
    "dl.acm.org",
    "link.springer.com",
    "sciencedirect.com",
    "onlinelibrary.wiley.com",
    "tandfonline.com",
    "nature.com",
    "science.org",
    "mdpi.com",
    "pubmed.ncbi.nlm.nih.gov",
]


class PaperSearchError(RuntimeError):
    """Base error for discovery failures."""


class ProviderError(PaperSearchError):
    """A scholarly provider returned an unusable response."""


@dataclass
class SearchQuery:
    label: str
    arxiv: str
    openalex: str


@dataclass
class Paper:
    title: str
    authors: list[str] = field(default_factory=list)
    abstract: str = ""
    published_date: str = ""
    updated_date: str = ""
    year: int | None = None
    doi: str = ""
    arxiv_id: str = ""
    landing_url: str = ""
    pdf_url: str = ""
    is_open_access: bool | None = None
    license: str = ""
    providers: list[str] = field(default_factory=list)
    provider_ids: dict[str, str] = field(default_factory=dict)
    matched_queries: list[str] = field(default_factory=list)
    score: float = 0.0
    score_reasons: list[str] = field(default_factory=list)
    candidate_id: str = ""
    title_matches: list[str] = field(default_factory=list)
    abstract_matches: list[str] = field(default_factory=list)
    local_pdf: str = ""
    acquisition_method: str = "auto_discovery"
    discovered_via: list[str] = field(default_factory=list)
    discovery_run: str = ""
    triage_status: str = "pending"
    selected_by_user: bool = False
    acquired_at: str = ""
    selected_at: str = ""
    canonicalized_at: str = ""


@dataclass
class SearchOutcome:
    papers: list[Paper]
    errors: list[str]
    provider_counts: dict[str, int]
    cache_hits: int = 0


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def normalize_space(value: str) -> str:
    return re.sub(r"\s+", " ", value or "").strip()


def normalize_title(value: str) -> str:
    value = unicodedata.normalize("NFKD", value).casefold()
    return "".join(character for character in value if character.isalnum())


def normalize_doi(value: str) -> str:
    value = (value or "").strip().casefold()
    value = re.sub(r"^https?://(?:dx\.)?doi\.org/", "", value)
    return re.sub(r"^doi:\s*", "", value).strip()


def normalize_arxiv_id(value: str) -> str:
    value = (value or "").strip()
    value = re.sub(r"^https?://arxiv\.org/(?:abs|pdf)/", "", value, flags=re.I)
    value = re.sub(r"\.pdf$", "", value, flags=re.I)
    return re.sub(r"v\d+$", "", value, flags=re.I).casefold()


def safe_component(value: str, max_length: int = 90) -> str:
    asciiish = unicodedata.normalize("NFKD", value)
    asciiish = "".join(c for c in asciiish if not unicodedata.combining(c))
    asciiish = re.sub(r'[<>:"/\\|?*\x00-\x1f]', "_", asciiish)
    asciiish = re.sub(r"\s+", "-", asciiish)
    asciiish = re.sub(r"[^\w\-.]+", "-", asciiish, flags=re.UNICODE)
    asciiish = re.sub(r"[-_]{2,}", "-", asciiish).strip(" .-_")
    return (asciiish or "paper")[:max_length].rstrip(" .-_") or "paper"


def atomic_write_text(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temp = path.with_name(path.name + ".tmp")
    temp.write_text(content, encoding="utf-8")
    temp.replace(path)


def atomic_write_json(path: Path, payload: Any) -> None:
    atomic_write_text(path, json.dumps(payload, ensure_ascii=False, indent=2) + "\n")


def clean_key_value(value: str) -> str:
    value = value.strip()
    if value.lower().startswith("bearer "):
        value = value[7:].strip()
    if len(value) >= 2 and value[0] == value[-1] and value[0] in {'"', "'"}:
        value = value[1:-1].strip()
    if "\n" in value or "\r" in value:
        raise PaperSearchError("API key 不能包含换行。")
    return value


def normalize_key_label(value: str) -> str:
    return re.sub(r"[^A-Za-z0-9]", "", value).upper()


def parse_key_file(path: Path) -> tuple[dict[str, str], list[str]]:
    try:
        content = path.expanduser().read_text(encoding="utf-8-sig")
    except FileNotFoundError:
        return {}, []
    except OSError as exc:
        raise PaperSearchError(f"无法读取 API key 文件：{path}（{exc}）") from exc

    labeled: dict[str, str] = {}
    unlabeled: list[str] = []
    for raw_line in content.splitlines():
        line = raw_line.strip()
        if not line or line.startswith(("#", ";")):
            continue
        match = re.match(r"^([A-Za-z][A-Za-z0-9_-]{1,50})\s*[:=]\s*(.+?)\s*$", line)
        if match:
            labeled[normalize_key_label(match.group(1))] = clean_key_value(match.group(2))
        else:
            unlabeled.append(clean_key_value(line))
    return labeled, unlabeled


def read_provider_key(
    provider: str,
    shared_path: Path | None,
    provider_path: Path | None = None,
    environ: Mapping[str, str] | None = None,
) -> str:
    environment = os.environ if environ is None else environ
    env_name = PROVIDER_ENV_KEYS[provider]
    value = clean_key_value(environment.get(env_name, ""))
    if value:
        return value

    for path, allow_unlabeled in ((provider_path, True), (shared_path, False)):
        if path is None:
            continue
        labeled, unlabeled = parse_key_file(path)
        for alias in PROVIDER_LABELS[provider]:
            if labeled.get(alias):
                return labeled[alias]
        if allow_unlabeled and len(unlabeled) == 1:
            return unlabeled[0]
    return ""


def build_session() -> Session:
    session = requests.Session()
    retry = Retry(
        total=3,
        connect=3,
        read=3,
        status=3,
        backoff_factor=1.0,
        status_forcelist=(429, 500, 502, 503, 504),
        allowed_methods=frozenset({"GET", "POST"}),
        respect_retry_after_header=True,
    )
    session.mount("https://", HTTPAdapter(max_retries=retry))
    session.headers.update({"User-Agent": USER_AGENT, "Accept": "application/json, application/atom+xml"})
    return session


class ResponseCache:
    def __init__(self, root: Path, max_age_hours: int = 24) -> None:
        self.root = root
        self.max_age = timedelta(hours=max_age_hours)
        self.hits = 0

    def _path(self, provider: str, params: Mapping[str, Any], suffix: str) -> Path:
        safe_params = {key: value for key, value in params.items() if key != "api_key"}
        identity = json.dumps([provider, safe_params], sort_keys=True, ensure_ascii=False)
        digest = hashlib.sha256(identity.encode("utf-8")).hexdigest()
        return self.root / provider / f"{digest}.{suffix}"

    def get(self, provider: str, params: Mapping[str, Any], suffix: str) -> bytes | None:
        path = self._path(provider, params, suffix)
        if not path.exists():
            return None
        modified = datetime.fromtimestamp(path.stat().st_mtime, timezone.utc)
        if datetime.now(timezone.utc) - modified > self.max_age:
            return None
        self.hits += 1
        return path.read_bytes()

    def put(self, provider: str, params: Mapping[str, Any], suffix: str, content: bytes) -> None:
        path = self._path(provider, params, suffix)
        path.parent.mkdir(parents=True, exist_ok=True)
        temp = path.with_name(path.name + ".tmp")
        temp.write_bytes(content)
        temp.replace(path)


def display_path(path: Path) -> str:
    """Prefer a stable project-relative path in persisted provenance."""

    resolved = path.expanduser().resolve()
    try:
        return resolved.relative_to(PROJECT_ROOT).as_posix()
    except ValueError:
        return resolved.as_posix()


def _safe_get(
    session: Session,
    endpoint: str,
    params: Mapping[str, Any],
    provider: str,
    timeout: tuple[int, int] = (10, 45),
):
    response = session.get(endpoint, params=params, timeout=timeout)
    if response.status_code >= 400:
        # Do not include response.url: it may contain the OpenAlex API key.
        retry_after = response.headers.get("Retry-After", "")
        suffix = f"，Retry-After={retry_after}" if retry_after else ""
        raise ProviderError(f"{provider} 返回 HTTP {response.status_code}{suffix}")
    return response


def _safe_post(
    session: Session,
    endpoint: str,
    payload: Mapping[str, Any],
    provider: str,
    headers: Mapping[str, str],
    timeout: tuple[int, int] = (10, 45),
):
    response = session.post(endpoint, json=payload, headers=dict(headers), timeout=timeout)
    if response.status_code >= 400:
        retry_after = response.headers.get("Retry-After", "")
        suffix = f"，Retry-After={retry_after}" if retry_after else ""
        raise ProviderError(f"{provider} 返回 HTTP {response.status_code}{suffix}")
    return response


def parse_arxiv_feed(content: bytes, query_label: str) -> list[Paper]:
    try:
        root = ET.fromstring(content)
    except ET.ParseError as exc:
        raise ProviderError(f"arXiv Atom 响应无法解析：{exc}") from exc

    atom = "{http://www.w3.org/2005/Atom}"
    arxiv = "{http://arxiv.org/schemas/atom}"
    papers: list[Paper] = []
    for entry in root.findall(f"{atom}entry"):
        entry_id = normalize_space(entry.findtext(f"{atom}id", ""))
        arxiv_id = normalize_arxiv_id(entry_id)
        links = {
            link.attrib.get("rel", "alternate"): link.attrib.get("href", "")
            for link in entry.findall(f"{atom}link")
        }
        pdf_url = ""
        for link in entry.findall(f"{atom}link"):
            if link.attrib.get("title") == "pdf" or link.attrib.get("type") == "application/pdf":
                pdf_url = link.attrib.get("href", "")
                break
        published = normalize_space(entry.findtext(f"{atom}published", ""))
        doi = normalize_doi(entry.findtext(f"{arxiv}doi", ""))
        authors = [
            normalize_space(author.findtext(f"{atom}name", ""))
            for author in entry.findall(f"{atom}author")
        ]
        authors = [author for author in authors if author]
        year = int(published[:4]) if re.match(r"^\d{4}", published) else None
        papers.append(
            Paper(
                title=normalize_space(entry.findtext(f"{atom}title", "")),
                authors=authors,
                abstract=normalize_space(entry.findtext(f"{atom}summary", "")),
                published_date=published[:10],
                updated_date=normalize_space(entry.findtext(f"{atom}updated", ""))[:10],
                year=year,
                doi=doi,
                arxiv_id=arxiv_id,
                landing_url=links.get("alternate", entry_id),
                pdf_url=pdf_url,
                is_open_access=True,
                license=normalize_space(entry.findtext(f"{arxiv}license", "")),
                providers=["arxiv"],
                provider_ids={"arxiv": arxiv_id},
                matched_queries=[query_label],
            )
        )
    return papers


def inverted_abstract(index: Any) -> str:
    if not isinstance(index, dict):
        return ""
    positioned: list[tuple[int, str]] = []
    for word, positions in index.items():
        if not isinstance(positions, list):
            continue
        for position in positions:
            if isinstance(position, int):
                positioned.append((position, str(word)))
    return " ".join(word for _, word in sorted(positioned))


def parse_openalex_payload(payload: Mapping[str, Any], query_label: str) -> list[Paper]:
    papers: list[Paper] = []
    for item in payload.get("results", []) or []:
        if not isinstance(item, dict):
            continue
        best_oa = item.get("best_oa_location") or {}
        primary = item.get("primary_location") or {}
        location = best_oa or primary
        open_access = item.get("open_access") or {}
        authors: list[str] = []
        for authorship in item.get("authorships", []) or []:
            author = (authorship or {}).get("author") or {}
            name = normalize_space(author.get("display_name", ""))
            if name:
                authors.append(name)
        date = normalize_space(item.get("publication_date", ""))
        year_value = item.get("publication_year")
        year = year_value if isinstance(year_value, int) else None
        provider_id = normalize_space(item.get("id", ""))
        papers.append(
            Paper(
                title=normalize_space(item.get("display_name", "")),
                authors=authors,
                abstract=normalize_space(inverted_abstract(item.get("abstract_inverted_index"))),
                published_date=date,
                year=year,
                doi=normalize_doi(item.get("doi", "")),
                landing_url=normalize_space(location.get("landing_page_url", "")) or provider_id,
                pdf_url=normalize_space(location.get("pdf_url", "")),
                is_open_access=open_access.get("is_oa") if isinstance(open_access.get("is_oa"), bool) else None,
                license=normalize_space(location.get("license", "")),
                providers=["openalex"],
                provider_ids={"openalex": provider_id},
                matched_queries=[query_label],
            )
        )
    return papers


def extract_scholarly_ids(*values: str) -> tuple[str, str]:
    text = " ".join(value for value in values if value)
    doi_match = re.search(r"\b10\.\d{4,9}/[-._;()/:A-Z0-9]+", text, flags=re.I)
    doi = normalize_doi(doi_match.group(0).rstrip(".,;)]}")) if doi_match else ""
    arxiv_match = re.search(
        r"(?:arxiv(?:\.org/(?:abs|pdf)/|:))\s*([a-z-]+/\d{7}|\d{4}\.\d{4,5})(?:v\d+)?",
        text,
        flags=re.I,
    )
    arxiv_id = normalize_arxiv_id(arxiv_match.group(1)) if arxiv_match else ""
    return doi, arxiv_id


def parse_tavily_payload(payload: Mapping[str, Any], query_label: str) -> list[Paper]:
    papers: list[Paper] = []
    for item in payload.get("results", []) or []:
        if not isinstance(item, dict):
            continue
        title = normalize_space(str(item.get("title", "")))
        url = normalize_space(str(item.get("url", "")))
        content = normalize_space(str(item.get("content", "")))
        if not title or not url:
            continue
        doi, arxiv_id = extract_scholarly_ids(url, title, content)
        published = normalize_space(str(item.get("published_date", "")))[:10]
        year_match = re.search(r"\b(19|20)\d{2}\b", published)
        year = int(year_match.group(0)) if year_match else None
        is_pdf = urlparse(url).path.casefold().endswith(".pdf") or "/pdf/" in url.casefold()
        provider_id = hashlib.sha256(url.encode("utf-8")).hexdigest()[:20]
        papers.append(
            Paper(
                title=title,
                abstract=content,
                published_date=published,
                year=year,
                doi=doi,
                arxiv_id=arxiv_id,
                landing_url=url,
                pdf_url=url if is_pdf else "",
                is_open_access=True if is_pdf else None,
                providers=["tavily"],
                provider_ids={"tavily": provider_id},
                matched_queries=[query_label],
            )
        )
    return papers


def parse_serpapi_payload(payload: Mapping[str, Any], query_label: str) -> list[Paper]:
    if payload.get("error"):
        raise ProviderError("SerpApi 返回业务错误，请检查额度、Key 或查询参数。")
    papers: list[Paper] = []
    for item in payload.get("organic_results", []) or []:
        if not isinstance(item, dict):
            continue
        title = normalize_space(str(item.get("title", "")))
        link = normalize_space(str(item.get("link", "")))
        snippet = normalize_space(str(item.get("snippet", "")))
        publication = item.get("publication_info") or {}
        summary = normalize_space(str(publication.get("summary", "")))
        if not title:
            continue
        resources = item.get("resources") or []
        pdf_url = ""
        for resource in resources:
            if not isinstance(resource, dict):
                continue
            resource_url = normalize_space(str(resource.get("link", "")))
            if str(resource.get("file_format", "")).casefold() == "pdf" and resource_url:
                pdf_url = resource_url
                break
        if not pdf_url and urlparse(link).path.casefold().endswith(".pdf"):
            pdf_url = link
        doi, arxiv_id = extract_scholarly_ids(link, pdf_url, title, snippet, summary)
        year_match = re.search(r"\b(19|20)\d{2}\b", summary)
        year = int(year_match.group(0)) if year_match else None
        author_text = summary.split(" - ", 1)[0].strip()
        authors = [normalize_space(author) for author in author_text.split(",") if normalize_space(author)]
        provider_id = normalize_space(str(item.get("result_id", "")))
        if not provider_id:
            provider_id = hashlib.sha256((link or title).encode("utf-8")).hexdigest()[:20]
        papers.append(
            Paper(
                title=title,
                authors=authors,
                abstract=snippet,
                year=year,
                doi=doi,
                arxiv_id=arxiv_id,
                landing_url=link,
                pdf_url=pdf_url,
                is_open_access=True if pdf_url else None,
                providers=["serpapi"],
                provider_ids={"serpapi": provider_id},
                matched_queries=[query_label],
            )
        )
    return papers


def search_arxiv(
    session: Session,
    cache: ResponseCache,
    query: SearchQuery,
    limit: int,
    last_request_at: list[float],
) -> list[Paper]:
    params = {
        "search_query": query.arxiv,
        "start": 0,
        "max_results": limit,
        "sortBy": "relevance",
        "sortOrder": "descending",
    }
    cached = cache.get("arxiv", params, "xml")
    if cached is None:
        elapsed = time.monotonic() - last_request_at[0]
        if last_request_at[0] and elapsed < ARXIV_MIN_INTERVAL_SECONDS:
            time.sleep(ARXIV_MIN_INTERVAL_SECONDS - elapsed)
        response = _safe_get(session, ARXIV_ENDPOINT, params, "arXiv")
        cached = response.content
        cache.put("arxiv", params, "xml", cached)
        last_request_at[0] = time.monotonic()
    return parse_arxiv_feed(cached, query.label)


def search_openalex(
    session: Session,
    cache: ResponseCache,
    query: SearchQuery,
    limit: int,
    key: str,
    since_year: int | None,
) -> list[Paper]:
    params: dict[str, Any] = {
        "search": query.openalex,
        "per_page": min(limit, 100),
        "sort": "relevance_score:desc",
        "api_key": key,
    }
    if since_year:
        params["filter"] = f"from_publication_date:{since_year}-01-01"
    cached = cache.get("openalex", params, "json")
    if cached is None:
        response = _safe_get(session, OPENALEX_ENDPOINT, params, "OpenAlex")
        cached = response.content
        cache.put("openalex", params, "json", cached)
    try:
        payload = json.loads(cached)
    except (json.JSONDecodeError, UnicodeDecodeError) as exc:
        raise ProviderError(f"OpenAlex JSON 响应无法解析：{exc}") from exc
    return parse_openalex_payload(payload, query.label)


def search_tavily(
    session: Session,
    cache: ResponseCache,
    query: SearchQuery,
    limit: int,
    key: str,
) -> list[Paper]:
    payload: dict[str, Any] = {
        "query": query.openalex,
        "search_depth": "basic",
        "topic": "general",
        "max_results": min(limit, 20),
        "include_answer": False,
        "include_raw_content": False,
        "include_images": False,
        "include_domains": ACADEMIC_DOMAINS,
    }
    cached = cache.get("tavily", payload, "json")
    if cached is None:
        response = _safe_post(
            session,
            TAVILY_ENDPOINT,
            payload,
            "Tavily",
            {"Authorization": f"Bearer {key}", "Content-Type": "application/json"},
        )
        cached = response.content
        cache.put("tavily", payload, "json", cached)
    try:
        response_payload = json.loads(cached)
    except (json.JSONDecodeError, UnicodeDecodeError) as exc:
        raise ProviderError(f"Tavily JSON 响应无法解析：{exc}") from exc
    return parse_tavily_payload(response_payload, query.label)


def search_serpapi(
    session: Session,
    cache: ResponseCache,
    query: SearchQuery,
    limit: int,
    key: str,
    since_year: int | None,
) -> list[Paper]:
    params: dict[str, Any] = {
        "engine": "google_scholar",
        "q": query.openalex,
        "num": min(limit, 20),
        "hl": "en",
        "output": "json",
        "api_key": key,
    }
    if since_year:
        params["as_ylo"] = since_year
    cached = cache.get("serpapi", params, "json")
    if cached is None:
        response = _safe_get(session, SERPAPI_ENDPOINT, params, "SerpApi")
        cached = response.content
        cache.put("serpapi", params, "json", cached)
    try:
        payload = json.loads(cached)
    except (json.JSONDecodeError, UnicodeDecodeError) as exc:
        raise ProviderError(f"SerpApi JSON 响应无法解析：{exc}") from exc
    return parse_serpapi_payload(payload, query.label)


def merge_text(primary: str, secondary: str) -> str:
    return primary if len(primary) >= len(secondary) else secondary


def merge_papers(target: Paper, incoming: Paper) -> Paper:
    target.title = merge_text(target.title, incoming.title)
    target.abstract = merge_text(target.abstract, incoming.abstract)
    target.authors = target.authors or incoming.authors
    target.published_date = target.published_date or incoming.published_date
    target.updated_date = target.updated_date or incoming.updated_date
    target.year = target.year or incoming.year
    target.doi = target.doi or incoming.doi
    target.arxiv_id = target.arxiv_id or incoming.arxiv_id
    target.landing_url = target.landing_url or incoming.landing_url
    target.pdf_url = target.pdf_url or incoming.pdf_url
    target.is_open_access = (
        True if target.is_open_access is True or incoming.is_open_access is True
        else target.is_open_access if target.is_open_access is not None
        else incoming.is_open_access
    )
    target.license = target.license or incoming.license
    target.providers = sorted(set(target.providers + incoming.providers))
    target.provider_ids.update(incoming.provider_ids)
    target.matched_queries = sorted(set(target.matched_queries + incoming.matched_queries))
    return target


def deduplicate(papers: Iterable[Paper]) -> list[Paper]:
    unique: list[Paper] = []
    doi_index: dict[str, Paper] = {}
    arxiv_index: dict[str, Paper] = {}
    title_index: dict[str, Paper] = {}
    for paper in papers:
        doi = normalize_doi(paper.doi)
        arxiv_id = normalize_arxiv_id(paper.arxiv_id)
        title = normalize_title(paper.title)
        existing = doi_index.get(doi) if doi else None
        existing = existing or (arxiv_index.get(arxiv_id) if arxiv_id else None)
        existing = existing or (title_index.get(title) if title else None)
        if existing:
            paper = merge_papers(existing, paper)
        else:
            unique.append(paper)
        if doi:
            doi_index[doi] = paper
        if arxiv_id:
            arxiv_index[arxiv_id] = paper
        if title:
            title_index[title] = paper
    return unique


STOPWORDS = {
    "a", "an", "and", "for", "in", "of", "on", "or", "the", "to", "with",
    "all", "wireless", "power", "transfer", "network", "networks",
}


def query_terms(queries: Sequence[SearchQuery]) -> set[str]:
    labels = " ".join(query.label for query in queries).casefold()
    return {
        term for term in re.findall(r"[a-z0-9]{3,}", labels)
        if term not in STOPWORDS
    }


def rank_papers(papers: Iterable[Paper], queries: Sequence[SearchQuery]) -> list[Paper]:
    terms = query_terms(queries)
    current_year = datetime.now(timezone.utc).year
    for paper in papers:
        title = paper.title.casefold()
        abstract = paper.abstract.casefold()
        title_hits = sorted(term for term in terms if term in title)
        abstract_hits = sorted(term for term in terms if term in abstract and term not in title_hits)
        paper.title_matches = title_hits
        paper.abstract_matches = abstract_hits
        score = min(len(title_hits), 8) * 3.0 + min(len(abstract_hits), 8) * 0.75
        reasons: list[str] = []
        if title_hits:
            reasons.append("标题命中：" + ", ".join(title_hits[:8]))
        if abstract_hits:
            reasons.append("摘要命中：" + ", ".join(abstract_hits[:8]))
        if paper.year:
            recency = max(0.0, 3.0 - max(0, current_year - paper.year) * 0.25)
            score += recency
            if recency >= 2:
                reasons.append("近年文献")
        if len(paper.providers) > 1:
            score += 1.0
            reasons.append("多源交叉命中")
        if paper.doi or paper.arxiv_id:
            score += 0.5
        paper.score = round(score, 2)
        paper.score_reasons = reasons or ["来源相关度排序命中"]
    return sorted(
        papers,
        key=lambda paper: (paper.score, paper.year or 0, paper.title.casefold()),
        reverse=True,
    )


def load_queries(preset_path: Path, preset: str | None, custom_queries: Sequence[str]) -> list[SearchQuery]:
    queries: list[SearchQuery] = []
    if preset:
        try:
            payload = json.loads(preset_path.read_text(encoding="utf-8-sig"))
        except FileNotFoundError as exc:
            raise PaperSearchError(f"未找到主题预设文件：{preset_path}") from exc
        except json.JSONDecodeError as exc:
            raise PaperSearchError(f"主题预设 JSON 无法解析：{exc}") from exc
        if preset not in payload:
            choices = ", ".join(sorted(payload))
            raise PaperSearchError(f"未知主题预设 {preset!r}；可用值：{choices}")
        for item in payload[preset].get("queries", []):
            queries.append(SearchQuery(item["label"], item["arxiv"], item["openalex"]))
    for value in custom_queries:
        value = normalize_space(value)
        if value:
            escaped = value.replace('"', "")
            queries.append(SearchQuery(value, f'all:"{escaped}"', value))
    if not queries:
        raise PaperSearchError("至少需要一个 --preset 或 --query。")
    return queries


def run_search(
    queries: Sequence[SearchQuery],
    providers: Sequence[str],
    limit: int,
    since_year: int | None,
    provider_keys: Mapping[str, str],
    cache: ResponseCache,
    session: Session | None = None,
) -> SearchOutcome:
    session = session or build_session()
    papers: list[Paper] = []
    errors: list[str] = []
    counts = {provider: 0 for provider in providers}
    last_arxiv_request = [0.0]
    for query in queries:
        for provider in providers:
            try:
                if provider != "arxiv" and not provider_keys.get(provider):
                    raise ProviderError(f"{provider} 未配置 API key")
                if provider == "arxiv":
                    found = search_arxiv(session, cache, query, limit, last_arxiv_request)
                elif provider == "openalex":
                    found = search_openalex(
                        session, cache, query, limit, provider_keys.get("openalex", ""), since_year
                    )
                elif provider == "tavily":
                    found = search_tavily(
                        session, cache, query, limit, provider_keys.get("tavily", "")
                    )
                elif provider == "serpapi":
                    found = search_serpapi(
                        session, cache, query, limit, provider_keys.get("serpapi", ""), since_year
                    )
                else:  # protected by argparse, useful to API callers
                    raise ProviderError(f"不支持的来源：{provider}")
                counts[provider] += len(found)
                papers.extend(found)
            except (ProviderError, requests.RequestException, OSError) as exc:
                errors.append(f"{provider} / {query.label}: {exc}")
    ranked = rank_papers(deduplicate(papers), queries)
    return SearchOutcome(ranked, errors, counts, cache.hits)


def paper_identity(paper: Paper) -> str:
    return paper.doi or paper.arxiv_id or normalize_title(paper.title) or "paper"


def candidate_id(paper: Paper) -> str:
    """Return a stable, non-secret identifier shared by discovery runs."""

    identity = paper_identity(paper).casefold().strip()
    return hashlib.sha256(identity.encode("utf-8")).hexdigest()[:20]


def previously_seen_identities(output_root: Path) -> set[str]:
    """Read earlier discovery manifests without treating them as canonical facts."""

    seen: set[str] = set()
    if not output_root.exists():
        return seen
    for manifest in output_root.glob("search-*/results.json"):
        try:
            payload = json.loads(manifest.read_text(encoding="utf-8-sig"))
        except (OSError, json.JSONDecodeError, UnicodeDecodeError):
            continue
        for item in payload.get("papers", []) or []:
            if not isinstance(item, dict):
                continue
            paper = Paper(
                title=str(item.get("title", "")),
                doi=normalize_doi(str(item.get("doi", ""))),
                arxiv_id=normalize_arxiv_id(str(item.get("arxiv_id", ""))),
            )
            seen.add(paper_identity(paper))
    return seen


def download_pdf(session: Session, url: str, destination: Path) -> None:
    parsed = urlparse(url)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise PaperSearchError(f"不安全的 PDF URL：{url}")
    # No API key or authorization header is attached to third-party PDF URLs.
    with session.get(url, stream=True, timeout=(15, 120), headers={"Accept": "application/pdf"}) as response:
        if response.status_code >= 400:
            raise PaperSearchError(f"PDF 下载返回 HTTP {response.status_code}")
        declared = response.headers.get("Content-Length", "")
        if declared.isdigit() and int(declared) > MAX_PDF_BYTES:
            raise PaperSearchError("PDF 超过 200MB 安全上限")
        destination.parent.mkdir(parents=True, exist_ok=True)
        temp = destination.with_name(destination.name + ".tmp")
        total = 0
        first = b""
        try:
            with temp.open("wb") as handle:
                for chunk in response.iter_content(1024 * 1024):
                    if not chunk:
                        continue
                    if not first:
                        first = chunk[:5]
                    total += len(chunk)
                    if total > MAX_PDF_BYTES:
                        raise PaperSearchError("PDF 超过 200MB 安全上限")
                    handle.write(chunk)
            content_type = response.headers.get("Content-Type", "").casefold()
            if not first.startswith(b"%PDF") and "application/pdf" not in content_type:
                raise PaperSearchError("下载内容不是 PDF")
            temp.replace(destination)
        except Exception:
            temp.unlink(missing_ok=True)
            raise


def download_open_pdfs(
    papers: Sequence[Paper], run_dir: Path, limit: int, session: Session | None = None
) -> tuple[int, list[str]]:
    session = session or build_session()
    downloaded = 0
    errors: list[str] = []
    for rank, paper in enumerate(papers, start=1):
        if downloaded >= limit:
            break
        if not paper.pdf_url or paper.is_open_access is False:
            continue
        folder = run_dir / "papers" / f"{rank:03d}-{safe_component(paper.title)}"
        destination = folder / "paper.pdf"
        try:
            download_pdf(session, paper.pdf_url, destination)
            paper.local_pdf = destination.relative_to(PROJECT_ROOT).as_posix()
            metadata = asdict(paper)
            metadata.update({"discovery_status": "candidate", "retrieved_at": utc_now()})
            atomic_write_json(folder / "metadata.json", metadata)
            downloaded += 1
        except (PaperSearchError, requests.RequestException, OSError) as exc:
            errors.append(f"{paper.title}: {exc}")
    return downloaded, errors


def markdown_escape(value: str) -> str:
    return value.replace("|", "\\|").replace("\n", " ")


def render_report(
    papers: Sequence[Paper],
    queries: Sequence[SearchQuery],
    providers: Sequence[str],
    outcome: SearchOutcome,
    retrieved_at: str,
) -> str:
    lines = [
        "# 论文自动发现候选报告",
        "",
        "> **边界声明：** 本报告只是外部检索产生的 triage 候选，不是 `raw/canonical`，",
        "> 未经人工确认与 A 编译不得作为 wiki 硬事实，也不代表完整的全球查新。",
        "",
        f"- 抓取时间（UTC）：`{retrieved_at}`",
        f"- 来源：`{', '.join(providers)}`",
        f"- 原始命中：`{sum(outcome.provider_counts.values())}`；去重后：`{len(papers)}`",
        f"- 缓存命中：`{outcome.cache_hits}`",
        "- 排序：标题/摘要词项命中 + 轻量时间加分；不是语义相关性或质量判定",
        "",
        "## 检索主题",
        "",
    ]
    lines.extend(f"- {query.label}" for query in queries)
    if outcome.errors:
        lines.extend(["", "## 来源告警", ""])
        lines.extend(f"- {error}" for error in outcome.errors)
    lines.extend(["", "## 候选列表", ""])
    for rank, paper in enumerate(papers, start=1):
        lines.extend(
            [
                f"### {rank}. {paper.title or 'Untitled'}",
                "",
                f"- 作者：{', '.join(paper.authors) if paper.authors else '未提供'}",
                f"- 日期：{paper.published_date or paper.year or '未提供'}",
                f"- 来源：{', '.join(paper.providers)}",
                f"- 筛选状态：`{paper.triage_status}`；人工选择：`{str(paper.selected_by_user).lower()}`",
                f"- DOI：{paper.doi or '—'}；arXiv：{paper.arxiv_id or '—'}",
                f"- 开放获取：{paper.is_open_access if paper.is_open_access is not None else '未知'}；许可：{paper.license or '未提供'}",
                f"- 相关度分数：{paper.score:.2f}（{'；'.join(paper.score_reasons)}）",
                f"- 命中主题：{', '.join(paper.matched_queries)}",
                f"- 页面：{paper.landing_url or '—'}",
                f"- PDF：{paper.pdf_url or '—'}",
                f"- 本地 PDF：{paper.local_pdf or '未下载'}",
                "",
                paper.abstract or "_来源未提供摘要。_",
                "",
            ]
        )
    lines.extend(
        [
            "## 下一步",
            "",
            "1. 用 `tools/paper-triage.ps1 <results.json> --select <序号>` 标记真正相关的候选；",
            "2. 从 `raw/inbox/auto-discovered/papers/` 选择项晋升 `raw/canonical/`；",
            "3. 用 MinerU 生成 Markdown，并保留 provenance；",
            "4. 按 `schema/agent-a-compile.md` 执行 A 编译，再更新 Graphify。",
            "",
        ]
    )
    return "\n".join(lines)


def save_run(
    outcome: SearchOutcome,
    queries: Sequence[SearchQuery],
    providers: Sequence[str],
    output_root: Path,
    top: int,
    download: bool,
    download_limit: int,
) -> tuple[Path, int, list[str]]:
    stamp = datetime.now().astimezone().strftime("%Y%m%d-%H%M%S")
    run_dir = output_root.expanduser().resolve() / f"search-{stamp}"
    run_dir.mkdir(parents=True, exist_ok=False)
    papers = outcome.papers[:top]
    acquired_at = utc_now()
    discovery_run = display_path(run_dir)
    for paper in papers:
        paper.candidate_id = candidate_id(paper)
        paper.acquisition_method = "auto_discovery"
        paper.discovered_via = list(paper.providers)
        paper.discovery_run = discovery_run
        paper.triage_status = "pending"
        paper.selected_by_user = False
        paper.acquired_at = acquired_at
        paper.selected_at = ""
        paper.canonicalized_at = ""
    downloaded = 0
    download_errors: list[str] = []
    if download:
        downloaded, download_errors = download_open_pdfs(papers, run_dir, download_limit)
        outcome.errors.extend(f"PDF / {error}" for error in download_errors)
    retrieved_at = acquired_at
    payload = {
        "kind": "paper_discovery_candidates",
        "discovery_status": "candidate",
        "acquisition_method": "auto_discovery",
        "triage_counts": {"pending": len(papers), "selected": 0, "rejected": 0, "promoted": 0},
        "retrieved_at": retrieved_at,
        "boundary": "not canonical; not wiki evidence; human triage required",
        "providers": list(providers),
        "queries": [asdict(query) for query in queries],
        "provider_counts": outcome.provider_counts,
        "cache_hits": outcome.cache_hits,
        "errors": outcome.errors,
        "papers": [asdict(paper) for paper in papers],
    }
    atomic_write_json(run_dir / "results.json", payload)
    atomic_write_text(
        run_dir / "README.md",
        render_report(papers, queries, providers, outcome, retrieved_at),
    )
    return run_dir, downloaded, download_errors


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="自动搜索相关领域论文，结果仅进入 raw/inbox/auto-discovered 待人工筛选。",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("--preset", default="wireless-charging-scheduling", help="主题预设名称")
    parser.add_argument("--no-preset", action="store_true", help="禁用默认主题，只使用 --query")
    parser.add_argument("--preset-file", type=Path, default=DEFAULT_PRESETS)
    parser.add_argument("--query", action="append", default=[], help="追加自定义检索词，可重复")
    parser.add_argument(
        "--provider",
        action="append",
        choices=("arxiv", "openalex", "tavily", "serpapi"),
        help="检索源，可重复；默认启用 arXiv 和已配置 Key 的来源",
    )
    parser.add_argument("--key-file", type=Path, default=DEFAULT_KEY_FILE, help="带标签的共享 Key 配置文件")
    parser.add_argument("--openalex-key-file", type=Path, help="仅包含 OpenAlex Key 的文件")
    parser.add_argument("--tavily-key-file", type=Path, help="仅包含 Tavily Key 的文件")
    parser.add_argument("--serpapi-key-file", type=Path, help="仅包含 SerpApi Key 的文件")
    parser.add_argument("--limit", type=int, default=20, help="每个主题、每个来源最多返回数")
    parser.add_argument("--top", type=int, default=50, help="去重排序后写入报告的候选数")
    parser.add_argument("--since-year", type=int, default=2015, help="OpenAlex 最早发表年份；arXiv 结果在归一化后统一过滤")
    parser.add_argument("--output-root", type=Path, default=DEFAULT_OUTPUT_ROOT)
    parser.add_argument("--cache-root", type=Path, default=DEFAULT_CACHE_ROOT, help="远端响应缓存目录")
    parser.add_argument("--cache-hours", type=int, default=24, help="同一请求缓存小时数")
    parser.add_argument("--download", action="store_true", help="显式下载有开放 PDF URL 的候选")
    parser.add_argument("--download-limit", type=int, default=10, help="最多下载的开放 PDF 数")
    parser.add_argument("--new-only", action="store_true", help="与历史 search-*/results.json 对比，只报告未见候选")
    parser.add_argument("--no-save", action="store_true", help="执行真实检索但不生成报告或下载")
    parser.add_argument("--dry-run", action="store_true", help="只显示检索计划，不联网、不写文件")
    return parser


def validate_args(args: argparse.Namespace) -> None:
    if not 1 <= args.limit <= 100:
        raise PaperSearchError("--limit 必须在 1 到 100 之间。")
    if args.top <= 0 or args.download_limit <= 0:
        raise PaperSearchError("--top 与 --download-limit 必须大于 0。")
    if args.cache_hours < 0:
        raise PaperSearchError("--cache-hours 不能小于 0。")
    if args.since_year and not 1900 <= args.since_year <= datetime.now().year + 1:
        raise PaperSearchError("--since-year 不在合理范围内。")
    if args.no_save and args.download:
        raise PaperSearchError("--no-save 与 --download 不能同时使用。")


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        validate_args(args)
        queries = load_queries(args.preset_file, None if args.no_preset else (args.preset or None), args.query)
        provider_paths = {
            "openalex": args.openalex_key_file,
            "tavily": args.tavily_key_file,
            "serpapi": args.serpapi_key_file,
        }
        provider_keys = {
            provider: read_provider_key(provider, args.key_file, provider_paths[provider])
            for provider in PROVIDER_ENV_KEYS
        }
        providers = args.provider or [
            "arxiv",
            *(provider for provider in ("openalex", "tavily", "serpapi") if provider_keys[provider]),
        ]
        providers = list(dict.fromkeys(providers))
        for provider in providers:
            if provider != "arxiv" and not provider_keys.get(provider):
                env_name = PROVIDER_ENV_KEYS[provider]
                raise PaperSearchError(
                    f"已指定 {provider}，但未在共享 Key 文件、专用 Key 文件或 {env_name} 中找到 Key。"
                )

        print(f"检索主题：{len(queries)}；来源：{', '.join(providers)}")
        for query in queries:
            print(f"  - {query.label}")
        if args.dry_run:
            print("dry-run 完成：未联网、未写入 raw/inbox/auto-discovered。")
            return 0

        cache = ResponseCache(args.cache_root, args.cache_hours)
        outcome = run_search(queries, providers, args.limit, args.since_year, provider_keys, cache)
        if args.since_year:
            outcome.papers = [paper for paper in outcome.papers if not paper.year or paper.year >= args.since_year]
        if args.new_only:
            seen = previously_seen_identities(args.output_root)
            before = len(outcome.papers)
            outcome.papers = [paper for paper in outcome.papers if paper_identity(paper) not in seen]
            print(f"历史去重：已见 {before - len(outcome.papers)}，新增 {len(outcome.papers)}。")
        print(
            f"检索完成：原始 {sum(outcome.provider_counts.values())}，"
            f"去重并按年份过滤后 {len(outcome.papers)}，缓存命中 {outcome.cache_hits}。"
        )
        print(
            "来源命中："
            + "，".join(f"{provider}={outcome.provider_counts.get(provider, 0)}" for provider in providers)
        )
        for error in outcome.errors:
            print(f"告警：{error}", file=sys.stderr)
        if args.new_only and not outcome.papers:
            print("本次没有未见候选，不生成空报告。")
            return 0
        if args.no_save:
            for index, paper in enumerate(outcome.papers[: args.top], start=1):
                print(f"{index:02d}. [{paper.score:.2f}] {paper.title}")
            return 0 if outcome.papers else 1

        run_dir, downloaded, download_errors = save_run(
            outcome,
            queries,
            providers,
            args.output_root,
            args.top,
            args.download,
            args.download_limit,
        )
        print(f"候选报告：{run_dir / 'README.md'}")
        print(f"结构化结果：{run_dir / 'results.json'}")
        if args.download:
            print(f"开放 PDF：成功 {downloaded}，失败 {len(download_errors)}")
        print("下一步：人工筛选后再移入 raw/canonical；不会自动写 wiki 或更新 Graphify。")
        return 0 if outcome.papers else 1
    except KeyboardInterrupt:
        print("已中断。", file=sys.stderr)
        return 130
    except (PaperSearchError, requests.RequestException, OSError) as exc:
        print(f"错误：{exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
