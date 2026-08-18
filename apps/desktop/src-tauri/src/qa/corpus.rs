use super::markdown_parser::{parse_markdown, sha256_hex};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) const CORPUS_SCHEMA_VERSION: &str = "markdown-corpus-v2";
const ACTIVE_SNAPSHOT_KEY: &str = "markdown_corpus_active_snapshot";
const CORPUS_SCHEMA_KEY: &str = "markdown_corpus_schema_version";

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    Wiki,
    Paper,
    Book,
}

impl DocumentKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Wiki => "wiki",
            Self::Paper => "paper",
            Self::Book => "book",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentGranularity {
    Document,
    Section,
    Semantic,
}

impl ContentGranularity {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Section => "section",
            Self::Semantic => "semantic",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentRole {
    Abstract,
    ResearchBackground,
    ResearchMotivation,
    ResearchObjective,
    RelatedWork,
    ProblemDefinition,
    Model,
    Method,
    Algorithm,
    Theory,
    Proof,
    Experiment,
    Result,
    Limitation,
    Conclusion,
    Reference,
    GeneralContent,
}

impl ContentRole {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Abstract => "abstract",
            Self::ResearchBackground => "research_background",
            Self::ResearchMotivation => "research_motivation",
            Self::ResearchObjective => "research_objective",
            Self::RelatedWork => "related_work",
            Self::ProblemDefinition => "problem_definition",
            Self::Model => "model",
            Self::Method => "method",
            Self::Algorithm => "algorithm",
            Self::Theory => "theory",
            Self::Proof => "proof",
            Self::Experiment => "experiment",
            Self::Result => "result",
            Self::Limitation => "limitation",
            Self::Conclusion => "conclusion",
            Self::Reference => "reference",
            Self::GeneralContent => "general_content",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocator {
    pub document_id: String,
    pub block_id: String,
    pub heading_path: Vec<String>,
    pub markdown_path: String,
    pub line_start: Option<i64>,
    pub line_end: Option<i64>,
    pub content_hash: String,
    pub snapshot_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRecord {
    pub id: String,
    pub kind: DocumentKind,
    pub canonical_title: String,
    pub aliases: Vec<String>,
    pub authors: Vec<String>,
    pub year: String,
    pub tags: Vec<String>,
    pub markdown_path: String,
    pub provenance: Value,
    pub content_hash: String,
    pub snapshot_id: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContentBlock {
    pub id: String,
    pub document_id: String,
    pub parent_block_id: Option<String>,
    pub granularity: ContentGranularity,
    pub heading: String,
    pub heading_path: Vec<String>,
    pub role: ContentRole,
    pub ordinal: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub markdown_path: String,
    pub content: String,
    pub content_hash: String,
    pub embedding_text: String,
    pub snapshot_id: String,
    pub active: bool,
    pub locator: SourceLocator,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CorpusBuildStats {
    pub snapshot_id: String,
    pub document_count: usize,
    pub block_count: usize,
    pub inserted_or_updated_documents: usize,
    pub reused_documents: usize,
    pub deactivated_documents: usize,
    pub duplicate_paper_paths: usize,
}

#[derive(Debug)]
struct IndexedDocument {
    record: DocumentRecord,
    alias_sources: Vec<(String, String)>,
    blocks: Vec<ContentBlock>,
}

type BookMetadata = (Vec<(String, String)>, Vec<String>, Vec<String>);

pub(crate) fn db_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS documents_v2 (
              id TEXT PRIMARY KEY,
              kind TEXT NOT NULL,
              canonical_title TEXT NOT NULL,
              markdown_path TEXT NOT NULL,
              authors_json TEXT NOT NULL DEFAULT '[]',
              year TEXT NOT NULL DEFAULT '',
              tags_json TEXT NOT NULL DEFAULT '[]',
              provenance_json TEXT NOT NULL DEFAULT '{}',
              content_hash TEXT NOT NULL,
              snapshot_id TEXT NOT NULL,
              active INTEGER NOT NULL DEFAULT 1,
              updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS document_aliases_v2 (
              document_id TEXT NOT NULL,
              alias TEXT NOT NULL,
              normalized_alias TEXT NOT NULL,
              language TEXT NOT NULL DEFAULT '',
              source TEXT NOT NULL DEFAULT '',
              PRIMARY KEY(document_id, normalized_alias),
              FOREIGN KEY(document_id) REFERENCES documents_v2(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS content_blocks_v2 (
              id TEXT PRIMARY KEY,
              document_id TEXT NOT NULL,
              parent_block_id TEXT,
              granularity TEXT NOT NULL,
              heading TEXT NOT NULL,
              heading_path_json TEXT NOT NULL,
              role TEXT NOT NULL,
              ordinal INTEGER NOT NULL,
              line_start INTEGER,
              line_end INTEGER,
              markdown_path TEXT NOT NULL,
              content TEXT NOT NULL,
              content_hash TEXT NOT NULL,
              embedding_text TEXT NOT NULL,
              locator_json TEXT NOT NULL,
              snapshot_id TEXT NOT NULL,
              active INTEGER NOT NULL DEFAULT 1,
              FOREIGN KEY(document_id) REFERENCES documents_v2(id) ON DELETE CASCADE
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS content_blocks_fts_v2 USING fts5(
              block_id UNINDEXED,
              document_id UNINDEXED,
              canonical_title,
              aliases,
              heading_path,
              role,
              content
            );
            CREATE INDEX IF NOT EXISTS idx_documents_v2_kind_active
              ON documents_v2(kind, active);
            CREATE INDEX IF NOT EXISTS idx_aliases_v2_normalized
              ON document_aliases_v2(normalized_alias);
            CREATE INDEX IF NOT EXISTS idx_blocks_v2_document_active
              ON content_blocks_v2(document_id, active);
            CREATE INDEX IF NOT EXISTS idx_blocks_v2_role_granularity
              ON content_blocks_v2(role, granularity, active);
            ",
        )
        .map_err(|error| format!("初始化 Markdown 语料 v2 数据库失败：{error}"))?;
    Ok(())
}

fn normalized_alias(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn alias_language(value: &str) -> &'static str {
    if value
        .chars()
        .any(|character| ('\u{4e00}'..='\u{9fff}').contains(&character))
    {
        "zh"
    } else {
        "en"
    }
}

fn parse_string_list(value: &str) -> Vec<String> {
    if let Ok(items) = serde_json::from_str::<Vec<Value>>(value) {
        return items
            .into_iter()
            .filter_map(|item| item.as_str().map(str::to_string))
            .collect();
    }
    value
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|item| !item.is_empty())
        .collect()
}

fn repository_relative_path(root: &Path, path: &Path) -> Result<String, String> {
    let canonical_root = root
        .canonicalize()
        .map_err(|error| format!("解析知识库根目录失败：{error}"))?;
    let canonical_path = path
        .canonicalize()
        .map_err(|error| format!("解析 Markdown 路径失败：{} ({error})", path.display()))?;
    if !canonical_path.starts_with(&canonical_root) || !canonical_path.is_file() {
        return Err(format!("Markdown 路径必须位于知识库内：{}", path.display()));
    }
    canonical_path
        .strip_prefix(canonical_root)
        .map(|value| value.to_string_lossy().replace('\\', "/"))
        .map_err(|error| format!("计算 Markdown 相对路径失败：{error}"))
}

fn resolve_relative_markdown(root: &Path, relative: &str) -> Option<PathBuf> {
    let normalized = relative.replace('\\', "/");
    if normalized.is_empty()
        || normalized.starts_with('/')
        || normalized.starts_with("//")
        || normalized.split('/').any(|part| part == "..")
        || (normalized.len() > 1 && normalized.as_bytes().get(1) == Some(&b':'))
    {
        return None;
    }
    let root = root.canonicalize().ok()?;
    let candidate = root.join(normalized).canonicalize().ok()?;
    (candidate.starts_with(&root) && candidate.is_file()).then_some(candidate)
}

fn frontmatter_map(value: &str) -> HashMap<String, String> {
    serde_json::from_str(value).unwrap_or_default()
}

fn aliases_from_fields(fields: &HashMap<String, String>, title: &str) -> Vec<(String, String)> {
    let mut aliases = vec![(title.to_string(), "title".to_string())];
    for alias in fields
        .get("aliases")
        .map(|value| parse_string_list(value))
        .unwrap_or_default()
    {
        aliases.push((alias, "frontmatter".to_string()));
    }
    let mut seen = HashSet::new();
    aliases.retain(|(alias, _)| {
        let normalized = normalized_alias(alias);
        !normalized.is_empty() && seen.insert(normalized)
    });
    aliases
}

fn wikilink_aliases(body: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("[[") {
        let after = &rest[start + 2..];
        let Some(end) = after.find("]]") else {
            break;
        };
        let value = &after[..end];
        if let Some((target, alias)) = value.split_once('|') {
            let target = target.split('#').next().unwrap_or("").trim();
            let alias = alias.trim();
            if !target.is_empty() && !alias.is_empty() {
                result.push((target.to_string(), alias.to_string()));
            }
        }
        rest = &after[end + 2..];
    }
    result
}

fn resolve_wikilink_page_id(
    source_page_id: &str,
    target: &str,
    known_ids: &HashSet<String>,
) -> Option<String> {
    let normalized = target
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_start_matches("wiki/")
        .trim_end_matches(".md")
        .to_string();
    if known_ids.contains(&normalized) {
        return Some(normalized);
    }
    let target_stem = normalized.rsplit('/').next()?;
    let mut stem_matches = known_ids
        .iter()
        .filter(|id| id.rsplit('/').next() == Some(target_stem));
    let first = stem_matches.next()?.clone();
    if stem_matches.next().is_none() {
        return Some(first);
    }
    let base = source_page_id
        .rsplit_once('/')
        .map(|(value, _)| value)
        .unwrap_or("");
    let candidate = format!("{base}/{normalized}");
    let mut components: Vec<String> = Vec::new();
    for component in candidate.split('/') {
        match component {
            "" | "." => {}
            ".." => {
                components.pop();
            }
            value => components.push(value.to_string()),
        }
    }
    let resolved = components.join("/");
    known_ids.contains(&resolved).then_some(resolved)
}

fn document_hash(
    kind: DocumentKind,
    title: &str,
    aliases: &[(String, String)],
    content: &str,
) -> String {
    sha256_hex(format!(
        "{}\n{}\n{}\n{}",
        kind.as_str(),
        title,
        aliases
            .iter()
            .map(|(alias, source)| format!("{source}:{alias}"))
            .collect::<Vec<_>>()
            .join("\n"),
        content
    ))
}

#[allow(clippy::too_many_arguments)]
fn make_document(
    id: String,
    kind: DocumentKind,
    title: String,
    alias_sources: Vec<(String, String)>,
    authors: Vec<String>,
    year: String,
    tags: Vec<String>,
    markdown_path: String,
    provenance: Value,
    content: &str,
) -> IndexedDocument {
    let aliases = alias_sources
        .iter()
        .map(|(alias, _)| alias.clone())
        .collect::<Vec<_>>();
    let content_hash = document_hash(kind, &title, &alias_sources, content);
    let snapshot_id = format!("sha256:{}", &content_hash[..32]);
    let record = DocumentRecord {
        id: id.clone(),
        kind,
        canonical_title: title.clone(),
        aliases: aliases.clone(),
        authors,
        year,
        tags,
        markdown_path: markdown_path.clone(),
        provenance,
        content_hash,
        snapshot_id: snapshot_id.clone(),
        active: true,
    };
    let blocks = parse_markdown(
        &id,
        kind.as_str(),
        &title,
        &aliases,
        &markdown_path,
        content,
        &snapshot_id,
        true,
    );
    IndexedDocument {
        record,
        alias_sources,
        blocks,
    }
}

fn discover_wiki_and_papers(
    connection: &Connection,
    root: &Path,
) -> Result<(Vec<IndexedDocument>, usize), String> {
    let mut statement = connection
        .prepare(
            "SELECT id,page_type,title,year,source_path,frontmatter,body FROM pages ORDER BY id",
        )
        .map_err(|error| format!("准备 Markdown 页面发现失败：{error}"))?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })
        .map_err(|error| format!("读取 Markdown 页面失败：{error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析 Markdown 页面失败：{error}"))?;

    let known_ids = rows
        .iter()
        .map(|(page_id, _, _, _, _, _, _)| page_id.clone())
        .collect::<HashSet<_>>();
    let mut linked_aliases: HashMap<String, Vec<String>> = HashMap::new();
    for (source_page_id, _, _, _, _, _, body) in &rows {
        for (target, alias) in wikilink_aliases(body) {
            if let Some(target_id) = resolve_wikilink_page_id(source_page_id, &target, &known_ids) {
                linked_aliases.entry(target_id).or_default().push(alias);
            }
        }
    }
    let mut documents = Vec::new();
    let mut seen_papers: HashMap<String, String> = HashMap::new();
    let mut duplicate_paper_paths = 0usize;
    for (page_id, page_type, title, year, source_path, frontmatter, body) in rows {
        let fields = frontmatter_map(&frontmatter);
        let source = PathBuf::from(&source_path);
        let markdown_path = repository_relative_path(root, &source)?;
        let mut alias_sources = aliases_from_fields(&fields, &title);
        for alias in linked_aliases.remove(&page_id).unwrap_or_default() {
            alias_sources.push((alias, "wikilink".to_string()));
        }
        let mut seen_aliases = HashSet::new();
        alias_sources.retain(|(alias, _)| seen_aliases.insert(normalized_alias(alias)));
        let wiki = make_document(
            format!("wiki:{page_id}"),
            DocumentKind::Wiki,
            title.clone(),
            alias_sources.clone(),
            fields
                .get("authors")
                .map(|value| parse_string_list(value))
                .unwrap_or_default(),
            year.clone(),
            fields
                .get("tags")
                .map(|value| parse_string_list(value))
                .unwrap_or_default(),
            markdown_path,
            json!({"pageId": page_id, "pageType": page_type}),
            &body,
        );
        documents.push(wiki);

        if page_type != "source"
            || fields
                .get("source_type")
                .is_some_and(|value| value == "book")
        {
            continue;
        }
        let Some(raw_path) = fields
            .get("raw_md")
            .and_then(|relative| resolve_relative_markdown(root, relative))
        else {
            continue;
        };
        let raw_relative = repository_relative_path(root, &raw_path)?;
        let raw_content = fs::read_to_string(&raw_path)
            .map_err(|error| format!("读取论文 Markdown 失败：{} ({error})", raw_path.display()))?;
        let raw_hash = sha256_hex(&raw_content);
        if let Some(existing_id) = seen_papers.get(&raw_hash) {
            duplicate_paper_paths += 1;
            if existing_id != &page_id {
                continue;
            }
        } else {
            seen_papers.insert(raw_hash, page_id.clone());
        }
        documents.push(make_document(
            format!("paper:{page_id}"),
            DocumentKind::Paper,
            title,
            alias_sources,
            fields
                .get("authors")
                .map(|value| parse_string_list(value))
                .unwrap_or_default(),
            year,
            fields
                .get("paper_keywords")
                .map(|value| parse_string_list(value))
                .unwrap_or_default(),
            raw_relative,
            json!({"sourcePageId": page_id, "rawMarkdown": raw_path.to_string_lossy()}),
            &raw_content,
        ));
    }
    Ok((documents, duplicate_paper_paths))
}

fn discover_books(connection: &Connection, root: &Path) -> Result<Vec<IndexedDocument>, String> {
    let mut metadata_by_title: HashMap<String, BookMetadata> = HashMap::new();
    let mut page_statement = connection
        .prepare("SELECT title,frontmatter FROM pages WHERE page_type='source'")
        .map_err(|error| format!("准备专著别名发现失败：{error}"))?;
    let page_rows = page_statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| format!("读取专著别名失败：{error}"))?;
    for row in page_rows {
        let (title, frontmatter) = row.map_err(|error| format!("解析专著别名失败：{error}"))?;
        let fields = frontmatter_map(&frontmatter);
        if fields
            .get("source_type")
            .is_some_and(|value| value == "book")
        {
            metadata_by_title.insert(
                title.to_lowercase(),
                (
                    aliases_from_fields(&fields, &title),
                    fields
                        .get("authors")
                        .map(|value| parse_string_list(value))
                        .unwrap_or_default(),
                    fields
                        .get("tags")
                        .map(|value| parse_string_list(value))
                        .unwrap_or_default(),
                ),
            );
        }
    }

    let mut book_statement = connection
        .prepare("SELECT id,title,year,source_path FROM books ORDER BY id")
        .map_err(|error| format!("准备专著发现失败：{error}"))?;
    let books = book_statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|error| format!("读取专著失败：{error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析专著失败：{error}"))?;
    let mut documents = Vec::new();
    for (book_id, title, year, source_path) in books {
        let (alias_sources, authors, tags) = metadata_by_title
            .remove(&title.to_lowercase())
            .unwrap_or_else(|| {
                (
                    vec![(title.clone(), "title".to_string())],
                    Vec::new(),
                    Vec::new(),
                )
            });
        let aliases = alias_sources
            .iter()
            .map(|(alias, _)| alias.clone())
            .collect::<Vec<_>>();
        let source_relative = repository_relative_path(root, Path::new(&source_path))?;
        let mut chapter_statement = connection
            .prepare(
                "SELECT id,title,markdown_path,chapter_number FROM book_chapters WHERE book_id=?1 ORDER BY chapter_number,id",
            )
            .map_err(|error| format!("准备专著章节发现失败：{error}"))?;
        let chapters = chapter_statement
            .query_map([&book_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            })
            .map_err(|error| format!("读取专著章节失败：{error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("解析专著章节失败：{error}"))?;
        let mut combined = String::new();
        let mut chapter_payloads = Vec::new();
        for (chapter_id, chapter_title, path, number) in chapters {
            let path = PathBuf::from(path);
            let relative = repository_relative_path(root, &path)?;
            let content = fs::read_to_string(&path)
                .map_err(|error| format!("读取专著章节失败：{} ({error})", path.display()))?;
            combined.push_str(&format!("\n{chapter_id}\n{}\n", sha256_hex(&content)));
            chapter_payloads.push((chapter_id, chapter_title, relative, number, content));
        }
        let content_hash = document_hash(DocumentKind::Book, &title, &alias_sources, &combined);
        let snapshot_id = format!("sha256:{}", &content_hash[..32]);
        let mut blocks = Vec::new();
        let catalog = chapter_payloads
            .iter()
            .map(|(_, chapter_title, _, number, _)| format!("Chapter {number}: {chapter_title}"))
            .collect::<Vec<_>>()
            .join("\n");
        blocks.extend(parse_markdown(
            &format!("book:{book_id}"),
            DocumentKind::Book.as_str(),
            &title,
            &aliases,
            &source_relative,
            &format!("# {title}\n\n## Chapters\n{catalog}"),
            &snapshot_id,
            true,
        ));
        for (_, _, relative, _, content) in chapter_payloads {
            blocks.extend(parse_markdown(
                &format!("book:{book_id}"),
                DocumentKind::Book.as_str(),
                &title,
                &aliases,
                &relative,
                &content,
                &snapshot_id,
                false,
            ));
        }
        documents.push(IndexedDocument {
            record: DocumentRecord {
                id: format!("book:{book_id}"),
                kind: DocumentKind::Book,
                canonical_title: title,
                aliases,
                authors,
                year,
                tags,
                markdown_path: source_relative,
                provenance: json!({"bookId": book_id, "chapterCount": blocks.iter().filter(|block| block.granularity == ContentGranularity::Section).count()}),
                content_hash,
                snapshot_id,
                active: true,
            },
            alias_sources,
            blocks,
        });
    }
    Ok(documents)
}

fn corpus_snapshot(documents: &[IndexedDocument]) -> String {
    let mut entries = documents
        .iter()
        .map(|document| format!("{}:{}", document.record.id, document.record.content_hash))
        .collect::<Vec<_>>();
    entries.sort();
    format!("sha256:{}", sha256_hex(entries.join("\n")))
}

fn delete_document_blocks(connection: &Connection, document_id: &str) -> Result<(), String> {
    connection
        .execute(
            "DELETE FROM content_blocks_fts_v2 WHERE block_id IN (SELECT id FROM content_blocks_v2 WHERE document_id=?1)",
            [document_id],
        )
        .map_err(|error| format!("清理 Markdown 块 FTS 失败：{error}"))?;
    connection
        .execute(
            "DELETE FROM content_blocks_v2 WHERE document_id=?1",
            [document_id],
        )
        .map_err(|error| format!("清理 Markdown 块失败：{error}"))?;
    connection
        .execute(
            "DELETE FROM document_aliases_v2 WHERE document_id=?1",
            [document_id],
        )
        .map_err(|error| format!("清理 Markdown 别名失败：{error}"))?;
    Ok(())
}

fn write_document(connection: &Connection, document: &IndexedDocument) -> Result<(), String> {
    let record = &document.record;
    connection
        .execute(
            "INSERT INTO documents_v2(id,kind,canonical_title,markdown_path,authors_json,year,tags_json,provenance_json,content_hash,snapshot_id,active,updated_at)
             VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1,CURRENT_TIMESTAMP)
             ON CONFLICT(id) DO UPDATE SET kind=excluded.kind,canonical_title=excluded.canonical_title,markdown_path=excluded.markdown_path,authors_json=excluded.authors_json,year=excluded.year,tags_json=excluded.tags_json,provenance_json=excluded.provenance_json,content_hash=excluded.content_hash,snapshot_id=excluded.snapshot_id,active=1,updated_at=CURRENT_TIMESTAMP",
            params![
                record.id,
                record.kind.as_str(),
                record.canonical_title,
                record.markdown_path,
                serde_json::to_string(&record.authors).unwrap_or_else(|_| "[]".to_string()),
                record.year,
                serde_json::to_string(&record.tags).unwrap_or_else(|_| "[]".to_string()),
                record.provenance.to_string(),
                record.content_hash,
                record.snapshot_id,
            ],
        )
        .map_err(|error| format!("写入 Markdown 文档失败：{error}"))?;
    for (alias, source) in &document.alias_sources {
        connection
            .execute(
                "INSERT OR REPLACE INTO document_aliases_v2(document_id,alias,normalized_alias,language,source) VALUES(?1,?2,?3,?4,?5)",
                params![record.id, alias, normalized_alias(alias), alias_language(alias), source],
            )
            .map_err(|error| format!("写入 Markdown 别名失败：{error}"))?;
    }
    let aliases = record.aliases.join(" ");
    for block in &document.blocks {
        let heading_path_json =
            serde_json::to_string(&block.heading_path).unwrap_or_else(|_| "[]".to_string());
        let locator_json =
            serde_json::to_string(&block.locator).unwrap_or_else(|_| "{}".to_string());
        connection
            .execute(
                "INSERT INTO content_blocks_v2(id,document_id,parent_block_id,granularity,heading,heading_path_json,role,ordinal,line_start,line_end,markdown_path,content,content_hash,embedding_text,locator_json,snapshot_id,active)
                 VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,1)",
                params![
                    block.id,
                    block.document_id,
                    block.parent_block_id,
                    block.granularity.as_str(),
                    block.heading,
                    heading_path_json,
                    block.role.as_str(),
                    block.ordinal as i64,
                    block.line_start as i64,
                    block.line_end as i64,
                    block.markdown_path,
                    block.content,
                    block.content_hash,
                    block.embedding_text,
                    locator_json,
                    block.snapshot_id,
                ],
            )
            .map_err(|error| format!("写入 Markdown 内容块失败：{error}"))?;
        connection
            .execute(
                "INSERT INTO content_blocks_fts_v2(block_id,document_id,canonical_title,aliases,heading_path,role,content) VALUES(?1,?2,?3,?4,?5,?6,?7)",
                params![
                    block.id,
                    block.document_id,
                    record.canonical_title,
                    aliases,
                    block.heading_path.join(" / "),
                    block.role.as_str(),
                    block.content,
                ],
            )
            .map_err(|error| format!("写入 Markdown 内容块 FTS 失败：{error}"))?;
    }
    Ok(())
}

pub(crate) fn sync_repository(
    connection: &Connection,
    root: &Path,
) -> Result<CorpusBuildStats, String> {
    db_schema(connection)?;
    let (mut documents, duplicate_paper_paths) = discover_wiki_and_papers(connection, root)?;
    documents.extend(discover_books(connection, root)?);
    documents.sort_by(|left, right| left.record.id.cmp(&right.record.id));
    let snapshot_id = corpus_snapshot(&documents);
    let existing = {
        let mut statement = connection
            .prepare("SELECT id,content_hash,active FROM documents_v2")
            .map_err(|error| format!("准备读取旧 Markdown 文档失败：{error}"))?;
        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, bool>(2)?,
                ))
            })
            .map_err(|error| format!("读取旧 Markdown 文档失败：{error}"))?;
        rows.map(|row| row.map(|(id, hash, active)| (id, (hash, active))))
            .collect::<Result<HashMap<_, _>, _>>()
            .map_err(|error| format!("解析旧 Markdown 文档失败：{error}"))?
    };
    let active_ids = documents
        .iter()
        .map(|document| document.record.id.clone())
        .collect::<HashSet<_>>();
    let mut stats = CorpusBuildStats {
        snapshot_id: snapshot_id.clone(),
        document_count: documents.len(),
        block_count: documents.iter().map(|document| document.blocks.len()).sum(),
        duplicate_paper_paths,
        ..CorpusBuildStats::default()
    };
    for document in &documents {
        if existing
            .get(&document.record.id)
            .is_some_and(|(hash, active)| hash == &document.record.content_hash && *active)
        {
            stats.reused_documents += 1;
            continue;
        }
        delete_document_blocks(connection, &document.record.id)?;
        write_document(connection, document)?;
        stats.inserted_or_updated_documents += 1;
    }
    for (document_id, (_, active)) in existing {
        if active && !active_ids.contains(&document_id) {
            connection
                .execute(
                    "DELETE FROM content_blocks_fts_v2 WHERE block_id IN (SELECT id FROM content_blocks_v2 WHERE document_id=?1)",
                    [&document_id],
                )
                .map_err(|error| format!("停用 Markdown 内容块 FTS 失败：{error}"))?;
            connection
                .execute(
                    "UPDATE content_blocks_v2 SET active=0 WHERE document_id=?1",
                    [&document_id],
                )
                .map_err(|error| format!("停用 Markdown 内容块失败：{error}"))?;
            connection
                .execute(
                    "UPDATE documents_v2 SET active=0 WHERE id=?1",
                    [&document_id],
                )
                .map_err(|error| format!("停用 Markdown 文档失败：{error}"))?;
            stats.deactivated_documents += 1;
        }
    }
    for (key, value) in [
        (CORPUS_SCHEMA_KEY, CORPUS_SCHEMA_VERSION),
        (ACTIVE_SNAPSHOT_KEY, snapshot_id.as_str()),
    ] {
        connection
            .execute(
                "INSERT INTO repository_metadata(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                params![key, value],
            )
            .map_err(|error| format!("写入 Markdown 语料元数据失败：{error}"))?;
    }
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn schema(connection: &Connection) {
        connection
            .execute_batch(
                "CREATE TABLE pages(id TEXT PRIMARY KEY,page_type TEXT,title TEXT,year TEXT,source_path TEXT,frontmatter TEXT,body TEXT);
                 CREATE TABLE books(id TEXT PRIMARY KEY,title TEXT,year TEXT,source_path TEXT);
                 CREATE TABLE book_chapters(id TEXT PRIMARY KEY,book_id TEXT,title TEXT,markdown_path TEXT,chapter_number INTEGER);
                 CREATE TABLE repository_metadata(key TEXT PRIMARY KEY,value TEXT NOT NULL);",
            )
            .unwrap();
        db_schema(connection).unwrap();
    }

    #[test]
    fn sync_reuses_unchanged_documents_and_supports_markdown_only_books() {
        let directory = tempdir().unwrap();
        let root = directory.path();
        fs::create_dir_all(root.join("wiki/sources")).unwrap();
        fs::create_dir_all(root.join("raw/canonical/book/chapters")).unwrap();
        let source = root.join("wiki/sources/book.md");
        fs::write(&source, "# Approximation Algorithms\nBook source").unwrap();
        let chapter = root.join("raw/canonical/book/chapters/ch-01.md");
        fs::write(&chapter, "# Chapter 1\n\n## Algorithm\nBody").unwrap();
        let connection = Connection::open_in_memory().unwrap();
        schema(&connection);
        connection.execute(
            "INSERT INTO pages VALUES('sources/book','source','Approximation Algorithms','2001',?1,?2,?3)",
            params![source.to_string_lossy(), r#"{"source_type":"book","aliases":"[\"近似算法\"]"}"#, "# Approximation Algorithms\nBook source"],
        ).unwrap();
        connection
            .execute(
                "INSERT INTO books VALUES('book','Approximation Algorithms','2001',?1)",
                [source.to_string_lossy().to_string()],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO book_chapters VALUES('book:ch-01','book','Chapter 1',?1,1)",
                [chapter.to_string_lossy().to_string()],
            )
            .unwrap();
        let first = sync_repository(&connection, root).unwrap();
        let second = sync_repository(&connection, root).unwrap();
        assert_eq!(first.inserted_or_updated_documents, 2);
        assert_eq!(second.inserted_or_updated_documents, 0);
        assert_eq!(second.reused_documents, 2);
        connection
            .execute(
                "UPDATE pages SET body='# Approximation Algorithms\nChanged source' WHERE id='sources/book'",
                [],
            )
            .unwrap();
        let changed = sync_repository(&connection, root).unwrap();
        assert_eq!(changed.inserted_or_updated_documents, 1);
        assert_eq!(changed.reused_documents, 1);
        let alias: String = connection.query_row(
            "SELECT alias FROM document_aliases_v2 WHERE document_id='book:book' AND normalized_alias=?1",
            [normalized_alias("近似算法")],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(alias, "近似算法");
        let semantic_count: i64 = connection.query_row(
            "SELECT COUNT(*) FROM content_blocks_v2 WHERE document_id='book:book' AND granularity='semantic'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert!(semantic_count > 0);
        connection
            .execute("DELETE FROM pages WHERE id='sources/book'", [])
            .unwrap();
        let removed = sync_repository(&connection, root).unwrap();
        assert_eq!(removed.deactivated_documents, 1);
        let active_wiki: bool = connection
            .query_row(
                "SELECT active FROM documents_v2 WHERE id='wiki:sources/book'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(!active_wiki);
    }
}
