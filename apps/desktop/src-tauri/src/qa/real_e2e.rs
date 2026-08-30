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
const REPORT_SCHEMA_VERSION: &str = "qa-real-generator-e2e-report-v5";
const CORE_VERSION: &str = "qa-production-core-v1";
const LOCAL_DIAGNOSTIC_ENV: &str = "QA_REAL_E2E_GROUNDING_DIAGNOSTIC_DIR";
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
    pub persisted_turn_count: usize,
    pub persisted: bool,
    pub pre_persist: Option<GroundingObservation>,
    #[serde(rename = "final")]
    pub final_result: Option<GroundingObservation>,
    pub state_valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimDiagnostic {
    pub claim_id: String,
    pub claim_type: String,
    pub verification_status: String,
    pub evidence_id_count: usize,
    pub reason_code: String,
    pub alignment_score: f64,
    pub claim_text_sha256: String,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroundingObservation {
    pub answer_non_empty: bool,
    pub provider: String,
    pub model: String,
    pub answer_format: String,
    pub manifest_schema: String,
    pub execution_mode: String,
    pub generator_stage_observed: bool,
    pub generator_budget_rejected: bool,
    pub routing_llm_call_budget: usize,
    pub routing_budget_rejection_count: usize,
    pub routing_budget_rejections: Vec<String>,
    pub routing_llm_calls_used: usize,
    pub routing_token_cost_used: u32,
    pub routing_token_cost_ceiling: u32,
    pub evidence_count: usize,
    pub evidence_selected_count: usize,
    pub citation_valid: bool,
    pub unknown_citation_count: usize,
    pub grounding_status: String,
    pub verification_status: String,
    pub answer_complete: bool,
    pub draft_claim_count: usize,
    pub draft_supported_claim_count: usize,
    pub draft_partially_supported_claim_count: usize,
    pub draft_contradicted_claim_count: usize,
    pub draft_not_verifiable_claim_count: usize,
    pub draft_research_suggestion_count: usize,
    pub draft_repaired_claim_count: usize,
    pub draft_uncited_knowledge_fact_count: usize,
    pub draft_unverified_claim_count: usize,
    pub repair_projection_status: String,
    pub repair_projection_error_code: String,
    pub repair_projection_operation_count: usize,
    pub final_factual_claim_count: usize,
    pub final_supported_claim_count: usize,
    pub final_unsupported_claim_count: usize,
    pub final_cited_claim_count: usize,
    pub final_unknown_citation_count: usize,
    pub final_citation_coverage: f64,
    pub final_visible_projection_valid: bool,
    pub semantic_status: String,
    pub semantic_fallback_reason: String,
    pub planner_attempted: bool,
    pub planner_used: bool,
    pub planner_status: String,
    pub planner_fallback: bool,
    pub planner_fallback_reason: String,
    pub planner_latency_ms: u64,
    pub planner_stage_observed: bool,
    pub planner_budget_rejected: bool,
    pub query_plan_version: String,
    pub planned_facet_count: usize,
    pub planned_required_facet_count: usize,
    pub planned_search_query_count: usize,
    pub requested_kind_count: usize,
    pub must_attempt_kind_count: usize,
    pub retrieval_round_count: usize,
    pub retrieval_stop_reason: String,
    pub reranker_provider: String,
    pub reranker_fallback: bool,
    pub draft_claims: Vec<ClaimDiagnostic>,
    pub final_claims: Vec<ClaimDiagnostic>,
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
    pub pre_persist_invalid_citation_count: usize,
    pub final_invalid_citation_count: usize,
    pub semantic_succeeded_count: usize,
    pub semantic_unavailable_count: usize,
    pub scope: String,
    pub executed_scope_passed: bool,
    pub full_suite_evaluated: bool,
    pub release_eligible: bool,
    pub passed: bool,
    pub results: Vec<RealE2eCaseResult>,
}

fn execution_scope_outcome(
    selected_case: Option<&str>,
    suite_case_count: usize,
    executed_case_count: usize,
    passed_count: usize,
) -> (String, bool, bool, bool) {
    let full_suite_evaluated = selected_case.is_none() && executed_case_count == suite_case_count;
    let executed_scope_passed = executed_case_count > 0 && passed_count == executed_case_count;
    let release_eligible = full_suite_evaluated && executed_scope_passed;
    (
        if full_suite_evaluated {
            "full_suite"
        } else {
            "single_case"
        }
        .to_string(),
        executed_scope_passed,
        full_suite_evaluated,
        release_eligible,
    )
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocalEvidenceDiagnostic {
    evidence_id: String,
    title: String,
    snippet: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocalClaimGroundingDiagnostic {
    claim_id: String,
    text: String,
    verification_status: String,
    reason: String,
    alignment_score: f64,
    cited_evidence: Vec<LocalEvidenceDiagnostic>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocalGroundingDiagnostic {
    schema_version: String,
    case_id: String,
    request_id_hash: String,
    draft_claims: Vec<LocalClaimGroundingDiagnostic>,
    final_claims: Vec<LocalClaimGroundingDiagnostic>,
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

fn enum_label<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

fn stable_code(value: &str) -> String {
    let normalized = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || *character == '_')
        .take(96)
        .collect::<String>()
        .to_ascii_lowercase();
    if normalized.is_empty() {
        "unknown".to_string()
    } else {
        normalized
    }
}

fn claim_diagnostics(claims: &[VerifiedClaim]) -> Vec<ClaimDiagnostic> {
    claims
        .iter()
        .map(|claim| {
            let verification_status = enum_label(&claim.verification_status);
            ClaimDiagnostic {
                claim_id: claim.id.clone(),
                claim_type: enum_label(&claim.claim_type),
                verification_status: verification_status.clone(),
                evidence_id_count: claim.evidence_ids.len(),
                reason_code: stable_code(&format!(
                    "{}_{}",
                    claim.verification_method, verification_status
                )),
                alignment_score: claim.alignment_score,
                claim_text_sha256: format!("{:x}", Sha256::digest(claim.text.as_bytes())),
            }
        })
        .collect()
}

fn local_claim_diagnostics(
    claims: &[VerifiedClaim],
    evidence: &[EvidenceItem],
) -> Vec<LocalClaimGroundingDiagnostic> {
    let by_id = evidence
        .iter()
        .map(|item| (item.id.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    claims
        .iter()
        .map(|claim| LocalClaimGroundingDiagnostic {
            claim_id: claim.id.clone(),
            text: project_natural_visible_text(&claim.text),
            verification_status: enum_label(&claim.verification_status),
            reason: project_natural_visible_text(&compact(&claim.reason, 240)),
            alignment_score: claim.alignment_score,
            cited_evidence: claim
                .evidence_ids
                .iter()
                .filter_map(|id| by_id.get(id.as_str()))
                .map(|item| LocalEvidenceDiagnostic {
                    evidence_id: item.id.clone(),
                    title: project_natural_visible_text(&compact(&item.title, 240)),
                    snippet: project_natural_visible_text(&compact(&item.snippet, 1_600)),
                })
                .collect(),
        })
        .collect()
}

fn write_local_grounding_diagnostic_to_dir(
    directory: &Path,
    repository: &Path,
    case_id: &str,
    request_id: &str,
    context: &QuestionContext,
    manifest: &QaRunManifest,
) -> Result<(), String> {
    if !directory.is_absolute() {
        return Err("QA_REAL_E2E_DIAGNOSTIC_DIR_INVALID".to_string());
    }
    fs::create_dir_all(directory)
        .map_err(|_| "QA_REAL_E2E_DIAGNOSTIC_DIR_CREATE_FAILED".to_string())?;
    let directory = directory
        .canonicalize()
        .map_err(|_| "QA_REAL_E2E_DIAGNOSTIC_DIR_INVALID".to_string())?;
    let repository = repository
        .canonicalize()
        .map_err(|_| "QA_REAL_E2E_REPOSITORY_INVALID".to_string())?;
    if directory.starts_with(&repository) {
        return Err("QA_REAL_E2E_DIAGNOSTIC_DIR_MUST_BE_OUTSIDE_REPOSITORY".to_string());
    }
    let request_id_hash = trace::request_id_hash(request_id);
    let payload = LocalGroundingDiagnostic {
        schema_version: "qa-local-grounding-diagnostic-v1".to_string(),
        case_id: case_id.to_string(),
        request_id_hash: request_id_hash.clone(),
        draft_claims: local_claim_diagnostics(&manifest.claim_verifications, &context.evidence),
        final_claims: local_claim_diagnostics(
            &manifest.final_grounding_audit.claims,
            &context.evidence,
        ),
    };
    let file_name = format!("{case_id}-{request_id_hash}.json");
    let output = directory.join(file_name);
    let part = output.with_extension("json.part");
    let mut bytes = serde_json::to_vec_pretty(&payload)
        .map_err(|_| "QA_REAL_E2E_DIAGNOSTIC_INVALID".to_string())?;
    bytes.push(b'\n');
    fs::write(&part, bytes).map_err(|_| "QA_REAL_E2E_DIAGNOSTIC_WRITE_FAILED".to_string())?;
    fs::rename(&part, output).map_err(|_| "QA_REAL_E2E_DIAGNOSTIC_RENAME_FAILED".to_string())
}

fn maybe_write_local_grounding_diagnostic(
    repository: &Path,
    case_id: &str,
    request_id: &str,
    context: &QuestionContext,
    manifest: &QaRunManifest,
) -> Result<(), String> {
    let Some(directory) = env::var_os(LOCAL_DIAGNOSTIC_ENV).filter(|value| !value.is_empty())
    else {
        return Ok(());
    };
    write_local_grounding_diagnostic_to_dir(
        &PathBuf::from(directory),
        repository,
        case_id,
        request_id,
        context,
        manifest,
    )
}

fn citation_contract_valid(validation: &CitationValidation, evidence_ids: &HashSet<&str>) -> bool {
    let ids_known = validation
        .appendix_evidence_ids
        .iter()
        .all(|id| evidence_ids.contains(id.as_str()));
    if validation.zero_evidence {
        validation.unknown_ids.is_empty()
            && !validation.has_citations
            && validation.appendix_evidence_ids.is_empty()
    } else {
        validation.supported
            && validation.unknown_ids.is_empty()
            && validation.appendix_integrity
            && ids_known
    }
}

fn observation_from_parts(
    answer_non_empty: bool,
    manifest: &QaRunManifest,
    validation: &CitationValidation,
    evidence_ids: &HashSet<&str>,
    evidence_count: usize,
    retrieval_query: &RetrievalQuery,
) -> GroundingObservation {
    let draft_claims = claim_diagnostics(&manifest.claim_verifications);
    let final_claims = claim_diagnostics(&manifest.final_grounding_audit.claims);
    let research_suggestion_count = draft_claims
        .iter()
        .filter(|claim| claim.claim_type == "research_suggestion")
        .count();
    let uncited_knowledge_fact_count = manifest
        .claim_verifications
        .iter()
        .filter(|claim| {
            enum_label(&claim.claim_type) == "knowledge_fact" && claim.evidence_ids.is_empty()
        })
        .count();
    GroundingObservation {
        answer_non_empty,
        provider: manifest.provider.clone(),
        model: manifest.model_resolved.clone(),
        answer_format: manifest.answer_format.clone(),
        manifest_schema: manifest.schema_version.clone(),
        execution_mode: manifest.execution_mode.clone(),
        generator_stage_observed: manifest
            .routing_llm_stages
            .iter()
            .any(|stage| stage == "generator"),
        generator_budget_rejected: manifest
            .routing_budget_rejections
            .iter()
            .any(|rejection| rejection.starts_with("generator:")),
        routing_llm_call_budget: manifest.routing_llm_call_budget,
        routing_budget_rejection_count: manifest.routing_budget_rejections.len(),
        routing_budget_rejections: manifest.routing_budget_rejections.clone(),
        routing_llm_calls_used: manifest.routing_llm_calls_used,
        routing_token_cost_used: manifest.routing_token_cost_used,
        routing_token_cost_ceiling: manifest.routing_token_cost_ceiling,
        evidence_count,
        evidence_selected_count: manifest.evidence_selected_count,
        citation_valid: citation_contract_valid(validation, evidence_ids),
        unknown_citation_count: validation.unknown_ids.len(),
        grounding_status: validation.grounding_status.clone(),
        verification_status: manifest.verification_status.clone(),
        answer_complete: manifest.answer_completeness.complete,
        draft_claim_count: draft_claims.len(),
        draft_supported_claim_count: manifest.verified_claim_count,
        draft_partially_supported_claim_count: manifest.partially_supported_claim_count,
        draft_contradicted_claim_count: manifest.contradicted_claim_count,
        draft_not_verifiable_claim_count: manifest.not_verifiable_claim_count,
        draft_research_suggestion_count: research_suggestion_count,
        draft_repaired_claim_count: manifest.repaired_claim_count,
        draft_uncited_knowledge_fact_count: uncited_knowledge_fact_count,
        draft_unverified_claim_count: manifest.unverified_claim_count,
        repair_projection_status: manifest.repair_projection_audit.status.clone(),
        repair_projection_error_code: manifest.repair_projection_audit.error_code.clone(),
        repair_projection_operation_count: manifest.repair_projection_audit.operation_count,
        final_factual_claim_count: manifest.final_grounding_audit.factual_claim_count,
        final_supported_claim_count: manifest.final_grounding_audit.supported_count,
        final_unsupported_claim_count: manifest.final_grounding_audit.unsupported_count,
        final_cited_claim_count: manifest.final_grounding_audit.cited_claim_count,
        final_unknown_citation_count: manifest.final_grounding_audit.unknown_evidence_ids.len(),
        final_citation_coverage: manifest.final_grounding_audit.citation_coverage,
        final_visible_projection_valid: manifest.final_grounding_audit.visible_projection_valid,
        semantic_status: manifest.semantic_verification_status.clone(),
        semantic_fallback_reason: manifest.semantic_verification_fallback_reason.clone(),
        planner_attempted: matches!(
            manifest.planner_status.as_str(),
            "succeeded" | "failed_fallback"
        ),
        planner_used: retrieval_query.planner_used,
        planner_status: manifest.planner_status.clone(),
        planner_fallback: manifest.planner_fallback,
        planner_fallback_reason: manifest.planner_fallback_reason.clone(),
        planner_latency_ms: manifest.planner_latency_ms,
        planner_stage_observed: manifest
            .routing_llm_stages
            .iter()
            .any(|stage| stage == "planner"),
        planner_budget_rejected: manifest
            .routing_budget_rejections
            .iter()
            .any(|rejection| rejection.starts_with("planner:")),
        query_plan_version: manifest.query_plan_version.clone(),
        planned_facet_count: manifest.planned_facet_ids.len(),
        planned_required_facet_count: retrieval_query.planned_required_facet_count,
        planned_search_query_count: retrieval_query.planned_search_query_count,
        requested_kind_count: retrieval_query.requested_kinds.len(),
        must_attempt_kind_count: retrieval_query.must_attempt_kind_count,
        retrieval_round_count: manifest.retrieval_round_count,
        retrieval_stop_reason: manifest.retrieval_stop_reason.clone(),
        reranker_provider: manifest.reranker_provider.clone(),
        reranker_fallback: manifest.reranker_fallback,
        draft_claims,
        final_claims,
    }
}

fn observation(result: &AskResult, context: &QuestionContext) -> GroundingObservation {
    let evidence_ids = result
        .evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect::<HashSet<_>>();
    observation_from_parts(
        !result.assistant_message.content.trim().is_empty(),
        &result.run_manifest,
        &result.citation_validation,
        &evidence_ids,
        result.evidence.len(),
        &context.retrieval_query,
    )
}

fn observation_from_audit(context: &QuestionContext, audit: &AnswerAudit) -> GroundingObservation {
    let evidence_ids = context
        .evidence
        .iter()
        .map(|item| item.id.as_str())
        .collect::<HashSet<_>>();
    observation_from_parts(
        !audit.answer.trim().is_empty(),
        &audit.run_manifest,
        &audit.citation_validation,
        &evidence_ids,
        context.evidence.len(),
        &context.retrieval_query,
    )
}

fn validate_observation(case: &RealE2eCase, observed: &GroundingObservation) -> Vec<String> {
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
        observed.manifest_schema == "qa-run-v22",
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
    require(observed.answer_complete, "answer_completeness_failed");
    let semantic_status_valid = if case.zero_evidence_expected && observed.evidence_count == 0 {
        observed.semantic_status.trim().is_empty() || observed.semantic_status == "not_requested"
    } else {
        observed.semantic_status == "succeeded"
            || (observed.semantic_status == "unavailable"
                && !observed.semantic_fallback_reason.trim().is_empty())
    };
    require(semantic_status_valid, "semantic_status_invalid");
    if !case.expected_execution_mode.is_empty() {
        require(
            observed.execution_mode == case.expected_execution_mode,
            "execution_mode_mismatch",
        );
    }
    if matches!(case.category.as_str(), "research" | "exploratory") {
        if observed.planner_status == "failed_fallback" {
            require(false, "planner_failed_fallback");
        }
        if observed.planner_status == "succeeded" && !observed.planner_used {
            require(false, "planner_success_without_plan");
        }
        require(observed.planner_attempted, "planner_not_attempted");
        require(observed.planner_used, "planner_not_used");
        require(
            observed.planner_status == "succeeded",
            "planner_status_not_succeeded",
        );
        require(!observed.planner_fallback, "planner_fallback_present");
        require(
            observed.planner_fallback_reason.is_empty(),
            "planner_fallback_reason_present",
        );
        require(observed.planner_stage_observed, "planner_stage_missing");
        require(!observed.planner_budget_rejected, "planner_budget_rejected");
        require(
            observed.query_plan_version == "qa-retrieval-contract-v2",
            "planner_contract_version_invalid",
        );
        require(observed.planned_facet_count >= 1, "planner_facets_empty");
        require(
            observed.planned_search_query_count >= 1,
            "planner_queries_empty",
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
            observed.repair_projection_status == "succeeded",
            "repair_projection_failed",
        );
        require(
            observed.repair_projection_error_code.is_empty(),
            "repair_projection_error_present",
        );
        require(
            observed.final_factual_claim_count > 0,
            "final_factual_claims_empty",
        );
        require(
            observed.final_supported_claim_count == observed.final_factual_claim_count,
            "final_supported_claim_mismatch",
        );
        require(
            observed.final_unsupported_claim_count == 0,
            "final_unsupported_claim",
        );
        require(
            observed.final_unknown_citation_count == 0,
            "final_unknown_citation_id",
        );
        require(
            observed.final_citation_coverage == 1.0,
            "final_citation_coverage_incomplete",
        );
        require(
            observed.final_visible_projection_valid,
            "final_visible_projection_invalid",
        );
        require(
            !observed.reranker_provider.trim().is_empty() && !observed.reranker_fallback,
            "real_reranker_not_measured",
        );
    }
    errors
}

fn apply_final_observation(
    case: &RealE2eCase,
    result: &mut RealE2eCaseResult,
    observed: GroundingObservation,
) {
    result.errors.extend(validate_observation(case, &observed));
    result.final_result = Some(observed);
}

fn apply_persistence_failure(result: &mut RealE2eCaseResult, error: &str) -> String {
    let code = safe_error_code(error);
    result.errors.push(code.clone());
    code
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
    trace::error_code(error)
}

fn trace_observation(
    event: &str,
    status: &str,
    request_id: &str,
    case_id: &str,
    observed: &GroundingObservation,
    persisted: Option<bool>,
    error_code: &str,
) {
    let mut trace = trace::QaTraceEvent::new(event, "real_e2e", status, request_id);
    trace.case_id = case_id.to_string();
    trace.execution_mode = observed.execution_mode.clone();
    trace.provider = observed.provider.clone();
    trace.model = observed.model.clone();
    trace.evidence_count = Some(observed.evidence_count);
    trace.claim_count = Some(observed.final_factual_claim_count);
    trace.supported_claim_count = Some(observed.final_supported_claim_count);
    trace.contradicted_claim_count = Some(observed.draft_contradicted_claim_count);
    trace.not_verifiable_claim_count = Some(observed.final_unsupported_claim_count);
    trace.repaired_claim_count = Some(observed.draft_repaired_claim_count);
    trace.persisted = persisted;
    trace.error_code = error_code.to_string();
    trace::emit(&trace);
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
    let repository_root = repository
        .canonicalize()
        .map_err(|_| "QA_REAL_E2E_REPOSITORY_INVALID".to_string())?;
    trace::configure_cli_file(
        repository_root
            .join("apps")
            .join("desktop")
            .join("logs")
            .join("qa-real-e2e.jsonl"),
    )?;
    trace::emit(&trace::QaTraceEvent::new(
        "qa_e2e_started",
        "real_e2e",
        "started",
        "",
    ));
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

    let selected_case = env::var("QA_REAL_E2E_CASE_ID")
        .ok()
        .filter(|value| !value.trim().is_empty());
    let selected_cases = suite
        .cases
        .iter()
        .filter(|case| {
            selected_case
                .as_deref()
                .map_or(true, |selected| selected == case.id)
        })
        .collect::<Vec<_>>();
    if selected_cases.is_empty() {
        return Err("QA_REAL_E2E_CASE_NOT_FOUND".to_string());
    }

    for case in selected_cases {
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
                    let code = safe_error_code(&error);
                    let mut trace = trace::QaTraceEvent::new(
                        "qa_prepare_failed",
                        "prepare",
                        "failed",
                        &request_id,
                    );
                    trace.case_id = case.id.clone();
                    trace.error_code = code.clone();
                    trace::emit(&trace);
                    result.errors.push(code);
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
                    let code = safe_error_code(&error);
                    let mut trace = trace::QaTraceEvent::new(
                        "qa_generate_failed",
                        "generator",
                        "failed",
                        &request_id,
                    );
                    trace.case_id = case.id.clone();
                    trace.error_code = code.clone();
                    trace::emit(&trace);
                    result.errors.push(code);
                    break;
                }
            };
            let pre_persist = observation_from_audit(&context, &generated.audit);
            result.turn_count += 1;
            trace_observation(
                "qa_pre_persist_audited",
                "completed",
                &request_id,
                &case.id,
                &pre_persist,
                Some(false),
                "",
            );
            models.insert(pre_persist.model.clone());
            if pre_persist.semantic_status == "succeeded" {
                semantic_succeeded_count += 1;
            } else if pre_persist.semantic_status == "unavailable" {
                semantic_unavailable_count += 1;
            }
            result.pre_persist = Some(pre_persist.clone());
            if let Err(error) = maybe_write_local_grounding_diagnostic(
                &repository_root,
                &case.id,
                &request_id,
                &context,
                &generated.audit.run_manifest,
            ) {
                let code = safe_error_code(&error);
                result.errors.push(code.clone());
                trace_observation(
                    "qa_local_grounding_diagnostic_failed",
                    "failed",
                    &request_id,
                    &case.id,
                    &pre_persist,
                    Some(false),
                    &code,
                );
                break;
            }
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
                    let code = apply_persistence_failure(&mut result, &error);
                    trace_observation(
                        "qa_persist_failed",
                        "failed",
                        &request_id,
                        &case.id,
                        &pre_persist,
                        Some(false),
                        &code,
                    );
                    break;
                }
            };
            let final_observation = observation(&persisted, &context);
            result.persisted_turn_count += 1;
            if turn_index + 1 == case.turns.len() {
                let state_valid = validate_state(case, &context);
                if !state_valid {
                    result.errors.push("research_state_mismatch".to_string());
                }
                result.state_valid &= state_valid;
            }
            trace_observation(
                "qa_persist_completed",
                "succeeded",
                &request_id,
                &case.id,
                &final_observation,
                Some(true),
                "",
            );
            apply_final_observation(case, &mut result, final_observation);
        }
        result.errors.sort();
        result.errors.dedup();
        result.persisted = result.persisted_turn_count == case.turns.len();
        result.passed = result.persisted && result.errors.is_empty();
        results.push(result);
    }

    let passed_count = results.iter().filter(|result| result.passed).count();
    let generator_fallback_count = results
        .iter()
        .filter_map(|result| result.pre_persist.as_ref())
        .filter(|observed| !observed.provider.is_empty() && observed.provider != PROVIDER_CODEX)
        .count();
    let generator_budget_rejection_count = results
        .iter()
        .filter_map(|result| result.pre_persist.as_ref())
        .filter(|observed| observed.generator_budget_rejected)
        .count();
    let pre_persist_invalid_citation_count = results
        .iter()
        .filter_map(|result| result.pre_persist.as_ref())
        .filter(|observed| !observed.provider.is_empty() && !observed.citation_valid)
        .count();
    let final_invalid_citation_count = results
        .iter()
        .filter_map(|result| result.final_result.as_ref())
        .filter(|observed| !observed.provider.is_empty() && !observed.citation_valid)
        .count();
    let mut models = models.into_iter().collect::<Vec<_>>();
    models.sort();
    let (scope, executed_scope_passed, full_suite_evaluated, release_eligible) =
        execution_scope_outcome(
            selected_case.as_deref(),
            suite.case_count,
            results.len(),
            passed_count,
        );
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
        real_provider_measured: results.iter().all(|result| {
            result.pre_persist.as_ref().is_some_and(|observed| {
                observed.provider == PROVIDER_CODEX && !observed.model.trim().is_empty()
            })
        }),
        generator_fallback_count,
        generator_budget_rejection_count,
        pre_persist_invalid_citation_count,
        final_invalid_citation_count,
        semantic_succeeded_count,
        semantic_unavailable_count,
        scope,
        executed_scope_passed,
        full_suite_evaluated,
        release_eligible,
        passed: executed_scope_passed,
        results,
    };
    write_report(&report, output)?;
    let mut completed = trace::QaTraceEvent::new(
        "qa_e2e_completed",
        "real_e2e",
        if report.passed { "passed" } else { "failed" },
        "",
    );
    completed.case_id = selected_case.unwrap_or_default();
    completed.claim_count = Some(report.case_count);
    completed.supported_claim_count = Some(report.passed_count);
    completed.persisted = Some(report.results.iter().all(|result| result.persisted));
    trace::emit(&completed);
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

    fn valid_observation(category: &str) -> GroundingObservation {
        let planner_required = matches!(category, "research" | "exploratory");
        GroundingObservation {
            answer_non_empty: true,
            provider: PROVIDER_CODEX.to_string(),
            model: "gpt-fixture".to_string(),
            answer_format: "natural-markdown-v2".to_string(),
            manifest_schema: "qa-run-v22".to_string(),
            execution_mode: if planner_required {
                category.to_string()
            } else {
                "direct".to_string()
            },
            generator_stage_observed: true,
            routing_llm_call_budget: if planner_required { 4 } else { 3 },
            routing_llm_calls_used: if planner_required { 4 } else { 3 },
            routing_token_cost_ceiling: if planner_required { 18_000 } else { 8_000 },
            evidence_count: 1,
            evidence_selected_count: 1,
            citation_valid: true,
            grounding_status: "supported".to_string(),
            verification_status: "succeeded".to_string(),
            answer_complete: true,
            draft_claim_count: 1,
            draft_supported_claim_count: 1,
            repair_projection_status: "succeeded".to_string(),
            final_factual_claim_count: 1,
            final_supported_claim_count: 1,
            final_cited_claim_count: 1,
            final_citation_coverage: 1.0,
            final_visible_projection_valid: true,
            semantic_status: "succeeded".to_string(),
            planner_attempted: planner_required,
            planner_used: planner_required,
            planner_status: if planner_required {
                "succeeded".to_string()
            } else {
                "policy_disabled".to_string()
            },
            planner_stage_observed: planner_required,
            query_plan_version: "qa-retrieval-contract-v2".to_string(),
            planned_facet_count: usize::from(planner_required),
            planned_required_facet_count: usize::from(planner_required),
            planned_search_query_count: usize::from(planner_required),
            requested_kind_count: 2,
            must_attempt_kind_count: 1,
            retrieval_round_count: 1,
            retrieval_stop_reason: "facet_sufficient".to_string(),
            reranker_provider: "cross-encoder-research-v1".to_string(),
            ..GroundingObservation::default()
        }
    }

    #[test]
    fn research_failed_fallback_fails_the_executed_scope() {
        let case = case("research");
        let observed = GroundingObservation {
            planner_attempted: true,
            planner_used: false,
            planner_status: "failed_fallback".to_string(),
            planner_fallback: true,
            planner_fallback_reason: "output_schema_rejected".to_string(),
            ..valid_observation("research")
        };

        let errors = validate_observation(&case, &observed);

        assert!(errors.contains(&"planner_failed_fallback".to_string()));
        assert!(!execution_scope_outcome(Some("fixture"), 5, 1, 0).1);
    }

    #[test]
    fn research_succeeded_requires_a_real_usable_plan() {
        let case = case("research");
        let observed = valid_observation("research");

        let errors = validate_observation(&case, &observed);

        assert!(errors.is_empty(), "{errors:?}");
        let no_plan = GroundingObservation {
            planner_used: false,
            ..observed
        };
        assert!(validate_observation(&case, &no_plan)
            .contains(&"planner_success_without_plan".to_string()));
    }

    #[test]
    fn direct_policy_disabled_remains_a_legal_planner_state() {
        let errors = validate_observation(&case("direct"), &valid_observation("direct"));
        assert!(errors.is_empty(), "{errors:?}");
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
        let observed = GroundingObservation {
            answer_non_empty: true,
            provider: PROVIDER_CODEX.to_string(),
            model: "gpt-fixture".to_string(),
            answer_format: "natural-markdown-v2".to_string(),
            manifest_schema: "qa-run-v22".to_string(),
            execution_mode: "direct".to_string(),
            generator_stage_observed: true,
            generator_budget_rejected: true,
            routing_budget_rejection_count: 1,
            routing_llm_calls_used: 1,
            routing_token_cost_used: 100,
            routing_token_cost_ceiling: 8_000,
            evidence_count: 1,
            evidence_selected_count: 1,
            citation_valid: false,
            unknown_citation_count: 1,
            grounding_status: "invalid".to_string(),
            verification_status: "succeeded".to_string(),
            answer_complete: true,
            draft_contradicted_claim_count: 0,
            draft_not_verifiable_claim_count: 0,
            draft_unverified_claim_count: 0,
            semantic_status: "succeeded".to_string(),
            semantic_fallback_reason: String::new(),
            planner_status: "not_requested".to_string(),
            retrieval_round_count: 1,
            reranker_provider: "cross-encoder-research-v1".to_string(),
            reranker_fallback: false,
            ..GroundingObservation::default()
        };
        let errors = validate_observation(&case, &observed);
        assert!(errors.contains(&"generator_budget_rejected".to_string()));
        assert!(errors.contains(&"unknown_citation_id".to_string()));
        assert!(errors.contains(&"citation_validation_failed".to_string()));
    }

    #[test]
    fn observation_rejects_failed_repair_projection_independently() {
        let case = case("direct");
        let observed = GroundingObservation {
            repair_projection_status: "failed".to_string(),
            repair_projection_error_code: "introduced_factual_claim".to_string(),
            ..valid_observation("direct")
        };

        let errors = validate_observation(&case, &observed);

        assert!(errors.contains(&"repair_projection_failed".to_string()));
        assert!(errors.contains(&"repair_projection_error_present".to_string()));
    }

    #[test]
    fn zero_evidence_accepts_semantic_not_run() {
        let case = case("zero_evidence");
        let observed = GroundingObservation {
            answer_non_empty: true,
            provider: PROVIDER_CODEX.to_string(),
            model: "gpt-fixture".to_string(),
            answer_format: "natural-markdown-v2".to_string(),
            manifest_schema: "qa-run-v22".to_string(),
            execution_mode: "exploratory".to_string(),
            generator_stage_observed: true,
            citation_valid: true,
            answer_complete: true,
            retrieval_round_count: 1,
            ..GroundingObservation::default()
        };

        assert!(validate_observation(&case, &observed).is_empty());
    }

    #[test]
    fn executed_scope_pass_and_release_eligibility_are_independent() {
        assert_eq!(
            execution_scope_outcome(Some("real-direct-rose"), 5, 1, 1),
            ("single_case".to_string(), true, false, false)
        );
        assert_eq!(
            execution_scope_outcome(Some("real-direct-rose"), 5, 1, 0),
            ("single_case".to_string(), false, false, false)
        );
        assert_eq!(
            execution_scope_outcome(None, 5, 5, 5),
            ("full_suite".to_string(), true, true, true)
        );
        assert_eq!(
            execution_scope_outcome(None, 5, 5, 4),
            ("full_suite".to_string(), false, true, false)
        );
    }

    #[test]
    fn cli_maps_executed_scope_failure_to_two_and_infrastructure_error_to_one() {
        let cli = include_str!("../bin/qa-real-e2e.rs");
        assert!(cli.contains("Ok(false)"));
        assert!(cli.contains("std::process::exit(2)"));
        assert!(cli.contains("Err(error)"));
        assert!(cli.contains("std::process::exit(1)"));
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
            pre_persist_invalid_citation_count: 0,
            final_invalid_citation_count: 0,
            semantic_succeeded_count: 1,
            semantic_unavailable_count: 0,
            scope: "single_case".to_string(),
            executed_scope_passed: true,
            full_suite_evaluated: false,
            release_eligible: false,
            passed: true,
            results: vec![RealE2eCaseResult {
                id: "fixture".to_string(),
                category: "research".to_string(),
                passed: true,
                final_result: Some(valid_observation("research")),
                ..RealE2eCaseResult::default()
            }],
        };
        let serialized = serde_json::to_string(&report).unwrap().to_ascii_lowercase();
        assert!(serialized.contains("qa-real-generator-e2e-report-v5"));
        assert!(serialized.contains("plannerfallbackreason"));
        assert!(serialized.contains("plannedsearchquerycount"));
        for forbidden in [
            "question",
            "prompt",
            "assistantmessage",
            "content",
            "temppath",
            "rawoutput",
            "searchqueries",
            "snippet",
            "stderr",
            "path",
            "repositorypath",
        ] {
            assert!(!serialized.contains(forbidden), "forbidden={forbidden}");
        }
    }

    #[test]
    fn claim_diagnostic_hashes_text_and_keeps_only_reason_code() {
        let claim_text = "Sensitive synthetic claim text [E1].";
        let diagnostics = claim_diagnostics(&[VerifiedClaim {
            id: "C1".to_string(),
            text: claim_text.to_string(),
            evidence_ids: vec!["E1".to_string()],
            claim_type: crate::qa::claim_verification::ClaimType::KnowledgeFact,
            verification_status: crate::qa::claim_verification::VerificationStatus::NotVerifiable,
            confidence: Some(0.7),
            verification_method: "semantic_nli".to_string(),
            alignment_score: 0.42,
            reason: "Provider explanation may repeat sensitive claim text".to_string(),
        }]);

        let serialized = serde_json::to_string(&diagnostics).unwrap();
        assert!(!serialized.contains(claim_text));
        assert!(!serialized.contains("Provider explanation"));
        assert_eq!(diagnostics[0].claim_text_sha256.len(), 64);
        assert_eq!(diagnostics[0].reason_code, "semantic_nli_not_verifiable");
    }

    #[test]
    fn local_grounding_diagnostic_contains_alignment_but_redacts_absolute_paths() {
        let claim = VerifiedClaim {
            id: "C1".to_string(),
            text: "Supported fixture at C:\\private\\claim.md [E1].".to_string(),
            evidence_ids: vec!["E1".to_string()],
            claim_type: crate::qa::claim_verification::ClaimType::KnowledgeFact,
            verification_status: crate::qa::claim_verification::VerificationStatus::Supported,
            confidence: Some(0.9),
            verification_method: "semantic_nli".to_string(),
            alignment_score: 0.91,
            reason: "direct support".to_string(),
        };
        let source = EvidenceItem {
            id: "E1".to_string(),
            title: "Fixture C:\\private\\title.md".to_string(),
            snippet: "Fixture snippet C:\\private\\snippet.md".to_string(),
            ..EvidenceItem::default()
        };

        let diagnostics = local_claim_diagnostics(&[claim], &[source]);
        let serialized = serde_json::to_string(&diagnostics).unwrap();
        assert!(serialized.contains("alignmentScore"));
        assert!(serialized.contains("citedEvidence"));
        assert!(serialized.contains("本地路径已隐藏"));
        assert!(!serialized.contains("C:\\\\private"));
    }

    #[test]
    fn evidence_backed_citation_contract_requires_supported_grounding() {
        let ids = HashSet::from(["E1"]);
        let invalid = CitationValidation {
            unknown_ids: Vec::new(),
            appendix_integrity: true,
            appendix_evidence_ids: vec!["E1".to_string()],
            grounding_status: "invalid".to_string(),
            supported: false,
            ..CitationValidation::default()
        };
        assert!(!citation_contract_valid(&invalid, &ids));

        let supported = CitationValidation {
            supported: true,
            grounding_status: "supported".to_string(),
            ..invalid
        };
        assert!(citation_contract_valid(&supported, &ids));
    }

    #[test]
    fn pre_persist_diagnostics_never_pollute_final_verdict_errors() {
        let case = case("direct");
        let pre_persist = GroundingObservation {
            citation_valid: false,
            grounding_status: "invalid".to_string(),
            ..GroundingObservation::default()
        };
        let final_observation = GroundingObservation {
            answer_non_empty: true,
            provider: PROVIDER_CODEX.to_string(),
            model: "gpt-fixture".to_string(),
            answer_format: "natural-markdown-v2".to_string(),
            manifest_schema: "qa-run-v22".to_string(),
            execution_mode: "direct".to_string(),
            generator_stage_observed: true,
            evidence_count: 1,
            evidence_selected_count: 1,
            citation_valid: true,
            grounding_status: "supported".to_string(),
            final_factual_claim_count: 1,
            final_supported_claim_count: 1,
            final_cited_claim_count: 1,
            final_citation_coverage: 1.0,
            final_visible_projection_valid: true,
            verification_status: "succeeded".to_string(),
            repair_projection_status: "succeeded".to_string(),
            answer_complete: true,
            semantic_status: "succeeded".to_string(),
            planner_status: "policy_disabled".to_string(),
            retrieval_round_count: 1,
            reranker_provider: "cross-encoder-research-v1".to_string(),
            ..GroundingObservation::default()
        };
        let mut result = RealE2eCaseResult {
            pre_persist: Some(pre_persist),
            ..RealE2eCaseResult::default()
        };

        apply_final_observation(&case, &mut result, final_observation);

        assert!(result.errors.is_empty());
        assert!(!result.pre_persist.as_ref().unwrap().citation_valid);
        assert!(result.final_result.as_ref().unwrap().citation_valid);
    }

    #[test]
    fn persistence_failure_has_no_final_observation() {
        let mut result = RealE2eCaseResult {
            pre_persist: Some(GroundingObservation {
                citation_valid: false,
                grounding_status: "invalid".to_string(),
                ..GroundingObservation::default()
            }),
            ..RealE2eCaseResult::default()
        };

        let code =
            apply_persistence_failure(&mut result, "CITATION_VALIDATION_FAILED: fixture details");

        assert_eq!(code, "citation_validation_failed");
        assert_eq!(result.errors, vec!["citation_validation_failed"]);
        assert!(result.final_result.is_none());
        assert!(!result.persisted);
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

    #[test]
    fn planner_lifecycle_events_are_wired_at_the_orchestration_boundary() {
        let qa = include_str!("../qa.rs");
        for event in [
            "qa_planner_started",
            "qa_planner_completed",
            "qa_planner_failed",
        ] {
            assert!(qa.contains(event), "missing={event}");
        }
        assert!(qa.contains("stable_planner_failure_kind"));
        assert!(!qa.contains("planner_finished_event.error_code = error"));
    }
}
