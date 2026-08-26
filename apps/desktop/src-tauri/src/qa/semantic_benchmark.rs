use super::{
    routing_policy, EvidenceItem, LlmBudgetGuard, LunaSettings, SemanticVerificationBatch,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicBool;

pub const BENCHMARK_SCHEMA_VERSION: &str = "qa-semantic-verifier-benchmark-v1";
pub const REPORT_SCHEMA_VERSION: &str = "qa-semantic-verifier-report-v1";
const BATCH_SIZE: usize = 20;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BenchmarkCase {
    pub id: String,
    pub category: String,
    pub claim: String,
    pub evidence: Vec<String>,
    pub gold: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BenchmarkSuite {
    pub schema_version: String,
    pub dataset_role: String,
    pub status: String,
    pub version: String,
    pub case_count: usize,
    pub cases_sha256: String,
    pub cases: Vec<BenchmarkCase>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkCaseResult {
    pub id: String,
    pub category: String,
    pub gold: String,
    pub predicted: String,
    pub correct: bool,
    pub status: String,
    pub fallback_reason: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticBenchmarkReport {
    pub schema_version: String,
    pub dataset_version: String,
    pub dataset_sha256: String,
    pub provider: String,
    pub model: String,
    pub real_provider_measured: bool,
    pub case_count: usize,
    pub completed_case_count: usize,
    pub accuracy: f64,
    pub contradiction_recall: f64,
    pub unknown_precision: f64,
    pub timeout_rate: f64,
    pub invalid_json_rate: f64,
    pub fallback_rate: f64,
    pub invalid_verified_state_count: usize,
    pub total_latency_ms: u64,
    pub results: Vec<BenchmarkCaseResult>,
}

fn cases_sha256(cases: &[BenchmarkCase]) -> Result<String, String> {
    let bytes = serde_json::to_vec(cases)
        .map_err(|error| format!("SEMANTIC_BENCHMARK_INVALID: {error}"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn load_suite(path: &Path) -> Result<BenchmarkSuite, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("SEMANTIC_BENCHMARK_READ_FAILED: {error}"))?;
    let suite = serde_json::from_str::<BenchmarkSuite>(&raw)
        .map_err(|error| format!("SEMANTIC_BENCHMARK_INVALID: {error}"))?;
    if suite.schema_version != BENCHMARK_SCHEMA_VERSION
        || suite.dataset_role != "model_independent_semantic_verification"
        || suite.status != "frozen"
        || suite.case_count != suite.cases.len()
        || suite.cases.len() < 100
    {
        return Err("SEMANTIC_BENCHMARK_INVALID: schema_or_count".to_string());
    }
    let mut ids = HashSet::new();
    for case in &suite.cases {
        if case.id.trim().is_empty()
            || !ids.insert(case.id.clone())
            || case.claim.trim().is_empty()
            || case.evidence.is_empty()
            || case.evidence.iter().any(|item| item.trim().is_empty())
            || !matches!(case.gold.as_str(), "entailed" | "contradicted" | "unknown")
        {
            return Err(format!("SEMANTIC_BENCHMARK_INVALID: case={}", case.id));
        }
    }
    if cases_sha256(&suite.cases)? != suite.cases_sha256 {
        return Err("SEMANTIC_BENCHMARK_INVALID: cases_sha256".to_string());
    }
    Ok(suite)
}

fn evidence(id: String, snippet: String, rank: usize) -> EvidenceItem {
    EvidenceItem {
        id,
        kind: "paper".to_string(),
        tier: "primary_source".to_string(),
        title: "Frozen semantic benchmark evidence".to_string(),
        snippet,
        score: 1.0,
        rank,
        page_id: "semantic-benchmark".to_string(),
        page_type: "source".to_string(),
        source_path: String::new(),
        wikilink: String::new(),
        book_id: String::new(),
        chapter_id: String::new(),
        physical_page_start: None,
        physical_page_end: None,
        markdown_path: String::new(),
        pdf_path: String::new(),
        node_id: String::new(),
        source_location: "frozen-fixture".to_string(),
        relation: "semantic_benchmark".to_string(),
        retrieval_reason: "frozen_claim_evidence_pair".to_string(),
        locator: None,
    }
}

fn run_batch(
    settings: &LunaSettings,
    model: &str,
    reasoning_effort: &str,
    cases: &[BenchmarkCase],
) -> Result<SemanticVerificationBatch, String> {
    let mut next_evidence = 1usize;
    let mut evidence_items = Vec::new();
    let mut answer_lines = Vec::new();
    for case in cases {
        let mut citations = Vec::new();
        for snippet in &case.evidence {
            let id = format!("E{next_evidence}");
            next_evidence += 1;
            citations.push(format!("[{id}]"));
            evidence_items.push(evidence(id, snippet.clone(), evidence_items.len() + 1));
        }
        answer_lines.push(format!("- {} {}", case.claim, citations.join(" ")));
    }
    let guard = LlmBudgetGuard::new(routing_policy("exploratory"));
    super::run_semantic_verification(
        settings,
        model,
        reasoning_effort,
        &answer_lines.join("\n"),
        &evidence_items,
        &guard,
        &AtomicBool::new(false),
    )
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

pub fn evaluate(
    suite: &BenchmarkSuite,
    settings: &LunaSettings,
    model: &str,
    reasoning_effort: &str,
) -> Result<SemanticBenchmarkReport, String> {
    let mut results = Vec::with_capacity(suite.cases.len());
    let mut total_latency_ms = 0u64;
    for cases in suite.cases.chunks(BATCH_SIZE) {
        let batch = run_batch(settings, model, reasoning_effort, cases)?;
        total_latency_ms = total_latency_ms.saturating_add(batch.latency_ms);
        let predictions = batch
            .results
            .iter()
            .map(|result| format!("{:?}", result.status).to_ascii_lowercase())
            .collect::<Vec<_>>();
        for (index, case) in cases.iter().enumerate() {
            let predicted = predictions.get(index).cloned().unwrap_or_default();
            results.push(BenchmarkCaseResult {
                id: case.id.clone(),
                category: case.category.clone(),
                gold: case.gold.clone(),
                correct: predicted == case.gold,
                predicted,
                status: batch.status.clone(),
                fallback_reason: batch.fallback_reason.clone(),
            });
        }
    }
    let completed = results
        .iter()
        .filter(|result| result.status == "succeeded" && !result.predicted.is_empty())
        .count();
    let correct = results.iter().filter(|result| result.correct).count();
    let gold_contradicted = results
        .iter()
        .filter(|result| result.gold == "contradicted")
        .count();
    let recalled_contradictions = results
        .iter()
        .filter(|result| result.gold == "contradicted" && result.predicted == "contradicted")
        .count();
    let predicted_unknown = results
        .iter()
        .filter(|result| result.predicted == "unknown")
        .count();
    let correct_unknown = results
        .iter()
        .filter(|result| result.gold == "unknown" && result.predicted == "unknown")
        .count();
    let failures = results
        .iter()
        .filter(|result| result.status != "succeeded")
        .count();
    let timeout = results
        .iter()
        .filter(|result| result.fallback_reason.contains("timeout"))
        .count();
    let invalid = results
        .iter()
        .filter(|result| result.fallback_reason.contains("invalid"))
        .count();
    let provider = results
        .first()
        .map(|_| settings.answer_provider.clone())
        .unwrap_or_default();
    let case_count = results.len();
    let real_provider_measured = case_count == suite.case_count
        && completed == case_count
        && matches!(
            provider.as_str(),
            super::PROVIDER_CODEX | super::PROVIDER_API
        );
    Ok(SemanticBenchmarkReport {
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        dataset_version: suite.version.clone(),
        dataset_sha256: suite.cases_sha256.clone(),
        provider,
        model: model.to_string(),
        real_provider_measured,
        case_count,
        completed_case_count: completed,
        accuracy: ratio(correct, case_count),
        contradiction_recall: ratio(recalled_contradictions, gold_contradicted),
        unknown_precision: ratio(correct_unknown, predicted_unknown),
        timeout_rate: ratio(timeout, case_count),
        invalid_json_rate: ratio(invalid, case_count),
        fallback_rate: ratio(failures, case_count),
        invalid_verified_state_count: results
            .iter()
            .filter(|result| result.status == "succeeded" && result.predicted.is_empty())
            .count(),
        total_latency_ms,
        results,
    })
}

pub fn write_report(report: &SemanticBenchmarkReport, output: &Path) -> Result<(), String> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("SEMANTIC_BENCHMARK_WRITE_FAILED: {error}"))?;
    }
    let part = output.with_extension(format!(
        "{}.part",
        output
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("json")
    ));
    let bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("SEMANTIC_BENCHMARK_SERIALIZE_FAILED: {error}"))?;
    fs::write(&part, [bytes, b"\n".to_vec()].concat())
        .map_err(|error| format!("SEMANTIC_BENCHMARK_WRITE_FAILED: {error}"))?;
    fs::rename(&part, output).map_err(|error| format!("SEMANTIC_BENCHMARK_WRITE_FAILED: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_suite_rejects_tampered_hash() {
        let cases = (0..100)
            .map(|index| BenchmarkCase {
                id: format!("case-{index}"),
                category: "direct".into(),
                claim: "The claim is supported.".into(),
                evidence: vec!["The claim is supported.".into()],
                gold: "entailed".into(),
            })
            .collect::<Vec<_>>();
        let suite = BenchmarkSuite {
            schema_version: BENCHMARK_SCHEMA_VERSION.into(),
            dataset_role: "model_independent_semantic_verification".into(),
            status: "frozen".into(),
            version: "fixture".into(),
            case_count: cases.len(),
            cases_sha256: cases_sha256(&cases).unwrap(),
            cases,
        };
        assert_eq!(suite.cases_sha256.len(), 64);
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("suite.json");
        fs::write(&path, serde_json::to_vec(&suite).unwrap()).unwrap();
        assert_eq!(load_suite(&path).unwrap().case_count, 100);
        let mut tampered = suite;
        tampered.cases[0].claim.push_str(" tampered");
        fs::write(&path, serde_json::to_vec(&tampered).unwrap()).unwrap();
        assert!(load_suite(&path).unwrap_err().contains("cases_sha256"));
    }
}
