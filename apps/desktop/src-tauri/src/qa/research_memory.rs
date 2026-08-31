use super::state_mutation::{
    extract_deterministic_patch, ResearchParameter, ResearchStatePatch, ResearchStateSummary,
};
use super::state_reducer::{apply_patch, StateApplyReport};
use super::{problem_understanding, ConversationTurn};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const RESEARCH_STATE_VERSION: &str = "research-session-state-v2";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSessionState {
    pub schema_version: String,
    pub state_version: String,
    pub revision: usize,
    pub active_problem: String,
    pub objectives: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
    pub methods: Vec<String>,
    #[serde(default)]
    pub excluded_methods: Vec<String>,
    #[serde(default)]
    pub parameters: BTreeMap<String, ResearchParameter>,
    pub papers: Vec<String>,
    pub hypotheses: Vec<String>,
    pub open_questions: Vec<String>,
    pub source_message_ids: Vec<String>,
    #[serde(default)]
    pub last_patch_id: String,
}

impl ResearchSessionState {
    pub fn default_v2() -> Self {
        Self {
            schema_version: RESEARCH_STATE_VERSION.to_string(),
            state_version: RESEARCH_STATE_VERSION.to_string(),
            ..Self::default()
        }
    }

    pub fn summary(&self) -> ResearchStateSummary {
        ResearchStateSummary {
            objectives: self.objectives.clone(),
            constraints: self.constraints.clone(),
            assumptions: self.assumptions.clone(),
            active_methods: self.methods.clone(),
            excluded_methods: self.excluded_methods.clone(),
            parameters: self.parameters.clone(),
        }
    }
}

pub fn derive_history(history: &[ConversationTurn]) -> ResearchSessionState {
    let mut state = ResearchSessionState::default_v2();
    for turn in history.iter().filter(|turn| turn.role == "user") {
        apply_turn(&mut state, &turn.content, Some(&turn.id), None, &[]);
    }
    bound_state(&mut state);
    state
}

pub fn load_canonical_state(
    connection: &Connection,
    repository_id: &str,
    session_id: &str,
) -> Result<Option<ResearchSessionState>, String> {
    let row = connection
        .query_row(
            "SELECT state_schema_version,state_json FROM qa_session_research_state
             WHERE session_id=?1 AND repository_id=?2",
            params![session_id, repository_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|error| format!("CANONICAL_STATE_LOAD_FAILED: {error}"))?;
    let Some((schema_version, state_json)) = row else {
        return Ok(None);
    };
    if schema_version != RESEARCH_STATE_VERSION {
        return Err("CANONICAL_STATE_SCHEMA_UNSUPPORTED".to_string());
    }
    let state = serde_json::from_str::<ResearchSessionState>(&state_json)
        .map_err(|error| format!("CANONICAL_STATE_INVALID: {error}"))?;
    if state.schema_version != RESEARCH_STATE_VERSION
        || state.state_version != RESEARCH_STATE_VERSION
    {
        return Err("CANONICAL_STATE_SCHEMA_UNSUPPORTED".to_string());
    }
    Ok(Some(state))
}

#[allow(clippy::too_many_arguments)]
pub fn persist_canonical_state(
    transaction: &Transaction<'_>,
    repository_id: &str,
    session_id: &str,
    user_message_id: &str,
    vocabulary_revision: u64,
    state: &ResearchSessionState,
    patch: &ResearchStatePatch,
    timestamp: &str,
) -> Result<(), String> {
    let state_json = serde_json::to_string(state)
        .map_err(|error| format!("CANONICAL_STATE_SERIALIZE_FAILED: {error}"))?;
    let patch_json = serde_json::to_string(patch)
        .map_err(|error| format!("CANONICAL_PATCH_SERIALIZE_FAILED: {error}"))?;
    transaction
        .execute(
            "INSERT INTO qa_message_state_patches(
               message_id,session_id,repository_id,patch_schema_version,vocabulary_revision,
               patch_json,created_at
             ) VALUES(?1,?2,?3,?4,?5,?6,?7)",
            params![
                user_message_id,
                session_id,
                repository_id,
                patch.schema_version,
                vocabulary_revision as i64,
                patch_json,
                timestamp,
            ],
        )
        .map_err(|error| format!("CANONICAL_PATCH_PERSIST_FAILED: {error}"))?;
    transaction
        .execute(
            "INSERT INTO qa_session_research_state(
               session_id,repository_id,state_schema_version,vocabulary_revision,state_json,
               last_source_message_id,updated_at
             ) VALUES(?1,?2,?3,?4,?5,?6,?7)
             ON CONFLICT(session_id) DO UPDATE SET
               repository_id=excluded.repository_id,
               state_schema_version=excluded.state_schema_version,
               vocabulary_revision=excluded.vocabulary_revision,
               state_json=excluded.state_json,
               last_source_message_id=excluded.last_source_message_id,
               updated_at=excluded.updated_at",
            params![
                session_id,
                repository_id,
                RESEARCH_STATE_VERSION,
                vocabulary_revision as i64,
                state_json,
                user_message_id,
                timestamp,
            ],
        )
        .map_err(|error| format!("CANONICAL_STATE_PERSIST_FAILED: {error}"))?;
    Ok(())
}

pub fn derive(history: &[ConversationTurn], current_question: &str) -> ResearchSessionState {
    let mut state = derive_history(history);
    apply_turn(&mut state, current_question, None, None, &[]);
    bound_state(&mut state);
    state
}

pub fn apply_current_patch(
    state: &mut ResearchSessionState,
    current_question: &str,
    patch: Option<ResearchStatePatch>,
    resolved_references: &[String],
) -> (ResearchStatePatch, StateApplyReport) {
    apply_turn(state, current_question, None, patch, resolved_references)
}

fn apply_turn(
    state: &mut ResearchSessionState,
    content: &str,
    message_id: Option<&str>,
    supplied_patch: Option<ResearchStatePatch>,
    resolved_references: &[String],
) -> (ResearchStatePatch, StateApplyReport) {
    let patch = supplied_patch.unwrap_or_else(|| {
        extract_deterministic_patch(
            content,
            resolved_references,
            &state.summary(),
            message_id.map(str::to_string),
        )
    });
    let report = apply_patch(state, &patch);
    let parsed = problem_understanding::understand(content);
    let lower = content.to_lowercase();
    let meaningful = report.changed
        || !patch.operations.is_empty()
        || parsed.representation.domain != "unknown"
        || !parsed.representation.objectives.is_empty()
        || !parsed.representation.constraints.is_empty()
        || !parsed.representation.assumptions.is_empty()
        || contains_any(&lower, &["假设", "如果", "论文", "paper"])
        || content.trim().ends_with(['?', '？']);
    if meaningful {
        state.revision = state.revision.saturating_add(1);
        if let Some(id) = message_id {
            push_unique(&mut state.source_message_ids, id);
        }
    }
    if parsed.representation.domain != "unknown" {
        state.active_problem = parsed.representation.domain;
    }
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
    bound_state(state);
    (patch, report)
}

fn bound_state(state: &mut ResearchSessionState) {
    state.open_questions.truncate(12);
    state.hypotheses.truncate(12);
    state.papers.truncate(16);
    state.methods.truncate(16);
    state.excluded_methods.truncate(16);
    state.objectives.truncate(16);
    state.constraints.truncate(24);
    state.assumptions.truncate(16);
    while state.parameters.len() > 16 {
        if let Some(key) = state.parameters.keys().next_back().cloned() {
            state.parameters.remove(&key);
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
    use crate::qa::state_mutation::{
        ParameterValue, PatchConfidence, ResearchStateOperation, StateAction, StateField,
        StateValue, STATE_PATCH_VERSION,
    };

    fn turn(index: usize, role: &str, content: &str) -> ConversationTurn {
        ConversationTurn {
            id: format!("m{index}"),
            role: role.to_string(),
            content: content.to_string(),
            request_id: format!("r{}", index / 2),
        }
    }

    #[test]
    fn mixed_state_mutation_removes_adds_overwrites_and_keeps_per_object() {
        let history = vec![
            turn(
                0,
                "user",
                "模型有 3 辆移动充电车，有 deadline 和障碍物，目标是最小化死亡节点。",
            ),
            turn(1, "assistant", "已记录。"),
            turn(2, "user", "先考虑 PSO 和 ALNS。"),
            turn(3, "assistant", "已记录。"),
        ];
        let state = derive(
            &history,
            "PSO 不用了，把 3 辆移动充电车改成 2 辆，deadline 保留，障碍物也保留。",
        );
        assert_eq!(
            state.parameters["mobile_charger_count"].value,
            ParameterValue::Integer(2)
        );
        assert!(!state
            .methods
            .contains(&"particle_swarm_optimization".to_string()));
        assert!(state
            .methods
            .contains(&"adaptive_large_neighborhood_search".to_string()));
        assert!(state
            .excluded_methods
            .contains(&"particle_swarm_optimization".to_string()));
        assert!(state.constraints.contains(&"deadlines".to_string()));
        assert!(state
            .constraints
            .contains(&"obstacle_avoidance".to_string()));
    }

    #[test]
    fn sentence_wide_remove_no_longer_deletes_the_kept_constraint() {
        let state = derive(
            &[],
            "加入时间窗和容量约束。去掉时间窗，但是容量约束继续保留。",
        );
        assert!(!state.constraints.contains(&"time_windows".to_string()));
        assert!(state.constraints.contains(&"battery_capacity".to_string()));
    }

    fn stress_history(message_count: usize) -> Vec<ConversationTurn> {
        assert!(matches!(message_count, 20 | 50 | 100));
        let user_count = message_count / 2;
        let mut turns = vec![
            "无线传感器网络移动充电目标是最大化网络寿命。".to_string(),
            "加入电池容量约束。".to_string(),
            "先考虑 PSO。".to_string(),
            "有 3 辆移动充电车。".to_string(),
        ];
        while turns.len() + 4 < user_count {
            turns.push("继续分析当前模型。".to_string());
        }
        turns.extend([
            "目标改成最小化死亡节点。".to_string(),
            "PSO 不用了，换成 ALNS。".to_string(),
            "增加 deadline。".to_string(),
            "移动充电车改成 2 辆。".to_string(),
        ]);
        turns
            .into_iter()
            .enumerate()
            .flat_map(|(index, content)| {
                [
                    turn(index * 2, "user", &content),
                    turn(index * 2 + 1, "assistant", "已按最新状态继续分析。"),
                ]
            })
            .collect()
    }

    #[test]
    fn twenty_fifty_and_one_hundred_message_stress_uses_canonical_latest_state() {
        for message_count in [20, 50, 100] {
            let history = stress_history(message_count);
            assert_eq!(history.len(), message_count);
            let state = derive(&history, "现在有什么方法适合这个模型？");
            assert_eq!(state.schema_version, RESEARCH_STATE_VERSION);
            assert_eq!(state.objectives, vec!["minimize_dead_nodes"]);
            assert_eq!(state.methods, vec!["adaptive_large_neighborhood_search"]);
            assert!(state.constraints.contains(&"battery_capacity".to_string()));
            assert!(state.constraints.contains(&"deadlines".to_string()));
            assert_eq!(
                state.parameters["mobile_charger_count"].value,
                ParameterValue::Integer(2)
            );
        }
    }

    #[test]
    fn semantic_custom_state_and_patch_survive_database_restart() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("qa.sqlite");
        let custom_id = "custom:constraint:00000000-0000-0000-0000-000000000003";
        {
            let mut connection = Connection::open(&path).unwrap();
            crate::qa::db_schema(&connection).unwrap();
            let state = ResearchSessionState {
                revision: 1,
                constraints: vec![custom_id.to_string()],
                last_patch_id: "patch:m1".to_string(),
                ..ResearchSessionState::default_v2()
            };
            let patch = ResearchStatePatch {
                schema_version: STATE_PATCH_VERSION.to_string(),
                patch_id: "patch:m1".to_string(),
                operations: vec![ResearchStateOperation {
                    action: StateAction::Add,
                    field: StateField::Constraint,
                    value: Some(StateValue::text(custom_id)),
                    previous_value: None,
                    confidence: PatchConfidence::High,
                }],
                confidence: PatchConfidence::High,
                source_message_id: Some("m1".to_string()),
                parameter_implicit_reference_resolved_count: 0,
                parameter_implicit_reference_rejected_count: 0,
                parameter_unknown_name_count: 0,
                parameter_state_corruption_count: 0,
            };
            let transaction = connection.transaction().unwrap();
            persist_canonical_state(
                &transaction,
                "repo",
                "session",
                "m1",
                4,
                &state,
                &patch,
                "1",
            )
            .unwrap();
            transaction.commit().unwrap();
        }
        let connection = Connection::open(&path).unwrap();
        crate::qa::db_schema(&connection).unwrap();
        let loaded = load_canonical_state(&connection, "repo", "session")
            .unwrap()
            .unwrap();
        assert_eq!(loaded.constraints, [custom_id]);
        let persisted_patch: String = connection
            .query_row(
                "SELECT patch_json FROM qa_message_state_patches WHERE message_id='m1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(persisted_patch.contains(custom_id));
    }

    #[test]
    fn legacy_session_has_no_snapshot_then_migrates_on_successful_save() {
        let mut connection = Connection::open_in_memory().unwrap();
        crate::qa::db_schema(&connection).unwrap();
        assert!(load_canonical_state(&connection, "repo", "legacy")
            .unwrap()
            .is_none());
        let history = vec![turn(0, "user", "加入 deadline 约束。")];
        let state = derive_history(&history);
        let patch = ResearchStatePatch::empty(Some("m2".to_string()));
        let transaction = connection.transaction().unwrap();
        persist_canonical_state(&transaction, "repo", "legacy", "m2", 0, &state, &patch, "2")
            .unwrap();
        transaction.commit().unwrap();
        assert!(load_canonical_state(&connection, "repo", "legacy")
            .unwrap()
            .unwrap()
            .constraints
            .contains(&"deadlines".to_string()));
    }
}
