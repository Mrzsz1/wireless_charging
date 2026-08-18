use keyring::Entry;
use reqwest::blocking::Client;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fmt;
use std::time::Duration;

pub(crate) const VECTOR_SCHEMA_VERSION: &str = "rag-vector-store-v2";
pub(crate) const VECTOR_DIMENSION: usize = 384;
const REMOTE_CREDENTIAL_SERVICE: &str = "wireless-charging-research-workbench.vector";
const REMOTE_CREDENTIAL_ACCOUNT: &str = "pgvector-api-key";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorStoreErrorKind {
    Configuration,
    Authentication,
    Timeout,
    RateLimit,
    Unavailable,
    Corrupt,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct VectorStoreError {
    pub kind: VectorStoreErrorKind,
    pub message: &'static str,
}

impl VectorStoreError {
    fn new(kind: VectorStoreErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }

    pub(crate) fn kind_code(&self) -> &'static str {
        match self.kind {
            VectorStoreErrorKind::Configuration => "configuration",
            VectorStoreErrorKind::Authentication => "authentication",
            VectorStoreErrorKind::Timeout => "timeout",
            VectorStoreErrorKind::RateLimit => "rate_limit",
            VectorStoreErrorKind::Unavailable => "unavailable",
            VectorStoreErrorKind::Corrupt => "corrupt",
            VectorStoreErrorKind::Cancelled => "cancelled",
        }
    }
}

pub(crate) fn cancelled_error() -> VectorStoreError {
    VectorStoreError::new(VectorStoreErrorKind::Cancelled, "向量操作已取消")
}

impl fmt::Display for VectorStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl std::error::Error for VectorStoreError {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VectorRecord {
    pub repository_id: String,
    pub snapshot_id: String,
    pub block_id: String,
    pub document_id: String,
    pub kind: String,
    pub granularity: String,
    pub role: String,
    pub model_id: String,
    pub dimension: usize,
    pub content_hash: String,
    pub embedding: Vec<f32>,
    pub active: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VectorFilter {
    pub kinds: Vec<String>,
    pub document_ids: Vec<String>,
    pub granularities: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct VectorQuery {
    pub repository_id: String,
    pub snapshot_id: String,
    pub model_id: String,
    pub embedding: Vec<f32>,
    pub limit: usize,
    pub min_score: Option<f64>,
    pub filter: VectorFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VectorHit {
    pub block_id: String,
    pub score: f64,
    pub store: String,
    pub model_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VectorStoreStats {
    pub store: String,
    pub ready: bool,
    pub vector_count: usize,
    pub document_count: usize,
    pub pending_sync_count: usize,
    pub model_id: String,
    pub dimension: usize,
    pub last_error: String,
}

pub trait VectorStore {
    fn health(&self) -> Result<VectorStoreStats, VectorStoreError>;
    fn stats(
        &self,
        repository_id: &str,
        model_id: &str,
    ) -> Result<VectorStoreStats, VectorStoreError>;
    fn upsert_batch(&self, records: &[VectorRecord]) -> Result<(), VectorStoreError>;
    fn query(&self, query: &VectorQuery) -> Result<Vec<VectorHit>, VectorStoreError>;
    fn delete_snapshot(
        &self,
        repository_id: &str,
        snapshot_id: &str,
        model_id: &str,
    ) -> Result<usize, VectorStoreError>;
    fn close(&self) -> Result<(), VectorStoreError> {
        Ok(())
    }
}

pub(crate) fn db_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS embedding_records_v2 (
              repository_id TEXT NOT NULL,
              snapshot_id TEXT NOT NULL,
              block_id TEXT NOT NULL,
              document_id TEXT NOT NULL,
              kind TEXT NOT NULL,
              granularity TEXT NOT NULL,
              role TEXT NOT NULL,
              model_id TEXT NOT NULL,
              dimension INTEGER NOT NULL,
              content_hash TEXT NOT NULL,
              embedding BLOB NOT NULL,
              active INTEGER NOT NULL DEFAULT 1,
              remote_sync_status TEXT NOT NULL DEFAULT 'local_only',
              last_error TEXT NOT NULL DEFAULT '',
              updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
              PRIMARY KEY(repository_id, model_id, block_id)
            );
            CREATE INDEX IF NOT EXISTS idx_embedding_v2_active_model
              ON embedding_records_v2(repository_id, model_id, active);
            CREATE INDEX IF NOT EXISTS idx_embedding_v2_document
              ON embedding_records_v2(repository_id, document_id, granularity, role);
            ",
        )
        .map_err(|error| format!("初始化向量存储 v2 失败：{error}"))?;
    Ok(())
}

fn encode_embedding(vector: &[f32]) -> Vec<u8> {
    vector
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn decode_embedding(bytes: &[u8], dimension: usize) -> Result<Vec<f32>, VectorStoreError> {
    if dimension == 0 || bytes.len() != dimension.saturating_mul(4) {
        return Err(VectorStoreError::new(
            VectorStoreErrorKind::Corrupt,
            "本地向量记录损坏",
        ));
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect())
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> Option<f64> {
    if left.is_empty() || left.len() != right.len() {
        return None;
    }
    let mut dot = 0.0_f64;
    let mut left_norm = 0.0_f64;
    let mut right_norm = 0.0_f64;
    for (left, right) in left.iter().zip(right) {
        let left = f64::from(*left);
        let right = f64::from(*right);
        dot += left * right;
        left_norm += left * left;
        right_norm += right * right;
    }
    if left_norm <= f64::EPSILON || right_norm <= f64::EPSILON {
        None
    } else {
        Some(dot / (left_norm.sqrt() * right_norm.sqrt()))
    }
}

fn filter_matches(filter: &VectorFilter, record: &LocalVectorRow) -> bool {
    (filter.kinds.is_empty() || filter.kinds.contains(&record.kind))
        && (filter.document_ids.is_empty() || filter.document_ids.contains(&record.document_id))
        && (filter.granularities.is_empty() || filter.granularities.contains(&record.granularity))
        && (filter.roles.is_empty() || filter.roles.contains(&record.role))
}

#[derive(Debug)]
struct LocalVectorRow {
    block_id: String,
    document_id: String,
    kind: String,
    granularity: String,
    role: String,
    dimension: usize,
    embedding: Vec<u8>,
}

pub struct SqliteVectorStore<'a> {
    connection: &'a Connection,
}

impl<'a> SqliteVectorStore<'a> {
    pub(crate) fn new(connection: &'a Connection) -> Result<Self, VectorStoreError> {
        db_schema(connection).map_err(|_| {
            VectorStoreError::new(VectorStoreErrorKind::Unavailable, "本地向量存储不可用")
        })?;
        Ok(Self { connection })
    }

    pub(crate) fn reusable_hashes(
        &self,
        repository_id: &str,
        model_id: &str,
    ) -> Result<std::collections::HashMap<String, String>, VectorStoreError> {
        let mut statement = self
            .connection
            .prepare(
                "SELECT block_id,content_hash FROM embedding_records_v2
                 WHERE repository_id=?1 AND model_id=?2 AND active=1",
            )
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Unavailable, "读取本地向量失败")
            })?;
        let result = statement
            .query_map(params![repository_id, model_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Unavailable, "读取本地向量失败")
            })?
            .collect::<Result<_, _>>()
            .map_err(|_| VectorStoreError::new(VectorStoreErrorKind::Corrupt, "本地向量记录损坏"));
        result
    }

    pub(crate) fn mark_remote_status(
        &self,
        repository_id: &str,
        model_id: &str,
        block_ids: &[String],
        status: &str,
        last_error: &str,
    ) -> Result<(), VectorStoreError> {
        for block_id in block_ids {
            self.connection
                .execute(
                    "UPDATE embedding_records_v2 SET remote_sync_status=?1,last_error=?2,updated_at=CURRENT_TIMESTAMP
                     WHERE repository_id=?3 AND model_id=?4 AND block_id=?5",
                    params![status, last_error, repository_id, model_id, block_id],
                )
                .map_err(|_| {
                    VectorStoreError::new(
                        VectorStoreErrorKind::Unavailable,
                        "更新向量同步状态失败",
                    )
                })?;
        }
        Ok(())
    }
}

impl VectorStore for SqliteVectorStore<'_> {
    fn health(&self) -> Result<VectorStoreStats, VectorStoreError> {
        self.connection
            .query_row("SELECT COUNT(*) FROM embedding_records_v2", [], |row| {
                row.get::<_, i64>(0)
            })
            .map(|count| VectorStoreStats {
                store: "local-sqlite".to_string(),
                ready: true,
                vector_count: count.max(0) as usize,
                dimension: VECTOR_DIMENSION,
                ..VectorStoreStats::default()
            })
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Unavailable, "本地向量存储不可用")
            })
    }

    fn stats(
        &self,
        repository_id: &str,
        model_id: &str,
    ) -> Result<VectorStoreStats, VectorStoreError> {
        self.connection
            .query_row(
                "SELECT COUNT(*),COUNT(DISTINCT document_id),
                        SUM(CASE WHEN remote_sync_status IN ('pending','failed') THEN 1 ELSE 0 END)
                 FROM embedding_records_v2 WHERE repository_id=?1 AND model_id=?2 AND active=1",
                params![repository_id, model_id],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, Option<i64>>(2)?.unwrap_or(0),
                    ))
                },
            )
            .map(|(vectors, documents, pending)| VectorStoreStats {
                store: "local-sqlite".to_string(),
                ready: true,
                vector_count: vectors.max(0) as usize,
                document_count: documents.max(0) as usize,
                pending_sync_count: pending.max(0) as usize,
                model_id: model_id.to_string(),
                dimension: VECTOR_DIMENSION,
                last_error: String::new(),
            })
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Unavailable, "读取本地向量统计失败")
            })
    }

    fn upsert_batch(&self, records: &[VectorRecord]) -> Result<(), VectorStoreError> {
        for record in records {
            if record.dimension != record.embedding.len()
                || record.dimension != VECTOR_DIMENSION
                || record.embedding.iter().any(|value| !value.is_finite())
            {
                return Err(VectorStoreError::new(
                    VectorStoreErrorKind::Corrupt,
                    "向量维度或数值无效",
                ));
            }
            self.connection
                .execute(
                    "INSERT INTO embedding_records_v2(repository_id,snapshot_id,block_id,document_id,kind,granularity,role,model_id,dimension,content_hash,embedding,active,remote_sync_status,last_error,updated_at)
                     VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,'local_only','',CURRENT_TIMESTAMP)
                     ON CONFLICT(repository_id,model_id,block_id) DO UPDATE SET snapshot_id=excluded.snapshot_id,document_id=excluded.document_id,kind=excluded.kind,granularity=excluded.granularity,role=excluded.role,dimension=excluded.dimension,content_hash=excluded.content_hash,embedding=excluded.embedding,active=excluded.active,remote_sync_status='local_only',last_error='',updated_at=CURRENT_TIMESTAMP",
                    params![
                        record.repository_id,
                        record.snapshot_id,
                        record.block_id,
                        record.document_id,
                        record.kind,
                        record.granularity,
                        record.role,
                        record.model_id,
                        record.dimension as i64,
                        record.content_hash,
                        encode_embedding(&record.embedding),
                        record.active,
                    ],
                )
                .map_err(|_| VectorStoreError::new(VectorStoreErrorKind::Unavailable, "写入本地向量失败"))?;
        }
        Ok(())
    }

    fn query(&self, query: &VectorQuery) -> Result<Vec<VectorHit>, VectorStoreError> {
        if query.embedding.len() != VECTOR_DIMENSION {
            return Err(VectorStoreError::new(
                VectorStoreErrorKind::Corrupt,
                "查询向量维度无效",
            ));
        }
        let mut statement = self
            .connection
            .prepare(
                "SELECT block_id,document_id,kind,granularity,role,dimension,embedding
                 FROM embedding_records_v2
                 WHERE repository_id=?1 AND model_id=?2 AND snapshot_id=?3 AND active=1",
            )
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Unavailable, "准备本地向量查询失败")
            })?;
        let rows = statement
            .query_map(
                params![query.repository_id, query.model_id, query.snapshot_id],
                |row| {
                    Ok(LocalVectorRow {
                        block_id: row.get(0)?,
                        document_id: row.get(1)?,
                        kind: row.get(2)?,
                        granularity: row.get(3)?,
                        role: row.get(4)?,
                        dimension: row.get::<_, i64>(5)?.max(0) as usize,
                        embedding: row.get(6)?,
                    })
                },
            )
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Unavailable, "执行本地向量查询失败")
            })?;
        let mut hits = Vec::new();
        for row in rows {
            let row = row.map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Corrupt, "本地向量记录损坏")
            })?;
            if !filter_matches(&query.filter, &row) {
                continue;
            }
            let embedding = decode_embedding(&row.embedding, row.dimension)?;
            let Some(score) = cosine_similarity(&query.embedding, &embedding) else {
                continue;
            };
            if query.min_score.is_some_and(|minimum| score < minimum) {
                continue;
            }
            hits.push(VectorHit {
                block_id: row.block_id,
                score,
                store: "local-sqlite".to_string(),
                model_id: query.model_id.clone(),
            });
        }
        hits.sort_by(|left, right| {
            right
                .score
                .partial_cmp(&left.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| left.block_id.cmp(&right.block_id))
        });
        hits.truncate(query.limit.clamp(1, 200));
        Ok(hits)
    }

    fn delete_snapshot(
        &self,
        repository_id: &str,
        snapshot_id: &str,
        model_id: &str,
    ) -> Result<usize, VectorStoreError> {
        self.connection
            .execute(
                "DELETE FROM embedding_records_v2 WHERE repository_id=?1 AND snapshot_id=?2 AND model_id=?3",
                params![repository_id, snapshot_id, model_id],
            )
            .map_err(|_| VectorStoreError::new(VectorStoreErrorKind::Unavailable, "清理本地向量快照失败"))
    }
}

#[derive(Debug, Clone)]
pub struct PgVectorConfig {
    pub endpoint: String,
    pub api_key: String,
    pub timeout_seconds: u64,
}

pub struct PgVectorStore {
    endpoint: String,
    api_key: String,
    client: Client,
}

impl PgVectorStore {
    pub fn new(config: PgVectorConfig) -> Result<Self, VectorStoreError> {
        let endpoint = config.endpoint.trim().trim_end_matches('/').to_string();
        let parsed = reqwest::Url::parse(&endpoint).map_err(|_| {
            VectorStoreError::new(VectorStoreErrorKind::Configuration, "远程向量地址无效")
        })?;
        let local_http = parsed.scheme() == "http"
            && parsed
                .host_str()
                .is_some_and(|host| matches!(host, "localhost" | "127.0.0.1" | "::1"));
        if parsed.scheme() != "https" && !local_http {
            return Err(VectorStoreError::new(
                VectorStoreErrorKind::Configuration,
                "远程向量地址必须使用 HTTPS",
            ));
        }
        if config.api_key.trim().is_empty() {
            return Err(VectorStoreError::new(
                VectorStoreErrorKind::Authentication,
                "远程向量凭据尚未配置",
            ));
        }
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds.clamp(3, 120)))
            .build()
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Unavailable, "创建远程向量连接失败")
            })?;
        Ok(Self {
            endpoint,
            api_key: config.api_key,
            client,
        })
    }

    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::blocking::RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.endpoint, path))
            .header("apikey", &self.api_key)
            .bearer_auth(&self.api_key)
    }

    fn classify_error(error: &reqwest::Error) -> VectorStoreError {
        if error.is_timeout() {
            VectorStoreError::new(VectorStoreErrorKind::Timeout, "远程向量服务响应超时")
        } else {
            VectorStoreError::new(VectorStoreErrorKind::Unavailable, "远程向量服务不可用")
        }
    }

    fn require_success(
        response: reqwest::blocking::Response,
    ) -> Result<reqwest::blocking::Response, VectorStoreError> {
        let status = response.status();
        if status.is_success() {
            Ok(response)
        } else if status.as_u16() == 401 || status.as_u16() == 403 {
            Err(VectorStoreError::new(
                VectorStoreErrorKind::Authentication,
                "远程向量凭据无效",
            ))
        } else if status.as_u16() == 429 {
            Err(VectorStoreError::new(
                VectorStoreErrorKind::RateLimit,
                "远程向量服务达到速率限制",
            ))
        } else {
            Err(VectorStoreError::new(
                VectorStoreErrorKind::Unavailable,
                "远程向量服务返回错误",
            ))
        }
    }
}

impl VectorStore for PgVectorStore {
    fn health(&self) -> Result<VectorStoreStats, VectorStoreError> {
        let response = self
            .request(
                reqwest::Method::GET,
                "/rest/v1/rag_embeddings?select=block_id&limit=1",
            )
            .send()
            .map_err(|error| Self::classify_error(&error))?;
        Self::require_success(response)?;
        Ok(VectorStoreStats {
            store: "pgvector".to_string(),
            ready: true,
            dimension: VECTOR_DIMENSION,
            ..VectorStoreStats::default()
        })
    }

    fn stats(
        &self,
        repository_id: &str,
        model_id: &str,
    ) -> Result<VectorStoreStats, VectorStoreError> {
        let path = format!("/rest/v1/rpc/rag_embedding_stats");
        let response = self
            .request(reqwest::Method::POST, &path)
            .json(&json!({"p_repository_id": repository_id, "p_model_id": model_id}))
            .send()
            .map_err(|error| Self::classify_error(&error))?;
        let payload: Value = Self::require_success(response)?.json().map_err(|_| {
            VectorStoreError::new(VectorStoreErrorKind::Corrupt, "远程向量统计格式无效")
        })?;
        let row = payload
            .as_array()
            .and_then(|items| items.first())
            .unwrap_or(&payload);
        Ok(VectorStoreStats {
            store: "pgvector".to_string(),
            ready: true,
            vector_count: row.get("vector_count").and_then(Value::as_u64).unwrap_or(0) as usize,
            document_count: row
                .get("document_count")
                .and_then(Value::as_u64)
                .unwrap_or(0) as usize,
            model_id: model_id.to_string(),
            dimension: VECTOR_DIMENSION,
            ..VectorStoreStats::default()
        })
    }

    fn upsert_batch(&self, records: &[VectorRecord]) -> Result<(), VectorStoreError> {
        if records.is_empty() {
            return Ok(());
        }
        let payload = records
            .iter()
            .map(|record| {
                json!({
                    "repository_id": record.repository_id,
                    "snapshot_id": record.snapshot_id,
                    "block_id": record.block_id,
                    "document_id": record.document_id,
                    "kind": record.kind,
                    "granularity": record.granularity,
                    "role": record.role,
                    "model_id": record.model_id,
                    "dimension": record.dimension,
                    "content_hash": record.content_hash,
                    "embedding": record.embedding,
                    "active": record.active
                })
            })
            .collect::<Vec<_>>();
        let response = self
            .request(
                reqwest::Method::POST,
                "/rest/v1/rag_embeddings?on_conflict=repository_id,model_id,block_id",
            )
            .header("Prefer", "resolution=merge-duplicates,return=minimal")
            .json(&payload)
            .send()
            .map_err(|error| Self::classify_error(&error))?;
        Self::require_success(response)?;
        Ok(())
    }

    fn query(&self, query: &VectorQuery) -> Result<Vec<VectorHit>, VectorStoreError> {
        let response = self
            .request(reqwest::Method::POST, "/rest/v1/rpc/match_rag_embeddings")
            .json(&json!({
                "p_repository_id": query.repository_id,
                "p_snapshot_id": query.snapshot_id,
                "p_model_id": query.model_id,
                "p_query_embedding": query.embedding,
                "p_match_count": query.limit.clamp(1, 200),
                "p_min_score": query.min_score,
                "p_kinds": query.filter.kinds,
                "p_document_ids": query.filter.document_ids,
                "p_granularities": query.filter.granularities,
                "p_roles": query.filter.roles
            }))
            .send()
            .map_err(|error| Self::classify_error(&error))?;
        let rows: Vec<Value> = Self::require_success(response)?.json().map_err(|_| {
            VectorStoreError::new(VectorStoreErrorKind::Corrupt, "远程向量结果格式无效")
        })?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                Some(VectorHit {
                    block_id: row.get("block_id")?.as_str()?.to_string(),
                    score: row.get("score")?.as_f64()?,
                    store: "pgvector".to_string(),
                    model_id: query.model_id.clone(),
                })
            })
            .collect())
    }

    fn delete_snapshot(
        &self,
        repository_id: &str,
        snapshot_id: &str,
        model_id: &str,
    ) -> Result<usize, VectorStoreError> {
        let mut url = reqwest::Url::parse(&format!("{}/rest/v1/rag_embeddings", self.endpoint))
            .map_err(|_| {
                VectorStoreError::new(VectorStoreErrorKind::Configuration, "远程向量地址无效")
            })?;
        url.query_pairs_mut()
            .append_pair("repository_id", &format!("eq.{repository_id}"))
            .append_pair("snapshot_id", &format!("eq.{snapshot_id}"))
            .append_pair("model_id", &format!("eq.{model_id}"));
        let response = self
            .client
            .delete(url)
            .header("apikey", &self.api_key)
            .bearer_auth(&self.api_key)
            .header("Prefer", "return=representation")
            .send()
            .map_err(|error| Self::classify_error(&error))?;
        let rows: Vec<Value> = Self::require_success(response)?.json().unwrap_or_default();
        Ok(rows.len())
    }
}

fn credential_entry() -> Result<Entry, String> {
    Entry::new(REMOTE_CREDENTIAL_SERVICE, REMOTE_CREDENTIAL_ACCOUNT)
        .map_err(|_| "Windows 凭据管理器初始化失败".to_string())
}

pub(crate) fn remote_key_configured() -> bool {
    load_remote_key().ok().flatten().is_some()
}

pub(crate) fn load_remote_key() -> Result<Option<String>, String> {
    match credential_entry()?.get_password() {
        Ok(value) => Ok((!value.trim().is_empty()).then_some(value)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(_) => Err("读取远程向量安全凭据失败".to_string()),
    }
}

pub(crate) fn save_remote_key(value: &str) -> Result<(), String> {
    let value = value.trim();
    if value.is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        return Err("远程向量凭据格式无效".to_string());
    }
    credential_entry()?
        .set_password(value)
        .map_err(|_| "保存远程向量安全凭据失败".to_string())
}

pub(crate) fn delete_remote_key() -> Result<(), String> {
    match credential_entry()?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_) => Err("清除远程向量安全凭据失败".to_string()),
    }
}

pub(crate) fn remote_store(endpoint: &str) -> Result<Option<PgVectorStore>, String> {
    remote_store_with_timeout(endpoint, 20)
}

pub(crate) fn remote_store_with_timeout(
    endpoint: &str,
    timeout_seconds: u64,
) -> Result<Option<PgVectorStore>, String> {
    let Some(api_key) = load_remote_key()? else {
        return Ok(None);
    };
    PgVectorStore::new(PgVectorConfig {
        endpoint: endpoint.to_string(),
        api_key,
        timeout_seconds,
    })
    .map(Some)
    .map_err(|error| error.to_string())
}

pub(crate) fn stale_snapshots(
    connection: &Connection,
    repository_id: &str,
    model_id: &str,
    active_snapshot: &str,
) -> Result<HashSet<String>, String> {
    let mut statement = connection
        .prepare(
            "SELECT DISTINCT snapshot_id FROM embedding_records_v2
             WHERE repository_id=?1 AND model_id=?2 AND snapshot_id<>?3",
        )
        .map_err(|error| format!("准备旧向量快照查询失败：{error}"))?;
    let result = statement
        .query_map(params![repository_id, model_id, active_snapshot], |row| {
            row.get(0)
        })
        .map_err(|error| format!("读取旧向量快照失败：{error}"))?
        .collect::<Result<_, _>>()
        .map_err(|error| format!("解析旧向量快照失败：{error}"));
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn read_http_request(stream: &mut std::net::TcpStream) -> String {
        let mut bytes = Vec::new();
        let mut chunk = [0_u8; 8_192];
        loop {
            let read = stream.read(&mut chunk).unwrap();
            if read == 0 {
                break;
            }
            bytes.extend_from_slice(&chunk[..read]);
            let Some(header_end) = bytes.windows(4).position(|part| part == b"\r\n\r\n") else {
                continue;
            };
            let headers = String::from_utf8_lossy(&bytes[..header_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.split_once(':').and_then(|(name, value)| {
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().ok())
                            .flatten()
                    })
                })
                .unwrap_or(0);
            if bytes.len() >= header_end + 4 + content_length {
                break;
            }
        }
        String::from_utf8(bytes).unwrap()
    }

    fn fake_pgvector_server(
        expected_requests: usize,
    ) -> (String, Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let endpoint = format!("http://{}", listener.local_addr().unwrap());
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&requests);
        let handle = thread::spawn(move || {
            for _ in 0..expected_requests {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_http_request(&mut stream);
                let request_line = request.lines().next().unwrap_or_default();
                let body = if request_line.contains("rag_embedding_stats") {
                    r#"[{"vector_count":2,"document_count":1}]"#
                } else if request_line.contains("match_rag_embeddings") {
                    r#"[{"block_id":"book:tsp","score":0.91}]"#
                } else if request_line.starts_with("DELETE ") {
                    r#"[{"block_id":"old"}]"#
                } else {
                    "[]"
                };
                captured.lock().unwrap().push(request);
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                )
                .unwrap();
            }
        });
        (endpoint, requests, handle)
    }

    fn record(block_id: &str, value: [f32; 2]) -> VectorRecord {
        let mut embedding = vec![0.0; VECTOR_DIMENSION];
        embedding[0] = value[0];
        embedding[1] = value[1];
        VectorRecord {
            repository_id: "repo".to_string(),
            snapshot_id: "snapshot".to_string(),
            block_id: block_id.to_string(),
            document_id: format!("doc:{block_id}"),
            kind: "book".to_string(),
            granularity: "semantic".to_string(),
            role: "algorithm".to_string(),
            model_id: "model".to_string(),
            dimension: VECTOR_DIMENSION,
            content_hash: format!("hash:{block_id}"),
            embedding,
            active: true,
        }
    }

    #[test]
    fn sqlite_store_round_trips_filters_and_cosine_order() {
        let connection = Connection::open_in_memory().unwrap();
        let store = SqliteVectorStore::new(&connection).unwrap();
        store
            .upsert_batch(&[record("near", [1.0, 0.0]), record("far", [0.0, 1.0])])
            .unwrap();
        let mut query_embedding = vec![0.0; VECTOR_DIMENSION];
        query_embedding[0] = 1.0;
        let hits = store
            .query(&VectorQuery {
                repository_id: "repo".to_string(),
                snapshot_id: "snapshot".to_string(),
                model_id: "model".to_string(),
                embedding: query_embedding,
                limit: 10,
                min_score: None,
                filter: VectorFilter {
                    kinds: vec!["book".to_string()],
                    granularities: vec!["semantic".to_string()],
                    ..VectorFilter::default()
                },
            })
            .unwrap();
        assert_eq!(hits[0].block_id, "near");
        assert_eq!(store.stats("repo", "model").unwrap().vector_count, 2);
        assert_eq!(
            store.delete_snapshot("repo", "snapshot", "model").unwrap(),
            2
        );
    }

    #[test]
    fn remote_errors_and_debug_never_contain_api_key() {
        let secret = "SECRET_VECTOR_TOKEN_123";
        let result = PgVectorStore::new(PgVectorConfig {
            endpoint: "not a url".to_string(),
            api_key: secret.to_string(),
            timeout_seconds: 5,
        });
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("invalid endpoint must fail"),
        };
        let rendered = format!("{error:?}");
        assert!(!rendered.contains(secret));
    }

    #[test]
    fn pgvector_http_adapter_upserts_filters_queries_and_cleans_snapshots() {
        let (endpoint, requests, server) = fake_pgvector_server(5);
        let store = PgVectorStore::new(PgVectorConfig {
            endpoint,
            api_key: "fixture-key".to_string(),
            timeout_seconds: 5,
        })
        .unwrap();
        assert!(store.health().unwrap().ready);
        assert_eq!(store.stats("repo", "model").unwrap().vector_count, 2);
        store
            .upsert_batch(&[record("book:tsp", [1.0, 0.0])])
            .unwrap();
        let mut embedding = vec![0.0; VECTOR_DIMENSION];
        embedding[0] = 1.0;
        let hits = store
            .query(&VectorQuery {
                repository_id: "repo".to_string(),
                snapshot_id: "snapshot".to_string(),
                model_id: "model".to_string(),
                embedding,
                limit: 8,
                min_score: Some(0.4),
                filter: VectorFilter {
                    kinds: vec!["book".to_string()],
                    document_ids: vec!["doc:book:tsp".to_string()],
                    granularities: vec!["semantic".to_string()],
                    roles: vec!["algorithm".to_string()],
                },
            })
            .unwrap();
        assert_eq!(hits[0].block_id, "book:tsp");
        assert_eq!(store.delete_snapshot("repo", "old", "model").unwrap(), 1);
        server.join().unwrap();

        let requests = requests.lock().unwrap().join("\n---\n");
        let lowercase_requests = requests.to_ascii_lowercase();
        assert!(lowercase_requests.contains("prefer: resolution=merge-duplicates,return=minimal"));
        assert!(requests.contains(r#""p_kinds":["book"]"#));
        assert!(requests.contains(r#""p_document_ids":["doc:book:tsp"]"#));
        assert!(requests.contains("snapshot_id=eq.old"));
        assert!(lowercase_requests.contains("apikey: fixture-key"));
    }

    #[test]
    fn invalid_vector_dimension_is_rejected() {
        let connection = Connection::open_in_memory().unwrap();
        let store = SqliteVectorStore::new(&connection).unwrap();
        let mut invalid = record("bad", [1.0, 0.0]);
        invalid.embedding.pop();
        assert_eq!(
            store.upsert_batch(&[invalid]).unwrap_err().kind,
            VectorStoreErrorKind::Corrupt
        );
    }
}
