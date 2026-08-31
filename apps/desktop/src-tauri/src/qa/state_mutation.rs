use super::state_vocabulary::{StateVocabularyRegistry, VocabularyKind};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub const STATE_PATCH_VERSION: &str = "research-state-patch-v1";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatchConfidence {
    High,
    #[default]
    Medium,
    Low,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateAction {
    Add,
    Remove,
    Keep,
    Replace,
    Set,
    SetAll,
    Clear,
}

impl StateAction {
    pub fn is_destructive(self) -> bool {
        matches!(
            self,
            Self::Remove | Self::Replace | Self::SetAll | Self::Clear
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StateField {
    Objective,
    Constraint,
    Assumption,
    Method,
    Parameter,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ParameterValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),
}

impl ParameterValue {
    pub fn search_text(&self) -> String {
        match self {
            Self::Integer(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
            Self::Boolean(value) => value.to_string(),
            Self::Text(value) => value.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResearchParameter {
    pub key: String,
    pub value: ParameterValue,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub source_message_id: Option<String>,
    #[serde(default)]
    pub updated_at_turn: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StateValue {
    Text { value: String },
    Parameter { parameter: ResearchParameter },
    TextList { values: Vec<String> },
}

impl StateValue {
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text {
            value: value.into(),
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text { value } => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResearchStateOperation {
    pub action: StateAction,
    pub field: StateField,
    #[serde(default)]
    pub value: Option<StateValue>,
    #[serde(default)]
    pub previous_value: Option<StateValue>,
    #[serde(default)]
    pub confidence: PatchConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResearchStatePatch {
    pub schema_version: String,
    pub patch_id: String,
    #[serde(default)]
    pub operations: Vec<ResearchStateOperation>,
    #[serde(default)]
    pub confidence: PatchConfidence,
    #[serde(default)]
    pub source_message_id: Option<String>,
    #[serde(skip, default)]
    pub parameter_implicit_reference_resolved_count: usize,
    #[serde(skip, default)]
    pub parameter_implicit_reference_rejected_count: usize,
    #[serde(skip, default)]
    pub parameter_unknown_name_count: usize,
    #[serde(skip, default)]
    pub parameter_state_corruption_count: usize,
}

impl ResearchStatePatch {
    pub fn empty(source_message_id: Option<String>) -> Self {
        Self {
            schema_version: STATE_PATCH_VERSION.to_string(),
            patch_id: source_message_id
                .as_deref()
                .map(|id| format!("patch:{id}"))
                .unwrap_or_else(|| "patch:current".to_string()),
            operations: Vec::new(),
            confidence: PatchConfidence::High,
            source_message_id,
            parameter_implicit_reference_resolved_count: 0,
            parameter_implicit_reference_rejected_count: 0,
            parameter_unknown_name_count: 0,
            parameter_state_corruption_count: 0,
        }
    }

    pub fn low_confidence_count(&self) -> usize {
        self.operations
            .iter()
            .filter(|operation| operation.confidence == PatchConfidence::Low)
            .count()
    }

    pub fn inherit_parameter_detection_telemetry(&mut self, detected: &Self) {
        self.parameter_implicit_reference_resolved_count =
            detected.parameter_implicit_reference_resolved_count;
        self.parameter_implicit_reference_rejected_count =
            detected.parameter_implicit_reference_rejected_count;
        self.parameter_unknown_name_count = detected.parameter_unknown_name_count;
        self.parameter_state_corruption_count = detected.parameter_state_corruption_count;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStateSummary {
    pub objectives: Vec<String>,
    pub constraints: Vec<String>,
    pub assumptions: Vec<String>,
    pub active_methods: Vec<String>,
    pub excluded_methods: Vec<String>,
    pub parameters: BTreeMap<String, ResearchParameter>,
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| value.contains(needle))
}

fn canonical_mentions(
    value: &str,
    field: StateField,
    registry: &StateVocabularyRegistry,
) -> Vec<String> {
    registry
        .exact_matches(value, VocabularyKind::from_state_field(field))
        .into_iter()
        .map(|definition| definition.id.clone())
        .collect()
}

fn parameter_key(
    value: &str,
    registry: &StateVocabularyRegistry,
) -> Option<(String, Option<String>)> {
    let matches = registry.exact_matches(value, VocabularyKind::Parameter);
    (matches.len() == 1).then(|| {
        let definition = matches[0];
        (
            definition.id.clone(),
            definition
                .parameter_spec
                .as_ref()
                .and_then(|spec| spec.unit.clone()),
        )
    })
}

fn first_number(value: &str) -> Option<ParameterValue> {
    let normalized = value.replace(['，', ',', '。', '；', ';'], " ");
    normalized
        .split_whitespace()
        .filter_map(|token| {
            let clean = token.trim_matches(|character: char| {
                !character.is_ascii_digit() && !matches!(character, '.' | '-')
            });
            if clean.is_empty() {
                None
            } else if clean.contains('.') {
                clean.parse::<f64>().ok().map(ParameterValue::Float)
            } else {
                clean.parse::<i64>().ok().map(ParameterValue::Integer)
            }
        })
        .next_back()
}

fn is_parameter_value_only_followup(clause: &str) -> bool {
    let Some(number_start) = clause
        .char_indices()
        .find_map(|(index, character)| character.is_ascii_digit().then_some(index))
    else {
        return false;
    };
    let prefix = clause[..number_start]
        .trim()
        .trim_end_matches(|character: char| {
            character.is_whitespace()
                || matches!(character, '-' | '+' | '.' | ':' | '：' | ',' | '，')
        })
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    matches!(
        prefix.as_str(),
        "改成"
            | "改为"
            | "设成"
            | "设为"
            | "设置为"
            | "还是改成"
            | "还是改为"
            | "还是设成"
            | "还是设为"
            | "还是用"
            | "这个参数改成"
            | "这个参数改为"
            | "这个参数设成"
            | "这个参数设为"
            | "这个参数设置为"
            | "那个参数改成"
            | "那个参数改为"
            | "那个参数设成"
            | "那个参数设为"
            | "那个参数设置为"
            | "它改成"
            | "它改为"
            | "changeto"
            | "setto"
            | "thisparameterto"
            | "thatparameterto"
    )
}

fn action_for_clause(clause: &str) -> Option<StateAction> {
    if contains_any(clause, &["只保留", "only keep", "set all"]) {
        Some(StateAction::SetAll)
    } else if contains_any(clause, &["清空", "不考虑任何", "clear all", "none of"]) {
        Some(StateAction::Clear)
    } else if contains_any(
        clause,
        &["换成", "替换为", "改用", "replace with", "switch to"],
    ) {
        Some(StateAction::Replace)
    } else if contains_any(
        clause,
        &[
            "去掉",
            "删除",
            "移除",
            "不用了",
            "不考虑",
            "不要了",
            "remove",
            "drop",
            "exclude",
        ],
    ) {
        Some(StateAction::Remove)
    } else if contains_any(clause, &["保留", "不变", "继续用", "keep", "remain"]) {
        Some(StateAction::Keep)
    } else if contains_any(
        clause,
        &[
            "改成",
            "改为",
            "设成",
            "设为",
            "设置",
            "还是用",
            "set to",
            "change to",
        ],
    ) {
        Some(StateAction::Set)
    } else if contains_any(
        clause,
        &[
            "增加", "加入", "添加", "使用", "考虑", "采用", "add", "include", "use ",
        ],
    ) {
        Some(StateAction::Add)
    } else {
        None
    }
}

fn clauses(message: &str) -> Vec<String> {
    message
        .split(['，', ',', '。', '；', ';', '\n'])
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn operation_for_text(
    action: StateAction,
    field: StateField,
    value: String,
) -> ResearchStateOperation {
    ResearchStateOperation {
        action,
        field,
        value: Some(StateValue::text(value)),
        previous_value: None,
        confidence: PatchConfidence::High,
    }
}

fn unique_current_value(state: &ResearchStateSummary, field: StateField) -> Option<String> {
    let values = match field {
        StateField::Objective => &state.objectives,
        StateField::Constraint => &state.constraints,
        StateField::Assumption => &state.assumptions,
        StateField::Method => &state.active_methods,
        StateField::Parameter => return None,
    };
    (values.len() == 1).then(|| values[0].clone())
}

fn explicit_field(clause: &str) -> Option<StateField> {
    if contains_any(clause, &["目标", "objective"]) {
        Some(StateField::Objective)
    } else if contains_any(clause, &["约束", "constraint"]) {
        Some(StateField::Constraint)
    } else if contains_any(clause, &["假设", "assumption"]) {
        Some(StateField::Assumption)
    } else if contains_any(clause, &["方法", "算法", "method", "algorithm"]) {
        Some(StateField::Method)
    } else if contains_any(clause, &["参数", "parameter"]) {
        Some(StateField::Parameter)
    } else {
        None
    }
}

pub fn extract_deterministic_patch(
    message: &str,
    resolved_references: &[String],
    current_state: &ResearchStateSummary,
    source_message_id: Option<String>,
) -> ResearchStatePatch {
    extract_deterministic_patch_with_registry(
        message,
        resolved_references,
        current_state,
        source_message_id,
        &StateVocabularyRegistry::default(),
    )
}

pub fn extract_deterministic_patch_with_registry(
    message: &str,
    resolved_references: &[String],
    current_state: &ResearchStateSummary,
    source_message_id: Option<String>,
    registry: &StateVocabularyRegistry,
) -> ResearchStatePatch {
    let mut patch = ResearchStatePatch::empty(source_message_id.clone());
    let mut last_targets = Vec::<(StateField, String, StateAction)>::new();
    for clause in clauses(message) {
        let action = action_for_clause(&clause);
        let mut clause_operations = Vec::new();

        let numeric_value = first_number(&clause);
        let known_parameter = parameter_key(&clause, registry);
        let set_like = matches!(action, Some(StateAction::Set | StateAction::Add));
        let parameter_target = if known_parameter.is_some() {
            known_parameter
        } else if set_like && numeric_value.is_some() && is_parameter_value_only_followup(&clause) {
            if current_state.parameters.len() == 1 {
                patch.parameter_implicit_reference_resolved_count += 1;
                current_state
                    .parameters
                    .iter()
                    .next()
                    .map(|(key, parameter)| (key.clone(), parameter.unit.clone()))
            } else {
                patch.parameter_implicit_reference_rejected_count += 1;
                None
            }
        } else {
            if set_like && numeric_value.is_some() {
                patch.parameter_implicit_reference_rejected_count += 1;
                patch.parameter_unknown_name_count += 1;
            }
            None
        };
        if let (Some((key, default_unit)), Some(value)) = (parameter_target, numeric_value) {
            let unit = if clause.contains("分钟") || clause.contains("minute") {
                Some("minute".to_string())
            } else if clause.contains("m/s") || clause.contains("米每秒") {
                Some("m/s".to_string())
            } else {
                default_unit
            };
            clause_operations.push(ResearchStateOperation {
                action: StateAction::Set,
                field: StateField::Parameter,
                value: Some(StateValue::Parameter {
                    parameter: ResearchParameter {
                        key: key.clone(),
                        value,
                        unit,
                        source_message_id: source_message_id.clone(),
                        updated_at_turn: 0,
                    },
                }),
                previous_value: None,
                confidence: PatchConfidence::High,
            });
            if key == "mobile_charger_count" {
                if let Some(ParameterValue::Integer(count)) = clause_operations
                    .last()
                    .and_then(|operation| operation.value.as_ref())
                    .and_then(|value| match value {
                        StateValue::Parameter { parameter } => Some(parameter.value.clone()),
                        _ => None,
                    })
                {
                    if count > 1 {
                        clause_operations.push(operation_for_text(
                            StateAction::Add,
                            StateField::Constraint,
                            "multi_vehicle_coordination".to_string(),
                        ));
                    }
                }
            }
        }

        for field in [
            StateField::Objective,
            StateField::Constraint,
            StateField::Assumption,
            StateField::Method,
        ] {
            let values = canonical_mentions(&clause, field, registry);
            if values.is_empty() {
                continue;
            }
            let effective_action = action.unwrap_or(StateAction::Add);
            if effective_action == StateAction::SetAll {
                clause_operations.push(ResearchStateOperation {
                    action: StateAction::SetAll,
                    field,
                    value: Some(StateValue::TextList {
                        values: values.clone(),
                    }),
                    previous_value: None,
                    confidence: PatchConfidence::High,
                });
            } else if effective_action == StateAction::Replace {
                let next = values.last().cloned().unwrap_or_default();
                let explicit_previous = (values.len() >= 2).then(|| values[0].clone());
                let prior_target = last_targets
                    .iter()
                    .rev()
                    .find(|(target_field, _, _)| *target_field == field)
                    .cloned();
                let prior_was_removed = prior_target
                    .as_ref()
                    .is_some_and(|(_, _, action)| *action == StateAction::Remove);
                if prior_was_removed {
                    clause_operations.push(operation_for_text(
                        StateAction::Add,
                        field,
                        next.clone(),
                    ));
                    last_targets.push((field, next, StateAction::Add));
                } else {
                    let previous = explicit_previous
                        .or_else(|| prior_target.map(|(_, value, _)| value))
                        .or_else(|| unique_current_value(current_state, field));
                    clause_operations.push(ResearchStateOperation {
                        action: StateAction::Replace,
                        field,
                        value: Some(StateValue::text(next.clone())),
                        previous_value: previous.map(StateValue::text),
                        confidence: PatchConfidence::High,
                    });
                    last_targets.push((field, next, StateAction::Replace));
                }
            } else {
                for value in values {
                    let reduced_action =
                        if effective_action == StateAction::Set && field != StateField::Parameter {
                            if clause_operations
                                .iter()
                                .any(|operation| operation.field == StateField::Parameter)
                            {
                                StateAction::Add
                            } else {
                                StateAction::SetAll
                            }
                        } else {
                            effective_action
                        };
                    let operation = if reduced_action == StateAction::SetAll {
                        ResearchStateOperation {
                            action: reduced_action,
                            field,
                            value: Some(StateValue::TextList {
                                values: vec![value.clone()],
                            }),
                            previous_value: None,
                            confidence: PatchConfidence::High,
                        }
                    } else {
                        operation_for_text(reduced_action, field, value.clone())
                    };
                    last_targets.push((field, value, reduced_action));
                    clause_operations.push(operation);
                }
            }
        }

        if clause_operations.is_empty() {
            if let Some(action) = action {
                let corrective =
                    contains_any(&clause, &["还是", "算了", "不对", "等等", "actually"]);
                if action == StateAction::Clear {
                    if let Some(field) = explicit_field(&clause) {
                        clause_operations.push(ResearchStateOperation {
                            action,
                            field,
                            value: None,
                            previous_value: None,
                            confidence: PatchConfidence::High,
                        });
                    }
                } else if corrective {
                    if let Some((field, value, _)) = last_targets.last().cloned() {
                        clause_operations.push(operation_for_text(action, field, value));
                    }
                } else if action.is_destructive()
                    && contains_any(
                        &clause,
                        &["那个", "这个", "第一个", "第二个", "that method", "it"],
                    )
                {
                    let field = if clause.contains("方法") || clause.contains("method") {
                        StateField::Method
                    } else {
                        StateField::Constraint
                    };
                    clause_operations.push(ResearchStateOperation {
                        action,
                        field,
                        value: resolved_references.first().cloned().map(StateValue::text),
                        previous_value: None,
                        confidence: if resolved_references.len() == 1 {
                            PatchConfidence::Medium
                        } else {
                            PatchConfidence::Low
                        },
                    });
                }
            }
        }
        patch.operations.extend(clause_operations);
    }

    if patch
        .operations
        .iter()
        .any(|operation| operation.confidence == PatchConfidence::Low)
    {
        patch.confidence = PatchConfidence::Low;
    } else if patch
        .operations
        .iter()
        .any(|operation| operation.confidence == PatchConfidence::Medium)
    {
        patch.confidence = PatchConfidence::Medium;
    } else {
        patch.confidence = PatchConfidence::High;
    }

    // An open question with no mutation remains an empty patch. Existing state is
    // deliberately not copied into operations.
    patch
}

pub fn validate_patch(mut patch: ResearchStatePatch) -> Result<ResearchStatePatch, String> {
    if patch.schema_version != STATE_PATCH_VERSION || patch.operations.len() > 32 {
        return Err("STATE_PATCH_INVALID: schema_or_count".to_string());
    }
    patch.patch_id = patch.patch_id.trim().chars().take(160).collect();
    if patch.patch_id.is_empty() {
        return Err("STATE_PATCH_INVALID: patch_id".to_string());
    }
    for operation in &mut patch.operations {
        match (&operation.field, &mut operation.value) {
            (StateField::Parameter, Some(StateValue::Parameter { parameter })) => {
                parameter.key = normalize_parameter_key(&parameter.key);
                if parameter.key.is_empty()
                    || !matches!(operation.action, StateAction::Set | StateAction::Add)
                {
                    return Err("STATE_PATCH_INVALID: parameter_operation".to_string());
                }
            }
            (StateField::Parameter, _) => {
                return Err("STATE_PATCH_INVALID: parameter_value".to_string())
            }
            (_, Some(StateValue::Text { value })) => {
                *value = normalize_text_value(value);
                if value.is_empty() {
                    return Err("STATE_PATCH_INVALID: empty_value".to_string());
                }
            }
            (_, Some(StateValue::TextList { values })) => {
                let mut seen = std::collections::HashSet::new();
                *values = values
                    .iter()
                    .map(|value| normalize_text_value(value))
                    .filter(|value| !value.is_empty() && seen.insert(value.clone()))
                    .take(16)
                    .collect();
                if values.is_empty() || operation.action != StateAction::SetAll {
                    return Err("STATE_PATCH_INVALID: set_all_value".to_string());
                }
            }
            (_, Some(StateValue::Parameter { .. })) => {
                return Err("STATE_PATCH_INVALID: field_value_mismatch".to_string())
            }
            (_, None)
                if operation.action != StateAction::Clear
                    && operation.confidence != PatchConfidence::Low =>
            {
                return Err("STATE_PATCH_INVALID: missing_value".to_string())
            }
            _ => {}
        }
    }
    Ok(patch)
}

fn normalize_text_value(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .filter(|character| character.is_alphanumeric() || matches!(character, '_' | '-'))
        .take(120)
        .collect()
}

fn normalize_parameter_key(value: &str) -> String {
    let lowered = value.trim().to_lowercase();
    let (explicit_custom, value) = lowered
        .strip_prefix("custom:")
        .map_or((false, lowered.as_str()), |suffix| (true, suffix));
    let normalized = value
        .trim()
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    let normalized = normalized.trim_matches('_');
    const KNOWN: &[&str] = &[
        "mobile_charger_count",
        "node_count",
        "battery_capacity",
        "charger_capacity",
        "deadline",
        "time_horizon",
        "charger_speed",
        "transmission_loss",
        "energy_threshold",
    ];
    if !explicit_custom && KNOWN.contains(&normalized) {
        normalized.to_string()
    } else if normalized.is_empty() {
        String::new()
    } else {
        format!("custom:{}", normalized.chars().take(64).collect::<String>())
    }
}

pub fn state_patch_schema() -> Value {
    let text_value = json!({
        "type":"object","additionalProperties":false,"required":["type","value"],
        "properties":{"type":{"type":"string","const":"text"},"value":{"type":"string","minLength":1,"maxLength":120}}
    });
    let text_list = json!({
        "type":"object","additionalProperties":false,"required":["type","values"],
        "properties":{"type":{"type":"string","const":"text_list"},"values":{"type":"array","minItems":1,"maxItems":16,"uniqueItems":true,"items":{"type":"string","minLength":1,"maxLength":120}}}
    });
    let parameter = json!({
        "type":"object","additionalProperties":false,"required":["type","parameter"],
        "properties":{"type":{"type":"string","const":"parameter"},"parameter":{"type":"object","additionalProperties":false,"required":["key","value","unit","sourceMessageId","updatedAtTurn"],"properties":{
            "key":{"type":"string","minLength":1,"maxLength":80},
            "value":{"oneOf":[
                {"type":"object","additionalProperties":false,"required":["type","value"],"properties":{"type":{"const":"integer"},"value":{"type":"integer"}}},
                {"type":"object","additionalProperties":false,"required":["type","value"],"properties":{"type":{"const":"float"},"value":{"type":"number"}}},
                {"type":"object","additionalProperties":false,"required":["type","value"],"properties":{"type":{"const":"boolean"},"value":{"type":"boolean"}}},
                {"type":"object","additionalProperties":false,"required":["type","value"],"properties":{"type":{"const":"text"},"value":{"type":"string","maxLength":120}}}
            ]},
            "unit":{"type":["string","null"],"maxLength":32},"sourceMessageId":{"type":["string","null"],"maxLength":160},"updatedAtTurn":{"type":"integer","minimum":0}
        }}}
    });
    json!({
        "type":"object","additionalProperties":false,
        "required":["schemaVersion","patchId","operations","confidence","sourceMessageId"],
        "properties":{
            "schemaVersion":{"type":"string","const":STATE_PATCH_VERSION},
            "patchId":{"type":"string","minLength":1,"maxLength":160},
            "confidence":{"type":"string","enum":["high","medium","low"]},
            "sourceMessageId":{"type":["string","null"],"maxLength":160},
            "operations":{"type":"array","maxItems":32,"items":{"type":"object","additionalProperties":false,
                "required":["action","field","value","previousValue","confidence"],"properties":{
                    "action":{"type":"string","enum":["add","remove","keep","replace","set","set_all","clear"]},
                    "field":{"type":"string","enum":["objective","constraint","assumption","method","parameter"]},
                    "value":{"oneOf":[text_value.clone(),text_list,parameter,{"type":"null"}]},
                    "previousValue":{"oneOf":[text_value,{"type":"null"}]},
                    "confidence":{"type":"string","enum":["high","medium","low"]}
                }
            }}
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::state_vocabulary::{StateFieldDefinition, VocabularyKind, VocabularyOrigin};

    #[test]
    fn deterministic_patch_is_clause_local_and_preserves_operation_order() {
        let patch = extract_deterministic_patch(
            "PSO 不用了，换成 ALNS，3 辆移动充电车改成 2 辆，deadline 保留。",
            &[],
            &ResearchStateSummary::default(),
            Some("u1".into()),
        );
        assert!(patch.operations.iter().any(|operation| {
            operation.action == StateAction::Remove
                && operation.value.as_ref().and_then(StateValue::as_text)
                    == Some("particle_swarm_optimization")
        }));
        assert!(patch.operations.iter().any(|operation| {
            operation.action == StateAction::Add
                && operation.value.as_ref().and_then(StateValue::as_text)
                    == Some("adaptive_large_neighborhood_search")
        }));
        assert!(patch.operations.iter().any(|operation| {
            matches!(operation.value, Some(StateValue::Parameter { ref parameter }) if parameter.key == "mobile_charger_count" && parameter.value == ParameterValue::Integer(2))
        }));
        assert!(patch.operations.iter().any(|operation| {
            operation.action == StateAction::Keep
                && operation.value.as_ref().and_then(StateValue::as_text) == Some("deadlines")
        }));
    }

    #[test]
    fn multiple_mobile_chargers_are_a_coordination_constraint() {
        let patch = extract_deterministic_patch(
            "无线传感器网络使用多个移动充电车和节点 deadline。",
            &[],
            &ResearchStateSummary::default(),
            Some("u1".into()),
        );

        assert!(patch.operations.iter().any(|operation| {
            operation.field == StateField::Constraint
                && operation.action == StateAction::Add
                && operation.value.as_ref().and_then(StateValue::as_text)
                    == Some("multi_vehicle_coordination")
        }));
    }

    #[test]
    fn registry_parser_handles_builtin_paraphrases_count_and_capacity_without_collision() {
        let state = ResearchStateSummary::default();
        let paraphrase =
            extract_deterministic_patch("模型需要多套移动供能设备共同调度。", &[], &state, None);
        assert!(paraphrase.operations.iter().any(|operation| {
            operation.field == StateField::Constraint
                && operation.value.as_ref().and_then(StateValue::as_text)
                    == Some("multi_vehicle_coordination")
        }));

        let count = extract_deterministic_patch("有 3 个移动充电器。", &[], &state, None);
        assert!(matches!(
            count.operations.first().and_then(|operation| operation.value.as_ref()),
            Some(StateValue::Parameter { parameter })
                if parameter.key == "mobile_charger_count"
                    && parameter.value == ParameterValue::Integer(3)
        ));

        let capacity = extract_deterministic_patch("充电器容量=50。", &[], &state, None);
        assert!(capacity.operations.iter().any(|operation| matches!(
            operation.value,
            Some(StateValue::Parameter { ref parameter })
                if parameter.key == "charger_capacity"
                    && parameter.value == ParameterValue::Integer(50)
        )));
        assert!(capacity.operations.iter().all(|operation| !matches!(
            operation.value,
            Some(StateValue::Parameter { ref parameter })
                if parameter.key == "mobile_charger_count"
        )));
    }

    #[test]
    fn repository_custom_exact_alias_is_available_to_deterministic_fast_path() {
        let custom = StateFieldDefinition {
            id: "custom:constraint:fixture".to_string(),
            kind: VocabularyKind::Constraint,
            label: "高温环境约束".to_string(),
            description: "环境温度过高时必须考虑充电效率和电池安全。".to_string(),
            aliases: vec!["温度过高".to_string()],
            examples: Vec::new(),
            parameter_spec: None,
            origin: VocabularyOrigin::Custom,
            enabled: true,
        };
        let registry = StateVocabularyRegistry::merged(1, vec![custom]);
        let patch = extract_deterministic_patch_with_registry(
            "还要考虑温度过高。",
            &[],
            &ResearchStateSummary::default(),
            None,
            &registry,
        );
        assert_eq!(
            patch.operations[0]
                .value
                .as_ref()
                .and_then(StateValue::as_text),
            Some("custom:constraint:fixture")
        );
    }

    #[test]
    fn ambiguous_destructive_reference_is_low_confidence() {
        let patch = extract_deterministic_patch(
            "那个方法不要了。",
            &[],
            &ResearchStateSummary::default(),
            None,
        );
        assert_eq!(patch.low_confidence_count(), 1);
        assert_eq!(patch.operations[0].field, StateField::Method);
    }

    #[test]
    fn parameter_follow_up_infers_the_only_active_parameter() {
        let mut current = ResearchStateSummary::default();
        current.parameters.insert(
            "mobile_charger_count".to_string(),
            ResearchParameter {
                key: "mobile_charger_count".to_string(),
                value: ParameterValue::Integer(3),
                unit: None,
                source_message_id: Some("m1".to_string()),
                updated_at_turn: 1,
            },
        );
        let patch =
            extract_deterministic_patch("改成 2 辆。", &[], &current, Some("m2".to_string()));
        assert!(matches!(
            patch
                .operations
                .first()
                .and_then(|operation| operation.value.as_ref()),
            Some(StateValue::Parameter { parameter })
                if parameter.key == "mobile_charger_count"
                    && parameter.value == ParameterValue::Integer(2)
        ));
    }

    #[test]
    fn replace_binds_previous_and_next_objects_without_a_fixed_pair_rule() {
        let patch = extract_deterministic_patch(
            "PSO 换成 ALNS。",
            &[],
            &ResearchStateSummary::default(),
            Some("m3".to_string()),
        );
        assert_eq!(patch.operations.len(), 1);
        assert_eq!(patch.operations[0].action, StateAction::Replace);
        assert_eq!(
            patch.operations[0]
                .previous_value
                .as_ref()
                .and_then(StateValue::as_text),
            Some("particle_swarm_optimization")
        );
        assert_eq!(
            patch.operations[0]
                .value
                .as_ref()
                .and_then(StateValue::as_text),
            Some("adaptive_large_neighborhood_search")
        );
    }

    #[test]
    fn clear_is_extracted_as_a_field_scoped_operation() {
        let patch = extract_deterministic_patch(
            "清空所有方法。",
            &[],
            &ResearchStateSummary::default(),
            None,
        );
        assert_eq!(patch.operations.len(), 1);
        assert_eq!(patch.operations[0].action, StateAction::Clear);
        assert_eq!(patch.operations[0].field, StateField::Method);
        assert!(patch.operations[0].value.is_none());
    }

    fn parameter(key: &str, value: ParameterValue, unit: Option<&str>) -> ResearchParameter {
        ResearchParameter {
            key: key.to_string(),
            value,
            unit: unit.map(str::to_string),
            source_message_id: Some("fixture".to_string()),
            updated_at_turn: 1,
        }
    }

    #[test]
    fn unknown_named_parameter_never_inherits_the_only_existing_parameter() {
        let mut current = ResearchStateSummary::default();
        current.parameters.insert(
            "mobile_charger_count".to_string(),
            parameter("mobile_charger_count", ParameterValue::Integer(2), None),
        );
        for message in [
            "充电功率改成 50W。",
            "无线充电效率改成 0.82。",
            "最大转向角设置为 30 度。",
            "单次最大服务时间改成 15 分钟。",
            "路径惩罚系数设成 0.4。",
        ] {
            let patch = extract_deterministic_patch(message, &[], &current, None);
            assert!(patch.operations.is_empty(), "{message}");
            assert_eq!(patch.parameter_implicit_reference_rejected_count, 1);
            assert_eq!(patch.parameter_unknown_name_count, 1);
            assert_eq!(patch.parameter_state_corruption_count, 0);
        }
    }

    #[test]
    fn value_only_followup_requires_exactly_one_parameter_candidate() {
        let mut unique = ResearchStateSummary::default();
        unique.parameters.insert(
            "deadline".to_string(),
            parameter("deadline", ParameterValue::Integer(30), Some("minute")),
        );
        let resolved = extract_deterministic_patch("还是改成 20 分钟吧。", &[], &unique, None);
        assert_eq!(resolved.parameter_implicit_reference_resolved_count, 1);
        assert!(matches!(
            resolved.operations.first().and_then(|operation| operation.value.as_ref()),
            Some(StateValue::Parameter { parameter })
                if parameter.key == "deadline"
                    && parameter.value == ParameterValue::Integer(20)
                    && parameter.unit.as_deref() == Some("minute")
        ));

        unique.parameters.insert(
            "mobile_charger_count".to_string(),
            parameter("mobile_charger_count", ParameterValue::Integer(2), None),
        );
        let rejected = extract_deterministic_patch("改成 20。", &[], &unique, None);
        assert!(rejected.operations.is_empty());
        assert_eq!(rejected.parameter_implicit_reference_rejected_count, 1);
        assert_eq!(rejected.parameter_state_corruption_count, 0);
    }

    #[test]
    fn known_and_structured_custom_parameters_do_not_overwrite_existing_keys() {
        let mut current = ResearchStateSummary::default();
        current.parameters.insert(
            "mobile_charger_count".to_string(),
            parameter("mobile_charger_count", ParameterValue::Integer(2), None),
        );
        let known = extract_deterministic_patch("deadline 改成 40 分钟。", &[], &current, None);
        assert!(matches!(
            known.operations.first().and_then(|operation| operation.value.as_ref()),
            Some(StateValue::Parameter { parameter }) if parameter.key == "deadline"
        ));

        for key in ["charging_power", "custom:charging_power"] {
            let mut structured = ResearchStatePatch::empty(None);
            structured.operations.push(ResearchStateOperation {
                action: StateAction::Set,
                field: StateField::Parameter,
                value: Some(StateValue::Parameter {
                    parameter: parameter(key, ParameterValue::Integer(50), Some("W")),
                }),
                previous_value: None,
                confidence: PatchConfidence::High,
            });
            let validated = validate_patch(structured).unwrap();
            assert!(matches!(
                validated.operations[0].value,
                Some(StateValue::Parameter { ref parameter })
                    if parameter.key == "custom:charging_power"
            ));
        }
    }
}
