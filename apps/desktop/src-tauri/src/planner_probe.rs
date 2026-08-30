use crate::codex_subscription::{run_codex_structured_probe, CodexStructuredProbeOutcome};
use crate::qa::{
    self, parse_query_plan, query_plan_prompt, query_plan_schema, QueryPlan,
    QueryPlanningCandidate, QueryPlanningInput, ResearchQueryContext,
    DEFAULT_CONTEXT_WINDOW_TOKENS,
};
use rusqlite::Connection;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use uuid::Uuid;

const PROBE_SCHEMA_VERSION: &str = "qa-codex-planner-probe-v1";
const DIAGNOSTIC_ENV: &str = "QA_CODEX_EXEC_DIAGNOSTIC_DIR";
const REAL_CASE_ID: &str = "real-research-improvement";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannerProbeReport {
    schema_version: String,
    probe_id: String,
    status: String,
    terminal_event_type: String,
    failure_category: String,
    message_sha256: String,
    exit_code: i32,
    latency_ms: u64,
    last_jsonl_event_type: String,
    turn_completed_seen: bool,
    agent_message_seen: bool,
    jsonl_event_count: usize,
    item_warning_count: usize,
    stderr_non_empty: bool,
    executable_source_type: String,
    executable_version: String,
    provider: String,
    model: String,
    reasoning_effort: String,
    prompt_sha256: String,
    schema_sha256: String,
    prompt_estimated_tokens: u32,
    schema_bytes: usize,
    baseline_candidate_count_before: usize,
    baseline_candidate_count_after: usize,
    baseline_excerpt_chars_before: usize,
    baseline_excerpt_chars_after: usize,
    contract_valid: bool,
    error_code: String,
}

struct ProbeWorkspace {
    root: PathBuf,
    database: PathBuf,
}

struct ProbeDefinition {
    probe_id: String,
    prompt: String,
    schema: Value,
    resolved_question: String,
    baseline_candidate_count: usize,
    baseline_excerpt_chars: usize,
}

impl ProbeWorkspace {
    fn create() -> Result<Self, String> {
        let root = env::temp_dir().join(format!("qa-planner-probe-{}", Uuid::new_v4()));
        fs::create_dir(&root).map_err(|_| "QA_PLANNER_PROBE_TEMP_CREATE_FAILED".to_string())?;
        Ok(Self {
            database: root.join("probe.sqlite"),
            root,
        })
    }
}

impl Drop for ProbeWorkspace {
    fn drop(&mut self) {
        for name in [
            "probe.sqlite-wal",
            "probe.sqlite-shm",
            "probe.sqlite-journal",
            "probe.sqlite",
        ] {
            let _ = fs::remove_file(self.root.join(name));
        }
        let _ = fs::remove_dir(&self.root);
    }
}

fn sha256(value: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(value.as_ref()))
}

fn safe_error_code(error: &str) -> String {
    qa::trace::error_code(error)
}

fn validate_diagnostic_root(repository: &Path, requested: &Path) -> Result<PathBuf, String> {
    if !requested.is_absolute() {
        return Err("QA_CODEX_EXEC_DIAGNOSTIC_DIR_INVALID: absolute_required".to_string());
    }
    fs::create_dir_all(requested)
        .map_err(|_| "QA_CODEX_EXEC_DIAGNOSTIC_DIR_INVALID: create_failed".to_string())?;
    let repository = fs::canonicalize(repository)
        .map_err(|_| "QA_CODEX_EXEC_DIAGNOSTIC_DIR_INVALID: repository".to_string())?;
    let requested = fs::canonicalize(requested)
        .map_err(|_| "QA_CODEX_EXEC_DIAGNOSTIC_DIR_INVALID: canonicalize".to_string())?;
    if requested.starts_with(&repository) {
        return Err("QA_CODEX_EXEC_DIAGNOSTIC_DIR_INVALID: inside_repository".to_string());
    }
    Ok(requested)
}

fn configured_diagnostic_root(repository: &Path) -> Result<Option<PathBuf>, String> {
    let Some(raw) = env::var_os(DIAGNOSTIC_ENV) else {
        return Ok(None);
    };
    if raw.is_empty() {
        return Ok(None);
    }
    validate_diagnostic_root(repository, Path::new(&raw)).map(Some)
}

fn minimal_planner_input() -> QueryPlanningInput {
    QueryPlanningInput {
        resolved_question: "wireless charging scheduling literature".to_string(),
        research_context: ResearchQueryContext::default(),
        baseline_candidates: vec![QueryPlanningCandidate {
            kind: "wiki".to_string(),
            page_type: "source".to_string(),
            title: "Wireless charging scheduling".to_string(),
            excerpt: "A bounded development fixture candidate.".to_string(),
        }],
    }
}

fn real_planner_input(repository: &Path) -> Result<QueryPlanningInput, String> {
    let suite =
        qa::real_e2e::load_suite(&repository.join("evals/qa_real_generator_e2e_cases.json"))?;
    let case = suite
        .cases
        .into_iter()
        .find(|case| case.id == REAL_CASE_ID)
        .ok_or_else(|| "QA_PLANNER_PROBE_CASE_MISSING".to_string())?;
    let question = case
        .turns
        .first()
        .map(|turn| turn.question.clone())
        .ok_or_else(|| "QA_PLANNER_PROBE_CASE_EMPTY".to_string())?;
    let workspace = ProbeWorkspace::create()?;
    let mut connection = Connection::open(&workspace.database)
        .map_err(|_| "QA_PLANNER_PROBE_DATABASE_OPEN_FAILED".to_string())?;
    crate::db_schema(&connection)?;
    let _ = crate::rebuild_connection(&mut connection, repository)?;
    let mut captured = None;
    let mut planner = |input: &QueryPlanningInput| {
        captured = Some(input.clone());
        Ok(QueryPlan::fallback(&input.resolved_question))
    };
    qa::prepare_question_with_history_budget_and_planner(
        &connection,
        repository,
        &question,
        14,
        "p1-3f-probe-c-input",
        Vec::new(),
        Some(&AtomicBool::new(false)),
        DEFAULT_CONTEXT_WINDOW_TOKENS,
        qa::LunaSettings::default().max_output_tokens,
        Some(&mut planner),
    )?;
    captured.ok_or_else(|| "QA_PLANNER_PROBE_INPUT_NOT_CAPTURED".to_string())
}

fn probe_definition(repository: &Path, probe_id: &str) -> Result<ProbeDefinition, String> {
    match probe_id {
        "a" => Ok(ProbeDefinition {
            probe_id: probe_id.to_string(),
            prompt: "Return the required JSON only.".to_string(),
            schema: json!({
                "type": "object",
                "additionalProperties": false,
                "required": ["ok"],
                "properties": {"ok": {"type": "boolean"}}
            }),
            resolved_question: String::new(),
            baseline_candidate_count: 0,
            baseline_excerpt_chars: 0,
        }),
        "b" => {
            let input = minimal_planner_input();
            let count = input.baseline_candidates.len();
            let excerpt_chars = input
                .baseline_candidates
                .iter()
                .map(|candidate| candidate.excerpt.chars().count())
                .sum();
            Ok(ProbeDefinition {
                probe_id: probe_id.to_string(),
                prompt: query_plan_prompt(&input),
                schema: query_plan_schema(),
                resolved_question: input.resolved_question,
                baseline_candidate_count: count,
                baseline_excerpt_chars: excerpt_chars,
            })
        }
        "c" => {
            let input = real_planner_input(repository)?;
            let count = input.baseline_candidates.len();
            let excerpt_chars = input
                .baseline_candidates
                .iter()
                .map(|candidate| candidate.excerpt.chars().count())
                .sum();
            Ok(ProbeDefinition {
                probe_id: probe_id.to_string(),
                prompt: query_plan_prompt(&input),
                schema: query_plan_schema(),
                resolved_question: input.resolved_question,
                baseline_candidate_count: count,
                baseline_excerpt_chars: excerpt_chars,
            })
        }
        _ => Err("QA_PLANNER_PROBE_ID_INVALID".to_string()),
    }
}

fn validate_probe_output(probe_id: &str, output: &str, question: &str) -> Result<(), String> {
    if probe_id == "a" {
        let value = serde_json::from_str::<Value>(output)
            .map_err(|_| "QA_PLANNER_PROBE_OUTPUT_INVALID: json".to_string())?;
        if value.get("ok").and_then(Value::as_bool).is_none() {
            return Err("QA_PLANNER_PROBE_OUTPUT_INVALID: control_schema".to_string());
        }
        Ok(())
    } else {
        parse_query_plan(output, question)
            .map(|_| ())
            .map_err(|_| "QA_PLANNER_PROBE_OUTPUT_INVALID: retrieval_contract".to_string())
    }
}

fn project_report(
    definition: &ProbeDefinition,
    model: &str,
    effort: &str,
    outcome: CodexStructuredProbeOutcome,
) -> PlannerProbeReport {
    let validation = outcome.output.as_deref().map(|output| {
        validate_probe_output(&definition.probe_id, output, &definition.resolved_question)
    });
    let contract_valid = matches!(validation, Some(Ok(())));
    let mut status = outcome.diagnostics.status.clone();
    let mut failure_category = outcome.diagnostics.failure_category.clone();
    let mut error_code = if outcome.error.is_empty() {
        String::new()
    } else {
        safe_error_code(&outcome.error)
    };
    if matches!(validation, Some(Err(_))) {
        status = "failed".to_string();
        failure_category = "contract_validation_invalid".to_string();
        error_code = "qa_planner_probe_output_invalid".to_string();
    }
    PlannerProbeReport {
        schema_version: PROBE_SCHEMA_VERSION.to_string(),
        probe_id: definition.probe_id.clone(),
        status,
        terminal_event_type: outcome.diagnostics.terminal_event_type,
        failure_category,
        message_sha256: outcome.diagnostics.message_sha256,
        exit_code: outcome.diagnostics.exit_code,
        latency_ms: outcome.latency_ms,
        last_jsonl_event_type: outcome.diagnostics.last_jsonl_event_type,
        turn_completed_seen: outcome.diagnostics.turn_completed_seen,
        agent_message_seen: outcome.diagnostics.agent_message_seen,
        jsonl_event_count: outcome.diagnostics.jsonl_event_count,
        item_warning_count: outcome.diagnostics.item_warning_count,
        stderr_non_empty: outcome.diagnostics.stderr_non_empty,
        executable_source_type: outcome.executable_source_type,
        executable_version: outcome.executable_version,
        provider: "codex-subscription".to_string(),
        model: model.to_string(),
        reasoning_effort: effort.to_string(),
        prompt_sha256: sha256(&definition.prompt),
        schema_sha256: sha256(serde_json::to_vec(&definition.schema).unwrap_or_default()),
        prompt_estimated_tokens: qa::estimate_tokens(&definition.prompt),
        schema_bytes: serde_json::to_vec(&definition.schema)
            .unwrap_or_default()
            .len(),
        baseline_candidate_count_before: definition.baseline_candidate_count,
        baseline_candidate_count_after: definition.baseline_candidate_count,
        baseline_excerpt_chars_before: definition.baseline_excerpt_chars,
        baseline_excerpt_chars_after: definition.baseline_excerpt_chars,
        contract_valid,
        error_code,
    }
}

fn atomic_write_report(output: &Path, report: &PlannerProbeReport) -> Result<(), String> {
    if output.exists() {
        return Err("QA_PLANNER_PROBE_OUTPUT_EXISTS".to_string());
    }
    let parent = output
        .parent()
        .ok_or_else(|| "QA_PLANNER_PROBE_OUTPUT_INVALID".to_string())?;
    fs::create_dir_all(parent).map_err(|_| "QA_PLANNER_PROBE_OUTPUT_CREATE_FAILED".to_string())?;
    let part = output.with_extension(format!(
        "{}.part",
        output
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("json")
    ));
    if part.exists() {
        return Err("QA_PLANNER_PROBE_PART_EXISTS".to_string());
    }
    let bytes = serde_json::to_vec_pretty(report)
        .map_err(|_| "QA_PLANNER_PROBE_REPORT_SERIALIZE_FAILED".to_string())?;
    fs::write(&part, bytes).map_err(|_| "QA_PLANNER_PROBE_REPORT_WRITE_FAILED".to_string())?;
    fs::rename(&part, output).map_err(|_| "QA_PLANNER_PROBE_REPORT_RENAME_FAILED".to_string())
}

fn probe_started_event(definition: &ProbeDefinition, model: &str) -> qa::trace::QaTraceEvent {
    let operation_id = format!("p1-3f-probe-{}", definition.probe_id);
    let mut event = qa::trace::QaTraceEvent::new(
        "qa_planner_probe_started",
        "planner_provider_probe",
        "started",
        &operation_id,
    );
    event.case_id = format!("probe-{}", definition.probe_id);
    event.provider = "codex-subscription".to_string();
    event.model = model.to_string();
    event.baseline_candidate_count = Some(definition.baseline_candidate_count);
    event
}

fn probe_finished_event(
    definition: &ProbeDefinition,
    model: &str,
    report: &PlannerProbeReport,
    passed: bool,
) -> qa::trace::QaTraceEvent {
    let operation_id = format!("p1-3f-probe-{}", definition.probe_id);
    let mut event = qa::trace::QaTraceEvent::new(
        if passed {
            "qa_planner_probe_completed"
        } else {
            "qa_planner_probe_failed"
        },
        "planner_provider_probe",
        if passed { "succeeded" } else { "failed" },
        &operation_id,
    );
    event.case_id = format!("probe-{}", definition.probe_id);
    event.provider = "codex-subscription".to_string();
    event.model = model.to_string();
    event.baseline_candidate_count = Some(definition.baseline_candidate_count);
    event.duration_ms = Some(report.latency_ms);
    event.error_code = report.failure_category.clone();
    event
}

pub fn run_planner_probe_files(
    repository: &Path,
    probe_id: &str,
    output: &Path,
    model: &str,
    effort: &str,
) -> Result<bool, String> {
    let repository = fs::canonicalize(repository)
        .map_err(|_| "QA_PLANNER_PROBE_REPOSITORY_INVALID".to_string())?;
    qa::trace::configure_cli_file(repository.join("apps/desktop/logs/qa-real-e2e.jsonl"))?;
    let definition = probe_definition(&repository, probe_id)?;
    let diagnostic_root = configured_diagnostic_root(&repository)?;
    let diagnostic_directory = diagnostic_root
        .as_ref()
        .map(|root| root.join(format!("probe-{probe_id}")));
    qa::trace::emit(&probe_started_event(&definition, model));

    let outcome = run_codex_structured_probe(
        &definition.prompt,
        &definition.schema,
        model,
        effort,
        Duration::from_secs(60),
        diagnostic_directory.as_deref(),
    )?;
    let report = project_report(&definition, model, effort, outcome);
    let passed = report.status == "succeeded" && report.contract_valid;
    qa::trace::emit(&probe_finished_event(&definition, model, &report, passed));
    atomic_write_report(output, &report)?;
    Ok(passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_directory_must_be_absolute_and_outside_repository() {
        let repository = tempfile::tempdir().unwrap();
        assert!(validate_diagnostic_root(repository.path(), Path::new("relative")).is_err());
        assert!(
            validate_diagnostic_root(repository.path(), &repository.path().join("diag")).is_err()
        );
        let external = tempfile::tempdir().unwrap();
        assert!(validate_diagnostic_root(repository.path(), external.path()).is_ok());
    }

    #[test]
    fn safe_probe_report_contains_no_prompt_output_or_path() {
        let outcome = CodexStructuredProbeOutcome {
            output: Some(r#"{"ok":true}"#.to_string()),
            error: String::new(),
            diagnostics: crate::codex_subscription::CodexExecDiagnostics {
                status: "succeeded".to_string(),
                exit_code: 0,
                agent_message_seen: true,
                turn_completed_seen: true,
                last_jsonl_event_type: "turn.completed".to_string(),
                jsonl_event_count: 3,
                ..crate::codex_subscription::CodexExecDiagnostics::default()
            },
            latency_ms: 12,
            executable_source_type: "npm-wrapper".to_string(),
            executable_version: "codex-cli fixture".to_string(),
        };
        let schema = json!({
            "type":"object",
            "additionalProperties":false,
            "required":["ok"],
            "properties":{"ok":{"type":"boolean"}}
        });
        let definition = ProbeDefinition {
            probe_id: "a".to_string(),
            prompt: "private prompt".to_string(),
            schema,
            resolved_question: String::new(),
            baseline_candidate_count: 0,
            baseline_excerpt_chars: 0,
        };
        let report = project_report(&definition, "gpt-fixture", "low", outcome);
        let serialized = serde_json::to_string(&report).unwrap().to_ascii_lowercase();
        assert!(report.contract_valid);
        assert_eq!(report.status, "succeeded");
        for forbidden in [
            "private prompt",
            "rawoutput",
            "stdout",
            "stderr.txt",
            "repositorypath",
            "diagnosticpath",
        ] {
            assert!(!serialized.contains(forbidden), "forbidden={forbidden}");
        }
        let started = probe_started_event(&definition, "gpt-fixture");
        let completed = probe_finished_event(&definition, "gpt-fixture", &report, true);
        let failed = probe_finished_event(&definition, "gpt-fixture", &report, false);
        assert_eq!(started.event, "qa_planner_probe_started");
        assert_eq!(completed.event, "qa_planner_probe_completed");
        assert_eq!(failed.event, "qa_planner_probe_failed");
        assert_eq!(started.request_id_hash, completed.request_id_hash);
        assert_eq!(started.request_id_hash, failed.request_id_hash);
    }

    #[test]
    fn probe_a_and_b_definitions_are_bounded_and_structured() {
        let repository = Path::new(".");
        let definition_a = probe_definition(repository, "a").unwrap();
        assert_eq!(definition_a.prompt, "Return the required JSON only.");
        assert_eq!(definition_a.schema["required"], json!(["ok"]));
        assert!(definition_a.resolved_question.is_empty());
        assert_eq!(definition_a.baseline_candidate_count, 0);

        let definition_b = probe_definition(repository, "b").unwrap();
        assert!(definition_b
            .prompt
            .contains("wireless charging scheduling literature"));
        assert_eq!(definition_b.schema["type"], "object");
        assert_eq!(
            definition_b.resolved_question,
            "wireless charging scheduling literature"
        );
        assert_eq!(definition_b.baseline_candidate_count, 1);
        assert!(definition_b.baseline_excerpt_chars > 0);
    }
}
