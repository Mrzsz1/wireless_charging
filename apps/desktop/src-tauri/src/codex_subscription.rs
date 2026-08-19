use crate::process_support::{configure_background_command, terminate_process_tree};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

fn event_model(line: &str) -> Option<String> {
    let value = serde_json::from_str::<Value>(line).ok()?;
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

fn apply_jsonl_line(line: &str, answer: &mut String) -> Option<String> {
    let value = serde_json::from_str::<Value>(line).ok()?;
    let kind = value
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if let Some(delta) = event_delta(&value).filter(|_| kind.contains("delta")) {
        answer.push_str(delta);
        return Some(delta.to_string());
    }
    let item_is_agent = value
        .pointer("/item/type")
        .and_then(Value::as_str)
        .map(|value| value == "agent_message")
        .unwrap_or(false);
    if !item_is_agent && !kind.contains("message") {
        return None;
    }
    let text = event_text(&value)?;
    if text.is_empty() {
        return None;
    }
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

fn stream_answer_with<F>(
    request: StreamAnswerRequest<'_>,
    mut on_token: F,
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
    thread::spawn(move || {
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
    });
    let stderr_reader = thread::spawn(move || {
        let mut text = String::new();
        let _ = BufReader::new(stderr)
            .take(16 * 1024)
            .read_to_string(&mut text);
        text
    });

    let started = Instant::now();
    let mut last_activity = started;
    let idle_timeout = timeout;
    let hard_timeout = timeout
        .saturating_mul(4)
        .max(Duration::from_secs(600))
        .min(Duration::from_secs(1800));
    let mut answer = String::new();
    let mut resolved_model = model.trim().to_string();
    let status = loop {
        if cancelled.load(Ordering::SeqCst) {
            terminate_process_tree(&mut child);
            let _ = stderr_reader.join();
            return Err("CODEX_CANCELLED: 用户停止了生成".to_string());
        }
        if started.elapsed() >= hard_timeout {
            terminate_process_tree(&mut child);
            let _ = stderr_reader.join();
            return Err("CODEX_TOTAL_TIMEOUT: 订阅回答超过总时限".to_string());
        }
        if last_activity.elapsed() >= idle_timeout {
            terminate_process_tree(&mut child);
            let _ = stderr_reader.join();
            return Err("CODEX_IDLE_TIMEOUT: 订阅回答长时间无活动".to_string());
        }
        match receiver.recv_timeout(POLL_INTERVAL) {
            Ok(OutputLine::Line(line)) => {
                if serde_json::from_str::<Value>(&line).is_ok() {
                    last_activity = Instant::now();
                }
                if let Some(observed) = event_model(&line) {
                    resolved_model = observed;
                }
                if let Some(delta) = apply_jsonl_line(&line, &mut answer) {
                    if let Err(error) = on_token(&delta) {
                        terminate_process_tree(&mut child);
                        let _ = stderr_reader.join();
                        return Err(error);
                    }
                }
            }
            Ok(OutputLine::Closed) | Err(mpsc::RecvTimeoutError::Disconnected) => {}
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        if let Some(status) = child
            .try_wait()
            .map_err(|_| "CODEX_WAIT_FAILED".to_string())?
        {
            for output in receiver.try_iter() {
                if let OutputLine::Line(line) = output {
                    if let Some(observed) = event_model(&line) {
                        resolved_model = observed;
                    }
                    if let Some(delta) = apply_jsonl_line(&line, &mut answer) {
                        if let Err(error) = on_token(&delta) {
                            let _ = stderr_reader.join();
                            return Err(error);
                        }
                    }
                }
            }
            break status;
        }
    };
    let stderr_output = stderr_reader.join().unwrap_or_default();
    let answer = answer.trim().to_string();
    if !status.success() {
        if output_schema.is_some() && stderr_output.to_ascii_lowercase().contains("schema") {
            return Err(format!(
                "CODEX_OUTPUT_SCHEMA_REJECTED: Codex CLI 未接受回答结构约束（退出码 {}）",
                status.code().unwrap_or(-1)
            ));
        }
        return Err(format!(
            "CODEX_EXIT_ERROR: Codex CLI 退出码 {}",
            status.code().unwrap_or(-1)
        ));
    }
    if answer.is_empty() {
        return Err("CODEX_RESPONSE_ERROR: 未收到回答文本".to_string());
    }
    Ok((
        answer,
        if resolved_model.is_empty() {
            "provider-default-unreported".to_string()
        } else {
            resolved_model
        },
    ))
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
        assert_eq!(
            event_model(r#"{"type":"turn.started","turn":{"model":"gpt-fixture"}}"#),
            Some("gpt-fixture".to_string())
        );
        assert_eq!(event_model(r#"{"type":"turn.started"}"#), None);
        assert_eq!(event_model("not-json"), None);
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
