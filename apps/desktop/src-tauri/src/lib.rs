use rfd::FileDialog;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tauri::ipc::Channel;
use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;
use walkdir::WalkDir;

mod codex_subscription;
mod compile_center;
mod literature_ingest;
mod process_support;
mod qa;
mod repository_watcher;
mod research_trail;
mod search_credentials;

#[derive(Default)]
struct RepositoryState {
    root: Option<PathBuf>,
    db: Option<Connection>,
    indexed_pages: usize,
}

#[derive(Default)]
struct AppState {
    repository: Mutex<RepositoryState>,
    repository_watcher: Mutex<Option<repository_watcher::RepositoryWatcher>>,
    cancellations: Mutex<HashMap<String, Arc<AtomicBool>>>,
    compile_cancellations: Mutex<HashMap<String, Arc<AtomicBool>>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RepositoryWatchStatus {
    active: bool,
    root: Option<String>,
    processed_changes: usize,
    full_rebuild: bool,
    graph_refresh: bool,
    pending_changes: usize,
    retry_attempt: u32,
    blocked: bool,
    last_error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RepositoryInfo {
    path: String,
    page_count: usize,
    indexed: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct IndexStats {
    path: String,
    page_count: usize,
    source_count: usize,
    method_count: usize,
    synthesis_count: usize,
    chapter_count: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    id: String,
    page_type: String,
    title: String,
    year: String,
    summary: String,
    source_path: String,
    snippet: String,
    score: f64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PageSummary {
    id: String,
    page_type: String,
    title: String,
    year: String,
    summary: String,
    source_path: String,
    modified_at: String,
    status: String,
    epistemic: String,
    method_family: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PageDetail {
    id: String,
    page_type: String,
    title: String,
    year: String,
    summary: String,
    body: String,
    source_path: String,
    modified_at: String,
    frontmatter: HashMap<String, String>,
    links: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LinkResolution {
    target: String,
    anchor: String,
    resolved: bool,
    page: Option<PageSummary>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Backlink {
    source: PageSummary,
    target: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PageFilters {
    page_type: Option<String>,
    query: Option<String>,
    year: Option<String>,
    status: Option<String>,
    method_family: Option<String>,
    sort: Option<String>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct BookSummary {
    id: String,
    title: String,
    year: String,
    page_count: usize,
    chapter_count: usize,
    source_path: String,
    pdf_path: String,
    quality_status: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct BookChapter {
    id: String,
    book_id: String,
    chapter_number: i64,
    title: String,
    markdown_path: String,
    pdf_path: String,
    physical_page_start: Option<i64>,
    physical_page_end: Option<i64>,
    printed_page_start: Option<i64>,
    printed_page_end: Option<i64>,
    char_count: usize,
    ingest_status: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct BookSearchResult {
    chapter: BookChapter,
    snippet: String,
    score: f64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct GraphNode {
    id: String,
    label: String,
    node_type: String,
    source_file: String,
    source_location: String,
    community: Option<i64>,
    origin: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct GraphEdge {
    source: String,
    target: String,
    relation: String,
    confidence: String,
    weight: f64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct GraphOverview {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    node_count: usize,
    edge_count: usize,
    community_count: usize,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct GraphFilters {
    query: Option<String>,
    node_type: Option<String>,
    community: Option<i64>,
    limit: Option<usize>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ComparisonCell {
    value: String,
    source_path: String,
    field: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ComparisonColumn {
    id: String,
    title: String,
    page_type: String,
    cells: HashMap<String, ComparisonCell>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ComparisonMatrix {
    fields: Vec<String>,
    columns: Vec<ComparisonColumn>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct BookChapterDetail {
    chapter: BookChapter,
    body: String,
}

fn validate_repository(path: &Path) -> Result<(), String> {
    if !path.is_dir() {
        return Err("知识库路径不是文件夹".to_string());
    }
    for required in ["AGENTS.md", "wiki", "schema"] {
        if !path.join(required).exists() {
            return Err(format!("知识库缺少必要入口：{required}"));
        }
    }
    Ok(())
}

fn db_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
      PRAGMA foreign_keys = ON;
      CREATE TABLE IF NOT EXISTS pages (
        id TEXT PRIMARY KEY,
        page_type TEXT NOT NULL,
        title TEXT NOT NULL,
        year TEXT NOT NULL DEFAULT '',
        summary TEXT NOT NULL DEFAULT '',
        body TEXT NOT NULL,
        source_path TEXT NOT NULL,
        modified_at TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT '',
        epistemic TEXT NOT NULL DEFAULT '',
        method_family TEXT NOT NULL DEFAULT '',
        scenario TEXT NOT NULL DEFAULT '',
        objectives TEXT NOT NULL DEFAULT '',
        constraints TEXT NOT NULL DEFAULT '',
        frontmatter TEXT NOT NULL DEFAULT '{}'
      );
      CREATE VIRTUAL TABLE IF NOT EXISTS pages_fts USING fts5(
        page_id UNINDEXED,
        title,
        body,
        keywords
      );
      CREATE TABLE IF NOT EXISTS paper_sections (
        id TEXT PRIMARY KEY,
        page_id TEXT NOT NULL,
        title TEXT NOT NULL,
        section_title TEXT NOT NULL,
        source_path TEXT NOT NULL,
        pdf_path TEXT NOT NULL DEFAULT '',
        line_start INTEGER NOT NULL,
        line_end INTEGER NOT NULL,
        body TEXT NOT NULL
      );
      CREATE VIRTUAL TABLE IF NOT EXISTS paper_sections_fts USING fts5(
        section_id UNINDEXED,
        title,
        section_title,
        body
      );
      CREATE TABLE IF NOT EXISTS wikilinks (
        source_id TEXT NOT NULL,
        target TEXT NOT NULL,
        UNIQUE(source_id, target)
      );
      CREATE INDEX IF NOT EXISTS idx_pages_type ON pages(page_type);
      CREATE INDEX IF NOT EXISTS idx_pages_year ON pages(year);
      CREATE INDEX IF NOT EXISTS idx_wikilinks_target ON wikilinks(target);
      CREATE INDEX IF NOT EXISTS idx_paper_sections_page ON paper_sections(page_id);
      CREATE TABLE IF NOT EXISTS books (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        year TEXT NOT NULL DEFAULT '',
        page_count INTEGER NOT NULL DEFAULT 0,
        chapter_count INTEGER NOT NULL DEFAULT 0,
        source_path TEXT NOT NULL DEFAULT '',
        pdf_path TEXT NOT NULL DEFAULT '',
        quality_status TEXT NOT NULL DEFAULT ''
      );
      CREATE TABLE IF NOT EXISTS book_chapters (
        id TEXT PRIMARY KEY,
        book_id TEXT NOT NULL,
        chapter_number INTEGER NOT NULL DEFAULT 0,
        title TEXT NOT NULL,
        markdown_path TEXT NOT NULL,
        pdf_path TEXT NOT NULL,
        physical_page_start INTEGER,
        physical_page_end INTEGER,
        printed_page_start INTEGER,
        printed_page_end INTEGER,
        char_count INTEGER NOT NULL DEFAULT 0,
        ingest_status TEXT NOT NULL DEFAULT ''
      );
      CREATE VIRTUAL TABLE IF NOT EXISTS book_chapters_fts USING fts5(
         chapter_id UNINDEXED,
         title,
         body
       );
       CREATE TABLE IF NOT EXISTS repository_metadata (
         key TEXT PRIMARY KEY,
         value TEXT NOT NULL
       );
       ",
        )
        .map_err(|error| format!("初始化SQLite失败：{error}"))?;
    // Older 0.2.x caches were created before the detail/filter columns existed.
    // Migrate them in place so an existing user database remains usable.
    for (name, definition) in [
        ("status", "TEXT NOT NULL DEFAULT ''"),
        ("epistemic", "TEXT NOT NULL DEFAULT ''"),
        ("method_family", "TEXT NOT NULL DEFAULT ''"),
        ("scenario", "TEXT NOT NULL DEFAULT ''"),
        ("objectives", "TEXT NOT NULL DEFAULT ''"),
        ("constraints", "TEXT NOT NULL DEFAULT ''"),
        ("frontmatter", "TEXT NOT NULL DEFAULT '{}'"),
    ] {
        let present = connection
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM pragma_table_info('pages') WHERE name = ?1)",
                [name],
                |row| row.get::<_, i64>(0),
            )
            .map_err(|error| format!("检查SQLite字段失败：{error}"))?;
        if present == 0 {
            connection
                .execute(
                    &format!("ALTER TABLE pages ADD COLUMN {name} {definition}"),
                    [],
                )
                .map_err(|error| format!("升级SQLite字段 {name} 失败：{error}"))?;
        }
    }
    connection
        .execute(
            "CREATE INDEX IF NOT EXISTS idx_pages_method_family ON pages(method_family)",
            [],
        )
        .map_err(|error| format!("创建方法族索引失败：{error}"))?;
    qa::db_schema(connection)?;
    compile_center::db_schema(connection)?;
    literature_ingest::db_schema(connection)?;
    Ok(())
}

fn parse_frontmatter(content: &str) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);
    if !normalized.starts_with("---") {
        return fields;
    }
    let mut lines = normalized.lines();
    let _ = lines.next();
    for line in lines {
        if line.trim() == "---" {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            fields.insert(
                key.trim().to_string(),
                value
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string(),
            );
        }
    }
    fields
}

fn body_without_frontmatter(content: &str) -> &str {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);
    if !normalized.starts_with("---") {
        return normalized;
    }
    if let Some(end) = normalized[3..].find("\n---") {
        return normalized[(end + 7)..].trim_start_matches(['\r', '\n', ' ']);
    }
    normalized
}

fn fallback_title(body: &str, path: &Path) -> String {
    body.lines()
        .find_map(|line| {
            line.strip_prefix("# ")
                .map(|value| value.trim().to_string())
        })
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| {
            path.file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("未命名页面")
                .to_string()
        })
}

fn extract_links(body: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("[[") {
        let after_start = &rest[(start + 2)..];
        let Some(end) = after_start.find("]]") else {
            break;
        };
        let target = after_start[..end].split('|').next().unwrap_or("").trim();
        if !target.is_empty() && !links.iter().any(|item: &String| item == target) {
            links.push(target.to_string());
        }
        rest = &after_start[(end + 2)..];
    }
    links
}

#[derive(Debug, Clone)]
struct PaperSectionChunk {
    section_title: String,
    line_start: usize,
    line_end: usize,
    body: String,
}

const PAPER_SECTION_MAX_CHARS: usize = 6_000;

fn markdown_heading(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let marker_len = trimmed.chars().take_while(|value| *value == '#').count();
    if !(1..=4).contains(&marker_len) {
        return None;
    }
    let remainder = trimmed.get(marker_len..)?.strip_prefix(' ')?;
    let title = remainder.trim().trim_end_matches('#').trim();
    (!title.is_empty()).then(|| title.to_string())
}

fn push_paper_section_chunks(
    output: &mut Vec<PaperSectionChunk>,
    section_title: &str,
    lines: &[(usize, String)],
) {
    let meaningful = lines.iter().any(|(_, line)| !line.trim().is_empty());
    if !meaningful {
        return;
    }
    let mut current: Vec<(usize, String)> = Vec::new();
    let mut current_chars = 0usize;
    for (line_number, line) in lines {
        let line_chars = line.chars().count() + 1;
        if !current.is_empty()
            && current_chars + line_chars > PAPER_SECTION_MAX_CHARS
            && (line.trim().is_empty() || current_chars >= PAPER_SECTION_MAX_CHARS)
        {
            let body = current
                .iter()
                .map(|(_, value)| value.as_str())
                .collect::<Vec<_>>()
                .join("\n")
                .trim()
                .to_string();
            if !body.is_empty() {
                output.push(PaperSectionChunk {
                    section_title: section_title.to_string(),
                    line_start: current.first().map(|item| item.0).unwrap_or(1),
                    line_end: current.last().map(|item| item.0).unwrap_or(1),
                    body,
                });
            }
            current.clear();
            current_chars = 0;
        }
        current.push((*line_number, line.clone()));
        current_chars += line_chars;
    }
    let body = current
        .iter()
        .map(|(_, value)| value.as_str())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();
    if !body.is_empty() {
        output.push(PaperSectionChunk {
            section_title: section_title.to_string(),
            line_start: current.first().map(|item| item.0).unwrap_or(1),
            line_end: current.last().map(|item| item.0).unwrap_or(1),
            body,
        });
    }
}

fn split_paper_markdown(content: &str, fallback_title: &str) -> Vec<PaperSectionChunk> {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut output = Vec::new();
    let mut section_title = fallback_title.to_string();
    let mut section_lines: Vec<(usize, String)> = Vec::new();
    let mut in_frontmatter = normalized.starts_with("---");

    for (index, line) in normalized.lines().enumerate() {
        let line_number = index + 1;
        if in_frontmatter {
            if line_number > 1 && line.trim() == "---" {
                in_frontmatter = false;
            }
            continue;
        }
        if let Some(heading) = markdown_heading(line) {
            push_paper_section_chunks(&mut output, &section_title, &section_lines);
            section_title = heading;
            section_lines.clear();
        }
        section_lines.push((line_number, line.to_string()));
    }
    push_paper_section_chunks(&mut output, &section_title, &section_lines);
    output
}

fn repository_raw_file(root: &Path, relative: &str) -> Option<PathBuf> {
    let raw_root = root.join("raw/canonical").canonicalize().ok()?;
    let candidate = root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let canonical = candidate.canonicalize().ok()?;
    (canonical.is_file() && canonical.starts_with(raw_root)).then_some(canonical)
}

fn index_paper_sections(
    connection: &Connection,
    root: &Path,
    page_id: &str,
    title: &str,
    fields: &HashMap<String, String>,
) -> Result<(), String> {
    if field_value(fields, "source_type") == "book" {
        return Ok(());
    }
    let raw_md = field_value(fields, "raw_md");
    let Some(raw_path) = repository_raw_file(root, &raw_md) else {
        return Ok(());
    };
    let content = fs::read_to_string(&raw_path)
        .map_err(|error| format!("读取论文原文 {} 失败：{error}", raw_path.display()))?;
    let pdf_path = repository_raw_file(root, &field_value(fields, "pdf_path"))
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_default();
    for (index, chunk) in split_paper_markdown(&content, title)
        .into_iter()
        .enumerate()
    {
        let id = format!("{page_id}#{}", index + 1);
        connection.execute(
            "INSERT INTO paper_sections (id,page_id,title,section_title,source_path,pdf_path,line_start,line_end,body) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![id,page_id,title,chunk.section_title,raw_path.to_string_lossy(),pdf_path,chunk.line_start as i64,chunk.line_end as i64,chunk.body],
        ).map_err(|error| format!("写入论文分节索引失败：{error}"))?;
        connection.execute(
            "INSERT INTO paper_sections_fts (section_id,title,section_title,body) VALUES (?1,?2,?3,?4)",
            params![id,title,chunk.section_title,chunk.body],
        ).map_err(|error| format!("写入论文分节全文索引失败：{error}"))?;
    }
    Ok(())
}

fn page_id(wiki_root: &Path, path: &Path) -> String {
    path.strip_prefix(wiki_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
        .trim_end_matches(".md")
        .to_string()
}

fn summary_from_body(body: &str) -> String {
    body.lines()
        .map(str::trim)
        .find(|line| {
            !line.is_empty()
                && !line.starts_with('#')
                && !line.starts_with('>')
                && !line.starts_with('|')
        })
        .unwrap_or("")
        .chars()
        .take(180)
        .collect()
}

fn field_value(fields: &HashMap<String, String>, key: &str) -> String {
    fields.get(key).cloned().unwrap_or_default()
}

fn serialize_frontmatter(fields: &HashMap<String, String>) -> String {
    serde_json::to_string(fields).unwrap_or_else(|_| "{}".to_string())
}

fn summary_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<PageSummary> {
    Ok(PageSummary {
        id: row.get(0)?,
        page_type: row.get(1)?,
        title: row.get(2)?,
        year: row.get(3)?,
        summary: row.get(4)?,
        source_path: row.get(5)?,
        modified_at: row.get(6)?,
        status: row.get(7)?,
        epistemic: row.get(8)?,
        method_family: row.get(9)?,
    })
}

fn normalize_link_target(target: &str) -> (String, String) {
    let without_brackets = target
        .trim()
        .trim_start_matches("[[")
        .trim_end_matches("]]");
    let target_without_alias = without_brackets.split('|').next().unwrap_or("").trim();
    let mut parts = target_without_alias.splitn(2, '#');
    let page = parts.next().unwrap_or("").trim();
    let anchor = parts.next().unwrap_or("").trim().to_string();
    let replaced = page.replace('\\', "/");
    let trimmed = replaced.trim_start_matches("./").trim_start_matches('/');
    let without_wiki = trimmed.strip_prefix("wiki/").unwrap_or(trimmed);
    let normalized = without_wiki.trim_end_matches(".md").to_string();
    (normalized, anchor)
}

fn page_summary_by_id(connection: &Connection, id: &str) -> Result<Option<PageSummary>, String> {
    connection
    .query_row(
      "SELECT id,page_type,title,year,summary,source_path,modified_at,status,epistemic,method_family FROM pages WHERE id = ?1",
      [id],
      summary_from_row,
    )
    .optional()
    .map_err(|error| format!("读取页面摘要失败：{error}"))
}

fn resolve_page_summary(
    connection: &Connection,
    target: &str,
) -> Result<Option<PageSummary>, String> {
    let (normalized, _) = normalize_link_target(target);
    let exact = page_summary_by_id(connection, &normalized)?;
    if exact.is_some() {
        return Ok(exact);
    }
    connection
    .query_row(
      "SELECT id,page_type,title,year,summary,source_path,modified_at,status,epistemic,method_family FROM pages WHERE lower(title)=lower(?1) OR lower(id)=lower(?1) OR lower(replace(id,'.md',''))=lower(?1) LIMIT 1",
      [&normalized],
      summary_from_row,
    )
    .optional()
    .map_err(|error| format!("解析Wiki链接失败：{error}"))
}

fn delete_wiki_page_index(
    connection: &Connection,
    wiki_root: &Path,
    path: &Path,
) -> Result<(), String> {
    let id = page_id(wiki_root, path);
    connection
        .execute(
            "DELETE FROM paper_sections_fts WHERE section_id IN (SELECT id FROM paper_sections WHERE page_id=?1)",
            [&id],
        )
        .map_err(|error| format!("删除论文分节全文索引失败：{error}"))?;
    connection
        .execute("DELETE FROM paper_sections WHERE page_id=?1", [&id])
        .map_err(|error| format!("删除论文分节索引失败：{error}"))?;
    connection
        .execute("DELETE FROM pages_fts WHERE page_id=?1", [&id])
        .map_err(|error| format!("删除页面全文索引失败：{error}"))?;
    connection
        .execute("DELETE FROM wikilinks WHERE source_id=?1", [&id])
        .map_err(|error| format!("删除页面链接索引失败：{error}"))?;
    connection
        .execute(
            "DELETE FROM pages WHERE id=?1 OR source_path=?2",
            params![id, path.to_string_lossy()],
        )
        .map_err(|error| format!("删除页面索引失败：{error}"))?;
    Ok(())
}

fn upsert_wiki_page_index(
    connection: &Connection,
    wiki_root: &Path,
    path: &Path,
) -> Result<(String, String), String> {
    if !path.is_file() || path.extension().and_then(|value| value.to_str()) != Some("md") {
        return Err(format!(
            "增量索引目标不是 Markdown 文件：{}",
            path.display()
        ));
    }
    let content =
        fs::read_to_string(path).map_err(|error| format!("读取{}失败：{error}", path.display()))?;
    let fields = parse_frontmatter(&content);
    let body = body_without_frontmatter(&content).to_string();
    let id = page_id(wiki_root, path);
    let page_type = fields
        .get("type")
        .cloned()
        .unwrap_or_else(|| "page".to_string());
    let title = fields
        .get("title")
        .cloned()
        .unwrap_or_else(|| fallback_title(&body, path));
    let year = field_value(&fields, "year");
    let summary = fields
        .get("why_relevant")
        .filter(|value| !value.is_empty())
        .cloned()
        .unwrap_or_else(|| summary_from_body(&body));
    let modified_at = fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .map(|value| format!("{value:?}"))
        .unwrap_or_default();
    let status = field_value(&fields, "status");
    let epistemic = field_value(&fields, "epistemic");
    let method_family = field_value(&fields, "method_family");
    let scenario = field_value(&fields, "scenario");
    let objectives = field_value(&fields, "objectives");
    let constraints = field_value(&fields, "constraints");
    let frontmatter = serialize_frontmatter(&fields);
    let keywords = [
        fields.get("paper_keywords").cloned().unwrap_or_default(),
        method_family.clone(),
        scenario.clone(),
        objectives.clone(),
        constraints.clone(),
    ]
    .join(" ");

    delete_wiki_page_index(connection, wiki_root, path)?;
    connection.execute(
        "INSERT INTO pages (id,page_type,title,year,summary,body,source_path,modified_at,status,epistemic,method_family,scenario,objectives,constraints,frontmatter) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
        params![id,page_type,title,year,summary,body,path.to_string_lossy(),modified_at,status,epistemic,method_family,scenario,objectives,constraints,frontmatter],
    ).map_err(|error| format!("写入页面索引失败：{error}"))?;
    connection
        .execute(
            "INSERT INTO pages_fts (page_id,title,body,keywords) VALUES (?1,?2,?3,?4)",
            params![id, title, body, keywords],
        )
        .map_err(|error| format!("写入全文索引失败：{error}"))?;
    for target in extract_links(&body) {
        connection
            .execute(
                "INSERT OR IGNORE INTO wikilinks (source_id,target) VALUES (?1,?2)",
                params![id, target],
            )
            .map_err(|error| format!("写入链接索引失败：{error}"))?;
    }
    if page_type == "source" {
        let root = wiki_root
            .parent()
            .ok_or_else(|| "Wiki 目录缺少知识库父目录".to_string())?;
        index_paper_sections(connection, root, &id, &title, &fields)?;
    }
    Ok((id, page_type))
}

fn current_index_stats(connection: &Connection, root: &Path) -> Result<IndexStats, String> {
    let count = |sql: &str| {
        connection
            .query_row(sql, [], |row| row.get::<_, i64>(0))
            .map(|value| value.max(0) as usize)
            .map_err(|error| error.to_string())
    };
    Ok(IndexStats {
        path: root.to_string_lossy().to_string(),
        page_count: count("SELECT COUNT(*) FROM pages")?,
        source_count: count("SELECT COUNT(*) FROM pages WHERE page_type='source'")?,
        method_count: count("SELECT COUNT(*) FROM pages WHERE page_type='method'")?,
        synthesis_count: count("SELECT COUNT(*) FROM pages WHERE page_type='synthesis'")?,
        chapter_count: count("SELECT COUNT(*) FROM book_chapters")?,
    })
}

const REPOSITORY_IDENTITY_KEY: &str = "knowledge_index_repository_id";
const KNOWLEDGE_INDEX_SCHEMA_KEY: &str = "knowledge_index_schema_version";
const KNOWLEDGE_INDEX_SCHEMA_VERSION: &str = "2";

fn canonical_repository_root(root: &Path) -> Result<PathBuf, String> {
    root.canonicalize()
        .map_err(|error| format!("解析知识库路径失败：{} ({error})", root.display()))
}

fn repository_identity(root: &Path) -> String {
    let value = root.to_string_lossy().replace('\\', "/");
    let value = value.trim_end_matches('/');
    if cfg!(windows) {
        value.to_ascii_lowercase()
    } else {
        value.to_string()
    }
}

fn read_repository_identity(connection: &Connection) -> Result<Option<String>, String> {
    connection
        .query_row(
            "SELECT value FROM repository_metadata WHERE key=?1",
            [REPOSITORY_IDENTITY_KEY],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("读取知识库身份失败：{error}"))
}

fn read_index_schema_version(connection: &Connection) -> Result<Option<String>, String> {
    connection
        .query_row(
            "SELECT value FROM repository_metadata WHERE key=?1",
            [KNOWLEDGE_INDEX_SCHEMA_KEY],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("读取知识索引版本失败：{error}"))
}

fn write_repository_identity(connection: &Connection, identity: &str) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO repository_metadata(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![REPOSITORY_IDENTITY_KEY, identity],
        )
        .map_err(|error| format!("写入知识库身份失败：{error}"))?;
    Ok(())
}

fn write_index_schema_version(connection: &Connection) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO repository_metadata(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            params![KNOWLEDGE_INDEX_SCHEMA_KEY, KNOWLEDGE_INDEX_SCHEMA_VERSION],
        )
        .map_err(|error| format!("写入知识索引版本失败：{error}"))?;
    Ok(())
}

fn ensure_repository_index(connection: &mut Connection, root: &Path) -> Result<IndexStats, String> {
    let identity = repository_identity(root);
    let stored = read_repository_identity(connection)?;
    let schema_version = read_index_schema_version(connection)?;
    if stored.as_deref() == Some(identity.as_str())
        && schema_version.as_deref() == Some(KNOWLEDGE_INDEX_SCHEMA_VERSION)
    {
        return current_index_stats(connection, root);
    }

    // A missing or mismatched identity means the shared database may contain
    // derived rows from another repository. Rebuild only derived knowledge
    // tables; chat, compile and app-settings tables are intentionally kept.
    let stats = rebuild_connection(connection, root)?;
    write_repository_identity(connection, &identity)?;
    write_index_schema_version(connection)?;
    Ok(stats)
}

fn rebuild_connection(connection: &mut Connection, root: &Path) -> Result<IndexStats, String> {
    let wiki_root = root.join("wiki");
    let tx = connection
        .transaction()
        .map_err(|error| format!("开启索引事务失败：{error}"))?;
    tx.execute("DELETE FROM pages", [])
        .map_err(|error| format!("清理页面索引失败：{error}"))?;
    tx.execute("DELETE FROM pages_fts", [])
        .map_err(|error| format!("清理全文索引失败：{error}"))?;
    tx.execute("DELETE FROM paper_sections", [])
        .map_err(|error| format!("清理论文分节索引失败：{error}"))?;
    tx.execute("DELETE FROM paper_sections_fts", [])
        .map_err(|error| format!("清理论文分节全文索引失败：{error}"))?;
    tx.execute("DELETE FROM wikilinks", [])
        .map_err(|error| format!("清理链接索引失败：{error}"))?;
    tx.execute("DELETE FROM books", [])
        .map_err(|error| format!("清理专著索引失败：{error}"))?;
    tx.execute("DELETE FROM book_chapters", [])
        .map_err(|error| format!("清理章节索引失败：{error}"))?;
    tx.execute("DELETE FROM book_chapters_fts", [])
        .map_err(|error| format!("清理章节全文索引失败：{error}"))?;

    let mut stats = IndexStats {
        path: root.to_string_lossy().to_string(),
        page_count: 0,
        source_count: 0,
        method_count: 0,
        synthesis_count: 0,
        chapter_count: 0,
    };
    for entry in WalkDir::new(&wiki_root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !entry.file_type().is_file()
            || path.extension().and_then(|value| value.to_str()) != Some("md")
        {
            continue;
        }
        let (id, page_type) = upsert_wiki_page_index(&tx, &wiki_root, path)?;
        stats.page_count += 1;
        match page_type.as_str() {
            "source" => stats.source_count += 1,
            "method" => stats.method_count += 1,
            "synthesis" => stats.synthesis_count += 1,
            _ => {}
        }
        if id.contains("book") || id.contains("chapter") {
            stats.chapter_count += 1;
        }
    }
    stats.chapter_count = 0;
    for book_id in ["algorithmic-game-theory", "approximation-algorithms"] {
        let Ok(chapters) = book_chapters(root, book_id) else {
            continue;
        };
        let Some((title, year, page_count, pdf_relative)) = core_book_meta(book_id) else {
            continue;
        };
        let source_relative = if book_id == "algorithmic-game-theory" {
            "wiki/sources/src-book-algorithmic-game-theory.md"
        } else {
            "wiki/sources/src-book-approximation-algorithms.md"
        };
        tx.execute("INSERT OR REPLACE INTO books (id,title,year,page_count,chapter_count,source_path,pdf_path,quality_status) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)", params![book_id, title, year, page_count as i64, chapters.len() as i64, root.join(source_relative).to_string_lossy().to_string(), root.join(pdf_relative).to_string_lossy().to_string(), if root.join("raw/canonical/core-books-quality.json").is_file() { "pass" } else { "unknown" }]).map_err(|error| format!("写入专著索引失败：{error}"))?;
        for chapter in chapters {
            let body = fs::read_to_string(&chapter.markdown_path).unwrap_or_default();
            tx.execute("INSERT OR REPLACE INTO book_chapters (id,book_id,chapter_number,title,markdown_path,pdf_path,physical_page_start,physical_page_end,printed_page_start,printed_page_end,char_count,ingest_status) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)", params![chapter.id, chapter.book_id, chapter.chapter_number, chapter.title, chapter.markdown_path, chapter.pdf_path, chapter.physical_page_start, chapter.physical_page_end, chapter.printed_page_start, chapter.printed_page_end, chapter.char_count as i64, chapter.ingest_status]) .map_err(|error| format!("写入章节索引失败：{error}"))?;
            tx.execute("INSERT OR REPLACE INTO book_chapters_fts (chapter_id,title,body) VALUES (?1,?2,?3)", params![chapter.id, chapter.title, body]).map_err(|error| format!("写入章节全文索引失败：{error}"))?;
            stats.chapter_count += 1;
        }
    }
    tx.commit()
        .map_err(|error| format!("提交索引事务失败：{error}"))?;
    Ok(stats)
}

fn repository_db_path(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app
        .path()
        .app_local_data_dir()
        .map_err(|error| format!("获取客户端数据目录失败：{error}"))?;
    Ok(root.join("knowledge.db"))
}

fn open_repository_state(
    state: &mut RepositoryState,
    app: &AppHandle,
    root: PathBuf,
) -> Result<RepositoryInfo, String> {
    let root = canonical_repository_root(&root)?;
    validate_repository(&root)?;
    let db_path = repository_db_path(app)?;
    fs::create_dir_all(db_path.parent().unwrap_or(root.as_path()))
        .map_err(|error| format!("创建客户端缓存目录失败：{error}"))?;
    let connection =
        Connection::open(&db_path).map_err(|error| format!("打开SQLite失败：{error}"))?;
    db_schema(&connection)?;
    let mut connection = connection;
    let stats = ensure_repository_index(&mut connection, &root)?;
    let indexed_pages = stats.page_count;
    state.root = Some(root.clone());
    state.db = Some(connection);
    state.indexed_pages = indexed_pages;
    if let Ok(data_dir) = app.path().app_local_data_dir() {
        let _ = fs::create_dir_all(&data_dir);
        let _ = fs::write(
            data_dir.join("repository.json"),
            serde_json::json!({ "path": root.to_string_lossy() }).to_string(),
        );
    }
    Ok(RepositoryInfo {
        path: root.to_string_lossy().to_string(),
        page_count: indexed_pages,
        indexed: indexed_pages > 0,
    })
}

fn start_repository_watcher(state: &AppState, root: PathBuf) -> Result<(), String> {
    let watcher = repository_watcher::RepositoryWatcher::start(root)
        .map_err(|error| format!("启动知识库监听失败：{error}"))?;
    *state
        .repository_watcher
        .lock()
        .map_err(|_| "知识库监听状态锁定失败".to_string())? = Some(watcher);
    Ok(())
}

#[tauri::command]
fn choose_repository(app: AppHandle, state: State<'_, AppState>) -> Result<RepositoryInfo, String> {
    let Some(path) = FileDialog::new()
        .set_title("选择无线充电调度知识库")
        .pick_folder()
    else {
        return Err("用户取消了目录选择".to_string());
    };
    let info = {
        let mut repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        open_repository_state(&mut repository, &app, path.clone())?
    };
    start_repository_watcher(&state, path)?;
    Ok(info)
}

#[tauri::command]
fn open_repository(
    app: AppHandle,
    path: String,
    state: State<'_, AppState>,
) -> Result<RepositoryInfo, String> {
    let root = PathBuf::from(path);
    let info = {
        let mut repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        open_repository_state(&mut repository, &app, root.clone())?
    };
    start_repository_watcher(&state, root)?;
    Ok(info)
}

#[tauri::command]
fn get_repository_watch_status(
    state: State<'_, AppState>,
) -> Result<RepositoryWatchStatus, String> {
    let watcher = state
        .repository_watcher
        .lock()
        .map_err(|_| "知识库监听状态锁定失败".to_string())?;
    let status = watcher
        .as_ref()
        .map(|item| item.status())
        .unwrap_or_default();
    Ok(RepositoryWatchStatus {
        active: watcher.is_some(),
        root: watcher
            .as_ref()
            .map(|item| item.root().to_string_lossy().to_string()),
        processed_changes: 0,
        full_rebuild: false,
        graph_refresh: false,
        pending_changes: status.pending_changes,
        retry_attempt: status.retry_attempt,
        blocked: status.blocked,
        last_error: status.last_error,
    })
}

struct AppliedRepositoryChanges {
    stats: IndexStats,
}

fn apply_repository_changes(
    repository: &mut RepositoryState,
    changes: &[repository_watcher::IndexChange],
) -> Result<AppliedRepositoryChanges, String> {
    let root = repository
        .root
        .clone()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_mut()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    let full_rebuild = changes.iter().any(|change| change.full_rebuild);
    let stats = if full_rebuild {
        rebuild_connection(connection, &root)?
    } else {
        let wiki_root = root.join("wiki");
        let tx = connection
            .transaction()
            .map_err(|error| format!("开启增量索引事务失败：{error}"))?;
        for change in changes {
            if change.graph_refresh {
                continue;
            }
            let is_wiki = |path: &Path| {
                path.strip_prefix(&wiki_root).is_ok()
                    && path.extension().and_then(|value| value.to_str()) == Some("md")
            };
            match change.kind {
                repository_watcher::ChangeKind::Remove => {
                    if is_wiki(&change.path) {
                        delete_wiki_page_index(&tx, &wiki_root, &change.path)?;
                    }
                }
                repository_watcher::ChangeKind::Rename => {
                    if let Some(previous) =
                        change.previous_path.as_ref().filter(|path| is_wiki(path))
                    {
                        delete_wiki_page_index(&tx, &wiki_root, previous)?;
                    }
                    if is_wiki(&change.path) && change.path.is_file() {
                        upsert_wiki_page_index(&tx, &wiki_root, &change.path)?;
                    }
                }
                _ => {
                    if is_wiki(&change.path) && change.path.is_file() {
                        upsert_wiki_page_index(&tx, &wiki_root, &change.path)?;
                    }
                }
            }
        }
        tx.commit()
            .map_err(|error| format!("提交增量索引事务失败：{error}"))?;
        current_index_stats(connection, &root)?
    };
    repository.indexed_pages = stats.page_count;
    Ok(AppliedRepositoryChanges { stats })
}

#[tauri::command]
fn process_repository_changes(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<RepositoryWatchStatus, String> {
    let batch = {
        let mut watcher = state
            .repository_watcher
            .lock()
            .map_err(|_| "知识库监听状态锁定失败".to_string())?;
        watcher.as_mut().and_then(|item| item.begin_batch())
    };
    let Some(batch) = batch else {
        return get_repository_watch_status(state);
    };
    let changes = batch.changes.clone();

    let full_rebuild = changes.iter().any(|change| change.full_rebuild);
    let graph_refresh = changes.iter().any(|change| change.graph_refresh);
    let changed_files = changes
        .iter()
        .map(|change| {
            serde_json::json!({
                "path": change.path.to_string_lossy(),
                "previousPath": change.previous_path.as_ref().map(|path| path.to_string_lossy().to_string()),
                "kind": change.kind,
            })
        })
        .collect::<Vec<_>>();
    let _ = app.emit(
        "index_update_started",
        serde_json::json!({ "changeCount": changes.len(), "fullRebuild": full_rebuild }),
    );
    let applied = match state.repository.lock() {
        Ok(mut repository) => apply_repository_changes(&mut repository, &changes),
        Err(_) => Err("知识库状态锁定失败".to_string()),
    };
    let applied = match applied {
        Ok(value) => value,
        Err(error) => {
            let retry = state
                .repository_watcher
                .lock()
                .ok()
                .and_then(|mut watcher| watcher.as_mut()?.fail_batch(batch.id, error.clone()));
            let _ = app.emit(
                "index_update_failed",
                serde_json::json!({
                    "batchId": batch.id,
                    "retryAttempt": retry.as_ref().map(|item| item.retry_attempt).unwrap_or(batch.retry_attempt),
                    "blocked": retry.as_ref().map(|item| item.blocked).unwrap_or(false),
                    "changeCount": changes.len(),
                    "error": error,
                }),
            );
            return Err(error);
        }
    };
    if let Ok(mut watcher) = state.repository_watcher.lock() {
        if let Some(item) = watcher.as_mut() {
            item.ack_batch(batch.id);
        }
    }
    let stats = applied.stats;
    let _ = app.emit(
        "index_update_completed",
        serde_json::json!({
            "batchId": batch.id,
            "changeCount": changes.len(),
            "pageCount": stats.page_count,
            "fullRebuild": full_rebuild,
            "graphRefresh": graph_refresh,
            "changedFiles": changed_files,
        }),
    );
    if full_rebuild {
        let _ = app.emit("graph_rebuild_required", ());
    }
    let root = state
        .repository
        .lock()
        .ok()
        .and_then(|repository| repository.root.clone())
        .map(|path| path.to_string_lossy().to_string());
    Ok(RepositoryWatchStatus {
        active: true,
        root,
        processed_changes: changes.len(),
        full_rebuild,
        graph_refresh,
        pending_changes: 0,
        retry_attempt: 0,
        blocked: false,
        last_error: None,
    })
}

#[tauri::command]
fn rebuild_index(state: State<'_, AppState>) -> Result<IndexStats, String> {
    let mut repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .clone()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_mut()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    let stats = rebuild_connection(connection, &root)?;
    repository.indexed_pages = stats.page_count;
    if let Ok(mut watcher) = state.repository_watcher.lock() {
        if let Some(item) = watcher.as_mut() {
            item.clear_after_full_rebuild();
        }
    }
    Ok(stats)
}

#[tauri::command]
fn repository_info(state: State<'_, AppState>) -> Result<RepositoryInfo, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let path = repository
        .root
        .clone()
        .ok_or_else(|| "尚未选择知识库目录".to_string())?;
    Ok(RepositoryInfo {
        path: path.to_string_lossy().to_string(),
        page_count: repository.indexed_pages,
        indexed: repository.indexed_pages > 0,
    })
}

#[tauri::command]
fn search_pages(
    query: String,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    query_pages(connection, &query, limit.unwrap_or(20))
}

fn query_pages(
    connection: &Connection,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    let query = query.trim().to_string();
    if query.is_empty() {
        return Ok(Vec::new());
    }
    let limit = limit.clamp(1, 100) as i64;
    let fts_query = query
        .split_whitespace()
        .map(|term| format!("\"{}\"*", term.replace('"', "")))
        .collect::<Vec<_>>()
        .join(" AND ");
    let mut results = Vec::new();
    let mut statement = connection.prepare("SELECT p.id,p.page_type,p.title,p.year,p.summary,p.source_path,snippet(pages_fts,2,'<mark>','</mark>',' … ',24),bm25(pages_fts) FROM pages_fts JOIN pages p ON p.id=pages_fts.page_id WHERE pages_fts MATCH ?1 ORDER BY bm25(pages_fts) LIMIT ?2").map_err(|error| format!("准备搜索失败：{error}"))?;
    let rows = statement
        .query_map(params![fts_query, limit], |row| {
            Ok(SearchResult {
                id: row.get(0)?,
                page_type: row.get(1)?,
                title: row.get(2)?,
                year: row.get(3)?,
                summary: row.get(4)?,
                source_path: row.get(5)?,
                snippet: row.get(6)?,
                score: row.get::<_, f64>(7).unwrap_or(0.0),
            })
        })
        .map_err(|error| format!("执行搜索失败：{error}"))?;
    for row in rows {
        results.push(row.map_err(|error| format!("读取搜索结果失败：{error}"))?);
    }
    if results.is_empty() {
        let like = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let mut fallback = connection.prepare("SELECT id,page_type,title,year,summary,source_path,substr(body,1,220) FROM pages WHERE title LIKE ?1 ESCAPE '\\' OR body LIKE ?1 ESCAPE '\\' ORDER BY year DESC LIMIT ?2").map_err(|error| format!("准备中文搜索失败：{error}"))?;
        let rows = fallback
            .query_map(params![like, limit], |row| {
                Ok(SearchResult {
                    id: row.get(0)?,
                    page_type: row.get(1)?,
                    title: row.get(2)?,
                    year: row.get(3)?,
                    summary: row.get(4)?,
                    source_path: row.get(5)?,
                    snippet: row.get(6)?,
                    score: 0.0,
                })
            })
            .map_err(|error| format!("执行中文搜索失败：{error}"))?;
        for row in rows {
            results.push(row.map_err(|error| format!("读取中文搜索结果失败：{error}"))?);
        }
    }
    Ok(results)
}

#[tauri::command]
fn list_pages(
    filters: Option<PageFilters>,
    state: State<'_, AppState>,
) -> Result<Vec<PageSummary>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let filters = filters.unwrap_or_default();
    let mut sql = "SELECT id,page_type,title,year,summary,source_path,modified_at,status,epistemic,method_family FROM pages WHERE 1=1".to_string();
    let mut args: Vec<String> = Vec::new();
    if let Some(page_type) = filters.page_type.filter(|value| !value.trim().is_empty()) {
        sql.push_str(" AND page_type = ?");
        args.push(page_type);
    }
    if let Some(query) = filters.query.filter(|value| !value.trim().is_empty()) {
        sql.push_str(" AND (title LIKE ? ESCAPE '\\' OR summary LIKE ? ESCAPE '\\' OR body LIKE ? ESCAPE '\\')");
        let like = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        args.extend([like.clone(), like.clone(), like]);
    }
    if let Some(year) = filters.year.filter(|value| !value.trim().is_empty()) {
        sql.push_str(" AND year = ?");
        args.push(year);
    }
    if let Some(status) = filters.status.filter(|value| !value.trim().is_empty()) {
        sql.push_str(" AND status = ?");
        args.push(status);
    }
    if let Some(method_family) = filters
        .method_family
        .filter(|value| !value.trim().is_empty())
    {
        sql.push_str(" AND method_family LIKE ? ESCAPE '\\'");
        args.push(format!(
            "%{}%",
            method_family.replace('%', "\\%").replace('_', "\\_")
        ));
    }
    match filters.sort.as_deref() {
        Some("title") => sql.push_str(" ORDER BY title COLLATE NOCASE ASC"),
        Some("year_asc") => sql.push_str(" ORDER BY year ASC, title COLLATE NOCASE ASC"),
        Some("modified") => sql.push_str(" ORDER BY modified_at DESC, title COLLATE NOCASE ASC"),
        _ => sql.push_str(" ORDER BY year DESC, title COLLATE NOCASE ASC"),
    }
    let limit = filters.limit.unwrap_or(100).clamp(1, 500);
    sql.push_str(" LIMIT ?");
    args.push(limit.to_string());
    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| format!("准备页面列表失败：{error}"))?;
    let rows = statement
        .query_map(rusqlite::params_from_iter(args.iter()), summary_from_row)
        .map_err(|error| format!("执行页面列表失败：{error}"))?;
    rows.map(|row| row.map_err(|error| format!("读取页面列表失败：{error}")))
        .collect()
}

#[tauri::command]
fn get_page(page_id: String, state: State<'_, AppState>) -> Result<PageDetail, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let id = resolve_page_summary(connection, &page_id)?
        .map(|page| page.id)
        .ok_or_else(|| "目标页面不存在".to_string())?;
    let (id, page_type, title, year, summary, body, source_path, modified_at, frontmatter_json) = connection
    .query_row(
      "SELECT id,page_type,title,year,summary,body,source_path,modified_at,frontmatter FROM pages WHERE id = ?1",
      [&id],
      |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?, row.get::<_, String>(6)?, row.get::<_, String>(7)?, row.get::<_, String>(8)?)),
    )
    .map_err(|error| format!("读取页面失败：{error}"))?;
    let frontmatter =
        serde_json::from_str::<HashMap<String, String>>(&frontmatter_json).unwrap_or_default();
    let mut links_statement = connection
        .prepare("SELECT target FROM wikilinks WHERE source_id = ?1 ORDER BY target COLLATE NOCASE")
        .map_err(|error| format!("准备页面链接失败：{error}"))?;
    let links = links_statement
        .query_map([&id], |row| row.get::<_, String>(0))
        .map_err(|error| format!("读取页面链接失败：{error}"))?
        .filter_map(Result::ok)
        .collect();
    Ok(PageDetail {
        id,
        page_type,
        title,
        year,
        summary,
        body,
        source_path,
        modified_at,
        frontmatter,
        links,
    })
}

#[tauri::command]
fn resolve_wikilink(target: String, state: State<'_, AppState>) -> Result<LinkResolution, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let (_, anchor) = normalize_link_target(&target);
    let page = resolve_page_summary(connection, &target)?;
    Ok(LinkResolution {
        target,
        anchor,
        resolved: page.is_some(),
        page,
    })
}

#[tauri::command]
fn get_backlinks(page_id: String, state: State<'_, AppState>) -> Result<Vec<Backlink>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let target_page =
        resolve_page_summary(connection, &page_id)?.ok_or_else(|| "目标页面不存在".to_string())?;
    let mut statement = connection.prepare("SELECT w.target,p.id,p.page_type,p.title,p.year,p.summary,p.source_path,p.modified_at,p.status,p.epistemic,p.method_family FROM wikilinks w JOIN pages p ON p.id = w.source_id ORDER BY p.title COLLATE NOCASE").map_err(|error| format!("准备反向链接失败：{error}"))?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                PageSummary {
                    id: row.get(1)?,
                    page_type: row.get(2)?,
                    title: row.get(3)?,
                    year: row.get(4)?,
                    summary: row.get(5)?,
                    source_path: row.get(6)?,
                    modified_at: row.get(7)?,
                    status: row.get(8)?,
                    epistemic: row.get(9)?,
                    method_family: row.get(10)?,
                },
            ))
        })
        .map_err(|error| format!("执行反向链接查询失败：{error}"))?;
    let candidates: Vec<(String, PageSummary)> = rows.filter_map(Result::ok).collect();
    let target_id = target_page.id.to_lowercase();
    Ok(candidates
        .into_iter()
        .filter_map(|(target, source)| {
            let resolved = resolve_page_summary(connection, &target).ok().flatten();
            resolved
                .filter(|page| page.id.to_lowercase() == target_id)
                .map(|_| Backlink { source, target })
        })
        .collect::<Vec<_>>())
}

#[tauri::command]
fn open_local_path(
    path: String,
    reveal: Option<bool>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let root = fs::canonicalize(root).map_err(|error| format!("解析知识库路径失败：{error}"))?;
    let requested = PathBuf::from(&path);
    let candidate = if requested.is_absolute() {
        requested
    } else {
        root.join(requested)
    };
    let canonical =
        fs::canonicalize(&candidate).map_err(|error| format!("文件不存在或无法访问：{error}"))?;
    if !canonical.starts_with(&root) {
        return Err("只允许打开知识库目录内的文件".to_string());
    }
    if !canonical.is_file() {
        return Err("目标路径不是文件".to_string());
    }
    #[cfg(target_os = "windows")]
    {
        let mut command = Command::new("explorer.exe");
        if reveal.unwrap_or(false) {
            command.args(["/select,", canonical.to_string_lossy().as_ref()]);
        } else {
            command.arg(&canonical);
        }
        command
            .spawn()
            .map_err(|error| format!("启动系统文件查看器失败：{error}"))?;
    }
    #[cfg(target_os = "macos")]
    {
        let mut command = Command::new("open");
        if reveal.unwrap_or(false) {
            command.arg("-R");
        }
        command.arg(&canonical);
        command
            .spawn()
            .map_err(|error| format!("启动系统文件查看器失败：{error}"))?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&canonical)
            .spawn()
            .map_err(|error| format!("启动系统文件查看器失败：{error}"))?;
    }
    Ok(canonical.to_string_lossy().to_string())
}

fn core_book_meta(book_id: &str) -> Option<(&'static str, &'static str, usize, &'static str)> {
    match book_id {
        "algorithmic-game-theory" => Some((
            "Algorithmic Game Theory",
            "2007",
            775,
            "raw/canonical/core-books/Algorithmic Game Theory-book.pdf",
        )),
        "approximation-algorithms" => Some((
            "Approximation Algorithms",
            "2001",
            396,
            "raw/canonical/core-books/Approximation Algorithms-book.pdf",
        )),
        _ => None,
    }
}

fn chapter_index(root: &Path, book_id: &str) -> Result<Vec<serde_json::Value>, String> {
    let index_path = root
        .join("raw/canonical")
        .join(book_id)
        .join("chapter-index.json");
    let content = fs::read_to_string(&index_path)
        .map_err(|error| format!("读取章节索引失败：{}：{error}", index_path.display()))?;
    let payload: serde_json::Value =
        serde_json::from_str(&content).map_err(|error| format!("解析章节索引失败：{error}"))?;
    payload
        .get("chapters")
        .and_then(|value| value.as_array())
        .cloned()
        .ok_or_else(|| "章节索引缺少 chapters 数组".to_string())
}

fn resolve_repository_file(root: &Path, relative: &str, context: &str) -> Result<PathBuf, String> {
    let normalized = relative.replace('\\', "/");
    let path = Path::new(relative);
    let has_windows_prefix = normalized.len() >= 2
        && normalized.as_bytes()[1] == b':'
        && normalized.as_bytes()[0].is_ascii_alphabetic();
    if normalized.is_empty()
        || normalized.starts_with('/')
        || normalized.starts_with("//")
        || has_windows_prefix
        || path.is_absolute()
        || normalized.split('/').any(|part| part == "..")
    {
        return Err(format!("章节路径越界：{context}"));
    }
    let canonical_root = fs::canonicalize(root)
        .map_err(|error| format!("解析知识库根目录失败：{} ({error})", root.display()))?;
    let candidate = root.join(path);
    let canonical = fs::canonicalize(&candidate)
        .map_err(|error| format!("章节文件不可读：{context} ({error})"))?;
    if !canonical.starts_with(&canonical_root) {
        return Err(format!("章节路径必须位于知识库目录内：{context}"));
    }
    if !canonical.is_file() {
        return Err(format!("章节路径不是文件：{context}"));
    }
    Ok(canonical)
}

fn value_i64(value: Option<&serde_json::Value>) -> Option<i64> {
    value.and_then(|item| item.as_i64())
}

fn book_chapters(root: &Path, book_id: &str) -> Result<Vec<BookChapter>, String> {
    let (_, _, _, pdf_relative) =
        core_book_meta(book_id).ok_or_else(|| format!("未知核心专著：{book_id}"))?;
    chapter_index(root, book_id)?
        .into_iter()
        .map(|item| {
            let chapter_id = item
                .get("chapter_id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let relative = item
                .get("path")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let markdown_path =
                resolve_repository_file(root, &relative, &format!("{book_id}:{chapter_id}"))?;
            let char_count = item
                .get("char_count")
                .and_then(|v| v.as_u64())
                .unwrap_or_else(|| {
                    fs::read_to_string(&markdown_path)
                        .map(|text| text.chars().count() as u64)
                        .unwrap_or(0)
                }) as usize;
            Ok(BookChapter {
                id: format!("{book_id}:{chapter_id}"),
                book_id: book_id.to_string(),
                chapter_number: item
                    .get("chapter_number")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0),
                title: item
                    .get("chapter_title")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&chapter_id)
                    .to_string(),
                markdown_path: markdown_path.to_string_lossy().to_string(),
                pdf_path: root.join(pdf_relative).to_string_lossy().to_string(),
                physical_page_start: value_i64(item.get("source_page_start")),
                physical_page_end: value_i64(item.get("source_page_end")),
                printed_page_start: value_i64(item.get("printed_page_start")),
                printed_page_end: value_i64(item.get("printed_page_end")),
                char_count,
                ingest_status: item
                    .get("ingest_status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
            })
        })
        .collect()
}

fn find_case_insensitive_range(text: &str, term: &str) -> Option<(usize, usize)> {
    let folded_term = term.to_lowercase();
    if folded_term.is_empty() {
        return None;
    }
    for (start, _) in text.char_indices() {
        let mut folded = String::new();
        for (offset, character) in text[start..].char_indices() {
            let end = start + offset + character.len_utf8();
            folded.push_str(&character.to_lowercase().collect::<String>());
            if folded == folded_term {
                return Some((start, end));
            }
            if !folded_term.starts_with(&folded) || folded.len() > folded_term.len() {
                break;
            }
        }
    }
    None
}

fn slice_chars(text: &str, start: usize, end: usize) -> &str {
    let start_byte = text
        .char_indices()
        .nth(start)
        .map(|(index, _)| index)
        .unwrap_or(text.len());
    let end_byte = text
        .char_indices()
        .nth(end)
        .map(|(index, _)| index)
        .unwrap_or(text.len());
    &text[start_byte.min(end_byte)..end_byte.max(start_byte)]
}

fn build_book_snippet(title: &str, body: &str, terms: &[String]) -> String {
    if let Some((start, end)) = terms
        .iter()
        .find_map(|term| find_case_insensitive_range(body, term))
    {
        let before = body[..start].chars().count();
        let matched_chars = body[start..end].chars().count();
        let snippet = slice_chars(
            body,
            before.saturating_sub(90),
            before.saturating_add(matched_chars).saturating_add(180),
        );
        return snippet.split_whitespace().collect::<Vec<_>>().join(" ");
    }
    if terms
        .iter()
        .any(|term| find_case_insensitive_range(title, term).is_some())
    {
        return body.chars().take(260).collect::<String>();
    }
    body.chars().take(260).collect::<String>()
}

#[tauri::command]
fn list_core_books(state: State<'_, AppState>) -> Result<Vec<BookSummary>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    ["algorithmic-game-theory", "approximation-algorithms"]
        .into_iter()
        .map(|book_id| {
            let (title, year, page_count, pdf_relative) =
                core_book_meta(book_id).expect("registered book");
            let chapters = book_chapters(root, book_id)?;
            let source_path =
                root.join("wiki/sources")
                    .join(if book_id == "algorithmic-game-theory" {
                        "src-book-algorithmic-game-theory.md"
                    } else {
                        "src-book-approximation-algorithms.md"
                    });
            let quality_status = if root.join("raw/canonical/core-books-quality.json").is_file() {
                "pass"
            } else {
                "unknown"
            };
            Ok(BookSummary {
                id: book_id.to_string(),
                title: title.to_string(),
                year: year.to_string(),
                page_count,
                chapter_count: chapters.len(),
                source_path: source_path.to_string_lossy().to_string(),
                pdf_path: root.join(pdf_relative).to_string_lossy().to_string(),
                quality_status: quality_status.to_string(),
            })
        })
        .collect()
}

#[tauri::command]
fn list_book_chapters(
    book_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<BookChapter>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    book_chapters(root, &book_id)
}

#[tauri::command]
fn get_book_chapter(
    book_id: String,
    chapter_id: String,
    state: State<'_, AppState>,
) -> Result<BookChapterDetail, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let chapters = book_chapters(root, &book_id)?;
    let chapter = chapters
        .into_iter()
        .find(|item| item.id == format!("{book_id}:{chapter_id}") || item.id == chapter_id)
        .ok_or_else(|| "章节不存在".to_string())?;
    let body = fs::read_to_string(&chapter.markdown_path)
        .map_err(|error| format!("读取章节 Markdown 失败：{error}"))?;
    Ok(BookChapterDetail { chapter, body })
}

#[tauri::command]
fn search_book_chapters(
    query: String,
    book_id: Option<String>,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<BookSearchResult>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let query_terms = query
        .split_whitespace()
        .map(|term| term.to_lowercase())
        .filter(|term| !term.is_empty())
        .collect::<Vec<_>>();
    if query_terms.is_empty() {
        return Ok(Vec::new());
    }
    let ids = book_id.map(|id| vec![id]).unwrap_or_else(|| {
        vec![
            "algorithmic-game-theory".to_string(),
            "approximation-algorithms".to_string(),
        ]
    });
    let mut results = Vec::new();
    for id in ids {
        for chapter in book_chapters(root, &id)? {
            let body = fs::read_to_string(&chapter.markdown_path).unwrap_or_default();
            let haystack = format!("{} {}", chapter.title, body).to_lowercase();
            let hits = query_terms
                .iter()
                .filter(|term| haystack.contains(term.as_str()))
                .count();
            if hits == 0 {
                continue;
            }
            let snippet = build_book_snippet(&chapter.title, &body, &query_terms);
            results.push(BookSearchResult {
                chapter,
                snippet,
                score: hits as f64 / query_terms.len() as f64,
            });
        }
    }
    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.chapter.chapter_number.cmp(&b.chapter.chapter_number))
    });
    results.truncate(limit.unwrap_or(20).clamp(1, 100));
    Ok(results)
}

fn graph_payload(root: &Path) -> Result<serde_json::Value, String> {
    let path = root.join("graphify-out/graph.json");
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("读取 Graphify 图文件失败：{}：{error}", path.display()))?;
    serde_json::from_str(&content).map_err(|error| format!("解析 Graphify 图文件失败：{error}"))
}

fn graph_node(value: &serde_json::Value) -> GraphNode {
    GraphNode {
        id: value
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        label: value
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        node_type: value
            .get("file_type")
            .and_then(|v| v.as_str())
            .or_else(|| value.get("type").and_then(|v| v.as_str()))
            .unwrap_or("unknown")
            .to_string(),
        source_file: value
            .get("source_file")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        source_location: value
            .get("source_location")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        community: value.get("community").and_then(|v| v.as_i64()),
        origin: value
            .get("_origin")
            .and_then(|v| v.as_str())
            .unwrap_or("derived")
            .to_string(),
    }
}

fn graph_edge(value: &serde_json::Value) -> GraphEdge {
    GraphEdge {
        source: value
            .get("source")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        target: value
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        relation: value
            .get("relation")
            .and_then(|v| v.as_str())
            .unwrap_or("related")
            .to_string(),
        confidence: value
            .get("confidence")
            .and_then(|v| v.as_str())
            .unwrap_or("DERIVED")
            .to_string(),
        weight: value.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0),
    }
}

fn graph_overview_from_payload(
    payload: &serde_json::Value,
    filters: &GraphFilters,
    forced_ids: Option<&HashSet<String>>,
) -> GraphOverview {
    let all_nodes = payload
        .get("nodes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let all_edges = payload
        .get("links")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let query = filters.query.as_deref().unwrap_or("").to_lowercase();
    let mut selected = all_nodes
        .iter()
        .filter_map(|value| {
            let node = graph_node(value);
            if node.id.is_empty() {
                return None;
            }
            if let Some(ids) = forced_ids {
                if !ids.contains(&node.id) {
                    return None;
                }
            }
            if let Some(kind) = filters.node_type.as_deref() {
                if !kind.is_empty() && node.node_type != kind {
                    return None;
                }
            }
            if let Some(community) = filters.community {
                if node.community != Some(community) {
                    return None;
                }
            }
            if !query.is_empty()
                && !format!("{} {}", node.label, node.source_file)
                    .to_lowercase()
                    .contains(&query)
            {
                return None;
            }
            Some(node)
        })
        .collect::<Vec<_>>();
    selected.sort_by_key(|a| a.label.to_lowercase());
    selected.truncate(filters.limit.unwrap_or(120).clamp(1, 500));
    let ids = selected
        .iter()
        .map(|node| node.id.clone())
        .collect::<HashSet<_>>();
    let edges = all_edges
        .iter()
        .map(graph_edge)
        .filter(|edge| ids.contains(&edge.source) && ids.contains(&edge.target))
        .collect::<Vec<_>>();
    let communities = all_nodes
        .iter()
        .filter_map(|value| value.get("community").and_then(|v| v.as_i64()))
        .collect::<HashSet<_>>()
        .len();
    GraphOverview {
        node_count: selected.len(),
        edge_count: edges.len(),
        community_count: communities,
        nodes: selected,
        edges,
    }
}

#[tauri::command]
fn graph_overview(
    filters: Option<GraphFilters>,
    state: State<'_, AppState>,
) -> Result<GraphOverview, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let payload = graph_payload(root)?;
    Ok(graph_overview_from_payload(
        &payload,
        &filters.unwrap_or_default(),
        None,
    ))
}

#[tauri::command]
fn graph_neighbors(
    node_id: String,
    depth: Option<usize>,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<GraphOverview, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let payload = graph_payload(root)?;
    let edges = payload
        .get("links")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|v| graph_edge(&v))
        .collect::<Vec<_>>();
    let mut found = HashSet::from([node_id]);
    let mut frontier = found.clone();
    for _ in 0..depth.unwrap_or(1).clamp(1, 3) {
        let mut next = HashSet::new();
        for edge in &edges {
            if frontier.contains(&edge.source) {
                next.insert(edge.target.clone());
            }
            if frontier.contains(&edge.target) {
                next.insert(edge.source.clone());
            }
        }
        next.retain(|id| !found.contains(id));
        found.extend(next.iter().cloned());
        frontier = next;
        if frontier.is_empty() {
            break;
        }
    }
    let filters = GraphFilters {
        limit,
        ..Default::default()
    };
    Ok(graph_overview_from_payload(
        &payload,
        &filters,
        Some(&found),
    ))
}

#[tauri::command]
fn graph_path(
    source_id: String,
    target_id: String,
    max_depth: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let payload = graph_payload(root)?;
    let edges = payload
        .get("links")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|v| graph_edge(&v))
        .collect::<Vec<_>>();
    let mut queue = VecDeque::from([(source_id.clone(), vec![source_id.clone()])]);
    let mut visited = HashSet::from([source_id]);
    let max_depth = max_depth.unwrap_or(6).clamp(1, 12);
    while let Some((current, path)) = queue.pop_front() {
        if current == target_id {
            return Ok(path);
        }
        if path.len() > max_depth {
            continue;
        }
        for edge in &edges {
            let next = if edge.source == current {
                &edge.target
            } else if edge.target == current {
                &edge.source
            } else {
                continue;
            };
            if visited.insert(next.clone()) {
                let mut next_path = path.clone();
                next_path.push(next.clone());
                queue.push_back((next.clone(), next_path));
            }
        }
    }
    Ok(Vec::new())
}

fn comparison_value(frontmatter: &HashMap<String, String>, body: &str, field: &str) -> String {
    let aliases: &[&str] = match field {
        "场景" => &["scenario"],
        "实体" => &["entities"],
        "目标" => &["objectives"],
        "约束" => &["constraints"],
        "方法族" => &["method_family"],
        "理论保证" => &["guarantee", "approximation_ratio", "theoretical_guarantee"],
        "复杂度" => &["complexity"],
        "实验" => &["experiments", "dataset", "evaluation"],
        "局限" => &["limitations", "caveats"],
        _ => &[],
    };
    for alias in aliases {
        if let Some(value) = frontmatter
            .get(*alias)
            .filter(|value| !value.trim().is_empty())
        {
            return value.clone();
        }
    }
    let heading = format!("## {field}");
    let mut collecting = false;
    let mut lines = Vec::new();
    for line in body.lines() {
        if line.trim().starts_with('#') {
            collecting = line.trim().to_lowercase().contains(&field.to_lowercase());
            continue;
        }
        if collecting && !line.trim().is_empty() {
            lines.push(line.trim());
            if lines.join(" ").chars().count() > 280 {
                break;
            }
        }
    }
    if !lines.is_empty() {
        return lines.join(" ");
    }
    if body.to_lowercase().contains(&heading.to_lowercase()) {
        return "见正文对应章节".to_string();
    }
    String::new()
}

#[tauri::command]
fn build_comparison(
    page_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<ComparisonMatrix, String> {
    if page_ids.len() < 2 || page_ids.len() > 5 {
        return Err("对比对象数量必须为2到5个".to_string());
    }
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let fields = vec![
        "年份",
        "场景",
        "实体",
        "目标",
        "约束",
        "方法族",
        "理论保证",
        "复杂度",
        "实验",
        "局限",
        "来源",
    ];
    let mut columns = Vec::new();
    for page_id in page_ids {
        let id = resolve_page_summary(connection, &page_id)?
            .ok_or_else(|| format!("页面不存在：{page_id}"))?
            .id;
        let (title, page_type, year, body, source_path, frontmatter_json): (
            String,
            String,
            String,
            String,
            String,
            String,
        ) = connection
            .query_row(
                "SELECT title,page_type,year,body,source_path,frontmatter FROM pages WHERE id=?1",
                [&id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                    ))
                },
            )
            .map_err(|error| format!("读取对比页面失败：{error}"))?;
        let frontmatter =
            serde_json::from_str::<HashMap<String, String>>(&frontmatter_json).unwrap_or_default();
        let mut cells = HashMap::new();
        for field in &fields {
            let value = if *field == "年份" {
                year.clone()
            } else if *field == "来源" {
                source_path.clone()
            } else {
                comparison_value(&frontmatter, &body, field)
            };
            cells.insert(
                (*field).to_string(),
                ComparisonCell {
                    value,
                    source_path: source_path.clone(),
                    field: (*field).to_string(),
                },
            );
        }
        columns.push(ComparisonColumn {
            id,
            title,
            page_type,
            cells,
        });
    }
    Ok(ComparisonMatrix {
        fields: fields.into_iter().map(String::from).collect(),
        columns,
    })
}

#[tauri::command]
fn get_luna_settings(state: State<'_, AppState>) -> Result<qa::LunaSettings, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    qa::get_luna_settings(connection, root, false)
}

#[tauri::command]
fn save_luna_settings(
    settings: qa::LunaSettings,
    state: State<'_, AppState>,
) -> Result<qa::LunaSettings, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    qa::save_luna_settings(connection, root, settings)
}

#[tauri::command]
async fn get_qa_settings(state: State<'_, AppState>) -> Result<qa::LunaSettings, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    qa::get_luna_settings(connection, root, false)
}

#[tauri::command]
fn save_qa_settings(
    settings: qa::LunaSettings,
    state: State<'_, AppState>,
) -> Result<qa::LunaSettings, String> {
    save_luna_settings(settings, state)
}

#[tauri::command]
async fn get_codex_subscription_status(
) -> Result<codex_subscription::CodexSubscriptionStatus, String> {
    tauri::async_runtime::spawn_blocking(codex_subscription::get_status)
        .await
        .map_err(|error| format!("Codex 状态线程失败：{error}"))
}

#[tauri::command]
async fn start_codex_login() -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(codex_subscription::start_login)
        .await
        .map_err(|error| format!("Codex 登录线程失败：{error}"))?
}

#[tauri::command]
fn list_chat_sessions(
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<qa::ChatSessionSummary>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先建立本地索引".to_string())?;
    qa::list_sessions(connection, root, limit.unwrap_or(100))
}

#[tauri::command]
fn get_chat_session(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<qa::ChatSessionDetail, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先建立本地索引".to_string())?;
    qa::get_session(connection, root, &session_id)
}

#[tauri::command]
fn create_chat_session(
    title: Option<String>,
    state: State<'_, AppState>,
) -> Result<qa::ChatSessionSummary, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先建立本地索引".to_string())?;
    qa::create_session(connection, root, title.as_deref().unwrap_or("新对话"))
}

#[tauri::command]
fn rename_chat_session(
    session_id: String,
    title: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先建立本地索引".to_string())?;
    qa::rename_session(connection, root, &session_id, &title)
}

#[tauri::command]
fn delete_chat_session(session_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先建立本地索引".to_string())?;
    qa::delete_session(connection, root, &session_id)
}

#[tauri::command]
fn prepare_question(
    question: String,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<qa::QuestionContext, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先建立本地索引".to_string())?;
    qa::prepare_question(connection, root, &question, limit.unwrap_or(14))
}

#[tauri::command]
fn prepare_research_trail(
    request: research_trail::ResearchTrailRequest,
    state: State<'_, AppState>,
) -> Result<research_trail::ResearchTrailResponse, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先建立本地索引".to_string())?;
    research_trail::prepare(connection, root, request)
}

#[tauri::command]
fn cancel_answer(request_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let cancellations = state
        .cancellations
        .lock()
        .map_err(|_| "问答取消状态锁定失败".to_string())?;
    let flag = cancellations
        .get(&request_id)
        .ok_or_else(|| "问答请求已结束或不存在".to_string())?;
    flag.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
async fn ask_luna(
    request: qa::AskRequest,
    on_event: Channel<qa::AnswerStreamEvent>,
    state: State<'_, AppState>,
) -> Result<qa::AskResult, String> {
    let (mut context, settings, root, session_id, authoritative_repository_id) = {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        let root = repository
            .root
            .as_ref()
            .ok_or_else(|| "请先选择知识库目录".to_string())?
            .clone();
        let authoritative_repository_id = qa::repository_id(&root);
        if request.repository_id != authoritative_repository_id {
            return Err("REPOSITORY_CHANGED: 当前知识库已切换，请重新提问".to_string());
        }
        let connection = repository
            .db
            .as_ref()
            .ok_or_else(|| "请先建立本地索引".to_string())?;
        let context = qa::prepare_question(
            connection,
            &root,
            &request.question,
            request.evidence_limit.unwrap_or(14),
        )?;
        let session_id = if let Some(session_id) = request.session_id.as_deref() {
            qa::get_session(connection, &root, session_id)?;
            session_id.to_string()
        } else {
            // Reserve a stable id for stream events, but create the SQLite row
            // only when the exchange is successfully persisted. Cancelled or
            // failed requests therefore cannot leave empty sessions behind.
            Uuid::new_v4().to_string()
        };
        let settings = qa::get_luna_settings(connection, &root, false)?;
        (
            context,
            settings,
            root,
            session_id,
            authoritative_repository_id,
        )
    };
    context.conversation = {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        if repository
            .root
            .as_ref()
            .map(|value| qa::repository_id(value))
            != Some(authoritative_repository_id.clone())
        {
            return Err("REPOSITORY_CHANGED: 当前知识库已切换，请重新提问".to_string());
        }
        let connection = repository
            .db
            .as_ref()
            .ok_or_else(|| "请先建立本地索引".to_string())?;
        qa::conversation_history(connection, &root, request.session_id.as_deref())?
    };

    let codex_ready = if settings.answer_provider == qa::PROVIDER_CODEX {
        tauri::async_runtime::spawn_blocking(codex_subscription::get_status)
            .await
            .map_err(|error| format!("Codex 状态线程失败：{error}"))?
            .ready
    } else {
        false
    };

    {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        if repository
            .root
            .as_ref()
            .map(|value| qa::repository_id(value))
            != Some(authoritative_repository_id.clone())
        {
            return Err("REPOSITORY_CHANGED: 当前知识库已切换，请重新提问".to_string());
        }
    }

    let request_id = context.request_id.clone();
    let cancel_flag = Arc::new(AtomicBool::new(false));
    state
        .cancellations
        .lock()
        .map_err(|_| "问答取消状态锁定失败".to_string())?
        .insert(request_id.clone(), cancel_flag.clone());

    let _ = on_event.send(qa::AnswerStreamEvent::Started {
        request_id: request_id.clone(),
        session_id: session_id.clone(),
    });
    let _ = on_event.send(qa::AnswerStreamEvent::RetrievalStarted {
        request_id: request_id.clone(),
    });
    let _ = on_event.send(qa::AnswerStreamEvent::RetrievalCompleted {
        request_id: request_id.clone(),
        evidence: context.evidence.clone(),
        waterline: context.waterline.clone(),
    });

    let generated: Result<(String, String, String), String> =
        match settings.answer_provider.as_str() {
            qa::PROVIDER_CODEX if codex_ready => {
                let prompt = qa::build_codex_prompt(&context);
                let model = settings.codex_model.clone();
                let timeout = Duration::from_secs(settings.timeout_seconds);
                let stream_channel = on_event.clone();
                let stream_request_id = request_id.clone();
                let stream_cancel_flag = cancel_flag.clone();
                tauri::async_runtime::spawn_blocking(move || {
                    codex_subscription::stream_answer(
                        &prompt,
                        &model,
                        timeout,
                        &stream_cancel_flag,
                        |content| {
                            stream_channel
                                .send(qa::AnswerStreamEvent::Token {
                                    request_id: stream_request_id.clone(),
                                    content: content.to_string(),
                                })
                                .map_err(|error| format!("CODEX_CHANNEL_ERROR: {error}"))
                        },
                    )
                    .map(|(answer, model)| (answer, qa::PROVIDER_CODEX.to_string(), model))
                })
                .await
                .map_err(|error| format!("CODEX_TASK_ERROR: {error}"))?
            }
            qa::PROVIDER_CODEX => Err("CODEX_NOT_READY: 请在设置中登录 ChatGPT".to_string()),
            qa::PROVIDER_API if settings.endpoint.is_empty() || !settings.api_key_configured => {
                Err("LUNA_NOT_CONFIGURED: endpoint 或 API Key 环境变量尚未配置".to_string())
            }
            qa::PROVIDER_API => {
                let remote_settings = settings.clone();
                let remote_context = context.clone();
                let stream_channel = on_event.clone();
                let stream_request_id = request_id.clone();
                let stream_cancel_flag = cancel_flag.clone();
                tauri::async_runtime::spawn_blocking(move || {
                    qa::stream_luna(
                        &remote_settings,
                        &remote_context,
                        &stream_cancel_flag,
                        |content| {
                            stream_channel
                                .send(qa::AnswerStreamEvent::Token {
                                    request_id: stream_request_id.clone(),
                                    content: content.to_string(),
                                })
                                .map_err(|error| format!("LUNA_CHANNEL_ERROR: {error}"))
                        },
                    )
                    .map(|answer| (answer, "luna".to_string(), remote_settings.model.clone()))
                })
                .await
                .map_err(|error| format!("LUNA_TASK_ERROR: {error}"))?
            }
            qa::PROVIDER_OFFLINE => Ok((
                qa::offline_answer(&context),
                qa::PROVIDER_OFFLINE.to_string(),
                "deterministic".to_string(),
            )),
            _ => Err("PROVIDER_INVALID: 不支持的回答引擎".to_string()),
        };

    if cancel_flag.load(Ordering::SeqCst) {
        let _ = on_event.send(qa::AnswerStreamEvent::Cancelled {
            request_id: request_id.clone(),
        });
        if let Ok(mut cancellations) = state.cancellations.lock() {
            cancellations.remove(&request_id);
        }
        return Err("问答已取消".to_string());
    }

    let (answer, provider, model, offline) = match generated {
        Ok((answer, provider, model)) => {
            let offline = provider == qa::PROVIDER_OFFLINE;
            (answer, provider, model, offline)
        }
        Err(error) => {
            let code = error
                .split(':')
                .next()
                .unwrap_or("ANSWER_FAILED")
                .to_string();
            let message = error
                .split_once(':')
                .map(|(_, value)| value.trim())
                .unwrap_or("回答引擎失败")
                .to_string();
            if let Ok(mut repository) = state.repository.lock() {
                if repository
                    .root
                    .as_ref()
                    .map(|value| qa::repository_id(value))
                    == Some(authoritative_repository_id.clone())
                {
                    if let Some(connection) = repository.db.as_mut() {
                        let _ = qa::persist_failure(
                            connection,
                            &root,
                            request.session_id.as_deref(),
                            &request_id,
                            &code,
                            &message,
                            &settings.answer_provider,
                        );
                    }
                }
            }
            let _ = on_event.send(qa::AnswerStreamEvent::Failed {
                request_id: request_id.clone(),
                code,
                message,
                retryable: true,
            });
            if let Ok(mut cancellations) = state.cancellations.lock() {
                cancellations.remove(&request_id);
            }
            return Err(error);
        }
    };

    // Generation may outlive a repository switch. The frontend also ignores
    // stale generations, while this authoritative check prevents old output
    // from being streamed by the offline provider or reaching persistence.
    {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        if repository
            .root
            .as_ref()
            .map(|value| qa::repository_id(value))
            != Some(authoritative_repository_id.clone())
        {
            if let Ok(mut cancellations) = state.cancellations.lock() {
                cancellations.remove(&request_id);
            }
            return Err("REPOSITORY_CHANGED: 当前知识库已切换，旧回答已丢弃".to_string());
        }
    }

    if offline {
        let characters = answer.chars().collect::<Vec<_>>();
        for chunk in characters.chunks(48) {
            if cancel_flag.load(Ordering::SeqCst) {
                let _ = on_event.send(qa::AnswerStreamEvent::Cancelled {
                    request_id: request_id.clone(),
                });
                if let Ok(mut cancellations) = state.cancellations.lock() {
                    cancellations.remove(&request_id);
                }
                return Err("问答已取消".to_string());
            }
            let _ = on_event.send(qa::AnswerStreamEvent::Token {
                request_id: request_id.clone(),
                content: chunk.iter().collect(),
            });
        }
    }

    let persisted = {
        let mut repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        if repository
            .root
            .as_ref()
            .map(|value| qa::repository_id(value))
            != Some(authoritative_repository_id.clone())
        {
            return Err("REPOSITORY_CHANGED: 当前知识库已切换，旧回答未保存".to_string());
        }
        let connection = repository
            .db
            .as_mut()
            .ok_or_else(|| "知识库在问答过程中已关闭".to_string())?;
        qa::persist_exchange(
            connection,
            &root,
            Some(&session_id),
            &context,
            answer,
            &provider,
            &model,
        )
    };
    let result = match persisted {
        Ok(result) => result,
        Err(error) => {
            let code = error
                .split(':')
                .next()
                .unwrap_or("PERSIST_FAILED")
                .to_string();
            let message = error
                .split_once(':')
                .map(|(_, value)| value.trim())
                .unwrap_or(error.as_str())
                .to_string();
            if let Ok(mut repository) = state.repository.lock() {
                if repository
                    .root
                    .as_ref()
                    .map(|value| qa::repository_id(value))
                    == Some(authoritative_repository_id.clone())
                {
                    if let Some(connection) = repository.db.as_mut() {
                        let _ = qa::persist_failure(
                            connection,
                            &root,
                            request.session_id.as_deref(),
                            &request_id,
                            &code,
                            &message,
                            &provider,
                        );
                    }
                }
            }
            let _ = on_event.send(qa::AnswerStreamEvent::Failed {
                request_id: request_id.clone(),
                code,
                message,
                retryable: true,
            });
            if let Ok(mut cancellations) = state.cancellations.lock() {
                cancellations.remove(&request_id);
            }
            return Err(error);
        }
    };
    debug_assert_eq!(result.offline, offline);
    let _ = on_event.send(qa::AnswerStreamEvent::Completed {
        request_id: request_id.clone(),
        result: result.clone(),
    });
    if let Ok(mut cancellations) = state.cancellations.lock() {
        cancellations.remove(&request_id);
    }
    Ok(result)
}

fn compile_repository_context(state: &State<'_, AppState>) -> Result<(PathBuf, PathBuf), String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .clone()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    let db_path = connection
        .path()
        .map(PathBuf::from)
        .ok_or_else(|| "任务数据库路径不可用".to_string())?;
    Ok((root, db_path))
}

#[tauri::command]
async fn get_literature_capabilities(
    state: State<'_, AppState>,
) -> Result<Vec<literature_ingest::LiteratureCapability>, String> {
    let root = {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        repository
            .root
            .clone()
            .ok_or_else(|| "请先选择知识库目录".to_string())?
    };
    tauri::async_runtime::spawn_blocking(move || literature_ingest::capabilities(&root))
        .await
        .map_err(|error| format!("依赖检查线程失败：{error}"))
}

#[tauri::command]
fn get_literature_settings(
    state: State<'_, AppState>,
) -> Result<literature_ingest::LiteratureIngestSettings, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    literature_ingest::get_settings(connection, &root.to_string_lossy())
}

#[tauri::command]
fn save_literature_settings(
    settings: literature_ingest::LiteratureIngestSettings,
    state: State<'_, AppState>,
) -> Result<literature_ingest::LiteratureIngestSettings, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    literature_ingest::save_settings(connection, &root.to_string_lossy(), &settings)
}

#[tauri::command]
async fn list_search_provider_statuses(
) -> Result<Vec<search_credentials::SearchProviderStatus>, String> {
    tauri::async_runtime::spawn_blocking(search_credentials::list_statuses)
        .await
        .map_err(|error| format!("安全凭据状态线程失败：{error}"))?
}

#[tauri::command]
async fn save_search_provider_key(
    provider: String,
    api_key: String,
) -> Result<search_credentials::SearchProviderStatus, String> {
    tauri::async_runtime::spawn_blocking(move || search_credentials::save_key(&provider, &api_key))
        .await
        .map_err(|error| format!("安全凭据保存线程失败：{error}"))?
}

#[tauri::command]
async fn delete_search_provider_key(
    provider: String,
) -> Result<search_credentials::SearchProviderStatus, String> {
    tauri::async_runtime::spawn_blocking(move || search_credentials::delete_key(&provider))
        .await
        .map_err(|error| format!("安全凭据清除线程失败：{error}"))?
}

#[tauri::command]
async fn test_search_provider(provider: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || search_credentials::test_provider(&provider))
        .await
        .map_err(|error| format!("检索源测试线程失败：{error}"))?
}

#[tauri::command]
fn get_ingest_startup_prompt(
    local_date: String,
    state: State<'_, AppState>,
) -> Result<literature_ingest::StartupPromptState, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    literature_ingest::startup_prompt(connection, &root.to_string_lossy(), &local_date)
}

#[tauri::command]
fn suppress_ingest_prompt_today(
    local_date: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    literature_ingest::suppress_today(connection, &root.to_string_lossy(), &local_date)
}

#[tauri::command]
fn choose_manual_pdfs(
    state: State<'_, AppState>,
) -> Result<Option<literature_ingest::ManualImportSession>, String> {
    let Some(paths) = FileDialog::new()
        .set_title("选择要添加到知识库的 PDF")
        .add_filter("PDF", &["pdf"])
        .pick_files()
    else {
        return Ok(None);
    };
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    literature_ingest::create_manual_session(connection, root, paths).map(Some)
}

#[tauri::command]
fn discard_manual_import_session(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    literature_ingest::discard_manual_session(connection, &root.to_string_lossy(), &session_id)
}

#[tauri::command]
async fn list_literature_candidates(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let (root, settings) = {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        let root = repository
            .root
            .clone()
            .ok_or_else(|| "请先选择知识库目录".to_string())?;
        let connection = repository
            .db
            .as_ref()
            .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
        let settings = literature_ingest::get_settings(connection, &root.to_string_lossy())?;
        (root, settings)
    };
    tauri::async_runtime::spawn_blocking(move || {
        literature_ingest::list_candidates(&root, &settings)
    })
    .await
    .map_err(|error| format!("候选读取线程失败：{error}"))?
}

#[tauri::command]
async fn update_candidate_triage(
    candidate_ids: Vec<String>,
    status: String,
    note: Option<String>,
    state: State<'_, AppState>,
) -> Result<u64, String> {
    let root = {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        repository
            .root
            .clone()
            .ok_or_else(|| "请先选择知识库目录".to_string())?
    };
    tauri::async_runtime::spawn_blocking(move || {
        literature_ingest::update_triage(
            &root,
            &candidate_ids,
            &status,
            note.as_deref().unwrap_or(""),
        )
    })
    .await
    .map_err(|error| format!("候选状态线程失败：{error}"))?
}

#[tauri::command]
async fn start_literature_run(
    request: literature_ingest::StartLiteratureRunRequest,
    on_event: Channel<compile_center::CompileStreamEvent>,
    state: State<'_, AppState>,
) -> Result<compile_center::CompileRunSummary, String> {
    let (task_kind, manifest_path) = {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        let root = repository
            .root
            .as_ref()
            .ok_or_else(|| "请先选择知识库目录".to_string())?;
        let connection = repository
            .db
            .as_ref()
            .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
        let db_path = connection
            .path()
            .map(PathBuf::from)
            .ok_or_else(|| "任务数据库路径不可用".to_string())?;
        let destination = db_path.parent().unwrap_or(root).join("literature-runs");
        let path = literature_ingest::build_run_manifest(connection, root, &request, &destination)?;
        (
            literature_ingest::task_kind(&request.mode)?.to_string(),
            path,
        )
    };
    let compile_request = compile_center::StartCompileRequest {
        task_kind,
        input_path: None,
        dry_run: false,
        download: request.mode != "manual",
        force: request.force_duplicates,
        timeout_seconds: request.timeout_seconds,
        literature_mode: request.mode.clone(),
        candidate_ids: request.candidate_ids.clone(),
        manual_session_id: request.manual_session_id.clone(),
        run_manifest: Some(manifest_path.to_string_lossy().to_string()),
    };
    let result = execute_compile_request(&state, compile_request, on_event, None).await;
    if let Ok(repository) = state.repository.lock() {
        if let (Some(root), Some(connection)) = (repository.root.as_ref(), repository.db.as_ref()) {
            if let Ok(mut settings) =
                literature_ingest::get_settings(connection, &root.to_string_lossy())
            {
                settings.last_attempt_at = SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string();
                if matches!(
                    result.as_ref().map(|summary| summary.status.as_str()),
                    Ok("succeeded" | "failed_partial")
                ) {
                    settings.last_success_at = settings.last_attempt_at.clone();
                }
                let _ = literature_ingest::save_settings(
                    connection,
                    &root.to_string_lossy(),
                    &settings,
                );
            }
        }
    }
    result
}

#[tauri::command]
fn get_compile_capabilities(
    state: State<'_, AppState>,
) -> Result<Vec<compile_center::CompileCapability>, String> {
    let (root, _) = compile_repository_context(&state)?;
    Ok(compile_center::capabilities(&root))
}

#[tauri::command]
fn list_compile_runs(
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<compile_center::CompileRunSummary>, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    compile_center::list_runs(connection, &root.to_string_lossy(), limit.unwrap_or(100))
}

#[tauri::command]
fn get_compile_run(
    run_id: String,
    state: State<'_, AppState>,
) -> Result<compile_center::CompileRunDetail, String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    compile_center::get_run(connection, &root.to_string_lossy(), &run_id)
}

async fn execute_compile_request(
    state: &State<'_, AppState>,
    request: compile_center::StartCompileRequest,
    on_event: Channel<compile_center::CompileStreamEvent>,
    retry_of: Option<String>,
) -> Result<compile_center::CompileRunSummary, String> {
    let (root, db_path) = compile_repository_context(state)?;
    let run_id = Uuid::new_v4().to_string();
    let cancellation = Arc::new(AtomicBool::new(false));
    {
        let mut cancellations = state
            .compile_cancellations
            .lock()
            .map_err(|_| "任务取消状态锁定失败".to_string())?;
        if cancellations.is_empty() {
            let recovery_connection = Connection::open(&db_path)
                .map_err(|error| format!("打开任务数据库失败：{error}"))?;
            compile_center::db_schema(&recovery_connection)?;
            compile_center::recover_interrupted_runs(&recovery_connection)?;
        }
        cancellations.insert(run_id.clone(), cancellation.clone());
    }
    let worker_run_id = run_id.clone();
    let worker_cancellation = cancellation.clone();
    let worker_result = tauri::async_runtime::spawn_blocking(move || {
        compile_center::execute_run(
            &db_path,
            &root,
            worker_run_id,
            request,
            on_event,
            worker_cancellation,
            retry_of,
        )
    })
    .await;
    if let Ok(mut cancellations) = state.compile_cancellations.lock() {
        cancellations.remove(&run_id);
    }
    worker_result.map_err(|error| format!("编译任务线程失败：{error}"))?
}

#[tauri::command]
async fn start_compile_run(
    request: compile_center::StartCompileRequest,
    on_event: Channel<compile_center::CompileStreamEvent>,
    state: State<'_, AppState>,
) -> Result<compile_center::CompileRunSummary, String> {
    if request.task_kind.starts_with("literature_") {
        return Err("文献入库任务必须通过受控文献入口启动".into());
    }
    execute_compile_request(&state, request, on_event, None).await
}

#[tauri::command]
async fn retry_compile_run(
    run_id: String,
    on_event: Channel<compile_center::CompileStreamEvent>,
    state: State<'_, AppState>,
) -> Result<compile_center::CompileRunSummary, String> {
    let request = {
        let repository = state
            .repository
            .lock()
            .map_err(|_| "知识库状态锁定失败".to_string())?;
        let root = repository
            .root
            .as_ref()
            .ok_or_else(|| "请先选择知识库目录".to_string())?;
        let connection = repository
            .db
            .as_ref()
            .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
        compile_center::get_run(connection, &root.to_string_lossy(), &run_id)?.request
    };
    execute_compile_request(&state, request, on_event, Some(run_id)).await
}

#[tauri::command]
fn cancel_compile_run(run_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let cancellations = state
        .compile_cancellations
        .lock()
        .map_err(|_| "任务取消状态锁定失败".to_string())?;
    let cancellation = cancellations
        .get(&run_id)
        .ok_or_else(|| "任务已经结束或不在当前进程中".to_string())?;
    cancellation.store(true, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
fn pause_compile_run(run_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    compile_center::set_pause_requested(connection, root, &run_id, true)
}

#[tauri::command]
fn resume_compile_run(run_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    compile_center::set_pause_requested(connection, root, &run_id, false)
}

#[tauri::command]
fn rollback_compile_run(run_id: String, state: State<'_, AppState>) -> Result<String, String> {
    let mut repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    let root = repository
        .root
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?
        .clone();
    let connection = repository
        .db
        .as_mut()
        .ok_or_else(|| "SQLite连接尚未建立".to_string())?;
    compile_center::rollback_run(connection, &root, &run_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(AppState::default())
        .setup(|app| {
            if let Ok(data_dir) = app.path().app_local_data_dir() {
                let path_file = data_dir.join("repository.json");
                if let Ok(content) = fs::read_to_string(path_file) {
                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(path) = payload.get("path").and_then(|value| value.as_str()) {
                            if Path::new(path).exists() {
                                let mut opened_ok = false;
                                if let Ok(mut state) = app.state::<AppState>().repository.lock() {
                                    let opened = open_repository_state(
                                        &mut state,
                                        app.handle(),
                                        PathBuf::from(path),
                                    );
                                    opened_ok = opened.is_ok();
                                }
                                if opened_ok {
                                    let _ = start_repository_watcher(
                                        &app.state::<AppState>(),
                                        PathBuf::from(path),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            choose_repository,
            open_repository,
            get_repository_watch_status,
            process_repository_changes,
            rebuild_index,
            repository_info,
            search_pages,
            list_pages,
            get_page,
            resolve_wikilink,
            get_backlinks,
            open_local_path,
            list_core_books,
            list_book_chapters,
            get_book_chapter,
            search_book_chapters,
            graph_overview,
            graph_neighbors,
            graph_path,
            build_comparison,
            get_luna_settings,
            save_luna_settings,
            get_qa_settings,
            save_qa_settings,
            get_codex_subscription_status,
            start_codex_login,
            list_chat_sessions,
            get_chat_session,
            create_chat_session,
            rename_chat_session,
            delete_chat_session,
            prepare_question,
            prepare_research_trail,
            ask_luna,
            cancel_answer,
            get_literature_capabilities,
            get_literature_settings,
            save_literature_settings,
            list_search_provider_statuses,
            save_search_provider_key,
            delete_search_provider_key,
            test_search_provider,
            get_ingest_startup_prompt,
            suppress_ingest_prompt_today,
            choose_manual_pdfs,
            discard_manual_import_session,
            list_literature_candidates,
            update_candidate_triage,
            start_literature_run,
            get_compile_capabilities,
            list_compile_runs,
            get_compile_run,
            start_compile_run,
            retry_compile_run,
            cancel_compile_run,
            pause_compile_run,
            resume_compile_run,
            rollback_compile_run
        ])
        .run(tauri::generate_context!())
        .expect("error while running wireless charging research workbench");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_frontmatter_and_links() {
        let content =
            "---\ntype: source\ntitle: Sample\n---\n# Sample\n\nSee [[methods/demo|Demo]].";
        let fields = parse_frontmatter(content);
        assert_eq!(fields.get("type").map(String::as_str), Some("source"));
        assert_eq!(
            fallback_title(body_without_frontmatter(content), Path::new("sample.md")),
            "Sample"
        );
        assert_eq!(extract_links(content), vec!["methods/demo"]);
    }

    #[test]
    fn rebuilds_a_small_wiki_into_sqlite() {
        let temp = tempfile::tempdir().expect("temp directory");
        let wiki = temp.path().join("wiki").join("sources");
        fs::create_dir_all(&wiki).expect("wiki directory");
        fs::write(temp.path().join("AGENTS.md"), "fixture").expect("agents");
        fs::create_dir_all(temp.path().join("schema")).expect("schema");
        fs::write(
            wiki.join("demo.md"),
            "---\ntype: source\ntitle: Demo\nyear: 2025\n---\n# Demo\n\nA searchable page.",
        )
        .expect("page");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        let stats = rebuild_connection(&mut connection, temp.path()).expect("index");
        assert_eq!(stats.page_count, 1);
        assert_eq!(stats.source_count, 1);
    }

    #[test]
    fn indexes_canonical_paper_sections_with_locations_and_keeps_wiki_evidence() {
        let temp = tempfile::tempdir().expect("temp directory");
        let wiki = temp.path().join("wiki").join("sources");
        let raw = temp.path().join("raw/canonical/demo");
        fs::create_dir_all(&wiki).expect("wiki directory");
        fs::create_dir_all(&raw).expect("raw directory");
        fs::write(temp.path().join("AGENTS.md"), "fixture").expect("agents");
        fs::create_dir_all(temp.path().join("schema")).expect("schema");
        fs::write(
            raw.join("full.md"),
            "---\ntitle: Demo raw\n---\n# System Model\n\nConcurrent wireless charging uses constructive interference.\n\n## 算法\n\n调度算法从无线充电器集合中选择并发子集。",
        )
        .expect("raw markdown");
        fs::write(raw.join("paper.pdf"), "%PDF-fixture").expect("pdf");
        fs::write(
            wiki.join("demo.md"),
            "---\ntype: source\ntitle: Demo Paper\nyear: 2025\nsource_type: paper\nraw_md: raw/canonical/demo/full.md\npdf_path: raw/canonical/demo/paper.pdf\n---\n# Demo Paper\n\nA Wiki summary about concurrent interference scheduling.",
        )
        .expect("page");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        rebuild_connection(&mut connection, temp.path()).expect("index");

        let section_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM paper_sections", [], |row| row.get(0))
            .expect("section count");
        assert_eq!(section_count, 2);
        let context = qa::prepare_question(
            &connection,
            temp.path(),
            "concurrent interference scheduling algorithm",
            10,
        )
        .expect("question context");
        assert!(context.evidence.iter().any(|item| item.kind == "wiki"));
        let paper = context
            .evidence
            .iter()
            .find(|item| item.kind == "paper")
            .expect("paper evidence");
        assert_eq!(paper.page_id, "sources/demo");
        assert!(paper.source_location.contains("原文第"));
        assert!(paper.wikilink.contains("sources/demo"));

        let chinese = qa::prepare_question(&connection, temp.path(), "调度算法", 10)
            .expect("Chinese paper query");
        assert!(chinese.evidence.iter().any(|item| {
            item.kind == "paper"
                && item.source_location.contains("算法")
                && item.snippet.contains("调度算法")
        }));
    }

    #[test]
    fn rejects_source_raw_paths_outside_canonical_root() {
        let temp = tempfile::tempdir().expect("temp directory");
        let wiki = temp.path().join("wiki").join("sources");
        fs::create_dir_all(&wiki).expect("wiki directory");
        fs::create_dir_all(temp.path().join("raw/canonical")).expect("raw directory");
        fs::write(temp.path().join("outside.md"), "# Outside\nsecret").expect("outside");
        fs::write(temp.path().join("AGENTS.md"), "fixture").expect("agents");
        fs::create_dir_all(temp.path().join("schema")).expect("schema");
        fs::write(
            wiki.join("demo.md"),
            "---\ntype: source\ntitle: Demo Paper\nyear: 2025\nsource_type: paper\nraw_md: outside.md\n---\n# Demo Paper\n\nSummary.",
        )
        .expect("page");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        rebuild_connection(&mut connection, temp.path()).expect("index");
        let section_count: i64 = connection
            .query_row("SELECT COUNT(*) FROM paper_sections", [], |row| row.get(0))
            .expect("section count");
        assert_eq!(section_count, 0);
    }

    #[test]
    fn search_pages_uses_valid_fts_snippet_for_prefix_and_unicode_queries() {
        let temp = tempfile::tempdir().expect("temp directory");
        let wiki = temp.path().join("wiki").join("methods");
        fs::create_dir_all(&wiki).expect("wiki directory");
        fs::write(temp.path().join("AGENTS.md"), "fixture").expect("agents");
        fs::create_dir_all(temp.path().join("schema")).expect("schema");
        fs::write(
            wiki.join("current.md"),
            "---\ntype: method\ntitle: Current Allocation\nyear: 2026\n---\n# Current Allocation\n\nA current scheduling method for 无线充电调度。",
        )
        .expect("page");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        rebuild_connection(&mut connection, temp.path()).expect("index");

        let english = query_pages(&connection, "curr", 20).expect("prefix search");
        assert_eq!(english.len(), 1);
        assert!(english[0].snippet.contains("<mark>current</mark>"));

        let chinese = query_pages(&connection, "无线充电调度", 20).expect("unicode search");
        assert_eq!(chinese.len(), 1);
        assert!(query_pages(&connection, "not-present-anywhere", 20)
            .expect("empty search")
            .is_empty());
    }

    #[test]
    fn rebuilding_index_preserves_chat_history() {
        let temp = tempfile::tempdir().expect("temp directory");
        let wiki = temp.path().join("wiki").join("methods");
        fs::create_dir_all(&wiki).expect("wiki directory");
        fs::write(temp.path().join("AGENTS.md"), "fixture").expect("agents");
        fs::create_dir_all(temp.path().join("schema")).expect("schema");
        fs::write(
            wiki.join("demo.md"),
            "---\ntype: method\ntitle: Demo\n---\n# Demo\n\nonline scheduling algorithm",
        )
        .expect("page");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        qa::create_session(&connection, temp.path(), "preserved").expect("session");
        rebuild_connection(&mut connection, temp.path()).expect("index");
        let sessions = qa::list_sessions(&connection, temp.path(), 10).expect("history");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].title, "preserved");
    }

    #[test]
    fn indexes_detail_fields_and_resolves_wikilinks() {
        let temp = tempfile::tempdir().expect("temp directory");
        let wiki = temp.path().join("wiki").join("methods");
        fs::create_dir_all(&wiki).expect("wiki directory");
        fs::write(temp.path().join("AGENTS.md"), "fixture").expect("agents");
        fs::create_dir_all(temp.path().join("schema")).expect("schema");
        fs::write(
            wiki.join("demo.md"),
            "---\ntype: method\ntitle: Demo Method\nyear: 2025\nstatus: active\nmethod_family: matching\nscenario: [wireless_charging]\n---\n# Demo Method\n\nA reusable method.",
        )
        .expect("page");
        let source_dir = temp.path().join("wiki").join("sources");
        fs::create_dir_all(&source_dir).expect("source directory");
        fs::write(
            source_dir.join("source.md"),
            "---\ntype: source\ntitle: Source\nyear: 2024\n---\n# Source\n\nUses [[methods/demo|Demo Method]].",
        )
        .expect("source page");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        rebuild_connection(&mut connection, temp.path()).expect("index");
        let resolved = resolve_page_summary(&connection, "Demo Method")
            .expect("resolve")
            .expect("page");
        assert_eq!(resolved.id, "methods/demo");
        assert_eq!(resolved.method_family, "matching");
        let links: String = connection
            .query_row(
                "SELECT target FROM wikilinks WHERE source_id='sources/source'",
                [],
                |row| row.get(0),
            )
            .expect("wikilink");
        assert_eq!(links, "methods/demo");
    }

    #[test]
    fn incrementally_upserts_renames_and_deletes_wiki_pages() {
        let temp = tempfile::tempdir().expect("temp directory");
        let wiki_root = temp.path().join("wiki");
        let methods = wiki_root.join("methods");
        fs::create_dir_all(&methods).expect("methods directory");
        let original = methods.join("original.md");
        fs::write(
            &original,
            "---\ntype: method\ntitle: Original\nstatus: active\n---\n# Original\n\nFirst body [[sources/a]].",
        )
        .expect("original page");
        let connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        let (id, page_type) = upsert_wiki_page_index(&connection, &wiki_root, &original)
            .expect("initial incremental index");
        assert_eq!(id, "methods/original");
        assert_eq!(page_type, "method");

        fs::write(
            &original,
            "---\ntype: method\ntitle: Updated\nstatus: active\n---\n# Updated\n\nSecond body [[sources/b]].",
        )
        .expect("updated page");
        upsert_wiki_page_index(&connection, &wiki_root, &original).expect("updated index");
        let fts_count: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM pages_fts WHERE page_id='methods/original'",
                [],
                |row| row.get(0),
            )
            .expect("fts count");
        assert_eq!(fts_count, 1);
        let links = connection
            .prepare("SELECT target FROM wikilinks WHERE source_id='methods/original'")
            .expect("links statement")
            .query_map([], |row| row.get::<_, String>(0))
            .expect("links")
            .collect::<Result<Vec<_>, _>>()
            .expect("link values");
        assert_eq!(links, vec!["sources/b"]);

        let renamed = methods.join("renamed.md");
        fs::rename(&original, &renamed).expect("rename page");
        delete_wiki_page_index(&connection, &wiki_root, &original).expect("delete old id");
        upsert_wiki_page_index(&connection, &wiki_root, &renamed).expect("index renamed id");
        assert!(page_summary_by_id(&connection, "methods/original")
            .expect("old summary")
            .is_none());
        assert!(page_summary_by_id(&connection, "methods/renamed")
            .expect("new summary")
            .is_some());

        fs::remove_file(&renamed).expect("remove page");
        delete_wiki_page_index(&connection, &wiki_root, &renamed).expect("delete index");
        assert_eq!(
            current_index_stats(&connection, temp.path())
                .expect("stats")
                .page_count,
            0
        );
    }

    #[test]
    fn repository_identity_rebuilds_derived_rows_and_preserves_user_tables() {
        let first = tempfile::tempdir().expect("first repository");
        let first_wiki = first.path().join("wiki").join("methods");
        fs::create_dir_all(&first_wiki).expect("first wiki");
        fs::write(first.path().join("AGENTS.md"), "fixture").expect("first agents");
        fs::create_dir_all(first.path().join("schema")).expect("first schema");
        fs::write(
            first_wiki.join("first.md"),
            "---\ntype: method\ntitle: First\n---\n# First\n\nfirst repository",
        )
        .expect("first page");

        let second = tempfile::tempdir().expect("second repository");
        let second_wiki = second.path().join("wiki").join("methods");
        fs::create_dir_all(&second_wiki).expect("second wiki");
        fs::write(second.path().join("AGENTS.md"), "fixture").expect("second agents");
        fs::create_dir_all(second.path().join("schema")).expect("second schema");
        fs::write(
            second_wiki.join("second.md"),
            "---\ntype: method\ntitle: Second\n---\n# Second\n\nsecond repository",
        )
        .expect("second page");

        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        qa::create_session(&connection, first.path(), "preserved").expect("session");
        connection
            .execute(
                "INSERT INTO app_settings(key,value) VALUES('fixture.setting','kept')",
                [],
            )
            .expect("setting");

        let first_root = first.path().canonicalize().expect("canonical first root");
        ensure_repository_index(&mut connection, &first_root).expect("first index");
        assert!(page_summary_by_id(&connection, "methods/first")
            .expect("first summary")
            .is_some());

        let second_root = second.path().canonicalize().expect("canonical second root");
        let second_stats =
            ensure_repository_index(&mut connection, &second_root).expect("second index");
        assert_eq!(second_stats.page_count, 1);
        assert!(page_summary_by_id(&connection, "methods/first")
            .expect("old summary")
            .is_none());
        assert!(page_summary_by_id(&connection, "methods/second")
            .expect("new summary")
            .is_some());
        assert_eq!(
            qa::list_sessions(&connection, first.path(), 10)
                .expect("sessions")
                .len(),
            1
        );
        let setting: String = connection
            .query_row(
                "SELECT value FROM app_settings WHERE key='fixture.setting'",
                [],
                |row| row.get(0),
            )
            .expect("setting value");
        assert_eq!(setting, "kept");
        assert_eq!(
            read_repository_identity(&connection)
                .expect("identity")
                .as_deref(),
            Some(repository_identity(&second_root).as_str())
        );
    }

    #[test]
    fn repository_identity_normalizes_separators_and_trailing_slashes() {
        let identity = repository_identity(Path::new("C:\\Knowledge\\Repo\\"));
        assert_eq!(
            identity,
            if cfg!(windows) {
                "c:/knowledge/repo"
            } else {
                "C:/Knowledge/Repo"
            }
        );
    }

    #[test]
    fn book_snippet_uses_body_offsets_and_preserves_unicode_boundaries() {
        let title = "A very long chapter title that must not shift the body offset";
        let body = format!("前缀🙂{} 中间内容 后缀", "x".repeat(120));
        let terms = vec!["中间内容".to_string()];
        let snippet = build_book_snippet(title, &body, &terms);
        assert!(snippet.contains("中间内容"));
        assert!(snippet.is_char_boundary(snippet.len()));
    }

    #[test]
    fn repository_file_resolution_rejects_escape_paths() {
        let temp = tempfile::tempdir().expect("repository");
        let chapter = temp.path().join("chapter.md");
        fs::write(&chapter, "chapter").expect("chapter");
        let resolved = resolve_repository_file(temp.path(), "chapter.md", "book:chapter")
            .expect("valid chapter");
        assert_eq!(resolved, chapter.canonicalize().expect("canonical chapter"));
        assert!(resolve_repository_file(temp.path(), "../outside.md", "book:escape").is_err());
        assert!(resolve_repository_file(temp.path(), "C:/outside.md", "book:drive").is_err());
    }

    #[test]
    fn core_book_indexes_match_library_baseline() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("repository root");
        let game = book_chapters(&root, "algorithmic-game-theory").expect("game theory chapters");
        let approximation =
            book_chapters(&root, "approximation-algorithms").expect("approximation chapters");
        assert_eq!(game.len(), 30);
        assert_eq!(approximation.len(), 31);
        assert_eq!(
            game.iter()
                .chain(approximation.iter())
                .filter(|chapter| chapter.physical_page_start.is_some())
                .count(),
            61
        );
    }

    #[test]
    fn p3_real_repository_question_returns_auditable_evidence() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("repository root");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        let stats = rebuild_connection(&mut connection, &root).expect("full index");
        assert_eq!(stats.source_count, 23);
        assert_eq!(stats.method_count, 20);
        assert_eq!(stats.chapter_count, 61);
        let context = qa::prepare_question(
            &connection,
            &root,
            "online wireless charging scheduling algorithm solution",
            14,
        )
        .expect("question context");
        assert!(context.evidence.iter().any(|item| item.kind == "wiki"));
        assert!(context.evidence.iter().any(|item| item.kind == "book"));
        assert!(context
            .evidence
            .iter()
            .filter(|item| item.kind == "book")
            .all(|item| item.physical_page_start.is_some()));
    }

    #[test]
    fn p3_gold_questions_recall_expected_wiki_evidence() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .canonicalize()
            .expect("repository root");
        let mut connection = Connection::open_in_memory().expect("sqlite");
        db_schema(&connection).expect("schema");
        rebuild_connection(&mut connection, &root).expect("full index");
        let payload: serde_json::Value =
            serde_json::from_str(include_str!("../../../../evals/gold_questions.json"))
                .expect("gold questions");
        let cases = payload["cases"].as_array().expect("cases");
        let mut missed = Vec::new();
        for case in cases {
            let question = case["question"].as_str().expect("question");
            let expected = case["expected_wikilinks"]
                .as_array()
                .expect("expected links")
                .iter()
                .filter_map(|value| value.as_str())
                .filter(|value| !value.contains("library-status"))
                .collect::<Vec<_>>();
            let contract = case["evidence_contract"]
                .as_object()
                .expect("evidence contract");
            let paper_sources = contract["paper_sources"]
                .as_array()
                .expect("paper sources")
                .iter()
                .filter_map(|value| value.as_str())
                .collect::<Vec<_>>();
            let context = qa::prepare_question(&connection, &root, question, 20)
                .unwrap_or_else(|error| panic!("question failed: {question}: {error}"));
            let wiki_hit = context.evidence.iter().any(|item| {
                item.kind == "wiki"
                    && expected
                        .iter()
                        .any(|target| item.page_id.trim_end_matches(".md").ends_with(target))
            });
            let paper_hit = context.evidence.iter().any(|item| {
                item.kind == "paper"
                    && paper_sources
                        .iter()
                        .any(|target| item.page_id.trim_end_matches(".md").ends_with(target))
                    && !item.source_location.is_empty()
                    && item.source_location.contains("原文第")
                    && item.source_location.contains('行')
            });
            if !wiki_hit || !paper_hit {
                missed.push(format!(
                    "{} -> wiki_hit={wiki_hit}, paper_hit={paper_hit}, evidence={:?}",
                    case["id"].as_str().unwrap_or("unknown"),
                    context
                        .evidence
                        .iter()
                        .map(|item| format!(
                            "{}:{}:{}",
                            item.kind, item.page_id, item.source_location
                        ))
                        .collect::<Vec<_>>()
                ));
            }
        }
        assert!(
            missed.is_empty(),
            "每个固定问题必须同时召回预期 Wiki 与可定位的 primary paper 证据；missed={missed:?}"
        );
    }

    #[test]
    fn graph_overview_respects_node_filters() {
        let payload = serde_json::json!({
            "nodes": [
                {"id":"a","label":"Alpha","file_type":"markdown","community":1,"source_file":"wiki/a.md"},
                {"id":"b","label":"Beta","file_type":"code","community":2,"source_file":"src/b.rs"}
            ],
            "links": [{"source":"a","target":"b","relation":"links","confidence":"EXTRACTED","weight":1.0}]
        });
        let overview = graph_overview_from_payload(
            &payload,
            &GraphFilters {
                node_type: Some("markdown".to_string()),
                limit: Some(10),
                ..Default::default()
            },
            None,
        );
        assert_eq!(overview.node_count, 1);
        assert_eq!(overview.edge_count, 0);
        assert_eq!(overview.nodes[0].id, "a");
    }
}
