use super::research_memory::ResearchSessionState;
use super::state_mutation::{
    PatchConfidence, ResearchParameter, ResearchStateOperation, ResearchStatePatch, StateAction,
    StateField, StateValue,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateApplyWarning {
    KeepTargetMissing,
    RemoveTargetMissing,
    ReplaceSourceMissing,
    LowConfidenceDestructiveRejected,
    InvalidFieldValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateApplyReport {
    pub applied_operations: Vec<ResearchStateOperation>,
    pub rejected_operations: Vec<ResearchStateOperation>,
    pub warnings: Vec<StateApplyWarning>,
    pub changed: bool,
}

fn field_values_mut(
    state: &mut ResearchSessionState,
    field: StateField,
) -> Option<&mut Vec<String>> {
    match field {
        StateField::Objective => Some(&mut state.objectives),
        StateField::Constraint => Some(&mut state.constraints),
        StateField::Assumption => Some(&mut state.assumptions),
        StateField::Method => Some(&mut state.methods),
        StateField::Parameter => None,
    }
}

fn text(value: &Option<StateValue>) -> Option<&str> {
    value.as_ref().and_then(StateValue::as_text)
}

fn insert_unique(values: &mut Vec<String>, value: &str) -> bool {
    if values.iter().any(|current| current == value) {
        false
    } else {
        values.push(value.to_string());
        true
    }
}

fn remove_value(values: &mut Vec<String>, value: &str) -> bool {
    let before = values.len();
    values.retain(|current| current != value);
    before != values.len()
}

fn parameter(value: &Option<StateValue>) -> Option<ResearchParameter> {
    match value {
        Some(StateValue::Parameter { parameter }) => Some(parameter.clone()),
        _ => None,
    }
}

fn text_list(value: &Option<StateValue>) -> Option<Vec<String>> {
    match value {
        Some(StateValue::TextList { values }) => Some(values.clone()),
        _ => None,
    }
}

fn reject(
    report: &mut StateApplyReport,
    operation: &ResearchStateOperation,
    warning: StateApplyWarning,
) {
    report.rejected_operations.push(operation.clone());
    report.warnings.push(warning);
}

pub fn apply_patch(
    state: &mut ResearchSessionState,
    patch: &ResearchStatePatch,
) -> StateApplyReport {
    let initial_state = state.clone();
    let mut report = StateApplyReport::default();
    for operation in &patch.operations {
        if operation.action.is_destructive() && operation.confidence == PatchConfidence::Low {
            reject(
                &mut report,
                operation,
                StateApplyWarning::LowConfidenceDestructiveRejected,
            );
            continue;
        }
        let changed = match operation.action {
            StateAction::Set if operation.field == StateField::Parameter => {
                let Some(mut parameter) = parameter(&operation.value) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                parameter.updated_at_turn = state.revision.saturating_add(1);
                let changed = state.parameters.get(&parameter.key) != Some(&parameter);
                state.parameters.insert(parameter.key.clone(), parameter);
                changed
            }
            StateAction::Add => {
                let Some(value) = text(&operation.value) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                let Some(values) = field_values_mut(state, operation.field) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                let changed = insert_unique(values, value);
                if operation.field == StateField::Method {
                    remove_value(&mut state.excluded_methods, value);
                }
                changed
            }
            StateAction::Remove => {
                let Some(value) = text(&operation.value) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                let Some(values) = field_values_mut(state, operation.field) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                let changed = remove_value(values, value);
                if !changed {
                    report.warnings.push(StateApplyWarning::RemoveTargetMissing);
                } else if operation.field == StateField::Method {
                    insert_unique(&mut state.excluded_methods, value);
                }
                changed
            }
            StateAction::Keep => {
                let Some(value) = text(&operation.value) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                let initially_present = match operation.field {
                    StateField::Objective => initial_state.objectives.contains(&value.to_string()),
                    StateField::Constraint => {
                        initial_state.constraints.contains(&value.to_string())
                    }
                    StateField::Assumption => {
                        initial_state.assumptions.contains(&value.to_string())
                    }
                    StateField::Method => initial_state.methods.contains(&value.to_string()),
                    StateField::Parameter => false,
                };
                let Some(values) = field_values_mut(state, operation.field) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                if values.iter().any(|current| current == value) {
                    false
                } else if initially_present {
                    insert_unique(values, value)
                } else {
                    report.warnings.push(StateApplyWarning::KeepTargetMissing);
                    false
                }
            }
            StateAction::Replace => {
                let Some(previous) = text(&operation.previous_value) else {
                    reject(
                        &mut report,
                        operation,
                        StateApplyWarning::ReplaceSourceMissing,
                    );
                    continue;
                };
                let Some(next) = text(&operation.value) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                let Some(values) = field_values_mut(state, operation.field) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                if !values.iter().any(|current| current == previous) {
                    reject(
                        &mut report,
                        operation,
                        StateApplyWarning::ReplaceSourceMissing,
                    );
                    continue;
                }
                remove_value(values, previous);
                insert_unique(values, next);
                if operation.field == StateField::Method {
                    insert_unique(&mut state.excluded_methods, previous);
                    remove_value(&mut state.excluded_methods, next);
                }
                true
            }
            StateAction::SetAll => {
                let Some(next) = text_list(&operation.value) else {
                    reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                    continue;
                };
                let (previous, current) = {
                    let Some(values) = field_values_mut(state, operation.field) else {
                        reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                        continue;
                    };
                    let previous = values.clone();
                    *values = next;
                    (previous, values.clone())
                };
                if operation.field == StateField::Method {
                    for removed in previous.iter().filter(|value| !current.contains(value)) {
                        insert_unique(&mut state.excluded_methods, removed);
                    }
                    for active in &current {
                        remove_value(&mut state.excluded_methods, active);
                    }
                }
                previous != current
            }
            StateAction::Clear => {
                if operation.field == StateField::Parameter {
                    let changed = !state.parameters.is_empty();
                    state.parameters.clear();
                    changed
                } else {
                    let Some(values) = field_values_mut(state, operation.field) else {
                        reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                        continue;
                    };
                    let previous = values.clone();
                    values.clear();
                    if operation.field == StateField::Method {
                        for removed in &previous {
                            insert_unique(&mut state.excluded_methods, removed);
                        }
                    }
                    !previous.is_empty()
                }
            }
            StateAction::Set => {
                reject(&mut report, operation, StateApplyWarning::InvalidFieldValue);
                continue;
            }
        };
        report.changed |= changed;
        report.applied_operations.push(operation.clone());
    }
    if !patch.operations.is_empty() {
        state.last_patch_id = patch.patch_id.clone();
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::state_mutation::{ParameterValue, ResearchStatePatch, StateValue};

    fn operation(action: StateAction, field: StateField, value: &str) -> ResearchStateOperation {
        ResearchStateOperation {
            action,
            field,
            value: Some(StateValue::text(value)),
            previous_value: None,
            confidence: PatchConfidence::High,
        }
    }

    fn patch(operations: Vec<ResearchStateOperation>) -> ResearchStatePatch {
        ResearchStatePatch {
            schema_version: "research-state-patch-v1".into(),
            patch_id: "fixture".into(),
            operations,
            confidence: PatchConfidence::High,
            source_message_id: None,
            parameter_implicit_reference_resolved_count: 0,
            parameter_implicit_reference_rejected_count: 0,
            parameter_unknown_name_count: 0,
            parameter_state_corruption_count: 0,
        }
    }

    #[test]
    fn reducer_add_remove_keep_and_ordered_self_correction() {
        let mut state = ResearchSessionState {
            constraints: vec!["deadlines".into(), "battery_capacity".into()],
            ..ResearchSessionState::default_v2()
        };
        let report = apply_patch(
            &mut state,
            &patch(vec![
                operation(StateAction::Remove, StateField::Constraint, "deadlines"),
                operation(StateAction::Keep, StateField::Constraint, "deadlines"),
            ]),
        );
        assert!(state.constraints.contains(&"deadlines".to_string()));
        assert!(!report
            .warnings
            .contains(&StateApplyWarning::KeepTargetMissing));

        apply_patch(
            &mut state,
            &patch(vec![operation(
                StateAction::Add,
                StateField::Constraint,
                "deadlines",
            )]),
        );
        assert!(state.constraints.contains(&"deadlines".to_string()));
    }

    #[test]
    fn reducer_replace_missing_source_and_ambiguous_remove_fail_closed() {
        let mut state = ResearchSessionState {
            methods: vec!["particle_swarm_optimization".into()],
            ..ResearchSessionState::default_v2()
        };
        let mut replace = operation(
            StateAction::Replace,
            StateField::Method,
            "adaptive_large_neighborhood_search",
        );
        replace.previous_value = Some(StateValue::text("missing_method"));
        let ambiguous = ResearchStateOperation {
            action: StateAction::Remove,
            field: StateField::Method,
            value: None,
            previous_value: None,
            confidence: PatchConfidence::Low,
        };
        let report = apply_patch(&mut state, &patch(vec![replace, ambiguous]));
        assert_eq!(state.methods, vec!["particle_swarm_optimization"]);
        assert_eq!(report.rejected_operations.len(), 2);
        assert!(!report.changed);
    }

    #[test]
    fn reducer_parameter_set_overwrites_three_with_two() {
        let mut state = ResearchSessionState::default_v2();
        let set = |value| ResearchStateOperation {
            action: StateAction::Set,
            field: StateField::Parameter,
            value: Some(StateValue::Parameter {
                parameter: ResearchParameter {
                    key: "mobile_charger_count".into(),
                    value: ParameterValue::Integer(value),
                    unit: None,
                    source_message_id: None,
                    updated_at_turn: 0,
                },
            }),
            previous_value: None,
            confidence: PatchConfidence::High,
        };
        apply_patch(&mut state, &patch(vec![set(3)]));
        apply_patch(&mut state, &patch(vec![set(2)]));
        assert_eq!(
            state.parameters["mobile_charger_count"].value,
            ParameterValue::Integer(2)
        );
    }

    #[test]
    fn reducer_set_all_and_clear_are_field_scoped() {
        let mut state = ResearchSessionState {
            constraints: vec![
                "deadlines".into(),
                "battery_capacity".into(),
                "obstacle_avoidance".into(),
            ],
            methods: vec!["particle_swarm_optimization".into()],
            ..ResearchSessionState::default_v2()
        };
        apply_patch(
            &mut state,
            &patch(vec![ResearchStateOperation {
                action: StateAction::SetAll,
                field: StateField::Constraint,
                value: Some(StateValue::TextList {
                    values: vec!["deadlines".into(), "battery_capacity".into()],
                }),
                previous_value: None,
                confidence: PatchConfidence::High,
            }]),
        );
        assert_eq!(state.constraints, vec!["deadlines", "battery_capacity"]);
        assert_eq!(state.methods, vec!["particle_swarm_optimization"]);
        apply_patch(
            &mut state,
            &patch(vec![ResearchStateOperation {
                action: StateAction::Clear,
                field: StateField::Method,
                value: None,
                previous_value: None,
                confidence: PatchConfidence::High,
            }]),
        );
        assert!(state.methods.is_empty());
        assert_eq!(state.excluded_methods, vec!["particle_swarm_optimization"]);
    }
}
