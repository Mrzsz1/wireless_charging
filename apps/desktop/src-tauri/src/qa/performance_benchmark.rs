use super::evaluation::{self, EvaluationReport, EvaluationSuite};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub const PROFILE_SCHEMA_VERSION: &str = "qa-target-performance-profile-v1";
pub const REPORT_SCHEMA_VERSION: &str = "qa-reranker-performance-report-v1";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModeSlo {
    pub warm_p95_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PerformanceProfile {
    pub schema_version: String,
    pub sealed: bool,
    pub version: String,
    pub machine_profile: String,
    pub model: String,
    pub warmup_runs: usize,
    pub measured_runs: usize,
    pub minimum_samples_per_mode: usize,
    pub cold_model_load_max_ms: u64,
    pub modes: HashMap<String, ModeSlo>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Percentiles {
    pub samples: usize,
    pub p50_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,
    pub max_p95_latency_ms: u64,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceReport {
    pub schema_version: String,
    pub profile_version: String,
    pub machine_profile: String,
    pub model: String,
    pub target_profile_frozen: bool,
    pub measured: bool,
    pub all_mode_slos_passed: bool,
    pub cold_model_load_ms: u64,
    pub cold_model_load_max_ms: u64,
    pub p95_latency_ms: u64,
    pub max_p95_latency_ms: u64,
    pub average_input_prepare_ms: f64,
    pub average_inference_ms: f64,
    pub average_input_tokens: f64,
    pub modes: HashMap<String, Percentiles>,
}

pub fn load_profile(path: &Path) -> Result<PerformanceProfile, String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| format!("PERFORMANCE_PROFILE_READ_FAILED: {error}"))?;
    let profile = serde_json::from_str::<PerformanceProfile>(&raw)
        .map_err(|error| format!("PERFORMANCE_PROFILE_INVALID: {error}"))?;
    if profile.schema_version != PROFILE_SCHEMA_VERSION
        || !profile.sealed
        || profile.version.trim().is_empty()
        || profile.machine_profile.trim().is_empty()
        || profile.model.trim().is_empty()
        || profile.warmup_runs == 0
        || profile.measured_runs == 0
        || profile.minimum_samples_per_mode == 0
        || profile.cold_model_load_max_ms == 0
        || profile
            .modes
            .keys()
            .map(String::as_str)
            .collect::<std::collections::HashSet<_>>()
            != std::collections::HashSet::from(["direct", "research", "exploratory"])
        || profile.modes.values().any(|mode| mode.warm_p95_ms == 0)
    {
        return Err("PERFORMANCE_PROFILE_INVALID: frozen_fields".to_string());
    }
    Ok(profile)
}

fn percentile(values: &mut [u64], percentile: f64) -> u64 {
    if values.is_empty() {
        return 0;
    }
    values.sort_unstable();
    let rank = ((values.len() as f64 * percentile).ceil() as usize)
        .saturating_sub(1)
        .min(values.len() - 1);
    values[rank]
}

fn collect_samples(report: &EvaluationReport, samples: &mut HashMap<String, Vec<u64>>) {
    for case in &report.cases {
        if let Some(mode) = samples.get_mut(&case.execution_mode) {
            mode.push(case.reranker_latency_ms);
        }
    }
}

pub fn evaluate(
    connection: &Connection,
    root: &Path,
    suite: &EvaluationSuite,
    profile: &PerformanceProfile,
) -> Result<PerformanceReport, String> {
    let mut cold_model_load_ms = 0u64;
    for _ in 0..profile.warmup_runs {
        let report = evaluation::evaluate(connection, root, suite)?;
        cold_model_load_ms = cold_model_load_ms.max(
            report
                .cases
                .iter()
                .map(|case| case.reranker_model_load_ms)
                .max()
                .unwrap_or_default(),
        );
    }
    let mut samples = profile
        .modes
        .keys()
        .map(|mode| (mode.clone(), Vec::new()))
        .collect::<HashMap<_, _>>();
    let mut prepare = Vec::new();
    let mut inference = Vec::new();
    let mut input_tokens = Vec::new();
    for _ in 0..profile.measured_runs {
        let report = evaluation::evaluate(connection, root, suite)?;
        collect_samples(&report, &mut samples);
        for case in report.cases {
            prepare.push(case.reranker_input_prepare_ms as f64);
            inference.push(case.reranker_inference_ms as f64);
            input_tokens.push(case.reranker_average_input_tokens as f64);
        }
    }
    let mut modes = HashMap::new();
    for (mode, values) in samples {
        let slo = profile.modes[&mode].warm_p95_ms;
        let mut p50_values = values.clone();
        let mut p95_values = values.clone();
        let mut p99_values = values.clone();
        let p95 = percentile(&mut p95_values, 0.95);
        modes.insert(
            mode,
            Percentiles {
                samples: values.len(),
                p50_latency_ms: percentile(&mut p50_values, 0.50),
                p95_latency_ms: p95,
                p99_latency_ms: percentile(&mut p99_values, 0.99),
                max_p95_latency_ms: slo,
                passed: values.len() >= profile.minimum_samples_per_mode && p95 <= slo,
            },
        );
    }
    let measured = modes
        .values()
        .all(|mode| mode.samples >= profile.minimum_samples_per_mode);
    let all_mode_slos_passed = measured
        && modes.values().all(|mode| mode.passed)
        && cold_model_load_ms <= profile.cold_model_load_max_ms;
    let average = |values: &[f64]| {
        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        }
    };
    Ok(PerformanceReport {
        schema_version: REPORT_SCHEMA_VERSION.to_string(),
        profile_version: profile.version.clone(),
        machine_profile: profile.machine_profile.clone(),
        model: profile.model.clone(),
        target_profile_frozen: profile.sealed,
        measured,
        all_mode_slos_passed,
        cold_model_load_ms,
        cold_model_load_max_ms: profile.cold_model_load_max_ms,
        p95_latency_ms: modes
            .values()
            .map(|mode| mode.p95_latency_ms)
            .max()
            .unwrap_or_default(),
        max_p95_latency_ms: modes
            .values()
            .map(|mode| mode.max_p95_latency_ms)
            .max()
            .unwrap_or_default(),
        average_input_prepare_ms: average(&prepare),
        average_inference_ms: average(&inference),
        average_input_tokens: average(&input_tokens),
        modes,
    })
}

pub fn write_report(report: &PerformanceReport, output: &Path) -> Result<(), String> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("PERFORMANCE_REPORT_WRITE_FAILED: {error}"))?;
    }
    let part = output.with_extension("json.part");
    let mut bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| format!("PERFORMANCE_REPORT_SERIALIZE_FAILED: {error}"))?;
    bytes.push(b'\n');
    fs::write(&part, bytes).map_err(|error| format!("PERFORMANCE_REPORT_WRITE_FAILED: {error}"))?;
    fs::rename(&part, output).map_err(|error| format!("PERFORMANCE_REPORT_WRITE_FAILED: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_uses_nearest_rank_and_never_interpolates_latency() {
        let mut values = vec![10, 20, 30, 40, 50];
        assert_eq!(percentile(&mut values, 0.50), 30);
        assert_eq!(percentile(&mut values, 0.95), 50);
    }
}
