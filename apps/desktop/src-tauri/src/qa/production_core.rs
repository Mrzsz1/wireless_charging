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
    pub offline: bool,
    pub semantic_verification: SemanticVerificationBatch,
    pub metadata: ProviderRunMetadata,
    pub audit: AnswerAudit,
}

pub fn prepare_production_qa(
    connection: &Connection,
    root: &Path,
    request: &AskRequest,
    request_id: &str,
    cancelled: &AtomicBool,
) -> Result<PreparedProductionQa, String> {
    trace::emit(&trace::QaTraceEvent::new(
        "qa_prepare_started",
        "prepare",
        "started",
        request_id,
    ));
    let result = (|| {
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
            let schema = super::understanding_provider_schema();
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
            let schema = query_plan_provider_schema();
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
        let planner = (planning_provider.is_some() && planning_capabilities.query_planning)
            .then_some(
                &mut query_planner
                    as &mut dyn FnMut(&QueryPlanningInput) -> Result<QueryPlan, String>,
            );
        let understanding = (planning_provider.is_some() && planning_capabilities.understanding)
            .then_some(
                &mut understanding_planner
                    as &mut dyn FnMut(
                        &UnderstandingPlanningInput,
                    ) -> Result<UnderstandingPlan, String>,
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
            request.session_id.as_deref(),
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
    })();
    match &result {
        Ok(prepared) => {
            let mut event = trace::QaTraceEvent::new(
                "qa_prepare_completed",
                "prepare",
                "succeeded",
                request_id,
            );
            event.execution_mode = prepared.context.retrieval_query.execution_mode.clone();
            event.provider = prepared.settings.answer_provider.clone();
            event.model = prepared.settings.codex_model.clone();
            event.evidence_count = Some(prepared.context.evidence.len());
            trace::emit(&event);
        }
        Err(error) => {
            let mut event =
                trace::QaTraceEvent::new("qa_prepare_failed", "prepare", "failed", request_id);
            event.error_code = trace::error_code(error);
            trace::emit(&event);
        }
    }
    result
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
    let evidence_availability = zero_evidence::classify_evidence_availability(
        &context.evidence,
        context.retrieval_query.planned_required_facet_count,
        context.retrieval_query.covered_facet_ids.len(),
    );
    let zero_evidence = evidence_availability.is_zero_usable();
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

    let (raw_answer, provider, model) = generated?;
    if cancelled.load(Ordering::SeqCst) {
        return Err("QUESTION_CANCELLED: 用户停止了问答".to_string());
    }
    let offline = provider == PROVIDER_OFFLINE;
    let answer = if zero_evidence {
        let mut started = trace::QaTraceEvent::new(
            "qa_zero_evidence_projection_started",
            "zero_evidence_projection",
            "started",
            &context.request_id,
        );
        started.execution_mode = context.retrieval_query.execution_mode.clone();
        started.provider = provider.clone();
        started.model = model.clone();
        started.evidence_count = Some(evidence_availability.raw_evidence_count);
        started.evidence_availability_mode = evidence_availability.mode.as_str().to_string();
        started.support_eligible_evidence_count =
            Some(evidence_availability.support_eligible_evidence_count);
        started.graph_only_evidence_count = Some(evidence_availability.graph_only_evidence_count);
        started.zero_evidence_reason = evidence_availability.reason.clone();
        trace::emit(&started);

        let projection = zero_evidence::project_zero_evidence_answer(&raw_answer);
        let projection_audit = zero_evidence::audit_zero_evidence_answer(
            &projection.markdown,
            &evidence_availability,
            0,
            "",
            Some(&projection),
        );
        if !projection_audit.complete {
            let mut failed = trace::QaTraceEvent::new(
                "qa_zero_evidence_projection_failed",
                "zero_evidence_projection",
                "failed",
                &context.request_id,
            );
            failed.execution_mode = context.retrieval_query.execution_mode.clone();
            failed.provider = provider.clone();
            failed.model = model.clone();
            failed.evidence_count = Some(evidence_availability.raw_evidence_count);
            failed.evidence_availability_mode = evidence_availability.mode.as_str().to_string();
            failed.support_eligible_evidence_count =
                Some(evidence_availability.support_eligible_evidence_count);
            failed.graph_only_evidence_count =
                Some(evidence_availability.graph_only_evidence_count);
            failed.zero_evidence_reason = evidence_availability.reason.clone();
            failed.error_code = projection_audit
                .error_codes
                .first()
                .cloned()
                .unwrap_or_else(|| "ZERO_EVIDENCE_PROJECTION_INVALID".to_string())
                .to_ascii_lowercase();
            trace::emit(&failed);
            return Err("ZERO_EVIDENCE_PROJECTION_INVALID".to_string());
        }
        let mut completed = trace::QaTraceEvent::new(
            "qa_zero_evidence_projection_completed",
            "zero_evidence_projection",
            "succeeded",
            &context.request_id,
        );
        completed.execution_mode = context.retrieval_query.execution_mode.clone();
        completed.provider = provider.clone();
        completed.model = model.clone();
        completed.evidence_count = Some(evidence_availability.raw_evidence_count);
        completed.evidence_availability_mode = evidence_availability.mode.as_str().to_string();
        completed.support_eligible_evidence_count =
            Some(evidence_availability.support_eligible_evidence_count);
        completed.graph_only_evidence_count = Some(evidence_availability.graph_only_evidence_count);
        completed.zero_evidence_reason = evidence_availability.reason.clone();
        trace::emit(&completed);
        if projection.fallback_applied {
            let mut fallback = trace::QaTraceEvent::new(
                "qa_zero_evidence_fallback_applied",
                "zero_evidence_projection",
                "succeeded",
                &context.request_id,
            );
            fallback.execution_mode = context.retrieval_query.execution_mode.clone();
            fallback.provider = provider.clone();
            fallback.model = model.clone();
            fallback.evidence_count = Some(evidence_availability.raw_evidence_count);
            fallback.evidence_availability_mode = evidence_availability.mode.as_str().to_string();
            fallback.support_eligible_evidence_count =
                Some(evidence_availability.support_eligible_evidence_count);
            fallback.graph_only_evidence_count =
                Some(evidence_availability.graph_only_evidence_count);
            fallback.zero_evidence_reason = projection.fallback_reason.clone();
            trace::emit(&fallback);
        }
        projection.markdown
    } else if direct_grounded_output(context) && provider != PROVIDER_OFFLINE {
        let mut started = trace::QaTraceEvent::new(
            "qa_direct_answer_parse_started",
            "direct_answer_parser",
            "started",
            &context.request_id,
        );
        started.execution_mode = context.retrieval_query.execution_mode.clone();
        started.provider = provider.clone();
        started.model = model.clone();
        started.evidence_count = Some(context.evidence.len());
        trace::emit(&started);
        match direct_answer::parse_validate_render(&raw_answer, &context.evidence) {
            Ok(rendered) => {
                let mut completed = trace::QaTraceEvent::new(
                    "qa_direct_answer_parse_completed",
                    "direct_answer_parser",
                    "succeeded",
                    &context.request_id,
                );
                completed.execution_mode = context.retrieval_query.execution_mode.clone();
                completed.provider = provider.clone();
                completed.model = model.clone();
                completed.evidence_count = Some(context.evidence.len());
                completed.claim_count =
                    Some(claim_verification::extract_atomic_claims(&rendered).len());
                trace::emit(&completed);
                rendered
            }
            Err(error) => {
                let mut failed = trace::QaTraceEvent::new(
                    "qa_direct_answer_parse_failed",
                    "direct_answer_parser",
                    "failed",
                    &context.request_id,
                );
                failed.execution_mode = context.retrieval_query.execution_mode.clone();
                failed.provider = provider.clone();
                failed.model = model.clone();
                failed.evidence_count = Some(context.evidence.len());
                failed.error_code = trace::error_code(&error);
                trace::emit(&failed);
                return Err(error);
            }
        }
    } else {
        raw_answer
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
        offline,
        semantic_verification,
        metadata,
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
    let mut started = trace::QaTraceEvent::new(
        "qa_generate_started",
        "generator",
        "started",
        &context.request_id,
    );
    started.execution_mode = context.retrieval_query.execution_mode.clone();
    started.provider = settings.answer_provider.clone();
    started.model = effective_codex_model.to_string();
    started.evidence_count = Some(context.evidence.len());
    trace::emit(&started);
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
    let result = result.map(|mut generated| {
        generated.audit = audit_generated_answer_with_semantic(
            context,
            &generated.answer,
            &generated.metadata,
            Some(&generated.semantic_verification),
        );
        generated
    });
    match &result {
        Ok(generated) => {
            let manifest = &generated.audit.run_manifest;
            let mut semantic = trace::QaTraceEvent::new(
                "qa_semantic_completed",
                adaptive_routing::SEMANTIC_VERIFIER_STAGE,
                &generated.semantic_verification.status,
                &context.request_id,
            );
            semantic.execution_mode = context.retrieval_query.execution_mode.clone();
            semantic.provider = generated.semantic_verification.provider.clone();
            semantic.model = generated.semantic_verification.model.clone();
            semantic.evidence_count = Some(context.evidence.len());
            semantic.claim_count = Some(manifest.claim_verifications.len());
            if !generated
                .semantic_verification
                .fallback_reason
                .trim()
                .is_empty()
            {
                semantic.error_code =
                    trace::error_code(&generated.semantic_verification.fallback_reason);
            }
            trace::emit(&semantic);

            let mut audit = trace::QaTraceEvent::new(
                "qa_audit_completed",
                "audit",
                &generated.audit.citation_validation.grounding_status,
                &context.request_id,
            );
            audit.execution_mode = context.retrieval_query.execution_mode.clone();
            audit.provider = generated.provider.clone();
            audit.model = generated.metadata.model_resolved.clone();
            audit.evidence_count = Some(context.evidence.len());
            audit.claim_count = Some(manifest.claim_verifications.len());
            audit.supported_claim_count = Some(manifest.verified_claim_count);
            audit.contradicted_claim_count = Some(manifest.contradicted_claim_count);
            audit.not_verifiable_claim_count = Some(manifest.not_verifiable_claim_count);
            audit.repaired_claim_count = Some(manifest.repaired_claim_count);
            trace::emit(&audit);

            let mut completed = trace::QaTraceEvent::new(
                "qa_generate_completed",
                "generator",
                "succeeded",
                &context.request_id,
            );
            completed.execution_mode = context.retrieval_query.execution_mode.clone();
            completed.provider = generated.provider.clone();
            completed.model = generated.metadata.model_resolved.clone();
            completed.evidence_count = Some(context.evidence.len());
            completed.claim_count = Some(manifest.claim_verifications.len());
            trace::emit(&completed);
        }
        Err(error) => {
            let mut failed = trace::QaTraceEvent::new(
                "qa_generate_failed",
                "generator",
                "failed",
                &context.request_id,
            );
            failed.execution_mode = context.retrieval_query.execution_mode.clone();
            failed.provider = settings.answer_provider.clone();
            failed.model = effective_codex_model.to_string();
            failed.evidence_count = Some(context.evidence.len());
            failed.error_code = trace::error_code(error);
            trace::emit(&failed);
        }
    }
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

    #[test]
    fn desktop_logging_is_enabled_for_debug_and_release_with_bounded_rotation() {
        let ui_source = include_str!("../lib.rs");
        assert!(ui_source.contains("tauri_plugin_log::Builder::default()"));
        assert!(ui_source.contains("RotationStrategy::KeepSome(5)"));
        assert!(ui_source.contains(".max_file_size(10 * 1024 * 1024)"));
        assert!(!ui_source.contains("if cfg!(debug_assertions)"));
    }

    #[test]
    fn zero_evidence_projection_has_complete_structured_lifecycle_logging() {
        let core_source = include_str!("production_core.rs");
        for event in [
            "qa_zero_evidence_projection_started",
            "qa_zero_evidence_projection_completed",
            "qa_zero_evidence_projection_failed",
            "qa_zero_evidence_fallback_applied",
        ] {
            assert!(core_source.contains(event), "missing={event}");
        }
        assert!(core_source.contains("support_eligible_evidence_count"));
        assert!(core_source.contains("graph_only_evidence_count"));
        assert!(core_source.contains("zero_evidence_reason"));
    }
}
