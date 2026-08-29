use super::*;
use crate::codex_subscription;
use rusqlite::Connection;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

pub struct PreparedProductionQa {
    pub context: QuestionContext,
    pub settings: LunaSettings,
    pub budget_guard: LlmBudgetGuard,
}

#[derive(Debug, Clone)]
pub struct ProductionQaGenerated {
    pub answer: String,
    pub provider: String,
    pub model: String,
    pub offline: bool,
    pub semantic_verification: SemanticVerificationBatch,
    pub audit: AnswerAudit,
}

pub fn prepare_production_qa(
    connection: &Connection,
    root: &Path,
    request: &AskRequest,
    request_id: &str,
    cancelled: &AtomicBool,
) -> Result<PreparedProductionQa, String> {
    if let Some(existing_session) = request.session_id.as_deref() {
        get_session(connection, root, existing_session)?;
    }
    let conversation = conversation_history(connection, root, request.session_id.as_deref())?;
    if cancelled.load(Ordering::SeqCst) {
        return Err("QUESTION_CANCELLED: 用户停止了问答".to_string());
    }
    let settings = get_luna_settings(connection, root, false)?;
    let initial_route = build_retrieval_query(connection, &request.question, &conversation);
    let budget_guard = LlmBudgetGuard::new(routing_policy(&initial_route.execution_mode));
    let planner_model = request
        .codex_model
        .as_deref()
        .unwrap_or(settings.codex_model.as_str())
        .to_string();
    let planner_effort = request
        .codex_reasoning_effort
        .as_deref()
        .unwrap_or(settings.codex_reasoning_effort.as_str())
        .to_string();
    let planning_provider = planning_provider(&settings, &planner_model, &planner_effort);
    let planning_capabilities = provider_descriptor(&settings.answer_provider).capabilities;
    let understanding_budget = budget_guard.clone();
    let understanding_provider = planning_provider.as_deref();
    let mut understanding_planner = |input: &UnderstandingPlanningInput| {
        let prompt = understanding_prompt(input);
        let schema = understanding_schema();
        let reserved = estimate_tokens(&prompt).saturating_add(1_024);
        let reservation = understanding_budget.reserve("understanding", reserved)?;
        let Some(provider) = understanding_provider else {
            reservation.release()?;
            return Err("PLANNING_PROVIDER_UNAVAILABLE: understanding".to_string());
        };
        let result = provider.complete_structured(&prompt, &schema, cancelled);
        let actual = result
            .as_ref()
            .map(|raw| estimate_tokens(&prompt).saturating_add(estimate_tokens(raw)))
            .unwrap_or_else(|_| estimate_tokens(&prompt));
        reservation.settle(actual)?;
        let raw = result?;
        parse_understanding_plan(&raw, input)
    };
    let planner_budget = budget_guard.clone();
    let query_planning_provider = planning_provider.as_deref();
    let mut query_planner = |input: &QueryPlanningInput| {
        let prompt = query_plan_prompt(input);
        let schema = query_plan_schema();
        let reserved = estimate_tokens(&prompt).saturating_add(1_536);
        let reservation = planner_budget.reserve("planner", reserved)?;
        let Some(provider) = query_planning_provider else {
            reservation.release()?;
            return Err("PLANNING_PROVIDER_UNAVAILABLE: query_plan".to_string());
        };
        let result = provider.complete_structured(&prompt, &schema, cancelled);
        let actual = result
            .as_ref()
            .map(|raw| estimate_tokens(&prompt).saturating_add(estimate_tokens(raw)))
            .unwrap_or_else(|_| estimate_tokens(&prompt));
        reservation.settle(actual)?;
        let raw = result?;
        parse_query_plan(&raw, &input.resolved_question)
    };
    let planner = (planning_provider.is_some() && planning_capabilities.query_planning).then_some(
        &mut query_planner as &mut dyn FnMut(&QueryPlanningInput) -> Result<QueryPlan, String>,
    );
    let understanding = (planning_provider.is_some() && planning_capabilities.understanding)
        .then_some(
            &mut understanding_planner
                as &mut dyn FnMut(&UnderstandingPlanningInput) -> Result<UnderstandingPlan, String>,
        );
    let mut context = prepare_question_with_history_budget_and_planners(
        connection,
        root,
        &request.question,
        request.evidence_limit.unwrap_or(14),
        request_id,
        conversation,
        Some(cancelled),
        settings.context_window_tokens,
        settings.max_output_tokens,
        planner,
        understanding,
        Some(&budget_guard),
    )?;
    context.retrieval_query.planning_provider = planning_provider
        .as_ref()
        .map(|provider| provider.descriptor().id)
        .unwrap_or_else(|| PROVIDER_OFFLINE.to_string());
    context.retrieval_query.provider_capabilities = [
        (planning_capabilities.understanding, "understanding"),
        (planning_capabilities.query_planning, "query_planning"),
        (
            planning_capabilities.semantic_verification,
            "semantic_verification",
        ),
        (planning_capabilities.structured_output, "structured_output"),
        (
            planning_capabilities.natural_generation,
            "natural_generation",
        ),
    ]
    .into_iter()
    .filter(|(enabled, _)| *enabled)
    .map(|(_, capability)| capability.to_string())
    .collect();

    Ok(PreparedProductionQa {
        context,
        settings,
        budget_guard,
    })
}

#[allow(clippy::too_many_arguments)]
fn run_production_qa_generation_inner<F>(
    context: &QuestionContext,
    settings: &LunaSettings,
    budget_guard: &LlmBudgetGuard,
    codex_ready: bool,
    effective_codex_model: &str,
    effective_codex_effort: &str,
    cancelled: &AtomicBool,
    mut on_token: F,
) -> Result<ProductionQaGenerated, String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    let zero_evidence = context.evidence.is_empty();
    let generated: Result<(String, String, String), String> =
        match settings.answer_provider.as_str() {
            PROVIDER_CODEX if codex_ready => {
                let prompt = build_codex_prompt(context);
                let output_schema = codex_output_schema(context);
                let reserved = estimate_tokens(&prompt).saturating_add(settings.max_output_tokens);
                match budget_guard.reserve("generator", reserved) {
                    Err(error) => Err(error),
                    Ok(reservation) => {
                        let prompt_cost = estimate_tokens(&prompt);
                        let result = codex_subscription::stream_answer(
                            &prompt,
                            output_schema.as_ref(),
                            effective_codex_model,
                            effective_codex_effort,
                            Duration::from_secs(settings.timeout_seconds.max(180)),
                            cancelled,
                            &mut on_token,
                        );
                        let actual = result
                            .as_ref()
                            .map(|(answer, _)| prompt_cost.saturating_add(estimate_tokens(answer)))
                            .unwrap_or(prompt_cost);
                        reservation.settle(actual)?;
                        result.map(|(answer, model)| (answer, PROVIDER_CODEX.to_string(), model))
                    }
                }
            }
            PROVIDER_CODEX => Err("CODEX_NOT_READY: 请在设置中登录 ChatGPT".to_string()),
            PROVIDER_API if settings.endpoint.is_empty() || !settings.api_key_configured => {
                Err("LUNA_NOT_CONFIGURED: endpoint 或 API Key 环境变量尚未配置".to_string())
            }
            PROVIDER_API => {
                let prompt_cost = estimate_tokens(&build_codex_prompt(context));
                let reserved = prompt_cost.saturating_add(settings.max_output_tokens);
                match budget_guard.reserve("generator", reserved) {
                    Err(error) => Err(error),
                    Ok(reservation) => {
                        let result = stream_luna(settings, context, cancelled, &mut on_token);
                        let actual = result
                            .as_ref()
                            .map(|(answer, _)| prompt_cost.saturating_add(estimate_tokens(answer)))
                            .unwrap_or(prompt_cost);
                        reservation.settle(actual)?;
                        result.map(|(answer, resolved_model)| {
                            (answer, PROVIDER_API.to_string(), resolved_model)
                        })
                    }
                }
            }
            PROVIDER_OFFLINE => Ok((
                offline_answer(context),
                PROVIDER_OFFLINE.to_string(),
                "deterministic".to_string(),
            )),
            _ => Err("PROVIDER_INVALID: 不支持的回答引擎".to_string()),
        };

    let (answer, provider, model) = generated?;
    if cancelled.load(Ordering::SeqCst) {
        return Err("QUESTION_CANCELLED: 用户停止了问答".to_string());
    }
    let offline = provider == PROVIDER_OFFLINE;
    let answer = if zero_evidence {
        normalize_unverified_answer(&answer)
    } else {
        answer
    };
    let semantic_verification = match run_semantic_verification(
        settings,
        &model,
        effective_codex_effort,
        &answer,
        &context.evidence,
        budget_guard,
        cancelled,
    ) {
        Ok(batch) => batch,
        Err(error)
            if cancelled.load(Ordering::SeqCst) || error.starts_with("QUESTION_CANCELLED") =>
        {
            return Err("QUESTION_CANCELLED: 用户停止了问答".to_string());
        }
        Err(_) => SemanticVerificationBatch {
            provider: provider.clone(),
            model: model.clone(),
            status: "unavailable".to_string(),
            fallback_reason: "semantic_verifier_task_error".to_string(),
            ..SemanticVerificationBatch::default()
        },
    };
    let model_requested = match provider.as_str() {
        PROVIDER_CODEX => effective_codex_model.to_string(),
        PROVIDER_API => settings.model.clone(),
        _ => "deterministic".to_string(),
    };
    let metadata = ProviderRunMetadata {
        provider: provider.clone(),
        model_requested,
        model_resolved: model.clone(),
        temperature: (provider == PROVIDER_API).then_some(settings.temperature),
        max_output_tokens: settings.max_output_tokens,
        context_window_tokens: settings.context_window_tokens,
        enforce_answer_schema: !natural_answer_v2_enabled() && provider != PROVIDER_OFFLINE,
    };
    let audit = audit_generated_answer_with_semantic(
        context,
        &answer,
        &metadata,
        Some(&semantic_verification),
    );
    Ok(ProductionQaGenerated {
        answer,
        provider,
        model,
        offline,
        semantic_verification,
        audit,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn run_production_qa_generation<F>(
    context: &mut QuestionContext,
    settings: &LunaSettings,
    budget_guard: &LlmBudgetGuard,
    codex_ready: bool,
    effective_codex_model: &str,
    effective_codex_effort: &str,
    cancelled: &AtomicBool,
    on_token: F,
) -> Result<ProductionQaGenerated, String>
where
    F: FnMut(&str) -> Result<(), String>,
{
    let result = run_production_qa_generation_inner(
        context,
        settings,
        budget_guard,
        codex_ready,
        effective_codex_model,
        effective_codex_effort,
        cancelled,
        on_token,
    );
    record_llm_budget_usage(context, budget_guard.usage());
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn tauri_adapter_delegates_to_the_shared_production_generator() {
        let ui_source = include_str!("../lib.rs");
        let core_source = include_str!("production_core.rs");
        assert!(ui_source.contains("run_production_qa_generation("));
        assert!(!ui_source.contains("codex_subscription::stream_answer("));
        assert!(core_source.contains("let result = codex_subscription::stream_answer("));
    }
}
