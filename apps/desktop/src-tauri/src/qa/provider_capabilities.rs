use super::{complete_luna_json, LunaSettings, PROVIDER_API, PROVIDER_CODEX, PROVIDER_OFFLINE};
use crate::codex_subscription;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderCapabilities {
    pub natural_generation: bool,
    pub structured_output: bool,
    pub understanding: bool,
    pub query_planning: bool,
    pub semantic_verification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderDescriptor {
    pub id: String,
    pub capabilities: ProviderCapabilities,
}

pub fn provider_descriptor(provider: &str) -> ProviderDescriptor {
    let capabilities = match provider {
        PROVIDER_CODEX | PROVIDER_API => ProviderCapabilities {
            natural_generation: true,
            structured_output: true,
            understanding: true,
            query_planning: true,
            semantic_verification: true,
        },
        PROVIDER_OFFLINE => ProviderCapabilities {
            natural_generation: false,
            structured_output: false,
            understanding: false,
            query_planning: false,
            semantic_verification: false,
        },
        _ => ProviderCapabilities {
            natural_generation: false,
            structured_output: false,
            understanding: false,
            query_planning: false,
            semantic_verification: false,
        },
    };
    ProviderDescriptor {
        id: provider.to_string(),
        capabilities,
    }
}

/// Reduce provider failures to a bounded, payload-free telemetry value.
///
/// Provider messages may contain endpoint details or response bodies.  Callers
/// must persist only this classification, never the original error string.
pub fn stable_provider_failure_kind(error: &str) -> &'static str {
    let upper = error.to_ascii_uppercase();
    if upper.starts_with("QUESTION_CANCELLED") || upper.contains("_CANCELLED") {
        "cancelled"
    } else if upper.contains("BUDGET") {
        "budget"
    } else if upper.contains("RATE_LIMIT")
        || upper.contains("RATE LIMIT")
        || upper.contains("HTTP 429")
    {
        "rate_limit"
    } else if upper.contains("TIMEOUT") || upper.contains("TIMED OUT") {
        "timeout"
    } else if upper.contains("INVALID")
        || upper.contains("JSON")
        || upper.contains("PROTOCOL")
        || upper.contains("SCHEMA")
    {
        "invalid_response"
    } else {
        "unavailable"
    }
}

pub trait PlanningProvider: Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;
    fn complete_structured(
        &self,
        prompt: &str,
        schema: &Value,
        cancelled: &AtomicBool,
    ) -> Result<String, String>;
}

struct CodexPlanningProvider {
    model: String,
    reasoning_effort: String,
    timeout: Duration,
}

impl PlanningProvider for CodexPlanningProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        provider_descriptor(PROVIDER_CODEX)
    }

    fn complete_structured(
        &self,
        prompt: &str,
        schema: &Value,
        cancelled: &AtomicBool,
    ) -> Result<String, String> {
        codex_subscription::stream_answer(
            prompt,
            Some(schema),
            &self.model,
            &self.reasoning_effort,
            self.timeout,
            cancelled,
            |_| Ok(()),
        )
        .map(|(content, _)| content)
    }
}

struct CompatibleApiPlanningProvider {
    settings: LunaSettings,
}

impl PlanningProvider for CompatibleApiPlanningProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        provider_descriptor(PROVIDER_API)
    }

    fn complete_structured(
        &self,
        prompt: &str,
        schema: &Value,
        cancelled: &AtomicBool,
    ) -> Result<String, String> {
        complete_luna_json(&self.settings, prompt, schema, cancelled).map(|(content, _)| content)
    }
}

pub fn planning_provider(
    settings: &LunaSettings,
    codex_model: &str,
    codex_reasoning_effort: &str,
) -> Option<Box<dyn PlanningProvider>> {
    let descriptor = provider_descriptor(&settings.answer_provider);
    if !descriptor.capabilities.understanding && !descriptor.capabilities.query_planning {
        return None;
    }
    match settings.answer_provider.as_str() {
        PROVIDER_CODEX => Some(Box::new(CodexPlanningProvider {
            model: codex_model.to_string(),
            reasoning_effort: codex_reasoning_effort.to_string(),
            timeout: Duration::from_secs(settings.timeout_seconds.clamp(30, 60)),
        })),
        PROVIDER_API => Some(Box::new(CompatibleApiPlanningProvider {
            settings: settings.clone(),
        })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codex_and_compatible_api_have_planning_parity() {
        let codex = provider_descriptor(PROVIDER_CODEX);
        let api = provider_descriptor(PROVIDER_API);
        assert_eq!(codex.capabilities, api.capabilities);
        assert!(codex.capabilities.understanding);
        assert!(codex.capabilities.query_planning);
        assert!(codex.capabilities.semantic_verification);
        assert!(
            !provider_descriptor(PROVIDER_OFFLINE)
                .capabilities
                .query_planning
        );
    }

    #[test]
    fn compatible_provider_is_constructed_without_reading_or_exposing_the_key() {
        let settings = LunaSettings {
            answer_provider: PROVIDER_API.to_string(),
            endpoint: "https://example.invalid/v1/chat/completions".to_string(),
            api_key_env: "MISSING_FIXTURE_KEY".to_string(),
            ..LunaSettings::default()
        };
        let provider = planning_provider(&settings, "", "").expect("planning provider");
        assert_eq!(provider.descriptor().id, PROVIDER_API);
    }

    #[test]
    fn frozen_provider_matrix_covers_capabilities_and_failure_telemetry() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../../../../../evals/provider_failure_matrix.json"
        ))
        .expect("provider matrix fixture");
        assert_eq!(fixture["schemaVersion"], "qa-provider-failure-matrix-v1");
        let providers = fixture["providers"].as_array().expect("providers");
        assert_eq!(providers.len(), 3);
        for row in providers {
            let descriptor = provider_descriptor(row["id"].as_str().expect("provider id"));
            let expected = &row["capabilities"];
            assert_eq!(
                descriptor.capabilities.natural_generation,
                expected["generation"].as_bool().expect("generation")
            );
            assert_eq!(
                descriptor.capabilities.structured_output,
                expected["structuredOutput"]
                    .as_bool()
                    .expect("structured output")
            );
            assert_eq!(
                descriptor.capabilities.query_planning,
                expected["planning"].as_bool().expect("planning")
            );
            assert_eq!(
                descriptor.capabilities.semantic_verification,
                expected["semanticVerification"]
                    .as_bool()
                    .expect("semantic verification")
            );
        }
        for failure in fixture["failures"].as_array().expect("failures") {
            assert_eq!(
                stable_provider_failure_kind(failure["error"].as_str().expect("error")),
                failure["expectedKind"].as_str().expect("expected kind")
            );
        }
    }
}
