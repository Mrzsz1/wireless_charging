#![cfg_attr(test, allow(dead_code))]

use super::{check_cancelled, compact, Candidate};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::io::{Cursor, Read, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, OnceLock};

const MODEL_NAME: &str = "intfloat/multilingual-e5-small";
const MAX_DOCUMENTS: usize = 8_000;
const MAX_EMBED_CHARS: usize = 2_400;
const EMBED_BATCH_SIZE: usize = 32;
const RESULT_LIMIT: usize = 30;
const MIN_COSINE_SIMILARITY: f32 = 0.60;
const MAX_DISTANCE_FROM_BEST: f32 = 0.14;
#[cfg(target_os = "windows")]
const ORT_RUNTIME_URL: &str =
    "https://github.com/microsoft/onnxruntime/releases/download/v1.20.1/onnxruntime-win-x64-1.20.1.zip";
#[cfg(target_os = "windows")]
const ORT_RUNTIME_SHA256: &str = "78d447051e48bd2e1e778bba378bec4ece11191c9e538cf7b2c4a4565e8f5581";
#[cfg(target_os = "windows")]
const ORT_ARCHIVE_LIMIT_BYTES: usize = 80 * 1024 * 1024;

static SEMANTIC_STATE: OnceLock<Mutex<SemanticState>> = OnceLock::new();

#[derive(Clone)]
struct SemanticDocument {
    key: String,
    kind: String,
    tier: String,
    title: String,
    body: String,
    page_id: String,
    page_type: String,
    source_path: String,
    wikilink: String,
    book_id: String,
    chapter_id: String,
    physical_page_start: Option<i64>,
    physical_page_end: Option<i64>,
    markdown_path: String,
    pdf_path: String,
    node_id: String,
    source_location: String,
}

impl SemanticDocument {
    fn passage(&self) -> String {
        let text = format!("passage: {}\n{}", self.title, self.body);
        text.chars().take(MAX_EMBED_CHARS).collect()
    }

    fn candidate(&self, score: f32) -> Candidate {
        Candidate {
            kind: self.kind.clone(),
            tier: self.tier.clone(),
            title: self.title.clone(),
            snippet: compact(&self.body, if self.kind == "paper" { 1_200 } else { 480 }),
            score: score as f64,
            page_id: self.page_id.clone(),
            page_type: self.page_type.clone(),
            source_path: self.source_path.clone(),
            wikilink: self.wikilink.clone(),
            book_id: self.book_id.clone(),
            chapter_id: self.chapter_id.clone(),
            physical_page_start: self.physical_page_start,
            physical_page_end: self.physical_page_end,
            markdown_path: self.markdown_path.clone(),
            pdf_path: self.pdf_path.clone(),
            node_id: self.node_id.clone(),
            source_location: self.source_location.clone(),
            relation: String::new(),
            retrieval_reason: format!(
                "本地多语言语义向量命中；模型 {MODEL_NAME}；cosine={score:.3}"
            ),
        }
    }
}

struct CachedCorpus {
    repository_key: String,
    snapshot_id: String,
    documents: Vec<SemanticDocument>,
    embeddings: Vec<Vec<f32>>,
}

#[derive(Default)]
struct SemanticState {
    model: Option<TextEmbedding>,
    model_initialization_failed: bool,
    corpus: Option<CachedCorpus>,
}

pub(super) fn semantic_candidates(
    connection: &Connection,
    root: &Path,
    question: &str,
    cancelled: Option<&AtomicBool>,
) -> Result<Vec<Candidate>, String> {
    check_cancelled(cancelled)?;
    let state = SEMANTIC_STATE.get_or_init(|| Mutex::new(SemanticState::default()));
    let Ok(mut state) = state.lock() else {
        return Ok(Vec::new());
    };
    match state.search(connection, root, question, cancelled) {
        Ok(candidates) => Ok(candidates),
        Err(SemanticFailure::Cancelled) => Err("QUESTION_CANCELLED: 用户停止了问答".to_string()),
        Err(SemanticFailure::Unavailable) => Ok(Vec::new()),
    }
}

#[derive(Debug)]
enum SemanticFailure {
    Cancelled,
    Unavailable,
}

impl SemanticState {
    fn search(
        &mut self,
        connection: &Connection,
        root: &Path,
        question: &str,
        cancelled: Option<&AtomicBool>,
    ) -> Result<Vec<Candidate>, SemanticFailure> {
        ensure_not_cancelled(cancelled)?;
        let repository_key = repository_key(root);
        let snapshot_id = super::context::index_snapshot_id(connection, root);
        let cache_matches = self.corpus.as_ref().is_some_and(|corpus| {
            corpus.repository_key == repository_key && corpus.snapshot_id == snapshot_id
        });
        if !cache_matches {
            let documents = load_documents(connection).map_err(|_| SemanticFailure::Unavailable)?;
            if documents.is_empty() {
                self.corpus = Some(CachedCorpus {
                    repository_key,
                    snapshot_id,
                    documents,
                    embeddings: Vec::new(),
                });
                return Ok(Vec::new());
            }
            ensure_embedding_table(connection).map_err(|_| SemanticFailure::Unavailable)?;
            let persisted = load_persisted_embeddings(connection, &repository_key, &snapshot_id)
                .map_err(|_| SemanticFailure::Unavailable)?;
            let mut embeddings = vec![Vec::new(); documents.len()];
            let missing = documents
                .iter()
                .enumerate()
                .filter_map(|(index, document)| {
                    if let Some(vector) = persisted.get(&document.key) {
                        embeddings[index] = vector.clone();
                        None
                    } else {
                        Some(index)
                    }
                })
                .collect::<Vec<_>>();
            if !missing.is_empty() {
                self.ensure_model()?;
                let model = self.model.as_mut().ok_or(SemanticFailure::Unavailable)?;
                for indices in missing.chunks(EMBED_BATCH_SIZE) {
                    ensure_not_cancelled(cancelled)?;
                    let passages = indices
                        .iter()
                        .map(|index| documents[*index].passage())
                        .collect::<Vec<_>>();
                    let batch = model
                        .embed(passages, Some(EMBED_BATCH_SIZE))
                        .map_err(|_| SemanticFailure::Unavailable)?;
                    if batch.len() != indices.len() {
                        return Err(SemanticFailure::Unavailable);
                    }
                    for (index, vector) in indices.iter().zip(batch) {
                        embeddings[*index] = vector;
                    }
                }
                persist_embeddings(
                    connection,
                    &repository_key,
                    &snapshot_id,
                    &documents,
                    &embeddings,
                    &missing,
                )
                .map_err(|_| SemanticFailure::Unavailable)?;
            }
            if embeddings.iter().any(Vec::is_empty) {
                return Err(SemanticFailure::Unavailable);
            }
            self.corpus = Some(CachedCorpus {
                repository_key,
                snapshot_id,
                documents,
                embeddings,
            });
        }

        self.ensure_model()?;
        ensure_not_cancelled(cancelled)?;
        let query = format!("query: {}", question.trim());
        let query_embedding = self
            .model
            .as_mut()
            .ok_or(SemanticFailure::Unavailable)?
            .embed(vec![query], Some(1))
            .map_err(|_| SemanticFailure::Unavailable)?
            .pop()
            .ok_or(SemanticFailure::Unavailable)?;
        ensure_not_cancelled(cancelled)?;
        let corpus = self.corpus.as_ref().ok_or(SemanticFailure::Unavailable)?;
        Ok(rank_candidates(
            &corpus.documents,
            &corpus.embeddings,
            &query_embedding,
            RESULT_LIMIT,
        ))
    }

    fn ensure_model(&mut self) -> Result<(), SemanticFailure> {
        if self.model.is_some() {
            return Ok(());
        }
        if self.model_initialization_failed {
            return Err(SemanticFailure::Unavailable);
        }
        let cache_dir = model_cache_dir();
        if std::fs::create_dir_all(&cache_dir).is_err() {
            self.model_initialization_failed = true;
            return Err(SemanticFailure::Unavailable);
        }
        if prepare_onnx_runtime(&cache_dir).is_err() {
            self.model_initialization_failed = true;
            return Err(SemanticFailure::Unavailable);
        }
        let options = InitOptions::new(EmbeddingModel::MultilingualE5Small)
            .with_cache_dir(cache_dir)
            .with_max_length(512)
            .with_show_download_progress(false);
        match catch_unwind(AssertUnwindSafe(|| TextEmbedding::try_new(options))) {
            Ok(Ok(model)) => {
                self.model = Some(model);
                Ok(())
            }
            Ok(Err(_)) | Err(_) => {
                self.model_initialization_failed = true;
                Err(SemanticFailure::Unavailable)
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn prepare_onnx_runtime(cache_dir: &Path) -> Result<(), ()> {
    let runtime_path = cache_dir.join("onnxruntime-1.20.1").join("onnxruntime.dll");
    if !runtime_path.is_file() {
        let response = reqwest::blocking::Client::builder()
            .connect_timeout(std::time::Duration::from_secs(15))
            .timeout(std::time::Duration::from_secs(180))
            .build()
            .map_err(|_| ())?
            .get(ORT_RUNTIME_URL)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .map_err(|_| ())?;
        if response
            .content_length()
            .is_some_and(|length| length as usize > ORT_ARCHIVE_LIMIT_BYTES)
        {
            return Err(());
        }
        let archive_bytes = response.bytes().map_err(|_| ())?.to_vec();
        if archive_bytes.len() > ORT_ARCHIVE_LIMIT_BYTES
            || format!("{:x}", Sha256::digest(&archive_bytes)) != ORT_RUNTIME_SHA256
        {
            return Err(());
        }
        let mut archive = zip::ZipArchive::new(Cursor::new(archive_bytes)).map_err(|_| ())?;
        let entry_index = (0..archive.len())
            .find(|index| {
                archive
                    .by_index(*index)
                    .map(|entry| entry.name().ends_with("/lib/onnxruntime.dll"))
                    .unwrap_or(false)
            })
            .ok_or(())?;
        let mut entry = archive.by_index(entry_index).map_err(|_| ())?;
        if entry.size() > 32 * 1024 * 1024 {
            return Err(());
        }
        let parent = runtime_path.parent().ok_or(())?;
        std::fs::create_dir_all(parent).map_err(|_| ())?;
        let temporary = runtime_path.with_extension("dll.tmp");
        let mut output = std::fs::File::create(&temporary).map_err(|_| ())?;
        let mut copied = 0_u64;
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            let count = entry.read(&mut buffer).map_err(|_| ())?;
            if count == 0 {
                break;
            }
            copied += count as u64;
            if copied > 32 * 1024 * 1024 {
                let _ = std::fs::remove_file(&temporary);
                return Err(());
            }
            output.write_all(&buffer[..count]).map_err(|_| ())?;
        }
        output.flush().map_err(|_| ())?;
        std::fs::rename(&temporary, &runtime_path).map_err(|_| ())?;
    }
    env::set_var("ORT_DYLIB_PATH", &runtime_path);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn prepare_onnx_runtime(_cache_dir: &Path) -> Result<(), ()> {
    // Linux/macOS packages may provide the ONNX Runtime dynamic library. The
    // semantic channel remains fail-soft when it is absent.
    Ok(())
}

fn ensure_not_cancelled(cancelled: Option<&AtomicBool>) -> Result<(), SemanticFailure> {
    check_cancelled(cancelled).map_err(|_| SemanticFailure::Cancelled)
}

fn model_cache_dir() -> PathBuf {
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(env::temp_dir)
        .join("LunaWiki")
        .join("fastembed")
}

fn repository_key(root: &Path) -> String {
    let normalized = root
        .canonicalize()
        .unwrap_or_else(|_| root.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/")
        .to_lowercase();
    format!("{:x}", Sha256::digest(normalized.as_bytes()))
}

fn table_exists(connection: &Connection, table: &str) -> rusqlite::Result<bool> {
    connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type IN ('table','view') AND name=?1)",
        [table],
        |row| row.get(0),
    )
}

fn load_documents(connection: &Connection) -> rusqlite::Result<Vec<SemanticDocument>> {
    let mut documents = Vec::new();
    if table_exists(connection, "pages")? {
        let mut statement = connection.prepare(
            "SELECT id,page_type,title,body,source_path FROM pages ORDER BY id LIMIT ?1",
        )?;
        let rows = statement.query_map([MAX_DOCUMENTS as i64], |row| {
            let page_id: String = row.get(0)?;
            let page_type: String = row.get(1)?;
            Ok(SemanticDocument {
                key: format!("wiki:{page_id}"),
                kind: "wiki".to_string(),
                tier: if page_type == "method" {
                    "transferable_method".to_string()
                } else {
                    "direct".to_string()
                },
                title: row.get(2)?,
                body: row.get(3)?,
                page_id: page_id.clone(),
                page_type,
                source_path: row.get(4)?,
                wikilink: format!("[[{}]]", page_id.trim_end_matches(".md")),
                book_id: String::new(),
                chapter_id: String::new(),
                physical_page_start: None,
                physical_page_end: None,
                markdown_path: String::new(),
                pdf_path: String::new(),
                node_id: String::new(),
                source_location: String::new(),
            })
        })?;
        documents.extend(rows.collect::<rusqlite::Result<Vec<_>>>()?);
    }
    if documents.len() < MAX_DOCUMENTS && table_exists(connection, "paper_sections")? {
        let remaining = MAX_DOCUMENTS - documents.len();
        let mut statement = connection.prepare(
            "SELECT id,page_id,title,section_title,source_path,pdf_path,line_start,line_end,body
             FROM paper_sections ORDER BY page_id,line_start LIMIT ?1",
        )?;
        let rows = statement.query_map([remaining as i64], |row| {
            let section_id: String = row.get(0)?;
            let page_id: String = row.get(1)?;
            let paper_title: String = row.get(2)?;
            let section_title: String = row.get(3)?;
            let source_path: String = row.get(4)?;
            let line_start: i64 = row.get(6)?;
            let line_end: i64 = row.get(7)?;
            Ok(SemanticDocument {
                key: format!("paper:{section_id}"),
                kind: "paper".to_string(),
                tier: "primary_source".to_string(),
                title: format!("{paper_title} · {section_title}"),
                body: row.get(8)?,
                page_id: page_id.clone(),
                page_type: "source".to_string(),
                source_path: source_path.clone(),
                wikilink: format!("[[{page_id}]]"),
                book_id: String::new(),
                chapter_id: String::new(),
                physical_page_start: None,
                physical_page_end: None,
                markdown_path: source_path,
                pdf_path: row.get(5)?,
                node_id: section_id,
                source_location: format!("{section_title} · 原文第 {line_start}–{line_end} 行"),
            })
        })?;
        documents.extend(rows.collect::<rusqlite::Result<Vec<_>>>()?);
    }
    if documents.len() < MAX_DOCUMENTS
        && table_exists(connection, "book_chapters")?
        && table_exists(connection, "book_chapters_fts")?
        && table_exists(connection, "books")?
    {
        let remaining = MAX_DOCUMENTS - documents.len();
        let mut statement = connection.prepare(
            "SELECT c.id,c.book_id,b.title,c.title,f.body,c.markdown_path,c.pdf_path,
                    c.physical_page_start,c.physical_page_end
             FROM book_chapters c
             JOIN book_chapters_fts f ON f.chapter_id=c.id
             JOIN books b ON b.id=c.book_id
             ORDER BY c.book_id,c.chapter_number LIMIT ?1",
        )?;
        let rows = statement.query_map([remaining as i64], |row| {
            let chapter_id: String = row.get(0)?;
            let book_id: String = row.get(1)?;
            let book_title: String = row.get(2)?;
            let chapter_title: String = row.get(3)?;
            let markdown_path: String = row.get(5)?;
            Ok(SemanticDocument {
                key: format!("book:{chapter_id}"),
                kind: "book".to_string(),
                tier: "theory".to_string(),
                title: format!("{book_title} · {chapter_title}"),
                body: row.get(4)?,
                page_id: String::new(),
                page_type: String::new(),
                source_path: markdown_path.clone(),
                wikilink: String::new(),
                book_id,
                chapter_id,
                physical_page_start: row.get(7)?,
                physical_page_end: row.get(8)?,
                markdown_path,
                pdf_path: row.get(6)?,
                node_id: String::new(),
                source_location: String::new(),
            })
        })?;
        documents.extend(rows.collect::<rusqlite::Result<Vec<_>>>()?);
    }
    Ok(documents)
}

fn ensure_embedding_table(connection: &Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "CREATE TABLE IF NOT EXISTS qa_semantic_embeddings (
           repository_key TEXT NOT NULL,
           snapshot_id TEXT NOT NULL,
           model TEXT NOT NULL,
           document_key TEXT NOT NULL,
           embedding BLOB NOT NULL,
           PRIMARY KEY(repository_key,snapshot_id,model,document_key)
         );
         CREATE INDEX IF NOT EXISTS idx_qa_semantic_embeddings_snapshot
           ON qa_semantic_embeddings(repository_key,snapshot_id,model);",
    )
}

fn load_persisted_embeddings(
    connection: &Connection,
    repository_key: &str,
    snapshot_id: &str,
) -> rusqlite::Result<HashMap<String, Vec<f32>>> {
    let mut statement = connection.prepare(
        "SELECT document_key,embedding FROM qa_semantic_embeddings
         WHERE repository_key=?1 AND snapshot_id=?2 AND model=?3",
    )?;
    let rows = statement.query_map(params![repository_key, snapshot_id, MODEL_NAME], |row| {
        let key: String = row.get(0)?;
        let bytes: Vec<u8> = row.get(1)?;
        Ok((key, decode_embedding(&bytes)))
    })?;
    let mut embeddings = HashMap::new();
    for row in rows {
        let (key, vector) = row?;
        if !vector.is_empty() {
            embeddings.insert(key, vector);
        }
    }
    Ok(embeddings)
}

fn persist_embeddings(
    connection: &Connection,
    repository_key: &str,
    snapshot_id: &str,
    documents: &[SemanticDocument],
    embeddings: &[Vec<f32>],
    indices: &[usize],
) -> rusqlite::Result<()> {
    connection.execute("BEGIN IMMEDIATE", [])?;
    let result = (|| {
        let mut statement = connection.prepare(
            "INSERT OR REPLACE INTO qa_semantic_embeddings
             (repository_key,snapshot_id,model,document_key,embedding)
             VALUES(?1,?2,?3,?4,?5)",
        )?;
        for index in indices {
            statement.execute(params![
                repository_key,
                snapshot_id,
                MODEL_NAME,
                documents[*index].key,
                encode_embedding(&embeddings[*index]),
            ])?;
        }
        Ok(())
    })();
    if result.is_ok() {
        connection.execute("COMMIT", [])?;
    } else {
        let _ = connection.execute("ROLLBACK", []);
    }
    result
}

fn encode_embedding(vector: &[f32]) -> Vec<u8> {
    vector
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn decode_embedding(bytes: &[u8]) -> Vec<f32> {
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return Vec::new();
    }
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> Option<f32> {
    if left.is_empty() || left.len() != right.len() {
        return None;
    }
    let mut dot = 0.0_f32;
    let mut left_norm = 0.0_f32;
    let mut right_norm = 0.0_f32;
    for (left, right) in left.iter().zip(right) {
        dot += left * right;
        left_norm += left * left;
        right_norm += right * right;
    }
    let denominator = left_norm.sqrt() * right_norm.sqrt();
    (denominator > f32::EPSILON)
        .then_some(dot / denominator)
        .filter(|score| score.is_finite())
}

fn rank_candidates(
    documents: &[SemanticDocument],
    embeddings: &[Vec<f32>],
    query: &[f32],
    limit: usize,
) -> Vec<Candidate> {
    let mut ranked = documents
        .iter()
        .zip(embeddings)
        .filter_map(|(document, embedding)| {
            cosine_similarity(query, embedding).map(|score| (document, score))
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.1.total_cmp(&left.1));
    let Some(best) = ranked.first().map(|item| item.1) else {
        return Vec::new();
    };
    let minimum = MIN_COSINE_SIMILARITY.max(best - MAX_DISTANCE_FROM_BEST);
    ranked
        .into_iter()
        .filter(|(_, score)| *score >= minimum)
        .take(limit)
        .map(|(document, score)| document.candidate(score))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(key: &str, title: &str) -> SemanticDocument {
        SemanticDocument {
            key: key.to_string(),
            kind: "wiki".to_string(),
            tier: "direct".to_string(),
            title: title.to_string(),
            body: title.to_string(),
            page_id: format!("{key}.md"),
            page_type: "method".to_string(),
            source_path: format!("wiki/{key}.md"),
            wikilink: format!("[[{key}]]"),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id: String::new(),
            source_location: String::new(),
        }
    }

    #[test]
    fn embedding_binary_round_trip_is_exact() {
        let vector = vec![0.125, -0.5, 1.0, 3.25];
        assert_eq!(decode_embedding(&encode_embedding(&vector)), vector);
    }

    #[test]
    fn semantic_ranking_uses_cosine_and_similarity_floor() {
        let documents = vec![document("related", "相关"), document("noise", "噪声")];
        let embeddings = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let ranked = rank_candidates(&documents, &embeddings, &[0.98, 0.02], 10);
        assert_eq!(ranked.len(), 1);
        assert_eq!(ranked[0].page_id, "related.md");
        assert!(ranked[0].retrieval_reason.contains("cosine="));
    }

    #[test]
    fn corpus_loader_degrades_when_optional_tables_are_absent() {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                "CREATE TABLE pages(id TEXT PRIMARY KEY,page_type TEXT,title TEXT,body TEXT,source_path TEXT);
                 INSERT INTO pages VALUES('method.md','method','Method','semantic body','wiki/method.md');",
            )
            .unwrap();
        let documents = load_documents(&connection).unwrap();
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].key, "wiki:method.md");
    }
}
