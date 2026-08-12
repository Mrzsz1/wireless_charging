use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RetrievalChannelDiagnostic {
    pub name: String,
    pub duration_ms: u64,
    pub candidate_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RetrievalDiagnostics {
    pub total_ms: u64,
    pub channels: Vec<RetrievalChannelDiagnostic>,
    pub selected_count: usize,
    pub cancel_check_count: usize,
    #[serde(default)]
    pub pass_count: usize,
    #[serde(default)]
    pub stop_reason: String,
    #[serde(default)]
    pub candidate_gains: Vec<usize>,
}

pub(super) struct RetrievalDiagnosticsBuilder {
    started_at: Instant,
    channels: Vec<RetrievalChannelDiagnostic>,
    cancel_check_count: usize,
    candidate_gains: Vec<usize>,
    stop_reason: String,
}

impl RetrievalDiagnosticsBuilder {
    pub(super) fn new() -> Self {
        Self {
            started_at: Instant::now(),
            channels: Vec::new(),
            cancel_check_count: 0,
            candidate_gains: Vec::new(),
            stop_reason: String::new(),
        }
    }

    pub(super) fn record(&mut self, name: &str, started_at: Instant, candidate_count: usize) {
        self.channels.push(RetrievalChannelDiagnostic {
            name: name.to_string(),
            duration_ms: elapsed_ms(started_at),
            candidate_count,
        });
    }

    pub(super) fn add_cancel_checks(&mut self, count: usize) {
        self.cancel_check_count += count;
    }

    pub(super) fn record_pass(&mut self, candidate_gain: usize) {
        self.candidate_gains.push(candidate_gain);
    }

    pub(super) fn stop(&mut self, reason: &str) {
        self.stop_reason = reason.to_string();
    }

    pub(super) fn finish(self, selected_count: usize) -> RetrievalDiagnostics {
        RetrievalDiagnostics {
            total_ms: elapsed_ms(self.started_at),
            channels: self.channels,
            selected_count,
            cancel_check_count: self.cancel_check_count,
            pass_count: self.candidate_gains.len(),
            stop_reason: self.stop_reason,
            candidate_gains: self.candidate_gains,
        }
    }
}

fn elapsed_ms(started_at: Instant) -> u64 {
    started_at.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
}

#[derive(Debug, Clone, Default)]
#[cfg(test)]
pub struct RetrievalRankingObservation {
    pub ranked_ids: Vec<String>,
    pub relevant_ids: Vec<String>,
    pub required_kind_covered: bool,
    pub pair_covered: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg(test)]
pub struct RetrievalQualityMetrics {
    pub recall_at_5: f64,
    pub recall_at_10: f64,
    pub recall_at_20: f64,
    pub mrr: f64,
    pub ndcg_at_10: f64,
    pub required_kind_coverage: f64,
    pub pair_coverage: f64,
}

#[cfg(test)]
pub fn evaluate_retrieval_quality(
    observations: &[RetrievalRankingObservation],
) -> RetrievalQualityMetrics {
    if observations.is_empty() {
        return RetrievalQualityMetrics::default();
    }
    let count = observations.len() as f64;
    let average_recall = |cutoff| {
        observations
            .iter()
            .map(|observation| recall_at(observation, cutoff))
            .sum::<f64>()
            / count
    };
    RetrievalQualityMetrics {
        recall_at_5: average_recall(5),
        recall_at_10: average_recall(10),
        recall_at_20: average_recall(20),
        mrr: observations.iter().map(reciprocal_rank).sum::<f64>() / count,
        ndcg_at_10: observations
            .iter()
            .map(|observation| ndcg_at(observation, 10))
            .sum::<f64>()
            / count,
        required_kind_coverage: observations
            .iter()
            .filter(|observation| observation.required_kind_covered)
            .count() as f64
            / count,
        pair_coverage: observations
            .iter()
            .filter(|observation| observation.pair_covered)
            .count() as f64
            / count,
    }
}

#[cfg(test)]
fn relevant_set(observation: &RetrievalRankingObservation) -> HashSet<&str> {
    observation
        .relevant_ids
        .iter()
        .map(String::as_str)
        .collect()
}

#[cfg(test)]
fn recall_at(observation: &RetrievalRankingObservation, cutoff: usize) -> f64 {
    let relevant = relevant_set(observation);
    if relevant.is_empty() {
        return 1.0;
    }
    let hits = observation
        .ranked_ids
        .iter()
        .take(cutoff)
        .filter(|id| relevant.contains(id.as_str()))
        .collect::<HashSet<_>>()
        .len();
    hits as f64 / relevant.len() as f64
}

#[cfg(test)]
fn reciprocal_rank(observation: &RetrievalRankingObservation) -> f64 {
    let relevant = relevant_set(observation);
    observation
        .ranked_ids
        .iter()
        .position(|id| relevant.contains(id.as_str()))
        .map(|index| 1.0 / (index + 1) as f64)
        .unwrap_or(0.0)
}

#[cfg(test)]
fn ndcg_at(observation: &RetrievalRankingObservation, cutoff: usize) -> f64 {
    let relevant = relevant_set(observation);
    if relevant.is_empty() {
        return 1.0;
    }
    let dcg = observation
        .ranked_ids
        .iter()
        .take(cutoff)
        .enumerate()
        .filter(|(_, id)| relevant.contains(id.as_str()))
        .map(|(index, _)| 1.0 / ((index + 2) as f64).log2())
        .sum::<f64>();
    let ideal_hits = relevant.len().min(cutoff);
    let idcg = (0..ideal_hits)
        .map(|index| 1.0 / ((index + 2) as f64).log2())
        .sum::<f64>();
    if idcg == 0.0 {
        1.0
    } else {
        dcg / idcg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranking_metrics_measure_order_recall_and_coverage() {
        let metrics = evaluate_retrieval_quality(&[RetrievalRankingObservation {
            ranked_ids: vec!["noise".into(), "relevant-a".into(), "relevant-b".into()],
            relevant_ids: vec!["relevant-a".into(), "relevant-b".into()],
            required_kind_covered: true,
            pair_covered: false,
        }]);
        assert_eq!(metrics.recall_at_5, 1.0);
        assert_eq!(metrics.mrr, 0.5);
        assert!(metrics.ndcg_at_10 > 0.6 && metrics.ndcg_at_10 < 1.0);
        assert_eq!(metrics.required_kind_coverage, 1.0);
        assert_eq!(metrics.pair_coverage, 0.0);
    }

    #[test]
    fn retrieval_diagnostics_expose_only_aggregate_metadata() {
        let mut builder = RetrievalDiagnosticsBuilder::new();
        builder.record("wiki", Instant::now(), 3);
        builder.add_cancel_checks(4);
        let payload = serde_json::to_string(&builder.finish(2)).expect("diagnostics JSON");
        assert!(payload.contains("candidateCount"));
        assert!(payload.contains("cancelCheckCount"));
        for prohibited in ["query", "question", "path", "snippet", "token", "secret"] {
            assert!(!payload.to_lowercase().contains(prohibited));
        }
    }
}
