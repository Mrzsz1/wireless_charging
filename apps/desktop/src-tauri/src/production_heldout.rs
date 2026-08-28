use crate::codex_subscription;
use crate::heldout_runner::{self, HeldoutRunOptions, HeldoutRuntimeConfig, QaCaseAudit};
use crate::qa;
use rusqlite::Connection;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

fn resolve_runtime(options: &HeldoutRunOptions) -> Result<HeldoutRuntimeConfig, String> {
    match options.provider.as_str() {
        qa::PROVIDER_CODEX => {
            let status = codex_subscription::get_status();
            if !status.ready {
                return Err("HELDOUT_PROVIDER_NOT_READY: codex-subscription".to_string());
            }
            let (model, reasoning_effort) = codex_subscription::resolve_model_selection(
                &options.model,
                &options.reasoning_effort,
                &status,
            );
            if model.trim().is_empty() || reasoning_effort.trim().is_empty() {
                return Err("HELDOUT_RUNTIME_INVALID: unresolved Codex model/effort".to_string());
            }
            Ok(HeldoutRuntimeConfig {
                provider: qa::PROVIDER_CODEX.to_string(),
                model,
                reasoning_effort,
            })
        }
        qa::PROVIDER_API => {
            let model = if options.model.trim().is_empty() {
                qa::LunaSettings::default().model
            } else {
                options.model.trim().to_string()
            };
            let reasoning_effort = if options.reasoning_effort.trim().is_empty() {
                "low".to_string()
            } else {
                options.reasoning_effort.trim().to_string()
            };
            if env::var("QA_COMPATIBLE_ENDPOINT")
                .map(|value| value.trim().is_empty())
                .unwrap_or(true)
            {
                return Err(
                    "HELDOUT_RUNTIME_INVALID: QA_COMPATIBLE_ENDPOINT is required".to_string(),
                );
            }
            Ok(HeldoutRuntimeConfig {
                provider: qa::PROVIDER_API.to_string(),
                model,
                reasoning_effort,
            })
        }
        _ => Err(
            "HELDOUT_RUNTIME_INVALID: provider must be codex-subscription or compatible-api"
                .to_string(),
        ),
    }
}

fn settings_for(
    connection: &Connection,
    repository: &Path,
    runtime: &HeldoutRuntimeConfig,
) -> Result<qa::LunaSettings, String> {
    let mut settings = qa::get_luna_settings(connection, repository, false)?;
    settings.answer_provider = runtime.provider.clone();
    settings.codex_model = runtime.model.clone();
    settings.codex_reasoning_effort = runtime.reasoning_effort.clone();
    if runtime.provider == qa::PROVIDER_API {
        settings.endpoint = env::var("QA_COMPATIBLE_ENDPOINT")
            .unwrap_or_default()
            .trim()
            .trim_end_matches('/')
            .to_string();
        settings.model = runtime.model.clone();
        settings.api_key_env =
            env::var("QA_COMPATIBLE_KEY_ENV").unwrap_or_else(|_| "LUNA_API_KEY".to_string());
        settings.api_key_configured = env::var(&settings.api_key_env)
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false);
        if !settings.api_key_configured {
            return Err(format!(
                "HELDOUT_PROVIDER_NOT_READY: missing {}",
                settings.api_key_env
            ));
        }
    }
    Ok(settings)
}

fn execute_case(
    connection: &Connection,
    repository: &Path,
    question: &str,
    runtime: &HeldoutRuntimeConfig,
) -> Result<QaCaseAudit, String> {
    let settings = settings_for(connection, repository, runtime)?;
    let cancel = AtomicBool::new(false);
    let initial_route = qa::build_retrieval_query(connection, question, &[]);
    let budget_guard = qa::LlmBudgetGuard::new(qa::routing_policy(&initial_route.execution_mode));
    let planning_provider =
        qa::planning_provider(&settings, &runtime.model, &runtime.reasoning_effort);
    let capabilities = qa::provider_descriptor(&settings.answer_provider).capabilities;
    let understanding_budget = budget_guard.clone();
    let understanding_provider = planning_provider.as_deref();
    let mut understanding_planner = |input: &qa::UnderstandingPlanningInput| {
        let prompt = qa::understanding_prompt(input);
        let schema = qa::understanding_schema();
        let reserved = qa::estimate_tokens(&prompt).saturating_add(1_024);
        let reservation = understanding_budget.reserve("understanding", reserved)?;
        let Some(provider) = understanding_provider else {
            reservation.release()?;
            return Err("PLANNING_PROVIDER_UNAVAILABLE: understanding".to_string());
        };
        let result = provider.complete_structured(&prompt, &schema, &cancel);
        let actual = result
            .as_ref()
            .map(|raw| qa::estimate_tokens(&prompt).saturating_add(qa::estimate_tokens(raw)))
            .unwrap_or_else(|_| qa::estimate_tokens(&prompt));
        reservation.settle(actual)?;
        let raw = result?;
        qa::parse_understanding_plan(&raw, input)
    };
    let planner_budget = budget_guard.clone();
    let query_planning_provider = planning_provider.as_deref();
    let mut query_planner = |input: &qa::QueryPlanningInput| {
        let prompt = qa::query_plan_prompt(input);
        let schema = qa::query_plan_schema();
        let reserved = qa::estimate_tokens(&prompt).saturating_add(1_536);
        let reservation = planner_budget.reserve("planner", reserved)?;
        let Some(provider) = query_planning_provider else {
            reservation.release()?;
            return Err("PLANNING_PROVIDER_UNAVAILABLE: query_plan".to_string());
        };
        let result = provider.complete_structured(&prompt, &schema, &cancel);
        let actual = result
            .as_ref()
            .map(|raw| qa::estimate_tokens(&prompt).saturating_add(qa::estimate_tokens(raw)))
            .unwrap_or_else(|_| qa::estimate_tokens(&prompt));
        reservation.settle(actual)?;
        let raw = result?;
        qa::parse_query_plan(&raw, &input.resolved_question)
    };
    let planner = (planning_provider.is_some() && capabilities.query_planning).then_some(
        &mut query_planner
            as &mut dyn FnMut(&qa::QueryPlanningInput) -> Result<qa::QueryPlan, String>,
    );
    let understanding = (planning_provider.is_some() && capabilities.understanding).then_some(
        &mut understanding_planner
            as &mut dyn FnMut(
                &qa::UnderstandingPlanningInput,
            ) -> Result<qa::UnderstandingPlan, String>,
    );
    let request_id = uuid::Uuid::new_v4().to_string();
    let mut context = qa::prepare_question_with_history_budget_and_planners(
        connection,
        repository,
        question,
        14,
        &request_id,
        Vec::new(),
        Some(&cancel),
        settings.context_window_tokens,
        settings.max_output_tokens,
        planner,
        understanding,
        Some(&budget_guard),
    )?;
    context.retrieval_query.planning_provider = planning_provider
        .as_ref()
        .map(|provider| provider.descriptor().id)
        .unwrap_or_else(|| qa::PROVIDER_OFFLINE.to_string());
    context.retrieval_query.provider_capabilities = [
        (capabilities.understanding, "understanding"),
        (capabilities.query_planning, "query_planning"),
        (capabilities.semantic_verification, "semantic_verification"),
        (capabilities.structured_output, "structured_output"),
        (capabilities.natural_generation, "natural_generation"),
    ]
    .into_iter()
    .filter(|(enabled, _)| *enabled)
    .map(|(_, capability)| capability.to_string())
    .collect();

    let prompt = qa::build_codex_prompt(&context);
    let prompt_cost = qa::estimate_tokens(&prompt);
    let reserved = prompt_cost.saturating_add(settings.max_output_tokens);
    let reservation = budget_guard.reserve("generator", reserved)?;
    let generated = match runtime.provider.as_str() {
        qa::PROVIDER_CODEX => codex_subscription::stream_answer(
            &prompt,
            qa::codex_output_schema(&context).as_ref(),
            &runtime.model,
            &runtime.reasoning_effort,
            Duration::from_secs(settings.timeout_seconds.max(180)),
            &cancel,
            |_| Ok(()),
        ),
        qa::PROVIDER_API => qa::stream_luna(&settings, &context, &cancel, |_| Ok(())),
        _ => Err("HELDOUT_RUNTIME_INVALID: unsupported provider".to_string()),
    };
    let actual = generated
        .as_ref()
        .map(|(answer, _)| prompt_cost.saturating_add(qa::estimate_tokens(answer)))
        .unwrap_or(prompt_cost);
    reservation.settle(actual)?;
    let (answer, resolved_model) = generated?;
    let answer = if context.evidence.is_empty() {
        qa::normalize_unverified_answer(&answer)
    } else {
        answer
    };
    let semantic = qa::run_semantic_verification(
        &settings,
        &resolved_model,
        &runtime.reasoning_effort,
        &answer,
        &context.evidence,
        &budget_guard,
        &cancel,
    )
    .unwrap_or_else(|_| qa::SemanticVerificationBatch {
        provider: runtime.provider.clone(),
        model: resolved_model.clone(),
        status: "unavailable".to_string(),
        fallback_reason: "semantic_verifier_task_error".to_string(),
        ..qa::SemanticVerificationBatch::default()
    });
    qa::record_llm_budget_usage(&mut context, budget_guard.usage());
    let metadata = qa::ProviderRunMetadata {
        provider: runtime.provider.clone(),
        model_requested: runtime.model.clone(),
        model_resolved: resolved_model,
        temperature: (runtime.provider == qa::PROVIDER_API).then_some(settings.temperature),
        max_output_tokens: settings.max_output_tokens,
        context_window_tokens: settings.context_window_tokens,
        enforce_answer_schema: !qa::natural_answer_v2_enabled(),
    };
    let audit =
        qa::audit_generated_answer_with_semantic(&context, &answer, &metadata, Some(&semantic));
    if let Some(error) = audit.structured_answer_error {
        return Err(format!("HELDOUT_AUDIT_INVALID: {error}"));
    }
    Ok(QaCaseAudit {
        answer: audit.answer,
        evidence: audit.evidence,
        run_manifest: audit.run_manifest,
    })
}

pub fn run(options: HeldoutRunOptions) -> Result<PathBuf, String> {
    let repository = options
        .repository
        .canonicalize()
        .map_err(|error| format!("HELDOUT_REPOSITORY_INVALID: {error}"))?;
    let contract = heldout_runner::load_contract(&repository.join("evals/heldout_contract.json"))?;
    let dataset = heldout_runner::load_and_validate_dataset(&options.dataset, &contract)?;
    let git = heldout_runner::clean_git_snapshot(&repository)?;
    let runtime = resolve_runtime(&options)?;
    let mut connection = Connection::open_in_memory()
        .map_err(|error| format!("HELDOUT_DATABASE_FAILED: {error}"))?;
    crate::db_schema(&connection)?;
    crate::rebuild_connection(&mut connection, &repository)?;
    heldout_runner::run_with_executor(
        &dataset,
        &options.output_dir,
        &git,
        &runtime,
        qa::embedding_model_name(),
        |case, _session_id| execute_case(&connection, &repository, &case.question, &runtime),
    )
}
