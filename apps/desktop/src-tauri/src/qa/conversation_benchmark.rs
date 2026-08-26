use super::{research_memory, ConversationTurn};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

pub const CASE_SCHEMA_VERSION: &str = "qa-production-conversation-cases-v1";
pub const REPORT_SCHEMA_VERSION: &str = "qa-production-conversation-report-v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConversationFixtureTurn {
    pub id: String,
    pub role: String,
    pub content: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConversationCase {
    pub id: String,
    pub turns: Vec<ConversationFixtureTurn>,
    pub question: String,
    pub expected_references: Vec<String>,
    pub expected_constraints: Vec<String>,
    pub expected_objectives: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConversationSuite {
    pub schema_version: String,
    pub dataset_role: String,
    pub status: String,
    pub version: String,
    pub case_count: usize,
    pub cases_sha256: String,
    pub canonical_values: HashMap<String, Vec<String>>,
    pub cases: Vec<ConversationCase>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationCaseResult {
    pub id: String,
    pub reference_hits: usize,
    pub reference_total: usize,
    pub constraint_hits: usize,
    pub constraint_total: usize,
    pub objective_hits: usize,
    pub objective_total: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationReport {
    pub schema_version: String,
    pub dataset_version: String,
    pub dataset_sha256: String,
    pub case_count: usize,
    pub reference_resolution: f64,
    pub constraint_preservation: f64,
    pub objective_preservation: f64,
    pub results: Vec<ConversationCaseResult>,
}

fn cases_sha256(cases: &[ConversationCase]) -> Result<String, String> {
    let bytes =
        serde_json::to_vec(cases).map_err(|error| format!("CONVERSATION_EVAL_INVALID: {error}"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn load_suite(path: &Path) -> Result<ConversationSuite, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("CONVERSATION_EVAL_READ_FAILED: {error}"))?;
    let suite = serde_json::from_str::<ConversationSuite>(&raw)
        .map_err(|error| format!("CONVERSATION_EVAL_INVALID: {error}"))?;
    if suite.schema_version != CASE_SCHEMA_VERSION
        || suite.dataset_role != "production_conversation_evaluation"
        || suite.status != "frozen"
        || suite.case_count != suite.cases.len()
        || suite.cases.len() < 50
        || cases_sha256(&suite.cases)? != suite.cases_sha256
    {
        return Err("CONVERSATION_EVAL_INVALID: schema_count_or_hash".to_string());
    }
    let mut ids = HashSet::new();
    for case in &suite.cases {
        if case.id.trim().is_empty()
            || !ids.insert(case.id.clone())
            || case.turns.is_empty()
            || case.question.trim().is_empty()
            || case.expected_references.is_empty()
            || case.expected_constraints.is_empty()
            || case.expected_objectives.is_empty()
        {
            return Err(format!("CONVERSATION_EVAL_INVALID: case={}", case.id));
        }
        for canonical in case
            .expected_constraints
            .iter()
            .chain(&case.expected_objectives)
        {
            if !suite.canonical_values.contains_key(canonical) {
                return Err(format!(
                    "CONVERSATION_EVAL_INVALID: missing canonical={canonical}"
                ));
            }
        }
    }
    Ok(suite)
}

fn contains_alias(values: &[String], aliases: &[String]) -> bool {
    let normalized = values.join(" ").to_lowercase();
    aliases
        .iter()
        .any(|alias| normalized.contains(&alias.to_lowercase()))
}

fn metric_totals(results: &[ConversationCaseResult], field: &str) -> (usize, usize) {
    results
        .iter()
        .fold((0, 0), |(hits, total), result| match field {
            "reference" => (hits + result.reference_hits, total + result.reference_total),
            "constraint" => (
                hits + result.constraint_hits,
                total + result.constraint_total,
            ),
            _ => (hits + result.objective_hits, total + result.objective_total),
        })
}

fn ratio((hits, total): (usize, usize)) -> f64 {
    if total == 0 {
        0.0
    } else {
        hits as f64 / total as f64
    }
}

pub fn evaluate(suite: &ConversationSuite) -> Result<ConversationReport, String> {
    let connection = Connection::open_in_memory()
        .map_err(|error| format!("CONVERSATION_EVAL_DATABASE_FAILED: {error}"))?;
    crate::db_schema(&connection)?;
    let results = suite
        .cases
        .iter()
        .map(|case| {
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
            let state = research_memory::derive(&turns, &case.question);
            let reference_hits = case
                .expected_references
                .iter()
                .filter(|expected| {
                    query
                        .entities
                        .iter()
                        .any(|entity| entity.eq_ignore_ascii_case(expected))
                        || query
                            .resolved_question
                            .to_lowercase()
                            .contains(&expected.to_lowercase())
                })
                .count();
            let constraint_hits = case
                .expected_constraints
                .iter()
                .filter(|expected| {
                    suite
                        .canonical_values
                        .get(*expected)
                        .is_some_and(|aliases| contains_alias(&state.constraints, aliases))
                })
                .count();
            let objective_hits = case
                .expected_objectives
                .iter()
                .filter(|expected| {
                    suite
                        .canonical_values
                        .get(*expected)
                        .is_some_and(|aliases| contains_alias(&state.objectives, aliases))
                })
                .count();
            ConversationCaseResult {
                id: case.id.clone(),
                reference_hits,
                reference_total: case.expected_references.len(),
                constraint_hits,
                constraint_total: case.expected_constraints.len(),
                objective_hits,
                objective_total: case.expected_objectives.len(),
            }
        })
        .collect::<Vec<_>>();
    Ok(ConversationReport {
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        dataset_version: suite.version.clone(),
        dataset_sha256: suite.cases_sha256.clone(),
        case_count: results.len(),
        reference_resolution: ratio(metric_totals(&results, "reference")),
        constraint_preservation: ratio(metric_totals(&results, "constraint")),
        objective_preservation: ratio(metric_totals(&results, "objective")),
        results,
    })
}

pub fn write_report(report: &ConversationReport, output: &Path) -> Result<(), String> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("CONVERSATION_EVAL_WRITE_FAILED: {error}"))?;
    }
    let part = output.with_extension("json.part");
    let mut bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("CONVERSATION_EVAL_SERIALIZE_FAILED: {error}"))?;
    bytes.push(b'\n');
    fs::write(&part, bytes).map_err(|error| format!("CONVERSATION_EVAL_WRITE_FAILED: {error}"))?;
    fs::rename(&part, output).map_err(|error| format!("CONVERSATION_EVAL_WRITE_FAILED: {error}"))
}
