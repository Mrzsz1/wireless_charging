use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

const TRACE_SCHEMA_VERSION: &str = "qa-trace-v1";
const MAX_CLI_LOG_BYTES: u64 = 5 * 1024 * 1024;

static CLI_TRACE_PATH: OnceLock<Mutex<Option<PathBuf>>> = OnceLock::new();

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QaTraceEvent {
    pub schema_version: String,
    pub timestamp_unix_ms: u128,
    pub event: String,
    pub stage: String,
    pub status: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub request_id_hash: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub case_id: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub execution_mode: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub provider: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_claim_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contradicted_claim_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_verifiable_claim_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repaired_claim_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persisted: Option<bool>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub error_code: String,
}

impl QaTraceEvent {
    pub fn new(event: &str, stage: &str, status: &str, request_id: &str) -> Self {
        Self {
            schema_version: TRACE_SCHEMA_VERSION.to_string(),
            timestamp_unix_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
            event: event.to_string(),
            stage: stage.to_string(),
            status: status.to_string(),
            request_id_hash: request_id_hash(request_id),
            ..Self::default()
        }
    }
}

pub fn request_id_hash(request_id: &str) -> String {
    if request_id.trim().is_empty() {
        return String::new();
    }
    format!("{:x}", Sha256::digest(request_id.as_bytes()))[..16].to_string()
}

pub fn error_code(error: &str) -> String {
    let raw = error.split(':').next().unwrap_or("qa_failure").trim();
    let code = raw
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || *character == '_')
        .take(96)
        .collect::<String>()
        .to_ascii_lowercase();
    if code.is_empty() {
        "qa_failure".to_string()
    } else {
        code
    }
}

pub fn configure_cli_file(path: PathBuf) -> Result<(), String> {
    if !path.is_absolute() || path.file_name().is_none() {
        return Err("QA_TRACE_CONFIGURATION_INVALID".to_string());
    }
    let parent = path
        .parent()
        .ok_or_else(|| "QA_TRACE_CONFIGURATION_INVALID".to_string())?;
    fs::create_dir_all(parent).map_err(|_| "QA_TRACE_DIRECTORY_CREATE_FAILED".to_string())?;
    let slot = CLI_TRACE_PATH.get_or_init(|| Mutex::new(None));
    *slot
        .lock()
        .map_err(|_| "QA_TRACE_CONFIGURATION_LOCKED".to_string())? = Some(path);
    Ok(())
}

pub fn emit(event: &QaTraceEvent) {
    let Ok(serialized) = serde_json::to_string(event) else {
        log::warn!(target: "qa_trace", "event=qa_trace_serialize_failed");
        return;
    };
    log::info!(target: "qa_trace", "{serialized}");
    let Some(slot) = CLI_TRACE_PATH.get() else {
        return;
    };
    let Ok(guard) = slot.lock() else {
        log::warn!(target: "qa_trace", "event=qa_trace_lock_failed");
        return;
    };
    let Some(path) = guard.as_deref() else {
        return;
    };
    if append_to_path(path, serialized.as_bytes()).is_err() {
        log::warn!(target: "qa_trace", "event=qa_trace_write_failed");
    }
}

fn append_to_path(path: &Path, serialized: &[u8]) -> Result<(), String> {
    if path
        .metadata()
        .map(|metadata| metadata.len() >= MAX_CLI_LOG_BYTES)
        .unwrap_or(false)
    {
        fs::write(path, []).map_err(|_| "QA_TRACE_ROTATION_FAILED".to_string())?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|_| "QA_TRACE_OPEN_FAILED".to_string())?;
    file.write_all(serialized)
        .and_then(|_| file.write_all(b"\n"))
        .map_err(|_| "QA_TRACE_WRITE_FAILED".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn trace_hashes_request_identity_and_exposes_only_safe_metadata() {
        let request_id = "aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee";
        let mut event = QaTraceEvent::new(
            "qa_generate_completed",
            "generator",
            "succeeded",
            request_id,
        );
        event.provider = "codex-subscription".to_string();
        event.model = "gpt-fixture".to_string();
        event.evidence_count = Some(2);
        let serialized = serde_json::to_string(&event).unwrap();
        assert!(!serialized.contains(request_id));
        for forbidden in [
            "question",
            "answer",
            "prompt",
            "claimText",
            "snippet",
            "repositoryPath",
            "tempPath",
            "credential",
            "providerPayload",
            "chainOfThought",
        ] {
            assert!(!serialized.contains(forbidden), "forbidden={forbidden}");
        }
    }

    #[test]
    fn cli_trace_appends_json_lines_inside_configured_file() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("qa-real-e2e.jsonl");
        let event = QaTraceEvent::new("qa_e2e_started", "runner", "started", "request-id");
        let serialized = serde_json::to_vec(&event).unwrap();
        append_to_path(&path, &serialized).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert_eq!(content.lines().count(), 1);
        assert!(content.contains("qa_e2e_started"));
    }

    #[test]
    fn error_code_drops_message_details() {
        assert_eq!(
            error_code("CITATION_VALIDATION_FAILED: sensitive details"),
            "citation_validation_failed"
        );
    }
}
