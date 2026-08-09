use crate::process_support::{configure_background_command, configure_python_command};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use walkdir::WalkDir;

const MAX_PDF_BYTES: u64 = 200 * 1024 * 1024;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiteratureIngestSettings {
    pub startup_prompt_enabled: bool,
    pub auto_promote_enabled: bool,
    pub min_score: f64,
    pub max_auto_ingest: u32,
    pub providers: Vec<String>,
    pub since_year: Option<u32>,
    pub suppressed_prompt_date: String,
    pub last_attempt_at: String,
    pub last_success_at: String,
}

impl Default for LiteratureIngestSettings {
    fn default() -> Self {
        Self {
            startup_prompt_enabled: true,
            auto_promote_enabled: false,
            min_score: 8.0,
            max_auto_ingest: 3,
            providers: vec!["arxiv".into(), "openalex".into()],
            since_year: Some(2015),
            suppressed_prompt_date: String::new(),
            last_attempt_at: String::new(),
            last_success_at: String::new(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartupPromptState {
    pub should_prompt: bool,
    pub mode: String,
    pub suppressed_today: bool,
    pub settings: LiteratureIngestSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateMatch {
    pub kind: String,
    pub value: String,
    pub existing_id: String,
    pub existing_path: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManualFilePreflight {
    pub id: String,
    pub path: String,
    pub name: String,
    pub size: u64,
    pub mtime_ns: u128,
    pub sha256: String,
    pub valid: bool,
    pub selected: bool,
    pub errors: Vec<String>,
    pub duplicate_matches: Vec<DuplicateMatch>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManualImportSession {
    pub id: String,
    pub files: Vec<ManualFilePreflight>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiteratureCapability {
    pub id: String,
    pub available: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartLiteratureRunRequest {
    pub mode: String,
    #[serde(default)]
    pub candidate_ids: Vec<String>,
    #[serde(default)]
    pub manual_session_id: String,
    #[serde(default)]
    pub selected_file_ids: Vec<String>,
    #[serde(default)]
    pub force_duplicates: bool,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
}

pub fn db_schema(connection: &Connection) -> Result<(), String> {
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS literature_ingest_settings(
               repository_path TEXT PRIMARY KEY,
               startup_prompt_enabled INTEGER NOT NULL DEFAULT 1,
               auto_promote_enabled INTEGER NOT NULL DEFAULT 0,
               min_score REAL NOT NULL DEFAULT 8.0,
               max_auto_ingest INTEGER NOT NULL DEFAULT 3,
               providers_json TEXT NOT NULL DEFAULT '[\"arxiv\",\"openalex\"]',
               since_year INTEGER,
               suppressed_prompt_date TEXT NOT NULL DEFAULT '',
               last_attempt_at TEXT NOT NULL DEFAULT '',
               last_success_at TEXT NOT NULL DEFAULT '',
               updated_at TEXT NOT NULL DEFAULT ''
             );
             CREATE TABLE IF NOT EXISTS manual_import_sessions(
               id TEXT PRIMARY KEY,
               repository_path TEXT NOT NULL,
               files_json TEXT NOT NULL,
               created_at TEXT NOT NULL,
               consumed_at TEXT NOT NULL DEFAULT '',
               status TEXT NOT NULL DEFAULT 'prepared'
             );
             CREATE INDEX IF NOT EXISTS idx_manual_import_repo_status
               ON manual_import_sessions(repository_path,status,created_at);",
        )
        .map_err(|error| format!("literature ingest schema failed: {error}"))?;
    Ok(())
}

fn now_epoch() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

fn valid_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

pub fn get_settings(
    connection: &Connection,
    repository_path: &str,
) -> Result<LiteratureIngestSettings, String> {
    db_schema(connection)?;
    let loaded = connection
        .query_row(
            "SELECT startup_prompt_enabled,auto_promote_enabled,min_score,max_auto_ingest,
                    providers_json,since_year,suppressed_prompt_date,last_attempt_at,last_success_at
             FROM literature_ingest_settings WHERE repository_path=?1",
            [repository_path],
            |row| {
                let providers_json: String = row.get(4)?;
                Ok(LiteratureIngestSettings {
                    startup_prompt_enabled: row.get::<_, i64>(0)? != 0,
                    auto_promote_enabled: row.get::<_, i64>(1)? != 0,
                    min_score: row.get(2)?,
                    max_auto_ingest: row.get::<_, i64>(3)?.max(1) as u32,
                    providers: serde_json::from_str(&providers_json).unwrap_or_default(),
                    since_year: row.get::<_, Option<i64>>(5)?.map(|value| value as u32),
                    suppressed_prompt_date: row.get(6)?,
                    last_attempt_at: row.get(7)?,
                    last_success_at: row.get(8)?,
                })
            },
        )
        .optional()
        .map_err(|error| error.to_string())?;
    Ok(loaded.unwrap_or_default())
}

pub fn save_settings(
    connection: &Connection,
    repository_path: &str,
    settings: &LiteratureIngestSettings,
) -> Result<LiteratureIngestSettings, String> {
    db_schema(connection)?;
    if !(0.0..=100.0).contains(&settings.min_score) {
        return Err("相关度阈值必须在 0 到 100 之间".into());
    }
    if !(1..=20).contains(&settings.max_auto_ingest) {
        return Err("单次自动入库上限必须在 1 到 20 之间".into());
    }
    let allowed = ["arxiv", "openalex", "tavily", "serpapi"];
    if settings.providers.is_empty() {
        return Err("至少选择一个论文来源".into());
    }
    if settings
        .providers
        .iter()
        .any(|provider| !allowed.contains(&provider.as_str()))
    {
        return Err("包含不支持的论文来源".into());
    }
    connection
        .execute(
            "INSERT INTO literature_ingest_settings(
               repository_path,startup_prompt_enabled,auto_promote_enabled,min_score,max_auto_ingest,
               providers_json,since_year,suppressed_prompt_date,last_attempt_at,last_success_at,updated_at
             ) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
             ON CONFLICT(repository_path) DO UPDATE SET
               startup_prompt_enabled=excluded.startup_prompt_enabled,
               auto_promote_enabled=excluded.auto_promote_enabled,
               min_score=excluded.min_score,max_auto_ingest=excluded.max_auto_ingest,
               providers_json=excluded.providers_json,since_year=excluded.since_year,
               suppressed_prompt_date=excluded.suppressed_prompt_date,
               last_attempt_at=excluded.last_attempt_at,last_success_at=excluded.last_success_at,
               updated_at=excluded.updated_at",
            params![
                repository_path,
                settings.startup_prompt_enabled as i64,
                settings.auto_promote_enabled as i64,
                settings.min_score,
                settings.max_auto_ingest as i64,
                serde_json::to_string(&settings.providers).map_err(|error| error.to_string())?,
                settings.since_year.map(|value| value as i64),
                settings.suppressed_prompt_date,
                settings.last_attempt_at,
                settings.last_success_at,
                now_epoch(),
            ],
        )
        .map_err(|error| error.to_string())?;
    get_settings(connection, repository_path)
}

pub fn startup_prompt(
    connection: &Connection,
    repository_path: &str,
    local_date: &str,
) -> Result<StartupPromptState, String> {
    if !valid_date(local_date) {
        return Err("本地日期格式必须为 YYYY-MM-DD".into());
    }
    let settings = get_settings(connection, repository_path)?;
    let suppressed_today = settings.suppressed_prompt_date == local_date;
    Ok(StartupPromptState {
        should_prompt: settings.startup_prompt_enabled && !suppressed_today,
        mode: if settings.auto_promote_enabled {
            "automatic".into()
        } else {
            "prepare".into()
        },
        suppressed_today,
        settings,
    })
}

pub fn suppress_today(
    connection: &Connection,
    repository_path: &str,
    local_date: &str,
) -> Result<(), String> {
    if !valid_date(local_date) {
        return Err("本地日期格式必须为 YYYY-MM-DD".into());
    }
    let mut settings = get_settings(connection, repository_path)?;
    settings.suppressed_prompt_date = local_date.into();
    save_settings(connection, repository_path, &settings)?;
    Ok(())
}

fn file_hash(path: &Path) -> Result<String, String> {
    let mut stream = fs::File::open(path).map_err(|error| error.to_string())?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];
    loop {
        let count = stream
            .read(&mut buffer)
            .map_err(|error| error.to_string())?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn modified_nanos(metadata: &fs::Metadata) -> u128 {
    metadata
        .modified()
        .ok()
        .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
        .map(|value| value.as_nanos())
        .unwrap_or_default()
}

fn pdf_header_valid(path: &Path) -> bool {
    let Ok(mut stream) = fs::File::open(path) else {
        return false;
    };
    let mut header = [0_u8; 5];
    stream.read_exact(&mut header).is_ok() && &header == b"%PDF-"
}

fn existing_pdf_duplicates(
    root: &Path,
    incoming_name: &str,
    size: u64,
    hash: &str,
) -> Vec<DuplicateMatch> {
    let mut matches = Vec::new();
    for base in [
        root.join("raw/canonical"),
        root.join("raw/inbox/manual-drop"),
    ] {
        if !base.exists() {
            continue;
        }
        for entry in WalkDir::new(base).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file()
                || path
                    .extension()
                    .and_then(|value| value.to_str())
                    .map(str::to_ascii_lowercase)
                    .as_deref()
                    != Some("pdf")
            {
                continue;
            }
            let Ok(metadata) = path.metadata() else {
                continue;
            };
            let existing_name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("PDF");
            let existing_path = path
                .strip_prefix(root)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");
            if metadata.len() == size && file_hash(path).ok().as_deref() == Some(hash) {
                matches.push(DuplicateMatch {
                    kind: "sha256".into(),
                    value: hash.into(),
                    existing_id: path
                        .file_stem()
                        .and_then(|value| value.to_str())
                        .unwrap_or("pdf")
                        .into(),
                    existing_path: existing_path.clone(),
                    title: existing_name.into(),
                });
            }
            if existing_name.eq_ignore_ascii_case(incoming_name) {
                matches.push(DuplicateMatch {
                    kind: "file_name".into(),
                    value: incoming_name.into(),
                    existing_id: path
                        .file_stem()
                        .and_then(|value| value.to_str())
                        .unwrap_or("pdf")
                        .into(),
                    existing_path,
                    title: existing_name.into(),
                });
            }
        }
    }
    matches
}

pub fn create_manual_session(
    connection: &Connection,
    root: &Path,
    paths: Vec<PathBuf>,
) -> Result<ManualImportSession, String> {
    db_schema(connection)?;
    let repository_path = root.to_string_lossy().to_string();
    let mut files = Vec::new();
    for path in paths {
        let canonical =
            fs::canonicalize(&path).map_err(|error| format!("文件路径无效：{error}"))?;
        let metadata = canonical.metadata().map_err(|error| error.to_string())?;
        let mut errors = Vec::new();
        if !metadata.is_file()
            || canonical
                .extension()
                .and_then(|value| value.to_str())
                .map(str::to_ascii_lowercase)
                .as_deref()
                != Some("pdf")
        {
            errors.push("只支持 PDF 文件".into());
        }
        if metadata.len() == 0 {
            errors.push("PDF 文件为空".into());
        }
        if metadata.len() > MAX_PDF_BYTES {
            errors.push("超过 MinerU 200MB 限制".into());
        }
        if errors.is_empty() && !pdf_header_valid(&canonical) {
            errors.push("文件头不是有效 PDF".into());
        }
        let hash = if errors.is_empty() {
            file_hash(&canonical)?
        } else {
            String::new()
        };
        let duplicates = if hash.is_empty() {
            Vec::new()
        } else {
            existing_pdf_duplicates(
                root,
                canonical
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or("paper.pdf"),
                metadata.len(),
                &hash,
            )
        };
        let valid = errors.is_empty();
        files.push(ManualFilePreflight {
            id: Uuid::new_v4().to_string(),
            path: canonical.to_string_lossy().to_string(),
            name: canonical
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("paper.pdf")
                .into(),
            size: metadata.len(),
            mtime_ns: modified_nanos(&metadata),
            sha256: hash,
            valid,
            selected: valid && duplicates.is_empty(),
            errors,
            duplicate_matches: duplicates,
        });
    }
    let session = ManualImportSession {
        id: Uuid::new_v4().to_string(),
        files,
        created_at: now_epoch(),
    };
    connection
        .execute(
            "INSERT INTO manual_import_sessions(id,repository_path,files_json,created_at,status)
             VALUES(?1,?2,?3,?4,'prepared')",
            params![
                session.id,
                repository_path,
                serde_json::to_string(&session.files).map_err(|error| error.to_string())?,
                session.created_at,
            ],
        )
        .map_err(|error| error.to_string())?;
    Ok(session)
}

pub fn discard_manual_session(
    connection: &Connection,
    repository_path: &str,
    session_id: &str,
) -> Result<(), String> {
    connection
        .execute(
            "UPDATE manual_import_sessions SET status='discarded',consumed_at=?3
             WHERE id=?1 AND repository_path=?2 AND status='prepared'",
            params![session_id, repository_path, now_epoch()],
        )
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn load_manual_session(
    connection: &Connection,
    repository_path: &str,
    session_id: &str,
) -> Result<ManualImportSession, String> {
    connection
        .query_row(
            "SELECT files_json,created_at FROM manual_import_sessions
             WHERE id=?1 AND repository_path=?2 AND status='prepared'",
            params![session_id, repository_path],
            |row| {
                let files_json: String = row.get(0)?;
                let files: Vec<ManualFilePreflight> =
                    serde_json::from_str(&files_json).unwrap_or_default();
                Ok(ManualImportSession {
                    id: session_id.into(),
                    files,
                    created_at: row.get(1)?,
                })
            },
        )
        .optional()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "手动导入会话不存在、已使用或属于其他知识库".into())
}

fn run_python(root: &Path, arguments: &[String]) -> Result<String, String> {
    let mut command = Command::new("py");
    configure_python_command(&mut command);
    let output = command
        .arg("-3")
        .arg("tools/literature_ingest.py")
        .args(arguments)
        .current_dir(root)
        .output()
        .map_err(|error| format!("启动文献工具失败：{error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

pub fn list_candidates(root: &Path, settings: &LiteratureIngestSettings) -> Result<Value, String> {
    let stdout = run_python(
        root,
        &[
            "--repository".into(),
            root.to_string_lossy().to_string(),
            "list-candidates".into(),
            "--min-score".into(),
            settings.min_score.to_string(),
            "--max-auto-ingest".into(),
            settings.max_auto_ingest.to_string(),
            "--json".into(),
        ],
    )?;
    let payload: Value =
        serde_json::from_str(&stdout).map_err(|error| format!("候选 JSON 无效：{error}"))?;
    payload
        .get("candidates")
        .cloned()
        .filter(Value::is_array)
        .ok_or_else(|| "候选 JSON 缺少 candidates 数组".into())
}

pub fn update_triage(
    root: &Path,
    candidate_ids: &[String],
    status: &str,
    note: &str,
) -> Result<u64, String> {
    if !["pending", "selected", "rejected", "promoted"].contains(&status) {
        return Err("无效候选状态".into());
    }
    if candidate_ids.is_empty() {
        return Err("没有选择候选".into());
    }
    let mut args = vec![
        "--repository".into(),
        root.to_string_lossy().to_string(),
        "triage".into(),
        "--ids".into(),
        candidate_ids.join(","),
        "--status".into(),
        status.into(),
    ];
    if !note.trim().is_empty() {
        args.extend(["--note".into(), note.trim().into()]);
    }
    let stdout = run_python(root, &args)?;
    let value: Value = serde_json::from_str(&stdout).map_err(|error| error.to_string())?;
    Ok(value.get("updated").and_then(Value::as_u64).unwrap_or(0))
}

fn command_available(command: &str) -> bool {
    let mut process = Command::new(command);
    configure_background_command(&mut process);
    if matches!(command, "py" | "python" | "python3") {
        configure_python_command(&mut process);
    }
    process
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn capabilities(root: &Path) -> Vec<LiteratureCapability> {
    let python = command_available("py") && root.join("tools/literature_ingest.py").is_file();
    let codex = command_available("codex");
    let graphify =
        command_available("graphify") || root.join("tools/graphify_refresh.py").is_file();
    let mineru = root.join("tools/mineru_to_md.py").is_file();
    vec![
        LiteratureCapability {
            id: "discovery".into(),
            available: python,
            reason: if python {
                "可用".into()
            } else {
                "缺少 Python 或文献发现工具".into()
            },
        },
        LiteratureCapability {
            id: "download".into(),
            available: python,
            reason: if python {
                "可用".into()
            } else {
                "缺少 Python".into()
            },
        },
        LiteratureCapability {
            id: "parse".into(),
            available: python && mineru,
            reason: if python && mineru {
                "可用；运行时仍会检查 MinerU Key".into()
            } else {
                "缺少 MinerU 工具".into()
            },
        },
        LiteratureCapability {
            id: "compile".into(),
            available: codex,
            reason: if codex {
                "可用".into()
            } else {
                "缺少 Codex CLI".into()
            },
        },
        LiteratureCapability {
            id: "graph".into(),
            available: graphify,
            reason: if graphify {
                "可用".into()
            } else {
                "缺少 Graphify".into()
            },
        },
        LiteratureCapability {
            id: "full_ingest".into(),
            available: python && mineru && codex && graphify,
            reason: if python && mineru && codex && graphify {
                "完整入库可用".into()
            } else {
                "完整入库依赖不完整".into()
            },
        },
    ]
}

pub fn task_kind(mode: &str) -> Result<&'static str, String> {
    match mode {
        "prepare" => Ok("literature_prepare"),
        "automatic" => Ok("literature_auto_ingest"),
        "manual" => Ok("literature_manual_ingest"),
        "download" => Ok("literature_candidate_download"),
        "candidate" => Ok("literature_candidate_ingest"),
        _ => Err("无效文献入库模式".into()),
    }
}

pub fn build_run_manifest(
    connection: &Connection,
    root: &Path,
    request: &StartLiteratureRunRequest,
    destination: &Path,
) -> Result<PathBuf, String> {
    let task_kind_value = task_kind(&request.mode)?;
    if matches!(request.mode.as_str(), "candidate" | "download") && request.candidate_ids.is_empty()
    {
        return Err("没有选择候选文献".into());
    }
    if request.mode == "manual" && request.manual_session_id.trim().is_empty() {
        return Err("缺少手动导入会话".into());
    }
    let repository_path = root.to_string_lossy().to_string();
    let settings = get_settings(connection, &repository_path)?;
    let mut payload = json!({
        "kind": "literature_ingest_run",
        "mode": if request.mode == "candidate" { "candidate" } else { request.mode.as_str() },
        "taskKind": task_kind_value,
        "candidateIds": request.candidate_ids,
        "settings": settings,
        "createdAt": now_epoch(),
    });
    if request.mode == "manual" {
        let session =
            load_manual_session(connection, &repository_path, &request.manual_session_id)?;
        let selected: HashSet<&str> = request
            .selected_file_ids
            .iter()
            .map(String::as_str)
            .collect();
        let files: Vec<Value> = session
            .files
            .iter()
            .map(|file| {
                let explicitly_selected = selected.is_empty() && file.selected || selected.contains(file.id.as_str());
                let duplicate_allowed = file.duplicate_matches.is_empty() || request.force_duplicates;
                json!({
                    "id": file.id,
                    "path": file.path,
                    "size": file.size,
                    "mtimeNs": file.mtime_ns,
                    "sha256": file.sha256,
                    "selected": explicitly_selected && file.valid && duplicate_allowed,
                    "duplicateOverride": request.force_duplicates && !file.duplicate_matches.is_empty(),
                })
            })
            .collect();
        payload["batchId"] = json!(session.id);
        payload["files"] = Value::Array(files);
    }
    fs::create_dir_all(destination).map_err(|error| error.to_string())?;
    let path = destination.join(format!("{}.json", Uuid::new_v4()));
    let temporary = path.with_extension("json.tmp");
    fs::write(
        &temporary,
        serde_json::to_vec_pretty(&payload).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    fs::rename(&temporary, &path).map_err(|error| error.to_string())?;
    if request.mode == "manual" {
        connection
            .execute(
                "UPDATE manual_import_sessions SET status='consumed',consumed_at=?3 WHERE id=?1 AND repository_path=?2",
                params![request.manual_session_id, repository_path, now_epoch()],
            )
            .map_err(|error| error.to_string())?;
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn settings_are_repository_scoped_and_validated() {
        let connection = Connection::open_in_memory().unwrap();
        db_schema(&connection).unwrap();
        let mut first = LiteratureIngestSettings {
            auto_promote_enabled: true,
            min_score: 9.5,
            ..LiteratureIngestSettings::default()
        };
        save_settings(&connection, "repo-a", &first).unwrap();
        assert_eq!(get_settings(&connection, "repo-a").unwrap().min_score, 9.5);
        assert_eq!(get_settings(&connection, "repo-b").unwrap().min_score, 8.0);
        first.max_auto_ingest = 0;
        assert!(save_settings(&connection, "repo-a", &first).is_err());
    }

    #[test]
    fn startup_prompt_has_three_action_semantics() {
        let connection = Connection::open_in_memory().unwrap();
        db_schema(&connection).unwrap();
        let initial = startup_prompt(&connection, "repo", "2026-08-09").unwrap();
        assert!(initial.should_prompt);
        assert_eq!(initial.mode, "prepare");
        suppress_today(&connection, "repo", "2026-08-09").unwrap();
        assert!(
            !startup_prompt(&connection, "repo", "2026-08-09")
                .unwrap()
                .should_prompt
        );
        assert!(
            startup_prompt(&connection, "repo", "2026-08-10")
                .unwrap()
                .should_prompt
        );
    }

    #[test]
    fn manual_preflight_detects_hash_duplicate() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let canonical = root.join("raw/canonical/existing");
        fs::create_dir_all(&canonical).unwrap();
        let existing = canonical.join("paper.pdf");
        fs::write(&existing, b"%PDF-1.4\nfixture").unwrap();
        let incoming = root.join("incoming.pdf");
        fs::write(&incoming, b"%PDF-1.4\nfixture").unwrap();
        let connection = Connection::open_in_memory().unwrap();
        let session = create_manual_session(&connection, root, vec![incoming]).unwrap();
        assert!(session.files[0].valid);
        assert!(!session.files[0].selected);
        assert_eq!(session.files[0].duplicate_matches[0].kind, "sha256");
    }

    #[test]
    fn manual_preflight_detects_same_file_name_even_when_content_differs() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let canonical = root.join("raw/canonical/existing");
        fs::create_dir_all(&canonical).unwrap();
        fs::write(canonical.join("paper.pdf"), b"%PDF-1.4\nold").unwrap();
        let incoming_root = root.join("outside");
        fs::create_dir_all(&incoming_root).unwrap();
        let incoming = incoming_root.join("paper.pdf");
        fs::write(&incoming, b"%PDF-1.4\nnew content").unwrap();
        let connection = Connection::open_in_memory().unwrap();
        let session = create_manual_session(&connection, root, vec![incoming]).unwrap();
        assert!(session.files[0]
            .duplicate_matches
            .iter()
            .any(|item| item.kind == "file_name"));
    }

    #[test]
    fn manual_run_manifest_never_uses_unselected_files() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("raw/canonical")).unwrap();
        let incoming = root.join("incoming.pdf");
        fs::write(&incoming, b"%PDF-1.4\nfixture").unwrap();
        let connection = Connection::open_in_memory().unwrap();
        let session = create_manual_session(&connection, root, vec![incoming]).unwrap();
        let request = StartLiteratureRunRequest {
            mode: "manual".into(),
            candidate_ids: Vec::new(),
            manual_session_id: session.id,
            selected_file_ids: vec![session.files[0].id.clone()],
            force_duplicates: false,
            timeout_seconds: None,
        };
        let path = build_run_manifest(&connection, root, &request, &root.join("runs")).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        assert_eq!(value["files"][0]["selected"], true);
    }
}
