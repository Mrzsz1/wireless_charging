use super::research_memory::ResearchSessionState;
use super::research_query_context::ResearchQueryContext;
use super::state_mutation::{ParameterValue, ResearchParameter};
use super::ConversationTurn;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

pub const CASE_SCHEMA_VERSION: &str = "qa-conversation-state-cases-v2";
pub const REPORT_SCHEMA_VERSION: &str = "qa-conversation-state-report-v2";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct StateFixtureTurn {
    pub id: String,
    pub role: String,
    pub content: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpectedParameter {
    pub value: ParameterValue,
    #[serde(default)]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpectedState {
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub methods: Vec<String>,
    #[serde(default)]
    pub excluded_methods: Vec<String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, ExpectedParameter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpectedQueryContext {
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub parameters: Vec<String>,
    #[serde(default)]
    pub active_methods: Vec<String>,
    #[serde(default)]
    pub excluded_methods: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConversationStateCase {
    pub id: String,
    pub turns: Vec<StateFixtureTurn>,
    pub question: String,
    pub expected_final_state: ExpectedState,
    #[serde(default)]
    pub expected_query_context: ExpectedQueryContext,
    #[serde(default)]
    pub expected_patch_operation_count: Option<usize>,
    #[serde(default)]
    pub mixed_operation: bool,
    #[serde(default)]
    pub parameter_overwrite: bool,
    #[serde(default)]
    pub protect_destructive: bool,
    #[serde(default)]
    pub reference_required: bool,
    #[serde(default)]
    pub protect_parameter_state: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConversationStateSuite {
    pub schema_version: String,
    pub dataset_role: String,
    pub status: String,
    pub version: String,
    pub case_count: usize,
    pub cases: Vec<ConversationStateCase>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationStateCaseResult {
    pub id: String,
    pub state_exact_match: bool,
    pub objective_exact_match: bool,
    pub constraint_exact_match: bool,
    pub method_exact_match: bool,
    pub parameter_exact_match: bool,
    pub patch_operation_count_match: bool,
    pub unexpected_state_count: usize,
    pub destructive_mutation_error: bool,
    pub reference_resolution_correct: bool,
    pub parameter_implicit_reference_resolved_count: usize,
    pub parameter_implicit_reference_rejected_count: usize,
    pub parameter_unknown_name_count: usize,
    pub reported_parameter_state_corruption_count: usize,
    pub parameter_state_corruption: bool,
    pub query_context_objective_hits: usize,
    pub query_context_objective_total: usize,
    pub query_context_constraint_hits: usize,
    pub query_context_constraint_total: usize,
    pub query_context_parameter_hits: usize,
    pub query_context_parameter_total: usize,
    pub query_context_excluded_method_hits: usize,
    pub query_context_excluded_method_total: usize,
    pub actual_state: ResearchSessionState,
    pub actual_query_context: ResearchQueryContext,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationStateReport {
    pub schema_version: String,
    pub dataset_version: String,
    pub dataset_sha256: String,
    pub case_count: usize,
    pub state_exact_match: f64,
    pub objective_exact_match: f64,
    pub constraint_exact_match: f64,
    pub method_exact_match: f64,
    pub parameter_exact_match: f64,
    pub mixed_operation_exact_match: f64,
    pub parameter_overwrite_exact_match: f64,
    pub unexpected_state_rate: f64,
    pub destructive_mutation_error_rate: f64,
    pub query_context_objective_recall: f64,
    pub query_context_constraint_recall: f64,
    pub query_context_parameter_recall: f64,
    pub query_context_excluded_method_accuracy: f64,
    pub reference_resolution_accuracy: f64,
    pub parameter_implicit_reference_resolved_count: usize,
    pub parameter_implicit_reference_rejected_count: usize,
    pub parameter_unknown_name_count: usize,
    pub parameter_state_corruption_count: usize,
    pub reported_parameter_state_corruption_count: usize,
    pub passed: bool,
    pub results: Vec<ConversationStateCaseResult>,
}

pub fn load_suite(path: &Path) -> Result<ConversationStateSuite, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("CONVERSATION_STATE_EVAL_READ_FAILED: {error}"))?;
    let suite = serde_json::from_str::<ConversationStateSuite>(&raw)
        .map_err(|error| format!("CONVERSATION_STATE_EVAL_INVALID: {error}"))?;
    if suite.schema_version != CASE_SCHEMA_VERSION
        || suite.dataset_role != "conversation_state_evaluation"
        || suite.status != "frozen"
        || suite.case_count != suite.cases.len()
        || suite.cases.len() < 17
    {
        return Err("CONVERSATION_STATE_EVAL_INVALID: schema_count_or_status".to_string());
    }
    let mut ids = HashSet::new();
    for case in &suite.cases {
        if case.id.trim().is_empty()
            || !ids.insert(case.id.clone())
            || case.question.trim().is_empty()
        {
            return Err(format!("CONVERSATION_STATE_EVAL_INVALID: case={}", case.id));
        }
    }
    Ok(suite)
}

fn expected_parameters(state: &ResearchSessionState) -> BTreeMap<String, ExpectedParameter> {
    state
        .parameters
        .iter()
        .map(|(key, parameter)| {
            (
                key.clone(),
                ExpectedParameter {
                    value: parameter.value.clone(),
                    unit: parameter.unit.clone(),
                },
            )
        })
        .collect()
}

fn state_exact_parts(
    state: &ResearchSessionState,
    expected: &ExpectedState,
) -> (bool, bool, bool, bool) {
    let objectives = state.objectives == expected.objectives;
    let constraints =
        state.constraints == expected.constraints && state.assumptions == expected.assumptions;
    let methods =
        state.methods == expected.methods && state.excluded_methods == expected.excluded_methods;
    let parameters = expected_parameters(state) == expected.parameters;
    (objectives, constraints, methods, parameters)
}

fn unexpected_count(state: &ResearchSessionState, expected: &ExpectedState) -> usize {
    state
        .objectives
        .iter()
        .filter(|item| !expected.objectives.contains(item))
        .count()
        + state
            .constraints
            .iter()
            .filter(|item| !expected.constraints.contains(item))
            .count()
        + state
            .assumptions
            .iter()
            .filter(|item| !expected.assumptions.contains(item))
            .count()
        + state
            .methods
            .iter()
            .filter(|item| !expected.methods.contains(item))
            .count()
        + state
            .excluded_methods
            .iter()
            .filter(|item| !expected.excluded_methods.contains(item))
            .count()
        + state
            .parameters
            .keys()
            .filter(|key| !expected.parameters.contains_key(*key))
            .count()
}

fn hits(expected: &[String], actual: &[String]) -> usize {
    expected.iter().filter(|item| actual.contains(item)).count()
}

fn parameter_hits(expected: &[String], actual: &BTreeMap<String, ResearchParameter>) -> usize {
    expected
        .iter()
        .filter(|key| actual.contains_key(*key))
        .count()
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        1.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn boolean_ratio<I>(values: I) -> f64
where
    I: IntoIterator<Item = bool>,
{
    let values = values.into_iter().collect::<Vec<_>>();
    ratio(values.iter().filter(|value| **value).count(), values.len())
}

pub fn evaluate(suite: &ConversationStateSuite) -> Result<ConversationStateReport, String> {
    let connection = Connection::open_in_memory()
        .map_err(|error| format!("CONVERSATION_STATE_EVAL_DATABASE_FAILED: {error}"))?;
    crate::db_schema(&connection)?;
    let dataset_sha256 = format!(
        "{:x}",
        Sha256::digest(
            serde_json::to_vec(&suite.cases)
                .map_err(|error| format!("CONVERSATION_STATE_EVAL_INVALID: {error}"))?
        )
    );
    let mut results = Vec::with_capacity(suite.cases.len());
    for case in &suite.cases {
        let turns = case
            .turns
            .iter()
            .map(|turn| ConversationTurn {
                id: turn.id.clone(),
                role: turn.role.clone(),
                content: turn.content.clone(),
                request_id: turn.request_id.clone(),
            })
            .collect::<Vec<_>>();
        let query = super::build_retrieval_query(&connection, &case.question, &turns);
        let state = query.canonical_research_state.clone();
        let context = query.research_query_context.clone();
        let (objective_exact, constraint_exact, method_exact, parameter_exact) =
            state_exact_parts(&state, &case.expected_final_state);
        let state_exact = objective_exact && constraint_exact && method_exact && parameter_exact;
        let patch_match = match case.expected_patch_operation_count {
            Some(expected) => expected == query.state_patch_operation_count,
            None => true,
        };
        let expected_context = &case.expected_query_context;
        let expected_context_items = expected_context.objectives.len()
            + expected_context.constraints.len()
            + expected_context.parameters.len()
            + expected_context.active_methods.len()
            + expected_context.excluded_methods.len();
        let reference_resolution_correct = !case.reference_required
            || (context.source_state_revision == state.revision && expected_context_items > 0);
        results.push(ConversationStateCaseResult {
            id: case.id.clone(),
            state_exact_match: state_exact,
            objective_exact_match: objective_exact,
            constraint_exact_match: constraint_exact,
            method_exact_match: method_exact,
            parameter_exact_match: parameter_exact,
            patch_operation_count_match: patch_match,
            unexpected_state_count: unexpected_count(&state, &case.expected_final_state),
            destructive_mutation_error: case.protect_destructive && !state_exact,
            reference_resolution_correct,
            parameter_implicit_reference_resolved_count: query
                .parameter_implicit_reference_resolved_count,
            parameter_implicit_reference_rejected_count: query
                .parameter_implicit_reference_rejected_count,
            parameter_unknown_name_count: query.parameter_unknown_name_count,
            reported_parameter_state_corruption_count: query.parameter_state_corruption_count,
            parameter_state_corruption: case.protect_parameter_state && !parameter_exact,
            query_context_objective_hits: hits(&expected_context.objectives, &context.objectives),
            query_context_objective_total: expected_context.objectives.len(),
            query_context_constraint_hits: hits(
                &expected_context.constraints,
                &context.constraints,
            ),
            query_context_constraint_total: expected_context.constraints.len(),
            query_context_parameter_hits: parameter_hits(
                &expected_context.parameters,
                &context.parameters,
            ),
            query_context_parameter_total: expected_context.parameters.len(),
            query_context_excluded_method_hits: hits(
                &expected_context.excluded_methods,
                &context.excluded_methods,
            ),
            query_context_excluded_method_total: expected_context.excluded_methods.len(),
            actual_state: state,
            actual_query_context: context,
        });
    }

    let count = results.len();
    let actual_state_items = results
        .iter()
        .map(|result| {
            result.actual_state.objectives.len()
                + result.actual_state.constraints.len()
                + result.actual_state.assumptions.len()
                + result.actual_state.methods.len()
                + result.actual_state.excluded_methods.len()
                + result.actual_state.parameters.len()
        })
        .sum::<usize>();
    let unexpected = results
        .iter()
        .map(|result| result.unexpected_state_count)
        .sum::<usize>();
    let destructive_cases = suite
        .cases
        .iter()
        .filter(|case| case.protect_destructive)
        .count();
    let destructive_errors = results
        .iter()
        .filter(|result| result.destructive_mutation_error)
        .count();
    let recall = |hits_field: fn(&ConversationStateCaseResult) -> usize,
                  total_field: fn(&ConversationStateCaseResult) -> usize| {
        ratio(
            results.iter().map(hits_field).sum(),
            results.iter().map(total_field).sum(),
        )
    };
    let state_exact_match = boolean_ratio(results.iter().map(|result| result.state_exact_match));
    let objective_exact_match =
        boolean_ratio(results.iter().map(|result| result.objective_exact_match));
    let constraint_exact_match =
        boolean_ratio(results.iter().map(|result| result.constraint_exact_match));
    let method_exact_match = boolean_ratio(results.iter().map(|result| result.method_exact_match));
    let parameter_exact_match =
        boolean_ratio(results.iter().map(|result| result.parameter_exact_match));
    let mixed_operation_exact_match = boolean_ratio(
        suite
            .cases
            .iter()
            .zip(&results)
            .filter(|(case, _)| case.mixed_operation)
            .map(|(_, result)| result.state_exact_match && result.patch_operation_count_match),
    );
    let parameter_overwrite_exact_match = boolean_ratio(
        suite
            .cases
            .iter()
            .zip(&results)
            .filter(|(case, _)| case.parameter_overwrite)
            .map(|(_, result)| result.parameter_exact_match),
    );
    let unexpected_state_rate = ratio(unexpected, actual_state_items);
    let destructive_mutation_error_rate = ratio(destructive_errors, destructive_cases);
    let query_context_objective_recall = recall(
        |result| result.query_context_objective_hits,
        |result| result.query_context_objective_total,
    );
    let query_context_constraint_recall = recall(
        |result| result.query_context_constraint_hits,
        |result| result.query_context_constraint_total,
    );
    let query_context_parameter_recall = recall(
        |result| result.query_context_parameter_hits,
        |result| result.query_context_parameter_total,
    );
    let query_context_excluded_method_accuracy = recall(
        |result| result.query_context_excluded_method_hits,
        |result| result.query_context_excluded_method_total,
    );
    let reference_resolution_accuracy = boolean_ratio(
        suite
            .cases
            .iter()
            .zip(&results)
            .filter(|(case, _)| case.reference_required)
            .map(|(_, result)| result.reference_resolution_correct),
    );
    let parameter_implicit_reference_resolved_count = results
        .iter()
        .map(|result| result.parameter_implicit_reference_resolved_count)
        .sum();
    let parameter_implicit_reference_rejected_count = results
        .iter()
        .map(|result| result.parameter_implicit_reference_rejected_count)
        .sum();
    let parameter_unknown_name_count = results
        .iter()
        .map(|result| result.parameter_unknown_name_count)
        .sum();
    let reported_parameter_state_corruption_count = results
        .iter()
        .map(|result| result.reported_parameter_state_corruption_count)
        .sum();
    let parameter_state_corruption_count = results
        .iter()
        .filter(|result| result.parameter_state_corruption)
        .count();
    let passed = count >= 17
        && mixed_operation_exact_match == 1.0
        && parameter_overwrite_exact_match == 1.0
        && destructive_mutation_error_rate == 0.0
        && state_exact_match >= 0.98
        && objective_exact_match >= 0.98
        && constraint_exact_match >= 0.98
        && method_exact_match >= 0.98
        && parameter_exact_match >= 0.98
        && unexpected_state_rate <= 0.01
        && query_context_objective_recall >= 0.97
        && query_context_constraint_recall >= 0.97
        && query_context_parameter_recall >= 0.97
        && query_context_excluded_method_accuracy >= 0.97
        && reference_resolution_accuracy >= 0.97
        && parameter_state_corruption_count == 0;
    Ok(ConversationStateReport {
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        dataset_version: suite.version.clone(),
        dataset_sha256,
        case_count: count,
        state_exact_match,
        objective_exact_match,
        constraint_exact_match,
        method_exact_match,
        parameter_exact_match,
        mixed_operation_exact_match,
        parameter_overwrite_exact_match,
        unexpected_state_rate,
        destructive_mutation_error_rate,
        query_context_objective_recall,
        query_context_constraint_recall,
        query_context_parameter_recall,
        query_context_excluded_method_accuracy,
        reference_resolution_accuracy,
        parameter_implicit_reference_resolved_count,
        parameter_implicit_reference_rejected_count,
        parameter_unknown_name_count,
        parameter_state_corruption_count,
        reported_parameter_state_corruption_count,
        passed,
        results,
    })
}

pub fn write_report(report: &ConversationStateReport, output: &Path) -> Result<(), String> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("CONVERSATION_STATE_EVAL_WRITE_FAILED: {error}"))?;
    }
    let part = output.with_extension("json.part");
    let mut bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("CONVERSATION_STATE_EVAL_SERIALIZE_FAILED: {error}"))?;
    bytes.push(b'\n');
    fs::write(&part, bytes)
        .map_err(|error| format!("CONVERSATION_STATE_EVAL_WRITE_FAILED: {error}"))?;
    fs::rename(&part, output)
        .map_err(|error| format!("CONVERSATION_STATE_EVAL_WRITE_FAILED: {error}"))
}
