use super::research_memory::ResearchSessionState;
use super::retrieval_contract::{RetrievalContract, RetrievalFacet};
use super::state_mutation::ResearchParameter;
use super::state_vocabulary::{
    active_ids_from_state, StateFieldDefinition, StateVocabularyRegistry,
};
use super::understanding::ResearchIntent;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

pub const RESEARCH_QUERY_CONTEXT_VERSION: &str = "research-query-context-v1";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResearchQueryContext {
    pub schema_version: String,
    pub current_question: String,
    pub research_intent: String,
    pub objectives: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
    pub parameters: BTreeMap<String, ResearchParameter>,
    pub active_methods: Vec<String>,
    pub excluded_methods: Vec<String>,
    pub resolved_references: Vec<String>,
    pub source_state_revision: usize,
    pub active_vocabulary_fields: Vec<ActiveVocabularyField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveVocabularyField {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub description: String,
    pub search_terms: Vec<String>,
}

fn bounded(values: &[String], maximum: usize) -> Vec<String> {
    let mut seen = HashSet::new();
    values
        .iter()
        .map(|value| value.trim().chars().take(120).collect::<String>())
        .filter(|value| !value.is_empty() && seen.insert(value.to_lowercase()))
        .take(maximum)
        .collect()
}

pub fn build_research_query_context(
    resolved_question: &str,
    intent: ResearchIntent,
    state: &ResearchSessionState,
    references: &[String],
) -> ResearchQueryContext {
    build_research_query_context_with_vocabulary(
        resolved_question,
        intent,
        state,
        references,
        &StateVocabularyRegistry::default(),
    )
}

pub fn build_research_query_context_with_vocabulary(
    resolved_question: &str,
    intent: ResearchIntent,
    state: &ResearchSessionState,
    references: &[String],
    registry: &StateVocabularyRegistry,
) -> ResearchQueryContext {
    let normalized_question = resolved_question.to_lowercase();
    let open_question = [
        "什么算法",
        "什么方法",
        "什么方案",
        "别的解法",
        "其他解法",
        "怎么改进",
        "如何改进",
        "别的领域",
        "其他领域",
        "迁移",
        "还有什么选择",
        "what algorithm",
        "what method",
        "how to improve",
        "transfer",
    ]
    .iter()
    .any(|marker| normalized_question.contains(marker));
    let open = open_question
        || matches!(
            intent,
            ResearchIntent::SolutionSearch
                | ResearchIntent::ExploratoryResearch
                | ResearchIntent::ProblemModeling
        );
    let method_focused = matches!(
        intent,
        ResearchIntent::MethodImprovement | ResearchIntent::Comparison | ResearchIntent::FollowUp
    );
    let include_all_constraints = open || method_focused;
    let parameters = if open || method_focused {
        state
            .parameters
            .iter()
            .take(12)
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    } else {
        BTreeMap::new()
    };
    let mut context = ResearchQueryContext {
        schema_version: RESEARCH_QUERY_CONTEXT_VERSION.to_string(),
        current_question: resolved_question.trim().chars().take(2_000).collect(),
        research_intent: intent.as_str().to_string(),
        objectives: bounded(
            &state.objectives,
            if open || method_focused { 8 } else { 3 },
        ),
        constraints: bounded(
            &state.constraints,
            if include_all_constraints { 12 } else { 4 },
        ),
        assumptions: if open || intent == ResearchIntent::ProblemModeling {
            bounded(&state.assumptions, 8)
        } else {
            Vec::new()
        },
        parameters,
        active_methods: if open || method_focused {
            bounded(&state.methods, 8)
        } else {
            Vec::new()
        },
        excluded_methods: if open || method_focused {
            bounded(&state.excluded_methods, 8)
        } else {
            Vec::new()
        },
        resolved_references: bounded(references, 8),
        source_state_revision: state.revision,
        active_vocabulary_fields: Vec::new(),
    };
    let active_ids = active_ids_from_state(
        &context.objectives,
        &context.constraints,
        &context.assumptions,
        &context.active_methods,
        &context.parameters,
    );
    context.active_vocabulary_fields = active_ids
        .iter()
        .filter_map(|id| registry.field(id))
        .map(active_vocabulary_field)
        .take(24)
        .collect();
    context
}

fn active_vocabulary_field(definition: &StateFieldDefinition) -> ActiveVocabularyField {
    let mut search_terms = vec![definition.id.clone(), definition.label.clone()];
    search_terms.extend(definition.aliases.iter().take(4).cloned());
    search_terms.extend(
        definition
            .description
            .split(|character: char| {
                character.is_whitespace()
                    || matches!(character, '，' | ',' | '。' | '；' | ';' | '：' | ':')
            })
            .map(str::trim)
            .filter(|term| (2..=32).contains(&term.chars().count()))
            .take(2)
            .map(str::to_string),
    );
    let mut seen = HashSet::new();
    search_terms.retain(|term| !term.trim().is_empty() && seen.insert(term.trim().to_lowercase()));
    search_terms.truncate(8);
    ActiveVocabularyField {
        id: definition.id.clone(),
        kind: definition.kind.as_str().to_string(),
        label: definition.label.clone(),
        description: definition.description.chars().take(240).collect(),
        search_terms,
    }
}

pub fn retrieval_terms(context: &ResearchQueryContext) -> Vec<String> {
    let mut values = Vec::new();
    values.extend(context.objectives.iter().cloned());
    values.extend(context.constraints.iter().cloned());
    values.extend(context.assumptions.iter().cloned());
    values.extend(context.active_methods.iter().cloned());
    for field in &context.active_vocabulary_fields {
        values.extend(field.search_terms.iter().cloned());
    }
    for parameter in context.parameters.values() {
        values.push(parameter.key.clone());
        values.push(parameter.value.search_text());
        if let Some(unit) = &parameter.unit {
            values.push(unit.clone());
        }
    }
    let mut seen = HashSet::new();
    values
        .into_iter()
        .filter(|value| !value.trim().is_empty() && seen.insert(value.to_lowercase()))
        .take(32)
        .collect()
}

pub fn enrich_contract(contract: &mut RetrievalContract, context: &ResearchQueryContext) {
    let terms = retrieval_terms(context);
    for value in terms.iter().take(10) {
        if !contract.concepts.iter().any(|current| current == value) {
            contract.concepts.push(value.clone());
        }
    }
    contract.concepts.truncate(12);
    for assumption in context.assumptions.iter().take(4) {
        if !contract.related_problems.contains(assumption) {
            contract.related_problems.push(assumption.clone());
        }
    }
    contract.related_problems.truncate(12);

    let mut append_facet = |id: &str, label: &str, values: Vec<String>| {
        if values.is_empty() || contract.facets.len() >= 8 {
            return;
        }
        contract.facets.push(RetrievalFacet {
            id: id.to_string(),
            label: label.to_string(),
            required: true,
            search_queries: values.into_iter().take(4).collect(),
            preferred_kinds: Vec::new(),
        });
    };
    append_facet(
        "state_objective",
        "当前研究目标",
        context.objectives.clone(),
    );
    append_facet(
        "state_constraints",
        "当前关键约束",
        context.constraints.clone(),
    );
    append_facet(
        "state_parameters",
        "当前模型参数",
        context
            .parameters
            .values()
            .map(|parameter| {
                format!(
                    "{} {} {}",
                    parameter.key,
                    parameter.value.search_text(),
                    parameter.unit.clone().unwrap_or_default()
                )
            })
            .collect(),
    );
}

pub fn method_is_excluded(method: &str, context: &ResearchQueryContext) -> bool {
    let normalized = method
        .to_lowercase()
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| token.len() >= 2)
        .collect::<Vec<_>>()
        .join("_");
    context.excluded_methods.iter().any(|excluded| {
        let excluded = excluded.to_lowercase();
        normalized.contains(&excluded)
            || excluded
                .split('_')
                .filter(|token| token.len() >= 3)
                .all(|token| normalized.contains(token))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::state_mutation::{ParameterValue, ResearchParameter};
    use crate::qa::state_vocabulary::{StateFieldDefinition, VocabularyKind, VocabularyOrigin};

    #[test]
    fn open_question_context_uses_current_state_and_exclusions() {
        let mut state = ResearchSessionState {
            objectives: vec!["minimize_dead_nodes".into()],
            constraints: vec!["deadlines".into(), "obstacle_avoidance".into()],
            methods: vec!["adaptive_large_neighborhood_search".into()],
            excluded_methods: vec!["particle_swarm_optimization".into()],
            revision: 4,
            ..ResearchSessionState::default_v2()
        };
        state.parameters.insert(
            "mobile_charger_count".into(),
            ResearchParameter {
                key: "mobile_charger_count".into(),
                value: ParameterValue::Integer(2),
                unit: None,
                source_message_id: None,
                updated_at_turn: 4,
            },
        );
        let context = build_research_query_context(
            "有什么算法适合这个模型？",
            ResearchIntent::SolutionSearch,
            &state,
            &[],
        );
        assert_eq!(context.source_state_revision, 4);
        assert_eq!(
            context.parameters["mobile_charger_count"].value,
            ParameterValue::Integer(2)
        );
        assert!(context.constraints.contains(&"deadlines".to_string()));
        assert!(context
            .excluded_methods
            .contains(&"particle_swarm_optimization".to_string()));
    }

    #[test]
    fn fallback_contract_receives_bounded_state_facets() {
        let context = ResearchQueryContext {
            objectives: vec!["minimize_dead_nodes".into()],
            constraints: vec!["deadlines".into(), "obstacle_avoidance".into()],
            ..ResearchQueryContext::default()
        };
        let mut contract = RetrievalContract::fallback("有什么算法适合当前模型");
        enrich_contract(&mut contract, &context);
        assert!(contract
            .concepts
            .contains(&"minimize_dead_nodes".to_string()));
        assert!(contract
            .facets
            .iter()
            .any(|facet| facet.id == "state_constraints"));
        assert!(contract.facets.len() <= 8);
    }

    #[test]
    fn active_custom_field_contributes_bounded_human_search_semantics() {
        let id = "custom:constraint:00000000-0000-0000-0000-000000000002";
        let registry = StateVocabularyRegistry::merged(
            3,
            vec![StateFieldDefinition {
                id: id.to_string(),
                kind: VocabularyKind::Constraint,
                label: "高温环境约束".to_string(),
                description: "环境温度过高时必须考虑充电效率和电池安全。".to_string(),
                aliases: vec!["温度过高".to_string(), "高温安全".to_string()],
                examples: Vec::new(),
                parameter_spec: None,
                origin: VocabularyOrigin::Custom,
                enabled: true,
            }],
        );
        let state = ResearchSessionState {
            constraints: vec![id.to_string()],
            revision: 2,
            ..ResearchSessionState::default_v2()
        };
        let context = build_research_query_context_with_vocabulary(
            "有什么方法适合当前模型？",
            ResearchIntent::SolutionSearch,
            &state,
            &[],
            &registry,
        );
        assert_eq!(context.active_vocabulary_fields.len(), 1);
        assert_eq!(context.active_vocabulary_fields[0].label, "高温环境约束");
        let terms = retrieval_terms(&context);
        assert!(terms.contains(&id.to_string()));
        assert!(terms.contains(&"高温环境约束".to_string()));
        assert!(terms.contains(&"温度过高".to_string()));
        assert!(terms.len() <= 32);
    }
}
