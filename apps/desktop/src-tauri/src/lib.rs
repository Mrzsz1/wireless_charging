use rfd::FileDialog;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tauri::ipc::Channel;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;
use walkdir::WalkDir;

mod compile_center;
mod qa;

#[derive(Default)]
struct RepositoryState {
    root: Option<PathBuf>,
    db: Option<Connection>,
    indexed_pages: usize,
}

#[derive(Default)]
struct AppState {
    repository: Mutex<RepositoryState>,
    cancellations: Mutex<HashMap<String, Arc<AtomicBool>>>,
    compile_cancellations: Mutex<HashMap<String, Arc<AtomicBool>>>,
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
      CREATE TABLE IF NOT EXISTS wikilinks (
        source_id TEXT NOT NULL,
        target TEXT NOT NULL,
        UNIQUE(source_id, target)
      );
      CREATE INDEX IF NOT EXISTS idx_pages_type ON pages(page_type);
      CREATE INDEX IF NOT EXISTS idx_pages_year ON pages(year);
      CREATE INDEX IF NOT EXISTS idx_wikilinks_target ON wikilinks(target);
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

fn rebuild_connection(connection: &mut Connection, root: &Path) -> Result<IndexStats, String> {
    let wiki_root = root.join("wiki");
    let tx = connection
        .transaction()
        .map_err(|error| format!("开启索引事务失败：{error}"))?;
    tx.execute("DELETE FROM pages", [])
        .map_err(|error| format!("清理页面索引失败：{error}"))?;
    tx.execute("DELETE FROM pages_fts", [])
        .map_err(|error| format!("清理全文索引失败：{error}"))?;
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
        let content = fs::read_to_string(path)
            .map_err(|error| format!("读取{}失败：{error}", path.display()))?;
        let fields = parse_frontmatter(&content);
        let body = body_without_frontmatter(&content).to_string();
        let id = page_id(&wiki_root, path);
        let page_type = fields
            .get("type")
            .cloned()
            .unwrap_or_else(|| "page".to_string());
        let title = fields
            .get("title")
            .cloned()
            .unwrap_or_else(|| fallback_title(&body, path));
        let year = fields.get("year").cloned().unwrap_or_default();
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
        tx.execute("INSERT INTO pages (id,page_type,title,year,summary,body,source_path,modified_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8)", params![id, page_type, title, year, summary, body, path.to_string_lossy().to_string(), modified_at]).map_err(|error| format!("写入页面索引失败：{error}"))?;
        let status = field_value(&fields, "status");
        let epistemic = field_value(&fields, "epistemic");
        let method_family = field_value(&fields, "method_family");
        let scenario = field_value(&fields, "scenario");
        let objectives = field_value(&fields, "objectives");
        let constraints = field_value(&fields, "constraints");
        let frontmatter = serialize_frontmatter(&fields);
        tx.execute("UPDATE pages SET status=?2,epistemic=?3,method_family=?4,scenario=?5,objectives=?6,constraints=?7,frontmatter=?8 WHERE id=?1", params![id, status, epistemic, method_family, scenario, objectives, constraints, frontmatter]).map_err(|error| format!("写入页面字段失败：{error}"))?;
        let keywords = [
            fields.get("paper_keywords").cloned().unwrap_or_default(),
            method_family,
            scenario,
            objectives,
            constraints,
        ]
        .join(" ");
        tx.execute(
            "INSERT INTO pages_fts (page_id,title,body,keywords) VALUES (?1,?2,?3,?4)",
            params![id, title, body, keywords],
        )
        .map_err(|error| format!("写入全文索引失败：{error}"))?;
        for target in extract_links(&body) {
            tx.execute(
                "INSERT OR IGNORE INTO wikilinks (source_id,target) VALUES (?1,?2)",
                params![id, target],
            )
            .map_err(|error| format!("写入链接索引失败：{error}"))?;
        }
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
    validate_repository(&root)?;
    let db_path = repository_db_path(app)?;
    fs::create_dir_all(db_path.parent().unwrap_or(root.as_path()))
        .map_err(|error| format!("创建客户端缓存目录失败：{error}"))?;
    let connection =
        Connection::open(&db_path).map_err(|error| format!("打开SQLite失败：{error}"))?;
    db_schema(&connection)?;
    let indexed_pages = connection
        .query_row("SELECT COUNT(*) FROM pages", [], |row| row.get::<_, i64>(0))
        .unwrap_or(0)
        .max(0) as usize;
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

#[tauri::command]
fn choose_repository(app: AppHandle, state: State<'_, AppState>) -> Result<RepositoryInfo, String> {
    let Some(path) = FileDialog::new()
        .set_title("选择无线充电调度知识库")
        .pick_folder()
    else {
        return Err("用户取消了目录选择".to_string());
    };
    let mut repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    open_repository_state(&mut repository, &app, path)
}

#[tauri::command]
fn open_repository(
    app: AppHandle,
    path: String,
    state: State<'_, AppState>,
) -> Result<RepositoryInfo, String> {
    let mut repository = state
        .repository
        .lock()
        .map_err(|_| "知识库状态锁定失败".to_string())?;
    open_repository_state(&mut repository, &app, PathBuf::from(path))
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
    let query = query.trim().to_string();
    if query.is_empty() {
        return Ok(Vec::new());
    }
    let limit = limit.unwrap_or(20).clamp(1, 100) as i64;
    let fts_query = query
        .split_whitespace()
        .map(|term| format!("\"{}\"*", term.replace('"', "")))
        .collect::<Vec<_>>()
        .join(" AND ");
    let mut results = Vec::new();
    let mut statement = connection.prepare("SELECT p.id,p.page_type,p.title,p.year,p.summary,p.source_path,snippet(pages_fts,2,'<mark>','</mark>'),bm25(pages_fts) FROM pages_fts JOIN pages p ON p.id=pages_fts.page_id WHERE pages_fts MATCH ?1 ORDER BY bm25(pages_fts) LIMIT ?2").map_err(|error| format!("准备搜索失败：{error}"))?;
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
            let markdown_path = root.join(&relative);
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
            let snippet = query_terms
                .iter()
                .find_map(|term| {
                    haystack.find(term).map(|index| {
                        let start = index.saturating_sub(90);
                        let end = (index + term.len() + 180).min(body.len());
                        body.get(start..end).unwrap_or("").replace('\n', " ")
                    })
                })
                .unwrap_or_else(|| body.chars().take(260).collect());
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
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    qa::get_luna_settings(connection)
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
    let connection = repository
        .db
        .as_ref()
        .ok_or_else(|| "请先选择知识库目录".to_string())?;
    qa::save_luna_settings(connection, settings)
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
    let (context, settings, root, session_id) = {
        let repository = state
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
        let settings = qa::get_luna_settings(connection)?;
        (context, settings, root, session_id)
    };

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

    let remote_settings = settings.clone();
    let remote_context = context.clone();
    let stream_channel = on_event.clone();
    let stream_request_id = request_id.clone();
    let stream_cancel_flag = cancel_flag.clone();
    let generated = if settings.endpoint.is_empty() || !settings.api_key_configured {
        Err("LUNA_NOT_CONFIGURED: endpoint 或 API Key 环境变量尚未配置".to_string())
    } else {
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
        })
        .await
        .map_err(|error| format!("LUNA_TASK_ERROR: {error}"))?
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
        Ok(answer) => (answer, "luna", settings.model.as_str(), false),
        Err(error) => {
            let mut answer = qa::offline_answer(&context);
            answer.push_str("\n\nLuna 状态：");
            answer.push_str(&error);
            (answer, "offline-evidence", "deterministic", true)
        }
    };

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
            provider,
            model,
        )
    };
    let result = match persisted {
        Ok(result) => result,
        Err(error) => {
            let _ = on_event.send(qa::AnswerStreamEvent::Failed {
                request_id: request_id.clone(),
                code: "PERSIST_FAILED".to_string(),
                message: error.clone(),
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

fn execute_compile_request(
    state: &State<'_, AppState>,
    request: compile_center::StartCompileRequest,
    on_event: Channel<compile_center::CompileStreamEvent>,
    retry_of: Option<String>,
) -> Result<compile_center::CompileRunSummary, String> {
    let (root, db_path) = compile_repository_context(state)?;
    let run_id = Uuid::new_v4().to_string();
    let cancellation = Arc::new(AtomicBool::new(false));
    let mut cancellations = state
        .compile_cancellations
        .lock()
        .map_err(|_| "任务取消状态锁定失败".to_string())?;
    if cancellations.is_empty() {
        let recovery_connection =
            Connection::open(&db_path).map_err(|error| format!("打开任务数据库失败：{error}"))?;
        compile_center::db_schema(&recovery_connection)?;
        compile_center::recover_interrupted_runs(&recovery_connection)?;
    }
    cancellations.insert(run_id.clone(), cancellation.clone());
    drop(cancellations);
    let result = compile_center::execute_run(
        &db_path,
        &root,
        run_id.clone(),
        request,
        on_event,
        cancellation,
        retry_of,
    );
    if let Ok(mut cancellations) = state.compile_cancellations.lock() {
        cancellations.remove(&run_id);
    }
    result
}

#[tauri::command]
fn start_compile_run(
    request: compile_center::StartCompileRequest,
    on_event: Channel<compile_center::CompileStreamEvent>,
    state: State<'_, AppState>,
) -> Result<compile_center::CompileRunSummary, String> {
    execute_compile_request(&state, request, on_event, None)
}

#[tauri::command]
fn retry_compile_run(
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
    execute_compile_request(&state, request, on_event, Some(run_id))
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
fn rollback_compile_run(run_id: String, state: State<'_, AppState>) -> Result<String, String> {
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
    compile_center::rollback_run(connection, root, &run_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            if let Ok(data_dir) = app.path().app_local_data_dir() {
                let path_file = data_dir.join("repository.json");
                if let Ok(content) = fs::read_to_string(path_file) {
                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(path) = payload.get("path").and_then(|value| value.as_str()) {
                            if Path::new(path).exists() {
                                if let Ok(mut state) = app.state::<AppState>().repository.lock() {
                                    let opened = open_repository_state(
                                        &mut state,
                                        app.handle(),
                                        PathBuf::from(path),
                                    );
                                    if opened.as_ref().is_ok_and(|info| !info.indexed) {
                                        let root = state.root.clone();
                                        if let (Some(root), Some(connection)) =
                                            (root, state.db.as_mut())
                                        {
                                            if let Ok(stats) = rebuild_connection(connection, &root)
                                            {
                                                state.indexed_pages = stats.page_count;
                                            }
                                        }
                                    }
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
            list_chat_sessions,
            get_chat_session,
            create_chat_session,
            rename_chat_session,
            delete_chat_session,
            prepare_question,
            ask_luna,
            cancel_answer,
            get_compile_capabilities,
            list_compile_runs,
            get_compile_run,
            start_compile_run,
            retry_compile_run,
            cancel_compile_run,
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
        let mut matched = 0usize;
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
            let context = qa::prepare_question(&connection, &root, question, 20)
                .unwrap_or_else(|error| panic!("question failed: {question}: {error}"));
            let hit = context.evidence.iter().any(|item| {
                item.kind == "wiki"
                    && expected
                        .iter()
                        .any(|target| item.page_id.trim_end_matches(".md").ends_with(target))
            });
            if hit {
                matched += 1;
            } else {
                missed.push(format!(
                    "{} -> {:?}",
                    case["id"].as_str().unwrap_or("unknown"),
                    context
                        .evidence
                        .iter()
                        .filter(|item| item.kind == "wiki")
                        .map(|item| item.page_id.as_str())
                        .collect::<Vec<_>>()
                ));
            }
        }
        assert_eq!(
            matched,
            cases.len(),
            "每个固定问题至少召回一个预期 Wiki 证据；missed={missed:?}"
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
