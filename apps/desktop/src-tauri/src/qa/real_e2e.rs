use super::*;
use crate::codex_subscription;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use uuid::Uuid;

const CASE_SCHEMA_VERSION: &str = "qa-real-generator-e2e-cases-v1";
const REPORT_SCHEMA_VERSION: &str = "qa-real-generator-e2e-report-v1";
const CORE_VERSION: &str = "qa-production-core-v1";
const EXPECTED_CATEGORIES: [&str; 5] = [
    "direct",
    "research",
    "exploratory",
    "multi_turn",
    "zero_evidence",
];

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RealE2eTurn {
    pub question: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpectedResearchState {
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub excluded_methods: Vec<String>,
    #[serde(default)]
    pub integer_parameters: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RealE2eCase {
    pub id: String,
    pub category: String,
    pub source: String,
    pub turns: Vec<RealE2eTurn>,
    #[serde(default)]
    pub expected_execution_mode: String,
    #[serde(default)]
    pub zero_evidence_expected: bool,
    #[serde(default)]
    pub expected_research_state: ExpectedResearchState,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RealE2eSuite {
    pub schema_version: String,
    pub dataset_role: String,
    pub version: String,
    pub case_count: usize,
    pub cases_sha256: String,
    pub cases: Vec<RealE2eCase>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RealE2eCaseResult {
    pub id: String,
    pub category: String,
    pub passed: bool,
    pub turn_count: usize,
    pub execution_mode: String,
    pub provider: String,
    pub model: String,
    pub evidence_count: usize,
    pub citation_valid: bool,
    pub semantic_status: String,
    pub semantic_fallback_reason: String,
    pub generator_stage_observed: bool,
    pub generator_budget_rejected: bool,
    pub planner_status: String,
    pub retrieval_round_count: usize,
    pub evidence_selected_count: usize,
    pub reranker_provider: String,
    pub reranker_fallback: bool,
    pub state_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RealE2eReport {
    pub schema_version: String,
    pub core_version: String,
    pub dataset_version: String,
    pub dataset_sha256: String,
    pub case_count: usize,
    pub passed_count: usize,
    pub generator_invocation_count: usize,
    pub provider: String,
    pub models: Vec<String>,
    pub real_provider_measured: bool,
    pub generator_fallback_count: usize,
    pub generator_budget_rejection_count: usize,
    pub invalid_citation_count: usize,
    pub semantic_succeeded_count: usize,
    pub semantic_unavailable_count: usize,
    pub passed: bool,
    pub results: Vec<RealE2eCaseResult>,
}

struct TemporaryWorkspace {
    root: PathBuf,
    database: PathBuf,
}

impl TemporaryWorkspace {
    fn create() -> Result<Self, String> {
        let root = env::temp_dir().join(format!("qa-real-e2e-{}", Uuid::new_v4()));
        fs::create_dir(&root)
            .map_err(|error| format!("QA_REAL_E2E_TEMP_CREATE_FAILED: {error}"))?;
        Ok(Self {
            database: root.join("runner.sqlite"),
            root,
        })
    }
}

impl Drop for TemporaryWorkspace {
    fn drop(&mut self) {
        for name in [
            "runner.sqlite-wal",
            "runner.sqlite-shm",
            "runner.sqlite-journal",
            "runner.sqlite",
        ] {
            let _ = fs::remove_file(self.root.join(name));
        }
        let _ = fs::remove_dir(&self.root);
    }
}

#[derive(Debug, Default)]
struct TurnObservation {
    answer_non_empty: bool,
    provider: String,
    model: String,
    answer_format: String,
    manifest_schema: String,
    execution_mode: String,
    generator_stage_observed: bool,
    generator_budget_rejected: bool,
    evidence_count: usize,
    evidence_selected_count: usize,
    citation_valid: bool,
    unknown_citation_count: usize,
    semantic_status: String,
    semantic_fallback_reason: String,
    planner_status: String,
    retrieval_round_count: usize,
    reranker_provider: String,
    reranker_fallback: bool,
}

fn cases_sha256(cases: &[RealE2eCase]) -> Result<String, String> {
    let bytes =
        serde_json::to_vec(cases).map_err(|error| format!("QA_REAL_E2E_CASES_INVALID: {error}"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn load_suite(path: &Path) -> Result<RealE2eSuite, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("QA_REAL_E2E_CASES_READ_FAILED: {error}"))?;
    let suite = serde_json::from_str::<RealE2eSuite>(&raw)
        .map_err(|error| format!("QA_REAL_E2E_CASES_INVALID: {error}"))?;
    if suite.schema_version != CASE_SCHEMA_VERSION
        || suite.dataset_role != "development_regression_synthetic"
        || suite.case_count != 5
        || suite.cases.len() != 5
        || cases_sha256(&suite.cases)? != suite.cases_sha256
    {
        return Err("QA_REAL_E2E_CASES_INVALID: schema_role_count_or_hash".to_string());
    }
    let mut ids = HashSet::new();
    let mut categories = HashSet::new();
    for case in &suite.cases {
        if case.id.trim().is_empty()
            || !case.id.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
            })
            || !ids.insert(case.id.clone())
            || !matches!(
                case.source.as_str(),
                "development" | "regression" | "synthetic"
            )
            || !(1..=3).contains(&case.turns.len())
            || case
                .turns
                .iter()
                .any(|turn| turn.question.trim().chars().count() < 8)
        {
            return Err(format!("QA_REAL_E2E_CASES_INVALID: case={}", case.id));
        }
        categories.insert(case.category.as_str());
        if case.category == "multi_turn" && case.turns.len() < 2 {
            return Err("QA_REAL_E2E_CASES_INVALID: multi_turn requires history".to_string());
        }
        if case.category == "zero_evidence" && !case.zero_evidence_expected {
            return Err("QA_REAL_E2E_CASES_INVALID: zero_evidence contract missing".to_string());
        }
    }
    if EXPECTED_CATEGORIES
        .iter()
        .any(|category| !categories.contains(category))
        || categories.len() != EXPECTED_CATEGORIES.len()
    {
        return Err("QA_REAL_E2E_CASES_INVALID: category_matrix".to_string());
    }
    Ok(suite)
}

fn configure_local_models() -> Result<(), String> {
    if let Some(path) = env::var_os("QA_SEMANTIC_MODEL_CACHE_DIR").filter(|value| !value.is_empty())
    {
        return configure_semantic_cache_dir(Some(PathBuf::from(path))).map(|_| ());
    }
    #[cfg(windows)]
    if let Some(local) = env::var_os("LOCALAPPDATA") {
        let settings_path = PathBuf::from(local)
            .join("com.wirelesscharging.research-workbench")
            .join("semantic-model-settings.json");
        let stored = crate::read_stored_semantic_settings(&settings_path)?;
        if !stored.cache_dir.trim().is_empty() {
            return configure_semantic_cache_dir(Some(PathBuf::from(stored.cache_dir))).map(|_| ());
        }
    }
    configure_semantic_cache_dir(None).map(|_| ())
}

fn observation(result: &AskResult) -> TurnObservation {
    let evidence_ids = result
        .evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect::<HashSet<_>>();
    let appendix_ids_known = result
        .citation_validation
        .appendix_evidence_ids
        .iter()
        .all(|id| evidence_ids.contains(id.as_str()));
    TurnObservation {
        answer_non_empty: !result.assistant_message.content.trim().is_empty(),
        provider: result.run_manifest.provider.clone(),
        model: result.run_manifest.model_resolved.clone(),
        answer_format: result.run_manifest.answer_format.clone(),
        manifest_schema: result.run_manifest.schema_version.clone(),
        execution_mode: result.run_manifest.execution_mode.clone(),
        generator_stage_observed: result
            .run_manifest
            .routing_llm_stages
            .iter()
            .any(|stage| stage == "generator"),
        generator_budget_rejected: result
            .run_manifest
            .routing_budget_rejections
            .iter()
            .any(|rejection| rejection.starts_with("generator:")),
        evidence_count: result.evidence.len(),
        evidence_selected_count: result.run_manifest.evidence_selected_count,
        citation_valid: result.citation_validation.unknown_ids.is_empty()
            && result.citation_validation.syntax_valid
            && result.citation_validation.appendix_integrity
            && appendix_ids_known,
        unknown_citation_count: result.citation_validation.unknown_ids.len(),
        semantic_status: result.run_manifest.semantic_verification_status.clone(),
        semantic_fallback_reason: result
            .run_manifest
            .semantic_verification_fallback_reason
            .clone(),
        planner_status: result.run_manifest.planner_status.clone(),
        retrieval_round_count: result.run_manifest.retrieval_round_count,
        reranker_provider: result.run_manifest.reranker_provider.clone(),
        reranker_fallback: result.run_manifest.reranker_fallback,
    }
}

fn validate_observation(case: &RealE2eCase, observed: &TurnObservation) -> Vec<String> {
    let mut errors = Vec::new();
    let mut require = |valid: bool, code: &str| {
        if !valid {
            errors.push(code.to_string());
        }
    };
    require(observed.answer_non_empty, "empty_final_output");
    require(
        observed.provider == PROVIDER_CODEX,
        "generator_provider_not_codex",
    );
    require(
        !observed.model.trim().is_empty() && observed.model != "deterministic",
        "generator_model_not_real",
    );
    require(
        observed.answer_format == "natural-markdown-v2",
        "natural_output_missing",
    );
    require(
        observed.manifest_schema == "qa-run-v21",
        "manifest_schema_invalid",
    );
    require(observed.generator_stage_observed, "generator_stage_missing");
    require(
        !observed.generator_budget_rejected,
        "generator_budget_rejected",
    );
    require(observed.citation_valid, "citation_validation_failed");
    require(observed.unknown_citation_count == 0, "unknown_citation_id");
    require(observed.retrieval_round_count > 0, "retrieval_not_executed");
    require(
        observed.semantic_status == "succeeded"
            || (observed.semantic_status == "unavailable"
                && !observed.semantic_fallback_reason.trim().is_empty()),
        "semantic_status_invalid",
    );
    if !case.expected_execution_mode.is_empty() {
        require(
            observed.execution_mode == case.expected_execution_mode,
            "execution_mode_mismatch",
        );
    }
    if matches!(case.category.as_str(), "research" | "exploratory") {
        require(
            observed.planner_status == "succeeded",
            "planner_not_succeeded",
        );
        require(
            observed.evidence_selected_count > 0,
            "research_evidence_empty",
        );
    }
    if case.zero_evidence_expected {
        require(observed.evidence_count == 0, "zero_evidence_false_positive");
    } else {
        require(observed.evidence_count > 0, "evidence_empty");
        require(
            !observed.reranker_provider.trim().is_empty() && !observed.reranker_fallback,
            "real_reranker_not_measured",
        );
    }
    errors
}

fn validate_state(case: &RealE2eCase, context: &QuestionContext) -> bool {
    let expected = &case.expected_research_state;
    if expected.objectives.is_empty()
        && expected.constraints.is_empty()
        && expected.excluded_methods.is_empty()
        && expected.integer_parameters.is_empty()
    {
        return true;
    }
    let state = &context.retrieval_query.canonical_research_state;
    let objectives = expected
        .objectives
        .iter()
        .all(|value| state.objectives.contains(value));
    let constraints = expected
        .constraints
        .iter()
        .all(|value| state.constraints.contains(value));
    let excluded = expected
        .excluded_methods
        .iter()
        .all(|value| state.excluded_methods.contains(value));
    let parameters = expected.integer_parameters.iter().all(|(key, expected)| {
        state.parameters.get(key).is_some_and(|parameter| {
            matches!(parameter.value, state_mutation::ParameterValue::Integer(value) if value == *expected)
        })
    });
    objectives
        && constraints
        && excluded
        && parameters
        && context.retrieval_query.parameter_state_corruption_count == 0
}

fn safe_error_code(error: &str) -> String {
    let raw = error.split(':').next().unwrap_or("runner_failure").trim();
    let normalized = raw
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || *character == '_')
        .take(64)
        .collect::<String>()
        .to_ascii_lowercase();
    if normalized.is_empty() {
        "runner_failure".to_string()
    } else {
        normalized
    }
}

fn write_report(report: &RealE2eReport, output: &Path) -> Result<(), String> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("QA_REAL_E2E_REPORT_WRITE_FAILED: {error}"))?;
    }
    let part = output.with_extension("json.part");
    let mut bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("QA_REAL_E2E_REPORT_INVALID: {error}"))?;
    bytes.push(b'\n');
    fs::write(&part, bytes).map_err(|error| format!("QA_REAL_E2E_REPORT_WRITE_FAILED: {error}"))?;
    fs::rename(&part, output).map_err(|error| format!("QA_REAL_E2E_REPORT_RENAME_FAILED: {error}"))
}

pub fn run_files(
    repository: &Path,
    cases_path: &Path,
    output: &Path,
    requested_model: &str,
    requested_effort: &str,
) -> Result<RealE2eReport, String> {
    let suite = load_suite(cases_path)?;
    configure_local_models()?;
    let codex_status = codex_subscription::get_status();
    if !codex_status.ready {
        return Err("QA_REAL_E2E_PROVIDER_BLOCKED: codex_subscription_not_ready".to_string());
    }
    let (model, effort) = codex_subscription::resolve_model_selection(
        requested_model,
        requested_effort,
        &codex_status,
    );
    if model.trim().is_empty() || model == "deterministic" {
        return Err("QA_REAL_E2E_PROVIDER_BLOCKED: real_model_unresolved".to_string());
    }

    let workspace = TemporaryWorkspace::create()?;
    let mut connection = Connection::open(&workspace.database)
        .map_err(|error| format!("QA_REAL_E2E_DATABASE_OPEN_FAILED: {error}"))?;
    crate::db_schema(&connection)?;
    let _ = crate::rebuild_connection(&mut connection, repository)?;
    save_luna_settings(
        &connection,
        repository,
        LunaSettings {
            answer_provider: PROVIDER_CODEX.to_string(),
            codex_model: model.clone(),
            codex_reasoning_effort: effort.clone(),
            ..LunaSettings::default()
        },
    )?;

    let cancelled = AtomicBool::new(false);
    let mut results = Vec::new();
    let mut generator_invocation_count = 0usize;
    let mut semantic_succeeded_count = 0usize;
    let mut semantic_unavailable_count = 0usize;
    let mut models = HashSet::new();

    for case in &suite.cases {
        let session_id = Uuid::new_v4().to_string();
        let mut result = RealE2eCaseResult {
            id: case.id.clone(),
            category: case.category.clone(),
            state_valid: true,
            ..RealE2eCaseResult::default()
        };
        for (turn_index, turn) in case.turns.iter().enumerate() {
            generator_invocation_count += 1;
            let request_id = Uuid::new_v4().to_string();
            let request = AskRequest {
                request_id: request_id.clone(),
                question: turn.question.clone(),
                session_id: (turn_index > 0).then(|| session_id.clone()),
                evidence_limit: Some(14),
                repository_id: repository_id(repository),
                codex_model: Some(model.clone()),
                codex_reasoning_effort: Some(effort.clone()),
            };
            let prepared = match prepare_production_qa(
                &connection,
                repository,
                &request,
                &request_id,
                &cancelled,
            ) {
                Ok(prepared) => prepared,
                Err(error) => {
                    result.errors.push(safe_error_code(&error));
                    break;
                }
            };
            let mut context = prepared.context;
            let generated = match run_production_qa_generation(
                &mut context,
                &prepared.settings,
                &prepared.budget_guard,
                true,
                &model,
                &effort,
                &cancelled,
                |_| Ok(()),
            ) {
                Ok(generated) => generated,
                Err(error) => {
                    result.errors.push(safe_error_code(&error));
                    break;
                }
            };
            let answer_non_empty = !generated.answer.trim().is_empty();
            let semantic = generated.semantic_verification.clone();
            let persisted = match persist_exchange_with_metadata_and_semantic(
                &mut connection,
                repository,
                Some(&session_id),
                &context,
                generated.answer,
                generated.metadata,
                Some(&semantic),
            ) {
                Ok(persisted) => persisted,
                Err(error) => {
                    result.errors.push(safe_error_code(&error));
                    break;
                }
            };
            let mut observed = observation(&persisted);
            observed.answer_non_empty &= answer_non_empty;
            result.errors.extend(validate_observation(case, &observed));
            let state_valid = validate_state(case, &context);
            if !state_valid {
                result.errors.push("research_state_mismatch".to_string());
            }
            result.state_valid &= state_valid;
            result.turn_count += 1;
            result.execution_mode = observed.execution_mode;
            result.provider = observed.provider;
            result.model = observed.model;
            result.evidence_count = observed.evidence_count;
            result.citation_valid = observed.citation_valid;
            result.semantic_status = observed.semantic_status;
            result.semantic_fallback_reason = observed.semantic_fallback_reason;
            result.generator_stage_observed = observed.generator_stage_observed;
            result.generator_budget_rejected = observed.generator_budget_rejected;
            result.planner_status = observed.planner_status;
            result.retrieval_round_count = observed.retrieval_round_count;
            result.evidence_selected_count = observed.evidence_selected_count;
            result.reranker_provider = observed.reranker_provider;
            result.reranker_fallback = observed.reranker_fallback;
            models.insert(result.model.clone());
            if result.semantic_status == "succeeded" {
                semantic_succeeded_count += 1;
            } else if result.semantic_status == "unavailable" {
                semantic_unavailable_count += 1;
            }
        }
        result.errors.sort();
        result.errors.dedup();
        result.passed = result.turn_count == case.turns.len() && result.errors.is_empty();
        results.push(result);
    }

    let passed_count = results.iter().filter(|result| result.passed).count();
    let generator_fallback_count = results
        .iter()
        .filter(|result| result.provider != PROVIDER_CODEX)
        .count();
    let generator_budget_rejection_count = results
        .iter()
        .filter(|result| result.generator_budget_rejected)
        .count();
    let invalid_citation_count = results
        .iter()
        .filter(|result| !result.citation_valid)
        .count();
    let mut models = models.into_iter().collect::<Vec<_>>();
    models.sort();
    let report = RealE2eReport {
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        core_version: CORE_VERSION.to_string(),
        dataset_version: suite.version,
        dataset_sha256: suite.cases_sha256,
        case_count: results.len(),
        passed_count,
        generator_invocation_count,
        provider: PROVIDER_CODEX.to_string(),
        models,
        real_provider_measured: results
            .iter()
            .all(|result| result.provider == PROVIDER_CODEX && !result.model.trim().is_empty()),
        generator_fallback_count,
        generator_budget_rejection_count,
        invalid_citation_count,
        semantic_succeeded_count,
        semantic_unavailable_count,
        passed: passed_count == results.len(),
        results,
    };
    write_report(&report, output)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case(category: &str) -> RealE2eCase {
        RealE2eCase {
            id: format!("case-{category}"),
            category: category.to_string(),
            source: "synthetic".to_string(),
            turns: vec![RealE2eTurn {
                question: "一个足够长的公开测试问题".to_string(),
            }],
            expected_execution_mode: String::new(),
            zero_evidence_expected: category == "zero_evidence",
            expected_research_state: ExpectedResearchState::default(),
        }
    }

    #[test]
    fn temporary_workspace_is_removed_on_drop() {
        let path = {
            let workspace = TemporaryWorkspace::create().unwrap();
            fs::write(&workspace.database, b"fixture").unwrap();
            workspace.root.clone()
        };
        assert!(!path.exists());
    }

    #[test]
    fn observation_rejects_unknown_citation_and_generator_budget_failure() {
        let case = case("direct");
        let observed = TurnObservation {
            answer_non_empty: true,
            provider: PROVIDER_CODEX.to_string(),
            model: "gpt-fixture".to_string(),
            answer_format: "natural-markdown-v2".to_string(),
            manifest_schema: "qa-run-v21".to_string(),
            execution_mode: "direct".to_string(),
            generator_stage_observed: true,
            generator_budget_rejected: true,
            evidence_count: 1,
            evidence_selected_count: 1,
            citation_valid: false,
            unknown_citation_count: 1,
            semantic_status: "succeeded".to_string(),
            semantic_fallback_reason: String::new(),
            planner_status: "not_requested".to_string(),
            retrieval_round_count: 1,
            reranker_provider: "cross-encoder-research-v1".to_string(),
            reranker_fallback: false,
        };
        let errors = validate_observation(&case, &observed);
        assert!(errors.contains(&"generator_budget_rejected".to_string()));
        assert!(errors.contains(&"unknown_citation_id".to_string()));
        assert!(errors.contains(&"citation_validation_failed".to_string()));
    }

    #[test]
    fn metadata_report_never_serializes_questions_or_outputs() {
        let report = RealE2eReport {
            schema_version: REPORT_SCHEMA_VERSION.to_string(),
            core_version: CORE_VERSION.to_string(),
            dataset_version: "fixture".to_string(),
            dataset_sha256: "a".repeat(64),
            case_count: 1,
            passed_count: 1,
            generator_invocation_count: 1,
            provider: PROVIDER_CODEX.to_string(),
            models: vec!["gpt-fixture".to_string()],
            real_provider_measured: true,
            generator_fallback_count: 0,
            generator_budget_rejection_count: 0,
            invalid_citation_count: 0,
            semantic_succeeded_count: 1,
            semantic_unavailable_count: 0,
            passed: true,
            results: vec![RealE2eCaseResult {
                id: "fixture".to_string(),
                category: "direct".to_string(),
                passed: true,
                ..RealE2eCaseResult::default()
            }],
        };
        let serialized = serde_json::to_string(&report).unwrap().to_ascii_lowercase();
        for forbidden in [
            "question",
            "prompt",
            "assistantmessage",
            "content",
            "temppath",
        ] {
            assert!(!serialized.contains(forbidden), "forbidden={forbidden}");
        }
    }

    #[test]
    fn suite_requires_exact_public_category_matrix_and_hash() {
        let mut cases = EXPECTED_CATEGORIES
            .iter()
            .map(|category| case(category))
            .collect::<Vec<_>>();
        cases
            .iter_mut()
            .find(|case| case.category == "multi_turn")
            .unwrap()
            .turns
            .push(RealE2eTurn {
                question: "这是第二个公开多轮测试问题".to_string(),
            });
        let suite = RealE2eSuite {
            schema_version: CASE_SCHEMA_VERSION.to_string(),
            dataset_role: "development_regression_synthetic".to_string(),
            version: "fixture".to_string(),
            case_count: 5,
            cases_sha256: cases_sha256(&cases).unwrap(),
            cases,
        };
        let temp = TemporaryWorkspace::create().unwrap();
        let path = temp.root.join("cases.json");
        fs::write(&path, serde_json::to_vec(&suite).unwrap()).unwrap();
        assert_eq!(load_suite(&path).unwrap().cases.len(), 5);
    }

    #[test]
    fn bundled_suite_is_public_five_case_contract() {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let suite = load_suite(&repository.join("evals/qa_real_generator_e2e_cases.json"))
            .expect("bundled real-generator suite must remain valid");
        assert_eq!(suite.case_count, 5);
        assert!(suite.cases.iter().all(|case| matches!(
            case.source.as_str(),
            "development" | "regression" | "synthetic"
        )));
    }

    #[test]
    fn runner_and_tauri_adapter_share_the_same_core_entrypoints() {
        let runner = include_str!("real_e2e.rs");
        let ui = include_str!("../lib.rs");
        for entrypoint in ["prepare_production_qa(", "run_production_qa_generation("] {
            assert!(runner.contains(entrypoint));
            assert!(ui.contains(entrypoint));
        }
        let forbidden = ["let result = ", "codex_subscription::", "stream_answer("].concat();
        assert!(!runner.contains(&forbidden));
    }
}
