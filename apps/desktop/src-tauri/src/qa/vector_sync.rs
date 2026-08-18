use super::semantic;
use super::vector_store::{
    remote_store, SqliteVectorStore, VectorFilter, VectorQuery, VectorRecord, VectorStore,
    VectorStoreStats, VECTOR_DIMENSION, VECTOR_SCHEMA_VERSION,
};
use super::{compact, Candidate};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{OnceLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

const SYNC_BATCH_SIZE: usize = 32;
const QUERY_REMOTE_TIMEOUT_SECONDS: u64 = 4;
static REMOTE_VECTOR_SETTINGS: OnceLock<RwLock<RemoteVectorSettings>> = OnceLock::new();

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RemoteVectorSettings {
    pub enabled: bool,
    pub endpoint: String,
}

fn remote_settings_state() -> &'static RwLock<RemoteVectorSettings> {
    REMOTE_VECTOR_SETTINGS.get_or_init(|| RwLock::new(RemoteVectorSettings::default()))
}

pub(crate) fn configure_remote_vector_settings(settings: RemoteVectorSettings) {
    if let Ok(mut current) = remote_settings_state().write() {
        *current = settings;
    }
}

fn current_remote_vector_settings() -> RemoteVectorSettings {
    remote_settings_state()
        .read()
        .map(|settings| settings.clone())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VectorSyncProgress {
    pub phase: String,
    pub status: String,
    pub total_blocks: usize,
    pub completed_blocks: usize,
    pub computed_blocks: usize,
    pub reused_blocks: usize,
    pub remote_synced_blocks: usize,
    pub percent: f64,
    pub message: String,
}

#[derive(Debug, Serialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SemanticVectorStatus {
    pub schema_version: String,
    pub model_name: String,
    pub dimension: usize,
    pub active_snapshot: String,
    pub local: VectorStoreStats,
    pub remote: VectorStoreStats,
    pub remote_enabled: bool,
    pub remote_key_configured: bool,
    pub counts_by_granularity: HashMap<String, usize>,
    pub last_sync_at: String,
    pub last_error: String,
}

#[derive(Debug, Clone)]
struct EmbeddingBlock {
    block_id: String,
    document_id: String,
    kind: String,
    granularity: String,
    role: String,
    content_hash: String,
    embedding_text: String,
}

fn now_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}

fn repository_id(root: &Path) -> String {
    super::repository_id(root)
}

fn active_snapshot(connection: &Connection) -> Result<String, String> {
    connection
        .query_row(
            "SELECT value FROM repository_metadata WHERE key='markdown_corpus_active_snapshot'",
            [],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|error| format!("读取 Markdown 语料快照失败：{error}"))?
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "VECTOR_INDEX_MISSING: 请先重建 Markdown 知识索引".to_string())
}

fn active_blocks(connection: &Connection) -> Result<Vec<EmbeddingBlock>, String> {
    let mut statement = connection
        .prepare(
            "SELECT b.id,b.document_id,d.kind,b.granularity,b.role,b.content_hash,b.embedding_text
             FROM content_blocks_v2 b JOIN documents_v2 d ON d.id=b.document_id
             WHERE b.active=1 AND d.active=1
             ORDER BY b.document_id,b.granularity,b.ordinal,b.id",
        )
        .map_err(|error| format!("准备多粒度向量计划失败：{error}"))?;
    let result = statement
        .query_map([], |row| {
            Ok(EmbeddingBlock {
                block_id: row.get(0)?,
                document_id: row.get(1)?,
                kind: row.get(2)?,
                granularity: row.get(3)?,
                role: row.get(4)?,
                content_hash: row.get(5)?,
                embedding_text: row.get(6)?,
            })
        })
        .map_err(|error| format!("读取多粒度向量计划失败：{error}"))?
        .collect::<Result<_, _>>()
        .map_err(|error| format!("解析多粒度向量计划失败：{error}"));
    result
}

fn plan_embeddings(
    blocks: &[EmbeddingBlock],
    reusable: &HashMap<String, String>,
) -> (Vec<EmbeddingBlock>, usize) {
    let missing = blocks
        .iter()
        .filter(|block| reusable.get(&block.block_id) != Some(&block.content_hash))
        .cloned()
        .collect::<Vec<_>>();
    let reused = blocks.len().saturating_sub(missing.len());
    (missing, reused)
}

fn check_cancelled(cancelled: Option<&AtomicBool>) -> Result<(), String> {
    if cancelled.is_some_and(|flag| flag.load(Ordering::Relaxed)) {
        let error = super::vector_store::cancelled_error();
        Err(format!("VECTOR_SYNC_CANCELLED: {error}"))
    } else {
        Ok(())
    }
}

fn emit_progress(
    emit: &mut impl FnMut(VectorSyncProgress),
    phase: &str,
    status: &str,
    total: usize,
    completed: usize,
    computed: usize,
    reused: usize,
    remote_synced: usize,
    message: &str,
) {
    emit(VectorSyncProgress {
        phase: phase.to_string(),
        status: status.to_string(),
        total_blocks: total,
        completed_blocks: completed,
        computed_blocks: computed,
        reused_blocks: reused,
        remote_synced_blocks: remote_synced,
        percent: if total == 0 {
            100.0
        } else {
            completed.min(total) as f64 * 100.0 / total as f64
        },
        message: message.to_string(),
    });
}

fn pending_remote_records(
    connection: &Connection,
    repository_id: &str,
    model_id: &str,
    snapshot_id: &str,
) -> Result<Vec<VectorRecord>, String> {
    let mut statement = connection
        .prepare(
            "SELECT block_id,document_id,kind,granularity,role,dimension,content_hash,embedding,active
             FROM embedding_records_v2
             WHERE repository_id=?1 AND model_id=?2 AND snapshot_id=?3 AND active=1
               AND remote_sync_status<>'synced'
             ORDER BY block_id",
        )
        .map_err(|error| format!("准备远程向量同步失败：{error}"))?;
    let result = statement
        .query_map(params![repository_id, model_id, snapshot_id], |row| {
            let dimension = row.get::<_, i64>(5)?.max(0) as usize;
            let bytes = row.get::<_, Vec<u8>>(7)?;
            if dimension == 0 || bytes.len() != dimension.saturating_mul(4) {
                return Err(rusqlite::Error::InvalidQuery);
            }
            let embedding = bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();
            Ok(VectorRecord {
                repository_id: repository_id.to_string(),
                snapshot_id: snapshot_id.to_string(),
                block_id: row.get(0)?,
                document_id: row.get(1)?,
                kind: row.get(2)?,
                granularity: row.get(3)?,
                role: row.get(4)?,
                model_id: model_id.to_string(),
                dimension,
                content_hash: row.get(6)?,
                embedding,
                active: row.get(8)?,
            })
        })
        .map_err(|error| format!("读取远程向量同步记录失败：{error}"))?
        .collect::<Result<_, _>>()
        .map_err(|error| format!("解析远程向量同步记录失败：{error}"));
    result
}

pub(crate) fn sync_embeddings(
    connection: &Connection,
    root: &Path,
    remote_settings: &RemoteVectorSettings,
    cancelled: Option<&AtomicBool>,
    mut emit: impl FnMut(VectorSyncProgress),
) -> Result<SemanticVectorStatus, String> {
    check_cancelled(cancelled)?;
    super::vector_store::db_schema(connection)?;
    let repository_id = repository_id(root);
    let snapshot_id = active_snapshot(connection)?;
    let model_id = semantic::MODEL_NAME;
    let blocks = active_blocks(connection)?;
    let local = SqliteVectorStore::new(connection).map_err(|error| error.to_string())?;
    let reusable = local
        .reusable_hashes(&repository_id, model_id)
        .map_err(|error| error.to_string())?;
    let (missing, reused) = plan_embeddings(&blocks, &reusable);
    connection
        .execute(
            "UPDATE embedding_records_v2 SET active=0
             WHERE repository_id=?1 AND model_id=?2
               AND block_id NOT IN (SELECT id FROM content_blocks_v2 WHERE active=1)",
            params![repository_id, model_id],
        )
        .map_err(|error| format!("停用已删除内容块向量失败：{error}"))?;
    let stale_remote_snapshots =
        super::vector_store::stale_snapshots(connection, &repository_id, model_id, &snapshot_id)?;
    connection
        .execute(
            "UPDATE embedding_records_v2
             SET snapshot_id=?1,active=1,
                 remote_sync_status=CASE
                   WHEN ?4 AND snapshot_id<>?1 THEN 'pending'
                   ELSE remote_sync_status END
             WHERE repository_id=?2 AND model_id=?3
               AND block_id IN (SELECT id FROM content_blocks_v2 WHERE active=1)",
            params![
                snapshot_id,
                repository_id,
                model_id,
                remote_settings.enabled
            ],
        )
        .map_err(|error| format!("更新复用向量快照失败：{error}"))?;
    emit_progress(
        &mut emit,
        "planning",
        "running",
        blocks.len(),
        reused,
        0,
        reused,
        0,
        "已完成增量向量计划",
    );

    let mut computed = 0usize;
    for batch in missing.chunks(SYNC_BATCH_SIZE) {
        check_cancelled(cancelled)?;
        let embeddings = semantic::embed_texts(
            batch
                .iter()
                .map(|block| block.embedding_text.clone())
                .collect(),
            cancelled,
        )?;
        let records = batch
            .iter()
            .zip(embeddings)
            .map(|(block, embedding)| VectorRecord {
                repository_id: repository_id.clone(),
                snapshot_id: snapshot_id.clone(),
                block_id: block.block_id.clone(),
                document_id: block.document_id.clone(),
                kind: block.kind.clone(),
                granularity: block.granularity.clone(),
                role: block.role.clone(),
                model_id: model_id.to_string(),
                dimension: VECTOR_DIMENSION,
                content_hash: block.content_hash.clone(),
                embedding,
                active: true,
            })
            .collect::<Vec<_>>();
        local
            .upsert_batch(&records)
            .map_err(|error| error.to_string())?;
        computed += records.len();
        emit_progress(
            &mut emit,
            "embedding",
            "running",
            blocks.len(),
            reused + computed,
            computed,
            reused,
            0,
            "正在生成并保存本地多粒度向量",
        );
    }

    let mut remote_synced = 0usize;
    let mut last_error = String::new();
    if remote_settings.enabled {
        match remote_store(&remote_settings.endpoint) {
            Ok(Some(remote)) => {
                let pending =
                    pending_remote_records(connection, &repository_id, model_id, &snapshot_id)?;
                for batch in pending.chunks(SYNC_BATCH_SIZE) {
                    check_cancelled(cancelled)?;
                    let ids = batch
                        .iter()
                        .map(|record| record.block_id.clone())
                        .collect::<Vec<_>>();
                    match remote.upsert_batch(batch) {
                        Ok(()) => {
                            local
                                .mark_remote_status(&repository_id, model_id, &ids, "synced", "")
                                .map_err(|error| error.to_string())?;
                            remote_synced += batch.len();
                        }
                        Err(error) => {
                            last_error = format!("{}: {error}", error.kind_code());
                            local
                                .mark_remote_status(
                                    &repository_id,
                                    model_id,
                                    &ids,
                                    "failed",
                                    &last_error,
                                )
                                .map_err(|error| error.to_string())?;
                            break;
                        }
                    }
                    emit_progress(
                        &mut emit,
                        "remote",
                        "running",
                        pending.len(),
                        remote_synced,
                        computed,
                        reused,
                        remote_synced,
                        "正在同步远程 pgvector",
                    );
                }
                if last_error.is_empty() {
                    for stale_snapshot in stale_remote_snapshots {
                        check_cancelled(cancelled)?;
                        let _ = remote.delete_snapshot(&repository_id, &stale_snapshot, model_id);
                    }
                }
                let _ = remote.close();
            }
            Ok(None) => last_error = "远程向量凭据尚未配置".to_string(),
            Err(error) => last_error = error,
        }
    }
    connection
        .execute(
            "INSERT INTO repository_metadata(key,value) VALUES('semantic_vector_last_sync_at',?1)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            [now_string()],
        )
        .map_err(|error| format!("记录向量同步时间失败：{error}"))?;
    connection
        .execute(
            "INSERT INTO repository_metadata(key,value) VALUES('semantic_vector_last_error',?1)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            [&last_error],
        )
        .map_err(|error| format!("记录向量同步状态失败：{error}"))?;
    emit_progress(
        &mut emit,
        "complete",
        "complete",
        blocks.len(),
        blocks.len(),
        computed,
        reused,
        remote_synced,
        if last_error.is_empty() {
            "多粒度向量同步完成"
        } else {
            "本地向量完成；远程向量已降级"
        },
    );
    let _ = local.close();
    vector_status(connection, root, remote_settings)
}

pub(crate) fn vector_status(
    connection: &Connection,
    root: &Path,
    remote_settings: &RemoteVectorSettings,
) -> Result<SemanticVectorStatus, String> {
    super::vector_store::db_schema(connection)?;
    let repository_id = repository_id(root);
    let snapshot = active_snapshot(connection).unwrap_or_default();
    let local_store = SqliteVectorStore::new(connection).map_err(|error| error.to_string())?;
    let _ = local_store.health().map_err(|error| error.to_string())?;
    let local = local_store
        .stats(&repository_id, semantic::MODEL_NAME)
        .map_err(|error| error.to_string())?;
    let mut counts_by_granularity = HashMap::new();
    let mut statement = connection
        .prepare(
            "SELECT granularity,COUNT(*) FROM embedding_records_v2
             WHERE repository_id=?1 AND model_id=?2 AND active=1 GROUP BY granularity",
        )
        .map_err(|error| format!("准备向量粒度统计失败：{error}"))?;
    let rows = statement
        .query_map(params![repository_id, semantic::MODEL_NAME], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })
        .map_err(|error| format!("读取向量粒度统计失败：{error}"))?;
    for row in rows {
        let (granularity, count) = row.map_err(|error| format!("解析向量粒度统计失败：{error}"))?;
        counts_by_granularity.insert(granularity, count.max(0) as usize);
    }
    let remote = if remote_settings.enabled {
        match super::vector_store::remote_store_with_timeout(
            &remote_settings.endpoint,
            QUERY_REMOTE_TIMEOUT_SECONDS,
        ) {
            Ok(Some(store)) => store
                .health()
                .and_then(|_| store.stats(&repository_id, semantic::MODEL_NAME))
                .unwrap_or_else(|error| VectorStoreStats {
                    store: "pgvector".to_string(),
                    ready: false,
                    dimension: VECTOR_DIMENSION,
                    last_error: error.to_string(),
                    ..VectorStoreStats::default()
                }),
            Ok(None) => VectorStoreStats {
                store: "pgvector".to_string(),
                ready: false,
                dimension: VECTOR_DIMENSION,
                last_error: "远程向量凭据尚未配置".to_string(),
                ..VectorStoreStats::default()
            },
            Err(error) => VectorStoreStats {
                store: "pgvector".to_string(),
                ready: false,
                dimension: VECTOR_DIMENSION,
                last_error: error,
                ..VectorStoreStats::default()
            },
        }
    } else {
        VectorStoreStats {
            store: "pgvector".to_string(),
            dimension: VECTOR_DIMENSION,
            ..VectorStoreStats::default()
        }
    };
    let metadata = |key: &str| {
        connection
            .query_row(
                "SELECT value FROM repository_metadata WHERE key=?1",
                [key],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .ok()
            .flatten()
            .unwrap_or_default()
    };
    Ok(SemanticVectorStatus {
        schema_version: VECTOR_SCHEMA_VERSION.to_string(),
        model_name: semantic::MODEL_NAME.to_string(),
        dimension: VECTOR_DIMENSION,
        active_snapshot: snapshot,
        local,
        remote,
        remote_enabled: remote_settings.enabled,
        remote_key_configured: super::vector_store::remote_key_configured(),
        counts_by_granularity,
        last_sync_at: metadata("semantic_vector_last_sync_at"),
        last_error: metadata("semantic_vector_last_error"),
    })
}

pub(super) fn semantic_candidates_v2(
    connection: &Connection,
    root: &Path,
    question: &str,
    cancelled: Option<&AtomicBool>,
) -> Result<Vec<Candidate>, String> {
    let repository_id = repository_id(root);
    let snapshot_id = match active_snapshot(connection) {
        Ok(value) => value,
        Err(_) => return Ok(Vec::new()),
    };
    let local = match SqliteVectorStore::new(connection) {
        Ok(value) => value,
        Err(_) => return Ok(Vec::new()),
    };
    let stats = local
        .stats(&repository_id, semantic::MODEL_NAME)
        .map_err(|error| error.to_string())?;
    let remote_settings = current_remote_vector_settings();
    let remote_possible = remote_settings.enabled
        && !remote_settings.endpoint.trim().is_empty()
        && super::vector_store::remote_key_configured();
    if stats.vector_count == 0 && !remote_possible {
        return Ok(Vec::new());
    }
    super::check_cancelled(cancelled)?;
    let query_embedding = semantic::embed_texts(vec![question.trim().to_string()], cancelled)?
        .into_iter()
        .next()
        .ok_or_else(|| "SEMANTIC_UNAVAILABLE: 查询向量为空".to_string())?;
    let query = VectorQuery {
        repository_id,
        snapshot_id,
        model_id: semantic::MODEL_NAME.to_string(),
        embedding: query_embedding,
        limit: 40,
        min_score: Some(0.42),
        filter: VectorFilter {
            granularities: vec!["section".to_string(), "semantic".to_string()],
            ..VectorFilter::default()
        },
    };
    let remote_hits = if remote_possible {
        super::vector_store::remote_store_with_timeout(
            &remote_settings.endpoint,
            QUERY_REMOTE_TIMEOUT_SECONDS,
        )
        .ok()
        .flatten()
        .and_then(|remote| remote.query(&query).ok())
        .unwrap_or_default()
    } else {
        Vec::new()
    };
    let hits = if remote_hits.is_empty() && stats.vector_count > 0 {
        local.query(&query).map_err(|error| error.to_string())?
    } else {
        remote_hits
    };
    let mut candidates = Vec::new();
    for hit in hits {
        super::check_cancelled(cancelled)?;
        let row = connection
            .query_row(
                "SELECT d.canonical_title,d.kind,b.heading,b.heading_path_json,b.role,b.content,b.markdown_path,b.line_start,b.line_end,b.document_id
                 FROM content_blocks_v2 b JOIN documents_v2 d ON d.id=b.document_id
                 WHERE b.id=?1 AND b.active=1 AND d.active=1",
                [&hit.block_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?,
                        row.get::<_, String>(6)?, row.get::<_, i64>(7)?, row.get::<_, i64>(8)?,
                        row.get::<_, String>(9)?,
                    ))
                },
            )
            .optional()
            .map_err(|error| format!("读取语义命中内容块失败：{error}"))?;
        let Some((
            title,
            kind,
            heading,
            heading_path_json,
            role,
            content,
            markdown_path,
            line_start,
            line_end,
            document_id,
        )) = row
        else {
            continue;
        };
        let heading_path: Vec<String> =
            serde_json::from_str(&heading_path_json).unwrap_or_default();
        let source_id = document_id
            .split_once(':')
            .map(|(_, value)| value)
            .unwrap_or("")
            .to_string();
        let tier = match (kind.as_str(), role.as_str()) {
            ("paper", _) => "primary_source",
            ("book", _) => "theory",
            (_, "method" | "algorithm") => "transferable_method",
            _ => "direct",
        };
        candidates.push(Candidate {
            kind: kind.clone(),
            tier: tier.to_string(),
            title: format!("{title} · {heading}"),
            snippet: compact(&content, 1_200),
            score: hit.score,
            page_id: if matches!(kind.as_str(), "wiki" | "paper") {
                source_id.clone()
            } else {
                String::new()
            },
            page_type: if kind == "paper" {
                "source".to_string()
            } else {
                String::new()
            },
            source_path: markdown_path.clone(),
            wikilink: if matches!(kind.as_str(), "wiki" | "paper") {
                format!("[[{source_id}]]")
            } else {
                String::new()
            },
            book_id: if kind == "book" {
                source_id
            } else {
                String::new()
            },
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path,
            pdf_path: String::new(),
            node_id: hit.block_id,
            source_location: format!(
                "{} · Markdown 第 {line_start}–{line_end} 行",
                heading_path.join(" / ")
            ),
            relation: "semantic_block_v2".to_string(),
            retrieval_reason: format!("多粒度本地向量命中；role={role}；store={}", hit.store),
        });
    }
    Ok(candidates)
}

#[cfg(test)]
mod tests {
    use super::super::vector_store::db_schema as vector_schema;
    use super::*;

    fn block(id: &str, granularity: &str, hash: &str) -> EmbeddingBlock {
        EmbeddingBlock {
            block_id: id.to_string(),
            document_id: "book:approximation-algorithms".to_string(),
            kind: "book".to_string(),
            granularity: granularity.to_string(),
            role: "algorithm".to_string(),
            content_hash: hash.to_string(),
            embedding_text: format!("近似算法 / {granularity} / TSP"),
        }
    }

    #[test]
    fn incremental_plan_reuses_unchanged_blocks_and_recomputes_only_changed_hashes() {
        let blocks = vec![
            block("document", "document", "h1"),
            block("section", "section", "h2"),
            block("semantic", "semantic", "h3"),
        ];
        let reusable = HashMap::from([
            ("document".to_string(), "h1".to_string()),
            ("section".to_string(), "h2".to_string()),
            ("semantic".to_string(), "h3".to_string()),
        ]);
        let (missing, reused) = plan_embeddings(&blocks, &reusable);
        assert!(missing.is_empty());
        assert_eq!(reused, 3);

        let changed = vec![
            blocks[0].clone(),
            block("section", "section", "h2-new"),
            blocks[2].clone(),
        ];
        let (missing, reused) = plan_embeddings(&changed, &reusable);
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].block_id, "section");
        assert_eq!(reused, 2);
    }

    #[test]
    #[ignore = "requires QA_SEMANTIC_MODEL_CACHE_DIR with a deployed local model"]
    fn deployed_multilingual_model_recalls_euclidean_tsp_for_mobile_path_planning() {
        let cache_dir = std::env::var("QA_SEMANTIC_MODEL_CACHE_DIR")
            .expect("QA_SEMANTIC_MODEL_CACHE_DIR must point to the deployed model cache");
        semantic::configure_cache_dir(Some(cache_dir.into())).unwrap();
        let texts = vec![
            "移动路径规划".to_string(),
            "Euclidean TSP. Given points in Euclidean space, find the minimum length tour. The chapter gives a polynomial-time approximation scheme using a dissection, portals, and dynamic programming.".to_string(),
            "The set cover problem selects a minimum collection of sets whose union covers every element.".to_string(),
            "Mechanism design studies truthful auctions, payments, and strategic agents.".to_string(),
        ];
        let mut embeddings = semantic::embed_texts(texts, None).unwrap();
        let query_embedding = embeddings.remove(0);
        let connection = Connection::open_in_memory().unwrap();
        let store = SqliteVectorStore::new(&connection).unwrap();
        let records = ["euclidean-tsp", "set-cover", "mechanism-design"]
            .into_iter()
            .zip(embeddings)
            .map(|(block_id, embedding)| VectorRecord {
                repository_id: "repo".to_string(),
                snapshot_id: "snapshot".to_string(),
                block_id: block_id.to_string(),
                document_id: "book:approximation-algorithms".to_string(),
                kind: "book".to_string(),
                granularity: "semantic".to_string(),
                role: "algorithm".to_string(),
                model_id: semantic::MODEL_NAME.to_string(),
                dimension: VECTOR_DIMENSION,
                content_hash: format!("hash:{block_id}"),
                embedding,
                active: true,
            })
            .collect::<Vec<_>>();
        store.upsert_batch(&records).unwrap();
        let hits = store
            .query(&VectorQuery {
                repository_id: "repo".to_string(),
                snapshot_id: "snapshot".to_string(),
                model_id: semantic::MODEL_NAME.to_string(),
                embedding: query_embedding,
                limit: 3,
                min_score: None,
                filter: VectorFilter {
                    kinds: vec!["book".to_string()],
                    granularities: vec!["semantic".to_string()],
                    ..VectorFilter::default()
                },
            })
            .unwrap();
        assert_eq!(hits[0].block_id, "euclidean-tsp", "hits={hits:?}");
    }

    #[test]
    fn cancelled_sync_does_not_mark_completion() {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(
            "CREATE TABLE repository_metadata(key TEXT PRIMARY KEY,value TEXT NOT NULL);
             CREATE TABLE documents_v2(id TEXT PRIMARY KEY,kind TEXT,active INTEGER);
             CREATE TABLE content_blocks_v2(id TEXT PRIMARY KEY,document_id TEXT,granularity TEXT,role TEXT,content_hash TEXT,embedding_text TEXT,active INTEGER);",
        ).unwrap();
        vector_schema(&connection).unwrap();
        connection.execute("INSERT INTO repository_metadata VALUES('markdown_corpus_active_snapshot','snapshot')", []).unwrap();
        let cancelled = AtomicBool::new(true);
        let result = sync_embeddings(
            &connection,
            Path::new("."),
            &RemoteVectorSettings::default(),
            Some(&cancelled),
            |_| {},
        );
        assert!(result.unwrap_err().starts_with("VECTOR_SYNC_CANCELLED"));
        let completed: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM repository_metadata WHERE key='semantic_vector_last_sync_at'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(completed, 0);
    }
}
