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
        },
        PROVIDER_OFFLINE => ProviderCapabilities {
            natural_generation: false,
            structured_output: false,
            understanding: false,
            query_planning: false,
        },
        _ => ProviderCapabilities {
            natural_generation: false,
            structured_output: false,
            understanding: false,
            query_planning: false,
        },
    };
    ProviderDescriptor {
        id: provider.to_string(),
        capabilities,
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
}
