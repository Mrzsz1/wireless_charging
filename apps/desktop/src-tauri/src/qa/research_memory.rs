use super::{problem_understanding, ConversationTurn};
use serde::{Deserialize, Serialize};

pub const RESEARCH_STATE_VERSION: &str = "research-session-state-v1";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSessionState {
    pub schema_version: String,
    pub revision: usize,
    pub active_problem: String,
    pub objectives: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
    pub methods: Vec<String>,
    pub papers: Vec<String>,
    pub hypotheses: Vec<String>,
    pub open_questions: Vec<String>,
    pub source_message_ids: Vec<String>,
}

pub fn derive(history: &[ConversationTurn], current_question: &str) -> ResearchSessionState {
    let mut state = ResearchSessionState {
        schema_version: RESEARCH_STATE_VERSION.to_string(),
        ..ResearchSessionState::default()
    };
    for turn in history.iter().filter(|turn| turn.role == "user") {
        apply_turn(&mut state, &turn.content, Some(&turn.id));
    }
    apply_turn(&mut state, current_question, None);
    state.open_questions.truncate(12);
    state.hypotheses.truncate(12);
    state.papers.truncate(16);
    state.methods.truncate(16);
    state
}

fn apply_turn(state: &mut ResearchSessionState, content: &str, message_id: Option<&str>) {
    let parsed = problem_understanding::understand(content);
    let lower = content.to_lowercase();
    let replace_objective = contains_any(
        &lower,
        &[
            "目标改为",
            "目标改成",
            "现在目标",
            "改成最小化",
            "改成最大化",
            "objective is now",
        ],
    );
    let replace_constraints = contains_any(
        &lower,
        &["约束改为", "约束改成", "现在约束", "constraints are now"],
    );
    let replace_methods = contains_any(&lower, &["改用", "换成", "方法改为", "method is now"]);
    let remove = contains_any(
        &lower,
        &["去掉", "删除", "移除", "不再考虑", "remove", "drop"],
    );
    let meaningful = parsed.representation.domain != "unknown"
        || !parsed.representation.objectives.is_empty()
        || !parsed.representation.constraints.is_empty()
        || !parsed.representation.assumptions.is_empty()
        || contains_any(
            &lower,
            &[
                "假设", "如果", "论文", "rose", "tide", "pso", "alns", "milp",
            ],
        );
    if !meaningful {
        if content.trim().ends_with(['?', '？']) {
            push_latest(&mut state.open_questions, content.trim(), 240);
        }
        return;
    }

    state.revision += 1;
    if let Some(id) = message_id {
        push_unique(&mut state.source_message_ids, id);
    }
    if parsed.representation.domain != "unknown" {
        state.active_problem = parsed.representation.domain;
    }
    update_values(
        &mut state.objectives,
        parsed.representation.objectives,
        replace_objective,
        remove,
    );
    update_values(
        &mut state.constraints,
        parsed.representation.constraints,
        replace_constraints,
        remove,
    );
    update_values(
        &mut state.assumptions,
        parsed.representation.assumptions,
        false,
        remove,
    );

    let mut mentioned_methods = parsed
        .candidate_methods
        .into_iter()
        .filter(|method| lower.contains(&method.method.replace('_', " ")))
        .map(|method| method.method)
        .collect::<Vec<_>>();
    for (needle, canonical) in [
        ("pso", "particle_swarm_optimization"),
        ("粒子群", "particle_swarm_optimization"),
        ("alns", "adaptive_large_neighborhood_search"),
        ("milp", "mixed_integer_linear_programming"),
        ("遗传算法", "genetic_algorithm"),
        ("强化学习", "reinforcement_learning"),
    ] {
        if lower.contains(needle) {
            push_unique(&mut mentioned_methods, canonical);
        }
    }
    update_values(
        &mut state.methods,
        mentioned_methods,
        replace_methods,
        remove,
    );

    for token in content.split(|character: char| !character.is_ascii_alphanumeric()) {
        if token.len() >= 3
            && token.len() <= 24
            && token.chars().all(|character| {
                character.is_ascii_uppercase() || character.is_ascii_digit() || character == '-'
            })
        {
            push_unique(&mut state.papers, token);
        }
    }
    if contains_any(&lower, &["假设", "如果", "hypothesis", "suppose"]) {
        push_latest(&mut state.hypotheses, content.trim(), 240);
    }
    if content.trim().ends_with(['?', '？']) {
        push_latest(&mut state.open_questions, content.trim(), 240);
    }
}

fn update_values(target: &mut Vec<String>, values: Vec<String>, replace: bool, remove: bool) {
    if values.is_empty() {
        return;
    }
    if replace {
        target.clear();
    }
    if remove {
        target.retain(|current| !values.contains(current));
    } else {
        for value in values {
            push_unique(target, &value);
        }
    }
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn push_unique(values: &mut Vec<String>, value: &str) {
    if !value.is_empty() && !values.iter().any(|current| current == value) {
        values.push(value.to_string());
    }
}

fn push_latest(values: &mut Vec<String>, value: &str, maximum_chars: usize) {
    let compact = value.chars().take(maximum_chars).collect::<String>();
    values.retain(|current| current != &compact);
    values.insert(0, compact);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn turn(index: usize, role: &str, content: &str) -> ConversationTurn {
        ConversationTurn {
            id: format!("m{index}"),
            role: role.to_string(),
            content: content.to_string(),
            request_id: format!("r{}", index / 2),
        }
    }

    #[test]
    fn twenty_turn_research_chat_uses_latest_objective_constraint_method_and_hypothesis() {
        let user_turns = [
            "无线传感器网络移动充电要最大化网络寿命。",
            "先用 PSO。",
            "加入时间窗约束。",
            "假设节点静态。",
            "参考 ROSE 论文。",
            "路径还受电池容量限制。",
            "如果采用多个充电车会怎样？",
            "方法改用 ALNS。",
            "目标改为最小化死亡节点。",
            "去掉时间窗约束，后续怎么解？",
        ];
        let mut history = Vec::new();
        for (index, content) in user_turns.iter().enumerate() {
            history.push(turn(index * 2, "user", content));
            history.push(turn(index * 2 + 1, "assistant", "已记录并继续分析。"));
        }
        assert_eq!(history.len(), 20);
        let state = derive(&history, "现在的方法和约束是什么？");
        assert_eq!(state.active_problem, "wireless_sensor_network");
        assert_eq!(state.objectives, vec!["minimize_dead_nodes"]);
        assert!(!state.constraints.contains(&"time_windows".to_string()));
        assert!(state.constraints.contains(&"battery_capacity".to_string()));
        assert!(state
            .constraints
            .contains(&"multi_vehicle_coordination".to_string()));
        assert_eq!(state.methods, vec!["adaptive_large_neighborhood_search"]);
        assert!(state
            .assumptions
            .contains(&"stationary_sensor_nodes".to_string()));
        assert!(state.papers.contains(&"ROSE".to_string()));
        assert!(!state.hypotheses.is_empty());
        assert!(state.open_questions[0].contains("现在的方法和约束"));
    }
}
