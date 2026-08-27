use super::{
    routing_policy, EvidenceItem, LlmBudgetGuard, LunaSettings, SemanticVerificationBatch,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;
use std::sync::atomic::AtomicBool;

pub const BENCHMARK_SCHEMA_VERSION: &str = "qa-semantic-verifier-benchmark-v1";
pub const BENCHMARK_SCHEMA_VERSION_V2: &str = "qa-semantic-verifier-benchmark-v2";
pub const REPORT_SCHEMA_VERSION: &str = "qa-semantic-verifier-report-v1";
pub const REPORT_SCHEMA_VERSION_V2: &str = "qa-semantic-verifier-report-v2";
const BATCH_SIZE: usize = 20;
const LABELS: [&str; 3] = ["entailed", "contradicted", "unknown"];

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
    #[serde(default)]
    pub label_distribution: BTreeMap<String, usize>,
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
    pub latency_ms: u64,
    pub provider: String,
    pub fallback: bool,
    pub fallback_reason: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryMetrics {
    pub count: usize,
    pub accuracy: f64,
    pub gold_distribution: BTreeMap<String, usize>,
    pub prediction_distribution: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FailedCase {
    pub id: String,
    pub category: String,
    pub gold: String,
    pub predicted: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticBenchmarkReport {
    pub schema_version: String,
    pub dataset_version: String,
    pub dataset_sha256: String,
    pub provider: String,
    pub model: String,
    pub reasoning_effort: String,
    pub batch_size: usize,
    pub case_latency_allocation: String,
    pub real_provider_measured: bool,
    pub case_count: usize,
    pub completed_case_count: usize,
    pub accuracy: f64,
    pub overall_accuracy: f64,
    pub entailed_precision: f64,
    pub entailed_recall: f64,
    pub contradiction_precision: f64,
    pub contradiction_recall: f64,
    pub unknown_precision: f64,
    pub unknown_recall: f64,
    pub macro_f1: f64,
    pub confusion_matrix: BTreeMap<String, BTreeMap<String, usize>>,
    pub category_metrics: BTreeMap<String, CategoryMetrics>,
    pub failed_cases: Vec<FailedCase>,
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
    let contract_valid = match suite.schema_version.as_str() {
        BENCHMARK_SCHEMA_VERSION => {
            suite.dataset_role == "model_independent_semantic_verification"
                && suite.cases.len() >= 100
        }
        BENCHMARK_SCHEMA_VERSION_V2 => {
            suite.dataset_role == "development_regression_semantic_verification"
                && suite.cases.len() == 60
        }
        _ => false,
    };
    if !contract_valid || suite.status != "frozen" || suite.case_count != suite.cases.len() {
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
    if suite.schema_version == BENCHMARK_SCHEMA_VERSION_V2 {
        let actual = label_distribution(&suite.cases);
        let expected = LABELS
            .iter()
            .map(|label| ((*label).to_string(), 20usize))
            .collect::<BTreeMap<_, _>>();
        if suite.label_distribution != expected || actual != expected {
            return Err("SEMANTIC_BENCHMARK_INVALID: label_distribution".to_string());
        }
    }
    Ok(suite)
}

fn label_distribution(cases: &[BenchmarkCase]) -> BTreeMap<String, usize> {
    let mut distribution = zero_distribution();
    for case in cases {
        *distribution.entry(case.gold.clone()).or_default() += 1;
    }
    distribution
}

fn zero_distribution() -> BTreeMap<String, usize> {
    LABELS
        .iter()
        .map(|label| ((*label).to_string(), 0usize))
        .collect()
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

fn class_precision_recall(results: &[BenchmarkCaseResult], label: &str) -> (f64, f64) {
    let true_positive = results
        .iter()
        .filter(|result| result.gold == label && result.predicted == label)
        .count();
    let predicted = results
        .iter()
        .filter(|result| result.predicted == label)
        .count();
    let gold = results.iter().filter(|result| result.gold == label).count();
    (ratio(true_positive, predicted), ratio(true_positive, gold))
}

fn f1(precision: f64, recall: f64) -> f64 {
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * precision * recall / (precision + recall)
    }
}

fn confusion_matrix(results: &[BenchmarkCaseResult]) -> BTreeMap<String, BTreeMap<String, usize>> {
    let mut matrix = LABELS
        .iter()
        .map(|gold| ((*gold).to_string(), zero_distribution()))
        .collect::<BTreeMap<_, _>>();
    for result in results {
        if LABELS.contains(&result.predicted.as_str()) {
            let Some(row) = matrix.get_mut(&result.gold) else {
                continue;
            };
            *row.entry(result.predicted.clone()).or_default() += 1;
        }
    }
    matrix
}

fn category_metrics(results: &[BenchmarkCaseResult]) -> BTreeMap<String, CategoryMetrics> {
    let mut grouped = BTreeMap::<String, Vec<&BenchmarkCaseResult>>::new();
    for result in results {
        grouped
            .entry(result.category.clone())
            .or_default()
            .push(result);
    }
    grouped
        .into_iter()
        .map(|(category, items)| {
            let mut gold_distribution = zero_distribution();
            let mut prediction_distribution = zero_distribution();
            for item in &items {
                *gold_distribution.entry(item.gold.clone()).or_default() += 1;
                *prediction_distribution
                    .entry(item.predicted.clone())
                    .or_default() += 1;
            }
            let correct = items.iter().filter(|item| item.correct).count();
            (
                category,
                CategoryMetrics {
                    count: items.len(),
                    accuracy: ratio(correct, items.len()),
                    gold_distribution,
                    prediction_distribution,
                },
            )
        })
        .collect()
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
        let per_case_latency = batch.latency_ms / cases.len() as u64;
        let latency_remainder = batch.latency_ms % cases.len() as u64;
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
                latency_ms: per_case_latency + u64::from((index as u64) < latency_remainder),
                provider: settings.answer_provider.clone(),
                fallback: batch.status != "succeeded" || !batch.fallback_reason.is_empty(),
                fallback_reason: batch.fallback_reason.clone(),
            });
        }
    }
    let completed = results
        .iter()
        .filter(|result| result.status == "succeeded" && !result.predicted.is_empty())
        .count();
    let correct = results.iter().filter(|result| result.correct).count();
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
    let accuracy = ratio(correct, case_count);
    let (entailed_precision, entailed_recall) = class_precision_recall(&results, "entailed");
    let (contradiction_precision, contradiction_recall) =
        class_precision_recall(&results, "contradicted");
    let (unknown_precision, unknown_recall) = class_precision_recall(&results, "unknown");
    let macro_f1 = (f1(entailed_precision, entailed_recall)
        + f1(contradiction_precision, contradiction_recall)
        + f1(unknown_precision, unknown_recall))
        / 3.0;
    let failed_cases = results
        .iter()
        .filter(|result| !result.correct)
        .map(|result| FailedCase {
            id: result.id.clone(),
            category: result.category.clone(),
            gold: result.gold.clone(),
            predicted: result.predicted.clone(),
        })
        .collect();
    Ok(SemanticBenchmarkReport {
        schema_version: if suite.schema_version == BENCHMARK_SCHEMA_VERSION_V2 {
            REPORT_SCHEMA_VERSION_V2
        } else {
            REPORT_SCHEMA_VERSION
        }
        .to_string(),
        dataset_version: suite.version.clone(),
        dataset_sha256: suite.cases_sha256.clone(),
        provider,
        model: model.to_string(),
        reasoning_effort: reasoning_effort.to_string(),
        batch_size: BATCH_SIZE,
        case_latency_allocation: "batch_total_evenly_allocated".to_string(),
        real_provider_measured,
        case_count,
        completed_case_count: completed,
        accuracy,
        overall_accuracy: accuracy,
        entailed_precision,
        entailed_recall,
        contradiction_precision,
        contradiction_recall,
        unknown_precision,
        unknown_recall,
        macro_f1,
        confusion_matrix: confusion_matrix(&results),
        category_metrics: category_metrics(&results),
        failed_cases,
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
            label_distribution: BTreeMap::new(),
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

    #[test]
    fn frozen_v2_suite_requires_sixty_balanced_cases() {
        let cases = (0..60)
            .map(|index| BenchmarkCase {
                id: format!("SV2-{:03}", index + 1),
                category: format!("category-{}", index % 4),
                claim: format!("claim-{index}"),
                evidence: vec![format!("evidence-{index}")],
                gold: LABELS[index / 20].into(),
            })
            .collect::<Vec<_>>();
        let suite = BenchmarkSuite {
            schema_version: BENCHMARK_SCHEMA_VERSION_V2.into(),
            dataset_role: "development_regression_semantic_verification".into(),
            status: "frozen".into(),
            version: "semantic-v2-fixture".into(),
            case_count: cases.len(),
            cases_sha256: cases_sha256(&cases).unwrap(),
            label_distribution: label_distribution(&cases),
            cases,
        };
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("suite-v2.json");
        fs::write(&path, serde_json::to_vec(&suite).unwrap()).unwrap();
        assert_eq!(load_suite(&path).unwrap().case_count, 60);

        let mut invalid = suite;
        invalid.label_distribution.insert("unknown".into(), 19);
        fs::write(&path, serde_json::to_vec(&invalid).unwrap()).unwrap();
        assert!(load_suite(&path)
            .unwrap_err()
            .contains("label_distribution"));
    }

    #[test]
    fn v2_metrics_expose_precision_recall_confusion_and_categories() {
        let result = |id: &str, category: &str, gold: &str, predicted: &str| BenchmarkCaseResult {
            id: id.into(),
            category: category.into(),
            gold: gold.into(),
            predicted: predicted.into(),
            correct: gold == predicted,
            status: "succeeded".into(),
            latency_ms: 10,
            provider: "fixture".into(),
            fallback: false,
            fallback_reason: String::new(),
        };
        let results = vec![
            result("1", "scope", "entailed", "entailed"),
            result("2", "scope", "contradicted", "unknown"),
            result("3", "causal", "unknown", "unknown"),
        ];
        assert_eq!(class_precision_recall(&results, "entailed"), (1.0, 1.0));
        assert_eq!(class_precision_recall(&results, "contradicted"), (0.0, 0.0));
        assert_eq!(class_precision_recall(&results, "unknown"), (0.5, 1.0));
        assert_eq!(confusion_matrix(&results)["contradicted"]["unknown"], 1);
        assert_eq!(category_metrics(&results)["scope"].count, 2);
        assert_eq!(results.iter().map(|item| item.latency_ms).sum::<u64>(), 30);
    }

    #[test]
    fn bundled_semantic_suites_are_sealed_and_v2_is_balanced() {
        let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let v1 = load_suite(&repository.join("evals/semantic_verification_real_cases.json"))
            .expect("sealed v1 suite");
        let v2 = load_suite(&repository.join("evals/semantic_verification_v2_cases.json"))
            .expect("sealed v2 suite");
        assert_eq!(v1.case_count, 100);
        assert_eq!(v1.version, "2026-08-27-semantic-v1.1");
        assert_eq!(v2.case_count, 60);
        assert_eq!(v2.label_distribution["entailed"], 20);
        assert_eq!(v2.label_distribution["contradicted"], 20);
        assert_eq!(v2.label_distribution["unknown"], 20);
    }
}
