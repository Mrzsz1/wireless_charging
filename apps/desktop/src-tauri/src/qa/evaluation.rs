use super::metrics::RetrievalDiagnosticsBuilder;
use super::retrieval_contract::{
    RetrievalBudget, RetrievalContract, RetrievalFacet, RetrievalScope,
};
use super::{
    apply_intent, candidate_key, candidate_source_locator, diverse_top_candidates, query_terms,
    retrieve_pass, Candidate, INTENT_SOLVE,
};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub const CASE_SCHEMA_VERSION: &str = "qa-rag-evaluation-cases-v1";
pub const REPORT_SCHEMA_VERSION: &str = "qa-rag-evaluation-report-v3";
pub const MRR_DIAGNOSTIC_SCHEMA_VERSION: &str = "qa-mrr-diagnostics-v1";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvaluationSuite {
    pub schema_version: String,
    pub name: String,
    pub cases: Vec<EvaluationCase>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvaluationCase {
    pub id: String,
    pub question: String,
    #[serde(default)]
    pub conversation: Vec<EvaluationConversationTurn>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub explicit_sources: Vec<String>,
    pub scope_expectation: String,
    pub must_attempt_kinds: Vec<String>,
    pub expected_documents: Vec<String>,
    #[serde(default)]
    pub expected_headings: Vec<String>,
    #[serde(default)]
    pub forbidden_evidence_kinds: Vec<String>,
    pub locator_required: bool,
    #[serde(default)]
    pub zero_evidence_expected: bool,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EvaluationConversationTurn {
    pub id: String,
    pub role: String,
    pub content: String,
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedEvidence {
    pub rank: usize,
    pub score: f64,
    pub stable_source_id: String,
    pub kind: String,
    pub document_id: String,
    pub canonical_document_id: String,
    pub block_id: String,
    pub title: String,
    pub relation: String,
    pub locator_valid: bool,
    pub locator_matched_by: String,
    pub retrieval_channels: Vec<String>,
    pub rrf_score: Option<f64>,
    pub base_rank: Option<usize>,
    pub base_score: Option<f64>,
    pub cross_encoder_score: Option<f64>,
    pub document_repeat_count: Option<usize>,
    pub document_repeat_penalty: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationCaseResult {
    pub id: String,
    pub question: String,
    pub resolved_question: String,
    pub execution_mode: String,
    pub notes: String,
    pub passed: bool,
    pub errors: Vec<String>,
    pub error_categories: Vec<String>,
    pub source_resolution_correct: bool,
    pub channel_attempt_rate: f64,
    pub document_recall_at_5: f64,
    pub document_recall_at_10: f64,
    pub document_recall_at_20: f64,
    pub heading_recall_at_20: f64,
    pub passage_mrr: f64,
    pub document_mrr: f64,
    pub mrr: f64,
    pub ndcg_at_10: f64,
    pub locator_validity: f64,
    pub zero_evidence_observed: bool,
    pub latency_ms: u64,
    pub round_count: usize,
    pub stop_reason: String,
    pub reranker_version: String,
    pub reranker_status: String,
    pub reranker_latency_ms: u64,
    pub reranker_candidate_count: usize,
    pub reranker_batch_size: usize,
    pub reranker_batch_count: usize,
    pub reranker_model_max_length: usize,
    pub reranker_model_load_ms: u64,
    pub reranker_input_prepare_ms: u64,
    pub reranker_inference_ms: u64,
    pub reranker_average_input_tokens: usize,
    pub reranker_fallback: bool,
    pub reranker_fallback_reason: String,
    pub attempted_kinds: Vec<String>,
    pub source_gaps: Vec<String>,
    pub v2_evidence: Vec<RankedEvidence>,
    pub v2_documents: Vec<RankedEvidence>,
    pub legacy_evidence: Vec<RankedEvidence>,
    pub improvements: Vec<String>,
    pub regressions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationAggregate {
    pub case_count: usize,
    pub passed_count: usize,
    pub source_resolution_accuracy: f64,
    pub channel_attempt_rate: f64,
    pub document_recall_at_5: f64,
    pub document_recall_at_10: f64,
    pub document_recall_at_20: f64,
    pub heading_recall_at_20: f64,
    pub passage_mrr: f64,
    pub document_mrr: f64,
    pub mrr: f64,
    pub ndcg_at_10: f64,
    pub locator_validity: f64,
    pub zero_evidence_false_negative: usize,
    pub zero_evidence_false_positive: usize,
    pub average_latency_ms: f64,
    pub average_round_count: f64,
    pub average_reranker_latency_ms: f64,
    pub average_reranker_model_load_ms: f64,
    pub average_reranker_input_prepare_ms: f64,
    pub average_reranker_inference_ms: f64,
    pub average_reranker_input_tokens: f64,
    pub reranker_fallback_count: usize,
    pub reranker_fallback_rate: f64,
    pub error_categories: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluationReport {
    pub schema_version: String,
    pub suite_name: String,
    pub generated_at_unix: u64,
    pub index_snapshot_id: String,
    pub retriever_version: String,
    pub answer_format: String,
    pub passed: bool,
    pub aggregate: EvaluationAggregate,
    pub cases: Vec<EvaluationCaseResult>,
    pub remaining_risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MrrDiagnosticCandidate {
    pub passage_rank: usize,
    pub document_rank: usize,
    pub stable_source_id: String,
    pub document_id: String,
    pub canonical_document_id: String,
    pub heading: String,
    pub is_relevant: bool,
    pub retrieval_channels: Vec<String>,
    pub rrf_score: Option<f64>,
    pub base_rank: Option<usize>,
    pub base_score: Option<f64>,
    pub cross_encoder_score: Option<f64>,
    pub final_score: f64,
    pub document_repeat_count: Option<usize>,
    pub document_repeat_penalty: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MrrDiagnosticCase {
    pub query_id: String,
    pub question: String,
    pub first_relevant_passage_rank: Option<usize>,
    pub first_relevant_document_rank: Option<usize>,
    pub passage_mrr: f64,
    pub document_mrr: f64,
    pub top_10: Vec<MrrDiagnosticCandidate>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MrrDiagnosticReport {
    pub schema_version: String,
    pub metric_unit: String,
    pub passage_metric_is_diagnostic_only: bool,
    pub cases: Vec<MrrDiagnosticCase>,
}

fn known_kind(value: &str) -> bool {
    matches!(value, "wiki" | "paper" | "book" | "graph")
}

pub fn load_suite(path: &Path) -> Result<EvaluationSuite, String> {
    let raw =
        fs::read_to_string(path).map_err(|error| format!("RAG_EVAL_CASES_READ_FAILED: {error}"))?;
    let suite = serde_json::from_str::<EvaluationSuite>(&raw)
        .map_err(|error| format!("RAG_EVAL_CASES_INVALID: {error}"))?;
    validate_suite(&suite)?;
    Ok(suite)
}

fn validate_suite(suite: &EvaluationSuite) -> Result<(), String> {
    if suite.schema_version != CASE_SCHEMA_VERSION {
        return Err("RAG_EVAL_CASES_INVALID: schemaVersion 不受支持".to_string());
    }
    if suite.name.trim().is_empty() || !(8..=80).contains(&suite.cases.len()) {
        return Err("RAG_EVAL_CASES_INVALID: name 不能为空且 cases 必须为 8–80 条".to_string());
    }
    let mut ids = HashSet::new();
    for case in &suite.cases {
        if case.id.trim().is_empty()
            || !case.id.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '-' | '_')
            })
            || !ids.insert(case.id.clone())
        {
            return Err(format!(
                "RAG_EVAL_CASES_INVALID: case id 非法或重复：{}",
                case.id
            ));
        }
        if case.question.trim().chars().count() < 2 {
            return Err(format!("RAG_EVAL_CASES_INVALID: {} question 过短", case.id));
        }
        if case.conversation.iter().any(|turn| {
            turn.id.trim().is_empty()
                || turn.request_id.trim().is_empty()
                || turn.content.trim().is_empty()
                || !matches!(turn.role.as_str(), "user" | "assistant")
        }) {
            return Err(format!(
                "RAG_EVAL_CASES_INVALID: {} conversation 非法",
                case.id
            ));
        }
        if !matches!(case.scope_expectation.as_str(), "open" | "sources") {
            return Err(format!(
                "RAG_EVAL_CASES_INVALID: {} scopeExpectation 非法",
                case.id
            ));
        }
        if case.must_attempt_kinds.is_empty()
            || case
                .must_attempt_kinds
                .iter()
                .any(|kind| !matches!(kind.as_str(), "wiki" | "paper" | "book"))
            || case
                .forbidden_evidence_kinds
                .iter()
                .any(|kind| !known_kind(kind))
        {
            return Err(format!(
                "RAG_EVAL_CASES_INVALID: {} evidence kind 非法",
                case.id
            ));
        }
        if case.zero_evidence_expected && !case.expected_documents.is_empty() {
            return Err(format!(
                "RAG_EVAL_CASES_INVALID: {} 零证据用例声明了 expectedDocuments",
                case.id
            ));
        }
        if case.locator_required && case.expected_documents.is_empty() {
            return Err(format!(
                "RAG_EVAL_CASES_INVALID: {} locatorRequired 缺少 expectedDocuments",
                case.id
            ));
        }
    }
    Ok(())
}

fn evaluation_contract(
    case: &EvaluationCase,
    resolved_question: &str,
    execution_mode: &str,
) -> RetrievalContract {
    let mut contract = RetrievalContract::fallback(resolved_question);
    contract.scope = RetrievalScope {
        mode: case.scope_expectation.clone(),
        explicit_sources: if case.scope_expectation == "sources" {
            if case.explicit_sources.is_empty() {
                contract.scope.explicit_sources
            } else {
                case.explicit_sources.clone()
            }
        } else {
            Vec::new()
        },
    };
    contract.concepts = vec![resolved_question.to_string()];
    contract.aliases = case.aliases.clone();
    contract.requested_kinds = if case.zero_evidence_expected {
        vec!["wiki".into(), "paper".into(), "book".into()]
    } else {
        case.must_attempt_kinds.clone()
    };
    contract.must_attempt_kinds = contract.requested_kinds.clone();
    contract.facets = vec![RetrievalFacet {
        id: "question".into(),
        label: "完整研究问题".into(),
        required: true,
        search_queries: std::iter::once(resolved_question.to_string())
            .chain(case.aliases.iter().cloned())
            .take(4)
            .collect(),
        preferred_kinds: case.must_attempt_kinds.clone(),
    }];
    contract.budget = RetrievalBudget {
        max_rounds: 3,
        max_queries: 12,
        max_candidates: super::routing_policy(execution_mode).max_candidates,
    };
    contract
}

fn legacy_candidates(
    connection: &Connection,
    root: &Path,
    question: &str,
) -> Result<Vec<Candidate>, String> {
    let mut diagnostics = RetrievalDiagnosticsBuilder::new();
    let terms = query_terms(question);
    let mut candidates = retrieve_pass(
        connection,
        root,
        question,
        &terms,
        &mut diagnostics,
        1,
        None,
    )?;
    apply_intent(INTENT_SOLVE, &mut candidates);
    candidates.sort_by(|left, right| right.score.total_cmp(&left.score));
    let mut seen = HashSet::new();
    candidates.retain(|candidate| seen.insert(candidate_key(candidate)));
    Ok(diverse_top_candidates(&candidates, 30))
}

fn candidate_document(connection: &Connection, candidate: &Candidate) -> Result<String, String> {
    if !candidate.node_id.trim().is_empty() {
        if let Some(document_id) = connection
            .query_row(
                "SELECT document_id FROM content_blocks_v2 WHERE id=?1 AND active=1",
                [&candidate.node_id],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| format!("RAG_EVAL_INDEX_READ_FAILED: {error}"))?
        {
            return Ok(document_id);
        }
    }
    Ok(match candidate.kind.as_str() {
        "paper" => format!("paper:{}", candidate.page_id),
        "wiki" => format!("wiki:{}", candidate.page_id),
        "book" => format!("book:{}", candidate.book_id),
        _ => format!("graph:{}", candidate.node_id),
    })
}

fn trace_field<'a>(trace: &'a str, name: &str) -> Option<&'a str> {
    let prefix = format!("{name}=");
    trace
        .split('；')
        .flat_map(|segment| segment.split_whitespace())
        .find_map(|token| token.strip_prefix(&prefix))
}

fn trace_number(trace: &str, name: &str) -> Option<f64> {
    trace_field(trace, name).and_then(|value| value.parse::<f64>().ok())
}

fn trace_integer(trace: &str, name: &str) -> Option<usize> {
    trace_field(trace, name).and_then(|value| value.parse::<usize>().ok())
}

fn trace_channels(trace: &str) -> Vec<String> {
    let mut channels = trace_field(trace, "channels")
        .into_iter()
        .flat_map(|value| value.split(','))
        .filter_map(|origin| origin.split('@').next())
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.len() <= 40)
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    channels.sort();
    channels.dedup();
    channels
}

fn project_candidates(
    connection: &Connection,
    root: &Path,
    candidates: &[Candidate],
) -> Result<Vec<RankedEvidence>, String> {
    candidates
        .iter()
        .take(30)
        .enumerate()
        .map(|(index, candidate)| {
            let resolved = candidate_source_locator(connection, root, candidate)
                .and_then(|locator| super::locator::resolve(connection, root, &locator).ok());
            let document_id = candidate_document(connection, candidate)?;
            Ok(RankedEvidence {
                rank: index + 1,
                score: candidate.score,
                stable_source_id: candidate_key(candidate),
                kind: candidate.kind.clone(),
                canonical_document_id: canonical_document_id(&document_id),
                document_id,
                block_id: candidate.node_id.clone(),
                title: candidate.title.clone(),
                relation: candidate.relation.clone(),
                locator_valid: resolved.is_some(),
                locator_matched_by: resolved
                    .map(|location| location.matched_by)
                    .unwrap_or_default(),
                retrieval_channels: trace_channels(&candidate.retrieval_reason),
                rrf_score: trace_number(&candidate.retrieval_reason, "RRF"),
                base_rank: trace_integer(&candidate.retrieval_reason, "base_rank"),
                base_score: trace_number(&candidate.retrieval_reason, "base_score"),
                cross_encoder_score: trace_number(&candidate.retrieval_reason, "cross_encoder"),
                document_repeat_count: trace_integer(
                    &candidate.retrieval_reason,
                    "document_repeat_count",
                ),
                document_repeat_penalty: trace_number(
                    &candidate.retrieval_reason,
                    "document_repeat_penalty",
                ),
            })
        })
        .collect()
}

fn collapse_documents(ranked: &[RankedEvidence]) -> Vec<RankedEvidence> {
    let mut seen = HashSet::new();
    ranked
        .iter()
        .filter(|item| seen.insert(item.canonical_document_id.clone()))
        .cloned()
        .enumerate()
        .map(|(index, mut item)| {
            item.rank = index + 1;
            item
        })
        .collect()
}

fn canonical_document_id(document_id: &str) -> String {
    document_id
        .strip_prefix("wiki:sources/")
        .or_else(|| document_id.strip_prefix("paper:sources/"))
        .map(|source| format!("source:{source}"))
        .unwrap_or_else(|| document_id.to_string())
}

fn recall_at(ranked: &[RankedEvidence], expected: &[String], cutoff: usize) -> f64 {
    if expected.is_empty() {
        return 1.0;
    }
    expected
        .iter()
        .filter(|id| {
            ranked
                .iter()
                .take(cutoff)
                .any(|item| item.document_id == id.as_str())
        })
        .count() as f64
        / expected.len() as f64
}

fn heading_recall(ranked: &[RankedEvidence], expected: &[String], cutoff: usize) -> f64 {
    if expected.is_empty() {
        return 1.0;
    }
    expected
        .iter()
        .filter(|heading| {
            let needle = heading.to_lowercase();
            ranked
                .iter()
                .take(cutoff)
                .any(|item| item.title.to_lowercase().contains(&needle))
        })
        .count() as f64
        / expected.len() as f64
}

#[cfg(test)]
fn reciprocal_rank(ranked: &[RankedEvidence], expected: &[String]) -> f64 {
    ranked
        .iter()
        .position(|item| expected.contains(&item.document_id))
        .map(|index| 1.0 / (index + 1) as f64)
        .unwrap_or_else(|| if expected.is_empty() { 1.0 } else { 0.0 })
}

fn reciprocal_rank_canonical(ranked: &[RankedEvidence], expected: &[String]) -> f64 {
    let expected = expected
        .iter()
        .map(|document| canonical_document_id(document))
        .collect::<HashSet<_>>();
    ranked
        .iter()
        .position(|item| expected.contains(&item.canonical_document_id))
        .map(|index| 1.0 / (index + 1) as f64)
        .unwrap_or_else(|| if expected.is_empty() { 1.0 } else { 0.0 })
}

fn ndcg_at(ranked: &[RankedEvidence], expected: &[String], cutoff: usize) -> f64 {
    if expected.is_empty() {
        return 1.0;
    }
    let mut seen = HashSet::new();
    let dcg = ranked
        .iter()
        .take(cutoff)
        .enumerate()
        .filter(|(_, item)| {
            expected.contains(&item.document_id) && seen.insert(item.document_id.clone())
        })
        .map(|(index, _)| 1.0 / ((index + 2) as f64).log2())
        .sum::<f64>();
    let ideal = (0..expected.len().min(cutoff))
        .map(|index| 1.0 / ((index + 2) as f64).log2())
        .sum::<f64>();
    if ideal == 0.0 {
        1.0
    } else {
        dcg / ideal
    }
}

fn evaluate_case(
    connection: &Connection,
    root: &Path,
    case: &EvaluationCase,
) -> Result<EvaluationCaseResult, String> {
    let conversation = case
        .conversation
        .iter()
        .map(|turn| super::ConversationTurn {
            id: turn.id.clone(),
            role: turn.role.clone(),
            content: turn.content.clone(),
            request_id: turn.request_id.clone(),
        })
        .collect::<Vec<_>>();
    let retrieval_query = super::build_retrieval_query(connection, &case.question, &conversation);
    let resolved_question = retrieval_query.resolved_question;
    let execution_mode = retrieval_query.execution_mode;
    let outcome = super::retrieval::run_retrieval(
        connection,
        root,
        &resolved_question,
        &evaluation_contract(case, &resolved_question, &execution_mode),
        None,
    )?;
    let v2 = project_candidates(connection, root, &outcome.candidates)?;
    let v2_documents = collapse_documents(&v2);
    let legacy = project_candidates(
        connection,
        root,
        &legacy_candidates(connection, root, &resolved_question)?,
    )?;
    let attempted = outcome
        .attempts
        .iter()
        .filter(|attempt| attempt.status != "not_requested")
        .map(|attempt| attempt.kind.clone())
        .collect::<HashSet<_>>();
    let mut attempted_kinds = attempted.iter().cloned().collect::<Vec<_>>();
    attempted_kinds.sort();
    let expected_unresolved_source = case.zero_evidence_expected
        && case.scope_expectation == "sources"
        && !outcome.sources.gaps.is_empty();
    let channel_attempt_rate = if expected_unresolved_source {
        1.0
    } else {
        case.must_attempt_kinds
            .iter()
            .filter(|kind| attempted.contains(*kind))
            .count() as f64
            / case.must_attempt_kinds.len() as f64
    };
    let zero_evidence_observed = !v2.iter().any(|item| item.kind != "graph");
    let expected_items = v2
        .iter()
        .filter(|item| case.expected_documents.contains(&item.document_id))
        .collect::<Vec<_>>();
    let locator_validity = if expected_items.is_empty() {
        if case.locator_required {
            0.0
        } else {
            1.0
        }
    } else {
        expected_items
            .iter()
            .filter(|item| item.locator_valid)
            .count() as f64
            / expected_items.len() as f64
    };
    let source_resolution_correct = case.scope_expectation != "sources"
        || if case.zero_evidence_expected {
            !outcome.sources.gaps.is_empty() && outcome.sources.resolved.is_empty()
        } else {
            outcome.sources.gaps.is_empty()
                && outcome
                    .sources
                    .resolved
                    .iter()
                    .any(|source| case.expected_documents.contains(&source.document_id))
        };
    let recall5 = recall_at(&v2, &case.expected_documents, 5);
    let recall10 = recall_at(&v2, &case.expected_documents, 10);
    let recall20 = recall_at(&v2, &case.expected_documents, 20);
    let heading20 = heading_recall(&v2, &case.expected_headings, 20);
    let mut errors = Vec::new();
    let mut categories = HashSet::new();
    let mut record = |condition: bool, message: &str, category: &str| {
        if condition {
            errors.push(message.to_string());
            categories.insert(category.to_string());
        }
    };
    record(
        !source_resolution_correct,
        "显式来源未正确解析",
        "source_resolution",
    );
    record(
        channel_attempt_rate < 1.0,
        "要求的来源通道未全部尝试",
        "coverage",
    );
    record(
        recall20 < 1.0,
        "Top20 未覆盖全部 expectedDocuments",
        "fusion_or_reranker",
    );
    record(
        heading20 < 1.0,
        "Top20 未覆盖全部 expectedHeadings",
        "chunking_or_retrieval",
    );
    record(
        case.locator_required && locator_validity < 1.0,
        "expected evidence locator 不可用",
        "locator",
    );
    record(
        zero_evidence_observed != case.zero_evidence_expected,
        "零证据分类与预期不一致",
        "zero_evidence",
    );
    record(
        v2.iter()
            .take(20)
            .any(|item| case.forbidden_evidence_kinds.contains(&item.kind)),
        "Top20 包含禁止的证据类型",
        "reranker",
    );
    let v2_docs = v2_documents
        .iter()
        .take(10)
        .map(|item| &item.document_id)
        .collect::<HashSet<_>>();
    let legacy_docs = legacy
        .iter()
        .take(10)
        .map(|item| &item.document_id)
        .collect::<HashSet<_>>();
    let improvements = case
        .expected_documents
        .iter()
        .filter(|document| v2_docs.contains(document) && !legacy_docs.contains(document))
        .cloned()
        .collect();
    let regressions = case
        .expected_documents
        .iter()
        .filter(|document| legacy_docs.contains(document) && !v2_docs.contains(document))
        .cloned()
        .collect();
    let round_count = outcome
        .attempts
        .iter()
        .map(|attempt| attempt.round)
        .max()
        .unwrap_or(0);
    let mut error_categories = categories.into_iter().collect::<Vec<_>>();
    error_categories.sort();
    Ok(EvaluationCaseResult {
        id: case.id.clone(),
        question: case.question.clone(),
        resolved_question,
        execution_mode,
        notes: case.notes.clone(),
        passed: errors.is_empty(),
        errors,
        error_categories,
        source_resolution_correct,
        channel_attempt_rate,
        document_recall_at_5: recall5,
        document_recall_at_10: recall10,
        document_recall_at_20: recall20,
        heading_recall_at_20: heading20,
        passage_mrr: reciprocal_rank_canonical(&v2, &case.expected_documents),
        document_mrr: reciprocal_rank_canonical(&v2_documents, &case.expected_documents),
        mrr: reciprocal_rank_canonical(&v2_documents, &case.expected_documents),
        ndcg_at_10: ndcg_at(&v2, &case.expected_documents, 10),
        locator_validity,
        zero_evidence_observed,
        latency_ms: outcome
            .attempts
            .iter()
            .map(|attempt| attempt.duration_ms)
            .sum(),
        round_count,
        stop_reason: outcome.stop_reason,
        reranker_version: outcome.reranker_version,
        reranker_status: outcome.reranker_status,
        reranker_latency_ms: outcome.reranker_latency_ms,
        reranker_candidate_count: outcome.reranker_candidate_count,
        reranker_batch_size: outcome.reranker_batch_size,
        reranker_batch_count: outcome.reranker_batch_count,
        reranker_model_max_length: outcome.reranker_model_max_length,
        reranker_model_load_ms: outcome.reranker_model_load_ms,
        reranker_input_prepare_ms: outcome.reranker_input_prepare_ms,
        reranker_inference_ms: outcome.reranker_inference_ms,
        reranker_average_input_tokens: outcome.reranker_average_input_tokens,
        reranker_fallback: outcome.reranker_fallback,
        reranker_fallback_reason: outcome.reranker_fallback_reason,
        attempted_kinds,
        source_gaps: outcome.sources.gaps,
        v2_evidence: v2,
        v2_documents,
        legacy_evidence: legacy,
        improvements,
        regressions,
    })
}

fn average(cases: &[EvaluationCaseResult], value: impl Fn(&EvaluationCaseResult) -> f64) -> f64 {
    if cases.is_empty() {
        0.0
    } else {
        cases.iter().map(value).sum::<f64>() / cases.len() as f64
    }
}

pub fn evaluate(
    connection: &Connection,
    root: &Path,
    suite: &EvaluationSuite,
) -> Result<EvaluationReport, String> {
    let cases = suite
        .cases
        .iter()
        .map(|case| evaluate_case(connection, root, case))
        .collect::<Result<Vec<_>, _>>()?;
    let mut category_counts = HashMap::new();
    for case in &cases {
        for category in &case.error_categories {
            *category_counts.entry(category.clone()).or_insert(0) += 1;
        }
    }
    let false_negative = suite
        .cases
        .iter()
        .zip(&cases)
        .filter(|(expected, observed)| {
            expected.zero_evidence_expected && !observed.zero_evidence_observed
        })
        .count();
    let false_positive = suite
        .cases
        .iter()
        .zip(&cases)
        .filter(|(expected, observed)| {
            !expected.zero_evidence_expected && observed.zero_evidence_observed
        })
        .count();
    let reranker_fallback_count = cases.iter().filter(|case| case.reranker_fallback).count();
    let aggregate = EvaluationAggregate {
        case_count: cases.len(),
        passed_count: cases.iter().filter(|case| case.passed).count(),
        source_resolution_accuracy: average(&cases, |case| {
            case.source_resolution_correct as u8 as f64
        }),
        channel_attempt_rate: average(&cases, |case| case.channel_attempt_rate),
        document_recall_at_5: average(&cases, |case| case.document_recall_at_5),
        document_recall_at_10: average(&cases, |case| case.document_recall_at_10),
        document_recall_at_20: average(&cases, |case| case.document_recall_at_20),
        heading_recall_at_20: average(&cases, |case| case.heading_recall_at_20),
        passage_mrr: average(&cases, |case| case.passage_mrr),
        document_mrr: average(&cases, |case| case.document_mrr),
        mrr: average(&cases, |case| case.document_mrr),
        ndcg_at_10: average(&cases, |case| case.ndcg_at_10),
        locator_validity: average(&cases, |case| case.locator_validity),
        zero_evidence_false_negative: false_negative,
        zero_evidence_false_positive: false_positive,
        average_latency_ms: average(&cases, |case| case.latency_ms as f64),
        average_round_count: average(&cases, |case| case.round_count as f64),
        average_reranker_latency_ms: average(&cases, |case| case.reranker_latency_ms as f64),
        average_reranker_model_load_ms: average(&cases, |case| case.reranker_model_load_ms as f64),
        average_reranker_input_prepare_ms: average(&cases, |case| {
            case.reranker_input_prepare_ms as f64
        }),
        average_reranker_inference_ms: average(&cases, |case| case.reranker_inference_ms as f64),
        average_reranker_input_tokens: average(&cases, |case| {
            case.reranker_average_input_tokens as f64
        }),
        reranker_fallback_count,
        reranker_fallback_rate: if cases.is_empty() {
            0.0
        } else {
            reranker_fallback_count as f64 / cases.len() as f64
        },
        error_categories: category_counts,
    };
    let remaining_risks = cases
        .iter()
        .filter(|case| !case.passed)
        .map(|case| format!("{}: {}", case.id, case.errors.join("；")))
        .collect::<Vec<_>>();
    Ok(EvaluationReport {
        schema_version: REPORT_SCHEMA_VERSION.into(),
        suite_name: suite.name.clone(),
        generated_at_unix: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        index_snapshot_id: super::context::index_snapshot_id(connection, root),
        retriever_version: super::context::RETRIEVER_VERSION.into(),
        answer_format: super::natural_answer::ANSWER_FORMAT.into(),
        passed: remaining_risks.is_empty(),
        aggregate,
        cases,
        remaining_risks,
    })
}

fn first_relevant_rank(ranked: &[RankedEvidence], expected: &[String]) -> Option<usize> {
    let expected = expected
        .iter()
        .map(|document| canonical_document_id(document))
        .collect::<HashSet<_>>();
    ranked
        .iter()
        .position(|item| expected.contains(&item.canonical_document_id))
        .map(|index| index + 1)
}

pub fn mrr_diagnostics(report: &EvaluationReport, suite: &EvaluationSuite) -> MrrDiagnosticReport {
    let expected_by_id = suite
        .cases
        .iter()
        .map(|case| (case.id.as_str(), &case.expected_documents))
        .collect::<HashMap<_, _>>();
    let mut cases = report
        .cases
        .iter()
        .map(|case| {
            let expected = expected_by_id
                .get(case.id.as_str())
                .copied()
                .cloned()
                .unwrap_or_default();
            let document_rank_by_id = case
                .v2_documents
                .iter()
                .map(|item| (item.canonical_document_id.as_str(), item.rank))
                .collect::<HashMap<_, _>>();
            let expected_canonical = expected
                .iter()
                .map(|document| canonical_document_id(document))
                .collect::<HashSet<_>>();
            let top_10 = case
                .v2_evidence
                .iter()
                .take(10)
                .map(|item| MrrDiagnosticCandidate {
                    passage_rank: item.rank,
                    document_rank: document_rank_by_id
                        .get(item.canonical_document_id.as_str())
                        .copied()
                        .unwrap_or(item.rank),
                    stable_source_id: item.stable_source_id.clone(),
                    document_id: item.document_id.clone(),
                    canonical_document_id: item.canonical_document_id.clone(),
                    heading: item.title.clone(),
                    is_relevant: expected_canonical.contains(&item.canonical_document_id),
                    retrieval_channels: item.retrieval_channels.clone(),
                    rrf_score: item.rrf_score,
                    base_rank: item.base_rank,
                    base_score: item.base_score,
                    cross_encoder_score: item.cross_encoder_score,
                    final_score: item.score,
                    document_repeat_count: item.document_repeat_count,
                    document_repeat_penalty: item.document_repeat_penalty,
                })
                .collect();
            MrrDiagnosticCase {
                query_id: case.id.clone(),
                question: case.question.clone(),
                first_relevant_passage_rank: first_relevant_rank(&case.v2_evidence, &expected),
                first_relevant_document_rank: first_relevant_rank(&case.v2_documents, &expected),
                passage_mrr: case.passage_mrr,
                document_mrr: case.document_mrr,
                top_10,
            }
        })
        .collect::<Vec<_>>();
    cases.sort_by(|left, right| {
        right
            .first_relevant_document_rank
            .unwrap_or(usize::MAX)
            .cmp(&left.first_relevant_document_rank.unwrap_or(usize::MAX))
            .then_with(|| left.query_id.cmp(&right.query_id))
    });
    MrrDiagnosticReport {
        schema_version: MRR_DIAGNOSTIC_SCHEMA_VERSION.to_string(),
        metric_unit: "document".to_string(),
        passage_metric_is_diagnostic_only: true,
        cases,
    }
}

pub fn write_mrr_diagnostics(
    report: &EvaluationReport,
    suite: &EvaluationSuite,
    path: &Path,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("RAG_EVAL_WRITE_FAILED: {error}"))?;
    }
    let diagnostics = serde_json::to_string_pretty(&mrr_diagnostics(report, suite))
        .map_err(|error| format!("RAG_EVAL_SERIALIZE_FAILED: {error}"))?;
    fs::write(path, format!("{diagnostics}\n"))
        .map_err(|error| format!("RAG_EVAL_WRITE_FAILED: {error}"))
}

pub fn write_report(
    report: &EvaluationReport,
    json_path: &Path,
    markdown_path: &Path,
) -> Result<(), String> {
    for path in [json_path, markdown_path] {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("RAG_EVAL_WRITE_FAILED: {error}"))?;
        }
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| format!("RAG_EVAL_SERIALIZE_FAILED: {error}"))?;
    fs::write(json_path, format!("{json}\n"))
        .map_err(|error| format!("RAG_EVAL_WRITE_FAILED: {error}"))?;
    let aggregate = &report.aggregate;
    let mut markdown = format!(
        "# 科研 RAG 检索评测\n\n- 状态：{}\n- 用例：{}/{}\n- Source resolution accuracy：{:.3}\n- Channel attempt rate：{:.3}\n- Document Recall@5/10/20：{:.3} / {:.3} / {:.3}\n- Heading Recall@20：{:.3}\n- Document MRR / Passage MRR / nDCG@10：{:.3} / {:.3} / {:.3}\n- Locator validity：{:.3}\n- Zero-evidence FN/FP：{} / {}\n- 平均检索耗时：{:.1} ms\n- 平均轮数：{:.2}\n- Reranker fallback：{} / {} ({:.3})\n- 平均 reranker 耗时：{:.1} ms\n- Reranker load/prepare/inference/input tokens：{:.1} / {:.1} / {:.1} ms / {:.1}\n\n## 用例\n\n",
        if report.passed { "PASS" } else { "REVIEW" },
        aggregate.passed_count,
        aggregate.case_count,
        aggregate.source_resolution_accuracy,
        aggregate.channel_attempt_rate,
        aggregate.document_recall_at_5,
        aggregate.document_recall_at_10,
        aggregate.document_recall_at_20,
        aggregate.heading_recall_at_20,
        aggregate.document_mrr,
        aggregate.passage_mrr,
        aggregate.ndcg_at_10,
        aggregate.locator_validity,
        aggregate.zero_evidence_false_negative,
        aggregate.zero_evidence_false_positive,
        aggregate.average_latency_ms,
        aggregate.average_round_count,
        aggregate.reranker_fallback_count,
        aggregate.case_count,
        aggregate.reranker_fallback_rate,
        aggregate.average_reranker_latency_ms,
        aggregate.average_reranker_model_load_ms,
        aggregate.average_reranker_input_prepare_ms,
        aggregate.average_reranker_inference_ms,
        aggregate.average_reranker_input_tokens,
    );
    for case in &report.cases {
        markdown.push_str(&format!(
            "### {} · {}\n\n- 状态：{}\n- 通道：{}\n- Stop：{}\n- Recall@5/20：{:.3} / {:.3}\n- Document/Passage MRR：{:.3} / {:.3}\n- Locator：{:.3}\n- Reranker：{} / {} / {} ms{}\n",
            case.id,
            case.question,
            if case.passed { "PASS" } else { "REVIEW" },
            case.attempted_kinds.join(", "),
            case.stop_reason,
            case.document_recall_at_5,
            case.document_recall_at_20,
            case.document_mrr,
            case.passage_mrr,
            case.locator_validity,
            case.reranker_version,
            case.reranker_status,
            case.reranker_latency_ms,
            if case.reranker_fallback_reason.is_empty() {
                String::new()
            } else {
                format!(" / fallback={}", case.reranker_fallback_reason)
            },
        ));
        if !case.improvements.is_empty() {
            markdown.push_str(&format!("- v2 改善：{}\n", case.improvements.join(", ")));
        }
        if !case.regressions.is_empty() {
            markdown.push_str(&format!("- v2 退化：{}\n", case.regressions.join(", ")));
        }
        if !case.errors.is_empty() {
            markdown.push_str(&format!("- 问题：{}\n", case.errors.join("；")));
        }
        markdown.push('\n');
    }
    if !report.remaining_risks.is_empty() {
        markdown.push_str("## 剩余风险\n\n");
        for risk in &report.remaining_risks {
            markdown.push_str(&format!("- {risk}\n"));
        }
    }
    fs::write(markdown_path, markdown).map_err(|error| format!("RAG_EVAL_WRITE_FAILED: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ranked(rank: usize, document_id: &str, block_id: &str) -> RankedEvidence {
        RankedEvidence {
            rank,
            score: 1.0 / rank as f64,
            stable_source_id: block_id.into(),
            kind: "paper".into(),
            document_id: document_id.into(),
            canonical_document_id: canonical_document_id(document_id),
            block_id: block_id.into(),
            title: block_id.into(),
            relation: "content_block_v2".into(),
            locator_valid: true,
            locator_matched_by: "block".into(),
            retrieval_channels: vec!["fts".into()],
            rrf_score: Some(0.01),
            base_rank: Some(rank),
            base_score: Some(0.01),
            cross_encoder_score: Some(0.5),
            document_repeat_count: Some(0),
            document_repeat_penalty: Some(0.0),
        }
    }

    fn case(id: &str) -> EvaluationCase {
        EvaluationCase {
            id: id.into(),
            question: "测试问题".into(),
            conversation: Vec::new(),
            aliases: Vec::new(),
            explicit_sources: Vec::new(),
            scope_expectation: "open".into(),
            must_attempt_kinds: vec!["wiki".into()],
            expected_documents: vec!["wiki:test".into()],
            expected_headings: Vec::new(),
            forbidden_evidence_kinds: Vec::new(),
            locator_required: true,
            zero_evidence_expected: false,
            notes: String::new(),
        }
    }

    #[test]
    fn strict_case_contract_rejects_duplicates_and_invalid_zero_evidence() {
        let mut suite = EvaluationSuite {
            schema_version: CASE_SCHEMA_VERSION.into(),
            name: "suite".into(),
            cases: (0..8).map(|index| case(&format!("case-{index}"))).collect(),
        };
        suite.cases[1].id = suite.cases[0].id.clone();
        assert!(validate_suite(&suite).unwrap_err().contains("重复"));
        suite.cases[1].id = "case-1".into();
        suite.cases[0].zero_evidence_expected = true;
        assert!(validate_suite(&suite).unwrap_err().contains("零证据"));
        assert!(serde_json::from_str::<EvaluationSuite>(
            r#"{"schemaVersion":"qa-rag-evaluation-cases-v1","name":"x","cases":[],"unknown":true}"#,
        )
        .is_err());
    }

    #[test]
    fn document_mrr_collapses_duplicate_passages_before_ranking() {
        let passages = vec![
            ranked(1, "paper:noise", "noise-1"),
            ranked(2, "paper:noise", "noise-2"),
            ranked(3, "paper:target", "target-1"),
        ];
        let documents = collapse_documents(&passages);
        assert_eq!(
            documents
                .iter()
                .map(|item| (item.rank, item.document_id.as_str()))
                .collect::<Vec<_>>(),
            vec![(1, "paper:noise"), (2, "paper:target")]
        );
        assert_eq!(
            reciprocal_rank(&passages, &["paper:target".into()]),
            1.0 / 3.0
        );
        assert_eq!(reciprocal_rank(&documents, &["paper:target".into()]), 0.5);
    }

    #[test]
    fn canonical_document_mrr_treats_wiki_source_and_primary_paper_as_one_work() {
        let ranked = vec![
            ranked(1, "wiki:sources/src-target", "wiki-target"),
            ranked(2, "paper:sources/src-target", "paper-target"),
        ];
        let documents = collapse_documents(&ranked);
        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].canonical_document_id, "source:src-target");
        assert_eq!(
            reciprocal_rank_canonical(&ranked, &["paper:sources/src-target".into()]),
            1.0
        );
    }
}
