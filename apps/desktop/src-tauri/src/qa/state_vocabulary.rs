use super::state_mutation::{
    ParameterValue, ResearchStatePatch, StateAction, StateField, StateValue,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};

pub const STATE_VOCABULARY_VERSION: &str = "qa-state-vocabulary-v1";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum VocabularyKind {
    Objective,
    Constraint,
    Assumption,
    Method,
    Parameter,
}

impl VocabularyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Objective => "objective",
            Self::Constraint => "constraint",
            Self::Assumption => "assumption",
            Self::Method => "method",
            Self::Parameter => "parameter",
        }
    }

    pub fn from_state_field(field: StateField) -> Self {
        match field {
            StateField::Objective => Self::Objective,
            StateField::Constraint => Self::Constraint,
            StateField::Assumption => Self::Assumption,
            StateField::Method => Self::Method,
            StateField::Parameter => Self::Parameter,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParameterValueKind {
    Integer,
    Float,
    Boolean,
    Text,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParameterSpec {
    pub value_kind: ParameterValueKind,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub minimum: Option<f64>,
    #[serde(default)]
    pub maximum: Option<f64>,
    #[serde(default)]
    pub enum_values: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VocabularyOrigin {
    BuiltIn,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateFieldDefinition {
    pub id: String,
    pub kind: VocabularyKind,
    pub label: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub examples: Vec<String>,
    #[serde(default)]
    pub parameter_spec: Option<ParameterSpec>,
    pub origin: VocabularyOrigin,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateVocabularyRegistry {
    pub schema_version: String,
    pub revision: u64,
    pub fields: Vec<StateFieldDefinition>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AllowedStateField {
    pub id: String,
    pub kind: VocabularyKind,
    pub label: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub examples: Vec<String>,
    pub parameter_spec: Option<ParameterSpec>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VocabularyValidationStats {
    pub unknown_id_count: usize,
    pub kind_mismatch_count: usize,
    pub disabled_field_count: usize,
    pub value_type_mismatch_count: usize,
    pub value_out_of_range_count: usize,
}

fn field(
    id: &str,
    kind: VocabularyKind,
    label: &str,
    description: &str,
    aliases: &[&str],
    examples: &[&str],
) -> StateFieldDefinition {
    StateFieldDefinition {
        id: id.to_string(),
        kind,
        label: label.to_string(),
        description: description.to_string(),
        aliases: aliases.iter().map(|value| (*value).to_string()).collect(),
        examples: examples.iter().map(|value| (*value).to_string()).collect(),
        parameter_spec: None,
        origin: VocabularyOrigin::BuiltIn,
        enabled: true,
    }
}

fn parameter(
    id: &str,
    label: &str,
    description: &str,
    aliases: &[&str],
    value_kind: ParameterValueKind,
    unit: Option<&str>,
    minimum: Option<f64>,
) -> StateFieldDefinition {
    let mut definition = field(
        id,
        VocabularyKind::Parameter,
        label,
        description,
        aliases,
        &[],
    );
    definition.parameter_spec = Some(ParameterSpec {
        value_kind,
        unit: unit.map(str::to_string),
        minimum,
        maximum: None,
        enum_values: Vec::new(),
    });
    definition
}

pub fn built_in_fields() -> Vec<StateFieldDefinition> {
    use VocabularyKind::{Assumption, Constraint, Method, Objective};
    vec![
        field(
            "minimize_dead_nodes",
            Objective,
            "最小化死亡节点",
            "减少能量耗尽的传感器节点数量。",
            &["最小化死亡节点", "减少死亡节点", "minimize dead nodes"],
            &[],
        ),
        field(
            "maximize_network_lifetime",
            Objective,
            "最大化网络寿命",
            "延长无线传感器网络的有效运行寿命。",
            &["最大化网络寿命", "maximize network lifetime"],
            &[],
        ),
        field(
            "minimize_delay",
            Objective,
            "最小化延迟",
            "降低调度或服务完成延迟。",
            &["最小化延迟", "降低延迟", "minimize delay"],
            &[],
        ),
        field(
            "minimize_travel_distance",
            Objective,
            "最小化行驶距离",
            "降低移动充电设备的总行驶距离。",
            &["最小化距离", "最小化行驶距离", "minimize distance"],
            &[],
        ),
        field(
            "time_windows",
            Constraint,
            "时间窗",
            "服务或访问必须发生在指定时间窗口内。",
            &["时间窗", "time window"],
            &[],
        ),
        field(
            "deadlines",
            Constraint,
            "截止期限",
            "任务或充电服务必须在截止时间前完成。",
            &["deadline", "截止时间", "最后期限"],
            &[],
        ),
        field(
            "battery_capacity",
            Constraint,
            "电池容量约束",
            "节点或设备受有限电池容量约束。",
            &["容量约束", "电池容量", "battery capacity"],
            &[],
        ),
        field(
            "obstacle_avoidance",
            Constraint,
            "障碍规避",
            "移动路径必须避开障碍物。",
            &["障碍物", "避障", "obstacle"],
            &[],
        ),
        field(
            "multi_vehicle_coordination",
            Constraint,
            "多移动充电设备协同",
            "存在多个移动充电车、移动充电器或移动供能设备，需要进行协同调度。",
            &[
                "多车协同",
                "多辆车",
                "多充电车",
                "多个移动充电车",
                "多辆移动充电车",
                "多个移动充电器",
                "多套移动供能设备",
                "multiple mobile chargers",
                "multi vehicle",
            ],
            &["多个移动充电器", "多套移动供能设备共同调度"],
        ),
        field(
            "transmission_loss",
            Constraint,
            "传输损耗约束",
            "通信或能量传输存在损耗。",
            &["传输损耗", "packet loss", "transmission loss"],
            &[],
        ),
        field(
            "stationary_sensor_nodes",
            Assumption,
            "静态传感器节点",
            "传感器节点的位置在规划期间保持不变。",
            &["节点静态", "静态节点", "stationary nodes"],
            &[],
        ),
        field(
            "single_mobile_charger",
            Assumption,
            "单移动充电设备",
            "系统只有一个移动充电设备。",
            &["单充电车", "single charger"],
            &[],
        ),
        field(
            "connected_graph",
            Assumption,
            "连通图",
            "网络拓扑保持连通。",
            &["连通图", "connected graph"],
            &[],
        ),
        field(
            "particle_swarm_optimization",
            Method,
            "粒子群优化",
            "使用粒子群优化方法求解。",
            &["pso", "粒子群"],
            &[],
        ),
        field(
            "adaptive_large_neighborhood_search",
            Method,
            "自适应大邻域搜索",
            "使用自适应大邻域搜索方法求解。",
            &["alns", "自适应大邻域"],
            &[],
        ),
        field(
            "mixed_integer_linear_programming",
            Method,
            "混合整数线性规划",
            "使用混合整数线性规划建模或求解。",
            &["milp", "混合整数线性规划"],
            &[],
        ),
        field(
            "genetic_algorithm",
            Method,
            "遗传算法",
            "使用遗传算法求解。",
            &["遗传算法", "genetic algorithm", " ga "],
            &[],
        ),
        field(
            "reinforcement_learning",
            Method,
            "强化学习",
            "使用强化学习方法求解。",
            &["强化学习", "reinforcement learning"],
            &[],
        ),
        field(
            "dynamic_programming",
            Method,
            "动态规划",
            "使用动态规划方法求解。",
            &["动态规划", "dynamic programming"],
            &[],
        ),
        field(
            "greedy",
            Method,
            "贪心方法",
            "使用贪心策略求解。",
            &["贪心", "greedy"],
            &[],
        ),
        parameter(
            "mobile_charger_count",
            "移动充电设备数量",
            "参与调度的移动充电车、移动充电器或移动供能设备数量。",
            &[
                "移动充电车",
                "充电车数量",
                "移动充电器数量",
                "mobile charger",
            ],
            ParameterValueKind::Integer,
            None,
            Some(1.0),
        ),
        parameter(
            "node_count",
            "节点数量",
            "无线传感器网络中的节点数量。",
            &["节点数", "sensor nodes", "node count"],
            ParameterValueKind::Integer,
            None,
            Some(1.0),
        ),
        parameter(
            "battery_capacity",
            "电池容量",
            "节点或设备的电池容量数值。",
            &["电池容量", "battery capacity"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
        parameter(
            "charger_capacity",
            "充电器容量",
            "充电器可用的容量数值。",
            &["充电器容量", "charger capacity"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
        parameter(
            "deadline",
            "截止时间",
            "任务截止时间参数。",
            &["deadline", "截止时间"],
            ParameterValueKind::Float,
            Some("minute"),
            Some(0.0),
        ),
        parameter(
            "time_horizon",
            "时间范围",
            "规划或仿真的时间范围。",
            &["时间范围", "time horizon"],
            ParameterValueKind::Float,
            Some("minute"),
            Some(0.0),
        ),
        parameter(
            "charger_speed",
            "充电车速度",
            "移动充电设备的行驶速度。",
            &["充电车速度", "charger speed"],
            ParameterValueKind::Float,
            Some("m/s"),
            Some(0.0),
        ),
        parameter(
            "transmission_loss",
            "传输损耗",
            "通信或能量传输的损耗参数。",
            &["传输损耗", "transmission loss"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
        parameter(
            "energy_threshold",
            "能量阈值",
            "触发服务或告警的剩余能量阈值。",
            &["能量阈值", "energy threshold"],
            ParameterValueKind::Float,
            None,
            Some(0.0),
        ),
    ]
}

impl Default for StateVocabularyRegistry {
    fn default() -> Self {
        Self {
            schema_version: STATE_VOCABULARY_VERSION.to_string(),
            revision: 0,
            fields: built_in_fields(),
        }
    }
}

impl StateVocabularyRegistry {
    pub fn merged(revision: u64, custom_fields: Vec<StateFieldDefinition>) -> Self {
        let mut fields = built_in_fields();
        fields.extend(custom_fields);
        fields.sort_by(|left, right| left.id.cmp(&right.id));
        Self {
            schema_version: STATE_VOCABULARY_VERSION.to_string(),
            revision,
            fields,
        }
    }

    pub fn field(&self, id: &str) -> Option<&StateFieldDefinition> {
        self.fields.iter().find(|field| field.id == id)
    }

    pub fn enabled_custom_count(&self) -> usize {
        self.fields
            .iter()
            .filter(|field| field.origin == VocabularyOrigin::Custom && field.enabled)
            .count()
    }

    pub fn hash(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        format!("{:x}", Sha256::digest(bytes))
    }

    pub fn allowed_state_fields(&self) -> Vec<AllowedStateField> {
        self.fields
            .iter()
            .filter(|field| field.enabled)
            .map(|field| AllowedStateField {
                id: field.id.clone(),
                kind: field.kind,
                label: field.label.clone(),
                description: field.description.clone(),
                aliases: field.aliases.iter().take(6).cloned().collect(),
                examples: field.examples.iter().take(3).cloned().collect(),
                parameter_spec: field.parameter_spec.clone(),
            })
            .collect()
    }

    /// Returns every field whose canonical ID, label, or alias appears verbatim in
    /// the normalized text. Callers must treat more than one result as ambiguous.
    pub fn exact_matches(&self, text: &str, kind: VocabularyKind) -> Vec<&StateFieldDefinition> {
        let normalized = text.trim().to_lowercase();
        let mut matches = self
            .fields
            .iter()
            .filter(|field| field.enabled && field.kind == kind)
            .filter(|field| {
                normalized.contains(&field.id.to_lowercase())
                    || normalized.contains(&field.label.to_lowercase())
                    || field.aliases.iter().any(|alias| {
                        let alias = alias.trim().to_lowercase();
                        !alias.is_empty() && normalized.contains(&alias)
                    })
            })
            .collect::<Vec<_>>();
        matches.sort_by(|left, right| left.id.cmp(&right.id));
        matches.dedup_by(|left, right| left.id == right.id);
        matches
    }

    pub fn active_definitions<'a>(
        &'a self,
        active: impl IntoIterator<Item = &'a str>,
    ) -> Vec<&'a StateFieldDefinition> {
        let active = active.into_iter().collect::<HashSet<_>>();
        self.fields
            .iter()
            .filter(|field| active.contains(field.id.as_str()))
            .collect()
    }
}

fn validate_parameter_value(
    value: &ParameterValue,
    spec: &ParameterSpec,
) -> Result<(), &'static str> {
    let type_matches = matches!(
        (spec.value_kind, value),
        (ParameterValueKind::Integer, ParameterValue::Integer(_))
            | (ParameterValueKind::Float, ParameterValue::Integer(_))
            | (ParameterValueKind::Float, ParameterValue::Float(_))
            | (ParameterValueKind::Boolean, ParameterValue::Boolean(_))
            | (ParameterValueKind::Text, ParameterValue::Text(_))
            | (ParameterValueKind::Enum, ParameterValue::Text(_))
    );
    if !type_matches {
        return Err("STATE_VOCABULARY_VALUE_TYPE_MISMATCH");
    }
    let numeric = match value {
        ParameterValue::Integer(value) => Some(*value as f64),
        ParameterValue::Float(value) => Some(*value),
        _ => None,
    };
    if numeric.is_some_and(|value| {
        spec.minimum.is_some_and(|minimum| value < minimum)
            || spec.maximum.is_some_and(|maximum| value > maximum)
    }) {
        return Err("STATE_VOCABULARY_VALUE_OUT_OF_RANGE");
    }
    if spec.value_kind == ParameterValueKind::Enum {
        let ParameterValue::Text(value) = value else {
            return Err("STATE_VOCABULARY_VALUE_TYPE_MISMATCH");
        };
        if !spec.enum_values.iter().any(|allowed| allowed == value) {
            return Err("STATE_VOCABULARY_VALUE_OUT_OF_RANGE");
        }
    }
    Ok(())
}

pub fn validate_patch_against_vocabulary(
    patch: &ResearchStatePatch,
    registry: &StateVocabularyRegistry,
) -> Result<VocabularyValidationStats, String> {
    let mut stats = VocabularyValidationStats::default();
    for operation in &patch.operations {
        let expected_kind = VocabularyKind::from_state_field(operation.field);
        let ids = match &operation.value {
            Some(StateValue::Text { value }) => vec![value.as_str()],
            Some(StateValue::TextList { values }) => values.iter().map(String::as_str).collect(),
            Some(StateValue::Parameter { parameter }) => vec![parameter.key.as_str()],
            None if operation.action == StateAction::Clear => continue,
            None => Vec::new(),
        };
        for id in ids {
            // Legacy ephemeral parameter keys remain readable and writable for
            // backward compatibility, but are never exposed to the model.
            if expected_kind == VocabularyKind::Parameter
                && id.starts_with("custom:")
                && !id.starts_with("custom:parameter:")
            {
                continue;
            }
            let Some(definition) = registry.field(id) else {
                stats.unknown_id_count += 1;
                return Err("STATE_VOCABULARY_UNKNOWN_ID".to_string());
            };
            if definition.kind != expected_kind {
                stats.kind_mismatch_count += 1;
                return Err("STATE_VOCABULARY_KIND_MISMATCH".to_string());
            }
            let removal = matches!(operation.action, StateAction::Remove | StateAction::Clear);
            if !definition.enabled && !removal {
                stats.disabled_field_count += 1;
                return Err("STATE_VOCABULARY_FIELD_DISABLED".to_string());
            }
            if let Some(StateValue::Parameter { parameter }) = &operation.value {
                let Some(spec) = &definition.parameter_spec else {
                    stats.kind_mismatch_count += 1;
                    return Err("STATE_VOCABULARY_KIND_MISMATCH".to_string());
                };
                if let Some(required_unit) = &spec.unit {
                    if parameter
                        .unit
                        .as_deref()
                        .is_some_and(|unit| unit != required_unit)
                    {
                        stats.value_type_mismatch_count += 1;
                        return Err("STATE_VOCABULARY_VALUE_TYPE_MISMATCH".to_string());
                    }
                }
                if let Err(error) = validate_parameter_value(&parameter.value, spec) {
                    if error.ends_with("OUT_OF_RANGE") {
                        stats.value_out_of_range_count += 1;
                    } else {
                        stats.value_type_mismatch_count += 1;
                    }
                    return Err(error.to_string());
                }
            }
        }
    }
    Ok(stats)
}

pub fn active_ids_from_state(
    objectives: &[String],
    constraints: &[String],
    assumptions: &[String],
    methods: &[String],
    parameters: &BTreeMap<String, super::state_mutation::ResearchParameter>,
) -> Vec<String> {
    let mut values = objectives
        .iter()
        .chain(constraints)
        .chain(assumptions)
        .chain(methods)
        .cloned()
        .collect::<Vec<_>>();
    values.extend(parameters.keys().cloned());
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qa::state_mutation::{
        PatchConfidence, ResearchParameter, ResearchStateOperation, STATE_PATCH_VERSION,
    };

    fn parameter_patch(key: &str, value: ParameterValue) -> ResearchStatePatch {
        ResearchStatePatch {
            schema_version: STATE_PATCH_VERSION.to_string(),
            patch_id: "fixture".to_string(),
            operations: vec![ResearchStateOperation {
                action: StateAction::Set,
                field: StateField::Parameter,
                value: Some(StateValue::Parameter {
                    parameter: ResearchParameter {
                        key: key.to_string(),
                        value,
                        unit: None,
                        source_message_id: None,
                        updated_at_turn: 0,
                    },
                }),
                previous_value: None,
                confidence: PatchConfidence::High,
            }],
            confidence: PatchConfidence::High,
            source_message_id: None,
            parameter_implicit_reference_resolved_count: 0,
            parameter_implicit_reference_rejected_count: 0,
            parameter_unknown_name_count: 0,
            parameter_state_corruption_count: 0,
        }
    }

    #[test]
    fn builtin_contract_distinguishes_coordination_count_and_capacity() {
        let registry = StateVocabularyRegistry::default();
        assert_eq!(
            registry
                .exact_matches("需要多套移动供能设备共同调度", VocabularyKind::Constraint)
                .iter()
                .map(|field| field.id.as_str())
                .collect::<Vec<_>>(),
            ["multi_vehicle_coordination"]
        );
        assert_eq!(
            registry
                .exact_matches("充电器容量=50", VocabularyKind::Parameter)
                .iter()
                .map(|field| field.id.as_str())
                .collect::<Vec<_>>(),
            ["charger_capacity"]
        );
        assert!(registry
            .exact_matches("充电器容量=50", VocabularyKind::Parameter)
            .iter()
            .all(|field| field.id != "mobile_charger_count"));
    }

    #[test]
    fn validator_rejects_hallucinated_wrong_kind_and_out_of_range_fields() {
        let registry = StateVocabularyRegistry::default();
        assert_eq!(
            validate_patch_against_vocabulary(
                &parameter_patch("invented_parameter", ParameterValue::Integer(1)),
                &registry
            ),
            Err("STATE_VOCABULARY_UNKNOWN_ID".to_string())
        );

        let mut wrong_kind =
            parameter_patch("multi_vehicle_coordination", ParameterValue::Integer(2));
        assert_eq!(
            validate_patch_against_vocabulary(&wrong_kind, &registry),
            Err("STATE_VOCABULARY_KIND_MISMATCH".to_string())
        );

        wrong_kind = parameter_patch("mobile_charger_count", ParameterValue::Integer(0));
        assert_eq!(
            validate_patch_against_vocabulary(&wrong_kind, &registry),
            Err("STATE_VOCABULARY_VALUE_OUT_OF_RANGE".to_string())
        );
    }
}
