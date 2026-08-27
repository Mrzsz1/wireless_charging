use super::state_mutation::{
    state_patch_schema, validate_patch, ResearchStatePatch, ResearchStateSummary,
};
use super::{compact, ConversationTurn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::Instant;

pub const UNDERSTANDING_SCHEMA_VERSION: &str = "qa-understanding-v2";
const MAX_HISTORY_TURNS: usize = 16;
const MAX_HISTORY_CHARS: usize = 1_200;
const MAX_STANDALONE_CHARS: usize = 2_000;
const MAX_ENTITIES: usize = 12;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResearchIntent {
    #[default]
    DirectFactual,
    LiteratureSearch,
    Comparison,
    OriginDerivation,
    MethodImprovement,
    SolutionSearch,
    ProblemModeling,
    Novelty,
    FollowUp,
    ExploratoryResearch,
}

impl ResearchIntent {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectFactual => "direct_factual",
            Self::LiteratureSearch => "literature_search",
            Self::Comparison => "comparison",
            Self::OriginDerivation => "origin_derivation",
            Self::MethodImprovement => "method_improvement",
            Self::SolutionSearch => "solution_search",
            Self::ProblemModeling => "problem_modeling",
            Self::Novelty => "novelty",
            Self::FollowUp => "follow_up",
            Self::ExploratoryResearch => "exploratory_research",
        }
    }

    pub fn answer_profile(self) -> &'static str {
        match self {
            Self::LiteratureSearch | Self::OriginDerivation => "literature",
            Self::Comparison => "relationship",
            Self::Novelty => "novelty",
            Self::MethodImprovement => "method_improvement",
            Self::SolutionSearch => "solution_search",
            Self::ProblemModeling => "problem_modeling",
            Self::ExploratoryResearch => "exploratory",
            _ => "solve",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    #[default]
    Direct,
    Research,
    Exploratory,
}

impl ExecutionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Research => "research",
            Self::Exploratory => "exploratory",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityCandidate {
    pub value: String,
    pub source_message_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedQuestion {
    pub original_question: String,
    pub standalone_question: String,
    pub resolved_entities: Vec<String>,
    pub used_history_message_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResearchQuery {
    pub original_question: String,
    pub standalone_question: String,
    pub entities: Vec<String>,
    pub intent: ResearchIntent,
    pub used_history_message_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RoutedResearchQuery {
    pub query: ResearchQuery,
    pub execution_mode: ExecutionMode,
    pub routing_reason: String,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UnderstandingTurn {
    pub message_id: String,
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UnderstandingPlanningInput {
    pub original_question: String,
    pub recent_history: Vec<UnderstandingTurn>,
    pub current_entities: Vec<String>,
    #[serde(skip)]
    pub history_entities: Vec<EntityCandidate>,
    #[serde(default)]
    pub current_state: ResearchStateSummary,
}

impl UnderstandingPlanningInput {
    pub fn new(
        question: &str,
        history: &[ConversationTurn],
        current_entities: Vec<String>,
        history_entities: Vec<EntityCandidate>,
    ) -> Self {
        let mut recent_history = history
            .iter()
            .rev()
            .take(MAX_HISTORY_TURNS)
            .map(|turn| UnderstandingTurn {
                message_id: turn.id.clone(),
                role: turn.role.clone(),
                content: bounded_history_content(&turn.content),
            })
            .collect::<Vec<_>>();
        recent_history.reverse();
        Self {
            original_question: question.trim().to_string(),
            recent_history,
            current_entities,
            history_entities,
            current_state: ResearchStateSummary::default(),
        }
    }

    pub fn with_current_state(mut self, current_state: ResearchStateSummary) -> Self {
        self.current_state = current_state;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UnderstandingPlan {
    pub schema_version: String,
    pub standalone_question: String,
    pub resolved_entities: Vec<String>,
    pub used_history_message_ids: Vec<String>,
    pub intent: ResearchIntent,
    pub execution_mode: ExecutionMode,
    #[serde(default)]
    pub state_patch: ResearchStatePatch,
}

pub type UnderstandingPlanner<'a> =
    dyn FnMut(&UnderstandingPlanningInput) -> Result<UnderstandingPlan, String> + 'a;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnderstandingDiagnostics {
    pub resolver_used: String,
    pub resolver_status: String,
    pub resolver_latency_ms: u64,
    pub resolver_fallback: bool,
    pub resolver_fallback_reason: String,
    pub router_used: String,
    pub router_status: String,
    pub router_latency_ms: u64,
    pub router_fallback: bool,
    pub routing_confidence: String,
    pub resolver_escalated: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnderstandingResult {
    pub routed: RoutedResearchQuery,
    pub diagnostics: UnderstandingDiagnostics,
    pub state_patch: ResearchStatePatch,
}

#[derive(Debug)]
pub struct ResolverOutcome {
    pub resolved: ResolvedQuestion,
    pub intent_hint: Option<ResearchIntent>,
    pub execution_mode_hint: Option<ExecutionMode>,
    pub resolver_used: String,
    pub resolver_status: String,
    pub resolver_latency_ms: u64,
    pub fallback: bool,
    pub fallback_reason: String,
    pub routing_confidence: String,
    pub escalated: bool,
    pub state_patch: ResearchStatePatch,
}

pub trait ConversationResolver {
    fn resolve(&mut self, input: &UnderstandingPlanningInput) -> ResolverOutcome;
}

pub struct DeterministicConversationResolver;

impl ConversationResolver for DeterministicConversationResolver {
    fn resolve(&mut self, input: &UnderstandingPlanningInput) -> ResolverOutcome {
        let started = Instant::now();
        let resolved = deterministic_resolution(input);
        let (_, _, reason) = deterministic_route(&resolved);
        ResolverOutcome {
            resolved,
            intent_hint: None,
            execution_mode_hint: None,
            resolver_used: "deterministic-conversation-v1".to_string(),
            resolver_status: "succeeded".to_string(),
            resolver_latency_ms: elapsed_ms(started),
            fallback: false,
            fallback_reason: String::new(),
            routing_confidence: deterministic_routing_confidence(&input.original_question, reason)
                .to_string(),
            escalated: false,
            state_patch: ResearchStatePatch::empty(None),
        }
    }
}

pub struct HybridConversationResolver<'a> {
    planner: Option<&'a mut UnderstandingPlanner<'a>>,
    fallback: DeterministicConversationResolver,
}

impl<'a> HybridConversationResolver<'a> {
    pub fn new(planner: Option<&'a mut UnderstandingPlanner<'a>>) -> Self {
        Self {
            planner,
            fallback: DeterministicConversationResolver,
        }
    }
}

impl ConversationResolver for HybridConversationResolver<'_> {
    fn resolve(&mut self, input: &UnderstandingPlanningInput) -> ResolverOutcome {
        let deterministic = self.fallback.resolve(input);
        let (_, deterministic_mode, _) = deterministic_route(&deterministic.resolved);
        let contextual =
            contains_reference(&input.original_question) && !input.recent_history.is_empty();
        let low_confidence = deterministic.routing_confidence == "low";
        let open_problem = deterministic_mode == ExecutionMode::Exploratory;
        if !contextual && !low_confidence && !open_problem {
            return deterministic;
        }
        let Some(planner) = self.planner.as_mut() else {
            return deterministic;
        };
        let started = Instant::now();
        match planner(input) {
            Ok(plan) => ResolverOutcome {
                resolved: ResolvedQuestion {
                    original_question: input.original_question.clone(),
                    standalone_question: plan.standalone_question,
                    resolved_entities: plan.resolved_entities,
                    used_history_message_ids: plan.used_history_message_ids,
                },
                intent_hint: Some(plan.intent),
                execution_mode_hint: Some(plan.execution_mode),
                resolver_used: "hybrid-conversation-v1".to_string(),
                resolver_status: "succeeded".to_string(),
                resolver_latency_ms: elapsed_ms(started),
                fallback: false,
                fallback_reason: String::new(),
                routing_confidence: deterministic.routing_confidence,
                escalated: true,
                state_patch: plan.state_patch,
            },
            Err(error) => {
                let mut outcome = deterministic;
                outcome.resolver_used = "hybrid-conversation-v1".to_string();
                outcome.resolver_status = "failed_fallback".to_string();
                outcome.resolver_latency_ms = elapsed_ms(started);
                outcome.fallback = true;
                outcome.fallback_reason =
                    super::provider_capabilities::stable_provider_failure_kind(&error).to_string();
                outcome.escalated = true;
                outcome
            }
        }
    }
}

#[derive(Debug)]
pub struct RoutingOutcome {
    pub routed: RoutedResearchQuery,
    pub router_used: String,
    pub router_status: String,
    pub router_latency_ms: u64,
    pub fallback: bool,
}

pub trait IntentRouter {
    fn route(
        &self,
        resolved: ResolvedQuestion,
        intent_hint: Option<ResearchIntent>,
        execution_mode_hint: Option<ExecutionMode>,
        provider_fallback: bool,
    ) -> RoutingOutcome;
}

pub struct HybridIntentRouter;

impl IntentRouter for HybridIntentRouter {
    fn route(
        &self,
        resolved: ResolvedQuestion,
        intent_hint: Option<ResearchIntent>,
        execution_mode_hint: Option<ExecutionMode>,
        provider_fallback: bool,
    ) -> RoutingOutcome {
        let started = Instant::now();
        let (intent, execution_mode, routing_reason, used_provider) =
            match (intent_hint, execution_mode_hint) {
                (Some(intent), Some(mode)) => {
                    (intent, mode, "provider_structured_classification", true)
                }
                _ => {
                    let (intent, mode, reason) = deterministic_route(&resolved);
                    (intent, mode, reason, false)
                }
            };
        let query = ResearchQuery {
            original_question: resolved.original_question,
            standalone_question: resolved.standalone_question,
            entities: resolved.resolved_entities,
            intent,
            used_history_message_ids: resolved.used_history_message_ids,
        };
        RoutingOutcome {
            routed: RoutedResearchQuery {
                query,
                execution_mode,
                routing_reason: routing_reason.to_string(),
            },
            router_used: if used_provider {
                "hybrid-intent-router-v1"
            } else {
                "deterministic-intent-router-v1"
            }
            .to_string(),
            router_status: if provider_fallback {
                "failed_fallback"
            } else {
                "succeeded"
            }
            .to_string(),
            router_latency_ms: elapsed_ms(started),
            fallback: provider_fallback,
        }
    }
}

pub fn resolve_and_route<'a>(
    input: &UnderstandingPlanningInput,
    planner: Option<&'a mut UnderstandingPlanner<'a>>,
) -> UnderstandingResult {
    let mut resolver = HybridConversationResolver::new(planner);
    let resolved = resolver.resolve(input);
    let routing_confidence = resolved.routing_confidence.clone();
    let resolver_escalated = resolved.escalated;
    let router = HybridIntentRouter;
    let routed = router.route(
        resolved.resolved,
        resolved.intent_hint,
        resolved.execution_mode_hint,
        resolved.fallback,
    );
    UnderstandingResult {
        routed: routed.routed,
        diagnostics: UnderstandingDiagnostics {
            resolver_used: resolved.resolver_used,
            resolver_status: resolved.resolver_status,
            resolver_latency_ms: resolved.resolver_latency_ms,
            resolver_fallback: resolved.fallback,
            resolver_fallback_reason: resolved.fallback_reason,
            router_used: routed.router_used,
            router_status: routed.router_status,
            router_latency_ms: routed.router_latency_ms,
            router_fallback: routed.fallback,
            routing_confidence,
            resolver_escalated,
        },
        state_patch: resolved.state_patch,
    }
}

fn deterministic_routing_confidence(question: &str, reason: &str) -> &'static str {
    if reason != "direct_factual_default" {
        return "high";
    }
    let lower = question.to_lowercase();
    if lower.chars().count() >= 28
        || contains_any(
            &lower,
            &[
                "如何",
                "怎样",
                "为什么",
                "是否",
                "能否",
                "哪些",
                "有什么",
                "what other",
                "how can",
                "why",
            ],
        )
    {
        "low"
    } else {
        "medium"
    }
}

pub fn contains_reference(question: &str) -> bool {
    let lower = question.to_lowercase();
    [
        "它",
        "它们",
        "二者",
        "两者",
        "这些",
        "上述",
        "前者",
        "后者",
        "这个方法",
        "这种方法",
        "这个模型",
        "这个约束",
        "那个",
        "那种",
        "该方法",
        "该模型",
        "第一个",
        "第二个",
        "第三个",
        "第一种",
        "第二种",
        "第三种",
        "上一个",
        "刚才",
        "继续",
        "我的模型",
        "之前那个",
        "they",
        "them",
        "these",
        "those",
        "both",
        "that method",
        "this model",
    ]
    .iter()
    .any(|marker| lower.contains(marker))
}

fn deterministic_resolution(input: &UnderstandingPlanningInput) -> ResolvedQuestion {
    // `resolved_entities` records only context imported from history. Entities
    // already present in a self-contained question remain in the question text
    // and must not make the audit claim that history resolution was used.
    let mut entities = Vec::new();
    let mut used_history_message_ids = Vec::new();
    if contains_reference(&input.original_question) && input.current_entities.len() < 2 {
        let ordinal = requested_ordinal(&input.original_question);
        if let Some((value, message_id)) =
            ordinal.and_then(|index| ordinal_history_candidate(&input.recent_history, index))
        {
            entities.push(value);
            used_history_message_ids.push(message_id);
        }
        for candidate in &input.history_entities {
            if entities.len() >= 8 {
                break;
            }
            if !entities
                .iter()
                .any(|value| value.eq_ignore_ascii_case(&candidate.value))
            {
                entities.push(candidate.value.clone());
            }
            if !candidate.source_message_id.is_empty()
                && !used_history_message_ids.contains(&candidate.source_message_id)
            {
                used_history_message_ids.push(candidate.source_message_id.clone());
            }
        }
    }
    entities = bounded_unique(entities, MAX_ENTITIES, 160);
    let standalone_question = if entities.is_empty() {
        input.original_question.clone()
    } else if contains_reference(&input.original_question) {
        format!(
            "{} 相关实体：{}",
            input.original_question,
            entities.join("；")
        )
    } else {
        input.original_question.clone()
    };
    ResolvedQuestion {
        original_question: input.original_question.clone(),
        standalone_question,
        resolved_entities: entities,
        used_history_message_ids,
    }
}

fn deterministic_route(
    resolved: &ResolvedQuestion,
) -> (ResearchIntent, ExecutionMode, &'static str) {
    let question = resolved.standalone_question.to_lowercase();
    if contains_any(
        &question,
        &[
            "新颖",
            "创新性",
            "novel",
            "是否有人做过",
            "idea",
            "是否已经覆盖",
            "是否已有完整解法",
        ],
    ) {
        return (
            ResearchIntent::Novelty,
            ExecutionMode::Research,
            "novelty_markers",
        );
    }
    if contains_any(
        &question,
        &[
            "还能改进",
            "怎么改进",
            "如何改进",
            "弱点",
            "替代方案",
            "hybrid",
        ],
    ) {
        return (
            ResearchIntent::MethodImprovement,
            ExecutionMode::Exploratory,
            "method_improvement_markers",
        );
    }
    if contains_any(
        &question,
        &["怎么来的", "如何推导", "由来", "起源", "最早提出", "origin"],
    ) {
        return (
            ResearchIntent::OriginDerivation,
            ExecutionMode::Research,
            "origin_markers",
        );
    }
    if !contains_any(
        &question,
        &[
            "怎么建模",
            "如何建模",
            "目标函数",
            "问题模型",
            "约束条件",
            "论文",
            "文献",
            "哪项工作",
            "哪本书",
        ],
    ) && contains_any(
        &question,
        &[
            "有没有解法",
            "有什么算法可以解",
            "如何求解",
            "怎么求解",
            "有哪些办法",
            "有什么办法",
            "有什么可直接使用的方法",
            "怎样",
            "如何",
            "如何降低",
            "应该参考",
            "solution",
        ],
    ) {
        return (
            ResearchIntent::SolutionSearch,
            ExecutionMode::Exploratory,
            "solution_search_markers",
        );
    }
    if contains_any(
        &question,
        &["怎么建模", "如何建模", "目标函数", "问题模型", "约束条件"],
    ) {
        return (
            ResearchIntent::ProblemModeling,
            ExecutionMode::Exploratory,
            "problem_modeling_markers",
        );
    }
    if contains_any(
        &question,
        &[
            "比较",
            "区别",
            "差异",
            "关系",
            "相比",
            "不能直接替代",
            "versus",
            " vs ",
        ],
    ) {
        return (
            ResearchIntent::Comparison,
            ExecutionMode::Research,
            "comparison_markers",
        );
    }
    if contains_any(
        &question,
        &[
            "哪些论文",
            "哪篇论文",
            "相关论文",
            "相关文献",
            "有没有文献",
            "哪本书",
            "论文",
            "哪项工作",
            "literature",
            "which paper",
        ],
    ) {
        return (
            ResearchIntent::LiteratureSearch,
            ExecutionMode::Research,
            "literature_markers",
        );
    }
    if contains_any(
        &question,
        &["可以从哪些方向研究", "开放问题", "研究方向", "explore"],
    ) {
        return (
            ResearchIntent::ExploratoryResearch,
            ExecutionMode::Exploratory,
            "exploratory_markers",
        );
    }
    if contains_reference(&resolved.original_question) {
        return (
            ResearchIntent::FollowUp,
            ExecutionMode::Research,
            "conversation_reference",
        );
    }
    (
        ResearchIntent::DirectFactual,
        ExecutionMode::Direct,
        "direct_factual_default",
    )
}

fn contains_any(value: &str, markers: &[&str]) -> bool {
    markers.iter().any(|marker| value.contains(marker))
}

fn requested_ordinal(question: &str) -> Option<usize> {
    let lower = question.to_lowercase();
    [
        (0, ["第一个", "第一种", "第一项", "first"]),
        (1, ["第二个", "第二种", "第二项", "second"]),
        (2, ["第三个", "第三种", "第三项", "third"]),
    ]
    .into_iter()
    .find_map(|(index, markers)| {
        markers
            .iter()
            .any(|marker| lower.contains(marker))
            .then_some(index)
    })
}

fn ordinal_history_candidate(
    history: &[UnderstandingTurn],
    requested_index: usize,
) -> Option<(String, String)> {
    for turn in history.iter().rev() {
        let candidates = turn
            .content
            .lines()
            .filter_map(enumerated_value)
            .collect::<Vec<_>>();
        if let Some(value) = candidates.get(requested_index) {
            return Some((value.clone(), turn.message_id.clone()));
        }
    }
    None
}

fn enumerated_value(line: &str) -> Option<String> {
    let line = line.trim().trim_start_matches(['-', '*', ' ']).trim();
    let prefixes = [
        "1.",
        "1、",
        "1)",
        "2.",
        "2、",
        "2)",
        "3.",
        "3、",
        "3)",
        "第一种",
        "第二种",
        "第三种",
        "第一个",
        "第二个",
        "第三个",
    ];
    let prefix = prefixes.iter().find(|prefix| line.starts_with(**prefix))?;
    let value = line[prefix.len()..]
        .trim_start_matches([':', '：', '.', '、', ')', ' '])
        .trim();
    (!value.is_empty()).then(|| compact(value, 120))
}

fn bounded_unique(values: Vec<String>, maximum: usize, maximum_chars: usize) -> Vec<String> {
    let mut seen = HashSet::new();
    values
        .into_iter()
        .map(|value| compact(&value, maximum_chars))
        .filter(|value| !value.is_empty() && seen.insert(value.to_lowercase()))
        .take(maximum)
        .collect()
}

fn bounded_history_content(value: &str) -> String {
    let value = value.trim();
    if value.chars().count() <= MAX_HISTORY_CHARS {
        value.to_string()
    } else {
        value.chars().take(MAX_HISTORY_CHARS).collect::<String>() + "…"
    }
}

fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
}

pub fn parse_understanding_plan(
    raw: &str,
    input: &UnderstandingPlanningInput,
) -> Result<UnderstandingPlan, String> {
    let mut plan = serde_json::from_str::<UnderstandingPlan>(raw.trim())
        .map_err(|error| format!("UNDERSTANDING_INVALID: JSON 解析失败：{error}"))?;
    if plan.schema_version != UNDERSTANDING_SCHEMA_VERSION {
        return Err("UNDERSTANDING_INVALID: schemaVersion 不受支持".to_string());
    }
    plan.state_patch = validate_patch(plan.state_patch)
        .map_err(|error| format!("UNDERSTANDING_INVALID: {error}"))?;
    plan.standalone_question = compact(&plan.standalone_question, MAX_STANDALONE_CHARS);
    if plan.standalone_question.chars().count() < 2 {
        return Err("UNDERSTANDING_INVALID: standaloneQuestion 不能为空".to_string());
    }
    plan.resolved_entities = bounded_unique(plan.resolved_entities, MAX_ENTITIES, 160);
    let allowed_ids = input
        .recent_history
        .iter()
        .map(|turn| turn.message_id.as_str())
        .collect::<HashSet<_>>();
    let mut seen_ids = HashSet::new();
    plan.used_history_message_ids
        .retain(|id| allowed_ids.contains(id.as_str()) && seen_ids.insert(id.clone()));
    if contains_reference(&input.original_question)
        && !input.recent_history.is_empty()
        && plan.used_history_message_ids.is_empty()
    {
        return Err("UNDERSTANDING_INVALID: 指代消解未标注历史消息".to_string());
    }
    Ok(plan)
}

pub fn understanding_schema() -> Value {
    let intents = [
        ResearchIntent::DirectFactual,
        ResearchIntent::LiteratureSearch,
        ResearchIntent::Comparison,
        ResearchIntent::OriginDerivation,
        ResearchIntent::MethodImprovement,
        ResearchIntent::SolutionSearch,
        ResearchIntent::ProblemModeling,
        ResearchIntent::Novelty,
        ResearchIntent::FollowUp,
        ResearchIntent::ExploratoryResearch,
    ]
    .map(ResearchIntent::as_str);
    let modes = [
        ExecutionMode::Direct,
        ExecutionMode::Research,
        ExecutionMode::Exploratory,
    ]
    .map(ExecutionMode::as_str);
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "additionalProperties": false,
        "required": ["schemaVersion", "standaloneQuestion", "resolvedEntities", "usedHistoryMessageIds", "intent", "executionMode", "statePatch"],
        "properties": {
            "schemaVersion": {"type": "string", "const": UNDERSTANDING_SCHEMA_VERSION},
            "standaloneQuestion": {"type": "string", "minLength": 2, "maxLength": MAX_STANDALONE_CHARS},
            "resolvedEntities": {"type": "array", "maxItems": MAX_ENTITIES, "items": {"type": "string", "minLength": 1, "maxLength": 160}},
            "usedHistoryMessageIds": {"type": "array", "maxItems": MAX_HISTORY_TURNS, "uniqueItems": true, "items": {"type": "string", "minLength": 1, "maxLength": 160}},
            "intent": {"type": "string", "enum": intents},
            "executionMode": {"type": "string", "enum": modes},
            "statePatch": state_patch_schema()
        }
    })
}

pub fn understanding_prompt(input: &UnderstandingPlanningInput) -> String {
    let payload = serde_json::to_string(input).unwrap_or_else(|_| "{}".to_string());
    format!(
        "你是科研问答的问题理解器。只输出符合原生 JSON Schema 的对象。\n\
         将当前问题改写为无需查看历史也能理解的 standaloneQuestion，保留目标、约束、假设、否定和比较对象。\n\
         resolvedEntities 只能来自当前问题或 recentHistory；usedHistoryMessageIds 只能填写实际参与消解的 messageId。\n\
         statePatch 只能描述当前消息对 currentState 的逐对象有序修改，不能生成最终 State；没有修改时 operations 必须为空。低置信度破坏性操作保持 low，让后端 fail closed。\n\
         intent 使用最具体的科研意图。简单事实用 direct；需要多来源检索用 research；方法改进、解法搜索、问题建模和开放探索用 exploratory。\n\
         不回答问题，不生成证据，不编造论文、方法或约束。输入 JSON：{payload}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(id: &str, role: &str, content: &str) -> ConversationTurn {
        ConversationTurn {
            id: id.to_string(),
            role: role.to_string(),
            content: content.to_string(),
            request_id: "request".to_string(),
        }
    }

    #[test]
    fn deterministic_resolver_uses_history_and_preserves_message_ids() {
        let history = vec![
            turn("u1", "user", "ROSE 这篇论文研究什么？"),
            turn("a1", "assistant", "ROSE 处理多径干涉与安全功率调度。"),
        ];
        let input = UnderstandingPlanningInput::new(
            "它还能改进吗？",
            &history,
            Vec::new(),
            vec![EntityCandidate {
                value: "ROSE".to_string(),
                source_message_id: "u1".to_string(),
            }],
        );
        let result = resolve_and_route(&input, None);
        assert!(result.routed.query.standalone_question.contains("ROSE"));
        assert_eq!(result.routed.query.used_history_message_ids, ["u1"]);
        assert_eq!(
            result.routed.query.intent,
            ResearchIntent::MethodImprovement
        );
        assert_eq!(result.routed.execution_mode, ExecutionMode::Exploratory);
    }

    #[test]
    fn ordinal_resolution_selects_the_requested_enumerated_item() {
        let history = vec![turn(
            "a1",
            "assistant",
            "1. 确定性贪心调度\n2. 语义重排方案\n3. 多轮补充检索",
        )];
        let input =
            UnderstandingPlanningInput::new("第二种方法呢？", &history, Vec::new(), Vec::new());
        let result = resolve_and_route(&input, None);
        assert!(result
            .routed
            .query
            .standalone_question
            .contains("语义重排方案"));
        assert_eq!(result.routed.query.used_history_message_ids, ["a1"]);
    }

    #[test]
    fn provider_failure_uses_deterministic_fallback_and_audits_it() {
        let history = vec![turn("u1", "user", "讨论 CCSP 并发充电")];
        let input = UnderstandingPlanningInput::new(
            "它有哪些相关论文？",
            &history,
            Vec::new(),
            vec![EntityCandidate {
                value: "CCSP".to_string(),
                source_message_id: "u1".to_string(),
            }],
        );
        let mut planner =
            |_input: &UnderstandingPlanningInput| Err("fixture provider failure".to_string());
        let result = resolve_and_route(&input, Some(&mut planner));
        assert!(result.diagnostics.resolver_fallback);
        assert_eq!(result.diagnostics.resolver_status, "failed_fallback");
        assert_eq!(result.diagnostics.resolver_fallback_reason, "unavailable");
        assert_eq!(result.routed.query.intent, ResearchIntent::LiteratureSearch);
    }

    #[test]
    fn provider_failure_matrix_preserves_stable_fallback_reasons() {
        let history = vec![turn("u1", "user", "讨论 CCSP 并发充电")];
        let input = UnderstandingPlanningInput::new(
            "它有哪些相关论文？",
            &history,
            Vec::new(),
            vec![EntityCandidate {
                value: "CCSP".to_string(),
                source_message_id: "u1".to_string(),
            }],
        );
        for (error, expected) in [
            ("PROVIDER_TIMEOUT: endpoint detail", "timeout"),
            ("UNDERSTANDING_INVALID: bad JSON", "invalid_response"),
            ("LUNA_HTTP_ERROR: HTTP 429", "rate_limit"),
            ("LLM_BUDGET_EXCEEDED: call budget", "budget"),
        ] {
            let mut planner = |_input: &UnderstandingPlanningInput| Err(error.to_string());
            let result = resolve_and_route(&input, Some(&mut planner));
            assert!(result.diagnostics.resolver_fallback, "{error}");
            assert_eq!(result.diagnostics.resolver_status, "failed_fallback");
            assert_eq!(result.diagnostics.resolver_fallback_reason, expected);
            assert!(!result
                .diagnostics
                .resolver_fallback_reason
                .contains("detail"));
        }
    }

    #[test]
    fn provider_plan_rejects_unknown_history_ids() {
        let input = UnderstandingPlanningInput::new(
            "它是什么？",
            &[turn("u1", "user", "讨论 ROSE")],
            Vec::new(),
            Vec::new(),
        );
        let raw = json!({
            "schemaVersion": UNDERSTANDING_SCHEMA_VERSION,
            "standaloneQuestion": "ROSE 是什么？",
            "resolvedEntities": ["ROSE"],
            "usedHistoryMessageIds": ["invented"],
            "intent": "follow_up",
            "executionMode": "research"
        });
        assert!(parse_understanding_plan(&raw.to_string(), &input).is_err());
    }

    #[test]
    fn low_confidence_self_contained_question_escalates_to_provider() {
        let input = UnderstandingPlanningInput::new(
            "这个复杂方案在多个互相耦合的约束下性能边界是否足够稳健并且可以推广？",
            &[],
            Vec::new(),
            Vec::new(),
        );
        let mut calls = 0;
        let mut planner = |_input: &UnderstandingPlanningInput| {
            calls += 1;
            Ok(UnderstandingPlan {
                schema_version: UNDERSTANDING_SCHEMA_VERSION.to_string(),
                standalone_question: "复杂约束方案的后续研究方向".to_string(),
                resolved_entities: Vec::new(),
                used_history_message_ids: Vec::new(),
                intent: ResearchIntent::ExploratoryResearch,
                execution_mode: ExecutionMode::Exploratory,
                state_patch: ResearchStatePatch::empty(None),
            })
        };
        let result = resolve_and_route(&input, Some(&mut planner));
        assert_eq!(calls, 1);
        assert!(result.diagnostics.resolver_escalated);
        assert_eq!(result.diagnostics.routing_confidence, "low");
        assert_eq!(
            result.routed.query.intent,
            ResearchIntent::ExploratoryResearch
        );
    }

    #[test]
    fn medium_confidence_direct_fact_skips_provider() {
        let input =
            UnderstandingPlanningInput::new("ROSE 的作者是谁？", &[], Vec::new(), Vec::new());
        let mut calls = 0;
        let mut planner = |_input: &UnderstandingPlanningInput| {
            calls += 1;
            Err("must not run".to_string())
        };
        let result = resolve_and_route(&input, Some(&mut planner));
        assert_eq!(calls, 0);
        assert!(!result.diagnostics.resolver_escalated);
        assert_eq!(result.diagnostics.routing_confidence, "medium");
        assert_eq!(result.routed.execution_mode, ExecutionMode::Direct);
    }

    #[test]
    fn specialized_research_intents_keep_distinct_answer_profiles() {
        assert_eq!(
            ResearchIntent::MethodImprovement.answer_profile(),
            "method_improvement"
        );
        assert_eq!(
            ResearchIntent::SolutionSearch.answer_profile(),
            "solution_search"
        );
        assert_eq!(
            ResearchIntent::ProblemModeling.answer_profile(),
            "problem_modeling"
        );
        assert_eq!(
            ResearchIntent::ExploratoryResearch.answer_profile(),
            "exploratory"
        );
    }

    #[test]
    fn frozen_follow_up_matrix_covers_fifty_resolution_and_routing_cases() {
        let fixture: Value = serde_json::from_str(include_str!(
            "../../../../../evals/conversation_understanding_cases.json"
        ))
        .expect("conversation understanding fixture");
        assert_eq!(
            fixture["schemaVersion"],
            "qa-conversation-understanding-cases-v1"
        );
        assert_eq!(fixture["datasetRole"], "development_regression");
        let subjects = fixture["subjects"].as_array().expect("subjects");
        let patterns = fixture["patterns"].as_array().expect("patterns");
        assert_eq!(subjects.len() * patterns.len(), 50);
        assert_eq!(fixture["caseCount"], 50);

        for subject in subjects {
            let subject_id = subject["id"].as_str().expect("subject id");
            let entity = subject["entity"].as_str().expect("entity");
            let history_id = format!("history-user-{subject_id}");
            let history = vec![
                turn(
                    &history_id,
                    "user",
                    subject["historyQuestion"]
                        .as_str()
                        .expect("history question"),
                ),
                turn(
                    &format!("history-assistant-{subject_id}"),
                    "assistant",
                    subject["historyAnswer"].as_str().expect("history answer"),
                ),
            ];
            for pattern in patterns {
                let input = UnderstandingPlanningInput::new(
                    pattern["question"].as_str().expect("question"),
                    &history,
                    Vec::new(),
                    vec![EntityCandidate {
                        value: entity.to_string(),
                        source_message_id: history_id.clone(),
                    }],
                );
                let result = resolve_and_route(&input, None);
                assert!(
                    result.routed.query.standalone_question.contains(entity),
                    "subject={subject_id} pattern={}",
                    pattern["id"].as_str().unwrap_or_default()
                );
                assert!(result
                    .routed
                    .query
                    .used_history_message_ids
                    .contains(&history_id));
                assert_eq!(
                    result.routed.query.intent.as_str(),
                    pattern["expectedIntent"].as_str().expect("expected intent")
                );
                assert_eq!(
                    result.routed.execution_mode.as_str(),
                    pattern["expectedMode"].as_str().expect("expected mode")
                );
            }
        }

        for case in fixture["ordinalCases"].as_array().expect("ordinal cases") {
            let history = vec![turn(
                "ordinal-assistant",
                "assistant",
                case["historyAnswer"].as_str().expect("history answer"),
            )];
            let input = UnderstandingPlanningInput::new(
                case["question"].as_str().expect("question"),
                &history,
                Vec::new(),
                Vec::new(),
            );
            let result = resolve_and_route(&input, None);
            assert!(result
                .routed
                .query
                .standalone_question
                .contains(case["expectedEntity"].as_str().expect("expected entity")));
        }
    }
}
