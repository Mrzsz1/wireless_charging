use reqwest::blocking::Client;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

const DEFAULT_MODEL: &str = "gpt-5.6-luna";
const DEFAULT_KEY_ENV: &str = "LUNA_API_KEY";
pub const PROVIDER_CODEX: &str = "codex-subscription";
pub const PROVIDER_API: &str = "compatible-api";
pub const PROVIDER_OFFLINE: &str = "offline-evidence";
const QA_SCHEMA_VERSION: i64 = 4;
const HISTORY_MESSAGE_LIMIT: usize = 8;
const HISTORY_CHARACTER_BUDGET: usize = 12_000;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LunaSettings {
    #[serde(default)]
    pub answer_provider: String,
    #[serde(default)]
    pub codex_model: String,
    pub endpoint: String,
    pub model: String,
    pub api_key_env: String,
    pub timeout_seconds: u64,
    pub max_output_tokens: u32,
    pub temperature: f64,
    #[serde(default)]
    pub api_key_configured: bool,
}

impl Default for LunaSettings {
    fn default() -> Self {
        Self {
            answer_provider: PROVIDER_OFFLINE.to_string(),
            codex_model: String::new(),
            endpoint: String::new(),
            model: DEFAULT_MODEL.to_string(),
            api_key_env: DEFAULT_KEY_ENV.to_string(),
            timeout_seconds: 90,
            max_output_tokens: 1800,
            temperature: 0.1,
            api_key_configured: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WaterlineSnapshot {
    pub source_count: usize,
    pub method_count: usize,
    pub synthesis_count: usize,
    pub chapter_count: usize,
    pub year_min: String,
    pub year_max: String,
    pub last_ingest_at: String,
    pub repository_path: String,
    pub captured_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceItem {
    pub id: String,
    pub kind: String,
    pub tier: String,
    pub title: String,
    pub snippet: String,
    pub score: f64,
    pub rank: usize,
    pub page_id: String,
    pub page_type: String,
    pub source_path: String,
    pub wikilink: String,
    pub book_id: String,
    pub chapter_id: String,
    pub physical_page_start: Option<i64>,
    pub physical_page_end: Option<i64>,
    pub markdown_path: String,
    pub pdf_path: String,
    pub node_id: String,
    pub source_location: String,
    pub relation: String,
    pub retrieval_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionContext {
    pub request_id: String,
    pub question: String,
    pub intent: String,
    pub conversation: Vec<ConversationTurn>,
    pub evidence: Vec<EvidenceItem>,
    pub waterline: WaterlineSnapshot,
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ConversationTurn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CitationValidation {
    pub cited_ids: Vec<String>,
    pub unknown_ids: Vec<String>,
    pub citation_precision: f64,
    pub has_citations: bool,
    pub supported: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatSessionSummary {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub message_count: usize,
    pub last_message_preview: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub status: String,
    pub created_at: String,
    pub error_code: String,
    pub error_message: String,
    pub provider: String,
    pub model: String,
    pub request_id: String,
    pub evidence: Vec<EvidenceItem>,
    pub waterline: Option<WaterlineSnapshot>,
    pub citation_validation: Option<CitationValidation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatSessionDetail {
    pub session: ChatSessionSummary,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AskRequest {
    pub question: String,
    pub session_id: Option<String>,
    pub evidence_limit: Option<usize>,
    #[serde(default)]
    pub repository_id: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AskResult {
    pub request_id: String,
    pub session_id: String,
    pub user_message: ChatMessage,
    pub assistant_message: ChatMessage,
    pub evidence: Vec<EvidenceItem>,
    pub waterline: WaterlineSnapshot,
    pub offline: bool,
    pub citation_validation: CitationValidation,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Clone)]
#[serde(
    tag = "type",
    content = "payload",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum AnswerStreamEvent {
    Started {
        request_id: String,
        session_id: String,
    },
    RetrievalStarted {
        request_id: String,
    },
    RetrievalCompleted {
        request_id: String,
        evidence: Vec<EvidenceItem>,
        waterline: WaterlineSnapshot,
    },
    Token {
        request_id: String,
        content: String,
    },
    Completed {
        request_id: String,
        result: AskResult,
    },
    Failed {
        request_id: String,
        code: String,
        message: String,
        retryable: bool,
    },
    Cancelled {
        request_id: String,
    },
}

#[derive(Clone)]
struct Candidate {
    kind: String,
    tier: String,
    title: String,
    snippet: String,
    score: f64,
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
    relation: String,
    retrieval_reason: String,
}

pub fn db_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS chat_sessions (
              id TEXT PRIMARY KEY,
              repository_id TEXT NOT NULL,
              title TEXT NOT NULL,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS chat_messages (
              id TEXT PRIMARY KEY,
              session_id TEXT NOT NULL,
              role TEXT NOT NULL,
              content TEXT NOT NULL,
              status TEXT NOT NULL,
              created_at TEXT NOT NULL,
              error_code TEXT NOT NULL DEFAULT '',
              error_message TEXT NOT NULL DEFAULT '',
              waterline TEXT NOT NULL DEFAULT '',
              provider TEXT NOT NULL DEFAULT '',
              model TEXT NOT NULL DEFAULT '',
              request_id TEXT NOT NULL DEFAULT '',
              citation_validation TEXT NOT NULL DEFAULT '',
              FOREIGN KEY(session_id) REFERENCES chat_sessions(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS chat_evidence (
              message_id TEXT NOT NULL,
              evidence_id TEXT NOT NULL,
              rank INTEGER NOT NULL,
              payload TEXT NOT NULL,
              PRIMARY KEY(message_id, evidence_id),
              FOREIGN KEY(message_id) REFERENCES chat_messages(id) ON DELETE CASCADE
            );
            CREATE TABLE IF NOT EXISTS app_settings (
              key TEXT PRIMARY KEY,
              value TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_chat_sessions_repository_updated
              ON chat_sessions(repository_id, updated_at DESC);
            CREATE INDEX IF NOT EXISTS idx_chat_messages_session_created
              ON chat_messages(session_id, created_at ASC);
            CREATE INDEX IF NOT EXISTS idx_chat_evidence_message_rank
              ON chat_evidence(message_id, rank ASC);
            ",
        )
        .map_err(|error| format!("初始化问答数据库失败：{error}"))?;
    let has_validation = connection
        .prepare("SELECT citation_validation FROM chat_messages LIMIT 0")
        .is_ok();
    if !has_validation {
        connection
            .execute(
                "ALTER TABLE chat_messages ADD COLUMN citation_validation TEXT NOT NULL DEFAULT ''",
                [],
            )
            .map_err(|error| format!("迁移问答引用校验字段失败：{error}"))?;
    }
    connection
        .pragma_update(None, "user_version", QA_SCHEMA_VERSION)
        .map_err(|error| format!("更新数据库版本失败：{error}"))?;
    Ok(())
}

fn now_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}

pub(crate) fn compact(value: &str, limit: usize) -> String {
    let normalized = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.chars().count() <= limit {
        return normalized;
    }
    normalized.chars().take(limit).collect::<String>() + "…"
}

pub fn repository_id(root: &Path) -> String {
    root.to_string_lossy()
        .replace('\\', "/")
        .trim_end_matches('/')
        .to_lowercase()
}

fn setting_map(connection: &Connection) -> Result<HashMap<String, String>, String> {
    let mut statement = connection
        .prepare("SELECT key,value FROM app_settings")
        .map_err(|error| format!("读取Luna设置失败：{error}"))?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|error| format!("查询Luna设置失败：{error}"))?;
    let mut values = HashMap::new();
    for row in rows {
        let (key, value) = row.map_err(|error| format!("解析Luna设置失败：{error}"))?;
        values.insert(key, value);
    }
    Ok(values)
}

pub fn get_luna_settings(
    connection: &Connection,
    root: &Path,
    codex_ready: bool,
) -> Result<LunaSettings, String> {
    let values = setting_map(connection)?;
    let scoped = |key: &str| {
        values
            .get(&format!("{key}::{}", repository_id(root)))
            .or_else(|| values.get(key))
    };
    let mut settings = LunaSettings::default();
    settings.endpoint = scoped("luna.endpoint").cloned().unwrap_or_default();
    settings.model = scoped("luna.model")
        .cloned()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_MODEL.to_string());
    settings.api_key_env = scoped("luna.api_key_env")
        .cloned()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_KEY_ENV.to_string());
    settings.timeout_seconds = scoped("luna.timeout_seconds")
        .and_then(|value| value.parse().ok())
        .unwrap_or(90);
    settings.max_output_tokens = scoped("luna.max_output_tokens")
        .and_then(|value| value.parse().ok())
        .unwrap_or(1800);
    settings.temperature = scoped("luna.temperature")
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.1);
    settings.api_key_configured = env::var(&settings.api_key_env)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    settings.codex_model = scoped("qa.codex_model").cloned().unwrap_or_default();
    settings.answer_provider = scoped("qa.answer_provider")
        .cloned()
        .filter(|value| {
            matches!(
                value.as_str(),
                PROVIDER_CODEX | PROVIDER_API | PROVIDER_OFFLINE
            )
        })
        .unwrap_or_else(|| {
            if !settings.endpoint.is_empty() && settings.api_key_configured {
                PROVIDER_API.to_string()
            } else if codex_ready {
                PROVIDER_CODEX.to_string()
            } else {
                PROVIDER_OFFLINE.to_string()
            }
        });
    Ok(settings)
}

pub fn save_luna_settings(
    connection: &Connection,
    root: &Path,
    mut settings: LunaSettings,
) -> Result<LunaSettings, String> {
    if !matches!(
        settings.answer_provider.as_str(),
        PROVIDER_CODEX | PROVIDER_API | PROVIDER_OFFLINE
    ) {
        return Err("不支持的问答引擎".to_string());
    }
    settings.codex_model = settings.codex_model.trim().to_string();
    if settings.codex_model.len() > 120 || settings.codex_model.chars().any(char::is_control) {
        return Err("Codex 模型覆盖格式无效".to_string());
    }
    settings.endpoint = settings.endpoint.trim().trim_end_matches('/').to_string();
    if !settings.endpoint.is_empty()
        && !settings.endpoint.starts_with("https://")
        && !settings.endpoint.starts_with("http://127.0.0.1")
        && !settings.endpoint.starts_with("http://localhost")
    {
        return Err("Luna endpoint 必须使用 HTTPS；本地 fixture 可使用 localhost".to_string());
    }
    settings.model = settings.model.trim().to_string();
    if settings.model.is_empty() {
        settings.model = DEFAULT_MODEL.to_string();
    }
    settings.api_key_env = settings.api_key_env.trim().to_string();
    if settings.api_key_env.is_empty()
        || !settings
            .api_key_env
            .chars()
            .all(|value| value.is_ascii_uppercase() || value.is_ascii_digit() || value == '_')
    {
        return Err("API Key 环境变量名仅允许大写字母、数字和下划线".to_string());
    }
    settings.timeout_seconds = settings.timeout_seconds.clamp(10, 300);
    settings.max_output_tokens = settings.max_output_tokens.clamp(256, 8000);
    settings.temperature = settings.temperature.clamp(0.0, 1.0);
    for (key, value) in [
        ("qa.answer_provider", settings.answer_provider.clone()),
        ("qa.codex_model", settings.codex_model.clone()),
        ("luna.endpoint", settings.endpoint.clone()),
        ("luna.model", settings.model.clone()),
        ("luna.api_key_env", settings.api_key_env.clone()),
        ("luna.timeout_seconds", settings.timeout_seconds.to_string()),
        (
            "luna.max_output_tokens",
            settings.max_output_tokens.to_string(),
        ),
        ("luna.temperature", settings.temperature.to_string()),
    ] {
        let key = format!("{key}::{}", repository_id(root));
        connection
            .execute(
                "INSERT INTO app_settings(key,value) VALUES(?1,?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                params![key, value],
            )
            .map_err(|error| format!("保存Luna设置失败：{error}"))?;
    }
    get_luna_settings(connection, root, false)
}

pub fn create_session(
    connection: &Connection,
    root: &Path,
    title: &str,
) -> Result<ChatSessionSummary, String> {
    create_session_with_id(connection, root, &Uuid::new_v4().to_string(), title)
}

fn create_session_with_id(
    connection: &Connection,
    root: &Path,
    id: &str,
    title: &str,
) -> Result<ChatSessionSummary, String> {
    let timestamp = now_string();
    let title = compact(title, 48);
    let title = if title.is_empty() {
        "新对话".to_string()
    } else {
        title
    };
    connection
        .execute(
            "INSERT INTO chat_sessions(id,repository_id,title,created_at,updated_at) VALUES(?1,?2,?3,?4,?5)",
            params![id, repository_id(root), title, timestamp, timestamp],
        )
        .map_err(|error| format!("创建问答会话失败：{error}"))?;
    Ok(ChatSessionSummary {
        id: id.to_string(),
        title,
        created_at: timestamp.clone(),
        updated_at: timestamp,
        message_count: 0,
        last_message_preview: String::new(),
    })
}

pub fn list_sessions(
    connection: &Connection,
    root: &Path,
    limit: usize,
) -> Result<Vec<ChatSessionSummary>, String> {
    let mut statement = connection
        .prepare(
            "SELECT s.id,s.title,s.created_at,s.updated_at,
                    COUNT(m.id),
                    COALESCE((SELECT content FROM chat_messages lm WHERE lm.session_id=s.id ORDER BY lm.created_at DESC, lm.rowid DESC LIMIT 1),'')
             FROM chat_sessions s
             LEFT JOIN chat_messages m ON m.session_id=s.id
             WHERE s.repository_id=?1
             GROUP BY s.id
             ORDER BY s.updated_at DESC
             LIMIT ?2",
        )
        .map_err(|error| format!("准备会话列表失败：{error}"))?;
    let rows = statement
        .query_map(
            params![repository_id(root), limit.clamp(1, 500) as i64],
            |row| {
                Ok(ChatSessionSummary {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    message_count: row.get::<_, i64>(4)?.max(0) as usize,
                    last_message_preview: compact(&row.get::<_, String>(5)?, 80),
                })
            },
        )
        .map_err(|error| format!("查询会话列表失败：{error}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析会话列表失败：{error}"))
}

pub fn rename_session(
    connection: &Connection,
    root: &Path,
    session_id: &str,
    title: &str,
) -> Result<(), String> {
    let changed = connection
        .execute(
            "UPDATE chat_sessions SET title=?3,updated_at=?4 WHERE id=?1 AND repository_id=?2",
            params![
                session_id,
                repository_id(root),
                compact(title, 80),
                now_string()
            ],
        )
        .map_err(|error| format!("重命名会话失败：{error}"))?;
    if changed == 0 {
        return Err("会话不存在或不属于当前知识库".to_string());
    }
    Ok(())
}

pub fn delete_session(
    connection: &Connection,
    root: &Path,
    session_id: &str,
) -> Result<(), String> {
    let changed = connection
        .execute(
            "DELETE FROM chat_sessions WHERE id=?1 AND repository_id=?2",
            params![session_id, repository_id(root)],
        )
        .map_err(|error| format!("删除会话失败：{error}"))?;
    if changed == 0 {
        return Err("会话不存在或不属于当前知识库".to_string());
    }
    Ok(())
}

fn evidence_for_message(
    connection: &Connection,
    message_id: &str,
) -> Result<Vec<EvidenceItem>, String> {
    let mut statement = connection
        .prepare("SELECT payload FROM chat_evidence WHERE message_id=?1 ORDER BY rank")
        .map_err(|error| format!("准备历史证据查询失败：{error}"))?;
    let rows = statement
        .query_map([message_id], |row| row.get::<_, String>(0))
        .map_err(|error| format!("查询历史证据失败：{error}"))?;
    let mut evidence = Vec::new();
    for row in rows {
        let payload = row.map_err(|error| format!("读取历史证据失败：{error}"))?;
        if let Ok(item) = serde_json::from_str::<EvidenceItem>(&payload) {
            evidence.push(item);
        }
    }
    Ok(evidence)
}

pub fn get_session(
    connection: &Connection,
    root: &Path,
    session_id: &str,
) -> Result<ChatSessionDetail, String> {
    let session = connection
        .query_row(
            "SELECT id,title,created_at,updated_at FROM chat_sessions WHERE id=?1 AND repository_id=?2",
            params![session_id, repository_id(root)],
            |row| {
                Ok(ChatSessionSummary {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                    updated_at: row.get(3)?,
                    message_count: 0,
                    last_message_preview: String::new(),
                })
            },
        )
        .optional()
        .map_err(|error| format!("读取会话失败：{error}"))?
        .ok_or_else(|| "会话不存在或不属于当前知识库".to_string())?;
    let mut statement = connection
        .prepare(
            "SELECT id,session_id,role,content,status,created_at,error_code,error_message,waterline,provider,model,request_id,citation_validation
             FROM chat_messages WHERE session_id=?1 ORDER BY created_at,rowid",
        )
        .map_err(|error| format!("准备历史消息查询失败：{error}"))?;
    let rows = statement
        .query_map([session_id], |row| {
            let waterline_json: String = row.get(8)?;
            Ok(ChatMessage {
                id: row.get(0)?,
                session_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                status: row.get(4)?,
                created_at: row.get(5)?,
                error_code: row.get(6)?,
                error_message: row.get(7)?,
                waterline: serde_json::from_str(&waterline_json).ok(),
                provider: row.get(9)?,
                model: row.get(10)?,
                request_id: row.get(11)?,
                citation_validation: serde_json::from_str(&row.get::<_, String>(12)?).ok(),
                evidence: Vec::new(),
            })
        })
        .map_err(|error| format!("查询历史消息失败：{error}"))?;
    let mut messages = Vec::new();
    for row in rows {
        let mut message = row.map_err(|error| format!("解析历史消息失败：{error}"))?;
        message.evidence = evidence_for_message(connection, &message.id)?;
        messages.push(message);
    }
    let mut session = session;
    session.message_count = messages.len();
    session.last_message_preview = messages
        .last()
        .map(|message| compact(&message.content, 80))
        .unwrap_or_default();
    Ok(ChatSessionDetail { session, messages })
}

pub fn conversation_history(
    connection: &Connection,
    root: &Path,
    session_id: Option<&str>,
) -> Result<Vec<ConversationTurn>, String> {
    let Some(session_id) = session_id.filter(|value| !value.trim().is_empty()) else {
        return Ok(Vec::new());
    };
    let owned = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM chat_sessions WHERE id=?1 AND repository_id=?2)",
            params![session_id, repository_id(root)],
            |row| row.get::<_, bool>(0),
        )
        .map_err(|error| format!("检查多轮会话失败：{error}"))?;
    if !owned {
        return Err("会话不存在或不属于当前知识库".to_string());
    }
    let mut statement = connection
        .prepare(
            "SELECT role,content FROM chat_messages
             WHERE session_id=?1 AND status='completed' AND role IN ('user','assistant')
             ORDER BY created_at DESC,rowid DESC LIMIT ?2",
        )
        .map_err(|error| format!("准备多轮历史失败：{error}"))?;
    let rows = statement
        .query_map(params![session_id, HISTORY_MESSAGE_LIMIT as i64], |row| {
            Ok(ConversationTurn {
                role: row.get(0)?,
                content: row.get(1)?,
            })
        })
        .map_err(|error| format!("读取多轮历史失败：{error}"))?;
    let mut newest = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析多轮历史失败：{error}"))?;
    newest.reverse();
    let mut remaining = HISTORY_CHARACTER_BUDGET;
    let mut selected = Vec::new();
    for mut turn in newest.into_iter().rev() {
        if remaining == 0 {
            break;
        }
        let count = turn.content.chars().count();
        if count > remaining {
            turn.content = turn.content.chars().take(remaining).collect();
        }
        remaining = remaining.saturating_sub(turn.content.chars().count());
        selected.push(turn);
    }
    selected.reverse();
    Ok(selected)
}

pub(crate) fn query_terms(question: &str) -> Vec<String> {
    let raw_terms = question
        .split(|value: char| !value.is_alphanumeric() && value != '-' && value != '_')
        .map(str::trim)
        .filter(|value| value.chars().count() >= 2)
        .map(|value| value.to_lowercase())
        .collect::<Vec<_>>();
    // Put bilingual domain expansions before long Chinese clauses. FTS5's
    // unicode tokenizer can keep a whole Chinese clause as one token; if raw
    // clauses consume the limit first, canonical English papers receive no
    // usable query term even though the Wiki summary is recalled correctly.
    let mut terms = Vec::new();
    for (needle, additions) in [
        ("开关组合", &["ccsp", "charger set", "charging cycle"][..]),
        (
            "已知轨迹",
            &["charging on the move", "known trajectory", "tunable power"][..],
        ),
        (
            "发起充电请求",
            &[
                "dynamic power distribution",
                "online charging request",
                "neighbor set",
            ][..],
        ),
        (
            "充电费",
            &[
                "cooperative charging",
                "charging as service",
                "cost sharing",
                "shapley",
            ][..],
        ),
        ("部分充电", &["partial charging", "on-demand charging"][..]),
        (
            "波干涉",
            &[
                "wave interference",
                "concurrent charging",
                "dynamic power distribution",
            ][..],
        ),
        (
            "城市路口",
            &[
                "infinite drive",
                "signalized intersections",
                "dynamic wireless charging",
            ][..],
        ),
        (
            "实时调度",
            &["real-time scheduling", "charging scheduling"][..],
        ),
    ] {
        if question.contains(needle) {
            terms.extend(additions.iter().map(|value| value.to_string()));
        }
    }
    for (needle, additions) in [
        ("无线充电", &["wireless charging", "wpt"][..]),
        ("调度", &["scheduling", "schedule"][..]),
        ("解决办法", &["solution", "algorithm", "method"][..]),
        ("算法", &["algorithm"][..]),
        ("近似", &["approximation", "approximate"][..]),
        ("博弈", &["game", "equilibrium"][..]),
        ("机制", &["mechanism", "mechanism design"][..]),
        ("在线", &["online"][..]),
        ("移动", &["mobile", "mobility"][..]),
        ("轨迹", &["trajectory"][..]),
        ("功率", &["power"][..]),
        ("请求", &["request", "service request"][..]),
        ("朝向", &["orientation", "directional"][..]),
        ("峰值", &["peak", "aoi"][..]),
        ("传输", &["transmission", "data transmission"][..]),
        ("收费", &["pricing", "payment"][..]),
        ("成本", &["cost"][..]),
        ("合作", &["cooperative", "nash"][..]),
        ("放置", &["placement"][..]),
        ("部署", &["deployment", "placement"][..]),
        ("干扰", &["interference"][..]),
        ("干涉", &["interference"][..]),
        ("并发", &["concurrent"][..]),
        ("安全", &["safety"][..]),
        ("公平", &["fairness", "utility"][..]),
        ("截止", &["deadline"][..]),
        ("道路", &["road", "dwpt"][..]),
        ("车辆", &["vehicle", "ev"][..]),
        ("覆盖", &["coverage"][..]),
    ] {
        if question.contains(needle) {
            terms.extend(additions.iter().map(|value| value.to_string()));
        }
    }
    terms.extend(raw_terms);
    let mut seen = HashSet::new();
    terms.retain(|value| seen.insert(value.clone()));
    terms.truncate(20);
    terms
}

pub(crate) fn fts_query(terms: &[String]) -> String {
    terms
        .iter()
        .filter_map(|term| {
            let clean = term.replace('"', "").trim().to_string();
            (!clean.is_empty()).then(|| format!("\"{clean}\"*"))
        })
        .collect::<Vec<_>>()
        .join(" OR ")
}

fn intent(question: &str) -> String {
    let lower = question.to_lowercase();
    if lower.contains("新颖") || lower.contains("novel") || lower.contains("做过") {
        "novelty".to_string()
    } else if lower.contains("关系") || lower.contains("区别") || lower.contains("比较") {
        "relationship".to_string()
    } else {
        "solve".to_string()
    }
}

fn intent_bonus(intent: &str, candidate: &Candidate) -> f64 {
    match intent {
        "novelty" => match (candidate.kind.as_str(), candidate.page_type.as_str()) {
            ("paper", _) | ("wiki", "source") | ("wiki", "synthesis") => 0.42,
            ("graph", _) => 0.08,
            _ => 0.0,
        },
        "relationship" => match candidate.kind.as_str() {
            "graph" => 0.48,
            "wiki" => 0.24,
            _ => 0.0,
        },
        _ => match (candidate.kind.as_str(), candidate.page_type.as_str()) {
            ("wiki", "method") | ("paper", _) => 0.34,
            ("book", _) => 0.18,
            _ => 0.0,
        },
    }
}

fn apply_intent(intent: &str, candidates: &mut [Candidate]) {
    for candidate in candidates {
        let bonus = intent_bonus(intent, candidate);
        candidate.score += bonus;
        if bonus > 0.0 {
            candidate
                .retrieval_reason
                .push_str(&format!("；{intent} 意图加权 +{bonus:.2}"));
        }
    }
}

fn wiki_candidates(connection: &Connection, terms: &[String]) -> Result<Vec<Candidate>, String> {
    if terms.is_empty() {
        return Ok(Vec::new());
    }
    let query = fts_query(terms);
    let mut statement = connection
        .prepare(
            "SELECT p.id,p.page_type,p.title,p.source_path,
                    snippet(pages_fts,2,'','',' … ',24),bm25(pages_fts,0.0,8.0,1.0,3.0)
             FROM pages_fts JOIN pages p ON p.id=pages_fts.page_id
             WHERE pages_fts MATCH ?1
             ORDER BY bm25(pages_fts,0.0,8.0,1.0,3.0) LIMIT 30",
        )
        .map_err(|error| format!("准备Wiki证据检索失败：{error}"))?;
    let rows = statement
        .query_map([query], |row| {
            let page_type: String = row.get(1)?;
            let title: String = row.get(2)?;
            let rank: f64 = row.get(5)?;
            let title_lower = title.to_lowercase();
            let title_hits = terms
                .iter()
                .filter(|term| title_lower.contains(term.as_str()))
                .count();
            let page_bonus = match page_type.as_str() {
                "source" | "synthesis" => 0.35,
                "method" => 0.3,
                _ => 0.1,
            };
            Ok(Candidate {
                kind: "wiki".to_string(),
                tier: if page_type == "method" {
                    "transferable_method".to_string()
                } else {
                    "direct".to_string()
                },
                title,
                snippet: compact(&row.get::<_, String>(4)?, 480),
                score: 1.0 / (1.0 + rank.abs()) + page_bonus + title_hits as f64 * 0.28,
                page_id: row.get(0)?,
                page_type: page_type.clone(),
                source_path: row.get(3)?,
                wikilink: String::new(),
                book_id: String::new(),
                chapter_id: String::new(),
                physical_page_start: None,
                physical_page_end: None,
                markdown_path: String::new(),
                pdf_path: String::new(),
                node_id: String::new(),
                source_location: String::new(),
                relation: String::new(),
                retrieval_reason: format!("Wiki FTS5 命中；页面类型 {page_type}"),
            })
        })
        .map_err(|error| format!("检索Wiki证据失败：{error}"))?;
    let mut candidates = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析Wiki证据失败：{error}"))?;
    for candidate in &mut candidates {
        candidate.wikilink = format!("[[{}]]", candidate.page_id.trim_end_matches(".md"));
    }
    Ok(candidates)
}

fn paper_candidates(connection: &Connection, terms: &[String]) -> Result<Vec<Candidate>, String> {
    if terms.is_empty() {
        return Ok(Vec::new());
    }
    let index_available = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='paper_sections_fts')",
            [],
            |row| row.get::<_, bool>(0),
        )
        .map_err(|error| format!("检查论文原文索引失败：{error}"))?;
    if !index_available {
        return Ok(Vec::new());
    }
    let query = fts_query(terms);
    let mut statement = connection
        .prepare(
            "SELECT s.id,s.page_id,s.title,s.section_title,s.source_path,s.pdf_path,
                    s.line_start,s.line_end,
                    snippet(paper_sections_fts,3,'','',' … ',72),
                    bm25(paper_sections_fts,0.0,7.0,5.0,1.0)
             FROM paper_sections_fts
             JOIN paper_sections s ON s.id=paper_sections_fts.section_id
             WHERE paper_sections_fts MATCH ?1
             ORDER BY bm25(paper_sections_fts,0.0,7.0,5.0,1.0) LIMIT 24",
        )
        .map_err(|error| format!("准备论文原文证据检索失败：{error}"))?;
    let rows = statement
        .query_map([query], |row| {
            let section_id: String = row.get(0)?;
            let page_id: String = row.get(1)?;
            let paper_title: String = row.get(2)?;
            let section_title: String = row.get(3)?;
            let source_path: String = row.get(4)?;
            let rank: f64 = row.get(9)?;
            let line_start: i64 = row.get(6)?;
            let line_end: i64 = row.get(7)?;
            Ok(Candidate {
                kind: "paper".to_string(),
                tier: "primary_source".to_string(),
                title: format!("{paper_title} · {section_title}"),
                snippet: compact(&row.get::<_, String>(8)?, 1_200),
                score: 1.0 / (1.0 + rank.abs()) + 0.32,
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
                relation: String::new(),
                retrieval_reason:
                    "canonical 论文原文章节 FTS5 命中；可直接支撑事实并回到 raw 行号核验"
                        .to_string(),
            })
        })
        .map_err(|error| format!("检索论文原文证据失败：{error}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析论文原文证据失败：{error}"))
}

fn linked_paper_candidates(
    connection: &Connection,
    wiki: &[Candidate],
) -> Result<Vec<Candidate>, String> {
    let mut candidates = Vec::new();
    let mut seen = HashSet::new();
    for source in wiki
        .iter()
        .filter(|candidate| candidate.page_type == "source")
        .filter(|candidate| seen.insert(candidate.page_id.clone()))
        .take(8)
    {
        let candidate = connection
            .query_row(
                "SELECT id,page_id,title,section_title,source_path,pdf_path,line_start,line_end,body
                 FROM paper_sections
                 WHERE page_id=?1
                   AND lower(section_title) NOT LIKE '%references%'
                   AND lower(section_title) NOT LIKE '%acknowledg%'
                 ORDER BY CASE
                   WHEN lower(section_title) LIKE '%abstract%' THEN 0
                   WHEN lower(section_title) LIKE '%problem%' THEN 1
                   WHEN lower(section_title) LIKE '%model%' THEN 2
                   WHEN lower(section_title) LIKE '%introduction%' THEN 3
                   ELSE 4 END,
                   line_start
                 LIMIT 1",
                [&source.page_id],
                |row| {
                    let section_id: String = row.get(0)?;
                    let page_id: String = row.get(1)?;
                    let paper_title: String = row.get(2)?;
                    let section_title: String = row.get(3)?;
                    let source_path: String = row.get(4)?;
                    let line_start: i64 = row.get(6)?;
                    let line_end: i64 = row.get(7)?;
                    Ok(Candidate {
                        kind: "paper".to_string(),
                        tier: "primary_source".to_string(),
                        title: format!("{paper_title} · {section_title}"),
                        snippet: compact(&row.get::<_, String>(8)?, 1_200),
                        score: source.score + 0.18,
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
                        source_location: format!(
                            "{section_title} · 原文第 {line_start}–{line_end} 行"
                        ),
                        relation: "wiki_source_to_primary".to_string(),
                        retrieval_reason:
                            "Wiki source 命中后下钻其 canonical 原文；用于保证摘要结论可回到 primary source 核验"
                                .to_string(),
                    })
                },
            )
            .optional()
            .map_err(|error| format!("按Wiki source下钻论文原文失败：{error}"))?;
        if let Some(candidate) = candidate {
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

fn book_candidates(connection: &Connection, terms: &[String]) -> Result<Vec<Candidate>, String> {
    if terms.is_empty() {
        return Ok(Vec::new());
    }
    let query = fts_query(terms);
    let mut statement = connection
        .prepare(
            "SELECT c.id,c.book_id,c.title,c.markdown_path,c.pdf_path,
                    c.physical_page_start,c.physical_page_end,
                    snippet(book_chapters_fts,2,'','',' … ',24),bm25(book_chapters_fts,0.0,6.0,1.0),b.title
             FROM book_chapters_fts
             JOIN book_chapters c ON c.id=book_chapters_fts.chapter_id
             JOIN books b ON b.id=c.book_id
             WHERE book_chapters_fts MATCH ?1
             ORDER BY bm25(book_chapters_fts,0.0,6.0,1.0) LIMIT 16",
        )
        .map_err(|error| format!("准备核心书籍检索失败：{error}"))?;
    let rows = statement
        .query_map([query], |row| {
            let chapter_title: String = row.get(2)?;
            let book_title: String = row.get(9)?;
            let rank: f64 = row.get(8)?;
            Ok(Candidate {
                kind: "book".to_string(),
                tier: "theory".to_string(),
                title: format!("{book_title} · {chapter_title}"),
                snippet: compact(&row.get::<_, String>(7)?, 480),
                score: 1.0 / (1.0 + rank.abs()) + 0.22,
                page_id: String::new(),
                page_type: String::new(),
                source_path: row.get(3)?,
                wikilink: String::new(),
                book_id: row.get(1)?,
                chapter_id: row.get(0)?,
                physical_page_start: row.get(5)?,
                physical_page_end: row.get(6)?,
                markdown_path: row.get(3)?,
                pdf_path: row.get(4)?,
                node_id: String::new(),
                source_location: String::new(),
                relation: String::new(),
                retrieval_reason: "核心书籍章节 FTS5 命中；提供 physical pages".to_string(),
            })
        })
        .map_err(|error| format!("检索核心书籍失败：{error}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("解析核心书籍证据失败：{error}"))
}

fn graph_candidates(connection: &Connection, root: &Path, terms: &[String]) -> Vec<Candidate> {
    let graph_path = root.join("graphify-out/graph.json");
    let Ok(content) = fs::read_to_string(graph_path) else {
        return Vec::new();
    };
    let Ok(payload) = serde_json::from_str::<Value>(&content) else {
        return Vec::new();
    };
    let nodes = payload
        .get("nodes")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let links = payload
        .get("links")
        .or_else(|| payload.get("edges"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let labels = nodes
        .iter()
        .filter_map(|node| {
            Some((
                node.get("id")?.as_str()?.to_string(),
                node.get("label")
                    .or_else(|| node.get("name"))?
                    .as_str()?
                    .to_string(),
            ))
        })
        .collect::<HashMap<_, _>>();
    let mut adjacency: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for link in links {
        let Some(source) = link.get("source").and_then(Value::as_str) else {
            continue;
        };
        let Some(target) = link.get("target").and_then(Value::as_str) else {
            continue;
        };
        let relation = link
            .get("relation")
            .and_then(Value::as_str)
            .unwrap_or("related_to")
            .to_string();
        adjacency
            .entry(source.to_string())
            .or_default()
            .push((target.to_string(), relation.clone()));
        adjacency
            .entry(target.to_string())
            .or_default()
            .push((source.to_string(), relation));
    }
    let mut candidates = Vec::new();
    for node in nodes {
        let label = node
            .get("label")
            .or_else(|| node.get("name"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        let source_file = node
            .get("source_file")
            .or_else(|| node.get("sourceFile"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        let normalized_source = source_file.replace('\\', "/");
        let source_path =
            if normalized_source.starts_with("wiki/") && normalized_source.ends_with(".md") {
                normalized_source.clone()
            } else if normalized_source.contains("/wiki/") && normalized_source.ends_with(".md") {
                normalized_source
                    .split_once("/wiki/")
                    .map(|(_, suffix)| format!("wiki/{suffix}"))
                    .unwrap_or_default()
            } else {
                String::new()
            };
        if source_path.is_empty() || !root.join(&source_path).is_file() {
            continue;
        }
        let haystack = format!("{label} {source_file}").to_lowercase();
        let hits = terms
            .iter()
            .filter(|term| haystack.contains(term.as_str()))
            .count();
        if hits == 0 {
            continue;
        }
        let node_id = node
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or(label)
            .to_string();
        let neighbors = adjacency
            .get(&node_id)
            .into_iter()
            .flatten()
            .take(4)
            .map(|(id, relation)| {
                format!(
                    "{}→{}",
                    relation,
                    labels.get(id).map(String::as_str).unwrap_or(id)
                )
            })
            .collect::<Vec<_>>();
        let community = node
            .get("community")
            .and_then(Value::as_i64)
            .map(|value| value.to_string())
            .unwrap_or_default();
        // Graphify source paths are provenance hints rather than stable desktop
        // identifiers. Resolve them through the canonical pages index so a
        // graph citation always opens an existing Wiki page.
        let indexed_page = connection
            .query_row(
                "SELECT id,page_type FROM pages WHERE replace(source_path,'\\','/')=?1 LIMIT 1",
                [&source_path],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()
            .ok()
            .flatten();
        let Some((page_id, page_type)) = indexed_page else {
            continue;
        };
        candidates.push(Candidate {
            kind: "graph".to_string(),
            tier: "graph_hint".to_string(),
            title: label.to_string(),
            snippet: if neighbors.is_empty() { "Graphify 关系候选；需回到 Wiki 正文核验。".to_string() } else { format!("Graphify 一跳关系：{}", neighbors.join("；")) },
            score: 0.15 + hits as f64 * 0.04 + (!neighbors.is_empty()) as usize as f64 * 0.08,
            page_id: page_id.clone(),
            page_type,
            source_path,
            wikilink: format!("[[{page_id}]]"),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id,
            source_location: node
                .get("source_location")
                .or_else(|| node.get("sourceLocation"))
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            relation: if neighbors.is_empty() { "graph_node".to_string() } else { "graph_one_hop".to_string() },
            retrieval_reason: format!("Graphify 节点命中 {hits} 个查询词；community={community}；一跳关系 {}；已回链 Wiki，仅作关系提示", neighbors.join("、")),
        });
    }
    candidates.sort_by(|left, right| right.score.total_cmp(&left.score));
    candidates.truncate(5);
    candidates
}

fn waterline(connection: &Connection, root: &Path) -> Result<WaterlineSnapshot, String> {
    let count = |page_type: &str| -> Result<usize, String> {
        connection
            .query_row(
                "SELECT COUNT(*) FROM pages WHERE page_type=?1",
                [page_type],
                |row| row.get::<_, i64>(0),
            )
            .map(|value| value.max(0) as usize)
            .map_err(|error| format!("读取库水位失败：{error}"))
    };
    let chapter_count = connection
        .query_row("SELECT COUNT(*) FROM book_chapters", [], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap_or(0)
        .max(0) as usize;
    let (year_min, year_max) = connection
        .query_row(
            "SELECT COALESCE(MIN(NULLIF(year,'')),''),COALESCE(MAX(NULLIF(year,'')),'') FROM pages WHERE page_type='source'",
            [],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .unwrap_or_default();
    let last_ingest_at = connection
        .query_row(
            "SELECT COALESCE(MAX(modified_at),'') FROM pages",
            [],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_default();
    Ok(WaterlineSnapshot {
        source_count: count("source")?,
        method_count: count("method")?,
        synthesis_count: count("synthesis")?,
        chapter_count,
        year_min,
        year_max,
        last_ingest_at,
        repository_path: root.to_string_lossy().to_string(),
        captured_at: now_string(),
    })
}

pub fn prepare_question(
    connection: &Connection,
    root: &Path,
    question: &str,
    limit: usize,
) -> Result<QuestionContext, String> {
    let question = question.trim();
    if question.chars().count() < 2 {
        return Err("问题至少需要两个字符".to_string());
    }
    let question_intent = intent(question);
    let terms = query_terms(question);
    let wiki = wiki_candidates(connection, &terms)?;
    let mut candidates = wiki.clone();
    candidates.extend(paper_candidates(connection, &terms)?);
    candidates.extend(linked_paper_candidates(connection, &wiki)?);
    candidates.extend(book_candidates(connection, &terms)?);
    candidates.extend(graph_candidates(connection, root, &terms));
    apply_intent(&question_intent, &mut candidates);
    candidates.sort_by(|left, right| right.score.total_cmp(&left.score));
    let mut seen = HashSet::new();
    candidates.retain(|candidate| {
        let key = if candidate.kind == "paper" {
            format!("paper:{}", candidate.node_id)
        } else if candidate.kind == "graph" {
            format!("graph:{}", candidate.node_id)
        } else if !candidate.page_id.is_empty() {
            format!("wiki:{}", candidate.page_id)
        } else if !candidate.chapter_id.is_empty() {
            format!("book:{}", candidate.chapter_id)
        } else {
            format!("graph:{}", candidate.node_id)
        };
        seen.insert(key)
    });
    let maximum = limit.clamp(4, 30);
    let mut selected = candidates.iter().take(maximum).cloned().collect::<Vec<_>>();
    // Preserve source diversity after global ranking: when a channel produced a
    // useful candidate, the final evidence package keeps at least one Wiki and
    // one core-book result instead of letting a single channel occupy all slots.
    let required_kinds: &[&str] = match question_intent.as_str() {
        "relationship" => &["wiki", "graph"],
        _ => &["wiki", "paper", "book"],
    };
    for required_kind in required_kinds {
        if selected
            .iter()
            .any(|candidate| candidate.kind == *required_kind)
        {
            continue;
        }
        if let Some(candidate) = candidates
            .iter()
            .find(|candidate| candidate.kind == *required_kind)
            .cloned()
        {
            if selected.len() >= maximum {
                selected.pop();
            }
            selected.push(candidate);
        }
    }
    // Solution and novelty questions need a reusable method when one was
    // recalled; raw source evidence alone does not answer "how" questions.
    if matches!(question_intent.as_str(), "solution" | "novelty")
        && !selected
            .iter()
            .any(|candidate| candidate.page_type == "method")
    {
        if let Some(method) = candidates
            .iter()
            .find(|candidate| candidate.page_type == "method")
            .cloned()
        {
            if selected.len() >= maximum {
                selected.pop();
            }
            selected.push(method);
        }
    }
    // A paper reached through a Wiki source is most useful as an auditable
    // pair: the structured page explains the claim and the canonical section
    // verifies it. Keep both sides instead of allowing paper boosts to evict
    // the very Wiki page that supplied the provenance link.
    let paired_pages = selected
        .iter()
        .filter(|candidate| {
            candidate.kind == "paper" && candidate.relation == "wiki_source_to_primary"
        })
        .map(|candidate| candidate.page_id.clone())
        .take(8)
        .collect::<Vec<_>>();
    for page_id in paired_pages {
        if selected
            .iter()
            .any(|candidate| candidate.kind == "wiki" && candidate.page_id == page_id)
        {
            continue;
        }
        let Some(wiki_pair) = candidates
            .iter()
            .find(|candidate| candidate.kind == "wiki" && candidate.page_id == page_id)
            .cloned()
        else {
            continue;
        };
        if selected.len() >= maximum {
            let removable = selected
                .iter()
                .rposition(|candidate| candidate.kind == "graph")
                .or_else(|| {
                    selected.iter().rposition(|candidate| {
                        candidate.kind == "wiki"
                            && candidate.page_type != "source"
                            && candidate.page_id != page_id
                            && selected
                                .iter()
                                .filter(|item| item.kind == candidate.kind)
                                .count()
                                > 1
                    })
                })
                .or_else(|| {
                    selected.iter().rposition(|candidate| {
                        !(candidate.kind == "paper"
                            && candidate.relation == "wiki_source_to_primary"
                            && candidate.page_id == page_id)
                            && selected
                                .iter()
                                .filter(|item| item.kind == candidate.kind)
                                .count()
                                > 1
                    })
                });
            if let Some(index) = removable {
                selected.remove(index);
            }
        }
        if selected.len() < maximum {
            selected.push(wiki_pair);
        }
    }
    selected.sort_by(|left, right| right.score.total_cmp(&left.score));
    let evidence = selected
        .into_iter()
        .enumerate()
        .map(|(index, candidate)| EvidenceItem {
            id: format!("E{}", index + 1),
            kind: candidate.kind,
            tier: candidate.tier,
            title: candidate.title,
            snippet: candidate.snippet,
            score: candidate.score,
            rank: index + 1,
            page_id: candidate.page_id,
            page_type: candidate.page_type,
            source_path: candidate.source_path,
            wikilink: candidate.wikilink,
            book_id: candidate.book_id,
            chapter_id: candidate.chapter_id,
            physical_page_start: candidate.physical_page_start,
            physical_page_end: candidate.physical_page_end,
            markdown_path: candidate.markdown_path,
            pdf_path: candidate.pdf_path,
            node_id: candidate.node_id,
            source_location: candidate.source_location,
            relation: candidate.relation,
            retrieval_reason: candidate.retrieval_reason,
        })
        .collect();
    Ok(QuestionContext {
        request_id: Uuid::new_v4().to_string(),
        question: question.to_string(),
        intent: question_intent,
        conversation: Vec::new(),
        evidence,
        waterline: waterline(connection, root)?,
        generated_at: now_string(),
    })
}

pub fn validate_citations(answer: &str, evidence: &[EvidenceItem]) -> CitationValidation {
    let known = evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect::<HashSet<_>>();
    let bytes = answer.as_bytes();
    let mut cited = Vec::new();
    let mut index = 0;
    while index + 3 < bytes.len() {
        if bytes[index] == b'[' && bytes.get(index + 1) == Some(&b'E') {
            let mut end = index + 2;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > index + 2 && bytes.get(end) == Some(&b']') {
                let id = &answer[index + 1..end];
                if !cited.iter().any(|value| value == id) {
                    cited.push(id.to_string());
                }
                index = end + 1;
                continue;
            }
        }
        index += 1;
    }
    let unknown_ids = cited
        .iter()
        .filter(|id| !known.contains(id.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let valid = cited.len().saturating_sub(unknown_ids.len());
    let has_citations = !cited.is_empty();
    CitationValidation {
        cited_ids: cited.clone(),
        unknown_ids: unknown_ids.clone(),
        citation_precision: if cited.is_empty() {
            0.0
        } else {
            valid as f64 / cited.len() as f64
        },
        has_citations,
        supported: unknown_ids.is_empty() && (evidence.is_empty() || has_citations),
    }
}

pub fn offline_answer(context: &QuestionContext) -> String {
    let waterline = &context.waterline;
    let mut answer = format!(
        "当前处于离线证据模式。库水位：{} 篇 source、{} 个 method、{} 个 synthesis、{} 个核心书籍章节；年份范围 {}–{}。\n\n",
        waterline.source_count,
        waterline.method_count,
        waterline.synthesis_count,
        waterline.chapter_count,
        if waterline.year_min.is_empty() { "未知" } else { &waterline.year_min },
        if waterline.year_max.is_empty() { "未知" } else { &waterline.year_max },
    );
    if context.evidence.is_empty() {
        answer.push_str("当前索引未召回可用证据。请换用更具体的模型、约束、目标或算法关键词。");
        return answer;
    }
    answer.push_str("已召回以下可审计证据，配置 Luna 后可基于同一证据包生成完整回答：\n\n");
    for item in &context.evidence {
        let location = if item.kind == "book" {
            match (item.physical_page_start, item.physical_page_end) {
                (Some(start), Some(end)) => format!("，PDF physical pages {start}–{end}"),
                (Some(start), None) => format!("，PDF physical page {start}"),
                _ => String::new(),
            }
        } else if item.kind == "paper" {
            format!("，{}，{}", item.wikilink, item.source_location)
        } else if !item.wikilink.is_empty() {
            format!("，{}", item.wikilink)
        } else {
            String::new()
        };
        answer.push_str(&format!(
            "- [{}] {}{}：{}\n",
            item.id,
            item.title,
            location,
            compact(&item.snippet, 220)
        ));
    }
    answer
}

fn build_prompt(context: &QuestionContext) -> String {
    let history = if context.conversation.is_empty() {
        "（无历史）".to_string()
    } else {
        context
            .conversation
            .iter()
            .map(|turn| {
                let role = if turn.role == "assistant" {
                    "助手"
                } else {
                    "用户"
                };
                format!("{role}：{}", turn.content)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let mut evidence_text = String::new();
    for item in &context.evidence {
        evidence_text.push_str(&format!(
            "[{}] kind={} tier={} title={} source={} location={} pages={:?}-{:?}\n{}\n\n",
            item.id,
            item.kind,
            item.tier,
            item.title,
            if !item.wikilink.is_empty() {
                &item.wikilink
            } else {
                &item.source_path
            },
            item.source_location,
            item.physical_page_start,
            item.physical_page_end,
            item.snippet,
        ));
    }
    format!(
        "会话历史（仅用于理解指代，不是本轮证据；历史引用编号不得沿用）：\n{}\n\n问题：{}\n意图：{}\n库水位：source={} method={} synthesis={} chapters={} years={}..{}\n\n本轮证据：\n{}",
        history,
        context.question,
        context.intent,
        context.waterline.source_count,
        context.waterline.method_count,
        context.waterline.synthesis_count,
        context.waterline.chapter_count,
        context.waterline.year_min,
        context.waterline.year_max,
        evidence_text,
    )
}

pub fn build_codex_prompt(context: &QuestionContext) -> String {
    format!(
        "你是无线充电调度科研知识库的回答模型。不要调用工具，不要读取文件，不要执行命令，也不要修改任何内容。只能依据下面提供的编号证据回答；每个事实判断必须引用 [E#]。必须先报告库水位，并按‘库内直接证据、相似模型、可迁移算法、核心书籍理论基础、库内尚未覆盖’组织。kind=paper 是 canonical 论文原文章节，可直接支撑其片段包含的事实，并应保留 sourceLocation；kind=wiki 是结构化导航与综合；kind=book 是核心书籍理论证据。Graphify 证据只能作为关系提示，不能单独支撑事实。库内未见不等于全球没有。不要编造引用编号。\n\n{}",
        build_prompt(context)
    )
}

pub fn stream_luna<F>(
    settings: &LunaSettings,
    context: &QuestionContext,
    cancelled: &AtomicBool,
    mut on_token: F,
) -> Result<String, String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    if settings.endpoint.is_empty() {
        return Err("LUNA_NOT_CONFIGURED: 尚未配置 Luna endpoint".to_string());
    }
    let api_key = env::var(&settings.api_key_env)
        .map_err(|_| format!("LUNA_KEY_MISSING: 环境变量 {} 未设置", settings.api_key_env))?;
    if api_key.trim().is_empty() {
        return Err(format!(
            "LUNA_KEY_MISSING: 环境变量 {} 为空",
            settings.api_key_env
        ));
    }
    let client = Client::builder()
        .timeout(Duration::from_secs(settings.timeout_seconds))
        .build()
        .map_err(|error| format!("LUNA_CLIENT_ERROR: {error}"))?;
    let system = "你是无线充电调度科研知识库的问答模型。只能依据编号证据回答；每个事实判断必须引用 [E#]。必须先报告库水位，并按‘库内直接证据、相似模型、可迁移算法、核心书籍理论基础、库内尚未覆盖’组织。Graphify 证据只能作为关系提示，不能单独支撑事实。库内未见不等于全球没有。不要编造引用编号。";
    let response = client
        .post(&settings.endpoint)
        .bearer_auth(api_key)
        .json(&json!({
            "model": settings.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": build_prompt(context)}
            ],
            "temperature": settings.temperature,
            "max_tokens": settings.max_output_tokens,
            "stream": true
        }))
        .send()
        .map_err(|error| format!("LUNA_NETWORK_ERROR: {error}"))?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("LUNA_HTTP_ERROR: HTTP {}", status.as_u16()));
    }
    let reader = BufReader::new(response);
    let mut answer = String::new();
    for line in reader.lines() {
        if cancelled.load(Ordering::SeqCst) {
            return Err("LUNA_CANCELLED: 用户停止了生成".to_string());
        }
        let line = line.map_err(|error| format!("LUNA_STREAM_ERROR: {error}"))?;
        let data = line
            .strip_prefix("data:")
            .map(str::trim)
            .unwrap_or(line.trim());
        if data.is_empty() || data.starts_with(':') {
            continue;
        }
        if data == "[DONE]" {
            break;
        }
        let Ok(payload) = serde_json::from_str::<Value>(data) else {
            continue;
        };
        let content = payload
            .pointer("/choices/0/delta/content")
            .or_else(|| payload.pointer("/choices/0/message/content"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        if content.is_empty() {
            continue;
        }
        answer.push_str(content);
        on_token(content)?;
    }
    let answer = answer.trim().to_string();
    if answer.is_empty() {
        Err("LUNA_RESPONSE_ERROR: 流式响应未包含回答文本".to_string())
    } else {
        Ok(answer)
    }
}

#[allow(clippy::too_many_arguments)]
fn make_message(
    session_id: &str,
    role: &str,
    content: String,
    status: &str,
    provider: &str,
    model: &str,
    request_id: &str,
    evidence: Vec<EvidenceItem>,
    waterline: Option<WaterlineSnapshot>,
    citation_validation: Option<CitationValidation>,
) -> ChatMessage {
    ChatMessage {
        id: Uuid::new_v4().to_string(),
        session_id: session_id.to_string(),
        role: role.to_string(),
        content,
        status: status.to_string(),
        created_at: now_string(),
        error_code: String::new(),
        error_message: String::new(),
        provider: provider.to_string(),
        model: model.to_string(),
        request_id: request_id.to_string(),
        evidence,
        waterline,
        citation_validation,
    }
}

pub fn persist_exchange(
    connection: &mut Connection,
    root: &Path,
    session_id: Option<&str>,
    context: &QuestionContext,
    answer: String,
    provider: &str,
    model: &str,
) -> Result<AskResult, String> {
    let citation_validation = validate_citations(&answer, &context.evidence);
    if !citation_validation.supported {
        let reason = if !citation_validation.unknown_ids.is_empty() {
            format!(
                "回答包含未知证据编号：{}",
                citation_validation.unknown_ids.join(", ")
            )
        } else {
            "回答未引用本轮任何有效证据".to_string()
        };
        return Err(format!("CITATION_VALIDATION_FAILED: {reason}"));
    }
    let session = if let Some(id) = session_id {
        let existing = connection
            .query_row(
                "SELECT id FROM chat_sessions WHERE id=?1 AND repository_id=?2",
                params![id, repository_id(root)],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| format!("检查会话失败：{error}"))?;
        match existing {
            Some(existing) => existing,
            None => create_session_with_id(connection, root, id, &context.question)?.id,
        }
    } else {
        create_session(connection, root, &context.question)?.id
    };
    let user_message = make_message(
        &session,
        "user",
        context.question.clone(),
        "completed",
        "local",
        "retrieval",
        &context.request_id,
        Vec::new(),
        Some(context.waterline.clone()),
        None,
    );
    let assistant_message = make_message(
        &session,
        "assistant",
        answer,
        "completed",
        provider,
        model,
        &context.request_id,
        context.evidence.clone(),
        Some(context.waterline.clone()),
        Some(citation_validation.clone()),
    );
    let tx = connection
        .transaction()
        .map_err(|error| format!("开启会话保存事务失败：{error}"))?;
    for message in [&user_message, &assistant_message] {
        tx.execute(
            "INSERT INTO chat_messages(id,session_id,role,content,status,created_at,error_code,error_message,waterline,provider,model,request_id,citation_validation)
             VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
            params![
                message.id,
                message.session_id,
                message.role,
                message.content,
                message.status,
                message.created_at,
                message.error_code,
                message.error_message,
                message.waterline.as_ref().and_then(|value| serde_json::to_string(value).ok()).unwrap_or_default(),
                message.provider,
                message.model,
                message.request_id,
                message.citation_validation.as_ref().and_then(|value| serde_json::to_string(value).ok()).unwrap_or_default(),
            ],
        )
        .map_err(|error| format!("保存会话消息失败：{error}"))?;
    }
    for item in &context.evidence {
        tx.execute(
            "INSERT INTO chat_evidence(message_id,evidence_id,rank,payload) VALUES(?1,?2,?3,?4)",
            params![
                assistant_message.id,
                item.id,
                item.rank as i64,
                serde_json::to_string(item).unwrap_or_default(),
            ],
        )
        .map_err(|error| format!("保存回答证据失败：{error}"))?;
    }
    tx.execute(
        "UPDATE chat_sessions SET updated_at=?2 WHERE id=?1",
        params![session, now_string()],
    )
    .map_err(|error| format!("更新会话时间失败：{error}"))?;
    tx.commit()
        .map_err(|error| format!("提交会话保存事务失败：{error}"))?;
    Ok(AskResult {
        request_id: context.request_id.clone(),
        session_id: session,
        user_message,
        assistant_message,
        evidence: context.evidence.clone(),
        waterline: context.waterline.clone(),
        offline: provider == "offline-evidence",
        citation_validation,
    })
}

pub fn persist_failure(
    connection: &mut Connection,
    root: &Path,
    session_id: Option<&str>,
    request_id: &str,
    code: &str,
    message: &str,
    provider: &str,
) -> Result<ChatMessage, String> {
    let existing = session_id.and_then(|id| {
        connection
            .query_row(
                "SELECT id FROM chat_sessions WHERE id=?1 AND repository_id=?2",
                params![id, repository_id(root)],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .ok()
            .flatten()
    });
    let Some(session) = existing else {
        return Err("失败请求没有可写入的既有会话".to_string());
    };
    let mut failure = make_message(
        &session,
        "assistant",
        "本轮回答生成失败。".to_string(),
        "failed",
        provider,
        "",
        request_id,
        Vec::new(),
        None,
        None,
    );
    failure.error_code = compact(code, 80);
    failure.error_message = compact(message, 240);
    let tx = connection
        .transaction()
        .map_err(|error| format!("开启失败状态事务失败：{error}"))?;
    tx.execute(
        "INSERT INTO chat_messages(id,session_id,role,content,status,created_at,error_code,error_message,waterline,provider,model,request_id,citation_validation)
         VALUES(?1,?2,?3,?4,?5,?6,?7,?8,'',?9,'',?10,'')",
        params![failure.id, failure.session_id, failure.role, failure.content, failure.status,
            failure.created_at, failure.error_code, failure.error_message, failure.provider, failure.request_id],
    ).map_err(|error| format!("保存失败状态失败：{error}"))?;
    tx.execute(
        "UPDATE chat_sessions SET updated_at=?2 WHERE id=?1",
        params![session, now_string()],
    )
    .map_err(|error| format!("更新失败会话时间失败：{error}"))?;
    tx.commit()
        .map_err(|error| format!("提交失败状态失败：{error}"))?;
    Ok(failure)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn candidate(kind: &str, page_type: &str) -> Candidate {
        Candidate {
            kind: kind.to_string(),
            tier: String::new(),
            title: "candidate".to_string(),
            snippet: String::new(),
            score: 1.0,
            page_id: String::new(),
            page_type: page_type.to_string(),
            source_path: String::new(),
            wikilink: String::new(),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id: String::new(),
            source_location: String::new(),
            relation: String::new(),
            retrieval_reason: String::new(),
        }
    }

    fn evidence(id: &str) -> EvidenceItem {
        EvidenceItem {
            id: id.to_string(),
            kind: "wiki".to_string(),
            tier: "direct".to_string(),
            title: "Evidence".to_string(),
            snippet: "Supported statement".to_string(),
            score: 1.0,
            rank: 1,
            page_id: "source.md".to_string(),
            page_type: "source".to_string(),
            source_path: "wiki/sources/source.md".to_string(),
            wikilink: "[[source]]".to_string(),
            book_id: String::new(),
            chapter_id: String::new(),
            physical_page_start: None,
            physical_page_end: None,
            markdown_path: String::new(),
            pdf_path: String::new(),
            node_id: String::new(),
            source_location: String::new(),
            relation: String::new(),
            retrieval_reason: String::new(),
        }
    }

    fn test_db() -> (tempfile::TempDir, Connection) {
        let root = tempdir().unwrap();
        fs::create_dir_all(root.path().join("graphify-out")).unwrap();
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                "CREATE TABLE pages(id TEXT PRIMARY KEY,page_type TEXT,title TEXT,year TEXT,body TEXT,source_path TEXT,modified_at TEXT);
                 CREATE VIRTUAL TABLE pages_fts USING fts5(page_id UNINDEXED,title,body,keywords);
                 CREATE TABLE books(id TEXT PRIMARY KEY,title TEXT);
                 CREATE TABLE book_chapters(id TEXT PRIMARY KEY,book_id TEXT,chapter_number INTEGER,title TEXT,markdown_path TEXT,pdf_path TEXT,physical_page_start INTEGER,physical_page_end INTEGER);
                 CREATE VIRTUAL TABLE book_chapters_fts USING fts5(chapter_id UNINDEXED,title,body);",
            )
            .unwrap();
        db_schema(&connection).unwrap();
        (root, connection)
    }

    #[test]
    fn migrates_chat_schema_without_touching_knowledge_tables() {
        let (_root, connection) = test_db();
        let version: i64 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        assert_eq!(version, QA_SCHEMA_VERSION);
        let pages: i64 = connection
            .query_row("SELECT COUNT(*) FROM pages", [], |row| row.get(0))
            .unwrap();
        assert_eq!(pages, 0);
    }

    #[test]
    fn migrates_existing_chat_messages_with_citation_validation() {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                "CREATE TABLE chat_messages(
                    id TEXT PRIMARY KEY,session_id TEXT,role TEXT,content TEXT,status TEXT,
                    created_at TEXT,error_code TEXT,error_message TEXT,waterline TEXT,
                    provider TEXT,model TEXT,request_id TEXT
                );",
            )
            .unwrap();
        db_schema(&connection).unwrap();
        assert!(connection
            .prepare("SELECT citation_validation FROM chat_messages LIMIT 0")
            .is_ok());
    }

    #[test]
    fn sessions_are_isolated_by_repository() {
        let (root, connection) = test_db();
        let other = tempdir().unwrap();
        create_session(&connection, root.path(), "first").unwrap();
        create_session(&connection, other.path(), "second").unwrap();
        let sessions = list_sessions(&connection, root.path(), 20).unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].title, "first");
    }

    #[test]
    fn prepares_ranked_wiki_and_book_evidence() {
        let (root, connection) = test_db();
        connection.execute("INSERT INTO pages VALUES('mtd-demo.md','method','Online Scheduling','2024','online scheduling algorithm','wiki/methods/mtd-demo.md','1')", []).unwrap();
        connection.execute("INSERT INTO pages_fts VALUES('mtd-demo.md','Online Scheduling','online scheduling algorithm','wireless charging')", []).unwrap();
        connection
            .execute(
                "INSERT INTO books VALUES('algorithmic-game-theory','Algorithmic Game Theory')",
                [],
            )
            .unwrap();
        connection.execute("INSERT INTO book_chapters VALUES('agt-ch01','algorithmic-game-theory',1,'Introduction','chapters/ch01.md','book.pdf',10,20)", []).unwrap();
        connection.execute("INSERT INTO book_chapters_fts VALUES('agt-ch01','Introduction','online game algorithm scheduling')", []).unwrap();
        let context =
            prepare_question(&connection, root.path(), "online scheduling algorithm", 10).unwrap();
        assert!(context.evidence.iter().any(|item| item.kind == "wiki"));
        assert!(context.evidence.iter().any(|item| item.kind == "book"));
        assert!(context.evidence.iter().all(|item| item.id.starts_with('E')));
    }

    #[test]
    fn conversation_history_is_repository_scoped_bounded_and_completed_only() {
        let (root, connection) = test_db();
        let session = create_session(&connection, root.path(), "history").unwrap();
        for index in 0..10 {
            connection
                .execute(
                    "INSERT INTO chat_messages(id,session_id,role,content,status,created_at)
                 VALUES(?1,?2,?3,?4,'completed',?5)",
                    params![
                        format!("m{index}"),
                        session.id,
                        if index % 2 == 0 { "user" } else { "assistant" },
                        format!("turn-{index}"),
                        format!("{index:02}")
                    ],
                )
                .unwrap();
        }
        connection
            .execute(
                "INSERT INTO chat_messages(id,session_id,role,content,status,created_at)
             VALUES('failed',?1,'assistant','must-not-appear','failed','99')",
                [&session.id],
            )
            .unwrap();
        let history = conversation_history(&connection, root.path(), Some(&session.id)).unwrap();
        assert_eq!(history.len(), HISTORY_MESSAGE_LIMIT);
        assert_eq!(history.first().unwrap().content, "turn-2");
        assert_eq!(history.last().unwrap().content, "turn-9");
        assert!(history.iter().all(|turn| turn.content != "must-not-appear"));

        let other = tempdir().unwrap();
        assert!(conversation_history(&connection, other.path(), Some(&session.id)).is_err());
    }

    #[test]
    fn prompt_includes_history_but_marks_it_non_evidence() {
        let (root, connection) = test_db();
        let mut context = prepare_question(&connection, root.path(), "charging", 4).unwrap();
        context.conversation = vec![ConversationTurn {
            role: "user".to_string(),
            content: "Earlier constraint".to_string(),
        }];
        let prompt = build_codex_prompt(&context);
        assert!(prompt.contains("Earlier constraint"));
        assert!(prompt.contains("历史引用编号不得沿用"));
    }

    #[test]
    fn citation_validation_rejects_missing_and_unknown_ids() {
        let items = vec![evidence("E1"), evidence("E2")];
        let valid = validate_citations("Claim [E1] and detail [E2].", &items);
        assert!(valid.supported);
        assert_eq!(valid.citation_precision, 1.0);

        let unknown = validate_citations("Claim [E9].", &items);
        assert!(!unknown.supported);
        assert_eq!(unknown.unknown_ids, vec!["E9"]);
        assert!(!validate_citations("Claim without citation.", &items).supported);
    }

    #[test]
    fn intent_weights_change_candidate_priority() {
        let graph = candidate("graph", "concept");
        let method = candidate("wiki", "method");
        let paper = candidate("paper", "source");
        assert!(intent_bonus("relationship", &graph) > intent_bonus("relationship", &method));
        assert!(intent_bonus("solve", &method) > intent_bonus("solve", &graph));
        assert!(intent_bonus("novelty", &paper) > intent_bonus("novelty", &graph));
    }

    #[test]
    fn graph_candidates_require_canonical_wiki_source_and_resolve_page_id() {
        let (root, connection) = test_db();
        fs::create_dir_all(root.path().join("wiki/methods")).unwrap();
        fs::write(root.path().join("wiki/methods/charging.md"), "# Charging").unwrap();
        connection.execute(
            "INSERT INTO pages VALUES('charging.md','method','Charging','2026','charging relation','wiki/methods/charging.md','1')",
            [],
        ).unwrap();
        fs::write(
            root.path().join("graphify-out/graph.json"),
            serde_json::to_vec(&json!({
                "nodes": [
                    {"id":"n1","label":"charging","source_file":"wiki/methods/charging.md","community":7},
                    {"id":"n2","label":"scheduler","source_file":"raw/not-canonical.md"}
                ],
                "links": [{"source":"n1","target":"n2","relation":"uses"}]
            })).unwrap(),
        ).unwrap();
        let found = graph_candidates(&connection, root.path(), &["charging".to_string()]);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].page_id, "charging.md");
        assert_eq!(found[0].relation, "graph_one_hop");
        assert!(found[0].retrieval_reason.contains("community=7"));
        assert!(found[0].snippet.contains("uses→scheduler"));
    }

    #[test]
    fn failed_generation_is_persisted_without_an_optimistic_user_message() {
        let (root, mut connection) = test_db();
        let session = create_session(&connection, root.path(), "failure").unwrap();
        persist_failure(
            &mut connection,
            root.path(),
            Some(&session.id),
            "request",
            "LUNA_HTTP_ERROR",
            "HTTP 500",
            PROVIDER_API,
        )
        .unwrap();
        let detail = get_session(&connection, root.path(), &session.id).unwrap();
        assert_eq!(detail.messages.len(), 1);
        assert_eq!(detail.messages[0].status, "failed");
        assert_eq!(detail.messages[0].error_code, "LUNA_HTTP_ERROR");
    }

    #[test]
    fn api_key_is_never_persisted_in_settings() {
        let (root, connection) = test_db();
        let settings = save_luna_settings(
            &connection,
            root.path(),
            LunaSettings {
                endpoint: "https://example.test/v1/chat/completions".to_string(),
                ..LunaSettings::default()
            },
        )
        .unwrap();
        assert_eq!(settings.api_key_env, DEFAULT_KEY_ENV);
        let stored: String = connection
            .query_row(
                "SELECT group_concat(value,' ') FROM app_settings",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(!stored.contains("Bearer"));
    }

    #[test]
    fn qa_provider_defaults_to_codex_only_when_ready() {
        let (root, connection) = test_db();
        assert_eq!(
            get_luna_settings(&connection, root.path(), true)
                .unwrap()
                .answer_provider,
            PROVIDER_CODEX
        );
        assert_eq!(
            get_luna_settings(&connection, root.path(), false)
                .unwrap()
                .answer_provider,
            PROVIDER_OFFLINE
        );
    }

    #[test]
    fn qa_settings_are_repository_scoped_with_legacy_fallback() {
        let (root, connection) = test_db();
        let repository_a = root.path().join("repository-a");
        let repository_b = root.path().join("repository-b");
        save_luna_settings(
            &connection,
            &repository_a,
            LunaSettings {
                answer_provider: PROVIDER_CODEX.to_string(),
                codex_model: "subscription-model".to_string(),
                ..LunaSettings::default()
            },
        )
        .unwrap();
        let first = get_luna_settings(&connection, &repository_a, false).unwrap();
        let second = get_luna_settings(&connection, &repository_b, false).unwrap();
        assert_eq!(first.answer_provider, PROVIDER_CODEX);
        assert_eq!(first.codex_model, "subscription-model");
        assert_eq!(second.answer_provider, PROVIDER_OFFLINE);
        assert!(second.codex_model.is_empty());
    }
}
