use crate::codex_subscription;
use crate::process_support::{configure_background_command, configure_python_command};
use crate::search_credentials;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tauri::ipc::Channel;
use uuid::Uuid;

pub const COMPILE_SCHEMA_VERSION: i64 = 6;
static WRITE_LOCKS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
const MAX_BACKUP_BYTES: u64 = 16 * 1024 * 1024;

fn file_hash(path: &Path) -> String {
    let data = fs::read(path).unwrap_or_default();
    let mut h = DefaultHasher::new();
    data.hash(&mut h);
    format!("{:016x}", h.finish())
}

struct RepositoryWriteGuard {
    key: Option<String>,
}

impl RepositoryWriteGuard {
    fn acquire(root: &Path, write: bool) -> Result<Self, String> {
        if !write {
            return Ok(Self { key: None });
        }
        let key = root.to_string_lossy().to_string();
        let mut locks = WRITE_LOCKS
            .get_or_init(|| Mutex::new(HashSet::new()))
            .lock()
            .map_err(|_| "write lock poisoned".to_string())?;
        if !locks.insert(key.clone()) {
            return Err("repository already has a write task running".into());
        }
        Ok(Self { key: Some(key) })
    }
}

impl Drop for RepositoryWriteGuard {
    fn drop(&mut self) {
        if let Some(key) = self.key.take() {
            if let Ok(mut locks) = WRITE_LOCKS
                .get_or_init(|| Mutex::new(HashSet::new()))
                .lock()
            {
                locks.remove(&key);
            }
        }
    }
}

#[derive(Clone)]
struct FileSnapshot {
    hash: String,
    backup_path: String,
}

fn visit_files(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_file() {
        output.push(path.to_path_buf());
        return Ok(());
    }
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path).map_err(|error| format!("scan artifact failed: {error}"))? {
        let entry = entry.map_err(|error| format!("scan artifact failed: {error}"))?;
        let child = entry.path();
        if child.is_dir() {
            visit_files(&child, output)?;
        } else if child.is_file() {
            output.push(child);
        }
    }
    Ok(())
}

fn snapshot_scope(
    root: &Path,
    scope: &Path,
    backup_root: Option<&Path>,
) -> Result<HashMap<String, FileSnapshot>, String> {
    let mut files = Vec::new();
    visit_files(scope, &mut files)?;
    let mut snapshots = HashMap::new();
    for file in files {
        let relative = file
            .strip_prefix(root)
            .map_err(|_| "artifact path is outside repository".to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let mut backup_path = String::new();
        if let Some(backup_root) = backup_root {
            let size = file.metadata().map(|value| value.len()).unwrap_or(u64::MAX);
            if size <= MAX_BACKUP_BYTES {
                let target = backup_root.join(&relative);
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|error| format!("create rollback backup failed: {error}"))?;
                }
                fs::copy(&file, &target)
                    .map_err(|error| format!("create rollback backup failed: {error}"))?;
                backup_path = target.to_string_lossy().to_string();
            }
        }
        snapshots.insert(
            relative,
            FileSnapshot {
                hash: file_hash(&file),
                backup_path,
            },
        );
    }
    Ok(snapshots)
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileCapability {
    pub task_kind: String,
    pub label: String,
    pub description: String,
    pub available: bool,
    pub reason: String,
    pub writes: bool,
    pub network: bool,
    pub requires_input: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartCompileRequest {
    pub task_kind: String,
    #[serde(default)]
    pub input_path: Option<String>,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub download: bool,
    #[serde(default)]
    pub force: bool,
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub literature_mode: String,
    #[serde(default)]
    pub candidate_ids: Vec<String>,
    #[serde(default)]
    pub manual_session_id: String,
    #[serde(default)]
    pub run_manifest: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileRunSummary {
    pub id: String,
    pub task_kind: String,
    pub display_name: String,
    pub status: String,
    pub current_stage: String,
    pub created_at: String,
    pub started_at: String,
    pub finished_at: String,
    pub exit_code: Option<i32>,
    pub failure_reason: String,
    pub retry_of: String,
    pub timeout_seconds: u64,
    pub current_stage_index: u64,
    pub total_stages: u64,
    pub pause_requested: bool,
    pub heartbeat: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileRunEvent {
    pub sequence: i64,
    pub event_kind: String,
    pub stage: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileArtifact {
    pub id: String,
    pub artifact_kind: String,
    pub relative_path: String,
    pub operation: String,
    pub rollback_eligible: bool,
    pub before_hash: String,
    pub after_hash: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileRunDetail {
    pub summary: CompileRunSummary,
    pub request: StartCompileRequest,
    pub events: Vec<CompileRunEvent>,
    pub artifacts: Vec<CompileArtifact>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CompileStreamEvent {
    Accepted {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    StageStarted {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Stdout {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Stderr {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Completed {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Failed {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Cancelled {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    StageCompleted {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Progress {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Paused {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    Resumed {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
    TimedOut {
        run_id: String,
        sequence: i64,
        stage: String,
        message: String,
        timestamp: String,
    },
}

#[derive(Debug)]
struct TaskSpec {
    executable: String,
    args: Vec<String>,
    label: &'static str,
    stage: &'static str,
    artifacts: Vec<(&'static str, &'static str)>,
    search_credentials: bool,
}

fn now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

pub fn db_schema(connection: &Connection) -> Result<(), String> {
    connection.execute_batch(
        "PRAGMA foreign_keys=ON;
         CREATE TABLE IF NOT EXISTS compile_runs(
           id TEXT PRIMARY KEY, repository_path TEXT NOT NULL, task_kind TEXT NOT NULL,
           display_name TEXT NOT NULL, status TEXT NOT NULL, current_stage TEXT NOT NULL DEFAULT '',
           created_at TEXT NOT NULL, started_at TEXT NOT NULL DEFAULT '', finished_at TEXT NOT NULL DEFAULT '',
           exit_code INTEGER, failure_reason TEXT NOT NULL DEFAULT '', parameters_json TEXT NOT NULL DEFAULT '{}',
           result_json TEXT NOT NULL DEFAULT '{}', retry_of TEXT NOT NULL DEFAULT '', rollback_of TEXT NOT NULL DEFAULT '',
           timeout_seconds INTEGER NOT NULL DEFAULT 3600, current_stage_index INTEGER NOT NULL DEFAULT 0,
           total_stages INTEGER NOT NULL DEFAULT 1, pause_requested INTEGER NOT NULL DEFAULT 0,
           heartbeat TEXT NOT NULL DEFAULT '');
         CREATE TABLE IF NOT EXISTS compile_run_events(
           id INTEGER PRIMARY KEY AUTOINCREMENT, run_id TEXT NOT NULL, sequence INTEGER NOT NULL,
           event_kind TEXT NOT NULL, stage TEXT NOT NULL DEFAULT '', message TEXT NOT NULL DEFAULT '',
           created_at TEXT NOT NULL, UNIQUE(run_id,sequence),
           FOREIGN KEY(run_id) REFERENCES compile_runs(id) ON DELETE CASCADE);
         CREATE TABLE IF NOT EXISTS compile_artifacts(
           id TEXT PRIMARY KEY, run_id TEXT NOT NULL, artifact_kind TEXT NOT NULL,
           relative_path TEXT NOT NULL, operation TEXT NOT NULL, before_hash TEXT NOT NULL DEFAULT '',
           after_hash TEXT NOT NULL DEFAULT '', rollback_eligible INTEGER NOT NULL DEFAULT 0,
           backup_path TEXT NOT NULL DEFAULT '',
           FOREIGN KEY(run_id) REFERENCES compile_runs(id) ON DELETE CASCADE);
         CREATE INDEX IF NOT EXISTS idx_compile_repo_created ON compile_runs(repository_path,created_at DESC);
         CREATE INDEX IF NOT EXISTS idx_compile_status ON compile_runs(status);
          CREATE INDEX IF NOT EXISTS idx_compile_event_sequence ON compile_run_events(run_id,sequence);",
    ).map_err(|error| format!("compile schema failed: {error}"))?;
    let _ = connection.execute(
        "ALTER TABLE compile_artifacts ADD COLUMN backup_path TEXT NOT NULL DEFAULT ''",
        [],
    );
    for ddl in [
        "ALTER TABLE compile_runs ADD COLUMN timeout_seconds INTEGER NOT NULL DEFAULT 3600",
        "ALTER TABLE compile_runs ADD COLUMN current_stage_index INTEGER NOT NULL DEFAULT 0",
        "ALTER TABLE compile_runs ADD COLUMN total_stages INTEGER NOT NULL DEFAULT 1",
        "ALTER TABLE compile_runs ADD COLUMN pause_requested INTEGER NOT NULL DEFAULT 0",
        "ALTER TABLE compile_runs ADD COLUMN heartbeat TEXT NOT NULL DEFAULT ''",
    ] {
        let _ = connection.execute(ddl, []);
    }
    connection
        .pragma_update(None, "user_version", COMPILE_SCHEMA_VERSION)
        .map_err(|error| format!("compile schema version failed: {error}"))?;
    Ok(())
}

pub fn recover_interrupted_runs(connection: &Connection) -> Result<usize, String> {
    connection
        .execute(
            "UPDATE compile_runs SET status='interrupted',finished_at=strftime('%s','now'),
             failure_reason='Application exited while task was running',pause_requested=0
             WHERE status IN ('queued','running','pause_requested','paused','resume_requested','cancel_requested')",
            [],
        )
        .map_err(|error| format!("compile recovery failed: {error}"))
}

fn command_exists(program: &str) -> bool {
    let mut command = Command::new(program);
    configure_background_command(&mut command);
    if matches!(program, "py" | "python" | "python3") {
        configure_python_command(&mut command);
    }
    command
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

pub fn capabilities(root: &Path) -> Vec<CompileCapability> {
    let python = command_exists("py");
    let graphify = command_exists("graphify");
    let codex = codex_subscription::available_executable().is_some();
    let tool = |name: &str| root.join("tools").join(name).is_file();
    let full_pipeline = python
        && graphify
        && codex
        && tool("full_pipeline.py")
        && tool("paper_search.py")
        && tool("wiki_lint.py")
        && tool("export_desktop_data.py");
    let definitions = [
        (
            "full_pipeline",
            "Full pipeline",
            "discover, optional parse, compile_a, lint, graphify, snapshot, verify",
            full_pipeline,
            "Requires py, codex, graphify and pipeline tools",
            true,
            true,
            false,
        ),
        (
            "lint",
            "Knowledge lint",
            "Read-only health check with a report",
            python && tool("wiki_lint.py"),
            "Requires py and tools/wiki_lint.py",
            true,
            false,
            false,
        ),
        (
            "graphify_update",
            "Update Graphify",
            "Rebuild the derived knowledge graph",
            graphify,
            "Requires graphify CLI",
            true,
            false,
            false,
        ),
        (
            "discover",
            "Discover papers",
            "Search candidates into raw/inbox",
            python && tool("paper_search.py"),
            "Requires py and paper_search.py",
            true,
            true,
            false,
        ),
        (
            "parse",
            "Parse PDF",
            "Convert a repository PDF with MinerU",
            python && tool("mineru_to_md.py"),
            "Requires py and mineru_to_md.py",
            true,
            true,
            true,
        ),
        (
            "literature_prepare",
            "Prepare literature candidates",
            "Discover, deduplicate and download candidates for review",
            python && tool("literature_ingest.py"),
            "Requires py and literature_ingest.py",
            true,
            true,
            false,
        ),
        (
            "literature_manual_ingest",
            "Ingest manual PDFs",
            "Run a trusted manual PDF batch through the governed pipeline",
            full_pipeline && tool("literature_ingest.py") && tool("mineru_to_md.py"),
            "Requires py, codex, graphify, MinerU and literature_ingest.py",
            true,
            true,
            false,
        ),
        (
            "literature_candidate_download",
            "Download selected candidates",
            "Download open PDFs without promoting them to the Wiki",
            python && tool("literature_ingest.py"),
            "Requires py and literature_ingest.py",
            true,
            true,
            false,
        ),
        (
            "literature_candidate_ingest",
            "Ingest confirmed candidates",
            "Download and compile explicitly confirmed candidates",
            full_pipeline && tool("literature_ingest.py") && tool("mineru_to_md.py"),
            "Requires py, codex, graphify, MinerU and literature_ingest.py",
            true,
            true,
            false,
        ),
        (
            "literature_auto_ingest",
            "Automatic governed ingest",
            "Prepare candidates and automatically ingest only qualified items",
            full_pipeline && tool("literature_ingest.py") && tool("mineru_to_md.py"),
            "Requires py, codex, graphify, MinerU and literature_ingest.py",
            true,
            true,
            false,
        ),
        (
            "compile_a",
            "Compile A pages",
            "Run the fixed Agent A protocol with Codex",
            codex,
            "Requires codex CLI",
            true,
            false,
            false,
        ),
    ];
    definitions
        .into_iter()
        .map(
            |(kind, label, description, available, missing, writes, network, requires_input)| {
                CompileCapability {
                    task_kind: kind.into(),
                    label: label.into(),
                    description: description.into(),
                    available,
                    reason: if available {
                        "Available".into()
                    } else {
                        missing.into()
                    },
                    writes,
                    network,
                    requires_input,
                }
            },
        )
        .collect()
}

fn summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<CompileRunSummary> {
    Ok(CompileRunSummary {
        id: row.get(0)?,
        task_kind: row.get(1)?,
        display_name: row.get(2)?,
        status: row.get(3)?,
        current_stage: row.get(4)?,
        created_at: row.get(5)?,
        started_at: row.get(6)?,
        finished_at: row.get(7)?,
        exit_code: row.get(8)?,
        failure_reason: row.get(9)?,
        retry_of: row.get(10)?,
        timeout_seconds: row.get::<_, i64>(11).unwrap_or(3600) as u64,
        current_stage_index: row.get::<_, i64>(12).unwrap_or(0) as u64,
        total_stages: row.get::<_, i64>(13).unwrap_or(1) as u64,
        pause_requested: row.get::<_, i64>(14).unwrap_or(0) != 0,
        heartbeat: row.get(15).unwrap_or_default(),
    })
}

pub fn list_runs(
    connection: &Connection,
    repository: &str,
    limit: usize,
) -> Result<Vec<CompileRunSummary>, String> {
    let mut statement = connection.prepare("SELECT id,task_kind,display_name,status,current_stage,created_at,started_at,finished_at,exit_code,failure_reason,retry_of,timeout_seconds,current_stage_index,total_stages,pause_requested,heartbeat FROM compile_runs WHERE repository_path=?1 ORDER BY rowid DESC LIMIT ?2").map_err(|e| e.to_string())?;
    let rows = statement
        .query_map(params![repository, limit.min(500) as i64], summary)
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(rows)
}

pub fn get_run(
    connection: &Connection,
    repository: &str,
    run_id: &str,
) -> Result<CompileRunDetail, String> {
    let (summary, json): (CompileRunSummary, String) = connection.query_row(
        "SELECT id,task_kind,display_name,status,current_stage,created_at,started_at,finished_at,exit_code,failure_reason,retry_of,timeout_seconds,current_stage_index,total_stages,pause_requested,heartbeat,parameters_json FROM compile_runs WHERE repository_path=?1 AND id=?2",
        params![repository, run_id], |row| Ok((summary(row)?, row.get(16)?)),
    ).optional().map_err(|e| e.to_string())?.ok_or_else(|| "compile run not found for repository".to_string())?;
    let request = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    let mut event_query = connection.prepare("SELECT sequence,event_kind,stage,message,created_at FROM compile_run_events WHERE run_id=?1 ORDER BY sequence").map_err(|e| e.to_string())?;
    let events = event_query
        .query_map([run_id], |row| {
            Ok(CompileRunEvent {
                sequence: row.get(0)?,
                event_kind: row.get(1)?,
                stage: row.get(2)?,
                message: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let mut artifact_query = connection.prepare("SELECT id,artifact_kind,relative_path,operation,rollback_eligible,before_hash,after_hash FROM compile_artifacts WHERE run_id=?1 ORDER BY rowid").map_err(|e| e.to_string())?;
    let artifacts = artifact_query
        .query_map([run_id], |row| {
            Ok(CompileArtifact {
                id: row.get(0)?,
                artifact_kind: row.get(1)?,
                relative_path: row.get(2)?,
                operation: row.get(3)?,
                rollback_eligible: row.get::<_, i64>(4)? != 0,
                before_hash: row.get(5)?,
                after_hash: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    Ok(CompileRunDetail {
        summary,
        request,
        events,
        artifacts,
    })
}

fn inside_repository(root: &Path, value: &str) -> Result<PathBuf, String> {
    let root = fs::canonicalize(root).map_err(|e| format!("repository path invalid: {e}"))?;
    let value = PathBuf::from(value);
    let candidate = if value.is_absolute() {
        value
    } else {
        root.join(value)
    };
    let candidate = fs::canonicalize(candidate).map_err(|e| format!("input path invalid: {e}"))?;
    if !candidate.starts_with(&root) {
        return Err("input path is outside repository".into());
    }
    Ok(candidate)
}

fn build_task(
    root: &Path,
    request: &StartCompileRequest,
    run_id: Option<&str>,
) -> Result<TaskSpec, String> {
    match request.task_kind.as_str() {
        "lint" => Ok(TaskSpec { executable: "py".into(), args: vec!["-3".into(), "tools/wiki_lint.py".into(), "--write-report".into()], label: "Knowledge lint", stage: "lint", artifacts: vec![("report", "logs")], search_credentials: false }),
        "graphify_update" => Ok(TaskSpec { executable: "graphify".into(), args: vec!["update".into(), ".".into(), "--force".into()], label: "Update Graphify", stage: "graphify", artifacts: vec![("graph", "graphify-out")], search_credentials: false }),
        "discover" => {
            let mut args = vec!["-3".into(), "tools/paper_search.py".into(), "--preset".into(), "wireless-charging-scheduling".into(), "--new-only".into()];
            if request.dry_run { args.push("--dry-run".into()); }
            if request.download { args.push("--download".into()); }
            Ok(TaskSpec { executable: "py".into(), args, label: "Discover papers", stage: "discover", artifacts: vec![("discovery", "raw/inbox/auto-discovered")], search_credentials: true })
        }
        "parse" => {
            let input = request.input_path.as_deref().filter(|v| !v.trim().is_empty()).ok_or_else(|| "parse requires inputPath".to_string())?;
            let input = inside_repository(root, input)?;
            let mut args = vec!["-3".into(), "tools/mineru_to_md.py".into(), input.to_string_lossy().into_owned(), "--output-root".into(), root.join("raw/canonical").to_string_lossy().into_owned()];
            if request.dry_run { args.push("--dry-run".into()); }
            if request.force { args.push("--force".into()); }
            Ok(TaskSpec { executable: "py".into(), args, label: "Parse PDF", stage: "parse", artifacts: vec![("canonical", "raw/canonical")], search_credentials: false })
        }
        "compile_a" => Ok(TaskSpec { executable: codex_subscription::available_executable().unwrap_or_else(|| "codex".into()), args: vec!["-a".into(), "never".into(), "-s".into(), "workspace-write".into(), "exec".into(), "-C".into(), root.to_string_lossy().into_owned(), "--skip-git-repo-check".into(), "--ephemeral".into(), "Read AGENTS.md and schema/agent-a-compile.md. Compile every pending_ingest through the Agent A protocol. Never write wiki/problems or wiki/ideas, never edit vocab.yaml, never delete files, and update index, library-status, logs and Graphify.".into()], label: "Compile A pages", stage: "compile_a", artifacts: vec![("wiki", "wiki")], search_credentials: false }),
        "full_pipeline" => {
            let mut args = vec!["-3".into(), "tools/full_pipeline.py".into()];
            if let Some(run_id) = run_id {
                args.extend([
                    "--control-file".into(),
                    format!(".codegraph/compile-control/{run_id}.pause"),
                ]);
            }
            if let Some(input) = request.input_path.as_deref().filter(|value| !value.trim().is_empty()) {
                inside_repository(root, input)?;
                args.extend(["--input-path".into(), input.into()]);
            }
            if request.download { args.push("--download".into()); }
            if request.force { args.push("--force".into()); }
            Ok(TaskSpec { executable: "py".into(), args, label: "Full knowledge pipeline", stage: "pipeline", artifacts: vec![("discovery", "raw/inbox/auto-discovered"), ("canonical", "raw/canonical"), ("wiki", "wiki"), ("logs", "logs"), ("graph", "graphify-out"), ("snapshot", "apps/desktop/public/data/library.json")], search_credentials: true })
        }
        "literature_prepare" | "literature_manual_ingest" | "literature_candidate_download" | "literature_candidate_ingest" | "literature_auto_ingest" => {
            let manifest = request.run_manifest.as_deref().ok_or_else(|| "literature task requires a trusted run manifest".to_string())?;
            let manifest = fs::canonicalize(manifest).map_err(|error| format!("literature run manifest invalid: {error}"))?;
            if manifest.extension().and_then(|value| value.to_str()) != Some("json") {
                return Err("literature run manifest must be JSON".into());
            }
            let payload: serde_json::Value = serde_json::from_slice(&fs::read(&manifest).map_err(|error| error.to_string())?).map_err(|error| error.to_string())?;
            if payload.get("kind").and_then(|value| value.as_str()) != Some("literature_ingest_run") {
                return Err("literature run manifest kind is invalid".into());
            }
            let label = match request.task_kind.as_str() {
                "literature_prepare" => "Prepare literature candidates",
                "literature_manual_ingest" => "Ingest manual PDFs",
                "literature_candidate_download" => "Download literature candidates",
                "literature_candidate_ingest" => "Ingest confirmed candidates",
                _ => "Automatic governed literature ingest",
            };
            Ok(TaskSpec {
                executable: "py".into(),
                args: vec![
                    "-3".into(),
                    "tools/literature_ingest.py".into(),
                    "--repository".into(),
                    root.to_string_lossy().into_owned(),
                    "run".into(),
                    "--manifest".into(),
                    manifest.to_string_lossy().into_owned(),
                ],
                label,
                stage: "literature",
                artifacts: vec![("discovery", "raw/inbox/auto-discovered"), ("manual", "raw/inbox/manual-drop"), ("canonical", "raw/canonical"), ("wiki", "wiki"), ("logs", "logs"), ("graph", "graphify-out"), ("snapshot", "apps/desktop/public/data/library.json")],
                search_credentials: matches!(request.task_kind.as_str(), "literature_prepare" | "literature_auto_ingest"),
            })
        }
        _ => Err("task kind is not in the compile allowlist".into()),
    }
}

/// Return ordered stages for a requested pipeline. `parse` is included only when inputPath resolves.
pub fn build_pipeline(root: &Path, request: &StartCompileRequest) -> Result<Vec<String>, String> {
    if request.task_kind.starts_with("literature_") {
        return Ok(match request.task_kind.as_str() {
            "literature_prepare" => vec!["discover".into()],
            "literature_candidate_download" => vec!["download".into()],
            "literature_manual_ingest" => vec![
                "stage".into(),
                "parse".into(),
                "compile_a".into(),
                "lint".into(),
                "graphify_update".into(),
                "rebuild_snapshot".into(),
            ],
            "literature_candidate_ingest" => vec![
                "download".into(),
                "parse".into(),
                "compile_a".into(),
                "lint".into(),
                "graphify_update".into(),
                "rebuild_snapshot".into(),
            ],
            "literature_auto_ingest" => vec![
                "discover".into(),
                "qualify".into(),
                "download".into(),
                "parse".into(),
                "compile_a".into(),
                "lint".into(),
                "graphify_update".into(),
                "rebuild_snapshot".into(),
            ],
            _ => return Err("unknown literature task kind".into()),
        });
    }
    if request.task_kind != "full_pipeline" {
        return Ok(vec![request.task_kind.clone()]);
    }
    let mut stages = vec!["discover".to_string()];
    if request
        .input_path
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
    {
        let _ = inside_repository(root, request.input_path.as_deref().unwrap())?;
        stages.push("parse".into());
    }
    stages.extend([
        "compile_a".into(),
        "lint".into(),
        "graphify_update".into(),
        "rebuild_snapshot".into(),
        "verify".into(),
    ]);
    Ok(stages)
}

fn redact(input: &str) -> String {
    let mut output = input.to_string();
    for marker in [
        "LUNA_API_KEY=",
        "MINERU_API_KEY=",
        "OPENALEX_API_KEY=",
        "TAVILY_API_KEY=",
        "SERPAPI_API_KEY=",
        "Authorization: Bearer ",
        "Bearer ",
    ] {
        if let Some(start) = output.find(marker) {
            let from = start + marker.len();
            let to = output[from..]
                .find(|c: char| c.is_whitespace() || c == '&')
                .map(|n| from + n)
                .unwrap_or(output.len());
            if from != to {
                output.replace_range(from..to, "[REDACTED]");
            }
        }
    }
    for key in ["token=", "signature=", "sig="] {
        if let Some(start) = output.to_ascii_lowercase().find(key) {
            let from = start + key.len();
            let to = output[from..]
                .find(|c: char| c == '&' || c.is_whitespace())
                .map(|n| from + n)
                .unwrap_or(output.len());
            if from != to {
                output.replace_range(from..to, "[REDACTED]");
            }
        }
    }
    if output.len() > 16_384 {
        output.truncate(16_384);
        output.push_str("...[truncated]");
    }
    output
}

fn emit_event(
    connection: &Connection,
    channel: &Channel<CompileStreamEvent>,
    run_id: &str,
    sequence: &mut i64,
    kind: &str,
    stage: &str,
    message: &str,
) {
    *sequence += 1;
    let timestamp = now();
    let message = redact(message);
    let _ = connection.execute("INSERT OR IGNORE INTO compile_run_events(run_id,sequence,event_kind,stage,message,created_at) VALUES(?1,?2,?3,?4,?5,?6)", params![run_id,*sequence,kind,stage,message,timestamp]);
    let fields = || {
        (
            run_id.to_string(),
            *sequence,
            stage.to_string(),
            message.clone(),
            timestamp.clone(),
        )
    };
    let event = match kind {
        "accepted" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Accepted {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "stage_started" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::StageStarted {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "stdout" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Stdout {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "stderr" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Stderr {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "completed" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Completed {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "cancelled" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Cancelled {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "stage_completed" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::StageCompleted {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "progress" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Progress {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "paused" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Paused {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "resumed" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Resumed {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        "timed_out" => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::TimedOut {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
        _ => {
            let (run_id, sequence, stage, message, timestamp) = fields();
            CompileStreamEvent::Failed {
                run_id,
                sequence,
                stage,
                message,
                timestamp,
            }
        }
    };
    let _ = channel.send(event);
}

#[allow(clippy::too_many_arguments)]
fn emit_process_line(
    connection: &Connection,
    channel: &Channel<CompileStreamEvent>,
    run_id: &str,
    sequence: &mut i64,
    kind: &str,
    fallback_stage: &str,
    line: &str,
    stage_index: &mut i64,
    total_stages: i64,
) {
    if kind == "stdout" {
        if let Some(stage) = line.strip_prefix("PIPELINE_STAGE_START ") {
            *stage_index += 1;
            let timestamp = now();
            let _ = connection.execute(
                "UPDATE compile_runs SET current_stage=?2,current_stage_index=?3,total_stages=?4,heartbeat=?5 WHERE id=?1",
                params![run_id,stage,*stage_index,total_stages,timestamp],
            );
            emit_event(
                connection,
                channel,
                run_id,
                sequence,
                "stage_started",
                stage,
                &format!("Stage {}/{} started", stage_index, total_stages),
            );
            emit_event(
                connection,
                channel,
                run_id,
                sequence,
                "progress",
                stage,
                &format!("{}/{}", stage_index, total_stages),
            );
            return;
        }
        if let Some(stage) = line.strip_prefix("PIPELINE_STAGE_COMPLETED ") {
            emit_event(
                connection,
                channel,
                run_id,
                sequence,
                "stage_completed",
                stage,
                "Stage completed",
            );
            return;
        }
        if let Some(stage) = line.strip_prefix("PIPELINE_PAUSED ") {
            let _ = connection.execute(
                "UPDATE compile_runs SET status='paused',current_stage=?2,pause_requested=1,heartbeat=?3 WHERE id=?1",
                params![run_id, stage, now()],
            );
            emit_event(
                connection,
                channel,
                run_id,
                sequence,
                "paused",
                stage,
                "Paused at a safe stage boundary",
            );
            return;
        }
        if let Some(stage) = line.strip_prefix("PIPELINE_RESUMED ") {
            let _ = connection.execute(
                "UPDATE compile_runs SET status='running',current_stage=?2,pause_requested=0,heartbeat=?3 WHERE id=?1",
                params![run_id, stage, now()],
            );
            emit_event(
                connection,
                channel,
                run_id,
                sequence,
                "resumed",
                stage,
                "Pipeline resumed",
            );
            return;
        }
        if let Some(rest) = line.strip_prefix("PIPELINE_STAGE_FAILED ") {
            let stage = rest.split_whitespace().next().unwrap_or(fallback_stage);
            emit_event(connection, channel, run_id, sequence, "failed", stage, line);
            return;
        }
    }
    emit_event(
        connection,
        channel,
        run_id,
        sequence,
        kind,
        fallback_stage,
        line,
    );
}

fn pause_control_path(root: &Path, run_id: &str) -> PathBuf {
    root.join(".codegraph")
        .join("compile-control")
        .join(format!("{run_id}.pause"))
}

fn task_timeout_seconds(request: &StartCompileRequest) -> u64 {
    request
        .timeout_seconds
        .unwrap_or(match request.task_kind.as_str() {
            "lint" => 600,
            "graphify_update" | "discover" => 1800,
            "parse" | "compile_a" | "full_pipeline" => 3600,
            _ => 1800,
        })
        .clamp(10, 86_400)
}

pub fn set_pause_requested(
    connection: &Connection,
    root: &Path,
    run_id: &str,
    pause: bool,
) -> Result<(), String> {
    let (task_kind, status): (String, String) = connection
        .query_row(
            "SELECT task_kind,status FROM compile_runs WHERE repository_path=?1 AND id=?2",
            params![root.to_string_lossy(), run_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|error| format!("读取任务状态失败：{error}"))?;
    if task_kind != "full_pipeline" {
        return Err("仅完整流水线支持阶段边界暂停".to_string());
    }
    let control = pause_control_path(root, run_id);
    if pause {
        if status != "running" {
            return Err(format!("任务当前状态不支持暂停：{status}"));
        }
        if let Some(parent) = control.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::write(&control, "pause\n").map_err(|error| error.to_string())?;
        connection
            .execute(
                "UPDATE compile_runs SET status='pause_requested',pause_requested=1,heartbeat=?2 WHERE id=?1",
                params![run_id, now()],
            )
            .map_err(|error| error.to_string())?;
    } else {
        if !matches!(status.as_str(), "pause_requested" | "paused") {
            return Err(format!("任务当前状态不支持继续：{status}"));
        }
        if control.exists() {
            fs::remove_file(&control).map_err(|error| error.to_string())?;
        }
        connection
            .execute(
                "UPDATE compile_runs SET status='resume_requested',pause_requested=0,heartbeat=?2 WHERE id=?1",
                params![run_id, now()],
            )
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn execute_run(
    db_path: &Path,
    root: &Path,
    run_id: String,
    request: StartCompileRequest,
    channel: Channel<CompileStreamEvent>,
    cancellation: Arc<AtomicBool>,
    retry_of: Option<String>,
) -> Result<CompileRunSummary, String> {
    let spec = build_task(root, &request, Some(&run_id))?;
    let stage_plan = build_pipeline(root, &request)?;
    let total_stages = stage_plan.len().max(1) as i64;
    let write = request.task_kind != "lint";
    let _write_guard = RepositoryWriteGuard::acquire(root, write)?;
    let backup_root = db_path
        .parent()
        .unwrap_or(root)
        .join("compile-backups")
        .join(&run_id);
    let artifact_scopes = spec
        .artifacts
        .iter()
        .map(|(kind, relative)| {
            let path = root.join(relative);
            let before = snapshot_scope(root, &path, Some(&backup_root))?;
            Ok((*kind, path, before))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let connection = Connection::open(db_path).map_err(|e| e.to_string())?;
    db_schema(&connection)?;
    let created = now();
    connection.execute("INSERT INTO compile_runs(id,repository_path,task_kind,display_name,status,current_stage,created_at,parameters_json,retry_of,timeout_seconds,total_stages) VALUES(?1,?2,?3,?4,'queued',?5,?6,?7,?8,?9,?10)", params![run_id,root.to_string_lossy(),request.task_kind,spec.label,spec.stage,created,serde_json::to_string(&request).map_err(|e| e.to_string())?,retry_of.unwrap_or_default(),task_timeout_seconds(&request) as i64,total_stages]).map_err(|e| e.to_string())?;
    let mut sequence = 0;
    emit_event(
        &connection,
        &channel,
        &run_id,
        &mut sequence,
        "accepted",
        spec.stage,
        "Task accepted",
    );
    let started = now();
    connection
        .execute(
            "UPDATE compile_runs SET status='running',started_at=?2 WHERE id=?1",
            params![run_id, started],
        )
        .map_err(|e| e.to_string())?;
    emit_event(
        &connection,
        &channel,
        &run_id,
        &mut sequence,
        "stage_started",
        spec.stage,
        spec.label,
    );
    if request.dry_run {
        let stages =
            build_pipeline(root, &request).unwrap_or_else(|_| vec![request.task_kind.clone()]);
        connection.execute("UPDATE compile_runs SET status='succeeded',finished_at=?2,total_stages=?3,current_stage_index=?3,heartbeat=?2 WHERE id=?1", params![run_id, now(), stages.len() as i64]).ok();
        emit_event(
            &connection,
            &channel,
            &run_id,
            &mut sequence,
            "progress",
            "pipeline",
            &format!("dry-run plan: {}", stages.join(",")),
        );
        emit_event(
            &connection,
            &channel,
            &run_id,
            &mut sequence,
            "completed",
            "pipeline",
            "Dry-run plan completed",
        );
        return get_run(&connection, &root.to_string_lossy(), &run_id).map(|d| d.summary);
    }
    let mut command = Command::new(&spec.executable);
    configure_background_command(&mut command);
    if let Some(codex) = codex_subscription::available_executable() {
        command.env("CODEX_CLI_PATH", codex);
    }
    if matches!(spec.executable.as_str(), "py" | "python" | "python3") {
        configure_python_command(&mut command);
    }
    if spec.search_credentials {
        if let Err(error) = search_credentials::apply_to_command(&mut command) {
            connection.execute("UPDATE compile_runs SET status='failed',finished_at=?2,failure_reason=?3 WHERE id=?1",params![run_id,now(),error]).ok();
            emit_event(
                &connection,
                &channel,
                &run_id,
                &mut sequence,
                "failed",
                spec.stage,
                &error,
            );
            return get_run(&connection, &root.to_string_lossy(), &run_id)
                .map(|detail| detail.summary);
        }
    }
    let mut child = match command
        .args(&spec.args)
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            let reason = format!("failed to start {}: {error}", spec.executable);
            connection.execute("UPDATE compile_runs SET status='failed',finished_at=?2,failure_reason=?3 WHERE id=?1",params![run_id,now(),reason]).ok();
            emit_event(
                &connection,
                &channel,
                &run_id,
                &mut sequence,
                "failed",
                spec.stage,
                &reason,
            );
            return get_run(&connection, &root.to_string_lossy(), &run_id).map(|d| d.summary);
        }
    };
    let (sender, receiver) = mpsc::channel::<(&'static str, String)>();
    if let Some(stdout) = child.stdout.take() {
        let tx = sender.clone();
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines().map_while(Result::ok) {
                let _ = tx.send(("stdout", line));
            }
        });
    }
    if let Some(stderr) = child.stderr.take() {
        let tx = sender.clone();
        thread::spawn(move || {
            for line in BufReader::new(stderr).lines().map_while(Result::ok) {
                let _ = tx.send(("stderr", line));
            }
        });
    }
    drop(sender);
    let timeout = Duration::from_secs(task_timeout_seconds(&request));
    let process_started = Instant::now();
    let mut current_stage_index = if request.task_kind == "full_pipeline" {
        0
    } else {
        1
    };
    let mut literature_result: Option<serde_json::Value> = None;
    let (final_status, exit_code, reason) = loop {
        while let Ok((kind, line)) = receiver.try_recv() {
            if kind == "stdout" {
                if let Some(payload) = line.strip_prefix("LITERATURE_RESULT ") {
                    literature_result = serde_json::from_str(payload).ok();
                }
            }
            emit_process_line(
                &connection,
                &channel,
                &run_id,
                &mut sequence,
                kind,
                spec.stage,
                &line,
                &mut current_stage_index,
                total_stages,
            );
        }
        if cancellation.load(Ordering::SeqCst) {
            #[cfg(target_os = "windows")]
            let mut command = Command::new("taskkill");
            configure_background_command(&mut command);
            let _ = command
                .args(["/PID", &child.id().to_string(), "/T", "/F"])
                .status();
            #[cfg(not(target_os = "windows"))]
            let _ = child.kill();
            let _ = child.wait();
            break ("cancelled", None, "Task cancelled".to_string());
        }
        if process_started.elapsed() >= timeout {
            #[cfg(target_os = "windows")]
            let mut command = Command::new("taskkill");
            configure_background_command(&mut command);
            let _ = command
                .args(["/PID", &child.id().to_string(), "/T", "/F"])
                .status();
            #[cfg(not(target_os = "windows"))]
            let _ = child.kill();
            let _ = child.wait();
            break (
                "timed_out",
                None,
                format!("Task exceeded {} seconds", timeout.as_secs()),
            );
        }
        match child.try_wait() {
            Ok(Some(status)) => {
                while let Ok((kind, line)) = receiver.recv_timeout(Duration::from_millis(20)) {
                    if kind == "stdout" {
                        if let Some(payload) = line.strip_prefix("LITERATURE_RESULT ") {
                            literature_result = serde_json::from_str(payload).ok();
                        }
                    }
                    emit_process_line(
                        &connection,
                        &channel,
                        &run_id,
                        &mut sequence,
                        kind,
                        spec.stage,
                        &line,
                        &mut current_stage_index,
                        total_stages,
                    );
                }
                if status.success() {
                    break ("succeeded", status.code(), String::new());
                }
                if request.task_kind.starts_with("literature_") && status.code() == Some(3) {
                    break (
                        "failed_partial",
                        status.code(),
                        "some literature items failed; successful items were retained".to_string(),
                    );
                }
                break (
                    "failed",
                    status.code(),
                    format!("command exited with {:?}", status.code()),
                );
            }
            Ok(None) => thread::sleep(Duration::from_millis(60)),
            Err(error) => break ("failed", None, format!("process status failed: {error}")),
        }
    };
    connection.execute("UPDATE compile_runs SET status=?2,finished_at=?3,exit_code=?4,failure_reason=?5,result_json=?6 WHERE id=?1",params![run_id,final_status,now(),exit_code,reason,serde_json::json!({"eventCount":sequence,"literature":literature_result}).to_string()]).map_err(|e|e.to_string())?;
    if matches!(final_status, "succeeded" | "failed_partial") {
        for (kind, path, before) in &artifact_scopes {
            let after = snapshot_scope(root, path, None)?;
            let mut paths = before
                .keys()
                .chain(after.keys())
                .cloned()
                .collect::<Vec<_>>();
            paths.sort();
            paths.dedup();
            for relative_path in paths {
                let previous = before.get(&relative_path);
                let current = after.get(&relative_path);
                let (operation, before_hash, after_hash, backup_path, eligible) =
                    match (previous, current) {
                        (None, Some(current)) => ("created", "", current.hash.as_str(), "", true),
                        (Some(previous), None) => (
                            "deleted",
                            previous.hash.as_str(),
                            "",
                            previous.backup_path.as_str(),
                            !previous.backup_path.is_empty(),
                        ),
                        (Some(previous), Some(current)) if previous.hash != current.hash => (
                            "modified",
                            previous.hash.as_str(),
                            current.hash.as_str(),
                            previous.backup_path.as_str(),
                            !previous.backup_path.is_empty(),
                        ),
                        _ => continue,
                    };
                connection.execute(
                    "INSERT INTO compile_artifacts(id,run_id,artifact_kind,relative_path,operation,before_hash,after_hash,rollback_eligible,backup_path) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9)",
                    params![Uuid::new_v4().to_string(),run_id,*kind,relative_path,operation,before_hash,after_hash,if eligible {1} else {0},backup_path],
                ).map_err(|error| format!("record artifact failed: {error}"))?;
            }
        }
        emit_event(
            &connection,
            &channel,
            &run_id,
            &mut sequence,
            if final_status == "succeeded" {
                "completed"
            } else {
                "failed"
            },
            spec.stage,
            if final_status == "succeeded" {
                "Task completed"
            } else {
                "Task completed with item failures; successful items were retained"
            },
        );
    } else if final_status == "cancelled" {
        emit_event(
            &connection,
            &channel,
            &run_id,
            &mut sequence,
            "cancelled",
            spec.stage,
            &reason,
        );
    } else if final_status == "timed_out" {
        emit_event(
            &connection,
            &channel,
            &run_id,
            &mut sequence,
            "timed_out",
            spec.stage,
            &reason,
        );
    } else {
        emit_event(
            &connection,
            &channel,
            &run_id,
            &mut sequence,
            "failed",
            spec.stage,
            &reason,
        );
    }
    let control = pause_control_path(root, &run_id);
    if control.exists() {
        let _ = fs::remove_file(control);
    }
    get_run(&connection, &root.to_string_lossy(), &run_id).map(|d| d.summary)
}

#[derive(Clone, Debug)]
struct RollbackArtifact {
    relative: String,
    operation: String,
    before_hash: String,
    after_hash: String,
    backup_path: String,
}

#[derive(Clone, Debug)]
struct RollbackJournalEntry {
    artifact: RollbackArtifact,
    target: PathBuf,
    quarantine: PathBuf,
    staged_backup: Option<PathBuf>,
}

fn rollback_target(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let normalized = relative.replace('\\', "/");
    let path = Path::new(relative);
    let windows_prefix = normalized.len() >= 2
        && normalized.as_bytes()[1] == b':'
        && normalized.as_bytes()[0].is_ascii_alphabetic();
    if normalized.is_empty()
        || normalized.starts_with('/')
        || normalized.starts_with("//")
        || windows_prefix
        || path.is_absolute()
        || normalized.split('/').any(|part| part == "..")
    {
        return Err(format!(
            "rollback artifact path is outside repository: {relative}"
        ));
    }
    let canonical_root = fs::canonicalize(root)
        .map_err(|error| format!("repository path invalid for rollback: {error}"))?;
    let target = root.join(path);
    let mut existing_parent = target.clone();
    while !existing_parent.exists() {
        existing_parent = existing_parent
            .parent()
            .ok_or_else(|| format!("rollback artifact has no repository parent: {relative}"))?
            .to_path_buf();
    }
    let canonical_parent = fs::canonicalize(&existing_parent)
        .map_err(|error| format!("rollback artifact parent invalid for {relative}: {error}"))?;
    if !canonical_parent.starts_with(&canonical_root) {
        return Err(format!(
            "rollback artifact is outside repository: {relative}"
        ));
    }
    if target.exists() {
        let canonical_target = fs::canonicalize(&target)
            .map_err(|error| format!("rollback artifact target invalid for {relative}: {error}"))?;
        if !canonical_target.starts_with(&canonical_root) {
            return Err(format!(
                "rollback artifact is outside repository: {relative}"
            ));
        }
    }
    Ok(target)
}

fn rollback_failure(
    connection: &Connection,
    rollback_id: &str,
    source_run_id: &str,
    reason: &str,
    status: &str,
    journal: &[RollbackJournalEntry],
) -> Result<String, String> {
    let applied = journal
        .iter()
        .map(|entry| entry.artifact.relative.clone())
        .collect::<Vec<_>>();
    let result = serde_json::json!({
        "error": reason,
        "compensated": status == "failed",
        "status": status,
        "restoredArtifacts": applied,
    });
    connection
        .execute(
            "UPDATE compile_runs SET status=?2,finished_at=?3,failure_reason=?4,result_json=?5 WHERE id=?1",
            params![rollback_id, status, now(), reason, result.to_string()],
        )
        .map_err(|error| format!("record rollback failure failed: {error}"))?;
    connection
        .execute(
            "INSERT INTO compile_run_events(run_id,sequence,event_kind,stage,message,created_at) VALUES(?1,1,'failed','rollback',?2,?3)",
            params![rollback_id, format!("Rollback of {source_run_id} failed ({status}): {reason}"), now()],
        )
        .map_err(|error| format!("record rollback failure event failed: {error}"))?;
    Err(format!("rollback failed ({status}): {reason}"))
}

fn compensate_rollback(journal: &[RollbackJournalEntry]) -> Result<(), String> {
    let mut errors = Vec::new();
    for entry in journal.iter().rev() {
        let result = match entry.artifact.operation.as_str() {
            "created" | "modified" => {
                if entry.quarantine.exists() {
                    if entry.target.exists() {
                        fs::remove_file(&entry.target)
                    } else {
                        Ok(())
                    }
                    .and_then(|_| {
                        if let Some(parent) = entry.target.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::rename(&entry.quarantine, &entry.target)
                    })
                } else {
                    Ok(())
                }
            }
            "deleted" => {
                if entry.target.exists() {
                    fs::remove_file(&entry.target)
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        };
        if let Err(error) = result {
            errors.push(format!("{}: {error}", entry.artifact.relative));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

pub fn rollback_run(
    connection: &mut Connection,
    root: &Path,
    run_id: &str,
) -> Result<String, String> {
    rollback_run_inner(connection, root, run_id, None)
}

fn rollback_run_inner(
    connection: &mut Connection,
    root: &Path,
    run_id: &str,
    fail_after: Option<usize>,
) -> Result<String, String> {
    let _write_guard = RepositoryWriteGuard::acquire(root, true)?;
    let detail = get_run(connection, &root.to_string_lossy(), run_id)?;
    if detail.summary.status != "succeeded" {
        return Err("only succeeded tasks can be rolled back".into());
    }
    let mut statement = connection
        .prepare(
            "SELECT relative_path,operation,before_hash,after_hash,backup_path
         FROM compile_artifacts WHERE run_id=?1 AND rollback_eligible=1 ORDER BY rowid DESC",
        )
        .map_err(|error| error.to_string())?;
    let artifacts = statement
        .query_map([run_id], |row| {
            Ok(RollbackArtifact {
                relative: row.get(0)?,
                operation: row.get(1)?,
                before_hash: row.get(2)?,
                after_hash: row.get(3)?,
                backup_path: row.get(4)?,
            })
        })
        .map_err(|error| error.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    drop(statement);
    if artifacts.is_empty() {
        return Err("task has no rollback material".into());
    }
    let mut targets = Vec::with_capacity(artifacts.len());
    for artifact in &artifacts {
        let path = rollback_target(root, &artifact.relative)?;
        match artifact.operation.as_str() {
            "created" | "modified" => {
                if !path.is_file()
                    || (!artifact.after_hash.is_empty() && file_hash(&path) != artifact.after_hash)
                {
                    return Err(format!("hash conflict for {}", artifact.relative));
                }
            }
            "deleted" if path.exists() => {
                return Err(format!("hash conflict for {}", artifact.relative))
            }
            "deleted" => {}
            _ => {
                return Err(format!(
                    "unsupported rollback operation: {}",
                    artifact.operation
                ))
            }
        }
        if matches!(artifact.operation.as_str(), "modified" | "deleted") {
            let backup = PathBuf::from(&artifact.backup_path);
            if !backup.is_file() || file_hash(&backup) != artifact.before_hash {
                return Err(format!("rollback backup invalid for {}", artifact.relative));
            }
        }
        targets.push(path);
    }
    let rollback_id = Uuid::new_v4().to_string();
    let timestamp = now();
    let staging_root = root
        .join("compile-backups")
        .join(format!("rollback-{rollback_id}"));
    fs::create_dir_all(&staging_root)
        .map_err(|error| format!("create rollback staging failed: {error}"))?;
    let mut journal = Vec::with_capacity(artifacts.len());
    for (index, (artifact, target)) in artifacts.iter().zip(targets.iter()).enumerate() {
        let staged_backup = if matches!(artifact.operation.as_str(), "modified" | "deleted") {
            let staged = staging_root.join(format!("{index}.backup"));
            fs::copy(&artifact.backup_path, &staged).map_err(|error| {
                format!(
                    "stage rollback backup failed for {}: {error}",
                    artifact.relative
                )
            })?;
            if file_hash(&staged) != artifact.before_hash {
                let _ = fs::remove_dir_all(&staging_root);
                return Err(format!(
                    "staged rollback backup hash mismatch for {}",
                    artifact.relative
                ));
            }
            Some(staged)
        } else {
            None
        };
        journal.push(RollbackJournalEntry {
            artifact: artifact.clone(),
            target: target.clone(),
            quarantine: staging_root.join(format!("{index}.current")),
            staged_backup,
        });
    }
    connection.execute(
        "INSERT INTO compile_runs(id,repository_path,task_kind,display_name,status,current_stage,created_at,started_at,parameters_json,rollback_of)
         VALUES(?1,?2,'rollback','Rollback','running','rollback',?3,?3,'{}',?4)",
        params![rollback_id,root.to_string_lossy(),timestamp,run_id],
    ).map_err(|error| error.to_string())?;
    for (index, entry) in journal.iter().enumerate() {
        let apply_result = if fail_after == Some(index) {
            Err("injected rollback apply failure".to_string())
        } else {
            apply_rollback_entry(entry)
        };
        if let Err(error) = apply_result {
            let compensation = compensate_rollback(&journal);
            let status = if compensation.is_ok() {
                "failed"
            } else {
                "failed_partial"
            };
            let detail = if let Err(compensation_error) = compensation {
                format!("{error}; compensation failed: {compensation_error}")
            } else {
                error
            };
            let result =
                rollback_failure(connection, &rollback_id, run_id, &detail, status, &journal);
            let _ = fs::remove_dir_all(&staging_root);
            return result;
        }
    }
    let db_result: Result<(), String> = (|| {
        let tx = connection
            .transaction()
            .map_err(|error| format!("begin rollback result transaction failed: {error}"))?;
        tx.execute(
            "UPDATE compile_runs SET status='succeeded',finished_at=?2,result_json=?3 WHERE id=?1",
            params![
                rollback_id,
                now(),
                serde_json::json!({"restoredArtifacts":artifacts.len()}).to_string()
            ],
        )
        .map_err(|error| format!("update rollback run failed: {error}"))?;
        tx.execute(
            "UPDATE compile_runs SET status='rolled_back',finished_at=?2 WHERE id=?1",
            params![run_id, now()],
        )
        .map_err(|error| format!("update source run failed: {error}"))?;
        tx.execute(
            "INSERT INTO compile_run_events(run_id,sequence,event_kind,stage,message,created_at) VALUES(?1,1,'completed','rollback',?2,?3)",
            params![rollback_id,format!("Restored {} artifact(s) from {run_id}",artifacts.len()),now()],
        )
        .map_err(|error| format!("write rollback event failed: {error}"))?;
        tx.commit()
            .map_err(|error| format!("commit rollback result failed: {error}"))
    })();
    if let Err(error) = db_result {
        let compensation = compensate_rollback(&journal);
        let status = if compensation.is_ok() {
            "failed"
        } else {
            "failed_partial"
        };
        let detail = if let Err(compensation_error) = compensation {
            format!("database commit failed: {error}; compensation failed: {compensation_error}")
        } else {
            format!("database commit failed: {error}")
        };
        let result = rollback_failure(connection, &rollback_id, run_id, &detail, status, &journal);
        let _ = fs::remove_dir_all(&staging_root);
        return result;
    }
    let _ = fs::remove_dir_all(&staging_root);
    Ok(format!("rollback completed: {rollback_id}"))
}

fn apply_rollback_entry(entry: &RollbackJournalEntry) -> Result<(), String> {
    match entry.artifact.operation.as_str() {
        "created" | "modified" => {
            fs::rename(&entry.target, &entry.quarantine).map_err(|error| {
                format!("quarantine {} failed: {error}", entry.artifact.relative)
            })?;
            if entry.artifact.operation == "modified" {
                let staged = entry
                    .staged_backup
                    .as_ref()
                    .expect("modified backup staged");
                if let Some(parent) = entry.target.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|error| format!("restore parent failed: {error}"))?;
                }
                fs::rename(staged, &entry.target)
                    .map_err(|error| format!("restore {} failed: {error}", entry.artifact.relative))
            } else {
                Ok(())
            }
        }
        "deleted" => {
            let staged = entry.staged_backup.as_ref().expect("deleted backup staged");
            if let Some(parent) = entry.target.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("restore parent failed: {error}"))?;
            }
            fs::rename(staged, &entry.target)
                .map_err(|error| format!("restore {} failed: {error}", entry.artifact.relative))
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn request(kind: &str) -> StartCompileRequest {
        StartCompileRequest {
            task_kind: kind.into(),
            input_path: None,
            dry_run: false,
            download: false,
            force: false,
            timeout_seconds: None,
            literature_mode: String::new(),
            candidate_ids: Vec::new(),
            manual_session_id: String::new(),
            run_manifest: None,
        }
    }

    #[test]
    fn schema_version_and_interrupted_recovery() {
        let connection = Connection::open_in_memory().unwrap();
        db_schema(&connection).unwrap();
        connection.execute("INSERT INTO compile_runs(id,repository_path,task_kind,display_name,status,current_stage,created_at) VALUES('x','r','lint','Lint','running','lint','1')",[]).unwrap();
        db_schema(&connection).unwrap();
        let running: String = connection
            .query_row("SELECT status FROM compile_runs WHERE id='x'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(running, "running");
        assert_eq!(recover_interrupted_runs(&connection).unwrap(), 1);
        let status: String = connection
            .query_row("SELECT status FROM compile_runs WHERE id='x'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(status, "interrupted");
        let version: i64 = connection
            .pragma_query_value(None, "user_version", |r| r.get(0))
            .unwrap();
        assert_eq!(version, COMPILE_SCHEMA_VERSION);
    }

    #[test]
    fn rollback_restores_modified_file_and_records_run() {
        let directory = tempdir().unwrap();
        let root = directory.path().join("repository");
        fs::create_dir_all(root.join("wiki")).unwrap();
        let target = root.join("wiki/page.md");
        fs::write(&target, "after").unwrap();
        let backup = directory.path().join("backup/page.md");
        fs::create_dir_all(backup.parent().unwrap()).unwrap();
        fs::write(&backup, "before").unwrap();
        let mut connection = Connection::open_in_memory().unwrap();
        db_schema(&connection).unwrap();
        connection.execute(
            "INSERT INTO compile_runs(id,repository_path,task_kind,display_name,status,current_stage,created_at,parameters_json) VALUES('run',?1,'compile_a','Compile','succeeded','compile_a','1',?2)",
            params![root.to_string_lossy(),serde_json::to_string(&request("compile_a")).unwrap()],
        ).unwrap();
        connection.execute(
            "INSERT INTO compile_artifacts(id,run_id,artifact_kind,relative_path,operation,before_hash,after_hash,rollback_eligible,backup_path) VALUES('a','run','wiki','wiki/page.md','modified',?1,?2,1,?3)",
            params![file_hash(&backup),file_hash(&target),backup.to_string_lossy()],
        ).unwrap();
        let result = rollback_run(&mut connection, &root, "run").unwrap();
        assert!(result.starts_with("rollback completed:"));
        assert_eq!(fs::read_to_string(&target).unwrap(), "before");
        let status: String = connection
            .query_row(
                "SELECT status FROM compile_runs WHERE id='run'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(status, "rolled_back");
    }

    #[test]
    fn rollback_restores_mixed_artifacts_and_compensates_partial_failure() {
        let directory = tempdir().unwrap();
        let root = directory.path().join("repository");
        fs::create_dir_all(root.join("wiki")).unwrap();
        let created = root.join("wiki/created.md");
        let modified = root.join("wiki/modified.md");
        let deleted = root.join("wiki/deleted.md");
        fs::write(&created, "created-after").unwrap();
        fs::write(&modified, "modified-after").unwrap();
        let backup_root = directory.path().join("backup");
        fs::create_dir_all(&backup_root).unwrap();
        let modified_backup = backup_root.join("modified.md");
        let deleted_backup = backup_root.join("deleted.md");
        fs::write(&modified_backup, "modified-before").unwrap();
        fs::write(&deleted_backup, "deleted-before").unwrap();
        let mut connection = Connection::open_in_memory().unwrap();
        db_schema(&connection).unwrap();
        connection
            .execute(
                "INSERT INTO compile_runs(id,repository_path,task_kind,display_name,status,current_stage,created_at,parameters_json) VALUES('mixed',?1,'compile_a','Compile','succeeded','compile_a','1',?2)",
                params![root.to_string_lossy(), serde_json::to_string(&request("compile_a")).unwrap()],
            )
            .unwrap();
        let insert = |id: &str,
                      path: &str,
                      operation: &str,
                      before: &str,
                      after: &str,
                      backup: &Path| {
            connection
                .execute(
                    "INSERT INTO compile_artifacts(id,run_id,artifact_kind,relative_path,operation,before_hash,after_hash,rollback_eligible,backup_path) VALUES(?1,'mixed','wiki',?2,?3,?4,?5,1,?6)",
                    params![id, path, operation, before, after, backup.to_string_lossy()],
                )
                .unwrap();
        };
        insert(
            "created",
            "wiki/created.md",
            "created",
            "",
            &file_hash(&created),
            Path::new(""),
        );
        insert(
            "modified",
            "wiki/modified.md",
            "modified",
            &file_hash(&modified_backup),
            &file_hash(&modified),
            &modified_backup,
        );
        insert(
            "deleted",
            "wiki/deleted.md",
            "deleted",
            &file_hash(&deleted_backup),
            "",
            &deleted_backup,
        );
        let failed = rollback_run_inner(&mut connection, &root, "mixed", Some(1));
        assert!(failed.is_err(), "unexpected rollback result: {failed:?}");
        assert_eq!(fs::read_to_string(&created).unwrap(), "created-after");
        assert_eq!(fs::read_to_string(&modified).unwrap(), "modified-after");
        assert!(!deleted.exists());
        let rollback_status: String = connection
            .query_row(
                "SELECT status FROM compile_runs WHERE rollback_of='mixed' ORDER BY rowid DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(rollback_status, "failed");
        let source_status: String = connection
            .query_row(
                "SELECT status FROM compile_runs WHERE id='mixed'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(source_status, "succeeded");

        let result = rollback_run(&mut connection, &root, "mixed").unwrap();
        assert!(result.starts_with("rollback completed:"));
        assert!(!created.exists());
        assert_eq!(fs::read_to_string(&modified).unwrap(), "modified-before");
        assert_eq!(fs::read_to_string(&deleted).unwrap(), "deleted-before");
    }

    #[test]
    fn history_is_repository_isolated_and_details_load() {
        let connection = Connection::open_in_memory().unwrap();
        db_schema(&connection).unwrap();
        let json = serde_json::to_string(&request("lint")).unwrap();
        connection.execute("INSERT INTO compile_runs(id,repository_path,task_kind,display_name,status,current_stage,created_at,parameters_json) VALUES('x','repo-a','lint','Lint','succeeded','lint','1',?1)",[json]).unwrap();
        connection.execute("INSERT INTO compile_run_events(run_id,sequence,event_kind,stage,message,created_at) VALUES('x',1,'completed','lint','ok','2')",[]).unwrap();
        connection.execute("INSERT INTO compile_artifacts(id,run_id,artifact_kind,relative_path,operation,rollback_eligible) VALUES('a','x','report','logs/report.md','derived',0)",[]).unwrap();
        assert_eq!(list_runs(&connection, "repo-a", 10).unwrap().len(), 1);
        assert!(list_runs(&connection, "repo-b", 10).unwrap().is_empty());
        let detail = get_run(&connection, "repo-a", "x").unwrap();
        assert_eq!(detail.events.len(), 1);
        assert_eq!(detail.artifacts.len(), 1);
        assert!(get_run(&connection, "repo-b", "x").is_err());
    }

    #[test]
    fn allowlist_and_input_boundary_are_enforced() {
        let root = tempdir().unwrap();
        let outside = tempdir().unwrap();
        assert!(build_task(root.path(), &request("shell"), None)
            .unwrap_err()
            .contains("allowlist"));
        let mut parse = request("parse");
        parse.input_path = Some(outside.path().to_string_lossy().into_owned());
        assert!(build_task(root.path(), &parse, None)
            .unwrap_err()
            .contains("outside repository"));
    }

    #[test]
    fn full_pipeline_plan_contains_all_governed_stages() {
        let root = tempdir().unwrap();
        let plan = build_pipeline(root.path(), &request("full_pipeline")).unwrap();
        assert_eq!(
            plan,
            vec![
                "discover",
                "compile_a",
                "lint",
                "graphify_update",
                "rebuild_snapshot",
                "verify",
            ]
        );
        let task = build_task(root.path(), &request("full_pipeline"), Some("run-1")).unwrap();
        assert_eq!(task.executable, "py");
        assert!(task.args.iter().any(|arg| arg.contains("run-1.pause")));
    }

    #[test]
    fn full_pipeline_pause_uses_safe_boundary_control_file() {
        let root = tempdir().unwrap();
        let connection = Connection::open_in_memory().unwrap();
        db_schema(&connection).unwrap();
        connection.execute(
            "INSERT INTO compile_runs(id,repository_path,task_kind,display_name,status,current_stage,created_at) VALUES('run',?1,'full_pipeline','Pipeline','running','discover','1')",
            [root.path().to_string_lossy().to_string()],
        ).unwrap();
        set_pause_requested(&connection, root.path(), "run", true).unwrap();
        assert!(pause_control_path(root.path(), "run").exists());
        set_pause_requested(&connection, root.path(), "run", false).unwrap();
        assert!(!pause_control_path(root.path(), "run").exists());
        let status: String = connection
            .query_row(
                "SELECT status FROM compile_runs WHERE id='run'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(status, "resume_requested");
    }

    #[test]
    fn secrets_are_redacted() {
        let output = redact(
            "MINERU_API_KEY=secret Authorization: Bearer token https://x.test?a=1&signature=signed",
        );
        assert!(!output.contains("secret"));
        assert!(!output.contains("Bearer token"));
        assert!(!output.contains("signed"));
        assert!(output.contains("[REDACTED]"));
    }
}
