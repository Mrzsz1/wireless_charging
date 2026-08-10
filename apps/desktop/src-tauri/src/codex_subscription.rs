use crate::process_support::{configure_background_command, terminate_process_tree};
use serde::Serialize;
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
pub struct CodexSubscriptionStatus {
    pub installed: bool,
    pub version: String,
    pub authenticated: bool,
    pub ready: bool,
    pub status_label: String,
    pub diagnostic: String,
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
        }
    }
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

fn executable() -> String {
    if let Some(explicit) = explicit_executable() {
        return explicit;
    }
    discovered_executables()
        .into_iter()
        .find_map(|candidate| {
            let executable = candidate.to_string_lossy().into_owned();
            match run_fixed_with(&executable, &["--version"], STATUS_TIMEOUT) {
                Ok((true, stdout, _)) if safe_version(&stdout) != "Codex CLI" => Some(executable),
                _ => None,
            }
        })
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

fn build_exec_args(workspace: &std::path::Path, model: &str) -> Vec<String> {
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
    args.push("-".to_string());
    args
}

fn stream_answer_with<F>(
    executable: &str,
    prompt: &str,
    model: &str,
    timeout: Duration,
    cancelled: &AtomicBool,
    mut on_token: F,
) -> Result<(String, String), String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    let workspace = TempWorkspace::create()?;
    let mut command = Command::new(executable);
    configure_background_command(&mut command);
    command
        .args(build_exec_args(&workspace.0, model))
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
    let mut answer = String::new();
    let status = loop {
        if cancelled.load(Ordering::SeqCst) {
            terminate_process_tree(&mut child);
            let _ = stderr_reader.join();
            return Err("CODEX_CANCELLED: 用户停止了生成".to_string());
        }
        if started.elapsed() >= timeout {
            terminate_process_tree(&mut child);
            let _ = stderr_reader.join();
            return Err("CODEX_TIMEOUT: 订阅回答超时".to_string());
        }
        match receiver.recv_timeout(POLL_INTERVAL) {
            Ok(OutputLine::Line(line)) => {
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
    let _stderr = stderr_reader.join().unwrap_or_default();
    let answer = answer.trim().to_string();
    if !status.success() {
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
        if model.trim().is_empty() {
            "codex-default".to_string()
        } else {
            model.trim().to_string()
        },
    ))
}

pub fn stream_answer<F>(
    prompt: &str,
    model: &str,
    timeout: Duration,
    cancelled: &AtomicBool,
    on_token: F,
) -> Result<(String, String), String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    stream_answer_with(&executable(), prompt, model, timeout, cancelled, on_token)
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
        let args = build_exec_args(workspace, " gpt-fixture ");
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
        assert!(!args.iter().any(|value| value.contains("question text")));
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
            &answer_fixture.to_string_lossy(),
            "question text",
            "",
            Duration::from_secs(3),
            &AtomicBool::new(false),
            |token| {
                streamed.push_str(token);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(result.0, "fixture [E1]");
        assert_eq!(streamed, "fixture [E1]");

        let (_failure_workspace, failure_fixture) = write_windows_fixture(
            "fake-codex-failure",
            "echo Authorization: fixture-secret 1>&2\r\nexit /b 7",
        );
        let error = stream_answer_with(
            &failure_fixture.to_string_lossy(),
            "question text",
            "",
            Duration::from_secs(3),
            &AtomicBool::new(false),
            |_| Ok(()),
        )
        .unwrap_err();
        assert!(error.contains("CODEX_EXIT_ERROR"));
        assert!(!error.contains("fixture-secret"));

        let (_hang_workspace, hang_fixture) =
            write_windows_fixture("fake-codex-hang", "ping 127.0.0.1 -n 30 >nul");
        let timeout_error = stream_answer_with(
            &hang_fixture.to_string_lossy(),
            "question text",
            "",
            Duration::from_millis(120),
            &AtomicBool::new(false),
            |_| Ok(()),
        )
        .unwrap_err();
        assert!(timeout_error.starts_with("CODEX_TIMEOUT"));

        let cancel_error = stream_answer_with(
            &hang_fixture.to_string_lossy(),
            "question text",
            "",
            Duration::from_secs(3),
            &AtomicBool::new(true),
            |_| Ok(()),
        )
        .unwrap_err();
        assert!(cancel_error.starts_with("CODEX_CANCELLED"));
    }
}
