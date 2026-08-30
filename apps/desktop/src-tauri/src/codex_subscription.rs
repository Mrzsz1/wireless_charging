use crate::process_support::{configure_background_command, terminate_process_tree};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use uuid::Uuid;

const STATUS_TIMEOUT: Duration = Duration::from_secs(8);
const POLL_INTERVAL: Duration = Duration::from_millis(60);

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexModelOption {
    pub id: String,
    pub display_name: String,
    pub default_reasoning_effort: String,
    pub supported_reasoning_efforts: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodexSubscriptionStatus {
    pub installed: bool,
    pub version: String,
    pub authenticated: bool,
    pub ready: bool,
    pub status_label: String,
    pub diagnostic: String,
    pub configured_model: String,
    pub configured_reasoning_effort: String,
    pub available_models: Vec<CodexModelOption>,
    pub model_catalog_status: String,
}

impl CodexSubscriptionStatus {
    fn unavailable(diagnostic: &str) -> Self {
        Self {
            installed: false,
            version: String::new(),
            authenticated: false,
            ready: false,
            status_label: "Codex CLI 未安装".to_string(),
            diagnostic: diagnostic.to_string(),
            configured_model: String::new(),
            configured_reasoning_effort: String::new(),
            available_models: Vec::new(),
            model_catalog_status: "missing".to_string(),
        }
    }
}

#[derive(Debug, Default)]
struct CodexConfigProjection {
    model: String,
    model_reasoning_effort: String,
}

fn parse_toml_string(value: &str) -> Option<String> {
    let value = value.trim();
    if value.starts_with('"') && value.ends_with('"') {
        serde_json::from_str::<String>(value).ok()
    } else if value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2 {
        Some(value[1..value.len() - 1].to_string())
    } else {
        None
    }
}

fn parse_codex_config(content: &str) -> CodexConfigProjection {
    let mut projection = CodexConfigProjection::default();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            break;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let Some(value) = parse_toml_string(value) else {
            continue;
        };
        match key.trim() {
            "model" => projection.model = value,
            "model_reasoning_effort" => projection.model_reasoning_effort = value,
            _ => {}
        }
    }
    projection
}

#[derive(Debug, Default, Deserialize)]
struct ModelCache {
    #[serde(default)]
    models: Vec<CachedModel>,
}

#[derive(Debug, Default, Deserialize)]
struct CachedModel {
    #[serde(default)]
    slug: String,
    #[serde(default)]
    display_name: String,
    #[serde(default)]
    default_reasoning_level: String,
    #[serde(default)]
    supported_reasoning_levels: Vec<CachedReasoningLevel>,
    #[serde(default)]
    visibility: String,
}

#[derive(Debug, Default, Deserialize)]
struct CachedReasoningLevel {
    #[serde(default)]
    effort: String,
}

fn safe_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 120
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
}

fn valid_reasoning_effort(value: &str) -> bool {
    matches!(
        value,
        "none" | "low" | "medium" | "high" | "xhigh" | "max" | "ultra"
    )
}

fn codex_home() -> Option<PathBuf> {
    env::var_os("CODEX_HOME").map(PathBuf::from).or_else(|| {
        env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
            .map(PathBuf::from)
            .map(|home| home.join(".codex"))
    })
}

fn load_model_catalog(home: Option<&Path>) -> (String, String, Vec<CodexModelOption>, String) {
    let Some(home) = home else {
        return (
            String::new(),
            String::new(),
            Vec::new(),
            "missing".to_string(),
        );
    };
    let config = fs::read_to_string(home.join("config.toml"))
        .ok()
        .map(|content| parse_codex_config(&content))
        .unwrap_or_default();
    let configured_model = config.model.trim().to_string();
    let configured_model = if safe_identifier(&configured_model) {
        configured_model
    } else {
        String::new()
    };
    let configured_reasoning_effort = config.model_reasoning_effort.trim().to_string();
    let configured_reasoning_effort = if valid_reasoning_effort(&configured_reasoning_effort) {
        configured_reasoning_effort
    } else {
        String::new()
    };

    let cache_path = home.join("models_cache.json");
    let cache_content = match fs::read_to_string(&cache_path) {
        Ok(content) => content,
        Err(_) => {
            let fallback = configured_model
                .is_empty()
                .then(Vec::new)
                .unwrap_or_else(|| {
                    vec![CodexModelOption {
                        id: configured_model.clone(),
                        display_name: configured_model.clone(),
                        default_reasoning_effort: configured_reasoning_effort.clone(),
                        supported_reasoning_efforts: Vec::new(),
                    }]
                });
            return (
                configured_model,
                configured_reasoning_effort,
                fallback,
                "missing".to_string(),
            );
        }
    };
    let cache = match serde_json::from_str::<ModelCache>(&cache_content) {
        Ok(cache) => cache,
        Err(_) => {
            let fallback = if configured_model.is_empty() {
                Vec::new()
            } else {
                vec![CodexModelOption {
                    id: configured_model.clone(),
                    display_name: configured_model.clone(),
                    default_reasoning_effort: configured_reasoning_effort.clone(),
                    supported_reasoning_efforts: Vec::new(),
                }]
            };
            return (
                configured_model,
                configured_reasoning_effort,
                fallback,
                "invalid".to_string(),
            );
        }
    };
    let mut seen = HashSet::new();
    let available_models = cache
        .models
        .into_iter()
        .filter(|model| model.visibility == "list" && safe_identifier(model.slug.trim()))
        .filter_map(|model| {
            let id = model.slug.trim().to_string();
            if !seen.insert(id.clone()) {
                return None;
            }
            let supported_reasoning_efforts = model
                .supported_reasoning_levels
                .into_iter()
                .map(|level| level.effort.trim().to_string())
                .filter(|effort| valid_reasoning_effort(effort))
                .collect::<Vec<_>>();
            let default_reasoning_effort = model.default_reasoning_level.trim().to_string();
            Some(CodexModelOption {
                display_name: if model.display_name.trim().is_empty() {
                    id.clone()
                } else {
                    model.display_name.trim().to_string()
                },
                default_reasoning_effort: if valid_reasoning_effort(&default_reasoning_effort) {
                    default_reasoning_effort
                } else {
                    String::new()
                },
                supported_reasoning_efforts,
                id,
            })
        })
        .collect();
    (
        configured_model,
        configured_reasoning_effort,
        available_models,
        "detected".to_string(),
    )
}

pub fn resolve_model_selection(
    model_override: &str,
    reasoning_override: &str,
    status: &CodexSubscriptionStatus,
) -> (String, String) {
    let model = if safe_identifier(model_override.trim()) {
        model_override.trim().to_string()
    } else {
        status.configured_model.clone()
    };
    let selected = status.available_models.iter().find(|item| item.id == model);
    let requested_effort = if valid_reasoning_effort(reasoning_override.trim()) {
        reasoning_override.trim()
    } else if valid_reasoning_effort(&status.configured_reasoning_effort) {
        status.configured_reasoning_effort.as_str()
    } else {
        selected
            .map(|item| item.default_reasoning_effort.as_str())
            .unwrap_or_default()
    };
    let effort = if let Some(selected) =
        selected.filter(|item| !item.supported_reasoning_efforts.is_empty())
    {
        if selected
            .supported_reasoning_efforts
            .iter()
            .any(|value| value == requested_effort)
        {
            requested_effort.to_string()
        } else if selected
            .supported_reasoning_efforts
            .iter()
            .any(|value| value == &selected.default_reasoning_effort)
        {
            selected.default_reasoning_effort.clone()
        } else {
            String::new()
        }
    } else if valid_reasoning_effort(requested_effort) {
        requested_effort.to_string()
    } else {
        String::new()
    };
    (model, effort)
}

fn explicit_executable() -> Option<String> {
    ["CODEX_CLI_PATH", "WIRELESS_CODEX_BIN"]
        .into_iter()
        .find_map(|name| {
            env::var(name)
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })
}

fn candidate_key(path: &Path) -> String {
    let value = path.to_string_lossy().to_string();
    if cfg!(windows) {
        value.to_ascii_lowercase()
    } else {
        value
    }
}

fn push_existing_candidate(
    candidates: &mut Vec<PathBuf>,
    seen: &mut HashSet<String>,
    path: PathBuf,
) {
    if path.is_file() && seen.insert(candidate_key(&path)) {
        candidates.push(path);
    }
}

#[cfg(windows)]
fn append_windows_path_candidates(
    candidates: &mut Vec<PathBuf>,
    seen: &mut HashSet<String>,
    path_value: &OsStr,
    native_only: bool,
) {
    let names: &[&str] = if native_only {
        &["codex.exe"]
    } else {
        &["codex.cmd", "codex.bat", "codex"]
    };
    for directory in env::split_paths(path_value) {
        for name in names {
            push_existing_candidate(candidates, seen, directory.join(name));
        }
    }
}

#[cfg(windows)]
fn read_registry_path(key: &str) -> Option<OsString> {
    let mut command = Command::new("reg.exe");
    configure_background_command(&mut command);
    let output = command
        .args(["query", key, "/v", "Path"])
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    text.lines().find_map(|line| {
        let marker = line.find("REG_")?;
        let after_type = &line[marker..];
        let value_start = after_type.find(char::is_whitespace)?;
        let value = after_type[value_start..].trim();
        (!value.is_empty()).then(|| OsString::from(value))
    })
}

#[cfg(windows)]
fn append_codex_desktop_binaries(
    candidates: &mut Vec<PathBuf>,
    seen: &mut HashSet<String>,
    local_app_data: &Path,
) {
    let bin_root = local_app_data.join("OpenAI").join("Codex").join("bin");
    let mut discovered = fs::read_dir(&bin_root)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path().join("codex.exe"))
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    discovered.sort_by_key(|path| {
        fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .ok()
    });
    for path in discovered.into_iter().rev() {
        push_existing_candidate(candidates, seen, path);
    }
}

fn discovered_executables() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let mut seen = HashSet::new();

    #[cfg(windows)]
    {
        if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
            let local_app_data = PathBuf::from(local_app_data);
            append_codex_desktop_binaries(&mut candidates, &mut seen, &local_app_data);
            push_existing_candidate(
                &mut candidates,
                &mut seen,
                local_app_data
                    .join("Microsoft")
                    .join("WinGet")
                    .join("Links")
                    .join("codex.exe"),
            );
            push_existing_candidate(
                &mut candidates,
                &mut seen,
                local_app_data
                    .join("Microsoft")
                    .join("WindowsApps")
                    .join("codex.exe"),
            );
        }

        let process_path = env::var_os("PATH");
        if let Some(path) = process_path.as_deref() {
            append_windows_path_candidates(&mut candidates, &mut seen, path, true);
        }
        let registry_paths = [
            read_registry_path(r"HKCU\Environment"),
            read_registry_path(
                r"HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
            ),
        ];
        for path in registry_paths.iter().flatten() {
            append_windows_path_candidates(&mut candidates, &mut seen, path, true);
        }
        if let Some(path) = process_path.as_deref() {
            append_windows_path_candidates(&mut candidates, &mut seen, path, false);
        }
        for path in registry_paths.iter().flatten() {
            append_windows_path_candidates(&mut candidates, &mut seen, path, false);
        }

        if let Some(app_data) = env::var_os("APPDATA") {
            let app_data = PathBuf::from(app_data);
            for name in ["codex.cmd", "codex.exe", "codex.bat"] {
                push_existing_candidate(
                    &mut candidates,
                    &mut seen,
                    app_data.join("npm").join(name),
                );
            }
        }
        if let Some(profile) = env::var_os("USERPROFILE") {
            let profile = PathBuf::from(profile);
            for path in [
                profile.join(".local").join("bin").join("codex.exe"),
                profile.join(".cargo").join("bin").join("codex.exe"),
                profile.join("scoop").join("shims").join("codex.exe"),
                profile.join("scoop").join("shims").join("codex.cmd"),
            ] {
                push_existing_candidate(&mut candidates, &mut seen, path);
            }
        }
    }

    #[cfg(not(windows))]
    if let Some(path) = env::var_os("PATH") {
        for directory in env::split_paths(&path) {
            push_existing_candidate(&mut candidates, &mut seen, directory.join("codex"));
        }
    }

    candidates
}

fn executable_has_valid_version(executable: &str) -> bool {
    matches!(
        run_fixed_with(executable, &["--version"], STATUS_TIMEOUT),
        Ok((true, stdout, _)) if safe_version(&stdout) != "Codex CLI"
    )
}

pub fn available_executable() -> Option<String> {
    if let Some(explicit) = explicit_executable() {
        return executable_has_valid_version(&explicit).then_some(explicit);
    }
    discovered_executables().into_iter().find_map(|candidate| {
        let executable = candidate.to_string_lossy().into_owned();
        executable_has_valid_version(&executable).then_some(executable)
    })
}

fn executable() -> String {
    available_executable()
        .or_else(explicit_executable)
        .unwrap_or_else(|| "codex".to_string())
}

fn run_fixed_with(
    executable: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<(bool, String, String), String> {
    let mut command = Command::new(executable);
    configure_background_command(&mut command);
    let mut child = command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                "CODEX_NOT_FOUND".to_string()
            } else {
                "CODEX_STATUS_START_FAILED".to_string()
            }
        })?;
    let started = Instant::now();
    loop {
        if child
            .try_wait()
            .map_err(|_| "CODEX_STATUS_WAIT_FAILED".to_string())?
            .is_some()
        {
            break;
        }
        if started.elapsed() >= timeout {
            terminate_process_tree(&mut child);
            return Err("CODEX_STATUS_TIMEOUT".to_string());
        }
        thread::sleep(POLL_INTERVAL);
    }
    let output = child
        .wait_with_output()
        .map_err(|_| "CODEX_STATUS_OUTPUT_FAILED".to_string())?;
    Ok((
        output.status.success(),
        String::from_utf8_lossy(&output.stdout).trim().to_string(),
        String::from_utf8_lossy(&output.stderr).trim().to_string(),
    ))
}

fn safe_version(output: &str) -> String {
    let first = output.lines().next().unwrap_or_default().trim();
    if first.starts_with("codex-cli ")
        && first
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || ".- _".contains(value))
    {
        first.to_string()
    } else {
        "Codex CLI".to_string()
    }
}

fn get_status_with(executable: &str) -> CodexSubscriptionStatus {
    let (configured_model, configured_reasoning_effort, available_models, model_catalog_status) =
        load_model_catalog(codex_home().as_deref());
    let (version_ok, version_stdout, _) =
        match run_fixed_with(executable, &["--version"], STATUS_TIMEOUT) {
            Ok(result) => result,
            Err(error) if error == "CODEX_NOT_FOUND" => {
                return CodexSubscriptionStatus::unavailable(
                    "请安装 Codex CLI，然后在设置中刷新状态。",
                )
            }
            Err(_) => {
                return CodexSubscriptionStatus::unavailable(
                    "Codex CLI 状态检测失败，请检查安装后重试。",
                )
            }
        };
    if !version_ok {
        return CodexSubscriptionStatus::unavailable("Codex CLI 版本检测失败，请检查安装后重试。");
    }
    let version = safe_version(&version_stdout);
    let (login_ok, login_stdout, login_stderr) =
        match run_fixed_with(executable, &["login", "status"], STATUS_TIMEOUT) {
            Ok(result) => result,
            Err(_) => {
                return CodexSubscriptionStatus {
                    installed: true,
                    version,
                    authenticated: false,
                    ready: false,
                    status_label: "登录状态检测失败".to_string(),
                    diagnostic: "请点击“登录 ChatGPT”或稍后刷新状态。".to_string(),
                    configured_model,
                    configured_reasoning_effort,
                    available_models,
                    model_catalog_status,
                }
            }
        };
    let login_summary = format!("{login_stdout}\n{login_stderr}").to_ascii_lowercase();
    let authenticated =
        login_ok && login_summary.contains("logged in") && login_summary.contains("chatgpt");
    CodexSubscriptionStatus {
        installed: true,
        version,
        authenticated,
        ready: authenticated,
        status_label: if authenticated {
            "已使用 ChatGPT 登录".to_string()
        } else {
            "尚未使用 ChatGPT 登录".to_string()
        },
        diagnostic: if authenticated {
            "可直接使用订阅问答，无需导入 API Key。".to_string()
        } else {
            "点击“登录 ChatGPT”完成官方浏览器登录。".to_string()
        },
        configured_model,
        configured_reasoning_effort,
        available_models,
        model_catalog_status,
    }
}

pub fn get_status() -> CodexSubscriptionStatus {
    get_status_with(&executable())
}

fn start_login_with(executable: &str) -> Result<String, String> {
    let mut command = Command::new(executable);
    configure_background_command(&mut command);
    command
        .arg("login")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                "未找到 Codex CLI，请先安装后重试".to_string()
            } else {
                "启动 Codex 登录流程失败".to_string()
            }
        })?;
    Ok("已启动 Codex 官方登录流程；在浏览器完成后点击“刷新状态”。".to_string())
}

pub fn start_login() -> Result<String, String> {
    start_login_with(&executable())
}

enum OutputLine {
    Line(String),
    Closed,
}

struct TempWorkspace(std::path::PathBuf);

impl TempWorkspace {
    fn create() -> Result<Self, String> {
        let path = env::temp_dir().join(format!("wireless-charging-codex-{}", Uuid::new_v4()));
        fs::create_dir(&path).map_err(|_| "CODEX_TEMP_CREATE_FAILED".to_string())?;
        Ok(Self(path))
    }

    fn write_output_schema(&self, schema: &Value) -> Result<PathBuf, String> {
        let path = self.0.join("answer-schema.json");
        let bytes = serde_json::to_vec_pretty(schema)
            .map_err(|_| "CODEX_OUTPUT_SCHEMA_SERIALIZE_FAILED".to_string())?;
        fs::write(&path, bytes).map_err(|_| "CODEX_OUTPUT_SCHEMA_WRITE_FAILED".to_string())?;
        Ok(path)
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn event_text(value: &Value) -> Option<&str> {
    value
        .pointer("/item/text")
        .or_else(|| value.pointer("/message/content"))
        .or_else(|| value.pointer("/content"))
        .and_then(Value::as_str)
}

fn event_delta(value: &Value) -> Option<&str> {
    value
        .pointer("/delta")
        .or_else(|| value.pointer("/item/delta"))
        .or_else(|| value.pointer("/message/delta"))
        .and_then(Value::as_str)
}

fn event_model(value: &Value) -> Option<String> {
    [
        "/model",
        "/model_slug",
        "/turn/model",
        "/thread/model",
        "/response/model",
    ]
    .iter()
    .find_map(|pointer| value.pointer(pointer).and_then(Value::as_str))
    .map(str::trim)
    .filter(|model| !model.is_empty() && model.len() <= 120 && !model.chars().any(char::is_control))
    .map(str::to_string)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CodexTerminalFailure {
    event_type: String,
    category: String,
    message_sha256: String,
}

impl CodexTerminalFailure {
    fn stable_error(&self) -> String {
        let prefix = if self.event_type == "turn.failed" {
            "CODEX_JSONL_TURN_FAILED"
        } else {
            "CODEX_JSONL_ERROR"
        };
        format!("{prefix}: {}", self.category)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CodexJsonlObservation {
    Activity {
        event_type: String,
        model: Option<String>,
    },
    AgentDelta {
        event_type: String,
        text: String,
        model: Option<String>,
    },
    AgentCompleted {
        event_type: String,
        text: String,
        model: Option<String>,
    },
    TurnCompleted {
        model: Option<String>,
    },
    Fatal(CodexTerminalFailure),
    NonFatalItemWarning(CodexTerminalFailure),
}

fn contains_any(value: &str, markers: &[&str]) -> bool {
    markers.iter().any(|marker| value.contains(marker))
}

fn classify_codex_terminal_message(message: &str) -> &'static str {
    let lower = message.to_ascii_lowercase();
    if contains_any(&lower, &["cancelled", "canceled", "user aborted"]) {
        "cancelled"
    } else if contains_any(
        &lower,
        &[
            "context length",
            "too many tokens",
            "maximum context",
            "input too large",
            "context window",
        ],
    ) {
        "context_too_large"
    } else if contains_any(
        &lower,
        &[
            "json schema",
            "response_format",
            "response format",
            "response schema",
            "invalid response schema",
            "unsupported keyword",
            "additionalproperties",
            "additional properties",
        ],
    ) {
        "schema_rejected"
    } else if contains_any(
        &lower,
        &[
            "unsupported model",
            "model is not supported",
            "model not supported",
        ],
    ) {
        "unsupported_model"
    } else if contains_any(
        &lower,
        &[
            "model unavailable",
            "model is unavailable",
            "model not available",
        ],
    ) {
        "model_unavailable"
    } else if contains_any(
        &lower,
        &[
            "unauthorized",
            "authentication",
            "login required",
            "token expired",
            "http 401",
            "status 401",
        ],
    ) {
        "auth_required"
    } else if contains_any(
        &lower,
        &[
            "usage limit",
            "quota",
            "insufficient_quota",
            "credit balance",
        ],
    ) {
        "usage_limit"
    } else if contains_any(
        &lower,
        &["rate limit", "rate_limit", "http 429", "status 429"],
    ) {
        "rate_limit"
    } else if contains_any(
        &lower,
        &[
            "overloaded",
            "server error",
            "service unavailable",
            "http 502",
            "http 503",
            "status 502",
            "status 503",
        ],
    ) {
        "server_overloaded"
    } else if contains_any(
        &lower,
        &[
            "websocket",
            "transport",
            "request timed out",
            "stream disconnected",
        ],
    ) {
        "transport"
    } else if contains_any(
        &lower,
        &["connection", "network error", "dns error", "tls error"],
    ) {
        "connection"
    } else if contains_any(
        &lower,
        &["protocol error", "invalid frame", "malformed event"],
    ) {
        "protocol"
    } else if contains_any(
        &lower,
        &[
            "invalid_request",
            "invalid request",
            "bad request",
            "http 400",
            "status 400",
        ],
    ) {
        "bad_request"
    } else if lower.contains("schema") {
        "schema_rejected"
    } else {
        "unknown"
    }
}

fn terminal_message(value: &Value, item_warning: bool) -> &str {
    let pointers: &[&str] = if item_warning {
        &["/item/error/message", "/item/message", "/item/error"]
    } else {
        &["/error/message", "/message", "/error"]
    };
    pointers
        .iter()
        .find_map(|pointer| value.pointer(pointer).and_then(Value::as_str))
        .unwrap_or_default()
}

fn terminal_failure(value: &Value, event_type: &str, item_warning: bool) -> CodexTerminalFailure {
    let message = terminal_message(value, item_warning);
    CodexTerminalFailure {
        event_type: event_type.to_string(),
        category: classify_codex_terminal_message(message).to_string(),
        message_sha256: format!("{:x}", Sha256::digest(message.as_bytes())),
    }
}

fn parse_codex_jsonl_line(line: &str) -> Option<CodexJsonlObservation> {
    let value = serde_json::from_str::<Value>(line).ok()?;
    let event_type = value
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    if matches!(event_type.as_str(), "turn.failed" | "error") {
        return Some(CodexJsonlObservation::Fatal(terminal_failure(
            &value,
            &event_type,
            false,
        )));
    }
    let model = event_model(&value);
    if event_type == "turn.completed" {
        return Some(CodexJsonlObservation::TurnCompleted { model });
    }
    if event_type == "item.completed"
        && value.pointer("/item/type").and_then(Value::as_str) == Some("error")
    {
        return Some(CodexJsonlObservation::NonFatalItemWarning(
            terminal_failure(&value, "item.error", true),
        ));
    }
    if let Some(delta) = event_delta(&value).filter(|_| event_type.contains("delta")) {
        return Some(CodexJsonlObservation::AgentDelta {
            event_type,
            text: delta.to_string(),
            model,
        });
    }
    let item_is_agent =
        value.pointer("/item/type").and_then(Value::as_str) == Some("agent_message");
    if item_is_agent || event_type.contains("message") {
        if let Some(text) = event_text(&value).filter(|text| !text.is_empty()) {
            return Some(CodexJsonlObservation::AgentCompleted {
                event_type,
                text: text.to_string(),
                model,
            });
        }
    }
    Some(CodexJsonlObservation::Activity { event_type, model })
}

fn apply_completed_text(text: &str, answer: &mut String) -> Option<String> {
    if text.starts_with(answer.as_str()) {
        let suffix = &text[answer.len()..];
        answer.clear();
        answer.push_str(text);
        (!suffix.is_empty()).then(|| suffix.to_string())
    } else if answer.is_empty() {
        answer.push_str(text);
        Some(text.to_string())
    } else {
        None
    }
}

enum AppliedJsonlLine {
    Ignored,
    Activity,
    Fatal(CodexTerminalFailure),
}

#[derive(Debug, Default)]
struct CodexJsonlState {
    answer: String,
    resolved_model: String,
    turn_completed_seen: bool,
    agent_message_seen: bool,
    last_jsonl_event_type: String,
    jsonl_event_count: usize,
    item_warning_count: usize,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CodexExecDiagnostics {
    pub status: String,
    pub terminal_event_type: String,
    pub failure_category: String,
    pub message_sha256: String,
    pub exit_code: i32,
    pub last_jsonl_event_type: String,
    pub turn_completed_seen: bool,
    pub agent_message_seen: bool,
    pub jsonl_event_count: usize,
    pub item_warning_count: usize,
    pub stderr_non_empty: bool,
}

fn sync_jsonl_diagnostics(diagnostics: &mut CodexExecDiagnostics, state: &CodexJsonlState) {
    diagnostics.last_jsonl_event_type = state.last_jsonl_event_type.clone();
    diagnostics.turn_completed_seen = state.turn_completed_seen;
    diagnostics.agent_message_seen = state.agent_message_seen;
    diagnostics.jsonl_event_count = state.jsonl_event_count;
    diagnostics.item_warning_count = state.item_warning_count;
}

fn record_terminal_failure(diagnostics: &mut CodexExecDiagnostics, failure: &CodexTerminalFailure) {
    diagnostics.status = "failed".to_string();
    diagnostics.terminal_event_type = failure.event_type.clone();
    diagnostics.failure_category = failure.category.clone();
    diagnostics.message_sha256 = failure.message_sha256.clone();
}

struct CodexRawDiagnosticCapture {
    directory: PathBuf,
    stdout_lines: Vec<String>,
    stderr: String,
    prompt_sha256: String,
    schema_sha256: String,
    cli_version: String,
}

impl CodexRawDiagnosticCapture {
    fn new(directory: PathBuf, prompt: &str, schema: &Value, cli_version: &str) -> Self {
        Self {
            directory,
            stdout_lines: Vec::new(),
            stderr: String::new(),
            prompt_sha256: format!("{:x}", Sha256::digest(prompt.as_bytes())),
            schema_sha256: format!(
                "{:x}",
                Sha256::digest(serde_json::to_vec(schema).unwrap_or_default())
            ),
            cli_version: cli_version.to_string(),
        }
    }

    fn record_stdout(&mut self, line: &str) {
        self.stdout_lines.push(line.to_string());
    }

    fn record_stderr(&mut self, stderr: String) {
        self.stderr = stderr;
    }

    fn write(&self, diagnostics: &CodexExecDiagnostics) -> Result<(), String> {
        fs::create_dir_all(&self.directory)
            .map_err(|_| "CODEX_DIAGNOSTIC_DIRECTORY_CREATE_FAILED".to_string())?;
        fs::write(
            self.directory.join("stdout.jsonl"),
            self.stdout_lines.join("\n"),
        )
        .map_err(|_| "CODEX_DIAGNOSTIC_STDOUT_WRITE_FAILED".to_string())?;
        fs::write(self.directory.join("stderr.txt"), &self.stderr)
            .map_err(|_| "CODEX_DIAGNOSTIC_STDERR_WRITE_FAILED".to_string())?;
        let metadata = serde_json::json!({
            "schemaVersion": "qa-codex-exec-raw-diagnostic-v1",
            "cliVersion": self.cli_version,
            "promptSha256": self.prompt_sha256,
            "schemaSha256": self.schema_sha256,
            "diagnostics": diagnostics,
        });
        fs::write(
            self.directory.join("metadata.json"),
            serde_json::to_vec_pretty(&metadata)
                .map_err(|_| "CODEX_DIAGNOSTIC_METADATA_SERIALIZE_FAILED".to_string())?,
        )
        .map_err(|_| "CODEX_DIAGNOSTIC_METADATA_WRITE_FAILED".to_string())
    }
}

fn apply_codex_jsonl_line<F>(
    line: &str,
    state: &mut CodexJsonlState,
    on_token: &mut F,
) -> Result<AppliedJsonlLine, String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    let Some(observation) = parse_codex_jsonl_line(line) else {
        return Ok(AppliedJsonlLine::Ignored);
    };
    state.jsonl_event_count += 1;
    state.last_jsonl_event_type = match &observation {
        CodexJsonlObservation::Activity { event_type, .. }
        | CodexJsonlObservation::AgentDelta { event_type, .. }
        | CodexJsonlObservation::AgentCompleted { event_type, .. } => event_type.clone(),
        CodexJsonlObservation::TurnCompleted { .. } => "turn.completed".to_string(),
        CodexJsonlObservation::Fatal(failure) => failure.event_type.clone(),
        CodexJsonlObservation::NonFatalItemWarning(_) => "item.completed".to_string(),
    };
    let model = match &observation {
        CodexJsonlObservation::Activity { model, .. }
        | CodexJsonlObservation::AgentDelta { model, .. }
        | CodexJsonlObservation::AgentCompleted { model, .. }
        | CodexJsonlObservation::TurnCompleted { model } => model.as_ref(),
        CodexJsonlObservation::Fatal(_) | CodexJsonlObservation::NonFatalItemWarning(_) => None,
    };
    if let Some(model) = model {
        state.resolved_model = model.clone();
    }
    match observation {
        CodexJsonlObservation::AgentDelta { text, .. } => {
            state.answer.push_str(&text);
            on_token(&text)?;
            Ok(AppliedJsonlLine::Activity)
        }
        CodexJsonlObservation::AgentCompleted { text, .. } => {
            state.agent_message_seen = true;
            if let Some(delta) = apply_completed_text(&text, &mut state.answer) {
                on_token(&delta)?;
            }
            Ok(AppliedJsonlLine::Activity)
        }
        CodexJsonlObservation::Fatal(failure) => Ok(AppliedJsonlLine::Fatal(failure)),
        CodexJsonlObservation::Activity { event_type, .. } => {
            let _ = event_type;
            Ok(AppliedJsonlLine::Activity)
        }
        CodexJsonlObservation::TurnCompleted { .. } => {
            state.turn_completed_seen = true;
            Ok(AppliedJsonlLine::Activity)
        }
        CodexJsonlObservation::NonFatalItemWarning(warning) => {
            let _ = warning;
            state.item_warning_count += 1;
            Ok(AppliedJsonlLine::Activity)
        }
    }
}

fn stop_stream_process(
    child: &mut std::process::Child,
    stdout_reader: &mut Option<thread::JoinHandle<()>>,
    stderr_reader: &mut Option<thread::JoinHandle<String>>,
) -> String {
    terminate_process_tree(child);
    if let Some(reader) = stdout_reader.take() {
        let _ = reader.join();
    }
    stderr_reader
        .take()
        .and_then(|reader| reader.join().ok())
        .unwrap_or_default()
}

fn build_exec_args(
    workspace: &std::path::Path,
    output_schema: Option<&std::path::Path>,
    model: &str,
    reasoning_effort: &str,
) -> Vec<String> {
    let mut args = [
        "-a",
        "never",
        "exec",
        "--json",
        "--ephemeral",
        "--skip-git-repo-check",
        "--ignore-user-config",
        "--ignore-rules",
        "--sandbox",
        "read-only",
        "--cd",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();
    args.push(workspace.to_string_lossy().into_owned());
    if !model.trim().is_empty() {
        args.extend(["--model".to_string(), model.trim().to_string()]);
    }
    if valid_reasoning_effort(reasoning_effort.trim()) {
        args.extend([
            "-c".to_string(),
            format!("model_reasoning_effort=\"{}\"", reasoning_effort.trim()),
        ]);
    }
    if let Some(path) = output_schema {
        args.extend([
            "--output-schema".to_string(),
            path.to_string_lossy().into_owned(),
        ]);
    }
    args.push("-".to_string());
    args
}

struct StreamAnswerRequest<'a> {
    executable: &'a str,
    prompt: &'a str,
    output_schema: Option<&'a Value>,
    model: &'a str,
    reasoning_effort: &'a str,
    timeout: Duration,
    cancelled: &'a AtomicBool,
}

fn stream_answer_with_diagnostics<F>(
    request: StreamAnswerRequest<'_>,
    mut on_token: F,
    diagnostics: &mut CodexExecDiagnostics,
    mut raw_capture: Option<&mut CodexRawDiagnosticCapture>,
) -> Result<(String, String), String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    let StreamAnswerRequest {
        executable,
        prompt,
        output_schema,
        model,
        reasoning_effort,
        timeout,
        cancelled,
    } = request;
    *diagnostics = CodexExecDiagnostics {
        status: "failed".to_string(),
        exit_code: -1,
        ..CodexExecDiagnostics::default()
    };
    let workspace = TempWorkspace::create()?;
    let output_schema_path = output_schema
        .map(|schema| workspace.write_output_schema(schema))
        .transpose()?;
    let mut command = Command::new(executable);
    configure_background_command(&mut command);
    command
        .args(build_exec_args(
            &workspace.0,
            output_schema_path.as_deref(),
            model,
            reasoning_effort,
        ))
        .env("NO_COLOR", "1")
        .env("TERM", "dumb")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            "CODEX_NOT_FOUND".to_string()
        } else {
            "CODEX_START_FAILED".to_string()
        }
    })?;
    match child.stdin.take() {
        Some(mut stdin) => {
            if stdin.write_all(prompt.as_bytes()).is_err() {
                terminate_process_tree(&mut child);
                return Err("CODEX_STDIN_FAILED".to_string());
            }
        }
        None => {
            terminate_process_tree(&mut child);
            return Err("CODEX_STDIN_FAILED".to_string());
        }
    }

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            terminate_process_tree(&mut child);
            return Err("CODEX_STDOUT_FAILED".to_string());
        }
    };
    let stderr = match child.stderr.take() {
        Some(stderr) => stderr,
        None => {
            terminate_process_tree(&mut child);
            return Err("CODEX_STDERR_FAILED".to_string());
        }
    };
    let (sender, receiver) = mpsc::channel();
    let mut stdout_reader = Some(thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            match line {
                Ok(line) => {
                    if sender.send(OutputLine::Line(line)).is_err() {
                        return;
                    }
                }
                Err(_) => break,
            }
        }
        let _ = sender.send(OutputLine::Closed);
    }));
    let mut stderr_reader = Some(thread::spawn(move || {
        let mut text = String::new();
        let _ = BufReader::new(stderr)
            .take(16 * 1024)
            .read_to_string(&mut text);
        text
    }));

    let started = Instant::now();
    let mut last_activity = started;
    let idle_timeout = timeout;
    let hard_timeout = timeout
        .saturating_mul(4)
        .max(Duration::from_secs(600))
        .min(Duration::from_secs(1800));
    let mut jsonl_state = CodexJsonlState {
        resolved_model: model.trim().to_string(),
        ..CodexJsonlState::default()
    };
    let status = loop {
        if cancelled.load(Ordering::SeqCst) {
            let stderr = stop_stream_process(&mut child, &mut stdout_reader, &mut stderr_reader);
            if let Some(capture) = raw_capture.as_deref_mut() {
                capture.record_stderr(stderr);
            }
            diagnostics.failure_category = "cancelled".to_string();
            return Err("CODEX_CANCELLED: 用户停止了生成".to_string());
        }
        if started.elapsed() >= hard_timeout {
            let stderr = stop_stream_process(&mut child, &mut stdout_reader, &mut stderr_reader);
            if let Some(capture) = raw_capture.as_deref_mut() {
                capture.record_stderr(stderr);
            }
            diagnostics.failure_category = "total_timeout".to_string();
            return Err("CODEX_TOTAL_TIMEOUT: 订阅回答超过总时限".to_string());
        }
        if last_activity.elapsed() >= idle_timeout {
            let stderr = stop_stream_process(&mut child, &mut stdout_reader, &mut stderr_reader);
            if let Some(capture) = raw_capture.as_deref_mut() {
                capture.record_stderr(stderr);
            }
            diagnostics.failure_category = "idle_timeout".to_string();
            return Err("CODEX_IDLE_TIMEOUT: 订阅回答长时间无活动".to_string());
        }
        match receiver.recv_timeout(POLL_INTERVAL) {
            Ok(OutputLine::Line(line)) => {
                if let Some(capture) = raw_capture.as_deref_mut() {
                    capture.record_stdout(&line);
                }
                let applied = match apply_codex_jsonl_line(&line, &mut jsonl_state, &mut on_token) {
                    Ok(applied) => applied,
                    Err(error) => {
                        let stderr =
                            stop_stream_process(&mut child, &mut stdout_reader, &mut stderr_reader);
                        if let Some(capture) = raw_capture.as_deref_mut() {
                            capture.record_stderr(stderr);
                        }
                        return Err(error);
                    }
                };
                match applied {
                    AppliedJsonlLine::Activity => last_activity = Instant::now(),
                    AppliedJsonlLine::Fatal(failure) => {
                        sync_jsonl_diagnostics(diagnostics, &jsonl_state);
                        record_terminal_failure(diagnostics, &failure);
                        let stderr =
                            stop_stream_process(&mut child, &mut stdout_reader, &mut stderr_reader);
                        if let Some(capture) = raw_capture.as_deref_mut() {
                            capture.record_stderr(stderr);
                        }
                        return Err(failure.stable_error());
                    }
                    AppliedJsonlLine::Ignored => {}
                }
            }
            Ok(OutputLine::Closed) | Err(mpsc::RecvTimeoutError::Disconnected) => {}
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        if let Some(status) = child
            .try_wait()
            .map_err(|_| "CODEX_WAIT_FAILED".to_string())?
        {
            diagnostics.exit_code = status.code().unwrap_or(-1);
            if let Some(reader) = stdout_reader.take() {
                let _ = reader.join();
            }
            let mut terminal_failure = None;
            for output in receiver.try_iter() {
                if let OutputLine::Line(line) = output {
                    if let Some(capture) = raw_capture.as_deref_mut() {
                        capture.record_stdout(&line);
                    }
                    match apply_codex_jsonl_line(&line, &mut jsonl_state, &mut on_token) {
                        Ok(AppliedJsonlLine::Fatal(failure)) => {
                            terminal_failure.get_or_insert(failure);
                        }
                        Ok(AppliedJsonlLine::Activity) | Ok(AppliedJsonlLine::Ignored) => {}
                        Err(error) => {
                            let stderr = stderr_reader
                                .take()
                                .and_then(|reader| reader.join().ok())
                                .unwrap_or_default();
                            if let Some(capture) = raw_capture.as_deref_mut() {
                                capture.record_stderr(stderr);
                            }
                            return Err(error);
                        }
                    }
                }
            }
            if let Some(failure) = terminal_failure {
                sync_jsonl_diagnostics(diagnostics, &jsonl_state);
                record_terminal_failure(diagnostics, &failure);
                let stderr = stderr_reader
                    .take()
                    .and_then(|reader| reader.join().ok())
                    .unwrap_or_default();
                if let Some(capture) = raw_capture.as_deref_mut() {
                    capture.record_stderr(stderr);
                }
                return Err(failure.stable_error());
            }
            break status;
        }
    };
    let stderr_output = stderr_reader
        .take()
        .and_then(|reader| reader.join().ok())
        .unwrap_or_default();
    if let Some(capture) = raw_capture {
        capture.record_stderr(stderr_output.clone());
    }
    diagnostics.stderr_non_empty = !stderr_output.trim().is_empty();
    sync_jsonl_diagnostics(diagnostics, &jsonl_state);
    let answer = jsonl_state.answer.trim().to_string();
    if !status.success() {
        let stderr_category = classify_codex_terminal_message(&stderr_output);
        if output_schema.is_some() && stderr_category == "schema_rejected" {
            diagnostics.failure_category = "schema_rejected".to_string();
            return Err(format!(
                "CODEX_OUTPUT_SCHEMA_REJECTED: Codex CLI 未接受回答结构约束（退出码 {}）",
                status.code().unwrap_or(-1)
            ));
        }
        if stderr_category != "unknown" {
            diagnostics.failure_category = stderr_category.to_string();
            return Err(format!("CODEX_STDERR_FAILURE: {stderr_category}"));
        }
        diagnostics.failure_category = "provider_exit".to_string();
        return Err(format!(
            "CODEX_EXIT_ERROR: Codex CLI 退出码 {}",
            status.code().unwrap_or(-1)
        ));
    }
    if answer.is_empty() || !jsonl_state.agent_message_seen {
        diagnostics.failure_category = "empty_response".to_string();
        return Err("CODEX_RESPONSE_ERROR: 未收到回答文本".to_string());
    }
    diagnostics.status = "succeeded".to_string();
    diagnostics.failure_category.clear();
    Ok((
        answer,
        if jsonl_state.resolved_model.is_empty() {
            "provider-default-unreported".to_string()
        } else {
            jsonl_state.resolved_model
        },
    ))
}

fn stream_answer_with<F>(
    request: StreamAnswerRequest<'_>,
    on_token: F,
) -> Result<(String, String), String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    let mut diagnostics = CodexExecDiagnostics::default();
    stream_answer_with_diagnostics(request, on_token, &mut diagnostics, None)
}

#[derive(Debug, Clone)]
pub(crate) struct CodexStructuredProbeOutcome {
    pub output: Option<String>,
    pub error: String,
    pub diagnostics: CodexExecDiagnostics,
    pub latency_ms: u64,
    pub executable_source_type: String,
    pub executable_version: String,
}

fn executable_source_type(executable: &str) -> String {
    let normalized = executable.replace('\\', "/").to_ascii_lowercase();
    let extension = Path::new(executable)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    if extension == "exe" && normalized.contains("/openai/codex/bin/") {
        "desktop".to_string()
    } else if matches!(extension.as_str(), "cmd" | "bat" | "ps1") {
        "npm-wrapper".to_string()
    } else {
        "native".to_string()
    }
}

pub(crate) fn run_codex_structured_probe(
    prompt: &str,
    schema: &Value,
    model: &str,
    reasoning_effort: &str,
    timeout: Duration,
    raw_diagnostic_directory: Option<&Path>,
) -> Result<CodexStructuredProbeOutcome, String> {
    let executable = executable();
    let status = get_status_with(&executable);
    if !status.ready {
        return Err("CODEX_PROBE_PROVIDER_NOT_READY".to_string());
    }
    let executable_version = status.version;
    let mut diagnostics = CodexExecDiagnostics::default();
    let mut raw_capture = raw_diagnostic_directory.map(|directory| {
        CodexRawDiagnosticCapture::new(directory.to_path_buf(), prompt, schema, &executable_version)
    });
    let started = Instant::now();
    let result = stream_answer_with_diagnostics(
        StreamAnswerRequest {
            executable: &executable,
            prompt,
            output_schema: Some(schema),
            model,
            reasoning_effort,
            timeout,
            cancelled: &AtomicBool::new(false),
        },
        |_| Ok(()),
        &mut diagnostics,
        raw_capture.as_mut(),
    );
    let latency_ms = started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64;
    if let Some(capture) = raw_capture.as_ref() {
        capture.write(&diagnostics)?;
    }
    let (output, error) = match result {
        Ok((output, _)) => (Some(output), String::new()),
        Err(error) => (None, error),
    };
    Ok(CodexStructuredProbeOutcome {
        output,
        error,
        diagnostics,
        latency_ms,
        executable_source_type: executable_source_type(&executable),
        executable_version,
    })
}

pub fn stream_answer<F>(
    prompt: &str,
    output_schema: Option<&Value>,
    model: &str,
    reasoning_effort: &str,
    timeout: Duration,
    cancelled: &AtomicBool,
    on_token: F,
) -> Result<(String, String), String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    let executable = executable();
    stream_answer_with(
        StreamAnswerRequest {
            executable: &executable,
            prompt,
            output_schema,
            model,
            reasoning_effort,
            timeout,
            cancelled,
        },
        on_token,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    fn write_windows_fixture(name: &str, body: &str) -> (TempWorkspace, std::path::PathBuf) {
        let workspace = TempWorkspace::create().unwrap();
        let path = workspace.0.join(format!("{name}.cmd"));
        fs::write(&path, format!("@echo off\r\n{body}\r\n")).unwrap();
        (workspace, path)
    }

    #[cfg(windows)]
    fn run_windows_answer_fixture(
        name: &str,
        body: &str,
        output_schema: Option<&Value>,
    ) -> Result<(String, String), String> {
        let (_workspace, fixture) = write_windows_fixture(name, body);
        stream_answer_with(
            StreamAnswerRequest {
                executable: &fixture.to_string_lossy(),
                prompt: "fixture prompt",
                output_schema,
                model: "",
                reasoning_effort: "",
                timeout: Duration::from_secs(10),
                cancelled: &AtomicBool::new(false),
            },
            |_| Ok(()),
        )
    }

    fn tiny_output_schema() -> Value {
        serde_json::json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["ok"],
            "properties": {"ok": {"type": "boolean"}}
        })
    }

    fn apply_jsonl_line(line: &str, answer: &mut String) -> Option<String> {
        let mut emitted = String::new();
        let mut state = CodexJsonlState {
            answer: std::mem::take(answer),
            ..CodexJsonlState::default()
        };
        let mut on_token = |token: &str| {
            emitted.push_str(token);
            Ok(())
        };
        let _ = apply_codex_jsonl_line(line, &mut state, &mut on_token).ok()?;
        *answer = state.answer;
        (!emitted.is_empty()).then_some(emitted)
    }

    #[cfg(windows)]
    #[test]
    fn j1_turn_failed_schema_event_is_preserved_and_redacted() {
        let schema = tiny_output_schema();
        let error = run_windows_answer_fixture(
            "fake-codex-j1-turn-failed",
            "echo {\"type\":\"thread.started\",\"thread_id\":\"x\"}\r\n\
             echo {\"type\":\"turn.started\"}\r\n\
             echo {\"type\":\"turn.failed\",\"error\":{\"message\":\"Invalid response schema private-detail\"}}\r\n\
             exit /b 1",
            Some(&schema),
        )
        .unwrap_err();

        assert_eq!(error, "CODEX_JSONL_TURN_FAILED: schema_rejected");
        assert!(!error.contains("private-detail"));
    }

    #[cfg(windows)]
    #[test]
    fn j2_top_level_error_event_classifies_rate_limit() {
        let error = run_windows_answer_fixture(
            "fake-codex-j2-rate-limit",
            "echo {\"type\":\"error\",\"message\":\"rate limit exceeded private-detail\"}\r\n\
             exit /b 1",
            None,
        )
        .unwrap_err();

        assert_eq!(error, "CODEX_JSONL_ERROR: rate_limit");
        assert!(!error.contains("private-detail"));
    }

    #[cfg(windows)]
    #[test]
    fn j3_turn_failed_event_classifies_auth_required() {
        let error = run_windows_answer_fixture(
            "fake-codex-j3-auth",
            "echo {\"type\":\"turn.failed\",\"error\":{\"message\":\"login required private-detail\"}}\r\n\
             exit /b 1",
            None,
        )
        .unwrap_err();

        assert_eq!(error, "CODEX_JSONL_TURN_FAILED: auth_required");
        assert!(!error.contains("private-detail"));
    }

    #[cfg(windows)]
    #[test]
    fn j4_top_level_error_event_classifies_context_too_large() {
        let error = run_windows_answer_fixture(
            "fake-codex-j4-context",
            "echo {\"type\":\"error\",\"message\":\"maximum context length exceeded private-detail\"}\r\n\
             exit /b 1",
            None,
        )
        .unwrap_err();

        assert_eq!(error, "CODEX_JSONL_ERROR: context_too_large");
        assert!(!error.contains("private-detail"));
    }

    #[cfg(windows)]
    #[test]
    fn j5_nonzero_exit_without_terminal_event_remains_generic() {
        let error = run_windows_answer_fixture(
            "fake-codex-j5-generic-exit",
            "echo {\"type\":\"thread.started\",\"thread_id\":\"x\"}\r\nexit /b 9",
            None,
        )
        .unwrap_err();

        assert!(error.starts_with("CODEX_EXIT_ERROR"));
    }

    #[cfg(windows)]
    #[test]
    fn j6_item_error_warning_does_not_override_completed_turn() {
        let result = run_windows_answer_fixture(
            "fake-codex-j6-item-warning",
            "echo {\"type\":\"item.completed\",\"item\":{\"type\":\"error\",\"message\":\"nonfatal warning\"}}\r\n\
             echo {\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"{\\\"ok\\\":true}\"}}\r\n\
             echo {\"type\":\"turn.completed\"}\r\n\
             exit /b 0",
            None,
        )
        .unwrap();

        assert_eq!(result.0, r#"{"ok":true}"#);
    }

    #[cfg(windows)]
    #[test]
    fn j7_fatal_event_terminates_process_without_waiting_for_child_exit() {
        let started = Instant::now();
        let error = run_windows_answer_fixture(
            "fake-codex-j7-fast-fatal",
            "echo {\"type\":\"turn.failed\",\"error\":{\"message\":\"service unavailable\"}}\r\n\
             ping 127.0.0.1 -n 5 >nul\r\n\
             exit /b 1",
            None,
        )
        .unwrap_err();

        assert_eq!(error, "CODEX_JSONL_TURN_FAILED: server_overloaded");
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "fatal JSONL must terminate promptly, elapsed={:?}",
            started.elapsed()
        );
    }

    #[test]
    fn terminal_failure_classifier_covers_fixed_categories_without_payload_leakage() {
        for (message, expected) in [
            ("Invalid response schema private-detail", "schema_rejected"),
            ("invalid_request private-detail", "bad_request"),
            (
                "maximum context length exceeded private-detail",
                "context_too_large",
            ),
            ("login required private-detail", "auth_required"),
            ("usage limit reached private-detail", "usage_limit"),
            ("rate limit exceeded private-detail", "rate_limit"),
            ("service unavailable private-detail", "server_overloaded"),
            ("websocket transport failed private-detail", "transport"),
            ("provider request timed out private-detail", "transport"),
            ("connection reset private-detail", "connection"),
            ("protocol error private-detail", "protocol"),
            ("model unavailable private-detail", "model_unavailable"),
            ("unsupported model private-detail", "unsupported_model"),
            ("request cancelled private-detail", "cancelled"),
            ("unrecognized private-detail", "unknown"),
        ] {
            let category = classify_codex_terminal_message(message);
            assert_eq!(category, expected, "message={message}");
            assert!(!category.contains("private-detail"));
        }

        let observation = parse_codex_jsonl_line(
            r#"{"type":"turn.failed","error":{"message":"login required private-detail"}}"#,
        )
        .unwrap();
        let CodexJsonlObservation::Fatal(failure) = observation else {
            panic!("expected fatal observation");
        };
        assert_eq!(failure.event_type, "turn.failed");
        assert_eq!(failure.category, "auth_required");
        assert_eq!(failure.message_sha256.len(), 64);
        assert!(!failure.stable_error().contains("private-detail"));
    }

    #[test]
    fn repository_external_raw_capture_writes_payload_only_to_explicit_artifacts() {
        let root = tempfile::tempdir().unwrap();
        let schema = tiny_output_schema();
        let mut capture = CodexRawDiagnosticCapture::new(
            root.path().join("probe-a"),
            "private prompt",
            &schema,
            "codex-cli fixture",
        );
        capture.record_stdout(r#"{"type":"turn.failed","error":{"message":"private"}}"#);
        capture.record_stderr("private stderr".to_string());
        let diagnostics = CodexExecDiagnostics {
            status: "failed".to_string(),
            terminal_event_type: "turn.failed".to_string(),
            failure_category: "schema_rejected".to_string(),
            message_sha256: "a".repeat(64),
            exit_code: 1,
            ..CodexExecDiagnostics::default()
        };

        capture.write(&diagnostics).unwrap();

        let directory = root.path().join("probe-a");
        assert!(directory.join("stdout.jsonl").is_file());
        assert!(directory.join("stderr.txt").is_file());
        let metadata = fs::read_to_string(directory.join("metadata.json")).unwrap();
        assert!(metadata.contains("promptSha256"));
        assert!(metadata.contains("schemaSha256"));
        assert!(metadata.contains("schema_rejected"));
        assert!(!metadata.contains("private prompt"));
        assert!(!metadata.contains(root.path().to_string_lossy().as_ref()));
    }

    #[test]
    fn status_dto_contains_no_authentication_secret_fields() {
        let status = CodexSubscriptionStatus {
            installed: true,
            version: "codex-cli 0.146.0".into(),
            authenticated: true,
            ready: true,
            status_label: "已使用 ChatGPT 登录".into(),
            diagnostic: "无需 API Key".into(),
            configured_model: "gpt-fixture".into(),
            configured_reasoning_effort: "xhigh".into(),
            available_models: Vec::new(),
            model_catalog_status: "detected".into(),
        };
        let json = serde_json::to_string(&status).unwrap();
        for forbidden in ["token", "cookie", "credentialPath", "apiKey"] {
            assert!(!json.contains(forbidden));
        }
    }

    #[test]
    fn jsonl_parser_accepts_delta_and_final_agent_message() {
        let mut answer = String::new();
        assert_eq!(
            apply_jsonl_line(r#"{"type":"message.delta","delta":"证据"}"#, &mut answer),
            Some("证据".into())
        );
        assert_eq!(
            apply_jsonl_line(
                r#"{"type":"item.completed","item":{"type":"agent_message","text":"证据支持 [E1]"}}"#,
                &mut answer
            ),
            Some("支持 [E1]".into())
        );
        assert_eq!(answer, "证据支持 [E1]");
    }

    #[test]
    fn jsonl_parser_ignores_reasoning_tools_and_malformed_lines() {
        let mut answer = String::new();
        assert_eq!(apply_jsonl_line("not-json", &mut answer), None);
        assert_eq!(
            apply_jsonl_line(
                r#"{"type":"item.completed","item":{"type":"reasoning","text":"secret thought"}}"#,
                &mut answer
            ),
            None
        );
        assert_eq!(answer, "");
    }

    #[test]
    fn jsonl_model_observation_is_explicit_and_secret_free() {
        let observed = |line: &str| {
            serde_json::from_str::<Value>(line)
                .ok()
                .as_ref()
                .and_then(event_model)
        };
        assert_eq!(
            observed(r#"{"type":"turn.started","turn":{"model":"gpt-fixture"}}"#),
            Some("gpt-fixture".to_string())
        );
        assert_eq!(observed(r#"{"type":"turn.started"}"#), None);
        assert_eq!(observed("not-json"), None);
    }

    #[test]
    fn version_output_is_allowlisted() {
        assert_eq!(safe_version("codex-cli 0.146.0\n"), "codex-cli 0.146.0");
        assert_eq!(safe_version("token=secret"), "Codex CLI");
    }

    #[cfg(windows)]
    #[test]
    fn windows_discovery_finds_desktop_binary_and_custom_path_shim_without_duplicates() {
        let fixture = tempfile::tempdir().unwrap();
        let local_app_data = fixture.path().join("Local App Data");
        let desktop_binary = local_app_data
            .join("OpenAI")
            .join("Codex")
            .join("bin")
            .join("release-id")
            .join("codex.exe");
        fs::create_dir_all(desktop_binary.parent().unwrap()).unwrap();
        fs::write(&desktop_binary, b"fixture").unwrap();

        let node_dir = fixture.path().join("custom node");
        fs::create_dir_all(&node_dir).unwrap();
        let node_shim = node_dir.join("codex.cmd");
        fs::write(&node_shim, b"@echo off\r\n").unwrap();
        let path_value = env::join_paths([node_dir.clone(), node_dir]).unwrap();

        let mut candidates = Vec::new();
        let mut seen = HashSet::new();
        append_codex_desktop_binaries(&mut candidates, &mut seen, &local_app_data);
        append_windows_path_candidates(&mut candidates, &mut seen, path_value.as_os_str(), false);

        assert_eq!(candidates, vec![desktop_binary, node_shim]);
    }

    #[test]
    fn exec_arguments_isolate_the_workspace_and_keep_the_prompt_on_stdin() {
        let workspace = std::path::Path::new("C:/fixture/codex-workspace");
        let schema = workspace.join("answer-schema.json");
        let args = build_exec_args(workspace, Some(&schema), " gpt-fixture ", "xhigh");
        assert_eq!(args.first().map(String::as_str), Some("-a"));
        assert_eq!(args.last().map(String::as_str), Some("-"));
        assert!(args
            .windows(2)
            .any(|pair| pair == ["--sandbox", "read-only"]));
        assert!(args
            .windows(2)
            .any(|pair| pair == ["--cd", "C:/fixture/codex-workspace"]));
        assert!(args
            .windows(2)
            .any(|pair| pair == ["--model", "gpt-fixture"]));
        assert!(args
            .windows(2)
            .any(|pair| pair == ["-c", "model_reasoning_effort=\"xhigh\""]));
        assert!(args.windows(2).any(|pair| {
            pair[0] == "--output-schema" && pair[1] == schema.to_string_lossy().as_ref()
        }));

        let isolated = args.iter().map(String::as_str).collect::<Vec<_>>();
        assert!(isolated.contains(&"--ignore-user-config"));
        assert!(isolated.contains(&"--ignore-rules"));
        assert!(!args.iter().any(|value| value.contains("question text")));
    }

    #[test]
    fn output_schema_is_written_inside_and_removed_with_temp_workspace() {
        let workspace = TempWorkspace::create().unwrap();
        let root = workspace.0.clone();
        let schema = serde_json::json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["answer"],
            "properties": {"answer": {"type": "string"}}
        });
        let path = workspace.write_output_schema(&schema).unwrap();
        assert_eq!(path.parent(), Some(root.as_path()));
        assert_eq!(
            serde_json::from_slice::<Value>(&fs::read(&path).unwrap()).unwrap(),
            schema
        );
        drop(workspace);
        assert!(!root.exists());
    }

    #[cfg(windows)]
    #[test]
    fn windows_fixture_covers_status_login_jsonl_failure_timeout_and_cancel() {
        let login_marker = env::temp_dir().join(format!("codex-login-{}.txt", Uuid::new_v4()));
        let escaped_marker = login_marker.to_string_lossy().replace('%', "%%");
        let status_body = format!(
            "if \"%~1\"==\"--version\" (echo codex-cli 9.9.9& exit /b 0)\r\n\
             if \"%~1\"==\"login\" if \"%~2\"==\"status\" (echo Logged in using ChatGPT& exit /b 0)\r\n\
             if \"%~1\"==\"login\" (echo launched>\"{escaped_marker}\"& exit /b 0)\r\n\
             exit /b 3"
        );
        let (_status_workspace, status_fixture) =
            write_windows_fixture("fake codex status", &status_body);
        let executable = status_fixture.to_string_lossy();
        let status = get_status_with(&executable);
        assert!(status.ready);
        assert_eq!(status.version, "codex-cli 9.9.9");
        start_login_with(&executable).unwrap();
        for _ in 0..40 {
            if login_marker.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(25));
        }
        assert!(login_marker.exists());
        let _ = fs::remove_file(&login_marker);

        let (_answer_workspace, answer_fixture) = write_windows_fixture(
            "fake-codex-answer",
            "echo {\"type\":\"message.delta\",\"delta\":\"fixture \"}\r\n\
             echo {\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"fixture [E1]\"}}\r\n\
             exit /b 0",
        );
        let mut streamed = String::new();
        let result = stream_answer_with(
            StreamAnswerRequest {
                executable: &answer_fixture.to_string_lossy(),
                prompt: "question text",
                output_schema: None,
                model: "",
                reasoning_effort: "",
                timeout: Duration::from_secs(3),
                cancelled: &AtomicBool::new(false),
            },
            |token| {
                streamed.push_str(token);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(result.0, "fixture [E1]");
        assert_eq!(result.1, "provider-default-unreported");
        assert_eq!(streamed, "fixture [E1]");

        let (_active_workspace, active_fixture) = write_windows_fixture(
            "fake-codex-active",
            "echo {\"type\":\"message.delta\",\"delta\":\"first \"}\r\n\
             ping 127.0.0.1 -n 2 >nul\r\n\
             echo {\"type\":\"message.delta\",\"delta\":\"second\"}\r\n\
             ping 127.0.0.1 -n 2 >nul\r\n\
             echo {\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"first second\"}}\r\n\
             exit /b 0",
        );
        let active = stream_answer_with(
            StreamAnswerRequest {
                executable: &active_fixture.to_string_lossy(),
                prompt: "question text",
                output_schema: None,
                model: "",
                reasoning_effort: "",
                timeout: Duration::from_millis(1_500),
                cancelled: &AtomicBool::new(false),
            },
            |_| Ok(()),
        )
        .unwrap();
        assert_eq!(active.0, "first second");

        let (_failure_workspace, failure_fixture) = write_windows_fixture(
            "fake-codex-failure",
            "echo Authorization: fixture-secret 1>&2\r\nexit /b 7",
        );
        let error = stream_answer_with(
            StreamAnswerRequest {
                executable: &failure_fixture.to_string_lossy(),
                prompt: "question text",
                output_schema: None,
                model: "",
                reasoning_effort: "",
                timeout: Duration::from_secs(3),
                cancelled: &AtomicBool::new(false),
            },
            |_| Ok(()),
        )
        .unwrap_err();
        assert!(error.contains("CODEX_EXIT_ERROR"));
        assert!(!error.contains("fixture-secret"));

        let schema = serde_json::json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["answer"],
            "properties": {"answer": {"type": "string"}}
        });
        let (_schema_failure_workspace, schema_failure_fixture) = write_windows_fixture(
            "fake-codex-schema-failure",
            "echo invalid output schema 1>&2\r\nexit /b 8",
        );
        let schema_error = stream_answer_with(
            StreamAnswerRequest {
                executable: &schema_failure_fixture.to_string_lossy(),
                prompt: "question text",
                output_schema: Some(&schema),
                model: "",
                reasoning_effort: "",
                timeout: Duration::from_secs(3),
                cancelled: &AtomicBool::new(false),
            },
            |_| Ok(()),
        )
        .unwrap_err();
        assert!(schema_error.starts_with("CODEX_OUTPUT_SCHEMA_REJECTED"));

        let (_hang_workspace, hang_fixture) =
            write_windows_fixture("fake-codex-hang", "ping 127.0.0.1 -n 30 >nul");
        let timeout_error = stream_answer_with(
            StreamAnswerRequest {
                executable: &hang_fixture.to_string_lossy(),
                prompt: "question text",
                output_schema: None,
                model: "",
                reasoning_effort: "",
                timeout: Duration::from_millis(120),
                cancelled: &AtomicBool::new(false),
            },
            |_| Ok(()),
        )
        .unwrap_err();
        assert!(timeout_error.starts_with("CODEX_IDLE_TIMEOUT"));

        let cancel_error = stream_answer_with(
            StreamAnswerRequest {
                executable: &hang_fixture.to_string_lossy(),
                prompt: "question text",
                output_schema: None,
                model: "",
                reasoning_effort: "",
                timeout: Duration::from_secs(3),
                cancelled: &AtomicBool::new(true),
            },
            |_| Ok(()),
        )
        .unwrap_err();
        assert!(cancel_error.starts_with("CODEX_CANCELLED"));
    }

    #[test]
    fn model_catalog_projects_only_visible_non_secret_metadata() {
        let home = tempfile::tempdir().unwrap();
        fs::write(
            home.path().join("config.toml"),
            "model = \"gpt-5.6-sol\"\nmodel_reasoning_effort = \"xhigh\"\napi_key = \"secret\"\n[profiles.private]\nmodel = \"hidden\"\n",
        )
        .unwrap();
        fs::write(
            home.path().join("models_cache.json"),
            r#"{"models":[{"slug":"gpt-5.6-sol","display_name":"GPT-5.6-Sol","default_reasoning_level":"medium","supported_reasoning_levels":[{"effort":"low"},{"effort":"medium"},{"effort":"xhigh"}],"visibility":"list"},{"slug":"hidden","display_name":"Hidden","visibility":"hide"}]}"#,
        )
        .unwrap();

        let (model, effort, available, status) = load_model_catalog(Some(home.path()));
        assert_eq!(model, "gpt-5.6-sol");
        assert_eq!(effort, "xhigh");
        assert_eq!(status, "detected");
        assert_eq!(available.len(), 1);
        assert_eq!(available[0].id, "gpt-5.6-sol");
        let serialized = serde_json::to_string(&available).unwrap();
        assert!(!serialized.contains("secret"));
        assert!(!serialized.contains("api_key"));

        let selection_status = CodexSubscriptionStatus {
            installed: true,
            version: String::new(),
            authenticated: true,
            ready: true,
            status_label: String::new(),
            diagnostic: String::new(),
            configured_model: model,
            configured_reasoning_effort: effort,
            available_models: available,
            model_catalog_status: status,
        };
        assert_eq!(
            resolve_model_selection("", "ultra", &selection_status),
            ("gpt-5.6-sol".to_string(), "medium".to_string())
        );
        assert_eq!(
            resolve_model_selection("gpt-5.6-sol", "xhigh", &selection_status),
            ("gpt-5.6-sol".to_string(), "xhigh".to_string())
        );

        let uncached_status = CodexSubscriptionStatus {
            available_models: Vec::new(),
            model_catalog_status: "missing".to_string(),
            ..selection_status
        };
        assert_eq!(
            resolve_model_selection("gpt-5.6-fixture", "none", &uncached_status),
            ("gpt-5.6-fixture".to_string(), "none".to_string())
        );
    }
}
